//! Shared data-driven closure walker.
//!
//! Walks the DCB record instance graph from a set of seed records (identified
//! by `file_name` path prefix) following Class / StrongPointer / WeakPointer /
//! Reference edges, and returns the set of struct indices actually observed.
//!
//! In addition to the runtime walk, the result is **closed under declared
//! field targets**: if type X is observed and X has a field with declared
//! target Y, then Y is also in the closure — even if that field is never
//! populated at runtime. This is required for correct compilation: the
//! struct definition of X names Y in a field type, so Y must be in scope
//! when X compiles.
//!
//! Used by:
//! - `features::classify_features` — per-feature closure for cfg assignment.
//!
//! The walker is intentionally simple — no caching, no parallelism. One
//! per-feature walk costs tens of milliseconds on the current DCB; running
//! it for all ~800 leaf features adds a few seconds to generator startup.

use std::collections::{HashMap, HashSet, VecDeque};

use svarog_common::CigGuid;
use svarog_datacore::{DataCoreDatabase, DataType, Instance, Value};

use crate::emit::PropertyCache;

/// Map of record GUIDs → (struct_index, instance_index) for `Reference`
/// resolution during a walk.
pub type GuidLookup = HashMap<CigGuid, (u32, u32)>;

/// Build a GUID → (struct_index, instance_index) lookup for every
/// record in the database.
pub fn build_guid_lookup(db: &DataCoreDatabase) -> GuidLookup {
    let mut out: GuidLookup = HashMap::with_capacity(db.all_records().count());
    for record in db.all_records() {
        out.insert(
            record.id(),
            (record.struct_index(), record.instance_index()),
        );
    }
    out
}

