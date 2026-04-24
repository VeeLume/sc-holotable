//! Tag tree — hierarchical tag system from the DCB's `TagDatabase`.
//!
//! Star Citizen's DCB contains a hierarchical tag system with ~18k tag
//! records arranged in a parent-child tree. Tags are referenced from
//! records (weapons, ships, contracts, jurisdictions, ...) via their
//! GUIDs. [`TagTree`] provides:
//!
//! - Lookup by GUID or name
//! - Ancestor / descendant navigation
//! - `is_descendant_of` for hierarchical filtering
//! - A dotted path from the root for display
//!
//! See `docs/sc-extract.md` for the rationale behind preserving the tree
//! structure rather than flattening to a list.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Guid;
use crate::svarog_datacore::DataCoreDatabase;

/// A single node in the [`TagTree`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagNode {
    pub guid: Guid,
    pub name: String,
    pub parent: Option<Guid>,
    pub children: Vec<Guid>,
    /// Legacy numeric id present on some older tags.
    /// `None` on tags that only carry a modern GUID.
    pub legacy_guid: Option<i32>,
}

/// Tree of every tag in the DCB's `TagDatabase`.
///
/// Built once during `parse_from_p4k` and serialized in the snapshot.
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

    /// Insert or replace a node. Maintains both indices.
    pub fn insert(&mut self, node: TagNode) {
        // Remove from the old name index if the guid already exists.
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
    /// order. The starting tag itself is **not** included; the first
    /// yielded node is the starting tag's parent.
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
        // Depth-first via an explicit stack.
        let mut stack: Vec<Guid> = self
            .get(guid)
            .map(|n| n.children.iter().rev().copied().collect())
            .unwrap_or_default();

        std::iter::from_fn(move || {
            while let Some(next) = stack.pop() {
                if let Some(node) = self.by_guid.get(&next) {
                    // Push children in reverse so they pop in original order.
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

    /// Dotted path from the root to this tag, as a list of names.
    /// Example: `["Global", "Manufacturer", "Aegis"]`.
    ///
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

    // ── Construction ────────────────────────────────────────────────────

    /// Walk the DCB and build a complete tag tree.
    ///
    /// `Tag` records carry `tagName`, `children` (array of references to
    /// child tags), and `legacyGUID`. Verified against SC 4.7 LIVE.
    ///
    /// The DCB does **not** store a `parent` field on tag records —
    /// hierarchy is expressed only via outgoing `children` references.
    /// Parent links in [`TagNode`] are derived in a second pass from the
    /// inverse of the children graph, so [`TagTree::path`] and
    /// [`TagTree::ancestors`] work regardless of record shape.
    pub fn from_database(db: &DataCoreDatabase) -> Self {
        let mut tree = Self::new();

        // Pass 1 — insert every tag node, children-only. Parent stays
        // `None` and gets filled in pass 2.
        for record in db.records_by_type("Tag") {
            let guid = record.id();
            let inst = record.as_instance();

            let name = inst.get_str("tagName").unwrap_or("").to_string();
            if name.is_empty() {
                continue;
            }

            let children: Vec<Guid> = inst
                .get_array("children")
                .map(|arr| {
                    arr.filter_map(|v| v.as_record_ref().map(|r| r.guid))
                        .collect()
                })
                .unwrap_or_default();

            let legacy_guid = inst.get_i32("legacyGUID");

            tree.insert(TagNode {
                guid,
                name,
                parent: None,
                children,
                legacy_guid,
            });
        }

        // Pass 2 — derive parent links from the inverse children graph.
        let child_to_parent: Vec<(Guid, Guid)> = tree
            .by_guid
            .iter()
            .flat_map(|(parent_guid, node)| {
                let p = *parent_guid;
                node.children.iter().map(move |child| (*child, p))
            })
            .collect();
        for (child, parent) in child_to_parent {
            if let Some(node) = tree.by_guid.get_mut(&child) {
                node.parent = Some(parent);
            }
        }

        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use svarog_common::CigGuid;

    fn g(byte: u8) -> Guid {
        CigGuid::from_bytes([byte; 16])
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

    /// Build a small test tree:
    ///   root ─┬─ manufacturer ─┬─ aegs
    ///         │                └─ anvl
    ///         └─ race ──────── human
    fn build_sample_tree() -> TagTree {
        let root = g(1);
        let manu = g(2);
        let race = g(3);
        let aegs = g(4);
        let anvl = g(5);
        let human = g(6);

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
    fn new_tree_is_empty() {
        let tree = TagTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn insert_and_lookup_by_guid() {
        let mut tree = TagTree::new();
        tree.insert(node(g(1), "Foo", None, vec![]));
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.get(&g(1)).map(|n| n.name.as_str()), Some("Foo"));
        assert!(tree.get(&g(2)).is_none());
    }

    #[test]
    fn lookup_by_name() {
        let mut tree = TagTree::new();
        tree.insert(node(g(1), "Foo", None, vec![]));
        tree.insert(node(g(2), "Bar", None, vec![]));
        assert_eq!(tree.by_name("Foo"), &[g(1)]);
        assert_eq!(tree.by_name("Bar"), &[g(2)]);
        assert!(tree.by_name("Missing").is_empty());
    }

    #[test]
    fn name_collision_returns_multiple_guids() {
        let mut tree = TagTree::new();
        tree.insert(node(g(1), "Duplicate", None, vec![]));
        tree.insert(node(g(2), "Duplicate", None, vec![]));
        let matches = tree.by_name("Duplicate");
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&g(1)));
        assert!(matches.contains(&g(2)));
    }

    #[test]
    fn insert_replaces_existing_and_updates_name_index() {
        let mut tree = TagTree::new();
        tree.insert(node(g(1), "OldName", None, vec![]));
        assert_eq!(tree.by_name("OldName"), &[g(1)]);

        tree.insert(node(g(1), "NewName", None, vec![]));
        assert_eq!(tree.len(), 1);
        assert!(tree.by_name("OldName").is_empty());
        assert_eq!(tree.by_name("NewName"), &[g(1)]);
    }

    #[test]
    fn roots_returns_only_top_level() {
        let tree = build_sample_tree();
        let roots: Vec<&str> = tree.roots().map(|n| n.name.as_str()).collect();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0], "Global");
    }

    #[test]
    fn ancestors_walks_up_to_root() {
        let tree = build_sample_tree();
        // aegs = g(4), parents are manufacturer → global
        let ancestors: Vec<&str> = tree.ancestors(&g(4)).map(|n| n.name.as_str()).collect();
        assert_eq!(ancestors, vec!["Manufacturer", "Global"]);
    }

    #[test]
    fn ancestors_for_root_is_empty() {
        let tree = build_sample_tree();
        let ancestors: Vec<&TagNode> = tree.ancestors(&g(1)).collect();
        assert!(ancestors.is_empty());
    }

    #[test]
    fn descendants_walks_subtree() {
        let tree = build_sample_tree();
        let descendants: Vec<&str> = tree.descendants(&g(1)).map(|n| n.name.as_str()).collect();
        // Order: depth-first through children as inserted (manu first, then race)
        assert_eq!(
            descendants,
            vec!["Manufacturer", "Aegis", "Anvil", "Race", "Human"]
        );
    }

    #[test]
    fn descendants_for_leaf_is_empty() {
        let tree = build_sample_tree();
        let descendants: Vec<&TagNode> = tree.descendants(&g(4)).collect();
        assert!(descendants.is_empty());
    }

    #[test]
    fn is_descendant_of() {
        let tree = build_sample_tree();
        // aegs is a descendant of manufacturer and global
        assert!(tree.is_descendant_of(&g(4), &g(2)));
        assert!(tree.is_descendant_of(&g(4), &g(1)));
        // aegs is not a descendant of race
        assert!(!tree.is_descendant_of(&g(4), &g(3)));
        // A tag is not a descendant of itself
        assert!(!tree.is_descendant_of(&g(4), &g(4)));
    }

    #[test]
    fn path_from_root_to_leaf() {
        let tree = build_sample_tree();
        assert_eq!(tree.path(&g(4)), vec!["Global", "Manufacturer", "Aegis"]);
        assert_eq!(tree.path(&g(1)), vec!["Global"]);
        assert_eq!(tree.path(&g(6)), vec!["Global", "Race", "Human"]);
    }

    #[test]
    fn path_for_missing_tag_is_empty() {
        let tree = build_sample_tree();
        assert!(tree.path(&g(99)).is_empty());
    }

    #[test]
    fn iter_yields_every_node() {
        let tree = build_sample_tree();
        assert_eq!(tree.iter().count(), 6);
    }

    #[test]
    fn serde_round_trip() {
        let tree = build_sample_tree();
        let json = serde_json::to_string(&tree).unwrap();
        let decoded: TagTree = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.len(), tree.len());
        assert_eq!(decoded.path(&g(4)), tree.path(&g(4)));
        assert!(decoded.is_descendant_of(&g(4), &g(1)));
    }
}
