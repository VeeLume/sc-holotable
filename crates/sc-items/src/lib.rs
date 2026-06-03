//! Universal item envelope for Star Citizen entity records.
//!
//! Owns the per-entity item metadata that used to live in `sc-extract`'s
//! snapshot as `LocalizedItemCache`. An [`Item`] is the curated view of an
//! `EntityClassDefinition`'s `SAttachableComponentParams.AttachDef`
//! (`SItemDefinition`) block: localization keys + typed `Type`/`SubType`
//! classification. [`Items`] indexes every such entity by GUID.
//!
//! # Why this is its own crate
//!
//! `sc-extract` is the generic DCB foundation; the AttachDef walk is
//! *item-shaped* domain knowledge, so it belongs here. Building the cache
//! is an explicit [`Items::build`] call — there is no `DatacoreConfig`
//! flag to forget, so the "silently empty" failure mode is gone.
//!
//! # Typed surface
//!
//! The walk goes through the **typed pool surface** (`Handle::get(pools)` +
//! the `DataForgeComponentParamsPtr` poly enum), not raw `Instance` pokes.
//! `Type`/`SubType` come back as `EItemType`/`EItemSubType`, and the same
//! `&SItemDefinition` is the entry point for future envelope additions
//! (item ports, inventory occupancy, manufacturer). sc-items therefore
//! enables the `item` sc-extract feature (it owns its feature closure).
//!
//! # Sharing
//!
//! [`Items::build`] returns an **owned** cache. Build it **once** and
//! pass `&Items` to the consumers/builders that need it (the umbrella
//! `sc-holotable` crate orchestrates this for end-consumers). Don't rebuild
//! per call site — the walk touches every `EntityClassDefinition`.
//!
//! # Coverage
//!
//! Exhaustive for **attachable items**: every entity carrying
//! `SAttachableComponentParams` also carries `AttachDef` (verified against
//! live DCB — no gap). Entities without that component (geometry, props,
//! spawners, templates, and a few commodity/consumable shapes like
//! `Commodities_Food_*`) are **not** items and are absent — those belong to
//! future domain crates (`sc-resources`/`sc-economy`). The cache still
//! includes non-inventory *attachable* entities (`NOITEM_Player`,
//! `NOITEM_Vehicle`, doors, seats, thrusters); use
//! [`Item::is_inventory_item`] to filter to real inventory items (those
//! markers are the typed `EItemType::{NOITEM_Player, NOITEM_Vehicle,
//! UNDEFINED}` variants).

use std::collections::HashMap;

use sc_extract::generated::{
    DataForgeComponentParamsPtr, EItemSubType, EItemType, EntityClassDefinition, RecordLookup,
    SAttachableComponentParams,
};
use sc_extract::{DataPools, Guid, LocaleKey, LocaleMap, RecordStore};
use serde::{Deserialize, Serialize};
use tracing::warn;

pub mod catalog;
pub mod variants;

pub use catalog::{Collection, CollectionId, ItemCatalog, Model, ModelId};

/// serde adapters for the generated item enums. They carry no serde of their
/// own (the generated crate stays serde-free to avoid the monomorphization
/// cliff — see the project note), so we store each as its DCB string via the
/// generator-emitted `as_dcb_str` / `from_dcb_str` round-trip.
mod enum_serde {
    use sc_extract::generated::{EItemSubType, EItemType};
    use serde::{Deserialize, Deserializer, Serializer};

    pub mod item_type {
        use super::*;
        pub fn serialize<S: Serializer>(v: &EItemType, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(v.as_dcb_str())
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EItemType, D::Error> {
            Ok(EItemType::from_dcb_str(&String::deserialize(d)?))
        }
    }

    pub mod item_sub_type {
        use super::*;
        pub fn serialize<S: Serializer>(v: &EItemSubType, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(v.as_dcb_str())
        }
        pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EItemSubType, D::Error> {
            Ok(EItemSubType::from_dcb_str(&String::deserialize(d)?))
        }
    }
}

/// Per-entity item metadata from the `SAttachableComponentParams.AttachDef`
/// (`SItemDefinition`) block.
///
/// The three [`LocaleKey`]s keep the leading `@` the DCB carries — keys are
/// raw, resolution happens at the call site (see [`Item::display_name`]).
/// `item_type` / `item_sub_type` are the typed [`EItemType`] /
/// [`EItemSubType`] enums (`Unrecognized(..)` only for values added in a
/// game patch the generator hasn't seen yet).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    /// `Localization.Name` — the primary display-name key.
    pub name_key: Option<LocaleKey>,
    /// `Localization.ShortName` — manufacturer ticker / short ID; empty on
    /// most entities.
    pub short_name_key: Option<LocaleKey>,
    /// `Localization.Description` — long-form description key.
    pub desc_key: Option<LocaleKey>,
    /// `AttachDef.Type` — typed item-type classification.
    #[serde(with = "enum_serde::item_type")]
    pub item_type: EItemType,
    /// `AttachDef.SubType` — typed item-subtype classification.
    #[serde(with = "enum_serde::item_sub_type")]
    pub item_sub_type: EItemSubType,
    /// `AttachDef.Size` — the item's size class (e.g. ship-weapon mount size
    /// S1–S6). `0` when the item carries no meaningful size.
    pub size: i32,
    /// `AttachDef.Grade` — the item's quality grade. `0` when ungraded.
    pub grade: i32,
}

