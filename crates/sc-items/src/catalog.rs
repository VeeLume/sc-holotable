//! Item catalog — a two-tier grouping of item entities by what users *read*.
//!
//! Star Citizen models items at two granularities a catalog UI cares about, but
//! CIG exposes neither cleanly. This module recovers both:
//!
//! - **[`Model`]** — one model plus its colorway variants: "Geist Armor Helmet"
//!   and all its Rogue/Desert/Snow-Camo paints; "LH86 Pistol" and its skins;
//!   "LH86 Magazine" on its own. The base + variants unit (the expandable row
//!   in a catalog).
//! - **[`Collection`]** — models that read as the **same design**: "Geist Armor"
//!   helmet + arms + core + legs; a gun together with its magazine.
//!
//! # Why not CIG's set tag?
//!
//! The DCB's `Armor / FPS / Set / <Brand> / <Model>` tag looks like a design
//! grouping but isn't: CIG reuses one set tag across visually-unrelated models.
//! `… / ClarkeDefense / FieldRecon` bundles a KastakArms "Geist Armor Helmet",
//! a ClarkeDefense "Field Recon Suit Helmet", and an "FBL-8a Helmet" — three
//! designs a user would never call one set. So the tag is dropped from
//! classification entirely (consumers can still read raw tags off the entity).
//!
//! # One signal: the display name
//!
//! Both tiers come from the **design name** — the base display name's words up
//! to the slot noun (Helmet/Arms/Core/…, known from `item_type`):
//! "Geist Armor Helmet" → design "Geist Armor". Colorways always trail *after*
//! the slot noun ("Geist Armor Helmet Snow Camo", "FBL-8a Helmet Justified"),
//! so they share a design regardless of how the colorway or the underlying
//! record is named — no fragile suffix-stripping needed.
//!
//! - A **[`Model`]** is one design + one slot (`item_type`/`item_sub_type`) +
//!   `size`/`grade`: "Geist Armor" + Helmet = the helmet and all its colorways.
//!   The `size`/`grade` split keeps distinct ship-weapon size classes apart
//!   ("Deadbolt I Cannon" S1 vs "Deadbolt V Cannon" S5, both design "Deadbolt").
//! - A **[`Collection`]** is one design across slots/sizes: "Geist Armor" =
//!   helmet + arms + core + legs; "Deadbolt" = the S1–S6 cannon ladder.
//!
//! This is the signal users actually read, and the only one we rely on
//! (CLAUDE.md design principle 5 — no typed alternative exists, the CIG tag
//! being unreliable); [`ItemCatalog::build`] logs the model/collection counts
//! so a silent data-shape change surfaces. Weapons/clothing have no armor slot
//! noun, so their design falls back to the leading word (grouping a gun with
//! its magazine under, e.g., "LH86"). Items with no display name can't be
//! placed by design and stand alone.
//!
//! # Base detection
//!
//! Within a model the base/header is the plain item — the one whose name is
//! just "design + slot" with no colorway ("Inquisitor Arms"). When no plain
//! item exists, the preference falls to a "…Base" default ("Inquisitor Arms
//! Base"), then CIG's canonical first colorway (entity name ending `_01_01_01`
//! — e.g. Odyssey II "Alpha", ADP-mk4 "Woodland"), then the shortest display
//! name. The base is always `members[0]`.
//!
//! # Coverage
//!
//! A **gear** catalog: armor, clothing, and weapons. Built over [`Items`],
//! restricted to real inventory items ([`crate::Item::is_inventory_item`]).
//! Excluded: non-inventory attachables (NPC archetypes, seat-access, doors,
//! tattoos); non-gear items (ship components, world props), which belong to
//! other domains and have no "design"; and dev-template items with placeholder
//! display names (`<= PLACEHOLDER =>`).

use std::collections::HashMap;

use sc_extract::{Guid, LocaleMap, RecordPaths};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{Items, RecordCollection};

/// Opaque model identifier. Treat as an opaque grouping key.
pub type ModelId = String;

/// Opaque collection identifier. Treat as an opaque grouping key.
pub type CollectionId = String;

