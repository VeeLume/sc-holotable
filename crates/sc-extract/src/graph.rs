//! Reference graph — cross-record references discovered during parsing.
//!
//! The [`ReferenceGraph`] stores every `Value::Reference` edge found while
//! walking the DCB. It provides both forward lookup (what does record X
//! reference?) and reverse lookup (who references record Y?) in O(1) per
//! query.
//!
//! # What's tracked and what isn't
//!
//! Only `Value::Reference` edges (GUID-based cross-record references) are
//! tracked. `Value::StrongPointer` and `Value::WeakPointer` are internal
//! structural pointers that target anonymous instances in a struct pool —
//! they don't have meaningful reverse-lookup semantics for consumers, and
//! treating them as edges would pollute the graph with structural noise.
//!
//! The walker still *descends* through strong pointers, weak pointers,
//! inline classes, class refs, and arrays to find any nested `Reference`
//! values — for example, a weapon's `Components` array contains a
//! `SCItemWeaponComponentParams` class whose `ammoContainer.ammoParamsRecord`
//! is a GUID reference to an ammo record. That nested reference gets
//! recorded as an edge from the weapon to the ammo.
//!
//! # Memory efficiency
//!
//! The graph stores only `Guid` pairs (no field-path strings). Per-record
//! traversal deduplicates visited pool instances via `(struct_index,
//! instance_index)` so shared subtrees are walked once per source record,
//! not once per incoming pointer.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::svarog_datacore::{DataCoreDatabase, Instance, Value};
use crate::Guid;

/// Directed graph of cross-record references.
///
/// Built once during `parse_from_p4k` by walking every record and
/// collecting every `Value::Reference` found — whether at the top level
/// or nested inside inline classes, class refs, strong/weak pointers, or
/// arrays.
///
/// Both forward and inverse adjacency lists are stored, so forward lookups
/// (`outgoing`) and reverse lookups (`incoming`) are O(1) in the adjacency
/// size.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReferenceGraph {
    /// Forward edges: source → targets.
    out_edges: HashMap<Guid, Vec<Guid>>,
    /// Reverse edges: target → sources.
    in_edges: HashMap<Guid, Vec<Guid>>,
}

impl ReferenceGraph {
    /// Construct an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a single (from, to) edge. Updates both forward and reverse
    /// indices. No deduplication — callers walk each source record once.
    pub fn insert(&mut self, from: Guid, to: Guid) {
        self.in_edges.entry(to).or_default().push(from);
        self.out_edges.entry(from).or_default().push(to);
    }