impl Item {
    /// Resolve the display name (`Name` key) through a [`LocaleMap`].
    /// Returns `None` when there's no key or it resolves to empty text.
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        let key = self.name_key.as_ref()?;
        let name = locale.resolve(key)?;
        (!name.is_empty()).then_some(name)
    }

    /// Whether this is a real **inventory item** (not a non-item attachable
    /// entity). Being in the cache means it has an `AttachDef`; this
    /// additionally excludes the `NOITEM_*` / `UNDEFINED` component shells
    /// (player/vehicle), doors/seats/thrusters carry their own real Types.
    /// Matches the known typed variants, plus a defensive guard for a future
    /// `NOITEM_*` value the generated enum hasn't seen yet (→ `Unrecognized`).
    pub fn is_inventory_item(&self) -> bool {
        !matches!(
            self.item_type,
            EItemType::NOITEM_Player | EItemType::NOITEM_Vehicle | EItemType::UNDEFINED
        ) && !matches!(&self.item_type, EItemType::Unrecognized(s) if s.starts_with("NOITEM_"))
    }
}

/// Class CRC of a record GUID — re-exported from the foundation. The reverse
/// lookup here ([`Items::by_crc`]) is item-scoped; for any record GUID use
/// `sc_extract::CrcIndex`.
pub use sc_extract::class_crc;

/// Per-entity [`Item`] metadata for every `EntityClassDefinition` that
/// exposes an `AttachDef`. Build **once** via [`Items::build`] and share
/// by reference.
///
/// Carries a reverse [`class_crc`] → GUID index ([`Items::guid_by_crc`] /
/// [`Items::by_crc`]) so an EntityGraph wire CRC resolves back to a GUID/item.
/// The index is derived from `by_record`, so it is not serialized — it's
/// rebuilt on deserialize via the [`ItemsRepr`] shadow.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "ItemsRepr", into = "ItemsRepr")]
pub struct Items {
    by_record: HashMap<Guid, Item>,
    /// Reverse index: [`class_crc`] of each cached GUID → that GUID. Rebuilt,
    /// not serialized (see [`ItemsRepr`]).
    by_crc: HashMap<u32, Guid>,
}

/// Serialization shadow for [`Items`]: only `by_record` is persisted; the
/// `by_crc` reverse index is recomputed on the way back in.
#[derive(Serialize, Deserialize)]
struct ItemsRepr {
    by_record: HashMap<Guid, Item>,
}

impl From<ItemsRepr> for Items {
    fn from(repr: ItemsRepr) -> Self {
        let mut items = Items {
            by_record: repr.by_record,
            by_crc: HashMap::new(),
        };
        items.rebuild_crc_index();
        items
    }
}

impl From<Items> for ItemsRepr {
    fn from(items: Items) -> Self {
        ItemsRepr {
            by_record: items.by_record,
        }
    }
}

impl Items {
    /// Empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the cache from a parsed [`RecordStore`] by walking the typed
    /// `EntityClassDefinition` pool. Returns an owned cache — build once,
    /// share `&Items`.
    pub fn build(store: &RecordStore) -> Self {
        let pools = &store.pools;
        let mut cache = Self::new();
        for (&guid, &handle) in &store.records.multi_feature.entity_class_definition {
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            if let Some(item) = item_for(ecd, pools) {
                cache.insert(guid, item);
            }
        }
        cache
    }

    /// Insert or replace a record's entry, keeping the [`class_crc`] reverse
    /// index in sync.
    pub fn insert(&mut self, guid: Guid, item: Item) {
        self.by_record.insert(guid, item);
        self.index_crc(guid);
    }

    /// Record `guid` in the reverse [`class_crc`] index. Logs (but tolerates) a
    /// CRC collision: CRC32 can in principle collide across the ~25k catalog
    /// entries, though none is observed in live data.
    fn index_crc(&mut self, guid: Guid) {
        let crc = class_crc(&guid);
        if let Some(prev) = self.by_crc.insert(crc, guid)
            && prev != guid
        {
            warn!(crc, %prev, new = %guid, "class_crc collision: by_crc entry overwritten");
        }
    }