/// One model: a base item plus its colorway variants. `members` always starts
/// with `base` and is ordered deterministically (shortest display name first).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: ModelId,
    /// The collection (design) this model belongs to, if it shares a design
    /// with at least one other model. `None` for a model whose design stands
    /// alone.
    pub collection: Option<CollectionId>,
    /// The canonical "unstyled" item. Same GUID also appears in `members[0]`.
    pub base: Guid,
    /// All members (base + colorway variants), base first.
    pub members: Vec<Guid>,
    /// `AttachDef.Type` shared by every member, as its DCB string.
    pub item_type: String,
    /// `AttachDef.SubType` shared by every member, as its DCB string.
    pub item_sub_type: String,
}

impl Model {
    /// Iterate the non-base variant GUIDs.
    pub fn variants(&self) -> impl Iterator<Item = Guid> + '_ {
        self.members.iter().skip(1).copied()
    }

    /// Total member count (base + variants). Always ≥ 1.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// `true` if this is a single-member model (just the base, no known
    /// colorway variants).
    pub fn is_solo(&self) -> bool {
        self.members.len() == 1
    }
}

/// One collection: the [`Model`]s that read as the same design — an armor
/// design across its slots, or a weapon with its accessories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Collection {
    pub id: CollectionId,
    /// Human design name, e.g. "Geist Armor".
    pub name: String,
    /// Member model ids, ordered deterministically (by base display name).
    pub models: Vec<ModelId>,
}

impl Collection {
    /// Number of models in this collection.
    pub fn model_count(&self) -> usize {
        self.models.len()
    }
}

/// Two-tier index of every inventory item's model and collection, with lookups
/// in both directions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemCatalog {
    by_member: HashMap<Guid, ModelId>,
    by_model: HashMap<ModelId, Model>,
    by_collection: HashMap<CollectionId, Collection>,
}

