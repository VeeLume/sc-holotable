//! Crafting blueprint catalog — the typed shape of the DCB's `crafting/`
//! records.
//!
//! A `CraftingBlueprintRecord` resolves 1:1 to one crafted item.
//! [`all_blueprints`] is the **primary entry point**: the full craftable
//! catalog. [`resolve_blueprint`] resolves a single record.
//!
//! Blueprint *pools* (the weighted sets mission rewards draw from) are a
//! mission-reward mechanic, not a catalog concept — they live on the
//! missions side (`sc-contracts` / `sc-missions`), built on top of this
//! catalog.
//!
//! # Display names are baked
//!
//! Each [`BlueprintItem`] bakes the crafted entity's name key (from an
//! [`ItemCache`]) at build time, so [`BlueprintItem::display_name`] needs
//! only a [`LocaleMap`] — consumers never thread the item cache.

use sc_extract::generated::{
    CraftingBlueprint_Base_NonRefPtr, CraftingProcess_BasePtr, DataPools, RecordIndex,
};
use sc_extract::{Datacore, Guid, LocaleKey, LocaleMap};
use sc_items::ItemCache;

/// A resolved blueprint: the crafted item's identity + display-name keys.
///
/// Name keys are baked at build time (see module docs); resolve text via
/// [`BlueprintItem::display_name`]. The blueprint→item identity is 1:1.
#[derive(Debug, Clone, PartialEq)]
pub struct BlueprintItem {
    /// GUID of the `CraftingBlueprintRecord` root record.
    pub blueprint_record_guid: Guid,
    /// GUID of the `EntityClassDefinition` the blueprint crafts — the item
    /// the player receives. `None` for non-Creation processes (refining,
    /// repair) or an unresolved reference.
    pub crafted_entity_guid: Option<Guid>,
    /// Crafted entity's display-name key, baked from the [`ItemCache`] at
    /// build time. Preferred name source. Raw (`@`-prefixed).
    pub entity_name_key: Option<LocaleKey>,
    /// Fallback `CraftingBlueprint.blueprintName` key — used when the
    /// crafted-entity name doesn't resolve. Raw.
    pub blueprint_name_key: Option<LocaleKey>,
}

impl BlueprintItem {
    /// Resolve the player-facing display name through a [`LocaleMap`].
    ///
    /// Tries the baked crafted-entity name first, then the blueprint-name
    /// fallback. CIG placeholders (`<= PLACEHOLDER =>`) count as unresolved.
    /// Returns `None` when neither produces real text.
    pub fn display_name<'a>(&self, locale: &'a LocaleMap) -> Option<&'a str> {
        if let Some(key) = &self.entity_name_key
            && let Some(name) = locale.resolve(key)
            && !name.is_empty()
            && !is_placeholder(name)
        {
            return Some(name);
        }
        if let Some(key) = &self.blueprint_name_key
            && let Some(text) = locale.resolve(key)
            && !is_placeholder(text)
        {
            return Some(text);
        }
        None
    }
}

/// Every `CraftingBlueprintRecord`, resolved — the **full craftable
/// catalog** (the primary entry point). Includes default-unlocked
/// blueprints (e.g. the P4-AR) that no mission reward pool lists.
///
/// Items with no `crafted_entity_guid` (non-Creation processes /
/// unresolved) are still returned; catalog callers filter on
/// `crafted_entity_guid`. Order is unspecified.
pub fn all_blueprints(datacore: &Datacore, items: &ItemCache) -> Vec<BlueprintItem> {
    let records = &datacore.records().records;
    let pools = &datacore.records().pools;
    records
        .multi_feature
        .crafting_blueprint_record
        .keys()
        .map(|guid| resolve_record(records, pools, items, *guid))
        .collect()
}

/// Resolve a single `CraftingBlueprintRecord` GUID to a [`BlueprintItem`].
///
/// The mission-pool builder uses this to resolve the records its reward
/// entries reference. Returns a [`BlueprintItem`] with `None` fields for an
/// unresolved record rather than dropping it.
pub fn resolve_blueprint(datacore: &Datacore, items: &ItemCache, record_guid: Guid) -> BlueprintItem {
    let records = &datacore.records().records;
    let pools = &datacore.records().pools;
    resolve_record(records, pools, items, record_guid)
}

/// Inner resolver shared by [`all_blueprints`] and [`resolve_blueprint`].
fn resolve_record(
    records: &RecordIndex,
    pools: &DataPools,
    items: &ItemCache,
    record_guid: Guid,
) -> BlueprintItem {
    let mut item = BlueprintItem {
        blueprint_record_guid: record_guid,
        crafted_entity_guid: None,
        entity_name_key: None,
        blueprint_name_key: None,
    };

    let Some(bp_record) = records
        .multi_feature
        .crafting_blueprint_record
        .get(&record_guid)
        .copied()
        .and_then(|h| h.get(pools))
    else {
        return item;
    };

    let bp = match &bp_record.blueprint {
        Some(CraftingBlueprint_Base_NonRefPtr::CraftingBlueprint(h)) => h.get(pools),
        _ => None,
    };
    let Some(bp) = bp else {
        return item;
    };

    item.crafted_entity_guid = extract_creation_entity(&bp.process_specific_data, pools);
    item.entity_name_key = item
        .crafted_entity_guid
        .and_then(|g| items.name_key(&g).cloned());
    if !bp.blueprint_name.is_empty() {
        item.blueprint_name_key = Some(bp.blueprint_name.clone());
    }

    item
}

/// Pull the crafted-entity GUID from a `CraftingProcess_*` variant. Only
/// `CraftingProcess_Creation` crafts an item; other variants (Refining,
/// Repair, …) don't.
fn extract_creation_entity(process: &Option<CraftingProcess_BasePtr>, pools: &DataPools) -> Option<Guid> {
    match process.as_ref()? {
        CraftingProcess_BasePtr::CraftingProcess_Creation(h) => h.get(pools)?.entity_class,
        _ => None,
    }
}

/// CIG localization placeholder sentinels — treat as unresolved.
fn is_placeholder(text: &str) -> bool {
    text.contains("PLACEHOLDER") || text == "<= PLACEHOLDER =>"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_detection() {
        assert!(is_placeholder("<= PLACEHOLDER =>"));
        assert!(is_placeholder("xx PLACEHOLDER xx"));
        assert!(!is_placeholder("Arclight Pistol"));
    }
}
