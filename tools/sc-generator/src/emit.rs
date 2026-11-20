//! Code emission — turns parsed DCB schema into Rust source strings.
//!
//! Emits a **flat pool** model organized by **feature**: each auto-feature
//! gets its own module directory containing type definitions, pool storage,
//! and record index. The top-level `DataPools` / `RecordIndex` compose
//! per-feature sub-structs behind `#[cfg(feature = "...")]` gates.
//!
//! Feature classification is done by `crate::features::classify_features`.

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::fmt::Write as _;

use svarog_datacore::structs::DataCorePropertyDefinition;
use svarog_datacore::{DataCoreDatabase, DataType};

use crate::features::FeatureAssignment;
use crate::model::{FieldDef, FieldTarget, Model, StructDef};
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
/// by wire-format construction: see `polymorphism.rs`).
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
    model: &Model,
) -> FeatureAssignmentMap {
    let mut out = FeatureAssignmentMap::new();
    for &id in emitted_names.keys() {
        let feature_name = match &model.structs[&id].assignment {
            FeatureAssignment::Core => "core".to_string(),
            FeatureAssignment::Dormant => DORMANT_MODULE.to_string(),
            FeatureAssignment::Single(f) => f.clone(),
            FeatureAssignment::Multi(_) => MULTI_FEATURE_MODULE.to_string(),
        };
        out.insert(id, feature_name);
    }
    out
}

/// Emitted Rust names for the model's structs, deduplicated in id order.
pub fn compute_model_struct_names(model: &Model) -> EmittedStructNames {
    let mut out: EmittedStructNames = BTreeMap::new();
    let mut seen: HashSet<String> = HashSet::new();
    for (&id, s) in &model.structs {
        let name = sanitize_struct_name(&s.raw_name);
        if name.is_empty() || !seen.insert(name.clone()) {
            continue;
        }
        out.insert(id, name);
    }
    out
}

/// Polymorphic bases of the model: declared pointer targets with at least
/// one emitted descendant. Same rule as [`compute_poly_bases`].
pub fn compute_model_poly_bases(model: &Model, emitted_names: &EmittedStructNames) -> PolyBaseSet {
    let mut out: PolyBaseSet = BTreeSet::new();
    for &id in emitted_names.keys() {
        for f in &model.structs[&id].fields {
            if !matches!(f.data_type, DataType::StrongPointer | DataType::WeakPointer) {
                continue;
            }
            let FieldTarget::Struct(Some(target)) = &f.target else {
                continue;
            };
            let Some(tid) = model.id(target) else {
                continue;
            };
            if !emitted_names.contains_key(&tid) {
                continue;
            }
            if let Some(kids) = model.descendants.get(&tid)
                && kids.iter().any(|k| emitted_names.contains_key(k))
            {
                out.insert(tid);
            }
        }
    }
    out
}

/// Rust field names of a struct, disambiguating DCB names that differ only
/// in case (`R` / `r` → `r`, `r_2`).
pub fn field_rust_names(s: &StructDef) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    s.fields
        .iter()
        .map(|f| {
            let base = sanitize_field_name(&f.orig_name);
            let mut name = base.clone();
            let mut n = 2u32;
            while !seen.insert(name.clone()) {
                name = format!("{base}_{n}");
                n += 1;
            }
            name
        })
        .collect()
}