impl ItemCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the catalog from a parsed datacore. `items` supplies item metadata
    /// and typed classification; `locale` resolves display names; `paths`
    /// supplies entity record names, used only to pick a model's base/header
    /// when the display names give no clean default. Classification (which items
    /// form a model/collection) is by display-name design only — no tag tree.
    pub fn build(items: &Items, paths: &RecordPaths, locale: &LocaleMap) -> Self {
        // Pass 1 — group items into models by (design, item_type, sub_type).
        // The design is derived per item; colorways of one model resolve to the
        // same design, so they land together regardless of record naming. Items
        // with no display name can't be designed and stand alone.
        let mut groups: HashMap<String, Vec<Guid>> = HashMap::new();
        // model id -> (collection key, design display name) for pass 3.
        let mut design_of_model: HashMap<String, (String, String)> = HashMap::new();
        for (guid, item) in items.iter() {
            if !item.is_inventory_item() {
                continue;
            }
            let it = item.item_type.as_dcb_str();
            let ist = item.item_sub_type.as_dcb_str();
            // Design grouping is a gear concept — armor, clothing, weapons. Other
            // inventory items (ship components, world props) belong to other
            // domains and have no design, so the gear catalog skips them.
            let cat = category_of(it);
            if cat == "other" {
                continue;
            }
            // Skip dev-template / unreleased items, which ship with placeholder
            // display names like "<= PLACEHOLDER =>".
            let Some(name) = display_of(guid, items, locale).filter(|n| !is_placeholder(n)) else {
                continue;
            };
            // The collection groups a whole design; the model is one item within
            // it, separated also by `size`/`grade`. That structural split keeps
            // distinct ship-weapon size classes apart ("Deadbolt I Cannon" S1 vs
            // "Deadbolt V Cannon" S5 share the design "Deadbolt") while colorways
            // — same design, type, size, grade — still fold into one model.
            let col_design = derive_design(name, it);
            let key = if col_design.is_empty() {
                format!("solo:{guid}:{it}:{ist}")
            } else {
                let col_key = format!("{cat}:{}", col_design.to_lowercase());
                let model_key = format!("{col_key}:{it}:{ist}:{}:{}", item.size, item.grade);
                design_of_model
                    .entry(model_key.clone())
                    .or_insert_with(|| (col_key, col_design));
                model_key
            };
            groups.entry(key).or_default().push(*guid);
        }

        // Pass 2 — one model per group: pick the base.
        let mut by_member: HashMap<Guid, ModelId> = HashMap::with_capacity(items.len());
        let mut by_model: HashMap<ModelId, Model> = HashMap::with_capacity(groups.len());
        for (key, mut members) in groups {
            members.sort_by(|a, b| {
                member_sort_key(*a, items, locale, paths)
                    .cmp(&member_sort_key(*b, items, locale, paths))
            });
            let base = members[0];
            let (it, ist) = items
                .get(&base)
                .map(|i| {
                    (
                        i.item_type.as_dcb_str().to_string(),
                        i.item_sub_type.as_dcb_str().to_string(),
                    )
                })
                .unwrap_or_default();
            for g in &members {
                by_member.insert(*g, key.clone());
            }
            by_model.insert(
                key.clone(),
                Model {
                    id: key,
                    collection: None,
                    base,
                    members,
                    item_type: it,
                    item_sub_type: ist,
                },
            );
        }

        // Pass 3 — group models into collections by design. A design with only
        // one model isn't a collection (nothing to group), so it's left
        // unlinked.
        let mut grouped: HashMap<String, (String, Vec<ModelId>)> = HashMap::new();
        for (mid, (col_key, design)) in design_of_model {
            let entry = grouped
                .entry(col_key)
                .or_insert_with(|| (design, Vec::new()));
            entry.1.push(mid);
        }
        let mut by_collection: HashMap<CollectionId, Collection> = HashMap::new();
        for (ck, (name, mut mids)) in grouped {
            if mids.len() < 2 {
                continue;
            }
            mids.sort_by(|a, b| {
                let ka = member_sort_key(by_model[a].base, items, locale, paths);
                let kb = member_sort_key(by_model[b].base, items, locale, paths);
                ka.cmp(&kb)
            });
            for mid in &mids {
                if let Some(m) = by_model.get_mut(mid) {
                    m.collection = Some(ck.clone());
                }
            }
            by_collection.insert(
                ck.clone(),
                Collection {
                    id: ck,
                    name,
                    models: mids,
                },
            );
        }

        info!(
            models = by_model.len(),
            collections = by_collection.len(),
            "ItemCatalog built"
        );

        Self {
            by_member,
            by_model,
            by_collection,
        }
    }

    /// The model a member belongs to.
    pub fn model_of(&self, guid: &Guid) -> Option<&Model> {
        let mid = self.by_member.get(guid)?;
        self.by_model.get(mid)
    }

    pub fn model_id_of(&self, guid: &Guid) -> Option<&str> {
        self.by_member.get(guid).map(String::as_str)
    }

    pub fn base_of(&self, guid: &Guid) -> Option<Guid> {
        self.model_of(guid).map(|m| m.base)
    }

    /// The collection a member belongs to, if its model links to one.
    pub fn collection_of(&self, guid: &Guid) -> Option<&Collection> {
        let cid = self.model_of(guid)?.collection.as_ref()?;
        self.by_collection.get(cid)
    }

    pub fn model_by_id(&self, id: &str) -> Option<&Model> {
        self.by_model.get(id)
    }

    pub fn collection_by_id(&self, id: &str) -> Option<&Collection> {
        self.by_collection.get(id)
    }

    /// Iterate the models of a collection.
    pub fn models_in<'a>(
        &'a self,
        collection: &'a Collection,
    ) -> impl Iterator<Item = &'a Model> + 'a {
        collection
            .models
            .iter()
            .filter_map(|mid| self.by_model.get(mid))
    }

    /// Iterate every member GUID across a collection's models.
    pub fn members_of_collection<'a>(
        &'a self,
        collection: &'a Collection,
    ) -> impl Iterator<Item = Guid> + 'a {
        self.models_in(collection)
            .flat_map(|m| m.members.iter().copied())
    }

    /// Iterate every model. Order unspecified.
    pub fn models(&self) -> impl Iterator<Item = &Model> + '_ {
        self.by_model.values()
    }

    /// Iterate every collection. Order unspecified.
    pub fn collections(&self) -> impl Iterator<Item = &Collection> + '_ {
        self.by_collection.values()
    }

    /// Number of distinct models.
    pub fn model_count(&self) -> usize {
        self.by_model.len()
    }

    /// Number of distinct collections.
    pub fn collection_count(&self) -> usize {
        self.by_collection.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_model.is_empty()
    }
}

