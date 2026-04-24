//! Tag classification for spawn queries.
//!
//! Contract spawn queries carry a soup of tags — ship-selective
//! identifiers (`Cutter`, `Gladius`), faction markers (`Criminal`,
//! `Ninetails`), AI skill levels (`HumanPilot70`), cargo descriptors
//! (`Full Cargo`, `Scraps Cargo`), difficulty labels, runtime
//! directives (`ArriveViaQT`), and role markers (`Target`, `Bounty`).
//!
//! [`SpawnContext`] groups these into structured fields by walking the
//! `sc_extract::TagTree` hierarchy — classification is subtree-based,
//! not name-substring-matched (workspace design principle §5).
//!
//! Ship-selective tags are **not** surfaced here — those live in
//! [`crate::ShipRegistry::resolve_spawn`]. `SpawnContext` is the
//! everything-else, so a displayed spawn slot combines
//! `ShipRegistry::resolve_spawn` for candidate ships and
//! `SpawnContext::classify` for the surrounding context.
//!
//! The specific subtree names (`AI`, `AI > Faction`,
//! `AI > CargoManifest`, `AI > SkillDefinitions`, …) come from the
//! live DCB tag tree and are stable across SC patches; if a patch
//! reorganises the tag hierarchy, these match names need to move with
//! it. The classifier is defensive — tags that don't fit any known
//! subtree fall into `other`, so nothing is silently dropped.

use std::collections::HashSet;

use sc_extract::{Guid, TagTree};

/// Structured breakdown of the non-ship context tags on a spawn query.
/// Ship-selective tags are handled separately by [`crate::ShipRegistry`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpawnContext {
    /// Pilot skill level parsed from `HumanPilotN` tags (10, 20, …, 100).
    /// If multiple `HumanPilotN` tags appear, holds the maximum.
    pub ai_skill: Option<u32>,
    /// True if the `AcePilot` tag is present (top-tier AI).
    pub ace_pilot: bool,
    /// Factions — descendants of `AI ▸ Faction`. Typically one (`Criminal`,
    /// `UEE`, `Ninetails`) but the DCB lets a query name multiple.
    pub factions: Vec<String>,
    /// Cargo descriptors — descendants of `AI ▸ CargoManifest`
    /// (`Full Cargo`, `Half Cargo`, `Scraps Cargo`, …).
    pub cargo: Vec<String>,
    /// Generic AI traits / role markers — descendants of `AI` that
    /// didn't fit Faction / Cargo / Skill (e.g. `Target`, `Defenders`,
    /// `Bounty`, `LowValue`, `HighValue`, `Mixed`).
    pub ai_traits: Vec<String>,
    /// Tags from the `Missions` root — difficulty labels, mission-type
    /// markers, setup flags. Typical values: `VeryHard`, `Super`,
    /// `Pirates`, `Investigator`.
    pub mission_tags: Vec<String>,
    /// Runtime spawn directives — tag names beginning with `Arrive`
    /// or `Ignore` (`ArriveViaQT`, `IgnoreCrimes`, `IgnoreHostility`).
    /// These are engine-resolved at spawn time.
    pub directives: Vec<String>,
    /// Anything that didn't fit the other buckets. Keeps the raw tag
    /// name so nothing is silently dropped.
    pub other: Vec<String>,
}

impl SpawnContext {
    /// Classify a set of positive tags from a spawn query.
    ///
    /// Ship-selective tags (under the `Ship` root) are skipped — they're
    /// consumed by [`crate::ShipRegistry::resolve_spawn`]. Everything
    /// else is sorted into the `SpawnContext` fields.
    ///
    /// Each field's order reflects iteration of the input set (which is
    /// a [`HashSet`] — so the order is not stable across runs); callers
    /// that need deterministic output should sort the vecs themselves.
    pub fn classify(tree: &TagTree, tags: &HashSet<Guid>) -> Self {
        let mut ctx = SpawnContext::default();

        for guid in tags {
            let Some(node) = tree.get(guid) else {
                continue;
            };
            let name = &node.name;

            // 1. Skill parse (works regardless of subtree placement —
            // HumanPilotN lives under AI but pattern-matching by name
            // is safer than depending on the exact subtree path).
            if let Some(level) = parse_ai_skill(name) {
                ctx.ai_skill = Some(ctx.ai_skill.map_or(level, |cur| cur.max(level)));
                continue;
            }
            if name == "AcePilot" {
                ctx.ace_pilot = true;
                continue;
            }

            // 2. Runtime directives — name-prefix check. These have
            // zero carriers so they're meaningful in queries but never
            // on ships; capturing them lets consumers see the engine's
            // intent (arrive via QT, ignore crimes, …).
            if name.starts_with("Arrive") || name.starts_with("Ignore") {
                ctx.directives.push(name.clone());
                continue;
            }

            // 3. Subtree classification via the tag path.
            let path = tree.path(guid);
            match root_and_category(&path) {
                ("Ship", _) => {} // handled by ShipRegistry
                ("AI", Some("Faction")) => ctx.factions.push(name.clone()),
                ("AI", Some("CargoManifest")) => ctx.cargo.push(name.clone()),
                ("AI", _) => ctx.ai_traits.push(name.clone()),
                ("Missions", _) => ctx.mission_tags.push(name.clone()),
                _ => ctx.other.push(name.clone()),
            }
        }

        // Sort each vec so the output is deterministic across runs.
        ctx.factions.sort();
        ctx.cargo.sort();
        ctx.ai_traits.sort();
        ctx.mission_tags.sort();
        ctx.directives.sort();
        ctx.other.sort();
        ctx
    }

    /// True when every field is empty (the context added nothing to a
    /// ship-only query). Useful for consumers that want to skip display
    /// when there's no information to show.
    pub fn is_empty(&self) -> bool {
        self.ai_skill.is_none()
            && !self.ace_pilot
            && self.factions.is_empty()
            && self.cargo.is_empty()
            && self.ai_traits.is_empty()
            && self.mission_tags.is_empty()
            && self.directives.is_empty()
            && self.other.is_empty()
    }
}

/// Parse `HumanPilot10`, `HumanPilot20`, …, `HumanPilot100` → the level
/// as `u32`. Returns `None` for anything that doesn't match the exact
/// `HumanPilotN` pattern.
pub fn parse_ai_skill(name: &str) -> Option<u32> {
    name.strip_prefix("HumanPilot")
        .and_then(|rest| rest.parse::<u32>().ok())
}

/// Inspect a tag-tree path and return its root name plus the first
/// category below the root. Example:
///
/// `["AI", "Faction", "Criminal"]` → `("AI", Some("Faction"))`
/// `["AI", "Criminal"]`            → `("AI", None)` (shouldn't normally occur)
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
        assert_eq!(root_and_category(&["AI", "Faction"]), ("AI", Some("Faction")));
        assert_eq!(
            root_and_category(&["AI", "Faction", "Criminal"]),
            ("AI", Some("Faction"))
        );
    }

    #[test]
    fn empty_context_when_no_tags() {
        let ctx = SpawnContext::default();
        assert!(ctx.is_empty());
        assert_eq!(ctx.ai_skill, None);
    }

    #[test]
    fn is_empty_flips_on_any_field() {
        let mut ctx = SpawnContext::default();
        assert!(ctx.is_empty());
        ctx.ai_skill = Some(50);
        assert!(!ctx.is_empty());

        let mut ctx = SpawnContext::default();
        ctx.directives.push("ArriveViaQT".into());
        assert!(!ctx.is_empty());
    }
}
