//! Code emission — turns parsed DCB schema into Rust source strings.
//!
//! Emits a **flat pool** model organized by **feature**: each auto-feature
//! gets its own module directory containing type definitions, pool storage,
//! and record index. The top-level `DataPools` / `RecordIndex` compose
//! per-feature sub-structs behind `#[cfg(feature = "...")]` gates.
//!
//! Feature classification is done by `crate::features::classify_features`.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Write as _;

use svarog_datacore::structs::DataCorePropertyDefinition;
use svarog_datacore::{DataCoreDatabase, DataType};

use crate::features::{FeatureAssignment, FeatureMap};
use crate::naming::{sanitize_field_name, sanitize_struct_name, sanitize_variant_name};
use crate::pipeline::Metadata;

// ── Public type aliases ───────────────────────────────────────────────────

/// Map from DCB struct index → sanitized Rust type name.
pub type EmittedStructNames = BTreeMap<u32, String>;

/// Map from DCB struct index → pool field name (snake_case, deduplicated).
pub type PoolFieldNames = BTreeMap<u32, String>;

/// Map from DCB struct index → feature module name (e.g., "core", "audio").
/// This replaces the old alphabetical BucketAssignments.
pub type FeatureAssignmentMap = BTreeMap<u32, String>;

/// Map from DCB struct index → optional `#[cfg(...)]` attribute string.
///
/// `None` means the type is always compiled (bare `FeatureAssignment::Core`
/// or `FeatureAssignment::Single` where the feature owns a whole module).
/// `Some("#[cfg(any(feature = \"f1\", feature = \"f2\"))]")` means the type
/// is `Multi`-assigned and must be gated at every emission site: the struct
/// definition, its `Pooled` / `Extract` impls, and the pool field inside
/// `CorePools`. Multi types live in the `core` module file but are only
/// compiled when one of their features is enabled.
pub type FeatureCfgMap = BTreeMap<u32, Option<String>>;

/// For each struct index, the transitive set of descendant struct indices
/// (NOT including the parent itself). Derived purely from the DCB's
/// `parent_type_index` chain, before reachability pruning.
pub type DescendantMap = BTreeMap<u32, Vec<u32>>;

/// Per-struct cached inherited property list, indexed by DCB struct index.
///
/// Each entry is the **full** inherited property list (own + walked
/// parent chain). Built once at the start of a generator run and shared
/// across every phase that needs to read fields: reachability BFS,
/// polymorphic base detection, feature classification, closure walks,
/// and per-struct emit. Folding everything through this table eliminates
/// the O(structs × inheritance_depth) redundancy the earlier pipeline had
/// from each phase re-walking parent chains on its own.
pub type PropertyCache<'a> = Vec<Vec<&'a DataCorePropertyDefinition>>;

/// Build the per-struct inherited property cache. Call once in the pipeline.
pub fn build_property_cache(db: &DataCoreDatabase) -> PropertyCache<'_> {
    (0..db.struct_definitions().len())
        .map(|idx| collect_full_properties(db, idx as u32))
        .collect()
}

/// Set of struct indices that serve as declared targets of StrongPointer /
/// WeakPointer fields in some emitted struct AND have at least one emitted
/// descendant. These are the types for which we emit a tagged enum instead
/// of a bare `Handle<T>` — the pointed-to instance may be a subclass at
/// runtime.
pub type PolyBaseSet = BTreeSet<u32>;

const FILE_HEADER: &str = "\
// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

";

// ── Precomputation (shared across all emission stages) ────────────────────

/// Build the descendant map from the DCB's parent_type_index chain.
///
/// For each struct index, walks the `parent_type_index` backwards to find
/// every struct that inherits from it (transitively). Does **not** include
/// the struct itself.
pub fn compute_descendants(db: &DataCoreDatabase) -> DescendantMap {
    let n = db.struct_definitions().len();
    // Direct children: parent_idx → Vec<child_idx>.
    let mut children: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for idx in 0..n {
        let def = &db.struct_definitions()[idx];
        if def.parent_type_index >= 0 && (def.parent_type_index as usize) < n {
            children
                .entry(def.parent_type_index as u32)
                .or_default()
                .push(idx as u32);
        }
    }

    // Transitive descendants via BFS down the tree.
    let mut out: DescendantMap = BTreeMap::new();
    for parent in children.keys().copied().collect::<Vec<_>>() {
        let mut all: Vec<u32> = Vec::new();
        let mut queue: VecDeque<u32> = VecDeque::new();
        if let Some(direct) = children.get(&parent) {
            for &c in direct {
                queue.push_back(c);
            }
        }
        while let Some(c) = queue.pop_front() {
            all.push(c);
            if let Some(grand) = children.get(&c) {
                for &g in grand {
                    queue.push_back(g);
                }
            }
        }
        all.sort_unstable();
        all.dedup();
        out.insert(parent, all);
    }
    out
}

/// Compute the set of struct indices reachable from top-level records.
///
/// Walks every emitted struct's Class/Pointer fields. For `StrongPointer`
/// and `WeakPointer` fields, also enqueues every **descendant** of the
/// declared target — those are the concrete types that may actually be
/// stored at runtime via polymorphic dispatch. For inline `Class` fields
/// only the declared target is reached (inline class data is monomorphic
/// by wire-format construction: see `docs/datacore.md` and
/// `tools/sc-generator/src/polymorphism.rs`).
pub fn compute_reachable_struct_indices(
    db: &DataCoreDatabase,
    descendants: &DescendantMap,
    cache: &PropertyCache<'_>,
) -> HashSet<u32> {
    let mut reachable: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<u32> = VecDeque::new();

    for record in db.records() {
        if record.struct_index < 0 {
            continue;
        }
        let idx = record.struct_index as u32;
        if reachable.insert(idx) {
            queue.push_back(idx);
        }
    }

    while let Some(idx) = queue.pop_front() {
        for prop in &cache[idx as usize] {
            let data_type = prop.get_data_type();
            let is_ptr = matches!(
                data_type,
                Some(DataType::StrongPointer) | Some(DataType::WeakPointer)
            );
            let is_class = matches!(data_type, Some(DataType::Class));
            if !(is_ptr || is_class) {
                continue;
            }
            let target = prop.struct_index as u32;
            if target as usize >= db.struct_definitions().len() {
                continue;
            }
            if reachable.insert(target) {
                queue.push_back(target);
            }
            // Pointers are polymorphic — pull in every descendant so the
            // tagged-enum variants can reference real emitted types.
            if is_ptr && let Some(kids) = descendants.get(&target) {
                for &k in kids {
                    if reachable.insert(k) {
                        queue.push_back(k);
                    }
                }
            }
        }
    }

    // Self-check: verify the closure is actually closed for both the
    // declared-target rule AND the pointer-descendant rule.
    for &idx in &reachable {
        for prop in &cache[idx as usize] {
            let data_type = prop.get_data_type();
            let is_ptr = matches!(
                data_type,
                Some(DataType::StrongPointer) | Some(DataType::WeakPointer)
            );
            let is_class = matches!(data_type, Some(DataType::Class));
            if !(is_ptr || is_class) {
                continue;
            }
            let target = prop.struct_index as u32;
            if (target as usize) >= db.struct_definitions().len() {
                continue;
            }
            if !reachable.contains(&target) {
                let from = db.struct_name(idx as usize).unwrap_or("<?>");
                let to = db.struct_name(target as usize).unwrap_or("<?>");
                let field = db.property_name(prop).unwrap_or("<?>");
                panic!(
                    "reachability BFS hole: `{from}` (idx {idx}) has Class/Pointer field \
                     `{field}` → `{to}` (idx {target}), but target was not reached."
                );
            }
            if is_ptr && let Some(kids) = descendants.get(&target) {
                for &k in kids {
                    if !reachable.contains(&k) {
                        let from = db.struct_name(idx as usize).unwrap_or("<?>");
                        let to = db.struct_name(target as usize).unwrap_or("<?>");
                        let kid = db.struct_name(k as usize).unwrap_or("<?>");
                        let field = db.property_name(prop).unwrap_or("<?>");
                        panic!(
                            "reachability BFS hole (descendant): `{from}` (idx {idx}) has \
                                 pointer field `{field}` → `{to}` (idx {target}) with descendant \
                                 `{kid}` (idx {k}) that was not reached."
                        );
                    }
                }
            }
        }
    }

    reachable
}

