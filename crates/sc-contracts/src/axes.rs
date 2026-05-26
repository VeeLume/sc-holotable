//! Tag-axis classification for [`crate::SlotGroup`] alternatives.
//!
//! When a `SpawnDescription_ShipOptions` has more than one inner
//! `option`, those options are weighted alternatives — the engine
//! picks ONE per spawn (the picker likely uses player profile /
//! party size / RNG). Display code needs to know **what varies**
//! across the alternatives to render them honestly:
//!
//! - Pure skill/difficulty scaling (HumanPilotNN, CombatClass) is
//!   render-noise — collapse to a range or omit when implied.
//! - Ship-class / hull / spawn-flag / effect differences are
//!   player-relevant variance — surface as distinct alternatives.
//!
//! This module classifies tags by walking their [`TagTree`] path
//! into a small set of [`AxisKind`] families. The classifier is
//! pure (no I/O), data-driven (path-prefix matching, no hardcoded
//! tag names), and forward-compatible (unknown tags land in
//! [`AxisKind::Other`]).
//!
//! Family list lives in [`AxisKind::for_path`]. Adding a new family
//! is a one-line addition.
//!
//! # API shape
//!
//! Each [`crate::SlotGroup`] carries an [`AxisDiff`] populated by
//! [`AxisDiff::compute`] from the per-option tag sets. Consumers
//! (sc-langpatch's renderer, sc-explorer's TUI, ...) walk the
//! [`AxisDiff`]'s [`AxisValues`] fields to discover what to display.

use std::collections::{BTreeMap, HashSet};

use sc_extract::{Guid, TagTree};

/// Classification of a single tag by the tag-tree family it lives in.
///
/// Computed by [`AxisKind::for_path`] from the tag's [`TagTree::path`].
/// Renderers use this to bucket variance into player-meaningful axes.
///
/// # Ordering convention
///
/// Members appear in roughly "display priority" order — the variants
/// most likely to drive a "one of:" rendering decision come first
/// (`Hull`, `ShipClass`, `Effect`, `SpawnFlags`) before scaling axes
/// (`Skill`, `CombatClass`). This is just a stylistic hint for code
/// review — there's no semantic dependence on the enum order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AxisKind {
    /// Specific ship hull or series. `Ship / Model / *`,
    /// `Ship / Series / *`. Examples: `Scythe`, `Blade`, `135c`,
    /// `Avenger_Titan`. The "which actual ship" axis.
    Hull,
    /// Mission-side ship class. `Missions / VehicleType / Ship / *`.
    /// Examples: `CombatShip`, `LargeCombatShip`, `HeavyInterceptor`,
    /// `Distortion`. Coarser than [`Self::Hull`].
    ShipClass,
    /// Entity-effect tag. `EntityEffectSystem / Tags / *`.
    /// `Distortion` appears here as a weapon-effect marker (it also
    /// shows under `ShipClass` — both classifications surface the same
    /// player-meaningful "this squad uses distortion" axis).
    Effect,
    /// Spawn behavior flag. `AI / Ship / SpawnFlags / *`,
    /// `AI / Ship / ShipState / *`. Examples: `ArriveViaQT`,
    /// `PoweredOff`, `EngineOff`.
    SpawnFlags,
    /// Faction marker. `AI / Faction / *`, `AI / Race / *`,
    /// `Global / Race / *`. Examples: `Criminal`, `Vanduul`,
    /// `XenoThreat`, `Ninetails`.
    Faction,
    /// Cargo amount. `AI / CargoManifest / * / PopulationVariation / *`.
    /// Examples: `Full Cargo`, `Half Cargo`, `Scraps Cargo`.
    CargoSize,
    /// Cargo value tier. `AI / CargoManifest / * / Value / *`.
    /// Examples: `LowValue`, `MediumValue`, `HighValue`.
    Value,
    /// AI pilot skill knob — fine scaling axis.
    /// `AI / SkillDefinitions / Description / *`,
    /// `AI / SkillLevel / *`. Examples: `HumanPilot10`,
    /// `HumanPilot30`, ..., `HumanPilot90`.
    Skill,
    /// Coarse combat-class difficulty tier.
    /// `AI / Ship / CombatClass / *`. Examples: `VeryEasy`, `Easy`,
    /// `Medium`, `Hard`, `VeryHard`, `Super`.
    CombatClass,
    /// Spawn role identifier — usually shared across siblings so it
    /// rarely *varies*, but we classify it so renderers can dedupe
    /// it from the variance set. `AI / Spawning / Identifier / *`.
    /// Examples: `Defenders`, `Target`.
    SpawnRole,
    /// Tag wasn't recognised by any family. Renderers can ignore or
    /// surface in a generic "other" bucket for debugging.
    Other,
}