// ── Model grouping helpers ─────────────────────────────────────────────

fn display_of<'a>(guid: &Guid, items: &Items, locale: &'a LocaleMap) -> Option<&'a str> {
    items.get(guid).and_then(|i| i.display_name(locale))
}

/// Dev-template / unreleased items ship with placeholder display names like
/// "&lt;= PLACEHOLDER =&gt;". Treat any name that opens with `<` as a placeholder.
fn is_placeholder(name: &str) -> bool {
    name.trim_start().starts_with('<')
}

/// Sort key for picking the base within a model: named members before unnamed,
/// then by [`base_rank`], then shortest display name, then alphabetical; GUID
/// string as the final tiebreak for unnamed members.
fn member_sort_key(
    guid: Guid,
    items: &Items,
    locale: &LocaleMap,
    paths: &RecordPaths,
) -> (u8, u8, usize, String) {
    let display = display_of(&guid, items, locale);
    let item_type = items.get(&guid).map(|i| i.item_type.as_dcb_str());
    let rank = base_rank(display, item_type, guid, paths);
    match display {
        Some(d) => (0, rank, d.chars().count(), d.to_string()),
        None => (1, rank, 0, guid.to_string()),
    }
}

/// Base/header preference (lower = more base-like):
/// `0` the plain item, `1` a "…Base" default, `2` the canonical first colorway
/// (entity record name ending `_01_01_01` — often the base when no plain item
/// exists, e.g. Odyssey II "Alpha", ADP-mk4 "Woodland"), `3` any other variant.
/// The entity hint ranks below the name signals so a cleanly-named base still
/// wins even when an event variant shipped first (e.g. "Strata Helmet").
fn base_rank(name: Option<&str>, item_type: Option<&str>, guid: Guid, paths: &RecordPaths) -> u8 {
    if let (Some(name), Some(it)) = (name, item_type)
        && let r @ (0 | 1) = display_base_rank(name, it)
    {
        return r;
    }
    if is_canonical_first(guid, paths) {
        2
    } else {
        3
    }
}

/// Display-name base signal: `0` = the plain item, `1` = a "…Base" default,
/// `3` = any other variant. The "plain item" is the one with nothing after the
/// armor slot noun ("Inquisitor Arms"); for weapons/clothing (no slot noun) the
/// one with no quoted paint qualifier (paints read e.g. `Parallax "ArcCorp"
/// Rifle`, the plain one "Parallax Energy Assault Rifle").
fn display_base_rank(name: &str, item_type: &str) -> u8 {
    let slots = slot_nouns_for(item_type);
    if slots.is_empty() {
        return if name.contains('"') { 3 } else { 0 };
    }
    let words: Vec<&str> = name.split_whitespace().collect();
    let Some(i) = words
        .iter()
        .position(|w| slots.iter().any(|s| w.eq_ignore_ascii_case(s)))
    else {
        return 3;
    };
    let after = &words[i + 1..];
    if after.is_empty() {
        0
    } else if after.len() == 1 && after[0].eq_ignore_ascii_case("base") {
        1
    } else {
        3
    }
}

/// True if the entity record name ends in `_01_01_01` — CIG's canonical first
/// colorway, a useful base hint when the display names offer no clean default.
fn is_canonical_first(guid: Guid, paths: &RecordPaths) -> bool {
    paths.get(&guid).is_some_and(|rp| {
        rp.name
            .strip_prefix("EntityClassDefinition.")
            .unwrap_or(&rp.name)
            .ends_with("_01_01_01")
    })
}

// ── Collection (design) derivation ─────────────────────────────────────

/// The design name of a model from its base display name: the words up to the
/// slot noun (known from `item_type`), e.g. "Geist Armor Helmet" → "Geist
/// Armor". When there is no slot noun for the type, or it isn't found in the
/// name, fall back to the leading word ("LH86 Pistol" → "LH86"). Empty when the
/// name is blank.
fn derive_design(name: &str, item_type: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();
    let Some(&first) = words.first() else {
        return String::new();
    };
    let slots = slot_nouns_for(item_type);
    if !slots.is_empty()
        && let Some(i) = words
            .iter()
            .position(|w| slots.iter().any(|s| w.eq_ignore_ascii_case(s)))
        && i > 0
    {
        return words[..i].join(" ");
    }
    first.to_string()
}

