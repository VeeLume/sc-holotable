//! Record paths — every DCB record's file path, name, and type, indexed by
//! GUID and navigable as a `/`-segment tree.
//!
//! The DataCore is a file tree: every record carries a `file_name()` like
//! `libs/foundry/records/entities/scitem/ships/cooler/cool_..._scitem.xml`.
//! That path *is* a classification axis — `scitemmanufacturer/personalweapons`
//! vs `scitemmanufacturer/paintcolorlogos` distinguishes a genuine
//! manufacturer from a paint/logo entry. svarog hands the path off
//! `db.all_records()`, so [`RecordPaths::build`] captures it once (in the same
//! single walk) instead of every consumer re-deriving it.
//!
//! Two addressing modes:
//! - **by GUID** — [`RecordPaths::get`] → [`RecordPath`]. The domain crates
//!   already hold records' GUIDs; this gives each one its path/name/type.
//! - **by path** — [`RecordPaths::at`] (exact path) / [`RecordPaths::under`]
//!   (whole subtree) over a `/`-segment trie, plus [`RecordPaths::children`] /
//!   [`RecordPaths::roots`] for structural navigation. This is the
//!   "understand the DCB layout" surface.
//!
//! # Owned, not borrowing
//!
//! `RecordPaths` owns its strings so it outlives the [`Datacore`] it was built
//! from and serializes standalone — it travels in the processed snapshot
//! alongside the other cooked indices. Type names are interned in a small
//! `struct_index -> name` side table rather than copied onto every
//! [`RecordPath`] (there are ~600–6000 distinct types but ~115k records).
//!
//! # Names are non-unique
//!
//! `RecordPath::name` is svarog's record name, which is **not** unique across
//! the DCB — it rides along for debug (`guid -> name`) but gets no reverse
//! index. `file_name` (the trie key) is the unique-ish address.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::{Datacore, Guid};

/// What [`RecordPaths`] knows about a single DCB record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordPath {
    /// The record's unique GUID.
    pub guid: Guid,
    /// svarog record name. **Non-unique** across the DCB — for debug /
    /// display only; no reverse index is built for it.
    pub name: String,
    /// DCB struct type index. Resolve to a type name via
    /// [`RecordPaths::type_name`].
    pub struct_index: u32,
    /// True for the one main record per file (the rest are sub-records that
    /// share the file path).
    pub is_main: bool,
    /// Full file path including the filename — the trie key. Example:
    /// `libs/foundry/records/scitemmanufacturer/personalweapons/arma.xml`.
    pub path: String,
}

/// One node in the `/`-segment path trie. Stored in an arena
/// ([`RecordPaths::nodes`]); children reference each other by index.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct TrieNode {
    /// Child path segment -> node index in the arena. `BTreeMap` keeps
    /// navigation order deterministic (stable iteration + serialization).
    children: BTreeMap<String, usize>,
    /// GUIDs of records whose full path ends exactly at this node. Usually
    /// one (the `.xml` leaf), but a file holding a main record plus
    /// sub-records puts several here.
    records: Vec<Guid>,
}

/// Every DCB record's path/name/type, indexed by GUID and navigable as a
/// `/`-segment tree. Build once via [`RecordPaths::build`], share by
/// reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPaths {
    by_guid: HashMap<Guid, RecordPath>,
    /// `struct_index -> type name` (interned; far fewer types than records).
    type_names: HashMap<u32, String>,
    /// Trie arena. Index 0 is the synthetic root (empty segment).
    nodes: Vec<TrieNode>,
}

impl Default for RecordPaths {
    fn default() -> Self {
        Self {
            by_guid: HashMap::new(),
            type_names: HashMap::new(),
            // Index 0 = synthetic root.
            nodes: vec![TrieNode::default()],
        }
    }
}

/// Split a DCB file path into non-empty segments, tolerating either slash.
fn segments(path: &str) -> impl Iterator<Item = &str> {
    path.split(['/', '\\']).filter(|s| !s.is_empty())
}

impl RecordPaths {
    /// Construct an empty index (just the synthetic trie root).
    pub fn new() -> Self {
        Self::default()
    }

    /// Build from a parsed [`Datacore`] by walking `db.all_records()` once,
    /// capturing each record's path, name, type index, and main-flag.
    ///
    /// Unlike the domain builders this takes `&Datacore` rather than
    /// `&RecordStore`: the path/name strings live on the raw `db`, not in the
    /// typed pools.
    pub fn build(datacore: &Datacore) -> Self {
        let mut paths = Self::new();
        for record in datacore.db().all_records() {
            paths.ingest(&record);
        }
        paths
    }