impl AxisKind {
    /// Classify a tag-tree path into a family. First-match-wins on the
    /// ordered match arms below; the order is chosen so the most
    /// specific path wins (e.g., `AI/Ship/CombatClass` matches before
    /// the broader `AI/Ship`).
    ///
    /// Path is the dotted-name list returned by [`TagTree::path`] —
    /// e.g., `["AI", "Ship", "CombatClass", "VeryEasy"]`.
    pub fn for_path(path: &[&str]) -> Self {
        match path {
            // Specific-ship and ship-class families first (most likely
            // to drive renderer decisions).
            ["Ship", "Model", ..] | ["Ship", "Series", ..] => Self::Hull,
            ["Missions", "VehicleType", "Ship", ..] => Self::ShipClass,
            ["EntityEffectSystem", "Tags", ..] => Self::Effect,
            ["AI", "Ship", "SpawnFlags", ..] | ["AI", "Ship", "ShipState", ..] => Self::SpawnFlags,
            ["AI", "Faction", ..] | ["AI", "Race", ..] | ["Global", "Race", ..] => Self::Faction,
            ["AI", "CargoManifest", _, "PopulationVariation", ..] => Self::CargoSize,
            ["AI", "CargoManifest", _, "Value", ..] => Self::Value,
            // Scaling families.
            ["AI", "SkillDefinitions", ..] | ["AI", "SkillLevel", ..] => Self::Skill,
            ["AI", "Ship", "CombatClass", ..] => Self::CombatClass,
            // Identifiers (often shared, but classified so renderers
            // can dedupe).
            ["AI", "Spawning", "Identifier", ..] => Self::SpawnRole,
            _ => Self::Other,
        }
    }
}

/// A tag present on every option of a [`crate::SlotGroup`], with its
/// pre-computed [`AxisKind`] classification.
///
/// Carrying the [`AxisKind`] on each shared tag lets consumers
/// (renderers, mission-level helpers like [`crate::Mission::combat_class`])
/// scan for tags of a particular family without re-walking the tag
/// tree at query time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedTag {
    pub guid: Guid,
    pub name: String,
    pub kind: AxisKind,
}

/// Per-axis variance across the options inside one [`crate::SlotGroup`].
///
/// Each option contributes the tags that fall in this axis kind but
/// *don't* appear in every other option (i.e., the varying tags).
/// Renderers walk [`Self::per_option`] to see which tag fired on which
/// alternative.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AxisValues {
    /// Outer length == number of options. Inner vec holds the
    /// `(guid, name)` of every tag on that option that classifies to
    /// this axis AND varies across the group.
    ///
    /// Sorted by name within each inner vec for deterministic output.
    pub per_option: Vec<Vec<(Guid, String)>>,
    /// True when no two options share the same set of values on this
    /// axis — i.e., every option has a distinct discriminator.
    /// Renderers use this as "could pick THIS axis as the alternative
    /// label".
    pub all_distinct: bool,
    /// True when at least one option has a value here. Equivalent to
    /// `per_option.iter().any(|v| !v.is_empty())`.
    pub varies: bool,
}

impl AxisValues {
    /// Empty result for a group of `n_options` options where no tag
    /// classifies to this axis (or all options carry identical values).
    fn empty_for(n_options: usize) -> Self {
        Self {
            per_option: vec![Vec::new(); n_options],
            all_distinct: false,
            varies: false,
        }
    }
}

/// Full per-axis breakdown of variance across one [`crate::SlotGroup`].
///
/// Every recognised [`AxisKind`] gets a slot. `other` collects tags
/// that didn't match any family — useful for debugging and for
/// future-proofing against new CIG tags we haven't classified yet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AxisDiff {
    pub hull: AxisValues,
    pub ship_class: AxisValues,
    pub effect: AxisValues,
    pub spawn_flags: AxisValues,
    pub faction: AxisValues,
    pub cargo_size: AxisValues,
    pub value: AxisValues,
    pub skill: AxisValues,
    pub combat_class: AxisValues,
    pub spawn_role: AxisValues,
    pub other: AxisValues,
}