    /// Rebuild the reverse [`class_crc`] index from `by_record` (used after
    /// deserialization, where only `by_record` is persisted).
    fn rebuild_crc_index(&mut self) {
        self.by_crc.clear();
        self.by_crc.reserve(self.by_record.len());
        let guids: Vec<Guid> = self.by_record.keys().copied().collect();
        for guid in guids {
            self.index_crc(guid);
        }
    }

    /// Resolve an EntityGraph wire [`class_crc`] back to its record GUID.
    pub fn guid_by_crc(&self, crc: u32) -> Option<Guid> {
        self.by_crc.get(&crc).copied()
    }

    /// Resolve an EntityGraph wire [`class_crc`] back to its [`Item`].
    pub fn by_crc(&self, crc: u32) -> Option<&Item> {
        self.by_record.get(self.by_crc.get(&crc)?)
    }

    /// Look up the entry for a record GUID.
    pub fn get(&self, guid: &Guid) -> Option<&Item> {
        self.by_record.get(guid)
    }

    /// Convenience: the `Name` key for a record.
    pub fn name_key(&self, guid: &Guid) -> Option<&LocaleKey> {
        self.by_record.get(guid).and_then(|i| i.name_key.as_ref())
    }

    /// Convenience: the `Description` key for a record.
    pub fn desc_key(&self, guid: &Guid) -> Option<&LocaleKey> {
        self.by_record.get(guid).and_then(|i| i.desc_key.as_ref())
    }

    /// Convenience: the `ShortName` key for a record.
    pub fn short_name_key(&self, guid: &Guid) -> Option<&LocaleKey> {
        self.by_record
            .get(guid)
            .and_then(|i| i.short_name_key.as_ref())
    }

    /// Convenience: the typed `Type` classification for a record.
    pub fn item_type(&self, guid: &Guid) -> Option<&EItemType> {
        self.by_record.get(guid).map(|i| &i.item_type)
    }

    /// Convenience: the typed `SubType` classification for a record.
    pub fn item_sub_type(&self, guid: &Guid) -> Option<&EItemSubType> {
        self.by_record.get(guid).map(|i| &i.item_sub_type)
    }

    /// Number of cached entities.
    pub fn len(&self) -> usize {
        self.by_record.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_record.is_empty()
    }

    /// Iterate `(guid, item)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&Guid, &Item)> + '_ {
        self.by_record.iter()
    }
}

/// Extract the [`Item`] metadata for one `EntityClassDefinition`, or `None`
/// if it exposes no `AttachDef`. Shared by [`Items::build`] and
/// [`ItemsBuilder`].
fn item_for(ecd: &EntityClassDefinition, pools: &DataPools) -> Option<Item> {
    let attachable = find_attachable(ecd, pools)?;
    let item_def = attachable.attach_def.and_then(|h| h.get(pools))?;
    let loc = item_def.localization.and_then(|h| h.get(pools));
    Some(Item {
        name_key: loc.and_then(|l| non_empty(&l.name)),
        short_name_key: loc.and_then(|l| non_empty(&l.short_name)),
        desc_key: loc.and_then(|l| non_empty(&l.description)),
        item_type: item_def.r#type.clone(),
        item_sub_type: item_def.sub_type.clone(),
        size: item_def.size,
        grade: item_def.grade,
    })
}

/// [`sc_extract::RecordVisitor`] that builds an [`Items`] in a bundled
/// walk. Declares interest in `EntityClassDefinition` records and reads each
/// one's typed struct via the record store. Equivalent to [`Items::build`]
/// but fusible with other visitors in one pass.
#[derive(Default)]
pub struct ItemsBuilder {
    inner: Items,
}

impl sc_extract::RecordVisitor for ItemsBuilder {
    type Output = Items;

    fn interest(&self) -> sc_extract::Interest {
        sc_extract::Interest::Types(&["EntityClassDefinition"])
    }

    fn visit(&mut self, item: sc_extract::VisitItem<'_>) {
        let store = item.store;
        let pools = &store.pools;
        let Some(handle) = EntityClassDefinition::lookup(&store.records, &item.guid) else {
            return;
        };
        let Some(ecd) = handle.get(pools) else {
            return;
        };
        if let Some(cached) = item_for(ecd, pools) {
            self.inner.insert(item.guid, cached);
        }
    }

    fn finish(self) -> Items {
        self.inner
    }
}

/// Find the first `SAttachableComponentParams` on an entity's `Components`
/// (typed poly-enum match — no raw `Instance` walk, no unsafe).
fn find_attachable<'a>(
    ecd: &EntityClassDefinition,
    pools: &'a DataPools,
) -> Option<&'a SAttachableComponentParams> {
    ecd.components.iter().find_map(|c| match c {
        DataForgeComponentParamsPtr::SAttachableComponentParams(h) => h.get(pools),
        _ => None,
    })
}