    /// Capture one raw svarog record (path/name/type/main-flag). Shared by
    /// [`RecordPaths::build`] and [`RecordPathsBuilder`] so both go through
    /// one code path.
    fn ingest(&mut self, record: &crate::svarog_datacore::Record<'_>) {
        let struct_index = record.struct_index();
        self.type_names
            .entry(struct_index)
            .or_insert_with(|| record.type_name().unwrap_or_default().to_string());
        self.insert(RecordPath {
            guid: record.id(),
            name: record.name().unwrap_or_default().to_string(),
            struct_index,
            is_main: record.is_main(),
            path: record.file_name().unwrap_or_default().to_string(),
        });
    }

    /// Insert a record ref, threading its path into the trie. Assumes each
    /// GUID is inserted once (as [`RecordPaths::build`] does); re-inserting a
    /// GUID re-adds it to the trie.
    pub fn insert(&mut self, r: RecordPath) {
        self.insert_trie(&r.path, r.guid);
        self.by_guid.insert(r.guid, r);
    }

    /// Walk/create the trie path for `path` and append `guid` to the terminal
    /// node. Index dance avoids holding a `&self.nodes` borrow across the
    /// `push`/`insert` mutations.
    fn insert_trie(&mut self, path: &str, guid: Guid) {
        let mut idx = 0usize;
        for seg in segments(path) {
            idx = match self.nodes[idx].children.get(seg) {
                Some(&child) => child,
                None => {
                    let new_idx = self.nodes.len();
                    self.nodes.push(TrieNode::default());
                    self.nodes[idx].children.insert(seg.to_string(), new_idx);
                    new_idx
                }
            };
        }
        self.nodes[idx].records.push(guid);
    }

    /// Resolve the trie node for a path, or `None` if no such path exists.
    fn node_for(&self, path: &str) -> Option<usize> {
        let mut idx = 0usize;
        for seg in segments(path) {
            idx = *self.nodes[idx].children.get(seg)?;
        }
        Some(idx)
    }

    /// Look up a record's path/name/type by GUID.
    pub fn get(&self, guid: &Guid) -> Option<&RecordPath> {
        self.by_guid.get(guid)
    }

    /// Resolve a [`RecordPath::struct_index`] to its DCB type name.
    pub fn type_name(&self, struct_index: u32) -> Option<&str> {
        self.type_names
            .get(&struct_index)
            .map(String::as_str)
            .filter(|s| !s.is_empty())
    }

    /// GUIDs of records whose full path is exactly `path`. Empty if no record
    /// lives at that exact path.
    pub fn at(&self, path: &str) -> &[Guid] {
        self.node_for(path)
            .map(|idx| self.nodes[idx].records.as_slice())
            .unwrap_or(&[])
    }

    /// Every GUID at or below `prefix`, depth-first. An empty prefix yields
    /// the whole tree. Order within a directory follows the `BTreeMap` key
    /// order of the trie.
    pub fn under<'a>(&'a self, prefix: &str) -> impl Iterator<Item = &'a Guid> + 'a {
        let mut stack: Vec<usize> = self.node_for(prefix).into_iter().collect();
        let empty: &'a [Guid] = &[];
        let mut pending = empty.iter();
        std::iter::from_fn(move || {
            loop {
                if let Some(g) = pending.next() {
                    return Some(g);
                }
                let idx = stack.pop()?;
                let node = &self.nodes[idx];
                stack.extend(node.children.values().copied());
                pending = node.records.iter();
            }
        })
    }

    /// The child path segments directly under `path` (one level down).
    pub fn children<'a>(&'a self, path: &str) -> impl Iterator<Item = &'a str> + 'a {
        self.node_for(path)
            .into_iter()
            .flat_map(move |idx| self.nodes[idx].children.keys().map(String::as_str))
    }

    /// The top-level path segments (children of the synthetic root).
    pub fn roots(&self) -> impl Iterator<Item = &str> + '_ {
        self.nodes[0].children.keys().map(String::as_str)
    }

    /// Iterate over every [`RecordPath`]. Order is unspecified.
    pub fn iter(&self) -> impl Iterator<Item = &RecordPath> + '_ {
        self.by_guid.values()
    }

    /// Total number of records indexed.
    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }
}