/// Compute the set of types that should be emitted as polymorphic tagged
/// enums. A type qualifies if it is the declared target of at least one
/// StrongPointer/WeakPointer field on some emitted struct, AND it has at
/// least one emitted descendant (i.e., runtime polymorphism is actually
/// possible at that field site).
pub fn compute_poly_bases(
    emitted_names: &EmittedStructNames,
    descendants: &DescendantMap,
    cache: &PropertyCache<'_>,
) -> PolyBaseSet {
    let mut out: PolyBaseSet = BTreeSet::new();
    for &parent_idx in emitted_names.keys() {
        for prop in &cache[parent_idx as usize] {
            let is_ptr = matches!(
                prop.get_data_type(),
                Some(DataType::StrongPointer) | Some(DataType::WeakPointer)
            );
            if !is_ptr {
                continue;
            }
            let target = prop.struct_index as u32;
            if !emitted_names.contains_key(&target) {
                continue;
            }
            let Some(kids) = descendants.get(&target) else {
                continue;
            };
            if kids.iter().any(|k| emitted_names.contains_key(k)) {
                out.insert(target);
            }
        }
    }
    out
}

/// Build the emitted struct name table (deduplicated, pruned).
///
/// `reachable` must be the set produced by `compute_reachable_struct_indices`
/// — this function no longer recomputes it, so the pipeline can hoist the
/// BFS to the top level and reuse the result across phases.
pub fn compute_emitted_struct_names(
    db: &DataCoreDatabase,
    reachable: &HashSet<u32>,
) -> EmittedStructNames {
    let mut out: EmittedStructNames = BTreeMap::new();
    let mut seen: HashSet<String> = HashSet::new();
    for struct_idx in 0..db.struct_definitions().len() {
        if !reachable.contains(&(struct_idx as u32)) {
            continue;
        }
        let Some(raw_name) = db.struct_name(struct_idx) else {
            continue;
        };
        let name = sanitize_struct_name(raw_name);
        if name.is_empty() || !seen.insert(name.clone()) {
            continue;
        }
        out.insert(struct_idx as u32, name);
    }
    out
}

/// Derive unique pool field names for every emitted struct.
pub fn compute_pool_field_names(emitted_names: &EmittedStructNames) -> PoolFieldNames {
    let mut used: HashSet<String> = HashSet::new();
    let mut out: PoolFieldNames = BTreeMap::new();
    for (&struct_idx, type_name) in emitted_names {
        let base = sanitize_field_name(type_name);
        let mut field = base.clone();
        let mut suffix = 2u32;
        while !used.insert(field.clone()) {
            field = format!("{base}_{suffix}");
            suffix += 1;
        }
        out.insert(struct_idx, field);
    }
    out
}

/// Build a map from struct index → feature module name.
///
/// - `Single(f)` → the type lives in feature `f`'s module.
/// - `Multi(_)` → the type lives in the `core` module but is gated by a
///   `#[cfg(any(...))]` attribute (see `compute_feature_cfg_map`).
/// - `Core` → the type lives unconditionally in `core`.
/// Synthetic feature name for the always-compiled module containing types
/// that are shared across multiple real features. Each type inside carries
/// its own `#[cfg(any(feature = "f1", feature = "f2", ...))]` gate; the
/// module itself is unconditional so `use crate::multi_feature::*;` glob
/// imports always resolve.
///
/// Not a real Cargo feature — never emitted in `Cargo.toml`.
pub const MULTI_FEATURE_MODULE: &str = "multi-feature";

/// Synthetic feature name for the module containing schema-reachable
/// types that were never observed in any record instance graph. Gated at
/// the module declaration with `#[cfg(feature = "dormant")]`, so
/// individual types inside do **not** need a per-type cfg.
pub const DORMANT_MODULE: &str = "dormant";

pub fn compute_feature_assignment_map(
    emitted_names: &EmittedStructNames,
    feature_map: &FeatureMap,
) -> FeatureAssignmentMap {
    let mut out = FeatureAssignmentMap::new();
    for &struct_idx in emitted_names.keys() {
        let feature_name = match feature_map
            .struct_feature
            .get(struct_idx as usize)
            .and_then(|a| a.as_ref())
        {
            Some(FeatureAssignment::Core) => "core".to_string(),
            Some(FeatureAssignment::Dormant) => DORMANT_MODULE.to_string(),
            Some(FeatureAssignment::Single(f)) => f.clone(),
            Some(FeatureAssignment::Multi(_)) => MULTI_FEATURE_MODULE.to_string(),
            // Orphan fallback (unreachable in practice — every emitted
            // type is classified somewhere). Park it in core.
            None => "core".to_string(),
        };
        out.insert(struct_idx, feature_name);
    }
    out
}

/// Build a map from struct index → optional `#[cfg(...)]` attribute.
///
/// Only `FeatureAssignment::Multi` produces a `Some` entry — a
/// `#[cfg(any(feature = "f1", feature = "f2", ...))]` attribute that
/// gates the struct definition, its `Pooled` / `Extract` impls, and the
/// corresponding pool field inside `MultiFeaturePools`. Multi types live
/// in the always-compiled `multi_feature` module but only compile when
/// at least one of their owning features is enabled.
///
/// Single-feature, Core, and Dormant types get `None` here: their gates
/// are inherited from the `#[cfg(feature = "…")]` on their module's
/// declaration at the top level (`pub mod X;` in `generated/mod.rs`).
/// Emitting per-type cfgs for those would be redundant and noisy.
pub fn compute_feature_cfg_map(
    emitted_names: &EmittedStructNames,
    feature_map: &FeatureMap,
) -> FeatureCfgMap {
    let mut out = FeatureCfgMap::new();
    for &struct_idx in emitted_names.keys() {
        let cfg = match feature_map
            .struct_feature
            .get(struct_idx as usize)
            .and_then(|a| a.as_ref())
        {
            Some(FeatureAssignment::Multi(features)) if !features.is_empty() => {
                let parts: Vec<String> = features
                    .iter()
                    .map(|f| format!("feature = \"{f}\""))
                    .collect();
                Some(format!("#[cfg(any({}))]", parts.join(", ")))
            }
            _ => None,
        };
        out.insert(struct_idx, cfg);
    }
    out
}

// ── Per-feature emission ──────────────────────────────────────────────────

/// Emit a feature's `types.rs` file: struct definitions + Extract + Pooled impls.
#[allow(clippy::too_many_arguments)]
pub fn emit_feature_types(
    db: &DataCoreDatabase,
    feature_name: &str,
    struct_indices: &[u32],
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
    poly_bases: &PolyBaseSet,
    feature_cfgs: &FeatureCfgMap,
    cache: &PropertyCache<'_>,
) -> String {
    let mut out = types_file_header(feature_name);
    for &struct_idx in struct_indices {
        emit_struct(
            db,
            struct_idx,
            emitted_names,
            pool_fields,
            poly_bases,
            feature_cfgs,
            feature_name,
            cache,
            &mut out,
        );
    }
    out
}

