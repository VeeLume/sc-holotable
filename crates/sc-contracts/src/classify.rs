//! `TagBag` — a resolved set of tags from one of the four `TagList` slots
//! on a `SpawnDescription_Ship` (positive, negative, markup, entity).
//!
//! Every encounter slot exposes its four tag lists symmetrically as
//! [`TagBag`]s. The bag holds raw `Guid`s (escape hatch for consumer-side
//! classification) plus parallel resolved names (cheap to display). All
//! semantic classification is delivered by methods that walk the bag
//! against a [`TagTree`] on demand — no eager bucketing of tags into
//! `Vec<String>` per category.
//!
//! # Subtree taxonomy
//!
//! Tag-tree subtrees that anchor classifier methods. Names are stable
//! across SC patches; if a patch reorganises the hierarchy, these
//! match arms move with it.
//!
//! - `AI ▸ Faction ▸ *` — [`TagBag::factions`] (`Criminal`, `UEE`, …)
//! - `AI ▸ CargoManifest ▸ *` — [`TagBag::cargo`] (`Full Cargo`, `Salvage`, `Bounty`, value tiers)
//! - `AI ▸ Spawning ▸ *` — [`TagBag::spawn_identifiers`] (`Target`, `Defenders`)
//! - `AI ▸ *` (other) — [`TagBag::ai_traits`] (`PoweredOff`, `EngineOff`, `EnableInteractions`, …)
//! - `Missions ▸ *` — [`TagBag::mission_tags`] (`AvailableToSalvage`, size markers)
//!
//! Ship-rooted tags are not surfaced through classifier methods — they
//! drive [`crate::ShipRegistry::resolve_spawn`] resolution, which
//! materialises [`crate::ShipCandidate`]s on the slot. Consumers that
//! want the raw Ship-rooted tag GUIDs use [`TagBag::guids`].

use sc_extract::{DataPools, Guid, TagTree};
use sc_extract::generated::{Handle, TagList};

/// One resolved tag list — the canonical shape exposed for each of the
/// four `SpawnDescription_Ship` tag slots (positive / negative / markup
/// / entity). The two vecs are parallel: `guids[i]` and `names[i]`
/// describe the same tag, sorted primarily by name (display order).
///
/// Tags whose GUID isn't in the live [`TagTree`] are dropped during
/// construction — there's nothing useful a consumer can do with an
/// unresolved GUID, and dropping keeps the parallel-vec invariant
/// trivial.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TagBag {
    /// Resolved tag GUIDs, sorted (by name primarily, by GUID secondary)
    /// and deduplicated.
    pub guids: Vec<Guid>,
    /// Resolved tag names, parallel to [`Self::guids`].
    pub names: Vec<String>,
}

