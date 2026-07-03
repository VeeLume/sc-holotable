//! Blueprint reward pools — the mission-reward mechanic.
//!
//! A `BlueprintPoolRecord` is a weighted set of blueprints a mission reward
//! draws from (the engine grants one weighted-random blueprint per pool).
//! Pools are a *mission* concept; the blueprint catalog itself lives in
//! [`sc_crafting`]. This module is the join: it resolves each pool's reward
//! entries against [`sc_crafting::Blueprints`] and pairs them with their
//! pool weight. The pool→missions reverse index lives on
//! [`crate::Missions`].

use std::collections::HashMap;

use sc_crafting::{Blueprint, Blueprints, Process};
use sc_extract::RecordCollection;
use sc_extract::{Datacore, Guid};
use sc_items::Items;

/// One weighted entry in a [`BlueprintPool`].
#[derive(Debug, Clone)]
pub struct BlueprintPoolEntry {
    /// The blueprint this entry awards (resolved from the catalog).
    pub blueprint: Blueprint,
    /// Pick-weight within the pool. Higher = more likely. Relative.
    pub weight: f32,
}

/// A resolved `BlueprintPoolRecord` — the weighted set a mission reward
/// draws from.
#[derive(Debug, Clone)]
pub struct BlueprintPool {
    /// GUID of the `BlueprintPoolRecord` root record.
    pub guid: Guid,
    /// Record name (`BlueprintPoolRecord.` prefix stripped). Empty if none.
    pub name: String,
    /// Entries sorted descending weight, then by blueprint record GUID
    /// (locale-independent; UIs resolve names + re-sort).
    pub items: Vec<BlueprintPoolEntry>,
}

/// Lookup from `BlueprintPoolRecord.guid` to resolved [`BlueprintPool`].
#[derive(Debug, Clone, Default)]
pub struct BlueprintPools {
    pools: HashMap<Guid, BlueprintPool>,
}

impl BlueprintPools {
    /// Build from a [`Datacore`] + [`Items`]: resolve the catalog once
    /// via [`sc_crafting::Blueprints::build`], then join each pool's
    /// reward entries against it (attaching the pool weight).
    pub fn build(datacore: &Datacore, items: &Items) -> Self {
        let catalog = Blueprints::build(datacore, items);

        let pools_data = &datacore.records().pools;
        let records = &datacore.records().records;
        let db = datacore.db();
        let mut out: HashMap<Guid, BlueprintPool> = HashMap::new();

        for (pool_guid, pool_handle) in &records.multi_feature.blueprint_pool_record {
            let Some(pool) = pool_handle.get(pools_data) else {
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

            let mut entries: Vec<BlueprintPoolEntry> = Vec::new();
            for reward_handle in &pool.blueprint_rewards {
                let Some(reward) = reward_handle.get(pools_data) else {
                    continue;
                };
                // A reward with no blueprint_record awards nothing — skip.
                let Some(record_guid) = reward.blueprint_record else {
                    continue;
                };
                // Every CraftingBlueprintRecord is in the catalog; the stub
                // only guards a feature-gated/missing record.
                let blueprint = catalog.get(&record_guid).cloned().unwrap_or(Blueprint {
                    blueprint_record_guid: record_guid,
                    category: None,
                    process: Process::Other {
                        type_name: "(missing from catalog)".into(),
                        struct_index: 0,
                    },
                    blueprint_name_key: None,
                    entity_name_key: None,
                    tiers: Vec::new(),
                });
                entries.push(BlueprintPoolEntry {
                    blueprint,
                    weight: reward.weight,
                });
            }
            entries.sort_by(|a, b| {
                b.weight
                    .partial_cmp(&a.weight)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| {
                        a.blueprint
                            .blueprint_record_guid
                            .to_string()
                            .cmp(&b.blueprint.blueprint_record_guid.to_string())
                    })
            });

            out.insert(
                *pool_guid,
                BlueprintPool {
                    guid: *pool_guid,
                    name,
                    items: entries,
                },
            );
        }

        Self { pools: out }
    }

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

    /// Every pool containing the given `CraftingBlueprintRecord` GUID.
    /// O(pools × items) — cheap at 4.x-LIVE scale.
    pub fn pools_containing_item(&self, blueprint_record_guid: &Guid) -> Vec<&BlueprintPool> {
        self.pools
            .values()
            .filter(|pool| {
                pool.items
                    .iter()
                    .any(|e| e.blueprint.blueprint_record_guid == *blueprint_record_guid)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_registry() {
        let reg = BlueprintPools::default();
        assert_eq!(reg.len(), 0);
        assert!(reg.is_empty());
        assert!(reg.get(&Guid::default()).is_none());
        assert!(reg.pools_containing_item(&Guid::default()).is_empty());
    }
}