/// Emit the top-level `poly_enums.rs` file: one tagged enum per poly base,
/// with per-variant `#[cfg(feature = "…")]` gates derived from each concrete
/// subclass's feature assignment. The file lives at the `generated` module
/// root so that every feature — core and optional alike — can refer to the
/// enums regardless of which feature owns the base or the variants.
///
/// At runtime, each enum's `from_ref` dispatcher matches on
/// `Instance::type_name()` rather than raw `struct_index`, keeping the
/// dispatch stable across game patches where struct ordering may drift.
pub fn emit_poly_enums_file(
    db: &DataCoreDatabase,
    poly_bases: &PolyBaseSet,
    emitted_names: &EmittedStructNames,
    descendants: &DescendantMap,
    feature_assignments: &FeatureAssignmentMap,
    feature_cfgs: &FeatureCfgMap,
) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Polymorphic-base tagged enums.\n");
    out.push_str("//!\n");
    out.push_str("//! For every DCB base class that is the declared target of a\n");
    out.push_str("//! StrongPointer/WeakPointer field and has at least one subclass,\n");
    out.push_str("//! the generator emits a `{Base}Ptr` enum here. At runtime, extract\n");
    out.push_str("//! dispatches on the instance's type name to select the correct\n");
    out.push_str("//! concrete variant.\n\n");
    out.push_str("#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]\n\n");
    out.push_str("use crate::{Builder, Handle};\n");
    out.push_str("use super::*;\n\n");

    for &base_idx in poly_bases {
        emit_poly_enum(
            db,
            base_idx,
            emitted_names,
            descendants,
            feature_assignments,
            feature_cfgs,
            &mut out,
        );
    }

    out
}

/// Emit one polymorphic-base tagged enum + `from_ref` dispatch helper into
/// the shared `poly_enums.rs` file.
fn emit_poly_enum(
    db: &DataCoreDatabase,
    base_idx: u32,
    emitted_names: &EmittedStructNames,
    descendants: &DescendantMap,
    feature_assignments: &FeatureAssignmentMap,
    feature_cfgs: &FeatureCfgMap,
    out: &mut String,
) {
    let Some(base_name) = emitted_names.get(&base_idx) else {
        return;
    };
    let Some(base_raw) = db.struct_name(base_idx as usize) else {
        return;
    };
    let enum_name = format!("{base_name}Ptr");

    // Resolve the per-variant cfg attribute. A variant's cfg must match
    // the cfg the variant's target type carries at its definition site:
    // - Multi types: `#[cfg(any(feature = ...))]` (from `feature_cfgs`).
    // - Dormant types: `#[cfg(feature = "dormant")]` (inherited from
    //   the `dormant` module's declaration).
    // - Single-feature types: the feature's own cfg, because the whole
    //   feature module is gated at `pub mod X;` in `generated/mod.rs`.
    // - Core and `multi-feature` module types: no cfg.
    let variant_cfg = |idx: u32| -> String {
        if let Some(Some(c)) = feature_cfgs.get(&idx) {
            return c.clone();
        }
        let feature = feature_assignments
            .get(&idx)
            .map(String::as_str)
            .unwrap_or("core");
        feature_module_cfg(feature).unwrap_or_default()
    };

    // Build variant list: (type_name, cfg_attr_or_empty, raw DCB name).
    // Includes the base itself as its own variant plus every emitted
    // descendant.
    let mut variants: Vec<(String, String, String)> = Vec::new();
    variants.push((
        base_name.clone(),
        variant_cfg(base_idx),
        base_raw.to_string(),
    ));
    if let Some(kids) = descendants.get(&base_idx) {
        for &k in kids {
            let Some(kid_name) = emitted_names.get(&k) else {
                continue;
            };
            let Some(kid_raw) = db.struct_name(k as usize) else {
                continue;
            };
            variants.push((kid_name.clone(), variant_cfg(k), kid_raw.to_string()));
        }
    }

    let _ = writeln!(out, "/// Polymorphic pointer to `{base_raw}`.");
    out.push_str("///\n");
    out.push_str("/// StrongPointer/WeakPointer fields declared with this base may\n");
    out.push_str("/// store any emitted subclass at runtime. Variants for types in\n");
    out.push_str("/// optional features are `#[cfg]`-gated; when their feature is\n");
    out.push_str("/// disabled at compile time, runtime dispatch falls through to\n");
    out.push_str("/// `Unknown { struct_index, instance_index }`.\n");
    out.push_str("#[derive(Debug, Clone)]\n");
    let _ = writeln!(out, "pub enum {enum_name} {{");
    for (type_name, cfg, _) in &variants {
        if !cfg.is_empty() {
            let _ = writeln!(out, "    {cfg}");
        }
        let _ = writeln!(out, "    {type_name}(Handle<{type_name}>),");
    }
    out.push_str("    /// Runtime pointer to a subclass whose feature was disabled at\n");
    out.push_str("    /// compile time, or a type the generator doesn't know about.\n");
    out.push_str("    /// Use `Datacore::db()` + the two indices to walk the raw instance.\n");
    out.push_str("    Unknown { struct_index: u32, instance_index: u32 },\n");
    out.push_str("}\n\n");

    // from_ref dispatch.
    let _ = writeln!(out, "impl {enum_name} {{");
    out.push_str("    #[allow(dead_code)]\n");
    out.push_str("    pub(crate) fn from_ref<'a>(b: &mut Builder<'a>, r: svarog_datacore::InstanceRef) -> Self {\n");
    out.push_str("        let inst = b.db.instance(r.struct_index, r.instance_index);\n");
    out.push_str("        match inst.type_name() {\n");
    for (type_name, cfg, raw_name) in &variants {
        let cfg_prefix = if cfg.is_empty() {
            String::new()
        } else {
            format!("{cfg} ")
        };
        let escaped = raw_name.replace('\\', "\\\\").replace('"', "\\\"");
        let _ = writeln!(
            out,
            "            {cfg_prefix}Some(\"{escaped}\") => Self::{type_name}(b.alloc_nested::<{type_name}>(inst, true)),",
        );
    }
    out.push_str("            _ => Self::Unknown { struct_index: r.struct_index, instance_index: r.instance_index },\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

/// Emit a feature's pool sub-struct.
///
/// Individual pool fields for `Multi`-assigned types get `#[cfg(...)]`
/// attributes from `feature_cfgs` so the pool struct compiles when only
/// some of the type's features are enabled. The pool struct itself stays
/// unconditional (it always exists, just with fewer fields when features
/// are disabled).
pub fn emit_feature_pools(
    feature_name: &str,
    struct_indices: &[u32],
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
    feature_cfgs: &FeatureCfgMap,
) -> String {
    let suffix = feature_to_type_suffix(feature_name);
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]\n\n");
    out.push_str("use super::super::*;\n\n");

    let _ = writeln!(out, "/// Pool storage for the `{feature_name}` feature.");
    out.push_str("#[derive(Default)]\n");
    let _ = writeln!(out, "pub struct {suffix}Pools {{");
    for &struct_idx in struct_indices {
        if let (Some(type_name), Some(field)) =
            (emitted_names.get(&struct_idx), pool_fields.get(&struct_idx))
        {
            if let Some(Some(cfg)) = feature_cfgs.get(&struct_idx) {
                let _ = writeln!(out, "    {cfg}");
            }
            let _ = writeln!(out, "    pub {field}: Vec<Option<{type_name}>>,");
        }
    }
    out.push_str("}\n");
    out
}

/// Emit a feature's record index sub-struct (only record types).
///
/// Record types almost always have a natural single-feature assignment
/// (they have a record path that seeds exactly one closure), so the cfg
/// plumbing here is usually a no-op. We still thread the cfg map through
/// for symmetry and to cover the theoretical case of a record type being
/// reachable from multiple features' closures.
pub fn emit_feature_index(
    _db: &DataCoreDatabase,
    feature_name: &str,
    record_struct_indices: &[u32],
    emitted_names: &EmittedStructNames,
    feature_cfgs: &FeatureCfgMap,
) -> Option<String> {
    // Filter to record types that belong to this feature.
    if record_struct_indices.is_empty() {
        return None;
    }

    let suffix = feature_to_type_suffix(feature_name);
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]\n\n");
    out.push_str("use std::collections::HashMap;\n");
    out.push_str("use svarog_common::CigGuid;\n");
    out.push_str("use crate::Handle;\n");
    out.push_str("use super::super::*;\n\n");

    let _ = writeln!(out, "/// Record index for the `{feature_name}` feature.");
    out.push_str("#[derive(Default)]\n");
    let _ = writeln!(out, "pub struct {suffix}Index {{");

    let mut used: HashSet<String> = HashSet::new();
    // (field_name, type_name, cfg_attr_or_empty)
    let mut fields: Vec<(String, String, String)> = Vec::new();
    for &struct_idx in record_struct_indices {
        let Some(type_name) = emitted_names.get(&struct_idx) else {
            continue;
        };
        let base = sanitize_field_name(type_name);
        let mut field = base.clone();
        let mut sfx = 2u32;
        while !used.insert(field.clone()) {
            field = format!("{base}_{sfx}");
            sfx += 1;
        }
        let cfg_attr = feature_cfgs
            .get(&struct_idx)
            .and_then(|c| c.as_deref())
            .unwrap_or("")
            .to_string();
        if !cfg_attr.is_empty() {
            let _ = writeln!(out, "    {cfg_attr}");
        }
        let _ = writeln!(
            out,
            "    pub {field}: HashMap<CigGuid, Handle<{type_name}>>,"
        );
        fields.push((field, type_name.clone(), cfg_attr));
    }
    out.push_str("}\n\n");

    // len() / is_empty()
    //
    // The accumulator is a mutable local so cfg-gated fields disappear
    // cleanly when their feature is disabled: `#[cfg(...)] total += ...;`
    // on a statement is idiomatic Rust and becomes a no-op at compile
    // time when the cfg is off.
    let _ = writeln!(out, "impl {suffix}Index {{");
    out.push_str("    #[allow(unused_mut)]\n");
    out.push_str("    pub fn len(&self) -> usize {\n");
    out.push_str("        let mut total = 0usize;\n");
    for (field, _, cfg_attr) in &fields {
        if cfg_attr.is_empty() {
            let _ = writeln!(out, "        total += self.{field}.len();");
        } else {
            let _ = writeln!(out, "        {cfg_attr}");
            let _ = writeln!(out, "        {{ total += self.{field}.len(); }}");
        }
    }
    out.push_str("        total\n");
    out.push_str("    }\n\n");
    out.push_str("    pub fn is_empty(&self) -> bool { self.len() == 0 }\n");
    out.push_str("}\n");

    Some(out)
}