impl TagBag {
    /// Build from a raw GUID set against a [`TagTree`]. GUIDs that don't
    /// resolve are silently dropped. Output is deterministic: sorted by
    /// name with GUID-byte tiebreak, deduplicated by GUID.
    pub fn new(raw_guids: impl IntoIterator<Item = Guid>, tree: &TagTree) -> Self {
        // Dedup by GUID via HashMap, then sort the resulting vec — name
        // is deterministic per GUID, so this collapses true duplicates
        // without losing distinct same-name tags.
        let by_guid: std::collections::HashMap<Guid, String> = raw_guids
            .into_iter()
            .filter_map(|g| tree.get(&g).map(|n| (g, n.name.clone())))
            .collect();
        let mut entries: Vec<(Guid, String)> = by_guid.into_iter().collect();
        // Sort by name (display order), tiebreak on GUID-byte ordering
        // for stable output across runs (CigGuid doesn't impl Ord).
        entries.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.as_bytes().cmp(b.0.as_bytes())));
        let mut guids = Vec::with_capacity(entries.len());
        let mut names = Vec::with_capacity(entries.len());
        for (g, n) in entries {
            guids.push(g);
            names.push(n);
        }
        Self { guids, names }
    }

    /// Convenience: build from an underlying `Handle<TagList>` reference.
    /// Used by extraction code to materialise the four slot tag lists.
    pub fn from_handle(
        pools: &DataPools,
        tree: &TagTree,
        h: Option<&Handle<TagList>>,
    ) -> Self {
        let Some(h) = h else { return Self::default() };
        let Some(list) = h.get(pools) else {
            return Self::default();
        };
        Self::new(list.tags.iter().copied(), tree)
    }

    /// Number of resolved tags.
    pub fn len(&self) -> usize {
        self.guids.len()
    }

    /// True when no tags resolved (or none were provided).
    pub fn is_empty(&self) -> bool {
        self.guids.is_empty()
    }

    /// Iterate `(guid, name)` pairs in stored order (sorted by name).
    pub fn iter(&self) -> impl Iterator<Item = (&Guid, &str)> {
        self.guids.iter().zip(self.names.iter().map(String::as_str))
    }

    /// True if the bag's resolved set contains `name` (exact, case-
    /// sensitive). Cheap shorthand for `bag.names.iter().any(|n| n == name)`.
    pub fn contains_name(&self, name: &str) -> bool {
        self.names.iter().any(|n| n == name)
    }

    // ── Subtree-classified iterators ───────────────────────────────────────
    //
    // Each method walks self.guids, asks the tree for the tag's path,
    // and yields names of tags rooted in the named subtree. No eager
    // bucketing — consumers pay only for the categories they read.

    /// Tags under `AI ▸ Faction ▸ *` (e.g. `Criminal`, `UEE`, `Ninetails`).
    pub fn factions<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str> + 'a {
        self.subtree_iter(tree, "AI", Some("Faction"))
    }

    /// Tags under `AI ▸ CargoManifest ▸ *` (volume tags like `Full Cargo`,
    /// value tiers, `Bounty`, `Salvage` — the latter two live here despite
    /// their semantic role-marker meaning).
    pub fn cargo<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str> + 'a {
        self.subtree_iter(tree, "AI", Some("CargoManifest"))
    }

    /// Tags under `AI ▸ Spawning ▸ *` (`Target`, `Defenders`). Canonical
    /// typed role markers in 4.7 — 3,073 + 318 references across all
    /// ship spawns.
    pub fn spawn_identifiers<'a>(
        &'a self,
        tree: &'a TagTree,
    ) -> impl Iterator<Item = &'a str> + 'a {
        self.subtree_iter(tree, "AI", Some("Spawning"))
    }

    /// Tags under `AI ▸ *` that aren't Faction / CargoManifest / Spawning.
    /// Includes power-state traits (`PoweredOff`, `EngineOff`,
    /// `EnableInteractions`, `DisablePowerInteractions`, `EmptyCrew`,
    /// `Unmanned`), value tiers, legality (`Legal` / `Illegal`).
    pub fn ai_traits<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str> + 'a {
        self.guids.iter().enumerate().filter_map(move |(i, g)| {
            let path = tree.path(g);
            match root_and_category(&path) {
                ("AI", Some("Faction"))
                | ("AI", Some("CargoManifest"))
                | ("AI", Some("Spawning")) => None,
                ("AI", _) => Some(self.names[i].as_str()),
                _ => None,
            }
        })
    }

    /// Tags under `Missions ▸ *` (`AvailableToSalvage`, size markers,
    /// mission-type flags).
    pub fn mission_tags<'a>(
        &'a self,
        tree: &'a TagTree,
    ) -> impl Iterator<Item = &'a str> + 'a {
        self.subtree_iter(tree, "Missions", None)
    }

    /// Tags under any other root the bag contains — catch-all so nothing
    /// is silently invisible. Excludes `Ship` (consumed by ShipRegistry)
    /// and the AI / Missions subtrees handled by named methods.
    pub fn other<'a>(&'a self, tree: &'a TagTree) -> impl Iterator<Item = &'a str> + 'a {
        self.guids.iter().enumerate().filter_map(move |(i, g)| {
            let path = tree.path(g);
            match root_and_category(&path) {
                ("Ship", _) | ("AI", _) | ("Missions", _) | ("", _) => None,
                _ => Some(self.names[i].as_str()),
            }
        })
    }

    /// Runtime spawn directives — name-prefix match on `Arrive*` /
    /// `Ignore*` (`ArriveViaQT`, `IgnoreCrimes`, `IgnoreHostility`).
    /// These are engine-resolved at spawn time; capturing them tells
    /// the consumer the engine's intent.
    pub fn directives(&self) -> impl Iterator<Item = &str> {
        self.names.iter().filter_map(|n| {
            (n.starts_with("Arrive") || n.starts_with("Ignore")).then_some(n.as_str())
        })
    }

    /// Highest pilot skill level parsed from `HumanPilotN` tag names
    /// (10, 20, …, 100). [`None`] if no `HumanPilotN` tag is present.
    pub fn ai_skill(&self) -> Option<u32> {
        self.names.iter().filter_map(|n| parse_ai_skill(n)).max()
    }

    /// True if the `AcePilot` tag (top-tier AI) is present.
    pub fn ace_pilot(&self) -> bool {
        self.contains_name("AcePilot")
    }

    // ── Discovery-driven heuristic predicates ──────────────────────────────
    //
    // Derived from the encounter_analytics finding (SC 4.7): cargo-
    // recovery and salvage missions are distinguishable by AI-trait
    // patterns. Predicates check name presence directly — no tag-tree
    // lookup needed since the relevant traits are name-stable.

    /// Pre-damaged ship the player can interact with (board, loot the
    /// cargo). 100% of `CargoShip_BP` slots match in 4.7. See
    /// `examples/encounter_analytics.rs` §2 for the full fingerprint.
    pub fn is_cargo_recovery(&self) -> bool {
        self.contains_name("EnableInteractions") && self.contains_name("EmptyCrew")
    }

    /// Pre-damaged wreck whose interaction is locked off (the wreck
    /// itself is the salvage objective). 100% of `SalvageSpawnDescription_BP`
    /// slots match in 4.7.
    pub fn is_pre_damaged_wreck(&self) -> bool {
        self.contains_name("DisablePowerInteractions")
    }

    /// `Missions ▸ MissionType ▸ Salvage ▸ AvailableToSalvage` tag
    /// present — explicit typed marker for "this ship is a salvage
    /// objective". Stronger than [`Self::is_pre_damaged_wreck`] which
    /// only checks the power-state pattern.
    pub fn is_salvage_target(&self) -> bool {
        self.contains_name("AvailableToSalvage")
    }

    // ── Internal helpers ───────────────────────────────────────────────────

    /// Iterate names whose tag-tree path matches `(root, Some(category))`
    /// or `(root, None)` (any first category) when `category == None`.
    fn subtree_iter<'a>(
        &'a self,
        tree: &'a TagTree,
        root: &'a str,
        category: Option<&'a str>,
    ) -> impl Iterator<Item = &'a str> + 'a {
        self.guids.iter().enumerate().filter_map(move |(i, g)| {
            let path = tree.path(g);
            let (r, c) = root_and_category(&path);
            if r == root && category.is_none_or(|want| c == Some(want)) {
                Some(self.names[i].as_str())
            } else {
                None
            }
        })
    }
}

