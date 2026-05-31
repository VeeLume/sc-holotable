//! Item families — every item entity has a **base** variant + zero or
//! more **variant** entities (paints, skins, special editions). The
//! family struct exposes that relationship explicitly so consumers can
//! render the base as a header and treat variants uniformly.
//!
//! # Mental model
//!
//! Every "skin" in SC has a base item it derives from — `Ripper SMG`
//! is the base of `Ripper "Sunblock" SMG`; `Overlord Core` is the
//! base of `Overlord Core Supernova`. The DCB has no `parent_item`
//! reference, so the family is recovered from a priority chain of
//! structural signals:
//!
//! 1. **ECD model tag.** A leaf in the tag tree like
//!    `Weapon / FPS / Pistol / Coda` or
//!    `Armor / FPS / Set / ClarkeDefense / FBL-8a`. Whitelisted
//!    prefixes declare a *model depth* — how many segments past the
//!    prefix together form the model identity; anything deeper
//!    (sub-variant markers like `… / Atzkav / AtzkavDE`) is truncated
//!    so all variants share one family id.
//! 2. **Entity record name stem + (item_type, item_sub_type).** For
//!    items whose ECD has no usable model tag (e.g. armor sets that
//!    ship with only the generic `Armor / FPS / Set` marker), the
//!    entity record name carries the brand in its leading segment
//!    (`kap_combat_heavy_core_02_01_01` for KAP's "Monde" core armor)
//!    and the variant suffix in its trailing segments. Strip trailing
//!    variant suffixes (numeric, alpha+digits, semantic words like
//!    `_mag`/`_spc`/`_scitem`) iteratively — but never below 3
//!    segments, so a stripped stem still meaningfully identifies a
//!    model. Pair the stem with item type / sub-type to prevent a
//!    helmet from bundling with a chest piece even if both share the
//!    same entity stem.
//! 3. **Solo (entity GUID).** Items that resolve to neither — keeps
//!    them as their own one-member family, so callers can always
//!    expect a family lookup to succeed for any item.
//!
//! # Why not `SItemDefinition.tags`?
//!
//! Earlier iterations used the first underscored token of
//! `SItemDefinition.tags` as a fallback signal. It catastrophically
//! over-grouped — most armor's `tags` string is `"Set_<n> Color_<n>
//! SM_RestrictedArm"`, with the only underscored non-parametric token
//! being `SM_RestrictedArm` (a generic suit-mannequin marker shared by
//! every armor item). That signal is removed; the entity-name-stem
//! approach is both more reliable and naturally gated by manufacturer
//! prefix.
//!
//! # Base detection
//!
//! Within a family, the base is the entity record with the *shortest*
//! name length (variants typically carry additional suffixes), with
//! alphabetical tiebreaker. The base is always the first entry in
//! `Family::members`.
//!
//! # Coverage
//!
//! Built over [`Items`] — covers every entity exposing an `AttachDef`,
//! independent of whether the entity has a blueprint, mission reward,
//! or any other downstream reference. Future consumers wanting "all
//! variants of Ripper SMG" get them whether or not Ripper itself is
//! craftable.

use std::collections::HashMap;

use sc_extract::generated::{EntityClassDefinition, RecordLookup};
use sc_extract::{Guid, RecordPaths, RecordStore};
use sc_tags::Tags;
use serde::{Deserialize, Serialize};

use crate::Items;

/// Opaque family identifier. Shape varies by signal — a tag path
/// (`Weapon / FPS / Pistol / Coda`), a stem-scoped key
/// (`stem:kap_combat_heavy_core:Char_Armor_Torso:UNDEFINED`), or a solo
/// fallback (`solo:<guid>`). Treat as an opaque grouping key.
pub type FamilyId = String;

/// One family: a base item plus its variants. `members` always starts
/// with `base` and is ordered deterministically (shortest entity name
/// first, alphabetical tiebreaker).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Family {
    pub id: FamilyId,
    /// The canonical "unstyled" item. Same GUID also appears in
    /// `members[0]`.
    pub base: Guid,
    /// All family members (base + variants), base first.
    pub members: Vec<Guid>,
}

impl Family {
    /// Iterate the non-base variant GUIDs.
    pub fn variants(&self) -> impl Iterator<Item = Guid> + '_ {
        self.members.iter().skip(1).copied()
    }

    /// Total member count (base + variants). Always ≥ 1, so there's
    /// no companion `is_empty` — see [`Self::is_solo`] for the
    /// useful "is this just one item?" check.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// `true` if this is a single-member family (just the base, no
    /// known variants). Useful for skipping the "VARIANTS" affordance
    /// in catalog UIs.
    pub fn is_solo(&self) -> bool {
        self.members.len() == 1
    }
}