/// Emit a feature's `mod.rs`.
pub fn emit_feature_mod(feature_name: &str, has_index: bool) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    let _ = writeln!(out, "//! Feature module: `{feature_name}`\n");
    // Inner submodules are private so the top-level `pub use
    // feature::*;` glob doesn't drag in the `types` / `pools` / `index`
    // module names themselves — those conflict with neighbor features'
    // submodules of the same name and trigger `ambiguous_glob_reexports`.
    // The `pub use` below re-exports their contents for public access.
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("mod types;\n");
    out.push_str("mod pools;\n");
    if has_index {
        out.push_str("mod index;\n");
    }
    out.push('\n');
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("pub use types::*;\n");
    out.push_str("pub use pools::*;\n");
    if has_index {
        out.push_str("pub use index::*;\n");
    }
    out
}

// ── Top-level composition ─────────────────────────────────────────────────

/// Emit the top-level `DataPools` that composes per-feature pool sub-structs.
pub fn emit_top_data_pools(feature_names: &[String]) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Top-level `DataPools` composing per-feature pool sub-structs.\n\n");
    out.push_str("#![allow(unused_imports)]\n\n");

    out.push_str("#[derive(Default)]\n");
    out.push_str("pub struct DataPools {\n");
    for feature in feature_names {
        let suffix = feature_to_type_suffix(feature);
        let field = feature_to_field_name(feature);
        let mod_name = feature_to_mod_name(feature);
        if let Some(cfg) = feature_module_cfg(feature) {
            let _ = writeln!(out, "    {cfg}");
        }
        let _ = writeln!(out, "    pub {field}: super::{mod_name}::{suffix}Pools,");
    }
    out.push_str("}\n");
    out
}

/// Emit the top-level `RecordIndex` that composes per-feature index sub-structs.
pub fn emit_top_record_index(features_with_index: &[String]) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Top-level `RecordIndex` composing per-feature index sub-structs.\n\n");
    out.push_str("#![allow(unused_imports)]\n\n");

    out.push_str("#[derive(Default)]\n");
    out.push_str("pub struct RecordIndex {\n");
    for feature in features_with_index {
        let suffix = feature_to_type_suffix(feature);
        let field = feature_to_field_name(feature);
        let mod_name = feature_to_mod_name(feature);
        if let Some(cfg) = feature_module_cfg(feature) {
            let _ = writeln!(out, "    {cfg}");
        }
        let _ = writeln!(out, "    pub {field}: super::{mod_name}::{suffix}Index,");
    }
    out.push_str("}\n\n");

    out.push_str("impl RecordIndex {\n");
    out.push_str("    #[allow(unused_mut)]\n");
    out.push_str("    pub fn len(&self) -> usize {\n");
    out.push_str("        let mut n = 0;\n");
    for feature in features_with_index {
        let field = feature_to_field_name(feature);
        match feature_module_cfg(feature) {
            None => {
                let _ = writeln!(out, "        n += self.{field}.len();");
            }
            Some(cfg) => {
                let _ = writeln!(out, "        {cfg}");
                let _ = writeln!(out, "        {{ n += self.{field}.len(); }}");
            }
        }
    }
    out.push_str("        n\n");
    out.push_str("    }\n\n");
    out.push_str("    pub fn is_empty(&self) -> bool { self.len() == 0 }\n");
    out.push_str("}\n");
    out
}

