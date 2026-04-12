//! Code emission — turns parsed DCB schema into Rust source strings.
//!
//! Emits a **flat pool** model organized by **feature**: each auto-feature
//! gets its own module directory containing type definitions, pool storage,
//! and record index. The top-level `DataPools` / `RecordIndex` compose
//! per-feature sub-structs behind `#[cfg(feature = "...")]` gates.
//!
//! Feature classification is done by `crate::features::classify_features`.

use std::collections::{BTreeMap, HashSet, VecDeque};
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

/// Compute the set of struct indices reachable from top-level records.
pub fn compute_reachable_struct_indices(db: &DataCoreDatabase) -> HashSet<u32> {
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
        let props = collect_full_properties(db, idx);
        for prop in props {
            let data_type = prop.get_data_type();
            if matches!(
                data_type,
                Some(DataType::Class)
                    | Some(DataType::StrongPointer)
                    | Some(DataType::WeakPointer)
            ) {
                let target = prop.struct_index as u32;
                if target as usize >= db.struct_definitions().len() {
                    continue;
                }
                if reachable.insert(target) {
                    queue.push_back(target);
                }
            }
        }
    }

    // Self-check: verify the closure is actually closed.
    for &idx in &reachable {
        for prop in collect_full_properties(db, idx) {
            if matches!(
                prop.get_data_type(),
                Some(DataType::Class)
                    | Some(DataType::StrongPointer)
                    | Some(DataType::WeakPointer)
            ) {
                let target = prop.struct_index as u32;
                if (target as usize) < db.struct_definitions().len() && !reachable.contains(&target)
                {
                    let from = db.struct_name(idx as usize).unwrap_or("<?>");
                    let to = db.struct_name(target as usize).unwrap_or("<?>");
                    let field = db.property_name(prop).unwrap_or("<?>");
                    panic!(
                        "reachability BFS hole: `{from}` (idx {idx}) has Class/Pointer field \
                         `{field}` → `{to}` (idx {target}), but target was not reached."
                    );
                }
            }
        }
    }

    reachable
}

/// Build the emitted struct name table (deduplicated, pruned).
pub fn compute_emitted_struct_names(db: &DataCoreDatabase) -> EmittedStructNames {
    let reachable = compute_reachable_struct_indices(db);
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
/// Core and Multi types map to "core". Single types map to their feature.
pub fn compute_feature_assignment_map(
    emitted_names: &EmittedStructNames,
    feature_map: &FeatureMap,
) -> FeatureAssignmentMap {
    let mut out = FeatureAssignmentMap::new();
    for &struct_idx in emitted_names.keys() {
        let feature_name = match feature_map.struct_feature.get(struct_idx as usize).and_then(|a| a.as_ref()) {
            Some(FeatureAssignment::Core) => "core".to_string(),
            Some(FeatureAssignment::Single(f)) => f.clone(),
            Some(FeatureAssignment::Multi(_)) => "core".to_string(),
            None => "core".to_string(),
        };
        out.insert(struct_idx, feature_name);
    }
    out
}

// ── Per-feature emission ──────────────────────────────────────────────────

/// Emit a feature's `types.rs` file: struct definitions + Extract + Pooled impls.
pub fn emit_feature_types(
    db: &DataCoreDatabase,
    feature_name: &str,
    struct_indices: &[u32],
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
) -> String {
    let mut out = types_file_header(feature_name);
    for &struct_idx in struct_indices {
        emit_struct(db, struct_idx, emitted_names, pool_fields, feature_name, &mut out);
    }
    out
}

/// Emit a feature's pool sub-struct.
pub fn emit_feature_pools(
    feature_name: &str,
    struct_indices: &[u32],
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
) -> String {
    let suffix = feature_to_type_suffix(feature_name);
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_snake_case, dead_code, unused_imports)]\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n");
    out.push_str("use super::super::*;\n\n");

    let _ = writeln!(out, "/// Pool storage for the `{feature_name}` feature.");
    out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
    let _ = writeln!(out, "pub struct {suffix}Pools {{");
    for &struct_idx in struct_indices {
        if let (Some(type_name), Some(field)) =
            (emitted_names.get(&struct_idx), pool_fields.get(&struct_idx))
        {
            out.push_str("    #[serde(default)]\n");
            let _ = writeln!(out, "    pub {field}: Vec<Option<{type_name}>>,");
        }
    }
    out.push_str("}\n");
    out
}