/// Index of every item's family, with both directions of lookup.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemFamilies {
    by_member: HashMap<Guid, FamilyId>,
    by_family: HashMap<FamilyId, Family>,
}

impl ItemFamilies {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the family index from a parsed datacore.
    ///
    /// Walks every entry in `items`, computes its family identity via
    /// the priority chain (see module docs), and groups them. `tags`
    /// resolves ECD tag GUIDs to readable path strings; `paths`
    /// supplies entity record names for the stem signal; `store` is
    /// needed to look up the typed `EntityClassDefinition` for ECD tag
    /// access.
    pub fn build(items: &Items, tags: &Tags, store: &RecordStore, paths: &RecordPaths) -> Self {
        // Pass 1: compute family id for every item.
        let mut family_id_of: HashMap<Guid, FamilyId> = HashMap::with_capacity(items.len());
        let mut members_of: HashMap<FamilyId, Vec<Guid>> = HashMap::new();
        for (guid, item) in items.iter() {
            let fid = compute_family_id(*guid, item, store, tags, paths);
            members_of.entry(fid.clone()).or_default().push(*guid);
            family_id_of.insert(*guid, fid);
        }

        // Pass 2: pick base + sort members for each family.
        let mut by_family: HashMap<FamilyId, Family> = HashMap::with_capacity(members_of.len());
        for (id, mut members) in members_of {
            members.sort_by_key(|guid| name_sort_key(*guid, paths));
            let base = members[0];
            by_family.insert(id.clone(), Family { id, base, members });
        }

        Self { by_member: family_id_of, by_family }
    }

    pub fn family_of(&self, guid: &Guid) -> Option<&Family> {
        let fid = self.by_member.get(guid)?;
        self.by_family.get(fid)
    }

    pub fn family_id_of(&self, guid: &Guid) -> Option<&str> {
        self.by_member.get(guid).map(String::as_str)
    }

    pub fn base_of(&self, guid: &Guid) -> Option<Guid> {
        self.family_of(guid).map(|f| f.base)
    }

    /// Iterate every family. Order unspecified.
    pub fn iter(&self) -> impl Iterator<Item = &Family> + '_ {
        self.by_family.values()
    }

    /// Number of distinct families (a base item with 5 paints counts
    /// as 1).
    pub fn len(&self) -> usize {
        self.by_family.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_family.is_empty()
    }
}

// ── Family-id computation ──────────────────────────────────────────────

fn compute_family_id(
    guid: Guid,
    item: &crate::Item,
    store: &RecordStore,
    tags: &Tags,
    paths: &RecordPaths,
) -> FamilyId {
    // 1. ECD model tag — first matching prefix, truncated to model depth.
    if let Some(handle) = EntityClassDefinition::lookup(&store.records, &guid)
        && let Some(ecd) = handle.get(&store.pools)
    {
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
                let prefix_seg_count = entry
                    .prefix
                    .trim_end_matches(" / ")
                    .split(" / ")
                    .count();
                let total = prefix_seg_count + entry.model_depth;
                if segs.len() < total {
                    continue;
                }
                return segs[..total].join(" / ");
            }
        }
    }

    // 2. Entity record name stem + (item_type, item_sub_type) gating.
    if let Some(rp) = paths.get(&guid) {
        let name = rp
            .name
            .strip_prefix("EntityClassDefinition.")
            .unwrap_or(&rp.name);
        let stem = entity_name_stem(name);
        return format!(
            "stem:{stem}:{}:{}",
            item.item_type.as_dcb_str(),
            item.item_sub_type.as_dcb_str()
        );
    }

    // 3. Solo fallback — every item gets *some* family entry.
    format!("solo:{guid}")
}

/// Strip trailing variant suffixes from an entity record name. Stops
/// at the first non-variant segment, or when the stem would shrink
/// below 3 underscore-separated segments (a stem that short rarely
/// encodes enough identity to distinguish models).
fn entity_name_stem(name: &str) -> &str {
    let mut s = name;
    while let Some(stripped) = strip_one_variant_suffix(s) {
        if segment_count(stripped) < 3 {
            break;
        }
        s = stripped;
    }
    s
}

fn segment_count(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        s.split('_').count()
    }
}