/// Name of the enum emitted for a field whose target type differs between
/// builds.
pub fn shape_enum_name(struct_name: &str, field_name: &str) -> String {
    format!("{struct_name}_{field_name}")
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
pub fn compute_feature_cfg_map(emitted_names: &EmittedStructNames, model: &Model) -> FeatureCfgMap {
    let mut out = FeatureCfgMap::new();
    for &struct_idx in emitted_names.keys() {
        let cfg = match &model.structs[&struct_idx].assignment {
            FeatureAssignment::Multi(features) if !features.is_empty() => {
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
    model: &Model,
    feature_name: &str,
    struct_indices: &[u32],
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
    poly_bases: &PolyBaseSet,
    feature_cfgs: &FeatureCfgMap,
) -> String {
    let mut out = types_file_header(feature_name);
    for &struct_idx in struct_indices {
        emit_struct(
            model,
            struct_idx,
            emitted_names,
            pool_fields,
            poly_bases,
            feature_cfgs,
            feature_name,
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
    model: &Model,
    poly_bases: &PolyBaseSet,
    emitted_names: &EmittedStructNames,
    feature_assignments: &FeatureAssignmentMap,
    feature_cfgs: &FeatureCfgMap,
) -> String {
    let descendants = &model.descendants;
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
            model,
            base_idx,
            emitted_names,
            descendants,
            feature_assignments,
            feature_cfgs,
            &mut out,
        );
    }

    // Shape enums: fields whose target type differs between the builds in
    // the window. Absent from single-build output.
    for (&id, s) in &model.structs {
        let Some(struct_name) = emitted_names.get(&id) else {
            continue;
        };
        for (f, field_name) in s.fields.iter().zip(field_rust_names(s)) {
            let FieldTarget::Shape(targets) = &f.target else {
                continue;
            };
            let variants: Vec<(u32, &String, &str)> = targets
                .iter()
                .filter_map(|raw| {
                    let (tid, name) = target_name(model, emitted_names, raw)?;
                    Some((tid, name, raw.as_str()))
                })
                .collect();
            emit_shape_enum(
                model,
                &shape_enum_name(struct_name, &field_name),
                s,
                f,
                &variants,
                feature_assignments,
                feature_cfgs,
                &mut out,
            );
        }
    }

    out
}

/// The cfg a reference to type `idx` must carry: its own Multi cfg, else
/// its module's gate.
fn variant_cfg_for(
    idx: u32,
    feature_assignments: &FeatureAssignmentMap,
    feature_cfgs: &FeatureCfgMap,
) -> String {
    if let Some(Some(c)) = feature_cfgs.get(&idx) {
        return c.clone();
    }
    let feature = feature_assignments
        .get(&idx)
        .map(String::as_str)
        .unwrap_or("core");
    feature_module_cfg(feature).unwrap_or_default()
}

/// Emit one shape enum: a field whose declared target type differs between
/// builds. One variant per target type, dispatched on the runtime type name.
#[allow(clippy::too_many_arguments)]
fn emit_shape_enum(
    model: &Model,
    enum_name: &str,
    s: &StructDef,
    f: &FieldDef,
    variants: &[(u32, &String, &str)],
    feature_assignments: &FeatureAssignmentMap,
    feature_cfgs: &FeatureCfgMap,
    out: &mut String,
) {
    let _ = writeln!(
        out,
        "/// Field `{}.{}`, whose target type differs between builds",
        s.raw_name, f.orig_name
    );
    let _ = writeln!(out, "/// ({}).", model.labels(model.all_bits()));
    out.push_str("///\n");
    out.push_str("/// Each variant is one target type; the runtime type name picks it.\n");
    out.push_str("#[derive(Debug, Clone)]\n");
    let _ = writeln!(out, "pub enum {enum_name} {{");
    for (tid, name, _) in variants {
        let cfg = variant_cfg_for(*tid, feature_assignments, feature_cfgs);
        if !cfg.is_empty() {
            let _ = writeln!(out, "    {cfg}");
        }
        let _ = writeln!(out, "    {name}(Handle<{name}>),");
    }
    out.push_str("    /// A target type the generator does not know, or whose feature is off.\n");
    out.push_str("    Unknown { struct_index: u32 },\n");
    out.push_str("}\n\n");

    let _ = writeln!(out, "impl {enum_name} {{");
    out.push_str("    #[allow(dead_code)]\n");
    out.push_str(
        "    pub(crate) fn from_instance<'a>(b: &mut Builder<'a>, inst: svarog_datacore::Instance<'a>, pooled: bool) -> Self {\n",
    );
    out.push_str("        match inst.type_name() {\n");
    for (tid, name, raw) in variants {
        let cfg = variant_cfg_for(*tid, feature_assignments, feature_cfgs);
        let cfg_prefix = if cfg.is_empty() {
            String::new()
        } else {
            format!("{cfg} ")
        };
        let escaped = raw.replace('\\', "\\\\").replace('"', "\\\"");
        let _ = writeln!(
            out,
            "            {cfg_prefix}Some(\"{escaped}\") => Self::{name}(b.alloc_nested::<{name}>(inst, pooled)),",
        );
    }
    out.push_str("            _ => Self::Unknown { struct_index: inst.struct_index() },\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

/// Emit one polymorphic-base tagged enum + `from_ref` dispatch helper into
/// the shared `poly_enums.rs` file.
fn emit_poly_enum(
    model: &Model,
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
    let base_raw = model.structs[&base_idx].raw_name.as_str();
    let enum_name = format!("{base_name}Ptr");

    // Resolve the per-variant cfg attribute. A variant's cfg must match
    // the cfg the variant's target type carries at its definition site:
    // - Multi types: `#[cfg(any(feature = ...))]` (from `feature_cfgs`).
    // - Dormant types: `#[cfg(feature = "dormant")]` (inherited from
    //   the `dormant` module's declaration).
    // - Single-feature types: the feature's own cfg, because the whole
    //   feature module is gated at `pub mod X;` in `generated/mod.rs`.
    // - Core and `multi-feature` module types: no cfg.
    let variant_cfg = |idx: u32| variant_cfg_for(idx, feature_assignments, feature_cfgs);

    // Build variant list: (type_name, cfg_attr_or_empty, raw DCB name).
    // Includes the base itself as its own variant plus every emitted
    // descendant.
    let mut variants: Vec<(String, String, String)> = Vec::new();
    let mut deprecated: Vec<Option<String>> = Vec::new();
    variants.push((
        base_name.clone(),
        variant_cfg(base_idx),
        base_raw.to_string(),
    ));
    deprecated.push(model.struct_deprecation(&model.structs[&base_idx]));
    if let Some(kids) = descendants.get(&base_idx) {
        for &k in kids {
            let Some(kid_name) = emitted_names.get(&k) else {
                continue;
            };
            let kid = &model.structs[&k];
            variants.push((kid_name.clone(), variant_cfg(k), kid.raw_name.clone()));
            deprecated.push(model.struct_deprecation(kid));
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
    for ((type_name, cfg, _), note) in variants.iter().zip(&deprecated) {
        if let Some(note) = note {
            let _ = writeln!(out, "    #[deprecated(note = {})]", quote_string(note));
        }
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
    model: &Model,
    emitted_names: &EmittedStructNames,
    _pool_fields: &PoolFieldNames,
    feature_assignments: &FeatureAssignmentMap,
    feature_cfgs: &FeatureCfgMap,
) -> String {
    // Record types: emitted structs with top-level records in any build.
    let record_struct_indices: Vec<u32> = emitted_names
        .keys()
        .copied()
        .filter(|id| model.structs[id].records != 0)
        .collect();

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

    // Typed GUID -> Handle resolution. One impl per record type, gated by
    // the same cfg as the type's index field, delegating to the
    // `CigGuid -> Handle<T>` map populated by the extractor above. This is
    // the data behind `Datacore::resolve`, letting consumers re-enter the
    // typed surface after following a cross-record `Reference` instead of
    // dropping to the raw svarog layer.
    for (_idx, type_name, field, feature, cfg_attr) in &entries {
        let feature_field = feature_to_field_name(feature);
        if !cfg_attr.is_empty() {
            let _ = writeln!(out, "{cfg_attr}");
        }
        let _ = writeln!(out, "impl crate::RecordLookup for {type_name} {{");
        let _ = writeln!(
            out,
            "    fn lookup(index: &RecordIndex, guid: &CigGuid) -> Option<Handle<Self>> {{"
        );
        let _ = writeln!(
            out,
            "        index.{feature_field}.{field}.get(guid).copied()"
        );
        out.push_str("    }\n}\n\n");
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

        // Every DCB record-type name for which the generator emitted a
        // binding, regardless of feature gating. Used below to distinguish
        // feature-gated-away types (expected for narrow consumers, skipped
        // silently) from truly-unknown types (real generator staleness).
        // Release builds keep it too: they still skip unknown records, but
        // report them through `Builder::unknown_record_types` instead of
        // dropping them unseen. Sorted + deduped so the runtime lookup is a
        // binary search.
        out.push_str("        const KNOWN_RECORD_TYPE_NAMES: &[&str] = &[\n");
        let mut known_names: Vec<&str> = entries
            .iter()
            .map(|(idx, _, _, _, _)| model.structs[idx].raw_name.as_str())
            .collect();
        known_names.sort_unstable();
        known_names.dedup();
        for name in &known_names {
            let _ = writeln!(out, "            {:?},", name);
        }
        out.push_str("        ];\n\n");

        // `binary_search` only gives correct answers on a strictly-sorted
        // slice. The generator sorts + dedups before emitting, but a
        // regression there (or a manual edit to the generated file) would
        // silently turn the staleness check into a false-pass. Guard with
        // a debug-only invariant check.
        out.push_str("        #[cfg(debug_assertions)]\n");
        out.push_str("        debug_assert!(\n");
        out.push_str("            KNOWN_RECORD_TYPE_NAMES.windows(2).all(|w| w[0] < w[1]),\n");
        out.push_str("            \"sc-extract-generated: KNOWN_RECORD_TYPE_NAMES must be strictly sorted \\\n");
        out.push_str("             and deduplicated for binary_search to be correct — regenerate bindings.\",\n");
        out.push_str("        );\n\n");

        // Records with no enabled extractor, counted per struct index. Most
        // are feature-gated away; the name check after the loop runs once
        // per type rather than once per record.
        out.push_str("        let mut undispatched: HashMap<u32, usize> = HashMap::new();\n\n");

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
        out.push_str("                *undispatched.entry(struct_idx).or_default() += 1;\n");
        out.push_str("            }\n");
        out.push_str("        }\n\n");

        out.push_str("        let mut unknown: Vec<crate::UnknownRecordType> = undispatched\n");
        out.push_str("            .into_iter()\n");
        out.push_str("            .filter_map(|(idx, records)| {\n");
        out.push_str(
            "                let name = self.db.struct_name(idx as usize).unwrap_or(\"<unnamed>\");\n",
        );
        out.push_str(
            "                KNOWN_RECORD_TYPE_NAMES.binary_search(&name).is_err().then(|| {\n",
        );
        out.push_str(
            "                    crate::UnknownRecordType { name: name.to_string(), records }\n",
        );
        out.push_str("                })\n");
        out.push_str("            })\n");
        out.push_str("            .collect();\n");
        out.push_str("        unknown.sort_unstable_by(|a, b| a.name.cmp(&b.name));\n\n");

        out.push_str("        #[cfg(debug_assertions)]\n");
        out.push_str("        if !unknown.is_empty() {\n");
        out.push_str(
            "            let names: Vec<&str> = unknown.iter().map(|t| t.name.as_str()).collect();\n",
        );
        out.push_str("            panic!(\n");
        out.push_str(
            "                \"sc-extract-generated: runtime DCB contains {} record type(s) \\\n",
        );
        out.push_str(
            "                 with no generated binding in any feature — generated bindings \\\n",
        );
        out.push_str("                 are stale relative to this DCB. Regenerate with \\\n");
        out.push_str("                 `cargo run -p sc-generator -- --p4k <path>`.\\n\\\n");
        out.push_str("                 (Feature-gated-away record types are skipped silently; this list \\\n");
        out.push_str("                 contains only types the generator has never seen.)\\n\\\n");
        out.push_str("                 Unknown types: {:?}\",\n");
        out.push_str("                names.len(),\n");
        out.push_str("                names,\n");
        out.push_str("            );\n");
        out.push_str("        }\n");
        out.push_str("        self.unknown_record_types = unknown;\n");
    }
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Emit the top-level `mod.rs` with cfg'd feature module includes.
pub fn emit_top_mod(
    feature_names: &[String],
    parent_features: &BTreeMap<String, Vec<String>>,
) -> String {
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
    for (parent, children) in parent_features {
        let _ = writeln!(out, "// parent: {parent} = [{}]", children.join(", "));
    }

    out
}

/// Emit `enums.rs` — enum type definitions plus a `from_dcb_str`
/// associated function per enum that maps raw DCB strings back to the
/// Rust variant (falling through to `Unrecognized(String)` for values
/// not present at generation time).
pub fn emit_enums(model: &Model) -> String {
    const FALLBACK_VARIANT: &str = "Unrecognized";

    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_camel_case_types, dead_code)]\n\n");

    let mut emitted: HashSet<String> = HashSet::new();
    for e in &model.enums {
        let name = e.raw_name.as_str();
        let sanitized = sanitize_struct_name(name);
        if sanitized.is_empty() || !emitted.insert(sanitized.clone()) {
            continue;
        }

        // Collect (raw DCB value, sanitized Rust variant) pairs, applying
        // the same collision-skip logic as before. We pass over the list
        // once here so the type definition and the `from_dcb_str` impl
        // stay in lockstep.
        let mut variant_names: HashSet<String> = HashSet::new();
        variant_names.insert(FALLBACK_VARIANT.to_string());
        let mut kept_variants: Vec<(&str, String)> = Vec::new();
        let mut kept_builds: Vec<crate::model::BuildSet> = Vec::new();
        for (value, builds) in &e.values {
            let raw: &str = value;
            let variant = sanitize_variant_name(raw);
            if variant.is_empty() || !variant_names.insert(variant.clone()) {
                continue;
            }
            kept_variants.push((raw, variant));
            kept_builds.push(*builds);
        }

        let _ = writeln!(out, "/// DCB enum: `{name}`");
        out.push_str("#[derive(Debug, Clone, PartialEq, Eq, Hash)]\n");
        let _ = writeln!(out, "pub enum {sanitized} {{");
        if kept_variants.is_empty() {
            out.push_str("    Empty,\n");
        } else {
            for ((raw, variant), builds) in kept_variants.iter().zip(&kept_builds) {
                let _ = writeln!(out, "    /// DCB value: `{raw}`");
                if let Some(doc) = model.new_only_doc(*builds) {
                    let _ = writeln!(out, "    ///\n    /// {doc}");
                }
                if model.is_multi_build() && builds & model.newest_bit() == 0 {
                    let note = format!("absent from {}", model.newest_label());
                    let _ = writeln!(out, "    #[deprecated(note = {})]", quote_string(&note));
                }
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

        // Per-enum `as_dcb_str` — the inverse of `from_dcb_str`. Returns the
        // raw DCB string for each variant so callers can round-trip a typed
        // variant back to its on-disk form (e.g. serde adapters that store the
        // enum as its DCB string without giving the generated enum serde).
        // Built from the same `kept_variants` list so it stays in lockstep.
        out.push_str("\n    /// The raw DCB enum string for this variant (inverse of\n");
        out.push_str("    /// `from_dcb_str`; round-trips for serialization).\n");
        out.push_str("    pub fn as_dcb_str(&self) -> &str {\n");
        out.push_str("        match self {\n");
        if kept_variants.is_empty() {
            out.push_str("            Self::Empty => \"\",\n");
        } else {
            for (raw, variant) in &kept_variants {
                let escaped = raw.replace('\\', "\\\\").replace('"', "\\\"");
                let _ = writeln!(out, "            Self::{variant} => \"{escaped}\",");
            }
        }
        let _ = writeln!(out, "            Self::{FALLBACK_VARIANT}(s) => s,");
        out.push_str("        }\n");
        out.push_str("    }\n");

        out.push_str("}\n\n");
    }
    out
}

/// Emit `metadata.rs`.
pub fn emit_metadata(m: &Metadata, builds: &[String]) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Version and provenance constants for the generated module.\n");
    out.push_str("//!\n");
    out.push_str("//! The single-build constants describe the newest build in the window.\n\n");
    out.push_str("/// Every build the bindings cover, oldest first.\n");
    out.push_str("pub const GENERATED_BUILDS: &[&str] = &[");
    for (i, b) in builds.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&quote_string(b));
    }
    out.push_str("];\n");
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
    model: &Model,
    struct_idx: u32,
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
    poly_bases: &PolyBaseSet,
    feature_cfgs: &FeatureCfgMap,
    feature_name: &str,
    out: &mut String,
) {
    let def = &model.structs[&struct_idx];
    let raw_name = def.raw_name.as_str();
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

    // Two DCB fields can legitimately differ only in case (e.g. `R` and
    // `r` on `TorusFieldGeom`); `field_rust_names` suffixes the later one
    // (`r_2`). Field names are opaque to consumers, so that is safe.
    let mut fields: Vec<FieldInfo> = Vec::new();
    for (f, field_name) in def.fields.iter().zip(field_rust_names(def)) {
        let shape = shape_enum_name(&name, &field_name);
        let rust_type = rust_type_for(model, f, emitted_names, poly_bases, &shape);
        let expr = extract_expr(model, f, emitted_names, poly_bases, &shape);
        fields.push(FieldInfo {
            orig_name: f.orig_name.clone(),
            field_name,
            rust_type,
            expr,
            data_type: Some(f.data_type),
            is_array: f.is_array,
            builds: f.builds,
        });
    }

    // Struct definition
    let _ = writeln!(out, "/// DCB type: `{raw_name}`");
    if let Some(parent_name) = &def.parent {
        let _ = writeln!(out, "/// Inherits from: `{parent_name}`");
    }
    if let Some(doc) = model.new_only_doc(def.builds) {
        let _ = writeln!(out, "///\n/// {doc}");
    }
    if let Some(note) = model.struct_deprecation(def) {
        let _ = writeln!(out, "#[deprecated(note = {})]", quote_string(&note));
    }
    if !cfg_attr.is_empty() {
        let _ = writeln!(out, "{cfg_attr}");
    }
    let _ = writeln!(out, "pub struct {name} {{");
    for f in &fields {
        let dt = f.data_type.map(|d| d.as_str()).unwrap_or("unknown");
        let array_tag = if f.is_array { " (array)" } else { "" };
        let _ = writeln!(out, "    /// `{}` ({dt}{array_tag})", f.orig_name);
        // Field markers only where the field's builds differ from its
        // struct's; a whole new or dropped struct is marked once, above.
        if f.builds != def.builds {
            if let Some(doc) = model.new_only_doc(f.builds) {
                let _ = writeln!(out, "    ///\n    /// {doc}");
            }
            if f.builds & model.newest_bit() == 0 {
                let note = format!("absent from {}", model.newest_label());
                let _ = writeln!(out, "    #[deprecated(note = {})]", quote_string(&note));
            }
        }
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
    builds: crate::model::BuildSet,
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

/// Struct id and emitted Rust name of a struct-kind target.
fn target_name<'n>(
    model: &Model,
    emitted_names: &'n EmittedStructNames,
    raw: &str,
) -> Option<(u32, &'n String)> {
    let id = model.id(raw)?;
    emitted_names.get(&id).map(|n| (id, n))
}

fn rust_type_for(
    model: &Model,
    f: &FieldDef,
    emitted_names: &EmittedStructNames,
    poly_bases: &PolyBaseSet,
    shape: &str,
) -> String {
    let elem_type: String = match f.data_type {
        DataType::Boolean => "bool".into(),
        DataType::SByte | DataType::Int16 | DataType::Int32 => "i32".into(),
        DataType::Int64 => "i64".into(),
        DataType::Byte | DataType::UInt16 | DataType::UInt32 => "u32".into(),
        DataType::UInt64 => "u64".into(),
        DataType::Single => "f32".into(),
        DataType::Double => "f64".into(),
        DataType::String => "String".into(),
        DataType::Locale => "LocaleKey".into(),
        DataType::EnumChoice => match &f.target {
            FieldTarget::Enum(Some(ty)) => ty.clone(),
            _ => "String".into(),
        },
        DataType::Guid => "CigGuid".into(),
        DataType::Class | DataType::StrongPointer | DataType::WeakPointer => match &f.target {
            FieldTarget::Shape(_) => shape.to_string(),
            // Inline Class is monomorphic (wire format has no discriminator);
            // pointers whose declared target has emitted descendants use the
            // generated tagged enum.
            FieldTarget::Struct(Some(raw)) => match target_name(model, emitted_names, raw) {
                Some((id, name)) if f.data_type != DataType::Class && poly_bases.contains(&id) => {
                    format!("{name}Ptr")
                }
                Some((_, name)) => format!("Handle<{name}>"),
                None => "CigGuid".into(),
            },
            _ => "CigGuid".into(),
        },
        DataType::Reference => "CigGuid".into(),
    };

    if f.is_array {
        format!("Vec<{elem_type}>")
    } else {
        match f.data_type {
            DataType::Class
            | DataType::StrongPointer
            | DataType::WeakPointer
            | DataType::Reference => format!("Option<{elem_type}>"),
            _ => elem_type,
        }
    }
}

fn extract_expr(
    model: &Model,
    f: &FieldDef,
    emitted_names: &EmittedStructNames,
    poly_bases: &PolyBaseSet,
    shape: &str,
) -> String {
    let escaped = f.orig_name.replace('\\', "\\\\").replace('"', "\\\"");

    if f.is_array {
        return array_expr(model, f, &escaped, emitted_names, poly_bases, shape);
    }

    match f.data_type {
        DataType::Boolean => format!("inst.get_bool(\"{escaped}\").unwrap_or_default()"),
        DataType::SByte | DataType::Int16 | DataType::Int32 => {
            format!("inst.get_i32(\"{escaped}\").unwrap_or_default()")
        }
        DataType::Int64 => format!("inst.get_i64(\"{escaped}\").unwrap_or_default()"),
        DataType::Byte | DataType::UInt16 | DataType::UInt32 => {
            format!("inst.get_u32(\"{escaped}\").unwrap_or_default()")
        }
        DataType::UInt64 => {
            format!("inst.get(\"{escaped}\").and_then(|v| v.as_u64()).unwrap_or_default()")
        }
        DataType::Single => format!("inst.get_f32(\"{escaped}\").unwrap_or_default()"),
        DataType::Double => format!("inst.get_f64(\"{escaped}\").unwrap_or_default()"),
        DataType::String => {
            format!("inst.get_str(\"{escaped}\").map(String::from).unwrap_or_default()")
        }
        DataType::Locale => {
            format!("inst.get_str(\"{escaped}\").map(LocaleKey::from).unwrap_or_default()")
        }
        DataType::EnumChoice => match &f.target {
            FieldTarget::Enum(Some(ty)) => {
                format!("{ty}::from_dcb_str(inst.get_str(\"{escaped}\").unwrap_or(\"\"))")
            }
            // Unresolvable enum type — keep the String fallback so codegen
            // stays robust on schema oddities.
            _ => format!("inst.get_str(\"{escaped}\").map(String::from).unwrap_or_default()"),
        },
        DataType::Guid => format!("inst.get_guid(\"{escaped}\").unwrap_or_default()"),
        DataType::Class => {
            if let FieldTarget::Shape(_) = f.target {
                return format!(
                    "match inst.get(\"{escaped}\") {{\n                Some(Value::Class {{ struct_index, data }}) => Some({shape}::from_instance(b, Instance::from_inline_data(b.db, struct_index, data), false)),\n                _ => None,\n            }}"
                );
            }
            // Inline Class: monomorphic. Handle only Value::Class variant;
            // ClassRef/pointer variants are impossible here by wire format.
            let FieldTarget::Struct(Some(raw)) = &f.target else {
                return "None".into();
            };
            let Some((_, target_name)) = target_name(model, emitted_names, raw) else {
                return "None".into();
            };
            format!(
                "match inst.get(\"{escaped}\") {{\n                Some(Value::Class {{ struct_index, data }}) => Some(b.alloc_nested::<{target_name}>(Instance::from_inline_data(b.db, struct_index, data), false)),\n                _ => None,\n            }}"
            )
        }
        DataType::StrongPointer | DataType::WeakPointer => {
            if let FieldTarget::Shape(_) = f.target {
                return format!(
                    "match inst.get(\"{escaped}\") {{\n                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some({shape}::from_instance(b, b.db.instance(r.struct_index, r.instance_index), true)),\n                _ => None,\n            }}"
                );
            }
            let FieldTarget::Struct(Some(raw)) = &f.target else {
                return "None".into();
            };
            let Some((target_idx, target_name)) = target_name(model, emitted_names, raw) else {
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
        DataType::Reference => {
            format!("inst.get(\"{escaped}\").and_then(|v| v.as_record_ref()).map(|r| r.guid)")
        }
    }
}

fn array_expr(
    model: &Model,
    f: &FieldDef,
    escaped: &str,
    emitted_names: &EmittedStructNames,
    poly_bases: &PolyBaseSet,
    shape: &str,
) -> String {
    let inner = match f.data_type {
        DataType::Boolean => "v.as_bool()".to_string(),
        DataType::SByte | DataType::Int16 | DataType::Int32 => "v.as_i32()".to_string(),
        DataType::Int64 => "v.as_i64()".to_string(),
        DataType::Byte | DataType::UInt16 | DataType::UInt32 => "v.as_u32()".to_string(),
        DataType::UInt64 => "v.as_u64()".to_string(),
        DataType::Single => "v.as_f32()".to_string(),
        DataType::Double => "v.as_f64()".to_string(),
        DataType::String => "v.as_str().map(String::from)".to_string(),
        DataType::Locale => "v.as_str().map(LocaleKey::from)".to_string(),
        DataType::EnumChoice => match &f.target {
            FieldTarget::Enum(Some(ty)) => format!("v.as_str().map({ty}::from_dcb_str)"),
            _ => "v.as_str().map(String::from)".to_string(),
        },
        DataType::Guid => "v.as_guid()".to_string(),
        DataType::Reference => {
            "if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }".to_string()
        }
        DataType::Class => {
            if let FieldTarget::Shape(_) = f.target {
                format!(
                    "match v {{\n                        Value::Class {{ struct_index, data }} => Some({shape}::from_instance(b, Instance::from_inline_data(b.db, struct_index, data), false)),\n                        Value::ClassRef(r) => Some({shape}::from_instance(b, b.db.instance(r.struct_index, r.instance_index), true)),\n                        _ => None,\n                    }}"
                )
            } else {
                // Array Class elements are monomorphic (all elements of the
                // declared type, fixed stride). ClassRef is the only variant
                // we see for these.
                let target = match &f.target {
                    FieldTarget::Struct(Some(raw)) => target_name(model, emitted_names, raw),
                    _ => None,
                };
                let Some((_, target_name)) = target else {
                    return format!(
                        "{{ let _ = inst.get_array(\"{escaped}\"); Vec::<CigGuid>::new() }}"
                    );
                };
                format!(
                    "match v {{\n                        Value::Class {{ struct_index, data }} => Some(b.alloc_nested::<{target_name}>(Instance::from_inline_data(b.db, struct_index, data), false)),\n                        Value::ClassRef(r) => Some(b.alloc_nested::<{target_name}>(b.db.instance(r.struct_index, r.instance_index), true)),\n                        _ => None,\n                    }}"
                )
            }
        }
        DataType::StrongPointer | DataType::WeakPointer => {
            if let FieldTarget::Shape(_) = f.target {
                format!(
                    "match v {{\n                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some({shape}::from_instance(b, b.db.instance(r.struct_index, r.instance_index), true)),\n                        _ => None,\n                    }}"
                )
            } else {
                let target = match &f.target {
                    FieldTarget::Struct(Some(raw)) => target_name(model, emitted_names, raw),
                    _ => None,
                };
                let Some((target_idx, target_name)) = target else {
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