/// Emit a feature's record index sub-struct (only record types).
pub fn emit_feature_index(
    db: &DataCoreDatabase,
    feature_name: &str,
    record_struct_indices: &[u32],
    emitted_names: &EmittedStructNames,
) -> Option<String> {
    // Filter to record types that belong to this feature.
    if record_struct_indices.is_empty() {
        return None;
    }

    let suffix = feature_to_type_suffix(feature_name);
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_snake_case, dead_code, unused_imports)]\n\n");
    out.push_str("use std::collections::HashMap;\n");
    out.push_str("use serde::{Deserialize, Serialize};\n");
    out.push_str("use svarog_common::CigGuid;\n");
    out.push_str("use crate::Handle;\n");
    out.push_str("use super::super::*;\n\n");

    let _ = writeln!(out, "/// Record index for the `{feature_name}` feature.");
    out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
    let _ = writeln!(out, "pub struct {suffix}Index {{");

    let mut used: HashSet<String> = HashSet::new();
    let mut fields: Vec<(String, String)> = Vec::new();
    for &struct_idx in record_struct_indices {
        let Some(type_name) = emitted_names.get(&struct_idx) else { continue };
        let base = sanitize_field_name(type_name);
        let mut field = base.clone();
        let mut sfx = 2u32;
        while !used.insert(field.clone()) {
            field = format!("{base}_{sfx}");
            sfx += 1;
        }
        out.push_str("    #[serde(default)]\n");
        let _ = writeln!(out, "    pub {field}: HashMap<CigGuid, Handle<{type_name}>>,");
        fields.push((field, type_name.clone()));
    }
    out.push_str("}\n\n");

    // len() / is_empty()
    let _ = writeln!(out, "impl {suffix}Index {{");
    out.push_str("    pub fn len(&self) -> usize {\n");
    for (i, (field, _)) in fields.iter().enumerate() {
        if i == 0 {
            let _ = write!(out, "        self.{field}.len()");
        } else {
            let _ = write!(out, "\n            + self.{field}.len()");
        }
    }
    if fields.is_empty() {
        out.push_str("        0");
    }
    out.push_str("\n    }\n\n");
    out.push_str("    pub fn is_empty(&self) -> bool { self.len() == 0 }\n");
    out.push_str("}\n");

    Some(out)
}

