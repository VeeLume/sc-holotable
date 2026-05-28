//! Universal item envelope for Star Citizen entity records.
//!
//! Owns the per-entity item metadata that used to live in `sc-extract`'s
//! snapshot as `LocalizedItemCache`. An [`Item`] is the curated view of an
//! `EntityClassDefinition`'s `SAttachableComponentParams.AttachDef`
//! (`SItemDefinition`) block: localization keys + typed `Type`/`SubType`
//! classification. [`ItemCache`] indexes every such entity by GUID.
//!
//! # Why this is its own crate
//!
//! `sc-extract` is the generic DCB foundation; the AttachDef walk is
//! *item-shaped* domain knowledge, so it belongs here. Building the cache
//! is an explicit [`ItemCache::build`] call — there is no `DatacoreConfig`
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
//! [`ItemCache::build`] returns an **owned** cache. Build it **once** and
//! pass `&ItemCache` to the consumers/builders that need it (the umbrella
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
    DataForgeComponentParamsPtr, EItemSubType, EItemType, EntityClassDefinition,
    SAttachableComponentParams,
};
use sc_extract::{DataPools, Datacore, Guid, LocaleKey, LocaleMap};

pub mod variants;

/// Per-entity item metadata from the `SAttachableComponentParams.AttachDef`
/// (`SItemDefinition`) block.
///
/// The three [`LocaleKey`]s keep the leading `@` the DCB carries — keys are
/// raw, resolution happens at the call site (see [`Item::display_name`]).
/// `item_type` / `item_sub_type` are the typed [`EItemType`] /
/// [`EItemSubType`] enums (`Unrecognized(..)` only for values added in a
/// game patch the generator hasn't seen yet).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    /// `Localization.Name` — the primary display-name key.
    pub name_key: Option<LocaleKey>,
    /// `Localization.ShortName` — manufacturer ticker / short ID; empty on
    /// most entities.
    pub short_name_key: Option<LocaleKey>,
    /// `Localization.Description` — long-form description key.
    pub desc_key: Option<LocaleKey>,
    /// `AttachDef.Type` — typed item-type classification.
    pub item_type: EItemType,
    /// `AttachDef.SubType` — typed item-subtype classification.
    pub item_sub_type: EItemSubType,
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

/// Per-entity [`Item`] metadata for every `EntityClassDefinition` that
/// exposes an `AttachDef`. Build **once** via [`ItemCache::build`] and share
/// by reference.
#[derive(Debug, Clone, Default)]
pub struct ItemCache {
    by_record: HashMap<Guid, Item>,
}

impl ItemCache {
    /// Empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the cache from a parsed [`Datacore`] by walking the typed
    /// `EntityClassDefinition` pool. Returns an owned cache — build once,
    /// share `&ItemCache`.
    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let mut cache = Self::new();

        for (&guid, &handle) in &store.records.multi_feature.entity_class_definition {
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            let Some(attachable) = find_attachable(ecd, pools) else {
                continue;
            };
            let Some(item_def) = attachable.attach_def.and_then(|h| h.get(pools)) else {
                continue;
            };

            let loc = item_def.localization.and_then(|h| h.get(pools));
            cache.by_record.insert(
                guid,
                Item {
                    name_key: loc.and_then(|l| non_empty(&l.name)),
                    short_name_key: loc.and_then(|l| non_empty(&l.short_name)),
                    desc_key: loc.and_then(|l| non_empty(&l.description)),
                    item_type: item_def.r#type.clone(),
                    item_sub_type: item_def.sub_type.clone(),
                },
            );
        }

        cache
    }

    /// Insert or replace a record's entry.
    pub fn insert(&mut self, guid: Guid, item: Item) {
        self.by_record.insert(guid, item);
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
    use super::*;

    fn item(t: EItemType) -> Item {
        Item {
            name_key: None,
            short_name_key: None,
            desc_key: None,
            item_type: t,
            item_sub_type: EItemSubType::Unrecognized(String::new()),
        }
    }

    #[test]
    fn cache_starts_empty() {
        let cache = ItemCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn insert_and_get_round_trip() {
        let mut cache = ItemCache::new();
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
    fn keys_keep_at_prefix() {
        let mut it = item(EItemType::Armor);
        it.name_key = Some(LocaleKey::new("@item_NameRaw"));
        assert!(it.name_key.unwrap().as_str().starts_with('@'));
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