/// [`crate::RecordVisitor`] that builds a [`RecordPaths`] in a bundled walk.
///
/// Declares [`crate::Interest::AllRecords`] and reads each record's raw
/// path/name/type. Equivalent to [`RecordPaths::build`] but fusible with other
/// visitors in a single pass.
#[derive(Default)]
pub struct RecordPathsBuilder {
    inner: RecordPaths,
}

impl crate::RecordVisitor for RecordPathsBuilder {
    type Output = RecordPaths;

    fn interest(&self) -> crate::Interest {
        crate::Interest::AllRecords
    }

    fn visit(&mut self, item: crate::VisitItem<'_>) {
        if let Some(record) = item.raw {
            self.inner.ingest(&record);
        }
    }

    fn finish(self) -> RecordPaths {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(byte: u8) -> Guid {
        Guid::from_bytes([byte; 16])
    }

    fn rref(guid: Guid, path: &str, name: &str) -> RecordPath {
        RecordPath {
            guid,
            name: name.to_string(),
            struct_index: 0,
            is_main: true,
            path: path.to_string(),
        }
    }

    /// libs/foundry/records/scitemmanufacturer/{personalweapons/arma.xml,
    ///                                          paintcolorlogos/paint.xml}
    /// libs/foundry/records/entities/scitem/ships/cooler/cool.xml
    fn sample() -> RecordPaths {
        let mut p = RecordPaths::new();
        p.insert(rref(
            g(1),
            "libs/foundry/records/scitemmanufacturer/personalweapons/arma.xml",
            "ARMA",
        ));
        p.insert(rref(
            g(2),
            "libs/foundry/records/scitemmanufacturer/paintcolorlogos/paint.xml",
            "PAINT",
        ));
        p.insert(rref(
            g(3),
            "libs/foundry/records/entities/scitem/ships/cooler/cool.xml",
            "COOL",
        ));
        p
    }

    #[test]
    fn guid_lookup() {
        let p = sample();
        assert_eq!(p.get(&g(1)).map(|r| r.name.as_str()), Some("ARMA"));
        assert_eq!(
            p.get(&g(3)).map(|r| r.path.as_str()),
            Some("libs/foundry/records/entities/scitem/ships/cooler/cool.xml")
        );
        assert!(p.get(&g(9)).is_none());
        assert_eq!(p.len(), 3);
    }

    #[test]
    fn exact_path() {
        let p = sample();
        assert_eq!(
            p.at("libs/foundry/records/scitemmanufacturer/personalweapons/arma.xml"),
            &[g(1)]
        );
        assert!(p.at("libs/foundry/records/nope.xml").is_empty());
        // Interior (non-leaf) path holds no records of its own.
        assert!(p.at("libs/foundry/records/scitemmanufacturer").is_empty());
    }

    #[test]
    fn subtree_and_classification() {
        let p = sample();
        // The manufacturer-kind win: paths under different prefixes separate
        // genuine manufacturers from paint/logo entries.
        let manus: Vec<_> = p
            .under("libs/foundry/records/scitemmanufacturer/personalweapons")
            .copied()
            .collect();
        assert_eq!(manus, vec![g(1)]);

        let all_mfr: std::collections::HashSet<_> =
            p.under("libs/foundry/records/scitemmanufacturer").copied().collect();
        assert_eq!(all_mfr, [g(1), g(2)].into_iter().collect());

        // Whole tree.
        assert_eq!(p.under("").count(), 3);
    }

    #[test]
    fn navigation() {
        let p = sample();
        assert_eq!(p.roots().collect::<Vec<_>>(), vec!["libs"]);
        let kids: Vec<_> = p.children("libs/foundry/records").collect();
        // BTreeMap order: entities, scitemmanufacturer.
        assert_eq!(kids, vec!["entities", "scitemmanufacturer"]);
    }

    #[test]
    fn serde_round_trip() {
        let p = sample();
        let json = serde_json::to_string(&p).unwrap();
        let decoded: RecordPaths = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.len(), p.len());
        assert_eq!(decoded.at(
            "libs/foundry/records/scitemmanufacturer/paintcolorlogos/paint.xml"
        ), &[g(2)]);
        assert_eq!(
            decoded.under("libs/foundry/records/scitemmanufacturer").count(),
            2
        );
    }
}
