//! Code emission — turns parsed DCB schema into Rust source strings.
//!
//! The generator emits a **flat pool** model: every struct type has its own
//! `Vec<Option<T>>` inside [`DataPools`], and every nested `Class` /
//! `StrongPointer` / `WeakPointer` field is stored as a `u32` slot index
//! wrapped in a per-type `TId` newtype. Materialisation is driven iteratively
//! by [`Builder`] via the [`Extract`] trait; nothing recurses through the
//! call stack. See `docs/codegen.md` for the design.

use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fmt::Write as _;

use svarog_datacore::structs::DataCorePropertyDefinition;
use svarog_datacore::{DataCoreDatabase, DataType};

use crate::naming::{sanitize_field_name, sanitize_struct_name, sanitize_variant_name};
use crate::pipeline::Metadata;

/// Keys are bucket suffixes (`"a"`, `"b"`, ..., `"other"`); values are the
/// full Rust source for each `types_<bucket>.rs` file.
pub type TypesBuckets = BTreeMap<String, String>;

/// Map from DCB struct index to the sanitized Rust type name that the
/// generator actually emits for it. Built once per run and shared between
/// [`emit_types`], [`emit_data_pools`], [`emit_record_index`], and
/// [`emit_record_store`] so every stage sees the same name-dedup decisions.
///
/// Struct indices that don't appear in this map were skipped (unnamed,
/// collision on sanitized name, etc.) and must not be referenced in any
/// emitted output.
pub type EmittedStructNames = BTreeMap<u32, String>;

/// Map from DCB struct index to the field name used in [`DataPools`] /
/// [`RecordIndex`] for that type.
///
/// Derived from [`EmittedStructNames`] via `sanitize_field_name`, with a
/// numeric suffix disambiguator applied on sanitized-name collisions. Every
/// struct in `EmittedStructNames` gets a unique pool field name.
pub type PoolFieldNames = BTreeMap<u32, String>;

/// Map from DCB struct index to the name of the bucket sub-struct inside
/// `DataPools` / `RecordIndex` that holds its pool field.
///
/// The bucket key is the same two-letter prefix (with an optional numeric
/// suffix on oversized groups) that the per-type bucket file under
/// `generated/types_*.rs` uses, so a struct's code and its pool slot live
/// in the same bucket name.
///
/// Splitting `DataPools` (and `RecordIndex`) along these buckets is what
/// keeps serde-derive's `Deserialize` tractable at compile time: one
/// bucket sub-struct has ~100-300 fields, one derive of ~100-300
/// match arms, which LLVM can actually finish in bounded time. Without
/// the split, a single derived `visit_map` on the full ~1.9k-field flat
/// struct is big enough that `opt-level = 3` LLVM passes run for over
/// an hour.
pub type BucketAssignments = BTreeMap<u32, String>;

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

/// Header written at the top of every per-bucket `types_<x>.rs` file.
fn types_bucket_header() -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_snake_case)]\n");
    out.push_str("#![allow(non_camel_case_types)]\n");
    out.push_str("#![allow(dead_code)]\n");
    // Many bucket files don't end up referencing every imported type — the
    // imports are emitted uniformly for simplicity, and `unused_imports`
    // would produce 100+ warnings per fresh generation otherwise.
    out.push_str("#![allow(unused_imports)]\n");
    out.push_str("#![allow(clippy::too_many_arguments)]\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");
    out.push_str("use svarog_common::CigGuid;\n");
    out.push_str("use svarog_datacore::{Instance, Value};\n");
    out.push_str("use crate::{Builder, Extract, Handle, Pooled};\n\n");
    // Bring every other generated type back into scope so cross-bucket
    // handle references resolve. rustc resolves these lazily so the implicit
    // cycle between bucket files is fine.
    out.push_str("use super::*;\n\n");
    out
}

/// Maximum number of structs allowed in a single generated bucket file.
/// Any two-letter prefix with more struct definitions than this gets split
/// into numbered sub-buckets. Tuned for rustc parallelism: small enough
/// that no single file dominates check/build time, large enough that we
/// don't explode into thousands of tiny files.
const MAX_STRUCTS_PER_BUCKET: usize = 300;

/// Compute the set of struct indices reachable from top-level records.
///
/// BFS from every struct index referenced by `db.records()` through
/// `Class` / `StrongPointer` / `WeakPointer` property targets (which are
/// the kinds the generator actually materialises). Struct types that
/// nothing reachable from a record references transitively are **pruned**
/// at generation time — they'd produce unused Rust code that still costs
/// full rustc/LLVM processing time.
///
/// `Reference` fields are not followed: those are cross-record GUID
/// pointers whose targets are themselves records, already covered by the
/// seed set. Primitive and array-of-primitive fields don't produce new
/// type references.
///
/// Inheritance is handled implicitly: `collect_full_properties` flattens
/// parent fields into the child's view, so walking a child's properties
/// visits every inherited Class/Pointer target too.
pub fn compute_reachable_struct_indices(db: &DataCoreDatabase) -> HashSet<u32> {
    let mut reachable: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<u32> = VecDeque::new();

    // Seed: every top-level record's struct type.
    for record in db.records() {
        if record.struct_index < 0 {
            continue;
        }
        let idx = record.struct_index as u32;
        if reachable.insert(idx) {
            queue.push_back(idx);
        }
    }

    // BFS through Class / StrongPointer / WeakPointer field targets.
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

    // Self-check: verify the closure is actually closed. Every Class /
    // StrongPointer / WeakPointer target of every reachable type must
    // itself be reachable. If this ever fires, the BFS has a hole and
    // `rust_type_for`'s fallback is silently turning the field into a
    // `CigGuid` stub instead of a real `Handle<T>`.
    //
    // Runs in O(|reachable| × avg props) — a few million operations on
    // a full DCB. Cheap relative to emission cost, and worth it to turn
    // "silent data loss" into "loud panic at codegen time".
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
                         `{field}` → `{to}` (idx {target}), but target was not reached. \
                         This is a bug in `compute_reachable_struct_indices`."
                    );
                }
            }
        }
    }

    reachable
}

