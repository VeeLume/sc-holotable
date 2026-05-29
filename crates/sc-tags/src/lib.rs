//! Tag tree — hierarchical tag system from the DCB's `TagDatabase`.
//!
//! Star Citizen's DCB contains a hierarchical tag system with ~18k tag
//! records arranged in a parent-child tree. Tags are referenced from
//! records (weapons, ships, contracts, jurisdictions, …) via their GUIDs.
//! Moved out of `sc-extract` (domain data, not a generic DCB primitive);
//! build it explicitly via [`TagTree::build`].
//!
//! [`TagTree`] provides:
//! - Lookup by GUID or name
//! - Ancestor / descendant navigation
//! - `is_descendant_of` for hierarchical filtering
//! - A path from the root ([`TagTree::path`])
//!
//! # The path *is* the semantics
//!
//! A tag's *use* is encoded in its position, not in any typed field — e.g.
//! `AI > Ship > SpawnFlags > …` is runtime spawn-state, while
//! `Manufacturer > Aegis` is entity identity. Consumers classify by walking
//! the path (`path` / `ancestors` / `is_descendant_of`); that's why the tree
//! structure is preserved rather than flattened, and why this stays a raw
//! structural walk (no generated-type enrichment to gain).

use std::collections::HashMap;

use sc_extract::generated::{RecordLookup, Tag};
use sc_extract::{Guid, RecordStore};
use serde::{Deserialize, Serialize};

/// A single node in the [`TagTree`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagNode {
    pub guid: Guid,
    pub name: String,
    pub parent: Option<Guid>,
    pub children: Vec<Guid>,
    /// Legacy numeric id present on some older tags.
    pub legacy_guid: Option<i32>,
}

/// Tree of every tag in the DCB's `TagDatabase`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagTree {
    by_guid: HashMap<Guid, TagNode>,
    /// Index from tag name to all GUIDs using that name. Name collisions
    /// exist in the real DCB — multiple tags can share a display name in
    /// different parts of the tree.
    by_name: HashMap<String, Vec<Guid>>,
}

impl TagTree {
    /// Construct an empty tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the tree from a parsed [`RecordStore`] via the typed `Tag` pool
    /// (`tagdatabase` feature). Build once, share by reference.
    ///
    /// `Tag` carries `tagName`, `children` (GUID references), and
    /// `legacyGUID`. The DCB stores no `parent` field — hierarchy is the
    /// inverse of the children graph, derived in a second pass. (`TagDatabase`
    /// is the root list of every tag GUID; iterating the `Tag` pool yields
    /// the same complete set more directly.)
    pub fn build(store: &RecordStore) -> Self {
        let pools = &store.pools;
        let mut tree = Self::new();
        // Pass 1 — insert every tag node, children-only.
        for (&guid, &handle) in &store.records.multi_feature.tag {
            let Some(tag) = handle.get(pools) else {
                continue;
            };
            if let Some(node) = node_for(guid, tag) {
                tree.insert(node);
            }
        }
        // Pass 2 — derive parent links from the inverse children graph.
        tree.derive_parents();
        tree
    }

    /// Pass 2 of the build: derive each node's `parent` from the inverse of
    /// the children graph. Run once after all nodes are inserted (the
    /// standalone build does this inline; [`TagTreeBuilder`] does it in
    /// `finish`).
    fn derive_parents(&mut self) {
        let child_to_parent: Vec<(Guid, Guid)> = self
            .by_guid
            .iter()
            .flat_map(|(parent_guid, node)| {
                let p = *parent_guid;
                node.children.iter().map(move |child| (*child, p))
            })
            .collect();
        for (child, parent) in child_to_parent {
            if let Some(node) = self.by_guid.get_mut(&child) {
                node.parent = Some(parent);
            }
        }
    }

    /// Insert or replace a node. Maintains both indices.
    pub fn insert(&mut self, node: TagNode) {
        if let Some(existing) = self.by_guid.get(&node.guid)
            && let Some(guids) = self.by_name.get_mut(&existing.name)
        {
            guids.retain(|g| g != &node.guid);
            if guids.is_empty() {
                self.by_name.remove(&existing.name);
            }
        }

        self.by_name
            .entry(node.name.clone())
            .or_default()
            .push(node.guid);
        self.by_guid.insert(node.guid, node);
    }

    /// Look up a node by GUID.
    pub fn get(&self, guid: &Guid) -> Option<&TagNode> {
        self.by_guid.get(guid)
    }

    /// Look up all tag GUIDs with a given name. Name collisions produce
    /// multiple results.
    pub fn by_name(&self, name: &str) -> &[Guid] {
        self.by_name.get(name).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Iterate over every root node (nodes with no parent).
    pub fn roots(&self) -> impl Iterator<Item = &TagNode> + '_ {
        self.by_guid.values().filter(|n| n.parent.is_none())
    }