/// Emit `RecordStore` + `seed_database` with cfg'd dispatch.
pub fn emit_record_store(
    db: &DataCoreDatabase,
    emitted_names: &EmittedStructNames,
    _pool_fields: &PoolFieldNames,
    feature_assignments: &FeatureAssignmentMap,
    feature_cfgs: &FeatureCfgMap,
) -> String {
    // Discover record types.
    let mut record_struct_indices: Vec<u32> = Vec::new();
    let mut seen: HashSet<u32> = HashSet::new();
    for record in db.records() {
        let struct_idx = record.struct_index;
        if struct_idx < 0 {
            continue;
        }
        let struct_idx = struct_idx as u32;
        if !seen.insert(struct_idx) {
            continue;
        }
        if !emitted_names.contains_key(&struct_idx) {
            continue;
        }
        record_struct_indices.push(struct_idx);
    }
    record_struct_indices.sort_unstable();

    // Build entries: (struct_idx, type_name, field_name, feature_name,
    // cfg_attr_string). cfg_attr_string is the resolved `#[cfg(...)]`
    // attribute that must gate the extractor fn, the seed_database
    // registration, and any per-type access in record_store.rs:
    // - `Multi`-assigned types: cfg comes from `feature_cfgs`
    //   (`#[cfg(any(feature = "f1", ...))]`) because the type lives in
    //   the core module file with a per-type gate.
    // - `Single`-assigned types in a non-core feature: cfg is the
    //   feature's own `#[cfg(feature = "X")]`, because the feature's
    //   module is gated at its `pub mod X;` declaration.
    // - `Core` fallback (orphan-with-no-closure): no cfg.
    let mut used: HashSet<String> = HashSet::new();
    let mut entries: Vec<(u32, String, String, String, String)> = Vec::new();
    for struct_idx in &record_struct_indices {
        let type_name = emitted_names.get(struct_idx).unwrap().clone();
        let base = sanitize_field_name(&type_name);
        let mut field = base.clone();
        let mut suffix = 2u32;
        while !used.insert(field.clone()) {
            field = format!("{base}_{suffix}");
            suffix += 1;
        }
        let feature = feature_assignments
            .get(struct_idx)
            .cloned()
            .unwrap_or("core".into());
        // Per-type Multi cfg wins if present (the type is gated on an
        // `any(feature = ...)` set); otherwise inherit the module-level
        // gate from the feature assignment, which returns `None` for
        // `core` and `multi-feature`.
        let cfg_attr = if let Some(Some(multi_cfg)) = feature_cfgs.get(struct_idx) {
            multi_cfg.clone()
        } else {
            feature_module_cfg(&feature).unwrap_or_default()
        };
        entries.push((*struct_idx, type_name, field, feature, cfg_attr));
    }

    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]\n\n");
    out.push_str("use std::collections::HashMap;\n");
    out.push_str("use svarog_common::CigGuid;\n");
    out.push_str("use svarog_datacore::Instance;\n");
    out.push_str("use crate::{Builder, Extract, Handle};\n");
    out.push_str("use super::*;\n\n");

    // RecordStore struct
    out.push_str("#[derive(Default)]\n");
    out.push_str("pub struct RecordStore {\n");
    out.push_str("    pub pools: DataPools,\n");
    out.push_str("    pub records: RecordIndex,\n");
    out.push_str("}\n\n");

    out.push_str("impl RecordStore {\n");
    out.push_str("    pub fn new() -> Self { Self::default() }\n");
    out.push_str("    pub fn len(&self) -> usize { self.records.len() }\n");
    out.push_str("    pub fn is_empty(&self) -> bool { self.records.is_empty() }\n");
    out.push_str("}\n\n");

    // Per-record-type extractor functions
    out.push_str("type RecordExtractor<'a> = fn(&mut Builder<'a>, CigGuid, Instance<'a>);\n\n");
    for (_idx, type_name, field, feature, cfg_attr) in &entries {
        let fn_name = format!("extract_record_{field}");
        let feature_field = feature_to_field_name(feature);
        if !cfg_attr.is_empty() {
            let _ = writeln!(out, "{cfg_attr}");
        }
        let _ = writeln!(
            out,
            "fn {fn_name}<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {{"
        );
        let _ = writeln!(
            out,
            "    let id: Handle<{type_name}> = Handle::new(b.alloc_record::<{type_name}>(inst, guid));"
        );
        let _ = writeln!(
            out,
            "    b.records.{feature_field}.{field}.insert(guid, id);"
        );
        let _ = writeln!(out, "}}\n");
    }

    // seed_database
    out.push_str("impl<'a> Builder<'a> {\n");
    // Core-only / narrow-feature builds may have zero record extractors,
    // in which case `name_to_idx` / `dispatch` are unused and `mut` is
    // dead. Suppress those warnings generously rather than emit conditional
    // bodies.
    out.push_str(
        "    #[allow(unused_variables, unused_mut, unused_assignments, clippy::let_and_return)]\n",
    );
    out.push_str("    pub fn seed_database(&mut self) {\n");
    if !entries.is_empty() {
        out.push_str("        let n_structs = self.db.struct_definitions().len();\n");
        out.push_str("        let name_to_idx: HashMap<&str, usize> = (0..n_structs)\n");
        out.push_str("            .filter_map(|i| self.db.struct_name(i).map(|n| (n, i)))\n");
        out.push_str("            .collect();\n\n");
        out.push_str(
            "        let mut dispatch: Vec<Option<RecordExtractor<'a>>> = vec![None; n_structs];\n",
        );

        for (_idx, type_name, field, _feature, cfg_attr) in &entries {
            let fn_name = format!("extract_record_{field}");
            if !cfg_attr.is_empty() {
                let _ = writeln!(out, "        {cfg_attr}");
            }
            let _ = writeln!(
                out,
                "        if let Some(&i) = name_to_idx.get({type_name}::TYPE_NAME) {{ dispatch[i] = Some({fn_name}); }}"
            );
        }
        out.push('\n');

        out.push_str("        #[cfg(debug_assertions)]\n");
        out.push_str("        let mut unknown_record_types: std::collections::HashSet<u32> = std::collections::HashSet::new();\n\n");

        out.push_str("        let records: Vec<(CigGuid, u32, Instance<'a>)> = self\n");
        out.push_str("            .db\n");
        out.push_str("            .all_records()\n");
        out.push_str("            .map(|r| (r.id(), r.struct_index(), r.as_instance()))\n");
        out.push_str("            .collect();\n");
        out.push_str("        for (guid, struct_idx, inst) in records {\n");
        out.push_str("            let i = struct_idx as usize;\n");
        out.push_str("            if let Some(Some(f)) = dispatch.get(i) {\n");
        out.push_str("                f(self, guid, inst);\n");
        out.push_str("            } else {\n");
        out.push_str("                #[cfg(debug_assertions)]\n");
        out.push_str("                { unknown_record_types.insert(struct_idx); }\n");
        out.push_str("            }\n");
        out.push_str("        }\n\n");

        out.push_str("        #[cfg(debug_assertions)]\n");
        out.push_str("        if !unknown_record_types.is_empty() {\n");
        out.push_str("            let mut names: Vec<&str> = unknown_record_types\n");
        out.push_str("                .iter()\n");
        out.push_str(
            "                .map(|&i| self.db.struct_name(i as usize).unwrap_or(\"<unnamed>\"))\n",
        );
        out.push_str("                .collect();\n");
        out.push_str("            names.sort_unstable();\n");
        out.push_str("            panic!(\n");
        out.push_str(
            "                \"sc-extract-generated: runtime DCB contains {} record type(s) \\\n",
        );
        out.push_str("                 the generator doesn't know about — generated bindings are stale. \\\n");
        out.push_str(
            "                 Regenerate with `cargo run -p sc-generator -- --p4k <path>`.\\n\\\n",
        );
        out.push_str("                 Unknown types: {:?}\",\n");
        out.push_str("                names.len(),\n");
        out.push_str("                names,\n");
        out.push_str("            );\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Emit the top-level `mod.rs` with cfg'd feature module includes.
pub fn emit_top_mod(feature_names: &[String], feature_map: &FeatureMap) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Machine-generated DataCore schema bindings.\n");
    out.push_str("//!\n");
    out.push_str("//! Types are organized by auto-feature: each feature module contains\n");
    out.push_str("//! the struct types, pool storage, and record index for records under\n");
    out.push_str("//! a specific path prefix. Disable features you don't need to cut\n");
    out.push_str("//! compile time.\n\n");

    out.push_str("pub mod enums;\n");
    out.push_str("pub mod metadata;\n");
    out.push_str("pub mod poly_enums;\n\n");

    // Feature modules
    for feature in feature_names {
        let mod_name = feature_to_mod_name(feature);
        if let Some(cfg) = feature_module_cfg(feature) {
            let _ = writeln!(out, "{cfg}");
        }
        let _ = writeln!(out, "pub mod {mod_name};");
    }
    out.push('\n');

    out.push_str("pub mod data_pools;\n");
    out.push_str("pub mod record_index;\n");
    out.push_str("pub mod record_store;\n\n");

    // Re-exports
    for feature in feature_names {
        let mod_name = feature_to_mod_name(feature);
        if let Some(cfg) = feature_module_cfg(feature) {
            let _ = writeln!(out, "{cfg}");
        }
        let _ = writeln!(out, "pub use {mod_name}::*;");
    }
    out.push_str("pub use enums::*;\n");
    out.push_str("pub use poly_enums::*;\n");
    out.push_str("pub use data_pools::DataPools;\n");
    out.push_str("pub use record_index::RecordIndex;\n");
    out.push_str("pub use record_store::RecordStore;\n\n");

    // Emit the feature list as a doc comment for reference.
    out.push_str("// ── Auto-generated feature list ────────────────────────────────────────\n");
    out.push_str("//\n");
    for feature in feature_names {
        if feature != "core" && feature != MULTI_FEATURE_MODULE {
            let _ = writeln!(out, "// feature: {feature}");
        }
    }
    // Parent features
    for (parent, children) in &feature_map.parent_features {
        let _ = writeln!(out, "// parent: {parent} = [{}]", children.join(", "));
    }

    out
}

/// Emit `enums.rs` — enum type definitions plus a `from_dcb_str`
/// associated function per enum that maps raw DCB strings back to the
/// Rust variant (falling through to `Unrecognized(String)` for values
/// not present at generation time).
pub fn emit_enums(db: &DataCoreDatabase) -> String {
    const FALLBACK_VARIANT: &str = "Unrecognized";

    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_camel_case_types, dead_code)]\n\n");

    let mut emitted: HashSet<String> = HashSet::new();
    for enum_idx in 0..db.enum_definitions().len() {
        let Some(name) = db.enum_name(enum_idx) else {
            continue;
        };
        let sanitized = sanitize_struct_name(name);
        if sanitized.is_empty() || !emitted.insert(sanitized.clone()) {
            continue;
        }
        let def = &db.enum_definitions()[enum_idx];
        let values = db.enum_options(def);

        // Collect (raw DCB value, sanitized Rust variant) pairs, applying
        // the same collision-skip logic as before. We pass over the list
        // once here so the type definition and the `from_dcb_str` impl
        // stay in lockstep.
        let mut variant_names: HashSet<String> = HashSet::new();
        variant_names.insert(FALLBACK_VARIANT.to_string());
        let mut kept_variants: Vec<(&str, String)> = Vec::new();
        for value in &values {
            let raw: &str = *value;
            let variant = sanitize_variant_name(raw);
            if variant.is_empty() || !variant_names.insert(variant.clone()) {
                continue;
            }
            kept_variants.push((raw, variant));
        }

        let _ = writeln!(out, "/// DCB enum: `{name}`");
        out.push_str("#[derive(Debug, Clone, PartialEq, Eq, Hash)]\n");
        let _ = writeln!(out, "pub enum {sanitized} {{");
        if kept_variants.is_empty() {
            out.push_str("    Empty,\n");
        } else {
            for (raw, variant) in &kept_variants {
                let _ = writeln!(out, "    /// DCB value: `{raw}`");
                let _ = writeln!(out, "    {variant},");
            }
        }
        let _ = writeln!(out, "    /// Unrecognised / newly-added enum value.");
        let _ = writeln!(out, "    {FALLBACK_VARIANT}(String),");
        let _ = writeln!(out, "}}\n");

        // Per-enum `from_dcb_str` — used by generated struct extract
        // impls to turn a DCB string back into the typed variant. Falls
        // through to `Unrecognized` for strings we didn't see at
        // generation time (newly-added enum values in a game patch).
        let _ = writeln!(out, "impl {sanitized} {{");
        out.push_str("    /// Resolve a raw DCB enum string to the typed variant.\n");
        out.push_str("    ///\n");
        out.push_str("    /// Unknown strings (including variants added in a game patch the\n");
        out.push_str("    /// generator didn't see) fall through to `Unrecognized(String)` for\n");
        out.push_str("    /// graceful forward compatibility.\n");
        out.push_str("    pub fn from_dcb_str(s: &str) -> Self {\n");
        if kept_variants.is_empty() {
            let _ = writeln!(out, "        Self::{FALLBACK_VARIANT}(s.to_string())");
        } else {
            out.push_str("        match s {\n");
            for (raw, variant) in &kept_variants {
                let escaped = raw.replace('\\', "\\\\").replace('"', "\\\"");
                let _ = writeln!(out, "            \"{escaped}\" => Self::{variant},");
            }
            let _ = writeln!(
                out,
                "            _ => Self::{FALLBACK_VARIANT}(s.to_string()),"
            );
            out.push_str("        }\n");
        }
        out.push_str("    }\n");
        out.push_str("}\n\n");
    }
    out
}