impl AxisDiff {
    /// Compute the diff across `options`, where each option contributes
    /// a set of tag GUIDs (typically from a `SpawnDescription_Ship`'s
    /// positive tag list). Tags that appear in **every** option are
    /// considered "shared" and excluded — only the varying tags land
    /// in the [`AxisValues::per_option`] entries.
    ///
    /// Returns `(diff, shared_tags)`. `shared_tags` is the intersection
    /// across options, sorted by name, useful for the renderer's
    /// "tags shared by every alternative in this group" line.
    ///
    /// Tags whose GUID isn't in the [`TagTree`] are silently dropped
    /// (matching [`crate::TagBag::new`] semantics).
    pub fn compute(per_option_tags: &[Vec<Guid>], tree: &TagTree) -> (Self, Vec<SharedTag>) {
        let n = per_option_tags.len();
        if n == 0 {
            return (Self::default(), Vec::new());
        }

        // Resolve to (guid, name) per option, dropping unresolved.
        // Maintain set semantics per option for the intersection step.
        let per_option_sets: Vec<HashSet<Guid>> = per_option_tags
            .iter()
            .map(|opts| {
                opts.iter()
                    .filter(|g| tree.get(g).is_some())
                    .copied()
                    .collect()
            })
            .collect();

        // Shared = intersection across all options. CigGuid doesn't
        // implement Ord (only Hash + Eq), so use HashSet rather than
        // BTreeSet.
        let mut shared: HashSet<Guid> = per_option_sets.first().cloned().unwrap_or_default();
        for s in per_option_sets.iter().skip(1) {
            shared.retain(|g| s.contains(g));
        }

        // Sort shared by name (then guid bytes for stability) for
        // deterministic output across runs. Each shared tag carries
        // its AxisKind classification so downstream consumers can scan
        // by family without re-walking the tag tree.
        let mut shared_named: Vec<SharedTag> = shared
            .iter()
            .map(|g| {
                let name = tree.get(g).map(|n| n.name.clone()).unwrap_or_default();
                let path = tree.path(g);
                let kind = AxisKind::for_path(&path);
                SharedTag {
                    guid: *g,
                    name,
                    kind,
                }
            })
            .collect();
        shared_named.sort_by(|a, b| {
            a.name
                .cmp(&b.name)
                .then_with(|| a.guid.as_bytes().cmp(b.guid.as_bytes()))
        });

        // Bucket each option's varying tags by AxisKind.
        let mut buckets: BTreeMap<AxisKind, Vec<Vec<(Guid, String)>>> = BTreeMap::new();
        for (opt_idx, set) in per_option_sets.iter().enumerate() {
            for &guid in set {
                if shared.contains(&guid) {
                    continue;
                }
                let path = tree.path(&guid);
                let kind = AxisKind::for_path(&path);
                let entries = buckets.entry(kind).or_insert_with(|| vec![Vec::new(); n]);
                let name = tree.get(&guid).map(|n| n.name.clone()).unwrap_or_default();
                entries[opt_idx].push((guid, name));
            }
        }

        // Sort each option's bucket entries by name for determinism.
        for entries in buckets.values_mut() {
            for option_entries in entries.iter_mut() {
                option_entries.sort_by(|a, b| a.1.cmp(&b.1));
            }
        }

        // Materialise AxisDiff fields.
        let mk = |kind: AxisKind| -> AxisValues {
            let Some(per_option) = buckets.get(&kind).cloned() else {
                return AxisValues::empty_for(n);
            };
            let varies = per_option.iter().any(|v| !v.is_empty());
            // all_distinct: each option's value-set (just the GUID sets)
            // must be unique across options.
            let mut seen: HashSet<Vec<Guid>> = HashSet::new();
            let mut all_distinct = true;
            for v in &per_option {
                let mut guids: Vec<Guid> = v.iter().map(|(g, _)| *g).collect();
                guids.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
                if !seen.insert(guids) {
                    all_distinct = false;
                    break;
                }
            }
            // Edge: with one option `all_distinct` is meaningless — set
            // false so renderers treat it as "not a useful discriminator".
            if n < 2 {
                all_distinct = false;
            }
            AxisValues {
                per_option,
                all_distinct,
                varies,
            }
        };

        let diff = Self {
            hull: mk(AxisKind::Hull),
            ship_class: mk(AxisKind::ShipClass),
            effect: mk(AxisKind::Effect),
            spawn_flags: mk(AxisKind::SpawnFlags),
            faction: mk(AxisKind::Faction),
            cargo_size: mk(AxisKind::CargoSize),
            value: mk(AxisKind::Value),
            skill: mk(AxisKind::Skill),
            combat_class: mk(AxisKind::CombatClass),
            spawn_role: mk(AxisKind::SpawnRole),
            other: mk(AxisKind::Other),
        };

        (diff, shared_named)
    }

