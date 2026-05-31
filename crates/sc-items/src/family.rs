//! Item families — group entity items into "models" so paint / skin /
//! special-edition variants collapse to one entry.
//!
//! CIG ships every paint / skin / "Modified" edition as its own SC
//! item entity with its own crafting recipe; there's no structural
//! `parent_item` reference in the DCB. The family identity is computed
//! from a priority chain of signals reverse-engineered against SC 4.8:
//!
//! 1. **ECD model tag.** A leaf in the tag tree like
//!    `Weapon / FPS / Pistol / Coda` or
//!    `Armor / FPS / Set / ClarkeDefense / FBL-8a`. Each whitelisted
//!    prefix declares its **model depth** — how many segments past the
//!    prefix together form the model identity. Anything deeper
//!    (sub-variant markers like `… / Atzkav / AtzkavDE` for a special
//!    edition) gets truncated, so all variants of a model share one
//!    family id.
//! 2. **`SItemDefinition.tags` first specific token.** For items whose
//!    ECD only carries a generic category marker (e.g. armor sets with
//!    just the 3-segment `Armor / FPS / Set` tag — no brand / model
//!    leaf) the model identifier lives in the SItemDefinition.tags
//!    whitespace-separated string. Find the first token that looks
//!    specific: it must contain an underscore (filters out plain words
//!    like `"stocked"` / `"pistol"`) and not be a parametric variant
//!    marker (`Set_*`, `Color_*`, `Texture_*`).
//! 3. **None.** No recognised family signal. The caller typically
//!    treats these as singletons (key on entity GUID, render as one
//!    row in catalog UIs).
//!
//! Build once via [`ItemFamilies::build`]; the index is small (~one
//! map entry per item with a family) and serde-clean for caching in
//! processed snapshots.
//!
//! # What about `sc-items::variants`?
//!
//! [`crate::variants`] holds *naming* heuristics (strip `_pu_ai*`
//! suffixes, detect `_<color><NN>` patterns). Those work on entity
//! record names. [`ItemFamilies`] uses *structural* data — tag-tree
//! paths and the item-definition tags string — which is more reliable
//! for grouping than name heuristics. The two are complementary; a
//! consumer wanting "all variants of model X" should start here.

use std::collections::HashMap;

use sc_extract::generated::{
    DataForgeComponentParamsPtr, EntityClassDefinition, RecordLookup, SItemDefinition,
};
use sc_extract::{DataPools, Guid, RecordStore};
use sc_tags::Tags;
use serde::{Deserialize, Serialize};

use crate::Items;

/// Opaque family identifier. All variants of one model share the same
/// string; the exact shape (truncated tag path or `item-tag:<token>`)
/// is an internal implementation detail — treat it as a grouping key,
/// not a display string.
pub type FamilyId = String;

/// Two-way index from item GUID ↔ family id. Items without a
/// recognised family signal are absent from both maps —
/// [`Self::family_id_of`] returns `None`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemFamilies {
    by_member: HashMap<Guid, FamilyId>,
    by_family: HashMap<FamilyId, Vec<Guid>>,
}

impl ItemFamilies {
    pub fn new() -> Self {
        Self::default()
    }

    /// Compute the family index by iterating every entry in [`Items`]
    /// and running the priority-chain signal (see module docs) for each.
    ///
    /// `store` is required for the `SItemDefinition` lookup (which goes
    /// through the entity's `SAttachableComponentParams` component);
    /// the cooked [`crate::Item`] surface doesn't expose `tags`. `tags`
    /// resolves ECD tag GUIDs to readable path strings for the
    /// whitelist match.
    pub fn build(items: &Items, tags: &Tags, store: &RecordStore) -> Self {
        let mut out = Self::new();
        for (guid, _) in items.iter() {
            let Some(fid) = compute_family_id(*guid, store, tags) else {
                continue;
            };
            out.by_family.entry(fid.clone()).or_default().push(*guid);
            out.by_member.insert(*guid, fid);
        }
        out
    }

    /// Family id for `guid`, or `None` if the item has no recognised
    /// family signal.
    pub fn family_id_of(&self, guid: &Guid) -> Option<&str> {
        self.by_member.get(guid).map(String::as_str)
    }

    /// Every member GUID of one family, in arbitrary order. Empty slice
    /// if the family id is unknown.
    pub fn members_of(&self, family_id: &str) -> &[Guid] {
        self.by_family
            .get(family_id)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    /// Iterate `(family_id, members)` pairs. Order unspecified.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &[Guid])> + '_ {
        self.by_family
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_slice()))
    }

    /// Number of distinct families (not items — same model with N paints
    /// counts as 1).
    pub fn len(&self) -> usize {
        self.by_family.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_family.is_empty()
    }
}

// ── Family computation ─────────────────────────────────────────────────