/// Emit `metadata.rs`.
pub fn emit_metadata(m: &Metadata) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Version and provenance constants for the generated module.\n\n");
    let _ = writeln!(
        out,
        "pub const GENERATED_GAME_VERSION: &str = {};",
        quote_string(&m.game_version)
    );
    let _ = writeln!(
        out,
        "pub const GENERATED_GAME_BRANCH: &str = {};",
        quote_string(&m.game_branch)
    );
    let _ = writeln!(
        out,
        "pub const GENERATED_BUILD_ID: &str = {};",
        quote_string(&m.build_id)
    );
    match &m.changelist {
        Some(cl) => {
            let _ = writeln!(
                out,
                "pub const GENERATED_CHANGELIST: Option<&str> = Some({});",
                quote_string(cl)
            );
        }
        None => {
            out.push_str("pub const GENERATED_CHANGELIST: Option<&str> = None;\n");
        }
    }
    let _ = writeln!(
        out,
        "pub const GENERATED_AT: &str = {};",
        quote_string(&m.generated_at)
    );
    let _ = writeln!(
        out,
        "pub const GENERATOR_VERSION: &str = {};",
        quote_string(&m.generator_version)
    );
    let _ = writeln!(out, "pub const SCHEMA_VERSION: u32 = {};", m.schema_version);
    let _ = writeln!(out, "pub const STRUCT_COUNT: usize = {};", m.struct_count);
    let _ = writeln!(out, "pub const ENUM_COUNT: usize = {};", m.enum_count);
    out
}

// ── Struct emission (unchanged core logic) ────────────────────────────────

fn types_file_header(feature_name: &str) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    let _ = writeln!(out, "//! Types for feature `{feature_name}`.\n");
    out.push_str("#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]\n");
    out.push_str("#![allow(clippy::too_many_arguments)]\n\n");
    out.push_str("use svarog_common::CigGuid;\n");
    out.push_str("use svarog_datacore::{Instance, Value};\n");
    // `LocaleKey` lives at the crate root (not inside `generated`), so pull
    // it in explicitly — `super::super::*` only reaches `generated::*`.
    out.push_str("use crate::{Builder, Extract, Handle, LocaleKey, Pooled};\n\n");
    // Bring all other generated types into scope for cross-feature Handle
    // refs and for the enum types emitted in `generated::enums`.
    out.push_str("use super::super::*;\n\n");
    out
}