/// The display nouns an armor slot is named with, keyed by `item_type`. Empty
/// for non-armor (weapons/clothing), where the design falls back to the leading
/// word.
fn slot_nouns_for(item_type: &str) -> &'static [&'static str] {
    match item_type {
        "Char_Armor_Helmet" => &["Helmet"],
        "Char_Armor_Arms" => &["Arms"],
        "Char_Armor_Torso" => &["Core", "Torso"],
        "Char_Armor_Legs" => &["Legs"],
        "Char_Armor_Backpack" => &["Backpack", "Pack"],
        "Char_Armor_Undersuit" => &["Undersuit"],
        _ => &[],
    }
}

/// Broad category from an `item_type`, scoping collection keys so two unrelated
/// designs that share a leading word (a "Pioneer" gun vs "Pioneer" armor) don't
/// merge.
fn category_of(item_type: &str) -> &'static str {
    if item_type.starts_with("Char_Armor") {
        "armor"
    } else if item_type.starts_with("Weapon") {
        "weapon"
    } else if item_type.starts_with("Char_Clothing") {
        "clothing"
    } else {
        "other"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(b: u8) -> Guid {
        Guid::from_bytes([b; 16])
    }

    // ── derive_design ──

    #[test]
    fn design_strips_at_slot_noun() {
        assert_eq!(
            derive_design("Geist Armor Helmet", "Char_Armor_Helmet"),
            "Geist Armor"
        );
        assert_eq!(derive_design("Lynx Arms", "Char_Armor_Arms"), "Lynx");
        assert_eq!(
            derive_design("Field Recon Suit Helmet", "Char_Armor_Helmet"),
            "Field Recon Suit"
        );
        assert_eq!(
            derive_design("CSP-68L Backpack", "Char_Armor_Backpack"),
            "CSP-68L"
        );
    }

    #[test]
    fn design_ignores_colorway_after_slot_noun() {
        // No bare base: base name carries a colorway after the slot noun.
        assert_eq!(
            derive_design("ORC-mkX Core Arctic", "Char_Armor_Torso"),
            "ORC-mkX"
        );
        assert_eq!(
            derive_design("Geist Armor Helmet Snow Camo", "Char_Armor_Helmet"),
            "Geist Armor"
        );
    }

    #[test]
    fn design_falls_back_to_first_word() {
        // Slot noun absent from the name → leading word.
        assert_eq!(
            derive_design("Antium Core Jet", "Char_Armor_Helmet"),
            "Antium"
        );
        // Non-armor types have no slot noun.
        assert_eq!(derive_design("LH86 Pistol", "WeaponPersonal"), "LH86");
        assert_eq!(
            derive_design("LH86 Pistol Magazine (25 cap)", "WeaponAttachment"),
            "LH86"
        );
    }

    #[test]
    fn design_empty_for_blank_name() {
        assert_eq!(derive_design("", "Char_Armor_Helmet"), "");
    }

    #[test]
    fn display_base_rank_prefers_plain_then_base() {
        // Plain "design + slot" wins.
        assert_eq!(display_base_rank("Inquisitor Arms", "Char_Armor_Arms"), 0);
        assert_eq!(display_base_rank("ORC-mkV Core", "Char_Armor_Torso"), 0);
        // A "…Base" default beats a colorway.
        assert_eq!(
            display_base_rank("Inquisitor Arms Base", "Char_Armor_Arms"),
            1
        );
        assert_eq!(
            display_base_rank("TrueDef-Pro Core Base", "Char_Armor_Torso"),
            1
        );
        // Ordinary colorways → 3 (entity-name hint, then length, decide later).
        assert_eq!(
            display_base_rank("Inquisitor Arms Red", "Char_Armor_Arms"),
            3
        );
        assert_eq!(
            display_base_rank("TrueDef-Pro Core CDF", "Char_Armor_Torso"),
            3
        );
        assert_eq!(
            display_base_rank("Corbel Helmet Mire", "Char_Armor_Helmet"),
            3
        );
        // Weapons: the plain (unquoted) name is the base; quoted paints aren't.
        assert_eq!(
            display_base_rank("Parallax Energy Assault Rifle", "WeaponPersonal"),
            0
        );
        assert_eq!(
            display_base_rank(
                "Parallax \"ArcCorp\" Energy Assault Rifle",
                "WeaponPersonal"
            ),
            3
        );
        assert_eq!(display_base_rank("LH86 Pistol", "WeaponPersonal"), 0);
        // Slot noun absent from an armor name → 3 (a colorway, not the plain item).
        assert_eq!(display_base_rank("Antium Core Jet", "Char_Armor_Helmet"), 3);
    }

    #[test]
    fn category_scoping() {
        assert_eq!(category_of("Char_Armor_Helmet"), "armor");
        assert_eq!(category_of("WeaponPersonal"), "weapon");
        assert_eq!(category_of("WeaponAttachment"), "weapon");
        assert_eq!(category_of("Char_Clothing_Torso_0"), "clothing");
        assert_eq!(category_of("Misc"), "other");
    }

    // ── Model / Collection / ItemCatalog API ──

    fn model(id: &str, collection: Option<&str>, base: u8, variants: &[u8]) -> Model {
        let base_guid = g(base);
        let mut members = vec![base_guid];
        members.extend(variants.iter().map(|&v| g(v)));
        Model {
            id: id.to_string(),
            collection: collection.map(str::to_string),
            base: base_guid,
            members,
            item_type: "Char_Armor_Helmet".into(),
            item_sub_type: "Light".into(),
        }
    }

    #[test]
    fn model_helpers() {
        let m = model("X", Some("armor:geist armor"), 1, &[2, 3]);
        assert_eq!(m.len(), 3);
        assert!(!m.is_solo());
        assert_eq!(m.variants().count(), 2);

        let solo = model("Y", None, 5, &[]);
        assert_eq!(solo.len(), 1);
        assert!(solo.is_solo());
    }

    #[test]
    fn catalog_api_and_serde() {
        // A collection "Geist Armor" with a helmet model + an arms model, plus
        // a standalone model with no collection.
        let mut cat = ItemCatalog::new();
        let helm = model("m-helm", Some("armor:geist armor"), 1, &[2]);
        let arms = model("m-arms", Some("armor:geist armor"), 3, &[]);
        let lone = model("m-lone", None, 5, &[]);
        for m in [&helm, &arms, &lone] {
            for mem in &m.members {
                cat.by_member.insert(*mem, m.id.clone());
            }
            cat.by_model.insert(m.id.clone(), m.clone());
        }
        cat.by_collection.insert(
            "armor:geist armor".into(),
            Collection {
                id: "armor:geist armor".into(),
                name: "Geist Armor".into(),
                models: vec!["m-helm".into(), "m-arms".into()],
            },
        );

        assert_eq!(cat.model_count(), 3);
        assert_eq!(cat.collection_count(), 1);
        assert_eq!(cat.model_id_of(&g(2)), Some("m-helm"));
        assert_eq!(cat.base_of(&g(2)), Some(g(1)));

        let col = cat.collection_of(&g(2)).unwrap();
        assert_eq!(col.name, "Geist Armor");
        assert_eq!(col.model_count(), 2);
        let model_ids: Vec<&str> = cat.models_in(col).map(|m| m.id.as_str()).collect();
        assert!(model_ids.contains(&"m-helm") && model_ids.contains(&"m-arms"));
        let mut col_members: Vec<Guid> = cat.members_of_collection(col).collect();
        col_members.sort_by_key(|x| x.to_string());
        assert_eq!(col_members, vec![g(1), g(2), g(3)]);

        assert!(cat.collection_of(&g(5)).is_none());

        let json = serde_json::to_string(&cat).unwrap();
        let back: ItemCatalog = serde_json::from_str(&json).unwrap();
        assert_eq!(back.model_count(), 3);
        assert_eq!(back.collection_count(), 1);
        assert_eq!(
            back.collection_of(&g(2)).map(|c| c.name.as_str()),
            Some("Geist Armor")
        );
    }
}