/// Emit a feature's `mod.rs`.
pub fn emit_feature_mod(feature_name: &str, has_index: bool) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    let _ = writeln!(out, "//! Feature module: `{feature_name}`\n");
    out.push_str("pub mod types;\n");
    out.push_str("pub mod pools;\n");
    if has_index {
        out.push_str("pub mod index;\n");
    }
    out.push('\n');
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
    out.push_str("use serde::{Deserialize, Serialize};\n\n");

    out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
    out.push_str("pub struct DataPools {\n");
    for feature in feature_names {
        let suffix = feature_to_type_suffix(feature);
        let field = feature_to_field_name(feature);
        let mod_name = feature_to_mod_name(feature);
        if feature == "core" {
            out.push_str("    #[serde(default)]\n");
            let _ = writeln!(out, "    pub {field}: super::{mod_name}::{suffix}Pools,");
        } else {
            let cfg = feature_cfg(feature);
            let _ = writeln!(out, "    {cfg}");
            out.push_str("    #[serde(default)]\n");
            let _ = writeln!(out, "    pub {field}: super::{mod_name}::{suffix}Pools,");
        }
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
    out.push_str("use serde::{Deserialize, Serialize};\n\n");

    out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
    out.push_str("pub struct RecordIndex {\n");
    for feature in features_with_index {
        let suffix = feature_to_type_suffix(feature);
        let field = feature_to_field_name(feature);
        let mod_name = feature_to_mod_name(feature);
        if feature == "core" {
            out.push_str("    #[serde(default)]\n");
            let _ = writeln!(out, "    pub {field}: super::{mod_name}::{suffix}Index,");
        } else {
            let cfg = feature_cfg(feature);
            let _ = writeln!(out, "    {cfg}");
            out.push_str("    #[serde(default)]\n");
            let _ = writeln!(out, "    pub {field}: super::{mod_name}::{suffix}Index,");
        }
    }
    out.push_str("}\n\n");

    // len() / is_empty()
    out.push_str("impl RecordIndex {\n");
    out.push_str("    pub fn len(&self) -> usize {\n");
    if features_with_index.is_empty() {
        out.push_str("        0\n");
    } else {
        for (i, feature) in features_with_index.iter().enumerate() {
            let field = feature_to_field_name(feature);
            let prefix = if feature == "core" {
                String::new()
            } else {
                format!("#[cfg(feature = \"{feature}\")] ")
            };
            if i == 0 && feature == "core" {
                let _ = write!(out, "        self.{field}.len()");
            } else if feature == "core" {
                let _ = write!(out, "\n            + self.{field}.len()");
            } else {
                // cfg'd features contribute 0 when disabled.
                // Use a block expression to handle cfg.
                let _ = write!(out, "\n            + {{ {prefix}{{ return self.{field}.len() }} 0 }}");
            }
        }
        // Actually this cfg approach is messy. Let's use a simpler pattern.
        // Rewrite: just sum unconditionally for core, and cfg-gate feature additions.
    }
    // Simplified: just count core, features add via cfg blocks
    out.clear();
    out.push_str(FILE_HEADER);
    out.push_str("//! Top-level `RecordIndex` composing per-feature index sub-structs.\n\n");
    out.push_str("#![allow(unused_imports)]\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");

    out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
    out.push_str("pub struct RecordIndex {\n");
    for feature in features_with_index {
        let suffix = feature_to_type_suffix(feature);
        let field = feature_to_field_name(feature);
        let mod_name = feature_to_mod_name(feature);
        if feature == "core" {
            out.push_str("    #[serde(default)]\n");
            let _ = writeln!(out, "    pub {field}: super::{mod_name}::{suffix}Index,");
        } else {
            let cfg = feature_cfg(feature);
            let _ = writeln!(out, "    {cfg}");
            out.push_str("    #[serde(default)]\n");
            let _ = writeln!(out, "    pub {field}: super::{mod_name}::{suffix}Index,");
        }
    }
    out.push_str("}\n\n");

    out.push_str("impl RecordIndex {\n");
    out.push_str("    pub fn len(&self) -> usize {\n");
    out.push_str("        let mut n = 0;\n");
    for feature in features_with_index {
        let field = feature_to_field_name(feature);
        if feature == "core" {
            let _ = writeln!(out, "        n += self.{field}.len();");
        } else {
            let cfg = feature_cfg(feature);
            let _ = writeln!(out, "        {cfg}");
            let _ = writeln!(out, "        {{ n += self.{field}.len(); }}");
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
    pool_fields: &PoolFieldNames,
    feature_assignments: &FeatureAssignmentMap,
) -> String {
    // Discover record types.
    let mut record_struct_indices: Vec<u32> = Vec::new();
    let mut seen: HashSet<u32> = HashSet::new();
    for record in db.records() {
        let struct_idx = record.struct_index;
        if struct_idx < 0 { continue; }
        let struct_idx = struct_idx as u32;
        if !seen.insert(struct_idx) { continue; }
        if !emitted_names.contains_key(&struct_idx) { continue; }
        record_struct_indices.push(struct_idx);
    }
    record_struct_indices.sort_unstable();

    // Build entries: (struct_idx, type_name, field_name, feature_name).
    let mut used: HashSet<String> = HashSet::new();
    let mut entries: Vec<(u32, String, String, String)> = Vec::new();
    for struct_idx in &record_struct_indices {
        let type_name = emitted_names.get(struct_idx).unwrap().clone();
        let base = sanitize_field_name(&type_name);
        let mut field = base.clone();
        let mut suffix = 2u32;
        while !used.insert(field.clone()) {
            field = format!("{base}_{suffix}");
            suffix += 1;
        }
        let feature = feature_assignments.get(struct_idx).cloned().unwrap_or("core".into());
        entries.push((*struct_idx, type_name, field, feature));
    }

    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]\n\n");
    out.push_str("use std::collections::HashMap;\n");
    out.push_str("use serde::{Deserialize, Serialize};\n");
    out.push_str("use svarog_common::CigGuid;\n");
    out.push_str("use svarog_datacore::Instance;\n");
    out.push_str("use crate::{Builder, Extract, Handle};\n");
    out.push_str("use super::*;\n\n");

    // RecordStore struct
    out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
    out.push_str("pub struct RecordStore {\n");
    out.push_str("    #[serde(default)]\n");
    out.push_str("    pub pools: DataPools,\n");
    out.push_str("    #[serde(default)]\n");
    out.push_str("    pub records: RecordIndex,\n");
    out.push_str("}\n\n");

    out.push_str("impl RecordStore {\n");
    out.push_str("    pub fn new() -> Self { Self::default() }\n");
    out.push_str("    pub fn len(&self) -> usize { self.records.len() }\n");
    out.push_str("    pub fn is_empty(&self) -> bool { self.records.is_empty() }\n");
    out.push_str("}\n\n");

    // Per-record-type extractor functions
    out.push_str("type RecordExtractor<'a> = fn(&mut Builder<'a>, CigGuid, Instance<'a>);\n\n");
    for (_idx, type_name, field, feature) in &entries {
        let fn_name = format!("extract_record_{field}");
        let feature_field = feature_to_field_name(feature);
        if feature != "core" {
            let _ = writeln!(out, "{}", feature_cfg(feature));
        }
        let _ = writeln!(out, "fn {fn_name}<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {{");
        let _ = writeln!(out, "    let id: Handle<{type_name}> = Handle::new(b.alloc_record::<{type_name}>(inst, guid));");
        let _ = writeln!(out, "    b.records.{feature_field}.{field}.insert(guid, id);");
        let _ = writeln!(out, "}}\n");
    }

    // seed_database
    out.push_str("impl<'a> Builder<'a> {\n");
    out.push_str("    pub fn seed_database(&mut self) {\n");
    if !entries.is_empty() {
        out.push_str("        let n_structs = self.db.struct_definitions().len();\n");
        out.push_str("        let name_to_idx: HashMap<&str, usize> = (0..n_structs)\n");
        out.push_str("            .filter_map(|i| self.db.struct_name(i).map(|n| (n, i)))\n");
        out.push_str("            .collect();\n\n");
        out.push_str("        let mut dispatch: Vec<Option<RecordExtractor<'a>>> = vec![None; n_structs];\n");

        for (_idx, type_name, field, feature) in &entries {
            let fn_name = format!("extract_record_{field}");
            if feature != "core" {
                let _ = writeln!(out, "        {}", feature_cfg(feature));
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
        out.push_str("                .map(|&i| self.db.struct_name(i as usize).unwrap_or(\"<unnamed>\"))\n");
        out.push_str("                .collect();\n");
        out.push_str("            names.sort_unstable();\n");
        out.push_str("            panic!(\n");
        out.push_str("                \"sc-extract-generated: runtime DCB contains {} record type(s) \\\n");
        out.push_str("                 the generator doesn't know about — generated bindings are stale. \\\n");
        out.push_str("                 Regenerate with `cargo run -p sc-generator -- --p4k <path>`.\\n\\\n");
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
    out.push_str("pub mod metadata;\n\n");

    // Feature modules
    for feature in feature_names {
        let mod_name = feature_to_mod_name(feature);
        if feature == "core" {
            let _ = writeln!(out, "pub mod {mod_name};");
        } else {
            let _ = writeln!(out, "{}", feature_cfg(feature));
            let _ = writeln!(out, "pub mod {mod_name};");
        }
    }
    out.push('\n');

    out.push_str("pub mod data_pools;\n");
    out.push_str("pub mod record_index;\n");
    out.push_str("pub mod record_store;\n\n");

    // Re-exports
    for feature in feature_names {
        let mod_name = feature_to_mod_name(feature);
        if feature == "core" {
            let _ = writeln!(out, "pub use {mod_name}::*;");
        } else {
            let _ = writeln!(out, "{}", feature_cfg(feature));
            let _ = writeln!(out, "pub use {mod_name}::*;");
        }
    }
    out.push_str("pub use enums::*;\n");
    out.push_str("pub use data_pools::DataPools;\n");
    out.push_str("pub use record_index::RecordIndex;\n");
    out.push_str("pub use record_store::RecordStore;\n\n");

    // Emit the feature list as a doc comment for reference.
    out.push_str("// ── Auto-generated feature list ────────────────────────────────────────\n");
    out.push_str("//\n");
    for feature in feature_names {
        if feature != "core" {
            let _ = writeln!(out, "// feature: {feature}");
        }
    }
    // Parent features
    for (parent, children) in &feature_map.parent_features {
        let _ = writeln!(out, "// parent: {parent} = [{}]", children.join(", "));
    }

    out
}

/// Emit `enums.rs` (unchanged — enums are always compiled).
pub fn emit_enums(db: &DataCoreDatabase) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_camel_case_types, dead_code)]\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");

    let mut emitted: HashSet<String> = HashSet::new();
    for enum_idx in 0..db.enum_definitions().len() {
        let Some(name) = db.enum_name(enum_idx) else { continue };
        let sanitized = sanitize_struct_name(name);
        if sanitized.is_empty() || !emitted.insert(sanitized.clone()) {
            continue;
        }
        let def = &db.enum_definitions()[enum_idx];
        let values = db.enum_options(def);
        const FALLBACK_VARIANT: &str = "Unrecognized";

        let _ = writeln!(out, "/// DCB enum: `{name}`");
        out.push_str("#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]\n");
        let _ = writeln!(out, "pub enum {sanitized} {{");
        let mut variant_names: HashSet<String> = HashSet::new();
        variant_names.insert(FALLBACK_VARIANT.to_string());
        if values.is_empty() {
            out.push_str("    Empty,\n");
        } else {
            for value in &values {
                let variant = sanitize_variant_name(value);
                if variant.is_empty() || !variant_names.insert(variant.clone()) {
                    continue;
                }
                let _ = writeln!(out, "    /// DCB value: `{value}`");
                let _ = writeln!(out, "    {variant},");
            }
        }
        let _ = writeln!(out, "    /// Unrecognised / newly-added enum value.");
        let _ = writeln!(out, "    {FALLBACK_VARIANT}(String),");
        let _ = writeln!(out, "}}\n");
    }
    out
}

/// Emit `metadata.rs`.
pub fn emit_metadata(m: &Metadata) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Version and provenance constants for the generated module.\n\n");
    let _ = writeln!(out, "pub const GENERATED_GAME_VERSION: &str = {};", quote_string(&m.game_version));
    let _ = writeln!(out, "pub const GENERATED_GAME_BRANCH: &str = {};", quote_string(&m.game_branch));
    let _ = writeln!(out, "pub const GENERATED_BUILD_ID: &str = {};", quote_string(&m.build_id));
    match &m.changelist {
        Some(cl) => { let _ = writeln!(out, "pub const GENERATED_CHANGELIST: Option<&str> = Some({});", quote_string(cl)); }
        None => { out.push_str("pub const GENERATED_CHANGELIST: Option<&str> = None;\n"); }
    }
    let _ = writeln!(out, "pub const GENERATED_AT: &str = {};", quote_string(&m.generated_at));
    let _ = writeln!(out, "pub const GENERATOR_VERSION: &str = {};", quote_string(&m.generator_version));
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
    out.push_str("use serde::{Deserialize, Serialize};\n");
    out.push_str("use svarog_common::CigGuid;\n");
    out.push_str("use svarog_datacore::{Instance, Value};\n");
    out.push_str("use crate::{Builder, Extract, Handle, Pooled};\n\n");
    // Bring all other generated types into scope for cross-feature Handle refs.
    out.push_str("use super::super::*;\n\n");
    out
}