/// Walk every DCB struct definition and decide which ones will actually be
/// emitted as Rust types, in a stable order. Dedups by sanitized name —
/// first occurrence of each name wins — and **prunes** struct types that
/// no top-level record transitively references (see
/// [`compute_reachable_struct_indices`]).
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

/// Derive the per-type pool field name for every emitted struct.
///
/// The field name is the snake_case of the type name with a numeric suffix
/// on sanitized-name collisions so the map has a unique key per type.
/// Shared by [`emit_types`] (for `reserve_slot` / `store` bodies),
/// [`emit_data_pools`] (for the struct fields), and [`emit_record_index`].
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

/// Assign every emitted struct to a bucket.
///
/// Buckets match the per-prefix file grouping used by [`emit_types`]:
/// two-letter prefix (`"sc"`, `"ab"`, `"we"`, ...) with a numeric suffix
/// when the group exceeds `MAX_STRUCTS_PER_BUCKET` and has to be split
/// (`"sc1"`, `"sc2"`).
///
/// The returned mapping is consumed by [`emit_types`] (so it doesn't
/// have to recompute), [`emit_data_pools`] / [`emit_record_index`] (so
/// they can group pool fields into bucket sub-structs that keep serde
/// derives small), and [`emit_struct`] (so each type's `Pooled` impl
/// points at the right bucket-qualified field path).
pub fn compute_bucket_assignments(emitted_names: &EmittedStructNames) -> BucketAssignments {
    // Pass 1: group emitted types by two-letter prefix, preserving
    // emission order within each group.
    let mut by_prefix: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for (&struct_idx, name) in emitted_names {
        let prefix = bucket_for(name);
        by_prefix.entry(prefix).or_default().push(struct_idx);
    }

    // Pass 2: assign each struct to either its bare prefix or a numbered
    // sub-bucket, matching the `emit_types` file-bucket splitting rule.
    let mut out: BucketAssignments = BTreeMap::new();
    for (prefix, indices) in by_prefix {
        if indices.len() <= MAX_STRUCTS_PER_BUCKET {
            for idx in indices {
                out.insert(idx, prefix.clone());
            }
        } else {
            for (chunk_idx, chunk) in indices.chunks(MAX_STRUCTS_PER_BUCKET).enumerate() {
                let key = format!("{prefix}{}", chunk_idx + 1);
                for &idx in chunk {
                    out.insert(idx, key.clone());
                }
            }
        }
    }
    out
}

/// Capitalise the first character of a bucket key so it can be used as
/// a Rust type-name suffix: `"ab"` → `"Ab"`, `"sc1"` → `"Sc1"`,
/// `"other"` → `"Other"`.
fn bucket_to_type_suffix(bucket: &str) -> String {
    let mut chars = bucket.chars();
    match chars.next() {
        Some(first) => {
            let first = first.to_ascii_uppercase();
            let rest: String = chars.collect();
            format!("{first}{rest}")
        }
        None => String::new(),
    }
}

/// Derive the Rust field name for a bucket inside `DataPools` /
/// `RecordIndex`. Prefixed with `b_` uniformly so that bucket names that
/// happen to be Rust keywords (`as`, `do`, `fn`, `if`, `in`, `let`,
/// `loop`, `match`, `mod`, `pub`, `ref`, `use`, `where`, ...) don't
/// collide with the language.
///
/// Example: `"ab"` → `"b_ab"`, `"as"` → `"b_as"`, `"sc1"` → `"b_sc1"`.
fn bucket_to_field_name(bucket: &str) -> String {
    format!("b_{bucket}")
}

/// Emit types bucketed into small per-prefix files so rustc can parallelize
/// compilation across modules. Returns a map of bucket name → file source.
///
/// Bucketing strategy:
///
/// 1. Group structs by first-two-letter prefix (e.g. `"SCItem*"` → `"sc"`).
/// 2. If a prefix has more than `MAX_STRUCTS_PER_BUCKET` structs, split it
///    into numbered sub-buckets (`sc1`, `sc2`, ...).
///
/// In practice the DCB has ~6500 structs spread across ~170 two-letter
/// prefixes, with a handful of prefixes (like `"bu"` at ~460 structs) that
/// need further splitting.
pub fn emit_types(
    db: &DataCoreDatabase,
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
    bucket_assignments: &BucketAssignments,
) -> TypesBuckets {
    // Reuse `bucket_assignments` so that a struct's generated code and
    // its pool slot land in the exact same bucket name.
    let mut buckets: TypesBuckets = BTreeMap::new();
    for (&struct_idx, _name) in emitted_names {
        let bucket = bucket_assignments
            .get(&struct_idx)
            .expect("every emitted struct has a bucket assignment");
        let src = buckets
            .entry(bucket.clone())
            .or_insert_with(types_bucket_header);
        emit_struct(
            db,
            struct_idx,
            emitted_names,
            pool_fields,
            bucket_assignments,
            src,
        );
    }
    buckets
}