    /// Iterate over every node in the tree. Order is unspecified.
    pub fn iter(&self) -> impl Iterator<Item = &TagNode> + '_ {
        self.by_guid.values()
    }

    /// Total number of tags in the tree.
    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }

    /// Walk upward from `guid` to the root, yielding each ancestor in
    /// order. The starting tag itself is **not** included.
    pub fn ancestors<'a>(&'a self, guid: &Guid) -> impl Iterator<Item = &'a TagNode> + 'a {
        let mut current = self.get(guid).and_then(|n| n.parent);
        std::iter::from_fn(move || {
            let parent_guid = current?;
            let parent = self.by_guid.get(&parent_guid)?;
            current = parent.parent;
            Some(parent)
        })
    }

    /// Walk downward from `guid` depth-first, yielding each descendant.
    /// The starting tag itself is **not** included.
    pub fn descendants<'a>(&'a self, guid: &Guid) -> impl Iterator<Item = &'a TagNode> + 'a {
        let mut stack: Vec<Guid> = self
            .get(guid)
            .map(|n| n.children.iter().rev().copied().collect())
            .unwrap_or_default();

        std::iter::from_fn(move || {
            while let Some(next) = stack.pop() {
                if let Some(node) = self.by_guid.get(&next) {
                    for child in node.children.iter().rev() {
                        stack.push(*child);
                    }
                    return Some(node);
                }
            }
            None
        })
    }

    /// True if `guid` is a (transitive) descendant of `ancestor`.
    /// Returns false if `guid == ancestor`.
    pub fn is_descendant_of(&self, guid: &Guid, ancestor: &Guid) -> bool {
        self.ancestors(guid).any(|n| &n.guid == ancestor)
    }

    /// Path from the root to this tag, as a list of names.
    /// Example: `["Global", "Manufacturer", "Aegis"]`.
    /// Returns an empty vec if the tag isn't in the tree.
    pub fn path(&self, guid: &Guid) -> Vec<&str> {
        let Some(node) = self.get(guid) else {
            return Vec::new();
        };
        let mut stack: Vec<&str> = vec![node.name.as_str()];
        for ancestor in self.ancestors(guid) {
            stack.push(ancestor.name.as_str());
        }
        stack.reverse();
        stack
    }
}

/// Build a children-only [`TagNode`] from a typed `Tag` record, or `None` for
/// an unnamed tag. Shared by [`TagTree::build`] and [`TagTreeBuilder`] (parent
/// links are derived in a later pass).
fn node_for(guid: Guid, tag: &Tag) -> Option<TagNode> {
    if tag.tag_name.is_empty() {
        return None;
    }
    Some(TagNode {
        guid,
        name: tag.tag_name.clone(),
        parent: None,
        children: tag.children.clone(),
        // 0 = no legacy id (modern-GUID-only tags).
        legacy_guid: (tag.legacy_guid != 0).then_some(tag.legacy_guid as i32),
    })
}

/// [`sc_extract::RecordVisitor`] that builds a [`TagTree`] in a bundled walk.
/// Declares interest in `Tag` records, accumulating nodes during the walk and
/// deriving parent links in `finish`. Equivalent to [`TagTree::build`] but
/// fusible with other visitors in one pass.
#[derive(Default)]
pub struct TagTreeBuilder {
    inner: TagTree,
}

impl sc_extract::RecordVisitor for TagTreeBuilder {
    type Output = TagTree;

    fn interest(&self) -> sc_extract::Interest {
        sc_extract::Interest::Types(&["Tag"])
    }

    fn visit(&mut self, item: sc_extract::VisitItem<'_>) {
        let store = item.store;
        let Some(handle) = Tag::lookup(&store.records, &item.guid) else {
            return;
        };
        let Some(tag) = handle.get(&store.pools) else {
            return;
        };
        if let Some(node) = node_for(item.guid, tag) {
            self.inner.insert(node);
        }
    }

    fn finish(mut self) -> TagTree {
        self.inner.derive_parents();
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(byte: u8) -> Guid {
        Guid::from_bytes([byte; 16])
    }

    fn node(guid: Guid, name: &str, parent: Option<Guid>, children: Vec<Guid>) -> TagNode {
        TagNode {
            guid,
            name: name.to_string(),
            parent,
            children,
            legacy_guid: None,
        }
    }

    /// root ─┬─ manufacturer ─┬─ aegs
    ///       │                └─ anvl
    ///       └─ race ──────── human
    fn build_sample_tree() -> TagTree {
        let (root, manu, race, aegs, anvl, human) = (g(1), g(2), g(3), g(4), g(5), g(6));
        let mut tree = TagTree::new();
        tree.insert(node(root, "Global", None, vec![manu, race]));
        tree.insert(node(manu, "Manufacturer", Some(root), vec![aegs, anvl]));
        tree.insert(node(race, "Race", Some(root), vec![human]));
        tree.insert(node(aegs, "Aegis", Some(manu), vec![]));
        tree.insert(node(anvl, "Anvil", Some(manu), vec![]));
        tree.insert(node(human, "Human", Some(race), vec![]));
        tree
    }

    #[test]
    fn lookup_and_name_collision() {
        let mut tree = TagTree::new();
        tree.insert(node(g(1), "Dup", None, vec![]));
        tree.insert(node(g(2), "Dup", None, vec![]));
        assert_eq!(tree.get(&g(1)).map(|n| n.name.as_str()), Some("Dup"));
        assert_eq!(tree.by_name("Dup").len(), 2);
        assert!(tree.by_name("Missing").is_empty());
    }

    #[test]
    fn ancestors_descendants_path() {
        let tree = build_sample_tree();
        assert_eq!(
            tree.ancestors(&g(4))
                .map(|n| n.name.as_str())
                .collect::<Vec<_>>(),
            vec!["Manufacturer", "Global"]
        );
        assert_eq!(tree.path(&g(4)), vec!["Global", "Manufacturer", "Aegis"]);
        assert!(tree.is_descendant_of(&g(4), &g(1)));
        assert!(!tree.is_descendant_of(&g(4), &g(3)));
        assert!(!tree.is_descendant_of(&g(4), &g(4)));
        assert_eq!(tree.descendants(&g(1)).count(), 5);
        assert_eq!(tree.roots().count(), 1);
    }

    #[test]
    fn serde_round_trip() {
        let tree = build_sample_tree();
        let json = serde_json::to_string(&tree).unwrap();
        let decoded: TagTree = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.len(), tree.len());
        assert_eq!(decoded.path(&g(4)), tree.path(&g(4)));
    }
}