fn emit_struct(
    db: &DataCoreDatabase,
    struct_idx: u32,
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
    feature_name: &str,
    out: &mut String,
) {
    let Some(raw_name) = db.struct_name(struct_idx as usize) else { return };
    let name = sanitize_struct_name(raw_name);
    if name.is_empty() { return; }
    let pool_field = pool_fields.get(&struct_idx).expect("every emitted struct has a pool field name");
    let feature_field = feature_to_field_name(feature_name);

    let def = &db.struct_definitions()[struct_idx as usize];
    let props = collect_full_properties(db, struct_idx);

    let mut fields: Vec<FieldInfo> = Vec::new();
    let mut seen_field_names: HashSet<String> = HashSet::new();
    for prop in &props {
        let Some(orig_name) = db.property_name(prop) else { continue };
        if orig_name.is_empty() { continue; }
        let field_name = sanitize_field_name(orig_name);
        if !seen_field_names.insert(field_name.clone()) { continue; }
        let rust_type = rust_type_for(db, prop, emitted_names);
        let expr = extract_expr(db, prop, orig_name, emitted_names);
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
    if def.parent_type_index >= 0 {
        if let Some(parent_name) = db.struct_name(def.parent_type_index as usize) {
            let _ = writeln!(out, "/// Inherits from: `{parent_name}`");
        }
    }
    out.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    let _ = writeln!(out, "pub struct {name} {{");
    for f in &fields {
        let dt = f.data_type.map(|d| d.as_str()).unwrap_or("unknown");
        let array_tag = if f.is_array { " (array)" } else { "" };
        let _ = writeln!(out, "    /// `{}` ({dt}{array_tag})", f.orig_name);
        out.push_str("    #[serde(default)]\n");
        let _ = writeln!(out, "    pub {}: {},", f.field_name, f.rust_type);
    }
    let _ = writeln!(out, "}}\n");

    // Pooled impl — references this feature's pool sub-struct
    let _ = writeln!(out, "impl Pooled for {name} {{");
    let _ = writeln!(out, "    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {{ &pools.{feature_field}.{pool_field} }}");
    let _ = writeln!(out, "    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {{ &mut pools.{feature_field}.{pool_field} }}");
    let _ = writeln!(out, "}}\n");

    // Extract impl
    let uses_builder = fields.iter().any(|f| matches!(f.data_type,
        Some(DataType::Class) | Some(DataType::StrongPointer) | Some(DataType::WeakPointer)));
    let inst_name = if fields.is_empty() { "_inst" } else { "inst" };
    let b_name = if uses_builder { "b" } else { "_b" };
    let _ = writeln!(out, "impl<'a> Extract<'a> for {name} {{");
    let _ = writeln!(out, "    const TYPE_NAME: &'static str = {};", quote_string(raw_name));
    let _ = writeln!(out, "    fn extract({inst_name}: &Instance<'a>, {b_name}: &mut Builder<'a>) -> Self {{");
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

// ── Type mapping (unchanged) ──────────────────────────────────────────────

fn rust_type_for(
    _db: &DataCoreDatabase,
    prop: &DataCorePropertyDefinition,
    emitted_names: &EmittedStructNames,
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
        Some(DataType::String) | Some(DataType::Locale) | Some(DataType::EnumChoice) => "String".into(),
        Some(DataType::Guid) => "CigGuid".into(),
        Some(DataType::Class) | Some(DataType::StrongPointer) | Some(DataType::WeakPointer) => {
            let target_idx = prop.struct_index as u32;
            match emitted_names.get(&target_idx) {
                Some(name) => format!("Handle<{name}>"),
                None => "CigGuid".into(),
            }
        }
        Some(DataType::Reference) => "CigGuid".into(),
        None => "String".into(),
    };

    if is_array {
        format!("Vec<{elem_type}>")
    } else {
        match data_type {
            Some(DataType::Class) | Some(DataType::StrongPointer) | Some(DataType::WeakPointer) => format!("Option<{elem_type}>"),
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
) -> String {
    let data_type = prop.get_data_type();
    let is_array = prop.is_array();
    let escaped = orig_name.replace('\\', "\\\\").replace('"', "\\\"");

    if is_array {
        return array_expr(db, prop, &escaped, emitted_names);
    }

    match data_type {
        Some(DataType::Boolean) => format!("inst.get_bool(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::SByte) | Some(DataType::Int16) | Some(DataType::Int32) => format!("inst.get_i32(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::Int64) => format!("inst.get_i64(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::Byte) | Some(DataType::UInt16) | Some(DataType::UInt32) => format!("inst.get_u32(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::UInt64) => format!("inst.get(\"{escaped}\").and_then(|v| v.as_u64()).unwrap_or_default()"),
        Some(DataType::Single) => format!("inst.get_f32(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::Double) => format!("inst.get_f64(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::String) | Some(DataType::Locale) | Some(DataType::EnumChoice) => format!("inst.get_str(\"{escaped}\").map(String::from).unwrap_or_default()"),
        Some(DataType::Guid) => format!("inst.get_guid(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::Class) | Some(DataType::StrongPointer) | Some(DataType::WeakPointer) => {
            let target_idx = prop.struct_index as u32;
            let Some(target_name) = emitted_names.get(&target_idx) else { return "None".into() };
            format!(
                "match inst.get(\"{escaped}\") {{\n                Some(Value::Class {{ struct_index, data }}) => Some(b.alloc_nested::<{target_name}>(Instance::from_inline_data(b.db, struct_index, data), false)),\n                Some(Value::ClassRef(r))\n                | Some(Value::StrongPointer(Some(r)))\n                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<{target_name}>(b.db.instance(r.struct_index, r.instance_index), true)),\n                _ => None,\n            }}"
            )
        }
        Some(DataType::Reference) => format!("inst.get(\"{escaped}\").and_then(|v| v.as_record_ref()).map(|r| r.guid)"),
        None => "String::new()".into(),
    }
}

fn array_expr(
    _db: &DataCoreDatabase,
    prop: &DataCorePropertyDefinition,
    escaped: &str,
    emitted_names: &EmittedStructNames,
) -> String {
    let data_type = prop.get_data_type();
    let inner = match data_type {
        Some(DataType::Boolean) => "v.as_bool()".to_string(),
        Some(DataType::SByte) | Some(DataType::Int16) | Some(DataType::Int32) => "v.as_i32()".to_string(),
        Some(DataType::Int64) => "v.as_i64()".to_string(),
        Some(DataType::Byte) | Some(DataType::UInt16) | Some(DataType::UInt32) => "v.as_u32()".to_string(),
        Some(DataType::UInt64) => "v.as_u64()".to_string(),
        Some(DataType::Single) => "v.as_f32()".to_string(),
        Some(DataType::Double) => "v.as_f64()".to_string(),
        Some(DataType::String) | Some(DataType::Locale) | Some(DataType::EnumChoice) => "v.as_str().map(String::from)".to_string(),
        Some(DataType::Guid) => "v.as_guid()".to_string(),
        Some(DataType::Reference) => "if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }".to_string(),
        Some(DataType::Class) | Some(DataType::StrongPointer) | Some(DataType::WeakPointer) => {
            let target_idx = prop.struct_index as u32;
            let Some(target_name) = emitted_names.get(&target_idx) else {
                return format!("{{ let _ = inst.get_array(\"{escaped}\"); Vec::<CigGuid>::new() }}");
            };
            format!(
                "match v {{\n                        Value::Class {{ struct_index, data }} => Some(b.alloc_nested::<{target_name}>(Instance::from_inline_data(b.db, struct_index, data), false)),\n                        Value::ClassRef(r)\n                        | Value::StrongPointer(Some(r))\n                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<{target_name}>(b.db.instance(r.struct_index, r.instance_index), true)),\n                        _ => None,\n                    }}"
            )
        }
        None => "None".to_string(),
    };
    format!("inst.get_array(\"{escaped}\")\n                .map(|arr| arr.filter_map(|v| {inner}).collect())\n                .unwrap_or_default()")
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn collect_full_properties(db: &DataCoreDatabase, struct_idx: u32) -> Vec<&DataCorePropertyDefinition> {
    let mut out = Vec::new();
    collect_recursive(db, struct_idx, &mut out);
    out
}

fn collect_recursive<'a>(db: &'a DataCoreDatabase, struct_idx: u32, out: &mut Vec<&'a DataCorePropertyDefinition>) {
    let Some(def) = db.struct_definitions().get(struct_idx as usize) else { return };
    if def.parent_type_index >= 0 {
        collect_recursive(db, def.parent_type_index as u32, out);
    }
    for prop in db.get_struct_properties(struct_idx as usize) {
        out.push(prop);
    }
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
            c if c.is_control() => { let _ = write!(out, "\\u{{{:x}}}", c as u32); }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