/// Emit `data_pools.rs` — one bucket sub-struct per letter-prefix group
/// plus a top-level [`DataPools`] that composes them.
///
/// Splitting the flat ~1.9k-field struct into ~140 bucket sub-structs of
/// ~100-300 fields each is what keeps serde-derive's `Deserialize` impl
/// tractable at compile time. A single derive on the flat struct
/// produces a `visit_map` with 1,900+ match arms that LLVM inlines into
/// one enormous function at `opt-level ≥ 2`, sending the compile into
/// the hour range. Bucket sub-structs keep each `visit_map` at ~300
/// match arms, well inside LLVM's efficient range, and the buckets are
/// small enough to distribute across codegen units.
///
/// Per-type `impl Pooled for T` blocks are still colocated with each
/// struct in its `types_*.rs` bucket file — see `emit_struct`.
pub fn emit_data_pools(
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
    bucket_assignments: &BucketAssignments,
) -> String {
    // Group pool fields by bucket. BTreeMap keeps iteration deterministic.
    let mut by_bucket: BTreeMap<String, Vec<(u32, String)>> = BTreeMap::new();
    for (&struct_idx, type_name) in emitted_names {
        let bucket = bucket_assignments
            .get(&struct_idx)
            .expect("every emitted struct has a bucket assignment")
            .clone();
        by_bucket
            .entry(bucket)
            .or_default()
            .push((struct_idx, type_name.clone()));
    }

    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Flat-pool storage for every emitted DCB struct type.\n");
    out.push_str("//!\n");
    out.push_str("//! Each field is a `Vec<Option<T>>` — nested struct fields store a\n");
    out.push_str("//! `Handle<T>` (a `u32` slot index tagged with the target type in\n");
    out.push_str("//! `PhantomData`) rather than a `Box<T>`, so deep DCB nesting cannot\n");
    out.push_str("//! overflow the stack during materialisation. See\n");
    out.push_str("//! `crate::builder::Builder` for the drive loop.\n");
    out.push_str("//!\n");
    out.push_str("//! `DataPools` is composed of bucket sub-structs (`DataPoolsAb`,\n");
    out.push_str("//! `DataPoolsSc1`, …) that mirror the letter-prefix groups used by\n");
    out.push_str("//! `generated/types_*.rs`. The split keeps each `serde::Deserialize`\n");
    out.push_str("//! derive small enough that LLVM's inliner doesn't blow up the\n");
    out.push_str("//! compile time on one monster `visit_map` function.\n\n");

    out.push_str("#![allow(non_snake_case)]\n");
    out.push_str("#![allow(non_camel_case_types)]\n");
    out.push_str("#![allow(dead_code)]\n");
    out.push_str("#![allow(unused_imports)]\n\n");

    out.push_str("use serde::{Deserialize, Serialize};\n\n");
    out.push_str("use super::*;\n\n");

    // ── Bucket sub-structs ───────────────────────────────────────────────
    for (bucket, members) in &by_bucket {
        let suffix = bucket_to_type_suffix(bucket);
        let _ = writeln!(
            out,
            "/// Bucket pool sub-struct for types in the `{bucket}` group."
        );
        out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
        let _ = writeln!(out, "pub struct DataPools{suffix} {{");
        for (struct_idx, type_name) in members {
            let field = pool_fields
                .get(struct_idx)
                .expect("every emitted struct has a pool field name");
            let _ = writeln!(out, "    #[serde(default)]");
            let _ = writeln!(out, "    pub {field}: Vec<Option<{type_name}>>,");
        }
        out.push_str("}\n\n");
    }

    // ── Top-level DataPools ──────────────────────────────────────────────
    out.push_str("/// Per-type `Vec<Option<T>>` storage for every generated DCB struct.\n");
    out.push_str("///\n");
    out.push_str("/// A freshly-reserved slot starts as `None` and is filled by the\n");
    out.push_str("/// [`Builder`](crate::Builder) drain loop. After a clean\n");
    out.push_str("/// `Builder::consume_database().finish()` every slot is `Some`.\n");
    out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
    out.push_str("pub struct DataPools {\n");
    for bucket in by_bucket.keys() {
        let suffix = bucket_to_type_suffix(bucket);
        let field = bucket_to_field_name(bucket);
        let _ = writeln!(out, "    #[serde(default)]");
        let _ = writeln!(out, "    pub {field}: DataPools{suffix},");
    }
    out.push_str("}\n");

    out
}