    /// Forward lookup: target GUIDs referenced by `from`.
    /// Returns an empty slice if the record has no outgoing references.
    pub fn outgoing(&self, from: &Guid) -> &[Guid] {
        self.out_edges.get(from).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Reverse lookup: source GUIDs that reference `target`.
    /// Returns an empty slice if no record references it.
    pub fn incoming(&self, target: &Guid) -> &[Guid] {
        self.in_edges.get(target).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Total number of edges across all sources.
    pub fn edge_count(&self) -> usize {
        self.out_edges.values().map(Vec::len).sum()
    }

    /// True if the graph contains no edges.
    pub fn is_empty(&self) -> bool {
        self.out_edges.is_empty()
    }

    /// Number of distinct records with outgoing edges.
    pub fn source_count(&self) -> usize {
        self.out_edges.len()
    }

    /// Number of distinct records with incoming edges.
    pub fn target_count(&self) -> usize {
        self.in_edges.len()
    }

    // ── Construction ────────────────────────────────────────────────────

    /// Walk the DCB and build a complete reference graph.
    ///
    /// Visits every record, walks its instance iteratively (descending
    /// through inline classes, class refs, strong/weak pointers, and
    /// arrays), and records every `Value::Reference` found as an edge
    /// from the source record's GUID to the target record's GUID.
    ///
    /// Uses an iterative worklist (not recursion) to handle DCB nesting
    /// depths of 50+. Pool instances (`ClassRef`, `StrongPointer`,
    /// `WeakPointer`) are deduplicated per source record so shared
    /// subtrees are walked at most once.
    pub fn from_database(db: &DataCoreDatabase) -> Self {
        let mut graph = Self::new();
        // Worklist: (instance to visit, source record GUID).
        // Instance<'_> is Copy (two u32s + two refs into the db).
        let mut worklist: Vec<(Instance<'_>, Guid)> = Vec::new();
        // Per-record dedup of visited pool instances to avoid re-walking
        // shared subtrees. Keyed by (struct_index, instance_index).
        let mut visited: HashSet<(u32, u32)> = HashSet::new();

        for record in db.all_records() {
            let source = record.id();
            worklist.push((record.as_instance(), source));
            visited.clear();

            while let Some((inst, source)) = worklist.pop() {
                for prop in inst.properties() {
                    match prop.value {
                        Value::Reference(Some(r)) => {
                            graph.insert(source, r.guid);
                        }
                        Value::Class { struct_index, data } => {
                            let nested = Instance::from_inline_data(db, struct_index, data);
                            worklist.push((nested, source));
                        }
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => {
                            if visited.insert((r.struct_index, r.instance_index)) {
                                let nested = db.instance(r.struct_index, r.instance_index);
                                worklist.push((nested, source));
                            }
                        }
                        Value::Array(_) => {
                            if let Some(arr) = inst.get_array(prop.name) {
                                for elem in arr {
                                    match elem {
                                        Value::Reference(Some(r)) => {
                                            graph.insert(source, r.guid);
                                        }
                                        Value::Class { struct_index, data } => {
                                            let nested = Instance::from_inline_data(
                                                db,
                                                struct_index,
                                                data,
                                            );
                                            worklist.push((nested, source));
                                        }
                                        Value::ClassRef(r)
                                        | Value::StrongPointer(Some(r))
                                        | Value::WeakPointer(Some(r)) => {
                                            if visited
                                                .insert((r.struct_index, r.instance_index))
                                            {
                                                let nested = db.instance(
                                                    r.struct_index,
                                                    r.instance_index,
                                                );
                                                worklist.push((nested, source));
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svarog_common::CigGuid;

    /// Construct a distinct guid for tests by filling all 16 bytes with `byte`.
    fn g(byte: u8) -> Guid {
        CigGuid::from_bytes([byte; 16])
    }

    #[test]
    fn new_graph_is_empty() {
        let graph = ReferenceGraph::new();
        assert!(graph.is_empty());
        assert_eq!(graph.edge_count(), 0);
        assert_eq!(graph.source_count(), 0);
        assert_eq!(graph.target_count(), 0);
    }

    #[test]
    fn insert_single_edge() {
        let mut graph = ReferenceGraph::new();
        graph.insert(g(1), g(2));

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.source_count(), 1);
        assert_eq!(graph.target_count(), 1);
        assert!(!graph.is_empty());
    }

    #[test]
    fn outgoing_returns_target() {
        let mut graph = ReferenceGraph::new();
        graph.insert(g(1), g(2));

        let outgoing = graph.outgoing(&g(1));
        assert_eq!(outgoing.len(), 1);
        assert_eq!(outgoing[0], g(2));
    }

    #[test]
    fn incoming_returns_source() {
        let mut graph = ReferenceGraph::new();
        graph.insert(g(1), g(2));

        let incoming = graph.incoming(&g(2));
        assert_eq!(incoming, &[g(1)]);
    }

    #[test]
    fn multiple_sources_referencing_same_target() {
        let mut graph = ReferenceGraph::new();
        graph.insert(g(1), g(99));
        graph.insert(g(2), g(99));
        graph.insert(g(3), g(99));

        let incoming = graph.incoming(&g(99));
        assert_eq!(incoming.len(), 3);
        assert!(incoming.contains(&g(1)));
        assert!(incoming.contains(&g(2)));
        assert!(incoming.contains(&g(3)));
        assert_eq!(graph.edge_count(), 3);
        assert_eq!(graph.source_count(), 3);
        assert_eq!(graph.target_count(), 1);
    }

    #[test]
    fn single_source_referencing_multiple_targets() {
        let mut graph = ReferenceGraph::new();
        graph.insert(g(1), g(10));
        graph.insert(g(1), g(20));
        graph.insert(g(1), g(30));

        let outgoing = graph.outgoing(&g(1));
        assert_eq!(outgoing.len(), 3);
        assert_eq!(graph.source_count(), 1);
        assert_eq!(graph.target_count(), 3);
    }

    #[test]
    fn lookup_missing_guids_returns_empty() {
        let graph = ReferenceGraph::new();
        assert!(graph.outgoing(&g(42)).is_empty());
        assert!(graph.incoming(&g(42)).is_empty());
    }

    #[test]
    fn serde_round_trip() {
        let mut graph = ReferenceGraph::new();
        graph.insert(g(1), g(2));
        graph.insert(g(3), g(4));

        let json = serde_json::to_string(&graph).unwrap();
        let decoded: ReferenceGraph = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.edge_count(), graph.edge_count());
        assert_eq!(decoded.outgoing(&g(1)).len(), 1);
        assert_eq!(decoded.incoming(&g(2)), &[g(1)]);
    }
}