/// Clone a [`LocaleKey`] only if it isn't the empty string.
fn non_empty(key: &LocaleKey) -> Option<LocaleKey> {
    (!key.as_str().is_empty()).then(|| key.clone())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn item(t: EItemType) -> Item {
        Item {
            name_key: None,
            short_name_key: None,
            desc_key: None,
            item_type: t,
            item_sub_type: EItemSubType::Unrecognized(String::new()),
            size: 0,
            grade: 0,
        }
    }

    #[test]
    fn cache_starts_empty() {
        let cache = Items::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn insert_and_get_round_trip() {
        let mut cache = Items::new();
        let guid = Guid::default();
        let mut it = item(EItemType::Armor);
        it.name_key = Some(LocaleKey::new("@item_NameTest"));
        it.desc_key = Some(LocaleKey::new("@item_DescTest"));
        cache.insert(guid, it.clone());

        let got = cache.get(&guid).unwrap();
        assert_eq!(got, &it);
        assert_eq!(cache.name_key(&guid).unwrap().as_str(), "@item_NameTest");
        assert_eq!(cache.item_type(&guid), Some(&EItemType::Armor));
        assert!(got.is_inventory_item());
    }

    #[test]
    fn serde_round_trip_via_dcb_strings() {
        let mut cache = Items::new();
        let mut a = item(EItemType::Armor);
        a.item_sub_type = EItemSubType::Unrecognized("WeirdSub".into());
        a.name_key = Some(LocaleKey::new("@item_NameA"));
        cache.insert(Guid::from_bytes([1; 16]), a.clone());
        // Unrecognized type must carry its raw DCB string through the trip.
        let b = item(EItemType::Unrecognized("FutureType".into()));
        cache.insert(Guid::from_bytes([2; 16]), b.clone());

        let json = serde_json::to_string(&cache).unwrap();
        let decoded: Items = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded.get(&Guid::from_bytes([1; 16])), Some(&a));
        assert_eq!(decoded.get(&Guid::from_bytes([2; 16])), Some(&b));
    }

    #[test]
    fn keys_keep_at_prefix() {
        let mut it = item(EItemType::Armor);
        it.name_key = Some(LocaleKey::new("@item_NameRaw"));
        assert!(it.name_key.unwrap().as_str().starts_with('@'));
    }

    #[test]
    fn class_crc_matches_live_ground_truth() {
        // Verified byte-exact against a live EntityGraph (guid, crc) pair: the
        // CRC is crc32c over CigGuid storage-order bytes.
        let guid = Guid::from_str("bba17984-86e7-4002-aab7-f33f1279fe1f").unwrap();
        assert_eq!(class_crc(&guid), 1_038_868_829);
    }

    #[test]
    fn crc_index_round_trips_to_guid_and_item() {
        let mut cache = Items::new();
        let guid = Guid::from_str("bba17984-86e7-4002-aab7-f33f1279fe1f").unwrap();
        let mut it = item(EItemType::Armor);
        it.name_key = Some(LocaleKey::new("@item_NameCrc"));
        cache.insert(guid, it.clone());

        let crc = class_crc(&guid);
        assert_eq!(cache.guid_by_crc(crc), Some(guid));
        assert_eq!(cache.by_crc(crc), Some(&it));
        // An unknown CRC resolves to nothing.
        assert_eq!(cache.guid_by_crc(crc.wrapping_add(1)), None);
    }

    #[test]
    fn crc_index_rebuilt_after_serde_round_trip() {
        let mut cache = Items::new();
        let guid = Guid::from_bytes([7; 16]);
        cache.insert(guid, item(EItemType::Armor));

        let crc = class_crc(&guid);
        let json = serde_json::to_string(&cache).unwrap();
        // The reverse index is not persisted, so it must not bloat the payload.
        assert!(!json.contains("by_crc"));
        let decoded: Items = serde_json::from_str(&json).unwrap();
        // ...but it is rebuilt on load.
        assert_eq!(decoded.guid_by_crc(crc), Some(guid));
    }

    #[test]
    fn is_inventory_item_excludes_noitem_and_undefined() {
        // Real typed non-item markers.
        assert!(!item(EItemType::NOITEM_Player).is_inventory_item());
        assert!(!item(EItemType::NOITEM_Vehicle).is_inventory_item());
        assert!(!item(EItemType::UNDEFINED).is_inventory_item());
        // Defensive: a future NOITEM_* the generator hasn't seen.
        assert!(!item(EItemType::Unrecognized("NOITEM_Future".into())).is_inventory_item());
        // Real, modeled item type → an item.
        assert!(item(EItemType::Armor).is_inventory_item());
        // Unknown-but-not-a-non-item-marker (e.g. a future item type) → item.
        assert!(item(EItemType::Unrecognized("SomeFutureType".into())).is_inventory_item());
    }
}