/// Walk the instance graph from every record whose `file_name` starts
/// with any of the given prefixes, following Class / Pointer / Reference
/// edges. Returns the set of observed runtime struct indices.
///
/// `cache` is the shared inherited-property cache; only the schema-closure
/// pass at the end uses it — the instance walker still reads own-properties
/// directly from `db` because it needs the field ordering tied to each
/// runtime struct's own layout.
///
/// `dangling` accumulates the set of Reference GUIDs that do not resolve
/// to any top-level record. Dangling refs used to panic the generator,
/// but real game data contains them (confirmed against svarog's own
/// export walker, which also silently skips unknown record GUIDs). The
/// walker logs each unique dangling GUID once per run and the pipeline
/// reports the total at the end.
pub fn walk_closure(
    db: &DataCoreDatabase,
    prefixes: &[String],
    guid_to_instance: &GuidLookup,
    cache: &PropertyCache<'_>,
    dangling: &mut HashSet<CigGuid>,
) -> HashSet<u32> {
    let mut observed: HashSet<u32> = HashSet::new();
    let mut worklist: Vec<Instance<'_>> = Vec::new();
    let mut visited_pool_targets: HashSet<(u32, u32)> = HashSet::new();
    let mut visited_records: HashSet<CigGuid> = HashSet::new();

    for record in db.all_records() {
        let Some(file_name) = record.file_name() else { continue };
        if !prefixes.iter().any(|p| file_name.starts_with(p.as_str())) {
            continue;
        }
        let inst = record.as_instance();
        observed.insert(inst.struct_index());
        visited_records.insert(record.id());
        worklist.push(inst);
    }

    while let Some(inst) = worklist.pop() {
        let parent_struct = inst.struct_index();
        let props = db.get_struct_properties(parent_struct as usize);
        for prop in props {
            let Some(name) = db.property_name(prop) else { continue };
            let data_type = prop.get_data_type();
            match data_type {
                Some(DataType::Class) => {
                    if prop.conversion_type == 0 {
                        if let Some(Value::Class { struct_index, data }) = inst.get(name) {
                            observed.insert(struct_index);
                            let nested = Instance::from_inline_data(db, struct_index, data);
                            worklist.push(nested);
                        }
                    } else if let Some(arr) = inst.get_array(name) {
                        for elem in arr {
                            if let Value::ClassRef(r) = elem {
                                observed.insert(r.struct_index);
                                if visited_pool_targets.insert((r.struct_index, r.instance_index)) {
                                    worklist.push(db.instance(r.struct_index, r.instance_index));
                                }
                            }
                        }
                    }
                }
                Some(DataType::StrongPointer) | Some(DataType::WeakPointer) => {
                    if prop.conversion_type == 0 {
                        match inst.get(name) {
                            Some(Value::StrongPointer(Some(r)))
                            | Some(Value::WeakPointer(Some(r))) => {
                                observed.insert(r.struct_index);
                                if visited_pool_targets.insert((r.struct_index, r.instance_index)) {
                                    worklist.push(db.instance(r.struct_index, r.instance_index));
                                }
                            }
                            _ => {}
                        }
                    } else if let Some(arr) = inst.get_array(name) {
                        for elem in arr {
                            match elem {
                                Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                                    observed.insert(r.struct_index);
                                    if visited_pool_targets.insert((r.struct_index, r.instance_index)) {
                                        worklist.push(db.instance(r.struct_index, r.instance_index));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Some(DataType::Reference) => {
                    if prop.conversion_type == 0 {
                        if let Some(Value::Reference(Some(r))) = inst.get(name)
                            && visited_records.insert(r.guid)
                        {
                            let Some(&(si, ii)) = guid_to_instance.get(&r.guid) else {
                                if dangling.insert(r.guid) {
                                    let parent_name = db.struct_name(parent_struct as usize).unwrap_or("<?>");
                                    tracing::warn!(
                                        parent = parent_name,
                                        parent_idx = parent_struct,
                                        field = name,
                                        guid = ?r.guid,
                                        "dangling Reference: target GUID not present in record map"
                                    );
                                }
                                continue;
                            };
                            observed.insert(si);
                            worklist.push(db.instance(si, ii));
                        }
                    } else if let Some(arr) = inst.get_array(name) {
                        for elem in arr {
                            if let Value::Reference(Some(r)) = elem
                                && visited_records.insert(r.guid)
                            {
                                let Some(&(si, ii)) = guid_to_instance.get(&r.guid) else {
                                    if dangling.insert(r.guid) {
                                        let parent_name = db.struct_name(parent_struct as usize).unwrap_or("<?>");
                                        tracing::warn!(
                                            parent = parent_name,
                                            parent_idx = parent_struct,
                                            field = name,
                                            guid = ?r.guid,
                                            "dangling Reference (array elem): target GUID not present in record map"
                                        );
                                    }
                                    continue;
                                };
                                observed.insert(si);
                                worklist.push(db.instance(si, ii));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Close the observed set under **declared** Class / Pointer field
    // targets. Polymorphic pointer descendants stay data-driven (a
    // subclass only enters the closure if it was actually observed),
    // but the declared target itself must be in scope because the
    // emitted struct literally names it in a field type.
    //
    // Without this, a Multi-assigned type X with a field `Handle<Y>`
    // would compile-fail in feature F if F's runtime walk reached X
    // (making F a member of X's Multi cfg list) but never populated
    // the field, so Y's cfg list didn't include F.
    close_under_declared_targets(db, &mut observed, cache);

    observed
}

/// BFS over already-observed types, following declared Class / Ptr
/// targets only. Pointer-descendant expansion is intentionally omitted
/// — subclass membership stays data-driven.
fn close_under_declared_targets(
    db: &DataCoreDatabase,
    observed: &mut HashSet<u32>,
    cache: &PropertyCache<'_>,
) {
    let n_structs = db.struct_definitions().len();
    let mut queue: VecDeque<u32> = observed.iter().copied().collect();
    while let Some(idx) = queue.pop_front() {
        for prop in &cache[idx as usize] {
            let data_type = prop.get_data_type();
            let is_ptr_or_class = matches!(
                data_type,
                Some(DataType::Class) | Some(DataType::StrongPointer) | Some(DataType::WeakPointer)
            );
            if !is_ptr_or_class {
                continue;
            }
            let target = prop.struct_index as u32;
            if (target as usize) >= n_structs {
                continue;
            }
            if observed.insert(target) {
                queue.push_back(target);
            }
        }
    }
}