    /// Convenience: every axis that varies, paired with its values.
    /// Useful for renderers that want to iterate "what's varying"
    /// without enumerating each field.
    pub fn varying(&self) -> Vec<(AxisKind, &AxisValues)> {
        let mut out: Vec<(AxisKind, &AxisValues)> = Vec::new();
        let probe = [
            (AxisKind::Hull, &self.hull),
            (AxisKind::ShipClass, &self.ship_class),
            (AxisKind::Effect, &self.effect),
            (AxisKind::SpawnFlags, &self.spawn_flags),
            (AxisKind::Faction, &self.faction),
            (AxisKind::CargoSize, &self.cargo_size),
            (AxisKind::Value, &self.value),
            (AxisKind::Skill, &self.skill),
            (AxisKind::CombatClass, &self.combat_class),
            (AxisKind::SpawnRole, &self.spawn_role),
            (AxisKind::Other, &self.other),
        ];
        for (k, v) in probe {
            if v.varies {
                out.push((k, v));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sc_extract::TagNode;

    fn mk_guid(n: u8) -> Guid {
        // Deterministic test guids — bytes[0] is unique per test tag.
        let mut bytes = [0u8; 16];
        bytes[0] = n;
        Guid::from_bytes(bytes)
    }

    /// Build a small TagTree from `(name, parent_idx)` pairs. Index 0
    /// is reserved for the root.
    fn build_tree(nodes: &[(&str, Option<usize>)]) -> TagTree {
        let mut tree = TagTree::new();
        for (i, (name, parent)) in nodes.iter().enumerate() {
            let guid = mk_guid(i as u8);
            tree.insert(TagNode {
                guid,
                name: (*name).to_string(),
                parent: parent.map(|p| mk_guid(p as u8)),
                children: Vec::new(),
                legacy_guid: None,
            });
        }
        tree
    }

    #[test]
    fn classifies_combat_class() {
        let path = vec!["AI", "Ship", "CombatClass", "VeryEasy"];
        assert_eq!(AxisKind::for_path(&path), AxisKind::CombatClass);
    }

    #[test]
    fn classifies_skill() {
        assert_eq!(
            AxisKind::for_path(&["AI", "SkillDefinitions", "Description", "HumanPilot10"]),
            AxisKind::Skill
        );
        assert_eq!(
            AxisKind::for_path(&["AI", "SkillLevel", "Easy"]),
            AxisKind::Skill
        );
    }

    #[test]
    fn classifies_ship_class_and_hull() {
        assert_eq!(
            AxisKind::for_path(&["Missions", "VehicleType", "Ship", "CombatShip"]),
            AxisKind::ShipClass
        );
        assert_eq!(
            AxisKind::for_path(&["Missions", "VehicleType", "Ship", "Distortion"]),
            AxisKind::ShipClass
        );
        assert_eq!(
            AxisKind::for_path(&["Ship", "Model", "Scythe"]),
            AxisKind::Hull
        );
        assert_eq!(
            AxisKind::for_path(&["Ship", "Series", "Cutlass", "Cutlass_Black"]),
            AxisKind::Hull
        );
    }

    #[test]
    fn classifies_effect_spawn_faction() {
        assert_eq!(
            AxisKind::for_path(&["EntityEffectSystem", "Tags", "Distortion"]),
            AxisKind::Effect
        );
        assert_eq!(
            AxisKind::for_path(&["AI", "Ship", "SpawnFlags", "ArriveViaQT"]),
            AxisKind::SpawnFlags
        );
        assert_eq!(
            AxisKind::for_path(&["AI", "Faction", "Vanduul"]),
            AxisKind::Faction
        );
    }

    #[test]
    fn classifies_cargo_and_value() {
        assert_eq!(
            AxisKind::for_path(&[
                "AI",
                "CargoManifest",
                "General",
                "PopulationVariation",
                "Full Cargo"
            ]),
            AxisKind::CargoSize
        );
        assert_eq!(
            AxisKind::for_path(&["AI", "CargoManifest", "General", "Value", "LowValue"]),
            AxisKind::Value
        );
    }

    #[test]
    fn unknown_path_falls_to_other() {
        assert_eq!(
            AxisKind::for_path(&["SomethingUnknown", "Whatever"]),
            AxisKind::Other
        );
        assert_eq!(AxisKind::for_path(&[]), AxisKind::Other);
    }

    #[test]
    fn diff_collapses_shared_tags_and_buckets_varying() {
        // Build a tag tree mirroring Settle a Score's Scouts pattern:
        //   AI / Ship / CombatClass / VeryEasy           (shared)
        //   AI / Faction / Criminal                       (shared)
        //   AI / SkillDefinitions / Description / HumanPilot10 / 20 / 30  (varies)
        let tree = build_tree(&[
            ("AI", None),                  // 0
            ("Ship", Some(0)),             // 1
            ("CombatClass", Some(1)),      // 2
            ("VeryEasy", Some(2)),         // 3   shared
            ("Faction", Some(0)),          // 4
            ("Criminal", Some(4)),         // 5   shared
            ("SkillDefinitions", Some(0)), // 6
            ("Description", Some(6)),      // 7
            ("HumanPilot10", Some(7)),     // 8   varies
            ("HumanPilot20", Some(7)),     // 9   varies
            ("HumanPilot30", Some(7)),     // 10  varies
        ]);

        let very_easy = mk_guid(3);
        let criminal = mk_guid(5);
        let pilot10 = mk_guid(8);
        let pilot20 = mk_guid(9);
        let pilot30 = mk_guid(10);

        // Three options matching the Settle a Score Scouts shape.
        let per_option = vec![
            vec![very_easy, criminal, pilot30],
            vec![very_easy, criminal, pilot20],
            vec![very_easy, criminal, pilot10],
        ];

        let (diff, shared) = AxisDiff::compute(&per_option, &tree);

        // Shared: VeryEasy + Criminal (sorted alphabetically).
        let shared_names: Vec<&str> = shared.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(shared_names, vec!["Criminal", "VeryEasy"]);
        // VeryEasy carries the CombatClass axis classification.
        let combat_class = shared.iter().find(|t| t.name == "VeryEasy").unwrap();
        assert_eq!(combat_class.kind, AxisKind::CombatClass);

        // Skill axis: each option has one distinct HumanPilot tag.
        assert!(diff.skill.varies);
        assert!(diff.skill.all_distinct);
        assert_eq!(diff.skill.per_option.len(), 3);
        assert_eq!(diff.skill.per_option[0].len(), 1);
        assert_eq!(diff.skill.per_option[0][0].1, "HumanPilot30");
        assert_eq!(diff.skill.per_option[2][0].1, "HumanPilot10");

        // CombatClass / Faction: shared, so no variance.
        assert!(!diff.combat_class.varies);
        assert!(!diff.faction.varies);

        // varying() helper returns just the skill axis.
        let varying: Vec<_> = diff.varying().into_iter().map(|(k, _)| k).collect();
        assert_eq!(varying, vec![AxisKind::Skill]);
    }

    #[test]
    fn diff_handles_distortion_overlay() {
        // Two options: one with Distortion (Effect + ShipClass), one without.
        // Mirrors the InterSec Nyx Hard pattern.
        let tree = build_tree(&[
            ("Missions", None),       // 0
            ("VehicleType", Some(0)), // 1
            ("Ship", Some(1)),        // 2
            ("CombatShip", Some(2)),  // 3  shared
            ("Distortion", Some(2)),  // 4  varies (ShipClass family)
            ("AI", None),             // 5
            ("Ship", Some(5)),        // 6
            ("CombatClass", Some(6)), // 7
            ("Hard", Some(7)),        // 8  shared
        ]);
        let combat_ship = mk_guid(3);
        let distortion_ship_class = mk_guid(4);
        let hard = mk_guid(8);

        let per_option = vec![
            vec![combat_ship, distortion_ship_class, hard], // with Distortion
            vec![combat_ship, hard],                        // without
        ];
        let (diff, shared) = AxisDiff::compute(&per_option, &tree);

        // Shared: CombatShip + Hard.
        let shared_names: Vec<&str> = shared.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(shared_names, vec!["CombatShip", "Hard"]);

        // ShipClass varies — opt[0] has Distortion, opt[1] has nothing.
        assert!(diff.ship_class.varies);
        assert_eq!(diff.ship_class.per_option[0].len(), 1);
        assert_eq!(diff.ship_class.per_option[0][0].1, "Distortion");
        assert_eq!(diff.ship_class.per_option[1].len(), 0);
        // Distinct-ness: opt[0] has [Distortion], opt[1] has []. Sets differ.
        assert!(diff.ship_class.all_distinct);
    }

    #[test]
    fn diff_one_option_yields_no_variance() {
        let tree = build_tree(&[("AI", None), ("Faction", Some(0)), ("Criminal", Some(1))]);
        let crim = mk_guid(2);
        let (diff, shared) = AxisDiff::compute(&[vec![crim]], &tree);
        assert_eq!(shared.len(), 1);
        assert!(!diff.faction.varies);
        assert!(!diff.faction.all_distinct);
    }

    #[test]
    fn diff_empty_input() {
        let tree = build_tree(&[("X", None)]);
        let (diff, shared) = AxisDiff::compute(&[], &tree);
        assert!(shared.is_empty());
        assert!(diff.varying().is_empty());
    }
}