#[allow(clippy::too_many_arguments)]
fn emit_struct(
    db: &DataCoreDatabase,
    struct_idx: u32,
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
    poly_bases: &PolyBaseSet,
    feature_cfgs: &FeatureCfgMap,
    feature_name: &str,
    cache: &PropertyCache<'_>,
    out: &mut String,
) {
    let Some(raw_name) = db.struct_name(struct_idx as usize) else {
        return;
    };
    let name = sanitize_struct_name(raw_name);
    if name.is_empty() {
        return;
    }
    let pool_field = pool_fields
        .get(&struct_idx)
        .expect("every emitted struct has a pool field name");
    let feature_field = feature_to_field_name(feature_name);
    // Per-type cfg gate — only `Multi`-assigned types get one.
    let cfg_attr: &str = feature_cfgs
        .get(&struct_idx)
        .and_then(|c| c.as_deref())
        .unwrap_or("");

    let def = &db.struct_definitions()[struct_idx as usize];
    let props = &cache[struct_idx as usize];

    let mut fields: Vec<FieldInfo> = Vec::new();
    let mut seen_field_names: HashMap<String, String> = HashMap::new();
    for prop in props {
        let Some(orig_name) = db.property_name(prop) else {
            continue;
        };
        if orig_name.is_empty() {
            continue;
        }
        let base_field_name = sanitize_field_name(orig_name);
        // Two DCB fields can legitimately differ only in case (e.g.
        // `R` and `r` on `TorusFieldGeom`), which both sanitize to the
        // same snake_case identifier. Rather than panic and require a
        // generator-wide fix, append a numeric suffix so the later
        // field becomes `r_2`, `r_3`, etc. The field names are already
        // opaque to consumers — they read through typed struct access —
        // so the disambiguation is safe.
        let mut field_name = base_field_name.clone();
        let mut disambiguator = 2u32;
        while seen_field_names.contains_key(&field_name) {
            field_name = format!("{base_field_name}_{disambiguator}");
            disambiguator += 1;
        }
        seen_field_names.insert(field_name.clone(), orig_name.to_string());
        let rust_type = rust_type_for(db, prop, emitted_names, poly_bases);
        let expr = extract_expr(db, prop, orig_name, emitted_names, poly_bases);
        fields.push(FieldInfo {
            orig_name: orig_name.to_string(),
            field_name,
            rust_type,
            expr,
            data_type: prop.get_data_type(),
            is_array: prop.is_array(),
        });
    }

    // Struct definition
    let _ = writeln!(out, "/// DCB type: `{raw_name}`");
    if def.parent_type_index >= 0
        && let Some(parent_name) = db.struct_name(def.parent_type_index as usize)
    {
        let _ = writeln!(out, "/// Inherits from: `{parent_name}`");
    }
    if !cfg_attr.is_empty() {
        let _ = writeln!(out, "{cfg_attr}");
    }
    let _ = writeln!(out, "pub struct {name} {{");
    for f in &fields {
        let dt = f.data_type.map(|d| d.as_str()).unwrap_or("unknown");
        let array_tag = if f.is_array { " (array)" } else { "" };
        let _ = writeln!(out, "    /// `{}` ({dt}{array_tag})", f.orig_name);
        let _ = writeln!(out, "    pub {}: {},", f.field_name, f.rust_type);
    }
    let _ = writeln!(out, "}}\n");

    // Pooled impl — references this feature's pool sub-struct
    if !cfg_attr.is_empty() {
        let _ = writeln!(out, "{cfg_attr}");
    }
    let _ = writeln!(out, "impl Pooled for {name} {{");
    let _ = writeln!(
        out,
        "    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {{ &pools.{feature_field}.{pool_field} }}"
    );
    let _ = writeln!(
        out,
        "    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {{ &mut pools.{feature_field}.{pool_field} }}"
    );
    let _ = writeln!(out, "}}\n");

    // Extract impl
    let uses_builder = fields.iter().any(|f| {
        matches!(
            f.data_type,
            Some(DataType::Class) | Some(DataType::StrongPointer) | Some(DataType::WeakPointer)
        )
    });
    let inst_name = if fields.is_empty() { "_inst" } else { "inst" };
    let b_name = if uses_builder { "b" } else { "_b" };
    if !cfg_attr.is_empty() {
        let _ = writeln!(out, "{cfg_attr}");
    }
    let _ = writeln!(out, "impl<'a> Extract<'a> for {name} {{");
    let _ = writeln!(
        out,
        "    const TYPE_NAME: &'static str = {};",
        quote_string(raw_name)
    );
    let _ = writeln!(
        out,
        "    fn extract({inst_name}: &Instance<'a>, {b_name}: &mut Builder<'a>) -> Self {{"
    );
    let _ = writeln!(out, "        Self {{");
    for f in &fields {
        let _ = writeln!(out, "            {}: {},", f.field_name, f.expr);
    }
    out.push_str("        }\n    }\n}\n\n");
}

struct FieldInfo {
    orig_name: String,
    field_name: String,
    rust_type: String,
    expr: String,
    data_type: Option<DataType>,
    is_array: bool,
}

// ── Naming helpers ────────────────────────────────────────────────────────

/// Feature name → Rust module name (replace `-` with `_`).
fn feature_to_mod_name(feature: &str) -> String {
    feature.replace('-', "_")
}

