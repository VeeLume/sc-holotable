//! Reward currency catalog.
//!
//! [`ContractResult_Item`] rewards point at `EntityClassDefinition`
//! GUIDs for both currency drops (merc scrip, council scrip, Banu
//! Wikelo favours, …) and concrete item drops (ships, weapons, armour
//! variants). The model layer distinguishes them so `Contract`
//! consumers can render `reward_scrip` separately from `reward_items`.
//!
//! Classification is **typed**, not name-based (workspace principle §5):
//! currency items are identified by walking the entity's
//! `SAttachableComponentParams.AttachDef` → `SItemDefinition` and
//! checking `type == EItemType::Currency`. The same path sc-weapons
//! uses to classify ship vs FPS weapons.

use std::collections::HashMap;

use sc_extract::generated::{
    DataForgeComponentParamsPtr, DataPools, EItemType, EntityClassDefinition, SItemDefinition,
};
use sc_extract::{Datacore, Guid, LocaleMap};

/// One resolved currency entity — scrip, favour, or other.
///
/// Display name is intentionally absent — resolve via
/// [`RewardCurrencyCatalog::display_name`] at the call site through the
/// active [`LocaleMap`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrencyInfo {
    /// GUID of the `EntityClassDefinition` root record.
    pub entity_guid: Guid,
    /// Record name stripped of the `EntityClassDefinition.` prefix —
    /// stable across patches, useful for debug / per-faction routing.
    pub record_name: String,
}

/// Lookup catalog for currency `EntityClassDefinition`s.
///
/// Built once per `Datacore`. Populated by walking every entity whose
/// `SItemDefinition.type == EItemType::Currency`. Typically small
/// (merc scrip, council scrip, Banu favours, plus any new currencies
/// CIG adds).
#[derive(Debug, Clone, Default)]
pub struct RewardCurrencyCatalog {
    by_guid: HashMap<Guid, CurrencyInfo>,
}

impl RewardCurrencyCatalog {
    /// Build by walking every `EntityClassDefinition` and retaining
    /// those whose attach-def `SItemDefinition.type` is
    /// `EItemType::Currency`.
    pub fn build(datacore: &Datacore) -> Self {
        let pools = &datacore.records().pools;
        let db = datacore.db();

        let mut by_guid: HashMap<Guid, CurrencyInfo> = HashMap::new();

        for (guid, handle) in &datacore
            .records()
            .records
            .multi_feature
            .entity_class_definition
        {
            let Some(ecd) = handle.get(pools) else {
                continue;
            };
            if !is_currency(ecd, pools) {
                continue;
            }

            let record_name = db
                .record(guid)
                .and_then(|r| r.name().map(String::from))
                .map(|n| {
                    n.strip_prefix("EntityClassDefinition.")
                        .unwrap_or(&n)
                        .to_string()
                })
                .unwrap_or_default();

            by_guid.insert(
                *guid,
                CurrencyInfo {
                    entity_guid: *guid,
                    record_name,
                },
            );
        }

        Self { by_guid }
    }

    /// Resolve the localized display name for a currency entity through
    /// the supplied [`LocaleMap`]. Returns `None` when `guid` isn't in
    /// the catalog or when the entity has no `Localization.Name` key /
    /// the key is absent from `locale`.
    pub fn display_name<'a>(
        &self,
        guid: &Guid,
        cache: &sc_extract::LocalizedItemCache,
        locale: &'a LocaleMap,
    ) -> Option<&'a str> {
        if !self.by_guid.contains_key(guid) {
            return None;
        }
        cache.name_key(guid).and_then(|k| locale.resolve(k))
    }

    /// True if `guid` points at a currency entity.
    pub fn is_currency(&self, guid: &Guid) -> bool {
        self.by_guid.contains_key(guid)
    }

    /// Fetch currency metadata for a reward-target GUID.
    pub fn get(&self, guid: &Guid) -> Option<&CurrencyInfo> {
        self.by_guid.get(guid)
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CurrencyInfo> + '_ {
        self.by_guid.values()
    }
}

/// Typed currency check — does the entity's `SItemDefinition.type`
/// equal `EItemType::Currency`? Walks the entity's components to find
/// the `SAttachableComponentParams.AttachDef` handle.
fn is_currency(ecd: &EntityClassDefinition, pools: &DataPools) -> bool {
    for comp in &ecd.components {
        if let DataForgeComponentParamsPtr::SAttachableComponentParams(h) = comp
            && let Some(attach) = h.get(pools)
            && let Some(def_h) = attach.attach_def.as_ref()
            && let Some(item_def) = def_h.get(pools)
        {
            return matches!(item_def.r#type, EItemType::Currency);
        }
    }
    false
}

// Silences dead_code if SItemDefinition is only used via the type-check path.
#[allow(dead_code)]
fn _anchor(_def: &SItemDefinition) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_catalog_reports_zero() {
        let cat = RewardCurrencyCatalog::default();
        assert_eq!(cat.len(), 0);
        assert!(cat.is_empty());
        assert!(!cat.is_currency(&Guid::default()));
        assert!(cat.get(&Guid::default()).is_none());
    }
}