/// One whitelisted model-tag prefix and the depth past it that forms
/// the model identity.
struct ModelPrefix {
    prefix: &'static str,
    /// Number of segments AFTER the prefix that together form the
    /// model id. Sub-variant markers deeper than that get truncated.
    /// Pistol-style 4-segment paths use depth 1 (just the model name);
    /// armor's 5-segment `Set / Brand / Model` uses depth 2 (brand +
    /// model leaf together — there's no model-vs-brand ambiguity within
    /// the FBL-8a-style identifier itself).
    model_depth: usize,
}

/// Whitelisted model-tag families. Verified against SC 4.8 live DCB
/// via cross-section probes (FPS Pistols, FPS Snipers, FPS Stocked,
/// FPS Armor sets). Each prefix REQUIRES content after the trailing
/// slash so generic category markers like `Weapon / FPS / Stocked`
/// (3 segs, no model leaf) don't match.
const MODEL_PREFIXES: &[ModelPrefix] = &[
    // FPS handheld weapons — `Weapon / FPS / <Class> / <Model>`.
    ModelPrefix { prefix: "Weapon / FPS / Pistol / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / SMG / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Shotgun / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Sniper / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / LMG / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / HMG / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Cannon / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Launcher / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Mining / ", model_depth: 1 },
    // FPS shoulder/stocked weapons — `Weapon / FPS / Stocked / <Class>
    // / <Model>`. Snipers + rifles + shotguns follow this nested path
    // instead of the flat one above.
    ModelPrefix { prefix: "Weapon / FPS / Stocked / Rifle / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / SniperRifle / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / Shotgun / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / SMG / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / LMG / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / HMG / ", model_depth: 1 },
    // FPS Armor — `Armor / FPS / Set / <Brand> / <Model>`. Brand +
    // model leaf together form the identity. Some armor sets ship
    // with ONLY the 3-segment `Armor / FPS / Set` marker (no brand /
    // model leaf) — those fall through to the item.tags signal.
    ModelPrefix { prefix: "Armor / FPS / Set / ", model_depth: 2 },
];

fn compute_family_id(entity_guid: Guid, store: &RecordStore, tags: &Tags) -> Option<FamilyId> {
    let handle = EntityClassDefinition::lookup(&store.records, &entity_guid)?;
    let ecd = handle.get(&store.pools)?;

    // 1. ECD model tag — first matching prefix, TRUNCATED to model depth.
    for tag_guid in &ecd.tags {
        let segs = tags.path(tag_guid);
        if segs.is_empty() {
            continue;
        }
        let path = segs.join(" / ");
        for entry in MODEL_PREFIXES {
            if !path.starts_with(entry.prefix) {
                continue;
            }
            // Prefix segment count = chunks before the trailing slash
            // (`Weapon / FPS / Pistol / ` → 3: Weapon, FPS, Pistol).
            let prefix_seg_count = entry
                .prefix
                .trim_end_matches(" / ")
                .split(" / ")
                .count();
            let total = prefix_seg_count + entry.model_depth;
            if segs.len() < total {
                continue;
            }
            return Some(segs[..total].join(" / "));
        }
    }

    // 2. SItemDefinition.tags — first whitespace token that looks
    //    model-specific (underscored, not a parametric marker).
    let item_def = find_item_def(ecd, &store.pools)?;
    let token = item_def.tags.split_whitespace().find(|t| {
        t.contains('_')
            && !t.starts_with("Set_")
            && !t.starts_with("Color_")
            && !t.starts_with("Texture_")
    })?;
    Some(format!("item-tag:{token}"))
}

fn find_item_def<'a>(
    ecd: &EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SItemDefinition> {
    let attachable = ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SAttachableComponentParams(h) => h.get(pools),
        _ => None,
    })?;
    attachable.attach_def.and_then(|h| h.get(pools))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_index_is_empty() {
        let f = ItemFamilies::new();
        assert!(f.is_empty());
        assert_eq!(f.len(), 0);
        assert!(f.family_id_of(&Guid::default()).is_none());
        assert!(f.members_of("nothing").is_empty());
        assert_eq!(f.iter().count(), 0);
    }

    #[test]
    fn serde_round_trip() {
        let mut f = ItemFamilies::new();
        let a = Guid::from_bytes([1; 16]);
        let b = Guid::from_bytes([2; 16]);
        let c = Guid::from_bytes([3; 16]);
        // Two variants of the same family + one singleton family.
        f.by_member.insert(a, "fam-a".to_string());
        f.by_member.insert(b, "fam-a".to_string());
        f.by_member.insert(c, "fam-b".to_string());
        f.by_family.insert("fam-a".to_string(), vec![a, b]);
        f.by_family.insert("fam-b".to_string(), vec![c]);

        let json = serde_json::to_string(&f).unwrap();
        let back: ItemFamilies = serde_json::from_str(&json).unwrap();
        assert_eq!(back.len(), 2);
        assert_eq!(back.family_id_of(&a), Some("fam-a"));
        assert_eq!(back.family_id_of(&c), Some("fam-b"));
        assert_eq!(back.members_of("fam-a").len(), 2);
        assert_eq!(back.members_of("fam-b"), &[c]);
    }
}