fn strip_one_variant_suffix(s: &str) -> Option<&str> {
    let last_underscore = s.rfind('_')?;
    let suffix = &s[last_underscore + 1..];
    is_variant_suffix(suffix).then(|| &s[..last_underscore])
}

/// Decide if a segment (text after the last `_`) is a variant marker
/// vs part of the model identity.
fn is_variant_suffix(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    // Pure digits — covers `_01`, `_02`, `_150`. The most common
    // armor-variant index.
    if s.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }

    // Alphabetic prefix + trailing digit(s) — covers `_chromic01`,
    // `_white01`, `_imp01`, `_S2`. Common for paint variants and size
    // suffixes.
    let trailing_digits = s.chars().rev().take_while(|c| c.is_ascii_digit()).count();
    if trailing_digits >= 1 {
        let alpha_part = &s[..s.len() - trailing_digits];
        if !alpha_part.is_empty() && alpha_part.chars().all(|c| c.is_ascii_alphabetic()) {
            return true;
        }
    }

    // Known semantic suffixes — pattern-stable across the DCB.
    const SEMANTIC: &[&str] = &["mag", "spc", "ammo", "ammobox", "scitem"];
    SEMANTIC.iter().any(|w| s.eq_ignore_ascii_case(w))
}

// ── Base selection ─────────────────────────────────────────────────────

/// Sort key for picking the "base" entity within a family — shortest
/// entity record name first, alphabetical tiebreaker.
fn name_sort_key(guid: Guid, paths: &RecordPaths) -> (usize, String) {
    let name = paths
        .get(&guid)
        .map(|rp| rp.name.as_str())
        .unwrap_or("");
    (name.len(), name.to_string())
}

// ── Model-tag prefixes ─────────────────────────────────────────────────

/// One whitelisted model-tag prefix and the depth past it that forms
/// the model identity. The path is truncated to `prefix_segs +
/// model_depth` segments, so sub-variant markers (e.g. `… / Atzkav /
/// AtzkavDE` for a special-edition Atzkav sniper) collapse to the
/// shared model path.
#[derive(Copy, Clone)]
struct ModelPrefix {
    prefix: &'static str,
    model_depth: usize,
}