/// Emit `record_index.rs` — per-record-type `HashMap<CigGuid, Handle<T>>`
/// lookups, split into bucket sub-structs.
///
/// Same letter-prefix bucket split as [`emit_data_pools`], for the same
/// reason: a flat ~600-field struct with serde derives is still big
/// enough to hurt LLVM compile time under aggressive inlining.
///
/// Only top-level record types (those referenced from `db.records()`) get
/// an entry here. Non-record types live in `DataPools` only — no GUID
/// access because they don't have one.
pub fn emit_record_index(
    db: &DataCoreDatabase,
    emitted_names: &EmittedStructNames,
    bucket_assignments: &BucketAssignments,
) -> String {
    // Discover top-level record types: the distinct set of struct indices
    // referenced by records, filtered to those with an emitted Rust type.
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

    // Group record types by bucket. Only emit sub-structs for buckets
    // that contain at least one record-typed struct.
    let mut by_bucket: BTreeMap<String, Vec<(u32, String, String)>> = BTreeMap::new();
    let mut used: HashSet<String> = HashSet::new();
    for struct_idx in &record_struct_indices {
        let type_name = emitted_names
            .get(struct_idx)
            .expect("filtered above")
            .clone();
        let base = sanitize_field_name(&type_name);
        let mut field = base.clone();
        let mut suffix = 2u32;
        while !used.insert(field.clone()) {
            field = format!("{base}_{suffix}");
            suffix += 1;
        }
        let bucket = bucket_assignments
            .get(struct_idx)
            .expect("every emitted struct has a bucket assignment")
            .clone();
        by_bucket
            .entry(bucket)
            .or_default()
            .push((*struct_idx, type_name, field));
    }

    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Per-record-type `HashMap<CigGuid, Handle<T>>` lookups.\n");
    out.push_str("//!\n");
    out.push_str("//! Only top-level DCB record types get an entry here. Non-record\n");
    out.push_str("//! pool types live in `DataPools` only — they have no GUID.\n");
    out.push_str("//!\n");
    out.push_str("//! Composed of bucket sub-structs (`RecordIndexAb`, `RecordIndexSc1`,\n");
    out.push_str("//! …) mirroring `DataPools` and the `generated/types_*.rs` bucket\n");
    out.push_str("//! files. Same rationale as `DataPools`: keep each serde-derive\n");
    out.push_str("//! `visit_map` small enough that LLVM doesn't die on inlining.\n\n");

    out.push_str("#![allow(non_snake_case)]\n");
    out.push_str("#![allow(non_camel_case_types)]\n");
    out.push_str("#![allow(dead_code)]\n");
    out.push_str("#![allow(unused_imports)]\n\n");

    out.push_str("use std::collections::HashMap;\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n");
    out.push_str("use svarog_common::CigGuid;\n");
    out.push_str("use crate::Handle;\n\n");
    out.push_str("use super::*;\n\n");

    // ── Bucket sub-structs ───────────────────────────────────────────────
    for (bucket, members) in &by_bucket {
        let suffix = bucket_to_type_suffix(bucket);
        let _ = writeln!(
            out,
            "/// Bucket record-index sub-struct for types in the `{bucket}` group."
        );
        out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
        let _ = writeln!(out, "pub struct RecordIndex{suffix} {{");
        for (_, type_name, field) in members {
            let _ = writeln!(out, "    #[serde(default)]");
            let _ = writeln!(out, "    pub {field}: HashMap<CigGuid, Handle<{type_name}>>,");
        }
        out.push_str("}\n\n");

        // Per-bucket inherent len() / is_empty() so the top-level
        // RecordIndex::len can sum per-bucket lens — shorter generated
        // code, and each body fits in one codegen unit. `is_empty` is
        // emitted alongside `len` to keep clippy's `len_without_is_empty`
        // lint satisfied.
        let _ = writeln!(out, "impl RecordIndex{suffix} {{");
        out.push_str("    /// Total number of records indexed in this bucket.\n");
        out.push_str("    pub fn len(&self) -> usize {\n");
        for (i, (_, _, field)) in members.iter().enumerate() {
            if i == 0 {
                let _ = write!(out, "        self.{field}.len()");
            } else {
                let _ = write!(out, "\n            + self.{field}.len()");
            }
        }
        out.push('\n');
        out.push_str("    }\n\n");
        out.push_str("    /// True if every per-type map in this bucket is empty.\n");
        out.push_str("    pub fn is_empty(&self) -> bool {\n");
        out.push_str("        self.len() == 0\n");
        out.push_str("    }\n");
        out.push_str("}\n\n");
    }

    // ── Top-level RecordIndex ────────────────────────────────────────────
    out.push_str("/// Per-record-type GUID → slot-index lookup.\n");
    out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
    out.push_str("pub struct RecordIndex {\n");
    for bucket in by_bucket.keys() {
        let suffix = bucket_to_type_suffix(bucket);
        let field = bucket_to_field_name(bucket);
        let _ = writeln!(out, "    #[serde(default)]");
        let _ = writeln!(out, "    pub {field}: RecordIndex{suffix},");
    }
    out.push_str("}\n\n");

    out.push_str("impl RecordIndex {\n");
    out.push_str("    /// Total number of records indexed across every type.\n");
    out.push_str("    pub fn len(&self) -> usize {\n");
    if by_bucket.is_empty() {
        out.push_str("        0\n");
    } else {
        for (i, bucket) in by_bucket.keys().enumerate() {
            let field = bucket_to_field_name(bucket);
            if i == 0 {
                let _ = write!(out, "        self.{field}.len()");
            } else {
                let _ = write!(out, "\n            + self.{field}.len()");
            }
        }
        out.push('\n');
    }
    out.push_str("    }\n\n");

    out.push_str("    /// True if every per-type map is empty.\n");
    out.push_str("    pub fn is_empty(&self) -> bool {\n");
    out.push_str("        self.len() == 0\n");
    out.push_str("    }\n");
    out.push_str("}\n");

    out
}