/// Feature name → CamelCase type suffix for pool/index structs.
fn feature_to_type_suffix(feature: &str) -> String {
    feature
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(c) => format!("{}{}", c.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect()
}

/// Feature name → field name in DataPools/RecordIndex.
fn feature_to_field_name(feature: &str) -> String {
    feature.replace('-', "_")
}

/// Generate `#[cfg(feature = "...")]` attribute string.
fn feature_cfg(feature: &str) -> String {
    format!("#[cfg(feature = \"{feature}\")]")
}

/// Resolve the `#[cfg(...)]` attribute that gates the **module** owning a
/// given type. Returns `None` for modules that are always compiled.
///
/// - `core` → None (truly unconditional)
/// - `multi-feature` → None (always compiled; per-type cfgs inside cover
///   the real feature gating)
/// - `dormant` → `Some("#[cfg(feature = \"dormant\")]")`
/// - any other feature `f` → `Some("#[cfg(feature = \"f\")]")`
fn feature_module_cfg(feature: &str) -> Option<String> {
    if feature == "core" || feature == MULTI_FEATURE_MODULE {
        None
    } else {
        Some(feature_cfg(feature))
    }
}

// ── Type mapping ──────────────────────────────────────────────────────────

/// For an `EnumChoice` property, resolve the Rust type name of the
/// generated enum by looking up the DCB enum index (which is stashed in
/// `prop.struct_index` — the same field is dual-used for enum-typed
/// properties, see svarog's c_header.rs).
///
/// Returns `None` when the enum index is out of range or the sanitized
/// type name is empty; callers should fall back to `String` in that case
/// so code generation stays robust across schema oddities.
fn enum_type_name(db: &DataCoreDatabase, prop: &DataCorePropertyDefinition) -> Option<String> {
    let enum_idx = prop.struct_index as usize;
    let name = db.enum_name(enum_idx)?;
    let sanitized = sanitize_struct_name(name);
    if sanitized.is_empty() {
        None
    } else {
        Some(sanitized)
    }
}

fn rust_type_for(
    db: &DataCoreDatabase,
    prop: &DataCorePropertyDefinition,
    emitted_names: &EmittedStructNames,
    poly_bases: &PolyBaseSet,
) -> String {
    let data_type = prop.get_data_type();
    let is_array = prop.is_array();

    let elem_type: String = match data_type {
        Some(DataType::Boolean) => "bool".into(),
        Some(DataType::SByte) | Some(DataType::Int16) | Some(DataType::Int32) => "i32".into(),
        Some(DataType::Int64) => "i64".into(),
        Some(DataType::Byte) | Some(DataType::UInt16) | Some(DataType::UInt32) => "u32".into(),
        Some(DataType::UInt64) => "u64".into(),
        Some(DataType::Single) => "f32".into(),
        Some(DataType::Double) => "f64".into(),
        Some(DataType::String) => "String".into(),
        Some(DataType::Locale) => "LocaleKey".into(),
        Some(DataType::EnumChoice) => enum_type_name(db, prop).unwrap_or_else(|| "String".into()),
        Some(DataType::Guid) => "CigGuid".into(),
        Some(DataType::Class) => {
            // Inline Class is monomorphic (wire format has no discriminator).
            let target_idx = prop.struct_index as u32;
            match emitted_names.get(&target_idx) {
                Some(name) => format!("Handle<{name}>"),
                None => "CigGuid".into(),
            }
        }
        Some(DataType::StrongPointer) | Some(DataType::WeakPointer) => {
            // Pointers can be polymorphic. If the declared target has
            // emitted descendants, use the generated tagged enum.
            let target_idx = prop.struct_index as u32;
            match emitted_names.get(&target_idx) {
                Some(name) if poly_bases.contains(&target_idx) => format!("{name}Ptr"),
                Some(name) => format!("Handle<{name}>"),
                None => "CigGuid".into(),
            }
        }
        Some(DataType::Reference) => "CigGuid".into(),
        None => {
            let idx = { prop.struct_index };
            panic!(
                "sc-generator: unknown DataType on property `{}` (struct_index {idx}). \
                 svarog's DataType enum may have grown a new variant — update \
                 rust_type_for/extract_expr to handle it.",
                db.property_name(prop).unwrap_or("<unknown>"),
            )
        }
    };

    if is_array {
        format!("Vec<{elem_type}>")
    } else {
        match data_type {
            Some(DataType::Class) | Some(DataType::StrongPointer) | Some(DataType::WeakPointer) => {
                format!("Option<{elem_type}>")
            }
            Some(DataType::Reference) => format!("Option<{elem_type}>"),
            _ => elem_type,
        }
    }
}

fn extract_expr(
    db: &DataCoreDatabase,
    prop: &DataCorePropertyDefinition,
    orig_name: &str,
    emitted_names: &EmittedStructNames,
    poly_bases: &PolyBaseSet,
) -> String {
    let data_type = prop.get_data_type();
    let is_array = prop.is_array();
    let escaped = orig_name.replace('\\', "\\\\").replace('"', "\\\"");

    if is_array {
        return array_expr(db, prop, &escaped, emitted_names, poly_bases);
    }

    match data_type {
        Some(DataType::Boolean) => format!("inst.get_bool(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::SByte) | Some(DataType::Int16) | Some(DataType::Int32) => {
            format!("inst.get_i32(\"{escaped}\").unwrap_or_default()")
        }
        Some(DataType::Int64) => format!("inst.get_i64(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::Byte) | Some(DataType::UInt16) | Some(DataType::UInt32) => {
            format!("inst.get_u32(\"{escaped}\").unwrap_or_default()")
        }
        Some(DataType::UInt64) => {
            format!("inst.get(\"{escaped}\").and_then(|v| v.as_u64()).unwrap_or_default()")
        }
        Some(DataType::Single) => format!("inst.get_f32(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::Double) => format!("inst.get_f64(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::String) => {
            format!("inst.get_str(\"{escaped}\").map(String::from).unwrap_or_default()")
        }
        Some(DataType::Locale) => {
            format!("inst.get_str(\"{escaped}\").map(LocaleKey::from).unwrap_or_default()")
        }
        Some(DataType::EnumChoice) => match enum_type_name(db, prop) {
            Some(ty) => {
                format!("{ty}::from_dcb_str(inst.get_str(\"{escaped}\").unwrap_or(\"\"))")
            }
            None => {
                // Unresolvable enum type — keep the legacy String fallback
                // so codegen stays robust on schema oddities.
                format!("inst.get_str(\"{escaped}\").map(String::from).unwrap_or_default()")
            }
        },
        Some(DataType::Guid) => format!("inst.get_guid(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::Class) => {
            // Inline Class: monomorphic. Handle only Value::Class variant;
            // ClassRef/pointer variants are impossible here by wire format.
            let target_idx = prop.struct_index as u32;
            let Some(target_name) = emitted_names.get(&target_idx) else {
                return "None".into();
            };
            format!(
                "match inst.get(\"{escaped}\") {{\n                Some(Value::Class {{ struct_index, data }}) => Some(b.alloc_nested::<{target_name}>(Instance::from_inline_data(b.db, struct_index, data), false)),\n                _ => None,\n            }}"
            )
        }
        Some(DataType::StrongPointer) | Some(DataType::WeakPointer) => {
            let target_idx = prop.struct_index as u32;
            let Some(target_name) = emitted_names.get(&target_idx) else {
                return "None".into();
            };
            if poly_bases.contains(&target_idx) {
                format!(
                    "match inst.get(\"{escaped}\") {{\n                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some({target_name}Ptr::from_ref(b, r)),\n                _ => None,\n            }}"
                )
            } else {
                format!(
                    "match inst.get(\"{escaped}\") {{\n                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<{target_name}>(b.db.instance(r.struct_index, r.instance_index), true)),\n                _ => None,\n            }}"
                )
            }
        }
        Some(DataType::Reference) => {
            format!("inst.get(\"{escaped}\").and_then(|v| v.as_record_ref()).map(|r| r.guid)")
        }
        None => {
            let idx = { prop.struct_index };
            panic!(
                "sc-generator: unknown DataType on property `{orig_name}` (struct_index {idx}). \
                 svarog's DataType enum may have grown a new variant — update \
                 rust_type_for/extract_expr to handle it."
            )
        }
    }
}

fn array_expr(
    db: &DataCoreDatabase,
    prop: &DataCorePropertyDefinition,
    escaped: &str,
    emitted_names: &EmittedStructNames,
    poly_bases: &PolyBaseSet,
) -> String {
    let data_type = prop.get_data_type();
    let inner = match data_type {
        Some(DataType::Boolean) => "v.as_bool()".to_string(),
        Some(DataType::SByte) | Some(DataType::Int16) | Some(DataType::Int32) => {
            "v.as_i32()".to_string()
        }
        Some(DataType::Int64) => "v.as_i64()".to_string(),
        Some(DataType::Byte) | Some(DataType::UInt16) | Some(DataType::UInt32) => {
            "v.as_u32()".to_string()
        }
        Some(DataType::UInt64) => "v.as_u64()".to_string(),
        Some(DataType::Single) => "v.as_f32()".to_string(),
        Some(DataType::Double) => "v.as_f64()".to_string(),
        Some(DataType::String) => "v.as_str().map(String::from)".to_string(),
        Some(DataType::Locale) => "v.as_str().map(LocaleKey::from)".to_string(),
        Some(DataType::EnumChoice) => match enum_type_name(db, prop) {
            Some(ty) => format!("v.as_str().map({ty}::from_dcb_str)"),
            None => "v.as_str().map(String::from)".to_string(),
        },
        Some(DataType::Guid) => "v.as_guid()".to_string(),
        Some(DataType::Reference) => {
            "if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }".to_string()
        }
        Some(DataType::Class) => {
            // Array Class elements are monomorphic (all elements of the
            // declared type, fixed stride). ClassRef is the only variant
            // we see for these.
            let target_idx = prop.struct_index as u32;
            let Some(target_name) = emitted_names.get(&target_idx) else {
                return format!(
                    "{{ let _ = inst.get_array(\"{escaped}\"); Vec::<CigGuid>::new() }}"
                );
            };
            format!(
                "match v {{\n                        Value::Class {{ struct_index, data }} => Some(b.alloc_nested::<{target_name}>(Instance::from_inline_data(b.db, struct_index, data), false)),\n                        Value::ClassRef(r) => Some(b.alloc_nested::<{target_name}>(b.db.instance(r.struct_index, r.instance_index), true)),\n                        _ => None,\n                    }}"
            )
        }
        Some(DataType::StrongPointer) | Some(DataType::WeakPointer) => {
            let target_idx = prop.struct_index as u32;
            let Some(target_name) = emitted_names.get(&target_idx) else {
                return format!(
                    "{{ let _ = inst.get_array(\"{escaped}\"); Vec::<CigGuid>::new() }}"
                );
            };
            if poly_bases.contains(&target_idx) {
                format!(
                    "match v {{\n                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some({target_name}Ptr::from_ref(b, r)),\n                        _ => None,\n                    }}"
                )
            } else {
                format!(
                    "match v {{\n                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<{target_name}>(b.db.instance(r.struct_index, r.instance_index), true)),\n                        _ => None,\n                    }}"
                )
            }
        }
        None => {
            let idx = { prop.struct_index };
            panic!(
                "sc-generator: unknown DataType on array property `{}` (struct_index {idx}). \
                 svarog's DataType enum may have grown a new variant — update \
                 rust_type_for/extract_expr/array_expr to handle it.",
                db.property_name(prop).unwrap_or("<unknown>"),
            )
        }
    };
    format!(
        "inst.get_array(\"{escaped}\")\n                .map(|arr| arr.filter_map(|v| {inner}).collect())\n                .unwrap_or_default()"
    )
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn collect_full_properties(
    db: &DataCoreDatabase,
    struct_idx: u32,
) -> Vec<&DataCorePropertyDefinition> {
    // `db.get_struct_properties` already walks the parent_type_index
    // chain internally and returns the full inherited property list in
    // `[root → ... → self]` order. Don't wrap it in a recursive walker
    // that also walks parents, or each ancestor's properties end up in
    // the result twice (an earlier version of this function did exactly
    // that and produced the `requestName`/`requestName` collisions on
    // structs like `ActivityBehaviorRequest_Animate`).
    db.get_struct_properties(struct_idx as usize)
}

fn quote_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{{{:x}}}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