/// Parse `HumanPilot10`, `HumanPilot20`, …, `HumanPilot100` → the level
/// as `u32`. Returns [`None`] for anything that doesn't match the exact
/// `HumanPilotN` pattern.
pub fn parse_ai_skill(name: &str) -> Option<u32> {
    name.strip_prefix("HumanPilot")
        .and_then(|rest| rest.parse::<u32>().ok())
}

/// Inspect a tag-tree path and return its root name plus the first
/// category below the root. Example:
///
/// `["AI", "Faction", "Criminal"]` → `("AI", Some("Faction"))`
/// `["Ship", "Model", "Cutter"]`   → `("Ship", Some("Model"))`
/// `[]`                            → `("", None)`
fn root_and_category<'a>(path: &'a [&'a str]) -> (&'a str, Option<&'a str>) {
    match path {
        [] => ("", None),
        [root] => (root, None),
        [root, cat, ..] => (root, Some(cat)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_skill_recognises_valid_pilot_levels() {
        assert_eq!(parse_ai_skill("HumanPilot10"), Some(10));
        assert_eq!(parse_ai_skill("HumanPilot100"), Some(100));
        assert_eq!(parse_ai_skill("HumanPilot"), None);
        assert_eq!(parse_ai_skill("AcePilot"), None);
        assert_eq!(parse_ai_skill("Criminal"), None);
        assert_eq!(parse_ai_skill(""), None);
    }

    #[test]
    fn root_and_category_shapes() {
        assert_eq!(root_and_category(&[]), ("", None));
        assert_eq!(root_and_category(&["AI"]), ("AI", None));
        assert_eq!(
            root_and_category(&["AI", "Faction"]),
            ("AI", Some("Faction"))
        );
        assert_eq!(
            root_and_category(&["AI", "Faction", "Criminal"]),
            ("AI", Some("Faction"))
        );
    }

    #[test]
    fn empty_bag_reports_empty() {
        let bag = TagBag::default();
        assert!(bag.is_empty());
        assert_eq!(bag.len(), 0);
        assert_eq!(bag.ai_skill(), None);
        assert!(!bag.ace_pilot());
        assert!(!bag.is_cargo_recovery());
    }

    #[test]
    fn contains_name_matches_exactly() {
        let bag = TagBag {
            guids: vec![],
            names: vec!["AcePilot".into(), "Criminal".into()],
        };
        assert!(bag.contains_name("AcePilot"));
        assert!(bag.contains_name("Criminal"));
        assert!(!bag.contains_name("acepilot"));
        assert!(!bag.contains_name("Ace"));
    }

    #[test]
    fn cargo_recovery_predicate_requires_both_markers() {
        let neither = TagBag::default();
        assert!(!neither.is_cargo_recovery());

        let one = TagBag {
            guids: vec![],
            names: vec!["EnableInteractions".into()],
        };
        assert!(!one.is_cargo_recovery());

        let both = TagBag {
            guids: vec![],
            names: vec!["EnableInteractions".into(), "EmptyCrew".into()],
        };
        assert!(both.is_cargo_recovery());
    }

    #[test]
    fn pre_damaged_wreck_predicate_checks_disable_power() {
        let bag = TagBag {
            guids: vec![],
            names: vec!["DisablePowerInteractions".into()],
        };
        assert!(bag.is_pre_damaged_wreck());
        assert!(!bag.is_cargo_recovery());
    }

    #[test]
    fn ai_skill_picks_max_when_multiple_pilot_tags() {
        let bag = TagBag {
            guids: vec![],
            names: vec![
                "HumanPilot20".into(),
                "HumanPilot80".into(),
                "HumanPilot40".into(),
            ],
        };
        assert_eq!(bag.ai_skill(), Some(80));
    }

    #[test]
    fn directives_filters_by_prefix() {
        let bag = TagBag {
            guids: vec![],
            names: vec![
                "ArriveViaQT".into(),
                "IgnoreCrimes".into(),
                "Criminal".into(),
                "AcePilot".into(),
            ],
        };
        let dirs: Vec<&str> = bag.directives().collect();
        assert_eq!(dirs, vec!["ArriveViaQT", "IgnoreCrimes"]);
    }
}
