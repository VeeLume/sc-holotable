//! Crafting blueprint catalog — the typed shape of the DCB's `crafting/`
//! records.
//!
//! A `CraftingBlueprintRecord` resolves to one crafted item (a 1:1
//! blueprint→item identity). [`BlueprintItem`] is the resolved view;
//! [`all_blueprints`] is the **full craftable catalog**; [`BlueprintPoolRegistry`]
//! groups blueprints by the `BlueprintPoolRecord`s that mission rewards
//! draw from.
//!
//! Moved out of `sc-contracts` (it's crafting-domain data, not missions).
//! The **mission↔pool reverse index** (which missions award which pool)
//! stays on the missions side — it's a join over `Mission`, not a property
//! of the catalog. This crate is pool/catalog only.
//!
//! # Display names are baked
//!
//! Each [`BlueprintItem`] bakes the crafted entity's name key (from an
//! [`ItemCache`]) at build time, so [`BlueprintItem::display_name`] needs
//! only a [`LocaleMap`] — consumers never thread the item cache.

use std::collections::HashMap;

use sc_extract::generated::{
    BlueprintReward, CraftingBlueprint_Base_NonRefPtr, CraftingProcess_BasePtr, DataPools,
    RecordIndex,
};
use sc_extract::{Datacore, Guid, LocaleKey, LocaleMap};
use sc_items::ItemCache;

/// A resolved blueprint: the crafted item's identity + display-name keys.
///
/// Name keys are baked at build time (see module docs); resolve text via
/// [`BlueprintItem::display_name`].
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
    /// Pick-weight within a pool (relative). 1.0 for pool-independent
    /// catalog entries from [`all_blueprints`].
    pub weight: f32,
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

/// A resolved `BlueprintPoolRecord` — the weighted set of blueprints a
/// mission reward draws from.
#[derive(Debug, Clone)]
pub struct BlueprintPool {
    /// GUID of the `BlueprintPoolRecord` root record.
    pub guid: Guid,
    /// Record name (`BlueprintPoolRecord.` prefix stripped). Empty if none.
    pub name: String,
    /// Items in the pool, sorted descending weight then by record GUID
    /// (locale-independent; UIs resolve names and re-sort).
    pub items: Vec<BlueprintItem>,
}

/// Lookup from `BlueprintPoolRecord.guid` to resolved [`BlueprintPool`].
///
/// Catalog only — the mission↔pool reverse index lives on the missions
/// side (it's a join over `Mission`).
#[derive(Debug, Clone, Default)]
pub struct BlueprintPoolRegistry {
    pools: HashMap<Guid, BlueprintPool>,
    unresolved_blueprint_records: usize,
}

impl BlueprintPoolRegistry {
    /// Build the registry from a [`Datacore`] + an [`ItemCache`] (used to
    /// bake crafted-entity names). Build the cache once and share it.
    pub fn build(datacore: &Datacore, items: &ItemCache) -> Self {
        let pools = &datacore.records().pools;
        let records = &datacore.records().records;
        let db = datacore.db();

        let mut out: HashMap<Guid, BlueprintPool> = HashMap::new();
        let mut unresolved_blueprint_records = 0usize;

        for (pool_guid, pool_handle) in &records.multi_feature.blueprint_pool_record {
            let Some(pool) = pool_handle.get(pools) else {
                continue;
            };
            let name = db
                .record(pool_guid)
                .and_then(|r| r.name().map(|s| s.to_string()))
                .map(|n| {
                    n.strip_prefix("BlueprintPoolRecord.")
                        .unwrap_or(&n)
                        .to_string()
                })
                .unwrap_or_default();

            let mut bp_items: Vec<BlueprintItem> = Vec::new();
            for reward_handle in &pool.blueprint_rewards {
                let Some(reward) = reward_handle.get(pools) else {
                    continue;
                };
                bp_items.push(resolve_blueprint_reward(
                    records,
                    pools,
                    items,
                    reward,
                    &mut unresolved_blueprint_records,
                ));
            }

            bp_items.sort_by(|a, b| {
                b.weight
                    .partial_cmp(&a.weight)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| {
                        a.blueprint_record_guid
                            .to_string()
                            .cmp(&b.blueprint_record_guid.to_string())
                    })
            });

            out.insert(
                *pool_guid,
                BlueprintPool {
                    guid: *pool_guid,
                    name,
                    items: bp_items,
                },
            );
        }