/// Emit `record_store.rs` — the public `RecordStore` envelope + the
/// generated `Builder::seed_database` dispatcher.
///
/// `RecordStore` is a thin container around `DataPools` + `RecordIndex`.
/// `seed_database` is the giant struct_index → alloc_record::<T> match that
/// walks `db.all_records()` and seeds the builder's worklist.
pub fn emit_record_store(
    db: &DataCoreDatabase,
    emitted_names: &EmittedStructNames,
    bucket_assignments: &BucketAssignments,
) -> String {
    // Same record-type discovery as emit_record_index — they need to agree
    // on which types have GUID maps.
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

    // Tuple layout: (struct_idx, type_name, record-index field name,
    // bucket name). Bucket is the same one the type belongs to in
    // `DataPools`, used to qualify the `b.records.{bucket}.{field}`
    // access path in the emitted record extractor functions.
    let mut used: HashSet<String> = HashSet::new();
    let mut entries: Vec<(u32, String, String, String)> =
        Vec::with_capacity(record_struct_indices.len());
    for struct_idx in &record_struct_indices {
        let type_name = emitted_names
            .get(struct_idx)
            .expect("filtered above")
            .clone();
        let base = sanitize_field_name(&type_name);
        let mut field = base.clone();
        let mut suffix = 2u32;
        while !used.insert(field.clone()) {
            field = format!("{base}_{suffix}");
            suffix += 1;
        }
        let bucket = bucket_assignments
            .get(struct_idx)
            .expect("every emitted struct has a bucket assignment")
            .clone();
        entries.push((*struct_idx, type_name, field, bucket));
    }

    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! `RecordStore` — the flat-pool record store.\n");
    out.push_str("//!\n");
    out.push_str("//! Thin wrapper around `DataPools` (per-type `Vec<Option<T>>`) and\n");
    out.push_str("//! `RecordIndex` (per-record-type `HashMap<CigGuid, TId>`). Built via\n");
    out.push_str("//! `Builder::new(db).consume_database().finish()`.\n\n");

    out.push_str("#![allow(non_snake_case)]\n");
    out.push_str("#![allow(non_camel_case_types)]\n");
    out.push_str("#![allow(dead_code)]\n");
    out.push_str("#![allow(unused_imports)]\n\n");

    out.push_str("use std::collections::HashMap;\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");
    out.push_str("use svarog_common::CigGuid;\n");
    out.push_str("use svarog_datacore::Instance;\n");
    out.push_str("use crate::{Builder, Extract, Handle};\n");
    out.push_str("use super::*;\n\n");

    // ── RecordStore struct ───────────────────────────────────────────────
    out.push_str("/// Flat-pool record store produced by [`Builder::finish`](crate::Builder::finish).\n");
    out.push_str("#[derive(Debug, Clone, Default, Serialize, Deserialize)]\n");
    out.push_str("pub struct RecordStore {\n");
    out.push_str("    /// Per-type `Vec<Option<T>>` storage.\n");
    out.push_str("    #[serde(default)]\n");
    out.push_str("    pub pools: DataPools,\n");
    out.push_str("    /// Per-record-type GUID → slot-index lookups.\n");
    out.push_str("    #[serde(default)]\n");
    out.push_str("    pub records: RecordIndex,\n");
    out.push_str("}\n\n");

    out.push_str("impl RecordStore {\n");
    out.push_str("    /// Create an empty record store.\n");
    out.push_str("    pub fn new() -> Self {\n");
    out.push_str("        Self::default()\n");
    out.push_str("    }\n\n");

    out.push_str("    /// Total number of records indexed across every type.\n");
    out.push_str("    pub fn len(&self) -> usize {\n");
    out.push_str("        self.records.len()\n");
    out.push_str("    }\n\n");

    out.push_str("    /// True if no records have been loaded.\n");
    out.push_str("    pub fn is_empty(&self) -> bool {\n");
    out.push_str("        self.records.is_empty()\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // ── Per-record-type extractor free functions ─────────────────────────
    //
    // Each function is a monomorphised adapter that allocates a record of a
    // specific concrete T and installs the resulting id into the right
    // `RecordIndex` field. We emit them as free functions so the dispatch
    // table in `seed_database` can hold them as `fn` pointers — methods on
    // `Builder<'a>` would require the generics encoded into the pointer
    // type in a way that doesn't coerce cleanly.
    out.push_str("// ── Per-type record extractor adapters ─────────────────────────────────\n");
    out.push_str("//\n");
    out.push_str("// One free function per emitted record type. `seed_database` resolves\n");
    out.push_str("// the current DCB's struct_index for each type by name and stores a\n");
    out.push_str("// pointer to the matching function in a dispatch table. This keeps the\n");
    out.push_str("// extractor robust against game patches that reorder struct definitions.\n");
    out.push_str("type RecordExtractor<'a> = fn(&mut Builder<'a>, CigGuid, Instance<'a>);\n\n");
    for (_idx, type_name, field, bucket) in &entries {
        let fn_name = format!("extract_record_{field}");
        let bucket_field = bucket_to_field_name(bucket);
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
            "    b.records.{bucket_field}.{field}.insert(guid, id);"
        );
        let _ = writeln!(out, "}}");
    }
    out.push('\n');

    // ── Builder::seed_database ───────────────────────────────────────────
    out.push_str("impl<'a> Builder<'a> {\n");
    out.push_str("    /// Walk every record in the database and seed the worklist.\n");
    out.push_str("    ///\n");
    out.push_str("    /// Called from [`Builder::consume_database`]. Resolves the runtime\n");
    out.push_str("    /// `struct_index` for every known record type **by name** against\n");
    out.push_str("    /// the live DCB so patched binaries remain compatible — game patches\n");
    out.push_str("    /// that add or reorder struct types no longer silently drop records.\n");
    out.push_str("    ///\n");
    out.push_str("    /// # Debug staleness check\n");
    out.push_str("    ///\n");
    out.push_str("    /// In debug builds (`cfg(debug_assertions)`), this method panics\n");
    out.push_str("    /// at the end of the seeding loop if the runtime DCB contains any\n");
    out.push_str("    /// record type the generated dispatch table doesn't know about.\n");
    out.push_str("    /// That's a direct signal the generator is out of date relative to\n");
    out.push_str("    /// the game patch whose DCB is being parsed, and you should\n");
    out.push_str("    /// regenerate with `sc-generator`. In release builds the check is\n");
    out.push_str("    /// dead-code-eliminated — unknown records are silently skipped and\n");
    out.push_str("    /// the app keeps working with whatever subset it does understand.\n");
    out.push_str("    pub fn seed_database(&mut self) {\n");
    if entries.is_empty() {
        out.push_str("        // No record types were emitted.\n");
    } else {
        // Build a name → struct_index HashMap once so each known type costs
        // O(1) to resolve. Avoids an O(n × m) scan over struct_definitions.
        out.push_str("        let n_structs = self.db.struct_definitions().len();\n");
        out.push_str("        let name_to_idx: HashMap<&str, usize> = (0..n_structs)\n");
        out.push_str("            .filter_map(|i| self.db.struct_name(i).map(|n| (n, i)))\n");
        out.push_str("            .collect();\n\n");

        // Per-index dispatch table. A Vec rather than HashMap so the
        // per-record lookup in the hot loop is just an array index.
        out.push_str("        let mut dispatch: Vec<Option<RecordExtractor<'a>>> = vec![None; n_structs];\n");
        for (_idx, type_name, field, _bucket) in &entries {
            let fn_name = format!("extract_record_{field}");
            let _ = writeln!(
                out,
                "        if let Some(&i) = name_to_idx.get({type_name}::TYPE_NAME) {{ dispatch[i] = Some({fn_name}); }}"
            );
        }
        out.push('\n');

        // Debug-only staleness guard: track every record struct_idx the
        // dispatch table doesn't know. After the loop, if any were seen,
        // `panic!` with the full list of vanished/added type names so a
        // debug build fails loudly the moment the generator falls behind
        // the runtime DCB. In release the whole collection + check is
        // dead-code-eliminated — records of unknown types are silently
        // skipped and the app keeps working.
        out.push_str("        #[cfg(debug_assertions)]\n");
        out.push_str("        let mut unknown_record_types: std::collections::HashSet<u32> = std::collections::HashSet::new();\n\n");

        // Collect records first to release the &db borrow before &mut self
        // reentry.
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

        // Fire the staleness panic once, with every unknown name listed.
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

/// Compute the bucket suffix for a Rust type name.
///
/// - `"SCItemWeaponParams"` → `"sc"`
/// - `"AbstractThing"` → `"ab"`
/// - `"X"` → `"x"` (single-letter)
/// - `"_SomethingWeird"` → `"so"` (first two *alphabetic* chars)
/// - `"123"` → `"other"`
fn bucket_for(rust_name: &str) -> String {
    let mut alphas = rust_name
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase());
    match (alphas.next(), alphas.next()) {
        (Some(a), Some(b)) => format!("{a}{b}"),
        (Some(a), None) => a.to_string(),
        _ => "other".to_string(),
    }
}

/// Emit `enums.rs` — one Rust enum per DCB enum (currently informational;
/// generated struct fields still store enum values as `String`).
pub fn emit_enums(db: &DataCoreDatabase) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("#![allow(non_camel_case_types)]\n");
    out.push_str("#![allow(dead_code)]\n\n");
    out.push_str("use serde::{Deserialize, Serialize};\n\n");

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

        // Deliberately chosen NOT to collide with any real DCB enum value.
        // `Unknown` is too common — several real DCB enums have a literal
        // `Unknown` value, and colliding with it produces a cascade of
        // E0428 / E0532 / E0277 errors downstream.
        const FALLBACK_VARIANT: &str = "Unrecognized";

        let _ = writeln!(out, "/// DCB enum: `{name}`");
        let _ = writeln!(
            out,
            "#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]"
        );
        let _ = writeln!(out, "pub enum {sanitized} {{");
        let mut variant_names: HashSet<String> = HashSet::new();
        // Reserve the fallback name so a real enum value called
        // "Unrecognized" (unlikely but not impossible) gets renamed.
        variant_names.insert(FALLBACK_VARIANT.to_string());
        if values.is_empty() {
            let _ = writeln!(out, "    Empty,");
        } else {
            for value in &values {
                let variant = sanitize_variant_name(value);
                if variant.is_empty() {
                    continue;
                }
                if !variant_names.insert(variant.clone()) {
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

/// Emit `metadata.rs` — version constants.
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

/// Emit `mod.rs` — re-exports the sub-modules so sc-extract can include
/// the whole `generated/` directory as a single module.
pub fn emit_mod_file(bucket_names: &[String]) -> String {
    let mut out = String::new();
    out.push_str(FILE_HEADER);
    out.push_str("//! Machine-generated DataCore schema bindings.\n");
    out.push_str("//!\n");
    out.push_str("//! Generated types are split into per-letter bucket files\n");
    out.push_str("//! (`types_a.rs` through `types_z.rs` plus `types_other.rs`)\n");
    out.push_str("//! so rustc can parallelize compilation across modules.\n");
    out.push_str("//! All types are re-exported flat at this module level.\n\n");

    out.push_str("pub mod enums;\n");
    out.push_str("pub mod metadata;\n");
    // data_pools references the type buckets via `use super::*` and thus
    // must come after them — but that's only a doc-order concern; mod file
    // order doesn't drive rustc's name resolution. Bucket files still come
    // first for readability.
    for bucket in bucket_names {
        let _ = writeln!(out, "pub mod types_{bucket};");
    }
    out.push_str("pub mod data_pools;\n");
    out.push_str("pub mod record_index;\n");
    out.push_str("pub mod record_store;\n");
    out.push('\n');
    for bucket in bucket_names {
        let _ = writeln!(out, "pub use types_{bucket}::*;");
    }
    out.push_str("pub use data_pools::DataPools;\n");
    out.push_str("pub use record_index::RecordIndex;\n");
    out.push_str("pub use record_store::RecordStore;\n");
    out
}

// ── Struct emission ────────────────────────────────────────────────────────

fn emit_struct(
    db: &DataCoreDatabase,
    struct_idx: u32,
    emitted_names: &EmittedStructNames,
    pool_fields: &PoolFieldNames,
    bucket_assignments: &BucketAssignments,
    out: &mut String,
) {
    // Callers in `emit_types` have already resolved the sanitized name and
    // deduplicated by that name — this function trusts the index it's given.
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
    let bucket = bucket_assignments
        .get(&struct_idx)
        .expect("every emitted struct has a bucket assignment");

    let def = &db.struct_definitions()[struct_idx as usize];
    let props = collect_full_properties(db, struct_idx);

    // Build field records, resolving rust type + extract expression.
    // We collect into Vec so we can deduplicate field names within a single
    // struct (the DCB has cases where an inherited field is shadowed by an
    // identically-named own property).
    let mut fields: Vec<FieldInfo> = Vec::new();
    let mut seen_field_names: HashSet<String> = HashSet::new();
    for prop in &props {
        let Some(orig_name) = db.property_name(prop) else {
            continue;
        };
        if orig_name.is_empty() {
            continue;
        }
        let field_name = sanitize_field_name(orig_name);
        if !seen_field_names.insert(field_name.clone()) {
            continue;
        }
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

    // ── Struct definition ────────────────────────────────────────────────
    let _ = writeln!(out, "/// DCB type: `{raw_name}`");
    if def.parent_type_index >= 0 {
        if let Some(parent_name) = db.struct_name(def.parent_type_index as usize) {
            let _ = writeln!(out, "///");
            let _ = writeln!(out, "/// Inherits from: `{parent_name}` (fields flattened)");
        }
    }
    // NOTE: `Default` is deliberately *not* derived on generated structs.
    // It would add ~6.5k impls (one per DCB struct) for no consumer benefit
    // — `Extract::extract` uses explicit field initializers.
    let _ = writeln!(out, "#[derive(Debug, Clone, Serialize, Deserialize)]");
    let _ = writeln!(out, "pub struct {name} {{");
    for f in &fields {
        let dt = f
            .data_type
            .map(|d| d.as_str())
            .unwrap_or("unknown");
        let array_tag = if f.is_array { " (array)" } else { "" };
        let _ = writeln!(out, "    /// DCB field: `{}` ({dt}{array_tag})", f.orig_name);
        let _ = writeln!(out, "    #[serde(default)]");
        let _ = writeln!(out, "    pub {}: {},", f.field_name, f.rust_type);
    }
    let _ = writeln!(out, "}}\n");

    // ── Pooled impl ──────────────────────────────────────────────────────
    // Two one-liners. Replaces the old per-type trio of `TId` newtype +
    // `impl Index<TId>` + inherent `get` — the generic `Handle<T>`
    // machinery in the hand-written `handle.rs` takes care of the rest.
    //
    // The field path goes through `pools.{bucket_field}.{pool_field}` —
    // `DataPools` is composed of bucket sub-structs (`DataPoolsAb`,
    // `DataPoolsSc1`, ...) so that serde-derive's `visit_map` stays
    // small per bucket. Bucket field names are prefixed with `b_` so
    // bucket prefixes that happen to be Rust keywords (`as`, `do`, ...)
    // don't collide.
    let bucket_field = bucket_to_field_name(bucket);
    let _ = writeln!(out, "impl Pooled for {name} {{");
    let _ = writeln!(
        out,
        "    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {{ &pools.{bucket_field}.{pool_field} }}"
    );
    let _ = writeln!(
        out,
        "    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {{ &mut pools.{bucket_field}.{pool_field} }}"
    );
    let _ = writeln!(out, "}}\n");

    // ── Extract impl ─────────────────────────────────────────────────────
    // `inst` is used whenever there's at least one field; `b` only when at
    // least one field is a Class / StrongPointer / WeakPointer (primitive
    // and Reference fields don't touch the builder).
    let uses_builder = fields.iter().any(|f| {
        matches!(
            f.data_type,
            Some(DataType::Class)
                | Some(DataType::StrongPointer)
                | Some(DataType::WeakPointer)
        )
    });
    let inst_name = if fields.is_empty() { "_inst" } else { "inst" };
    let b_name = if uses_builder { "b" } else { "_b" };
    let _ = writeln!(out, "impl<'a> Extract<'a> for {name} {{");
    let quoted_raw = quote_string(raw_name);
    let _ = writeln!(
        out,
        "    const TYPE_NAME: &'static str = {quoted_raw};"
    );
    let _ = writeln!(
        out,
        "    fn extract({inst_name}: &Instance<'a>, {b_name}: &mut Builder<'a>) -> Self {{"
    );
    let _ = writeln!(out, "        Self {{");
    for f in &fields {
        let _ = writeln!(out, "            {}: {},", f.field_name, f.expr);
    }
    let _ = writeln!(out, "        }}");
    let _ = writeln!(out, "    }}");
    let _ = writeln!(out, "}}\n");
}

struct FieldInfo {
    orig_name: String,
    field_name: String,
    rust_type: String,
    expr: String,
    data_type: Option<DataType>,
    is_array: bool,
}

/// Walk a struct's inheritance chain and return all properties (parent
/// fields first, then own), flattened.
fn collect_full_properties(
    db: &DataCoreDatabase,
    struct_idx: u32,
) -> Vec<&DataCorePropertyDefinition> {
    let mut out = Vec::new();
    collect_recursive(db, struct_idx, &mut out);
    out
}

fn collect_recursive<'a>(
    db: &'a DataCoreDatabase,
    struct_idx: u32,
    out: &mut Vec<&'a DataCorePropertyDefinition>,
) {
    let Some(def) = db.struct_definitions().get(struct_idx as usize) else {
        return;
    };
    let parent_idx = def.parent_type_index;
    if parent_idx >= 0 {
        collect_recursive(db, parent_idx as u32, out);
    }
    for prop in db.get_struct_properties(struct_idx as usize) {
        out.push(prop);
    }
}

// ── Type mapping ───────────────────────────────────────────────────────────

/// Compute the Rust type for a DCB property.
fn rust_type_for(
    _db: &DataCoreDatabase,
    prop: &DataCorePropertyDefinition,
    emitted_names: &EmittedStructNames,
) -> String {
    let data_type = prop.get_data_type();
    let is_array = prop.is_array();

    let elem_type: String = match data_type {
        Some(DataType::Boolean) => "bool".into(),
        Some(DataType::SByte)
        | Some(DataType::Int16)
        | Some(DataType::Int32) => "i32".into(),
        Some(DataType::Int64) => "i64".into(),
        Some(DataType::Byte)
        | Some(DataType::UInt16)
        | Some(DataType::UInt32) => "u32".into(),
        Some(DataType::UInt64) => "u64".into(),
        Some(DataType::Single) => "f32".into(),
        Some(DataType::Double) => "f64".into(),
        Some(DataType::String)
        | Some(DataType::Locale)
        | Some(DataType::EnumChoice) => "String".into(),
        Some(DataType::Guid) => "CigGuid".into(),
        Some(DataType::Class)
        | Some(DataType::StrongPointer)
        | Some(DataType::WeakPointer) => {
            // Resolve the target struct's *emitted* name. If the target
            // was pruned by reachability (no record transitively touches
            // it) we fall back to `CigGuid` in the surrounding wrapper —
            // see the handling in the wrapper section below.
            let target_idx = prop.struct_index as u32;
            match emitted_names.get(&target_idx) {
                Some(name) => format!("Handle<{name}>"),
                None => "CigGuid".into(),
            }
        }
        // Reference targets stay as bare GUIDs — they cross record
        // boundaries and are looked up via `RecordIndex`, not the pool.
        Some(DataType::Reference) => "CigGuid".into(),
        None => "String".into(),
    };

    if is_array {
        format!("Vec<{elem_type}>")
    } else {
        match data_type {
            Some(DataType::Class)
            | Some(DataType::StrongPointer)
            | Some(DataType::WeakPointer) => format!("Option<{elem_type}>"),
            Some(DataType::Reference) => format!("Option<{elem_type}>"),
            _ => elem_type,
        }
    }
}

/// Compute the expression to use inside `Extract::extract` for a given
/// property. Returns an expression that produces a value of the matching
/// Rust type computed by `rust_type_for`.
fn extract_expr(
    db: &DataCoreDatabase,
    prop: &DataCorePropertyDefinition,
    orig_name: &str,
    emitted_names: &EmittedStructNames,
) -> String {
    let data_type = prop.get_data_type();
    let is_array = prop.is_array();
    // Double-escape quotes in the name just in case a DCB property name
    // contains a double quote.
    let escaped = orig_name.replace('\\', "\\\\").replace('"', "\\\"");

    if is_array {
        return array_expr(db, prop, &escaped, emitted_names);
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
        Some(DataType::String) | Some(DataType::Locale) | Some(DataType::EnumChoice) => {
            format!(
                "inst.get_str(\"{escaped}\").map(String::from).unwrap_or_default()"
            )
        }
        Some(DataType::Guid) => format!("inst.get_guid(\"{escaped}\").unwrap_or_default()"),
        Some(DataType::Class)
        | Some(DataType::StrongPointer)
        | Some(DataType::WeakPointer) => {
            let target_idx = prop.struct_index as u32;
            let Some(target_name) = emitted_names.get(&target_idx) else {
                // Target was pruned (unreachable from any record) or the
                // sanitized name collided — field falls back to a GUID
                // stub that always resolves to None. See
                // `rust_type_for`'s matching fallback.
                return "None".into();
            };
            // Match on the raw Value so we can decide inline-vs-pooled and
            // pick the right `from_pool` flag for alloc_nested. The
            // builder returns `Handle<{target_name}>` directly — no
            // per-type newtype wrapping needed.
            format!(
                "match inst.get(\"{escaped}\") {{\n                Some(Value::Class {{ struct_index, data }}) => Some(b.alloc_nested::<{target_name}>(Instance::from_inline_data(b.db, struct_index, data), false)),\n                Some(Value::ClassRef(r))\n                | Some(Value::StrongPointer(Some(r)))\n                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<{target_name}>(b.db.instance(r.struct_index, r.instance_index), true)),\n                _ => None,\n            }}"
            )
        }
        Some(DataType::Reference) => {
            format!(
                "inst.get(\"{escaped}\").and_then(|v| v.as_record_ref()).map(|r| r.guid)"
            )
        }
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
        Some(DataType::String) | Some(DataType::Locale) | Some(DataType::EnumChoice) => {
            "v.as_str().map(String::from)".to_string()
        }
        Some(DataType::Guid) => "v.as_guid()".to_string(),
        Some(DataType::Reference) => {
            // Vec<CigGuid> — filter out Null references and unwrap the guid.
            "if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }"
                .to_string()
        }
        Some(DataType::Class)
        | Some(DataType::StrongPointer)
        | Some(DataType::WeakPointer) => {
            let target_idx = prop.struct_index as u32;
            let Some(target_name) = emitted_names.get(&target_idx) else {
                // Target was pruned or name-collided — leave the whole
                // array empty. We still consume `inst.get_array` to stay
                // structurally uniform with the happy path, but never
                // push any elements.
                return format!(
                    "{{ let _ = inst.get_array(\"{escaped}\"); Vec::<CigGuid>::new() }}"
                );
            };
            // Build-time-ish: the closure runs per element and decides
            // inline-vs-pool per occurrence; dedup (from_pool=true) is only
            // applied for ClassRef / StrongPointer / WeakPointer variants.
            // `alloc_nested` returns `Handle<{target_name}>` directly.
            format!(
                "match v {{\n                        Value::Class {{ struct_index, data }} => Some(b.alloc_nested::<{target_name}>(Instance::from_inline_data(b.db, struct_index, data), false)),\n                        Value::ClassRef(r)\n                        | Value::StrongPointer(Some(r))\n                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<{target_name}>(b.db.instance(r.struct_index, r.instance_index), true)),\n                        _ => None,\n                    }}"
            )
        }
        None => "None".to_string(),
    };

    format!(
        "inst.get_array(\"{escaped}\")\n                .map(|arr| arr.filter_map(|v| {inner}).collect())\n                .unwrap_or_default()"
    )
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// Produce a Rust string literal expression for a given string value.
/// Escapes quotes, backslashes, and other special characters.
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