/// Whitelisted families. Verified against SC 4.8 live DCB.
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
    // FPS shoulder/stocked weapons.
    ModelPrefix { prefix: "Weapon / FPS / Stocked / Rifle / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / SniperRifle / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / Shotgun / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / SMG / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / LMG / ", model_depth: 1 },
    ModelPrefix { prefix: "Weapon / FPS / Stocked / HMG / ", model_depth: 1 },
    // FPS Armor — `Armor / FPS / Set / <Brand> / <Model>` when the
    // tag tree exposes a specific brand/model leaf (Geist Arms has
    // this; Corbel / Monde do not — those fall to the stem signal).
    ModelPrefix { prefix: "Armor / FPS / Set / ", model_depth: 2 },
];

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_variant_suffix ──

    #[test]
    fn variant_suffix_pure_digits() {
        assert!(is_variant_suffix("01"));
        assert!(is_variant_suffix("17"));
        assert!(is_variant_suffix("150"));
        assert!(is_variant_suffix("0"));
    }

    #[test]
    fn variant_suffix_alpha_digits() {
        assert!(is_variant_suffix("chromic01"));
        assert!(is_variant_suffix("white01"));
        assert!(is_variant_suffix("imp01"));
        assert!(is_variant_suffix("S2"));
        assert!(is_variant_suffix("S00"));
    }

    #[test]
    fn variant_suffix_semantic_words() {
        assert!(is_variant_suffix("mag"));
        assert!(is_variant_suffix("MAG"));
        assert!(is_variant_suffix("spc"));
        assert!(is_variant_suffix("SCItem"));
    }

    #[test]
    fn variant_suffix_rejects_model_names() {
        assert!(!is_variant_suffix("arms"));
        assert!(!is_variant_suffix("helmet"));
        assert!(!is_variant_suffix("core"));
        assert!(!is_variant_suffix("pistol"));
        assert!(!is_variant_suffix(""));
        // Letters with no trailing digit.
        assert!(!is_variant_suffix("abc"));
    }

    // ── entity_name_stem ──

    #[test]
    fn stem_strips_paint_suffix() {
        assert_eq!(
            entity_name_stem("gmni_pistol_ballistic_01_white01"),
            "gmni_pistol_ballistic"
        );
        assert_eq!(
            entity_name_stem("gmni_pistol_ballistic_01_chromic01"),
            "gmni_pistol_ballistic"
        );
    }

    #[test]
    fn stem_strips_magazine_suffix() {
        assert_eq!(
            entity_name_stem("ksar_pistol_ballistic_01_mag"),
            "ksar_pistol_ballistic"
        );
    }

    #[test]
    fn stem_strips_armor_set_color_suffixes() {
        // Geist Arms — 3 trailing numeric segments.
        assert_eq!(
            entity_name_stem("kap_combat_light_arms_01_01_01"),
            "kap_combat_light_arms"
        );
        // Snow Camo variant lives in set 02 — same stem.
        assert_eq!(
            entity_name_stem("kap_combat_light_arms_02_02_01"),
            "kap_combat_light_arms"
        );
        // Monde Core (KAP heavy core, set 02).
        assert_eq!(
            entity_name_stem("kap_combat_heavy_core_02_01_01"),
            "kap_combat_heavy_core"
        );
        // Corbel Helmet (OMC).
        assert_eq!(
            entity_name_stem("omc_utility_heavy_helmet_01_01_17"),
            "omc_utility_heavy_helmet"
        );
    }

    #[test]
    fn stem_strips_special_edition_suffix() {
        assert_eq!(
            entity_name_stem("lbco_sniper_energy_imp01"),
            "lbco_sniper_energy"
        );
    }

    #[test]
    fn stem_keeps_short_names() {
        // Don't shrink below 3 segments — that would over-bundle.
        assert_eq!(entity_name_stem("kap_01"), "kap_01");
        assert_eq!(entity_name_stem("kap_combat_01"), "kap_combat_01");
        // 3 segments + numeric suffix → strips once (would leave 3).
        assert_eq!(entity_name_stem("kap_combat_light_01"), "kap_combat_light");
        // 3 segments → already at floor, no strip.
        assert_eq!(entity_name_stem("kap_combat_light"), "kap_combat_light");
    }

    #[test]
    fn stem_keeps_model_name_segments() {
        // `_arms`, `_helmet`, `_pistol` are model parts, not variants.
        assert_eq!(
            entity_name_stem("kap_combat_light_arms"),
            "kap_combat_light_arms"
        );
    }

    // ── Family struct + ItemFamilies API ──

    fn family(id: &str, base: u8, variants: &[u8]) -> Family {
        let base_guid = Guid::from_bytes([base; 16]);
        let mut members = vec![base_guid];
        members.extend(
            variants
                .iter()
                .map(|&v| Guid::from_bytes([v; 16])),
        );
        Family { id: id.to_string(), base: base_guid, members }
    }

    #[test]
    fn family_helpers() {
        let f = family("X", 1, &[2, 3]);
        assert_eq!(f.len(), 3);
        assert!(!f.is_solo());
        assert_eq!(f.variants().count(), 2);

        let solo = family("Y", 5, &[]);
        assert_eq!(solo.len(), 1);
        assert!(solo.is_solo());
        assert_eq!(solo.variants().count(), 0);
    }

    #[test]
    fn item_families_api() {
        let mut f = ItemFamilies::new();
        let fam = family("fam-a", 1, &[2, 3]);
        let solo = family("fam-b", 5, &[]);
        f.by_member.insert(Guid::from_bytes([1; 16]), "fam-a".into());
        f.by_member.insert(Guid::from_bytes([2; 16]), "fam-a".into());
        f.by_member.insert(Guid::from_bytes([3; 16]), "fam-a".into());
        f.by_member.insert(Guid::from_bytes([5; 16]), "fam-b".into());
        f.by_family.insert("fam-a".into(), fam);
        f.by_family.insert("fam-b".into(), solo);

        assert_eq!(f.len(), 2);
        assert_eq!(
            f.family_id_of(&Guid::from_bytes([2; 16])),
            Some("fam-a")
        );
        assert_eq!(
            f.base_of(&Guid::from_bytes([3; 16])),
            Some(Guid::from_bytes([1; 16]))
        );
        assert!(f.family_of(&Guid::from_bytes([99; 16])).is_none());

        let json = serde_json::to_string(&f).unwrap();
        let back: ItemFamilies = serde_json::from_str(&json).unwrap();
        assert_eq!(back.len(), 2);
        assert_eq!(
            back.base_of(&Guid::from_bytes([2; 16])),
            Some(Guid::from_bytes([1; 16]))
        );
    }
}