        Self {
            pools: out,
            unresolved_blueprint_records,
        }
    }

    /// Number of pools.
    pub fn len(&self) -> usize {
        self.pools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pools.is_empty()
    }

    /// Look up a pool by its `BlueprintPoolRecord` GUID.
    pub fn get(&self, guid: &Guid) -> Option<&BlueprintPool> {
        self.pools.get(guid)
    }

    pub fn iter(&self) -> impl Iterator<Item = &BlueprintPool> + '_ {
        self.pools.values()
    }

    /// Blueprint records that couldn't be resolved to a concrete
    /// `CraftingBlueprintRecord` (missing record or `Unknown` pointer).
    pub fn unresolved_blueprint_records(&self) -> usize {
        self.unresolved_blueprint_records
    }

    /// Every pool containing the given `CraftingBlueprintRecord` GUID. A
    /// blueprint can appear in multiple pools. O(pools × items) — cheap at
    /// 4.7-LIVE scale; cache if called in a hot loop.
    pub fn pools_containing_item(&self, blueprint_record_guid: &Guid) -> Vec<&BlueprintPool> {
        self.pools
            .values()
            .filter(|pool| {
                pool.items
                    .iter()
                    .any(|item| item.blueprint_record_guid == *blueprint_record_guid)
            })
            .collect()
    }
}

/// Resolve a pool's [`BlueprintReward`] entry to a [`BlueprintItem`].
fn resolve_blueprint_reward(
    records: &RecordIndex,
    pools: &DataPools,
    items: &ItemCache,
    reward: &BlueprintReward,
    unresolved: &mut usize,
) -> BlueprintItem {
    let Some(record_guid) = reward.blueprint_record else {
        *unresolved += 1;
        return BlueprintItem {
            blueprint_record_guid: Default::default(),
            crafted_entity_guid: None,
            entity_name_key: None,
            blueprint_name_key: None,
            weight: reward.weight,
        };
    };
    resolve_blueprint_record(records, pools, items, record_guid, reward.weight, unresolved)
}

/// Resolve a single `CraftingBlueprintRecord` GUID to a [`BlueprintItem`],
/// pool-independent. Bakes the crafted entity's name key from `items`.
fn resolve_blueprint_record(
    records: &RecordIndex,
    pools: &DataPools,
    items: &ItemCache,
    record_guid: Guid,
    weight: f32,
    unresolved: &mut usize,
) -> BlueprintItem {
    let mut item = BlueprintItem {
        blueprint_record_guid: record_guid,
        crafted_entity_guid: None,
        entity_name_key: None,
        blueprint_name_key: None,
        weight,
    };

    let Some(bp_record_handle) = records
        .multi_feature
        .crafting_blueprint_record
        .get(&record_guid)
        .copied()
    else {
        *unresolved += 1;
        return item;
    };
    let Some(bp_record) = bp_record_handle.get(pools) else {
        *unresolved += 1;
        return item;
    };

    let bp = match &bp_record.blueprint {
        Some(CraftingBlueprint_Base_NonRefPtr::CraftingBlueprint(h)) => h.get(pools),
        _ => None,
    };
    let Some(bp) = bp else {
        *unresolved += 1;
        return item;
    };

    item.crafted_entity_guid = extract_creation_entity(&bp.process_specific_data, pools);
    // Bake the crafted entity's display-name key from the item cache.
    item.entity_name_key = item
        .crafted_entity_guid
        .and_then(|g| items.name_key(&g).cloned());

    if !bp.blueprint_name.is_empty() {
        item.blueprint_name_key = Some(bp.blueprint_name.clone());
    }

    item
}

/// Every `CraftingBlueprintRecord`, resolved — the **full craftable
/// catalog** (pool-independent). Includes default-unlocked blueprints
/// (e.g. the P4-AR) that no mission reward pool lists.
///
/// `weight` is 1.0 for every item. Items with no `crafted_entity_guid`
/// (non-Creation processes / unresolved) are still returned; catalog
/// callers filter on `crafted_entity_guid`. Order unspecified.
pub fn all_blueprints(datacore: &Datacore, items: &ItemCache) -> Vec<BlueprintItem> {
    let records = &datacore.records().records;
    let pools = &datacore.records().pools;
    let mut unresolved = 0usize;
    records
        .multi_feature
        .crafting_blueprint_record
        .keys()
        .map(|guid| resolve_blueprint_record(records, pools, items, *guid, 1.0, &mut unresolved))
        .collect()
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
    fn empty_registry() {
        let reg = BlueprintPoolRegistry::default();
        assert_eq!(reg.len(), 0);
        assert!(reg.is_empty());
        assert!(reg.get(&Guid::default()).is_none());
        assert!(reg.pools_containing_item(&Guid::default()).is_empty());
    }

    #[test]
    fn placeholder_detection() {
        assert!(is_placeholder("<= PLACEHOLDER =>"));
        assert!(is_placeholder("xx PLACEHOLDER xx"));
        assert!(!is_placeholder("Arclight Pistol"));
    }
}
