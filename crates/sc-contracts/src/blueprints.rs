//! Blueprint pool registry — resolves `BlueprintPoolRecord` IDs into
//! sets of craftable items with display names.
//!
//! Contracts that reward blueprints hang a `BlueprintRewards { chance,
//! blueprint_pool }` entry off their `contractResults`. `blueprint_pool`
//! is a reference to a root `BlueprintPoolRecord`, whose
//! `blueprint_rewards[]` array names [`CraftingBlueprintRecord`]s.
//! Each record ultimately resolves to a localized blueprint name
//! (`CraftingBlueprint.blueprintName`) — the displayable text for
//! "these are the blueprints this mission can award."
//!
//! The registry is the shared resolver: walk once, materialise
//! `BlueprintPool` entries, and let consumers look up by pool GUID
//! when building `Contract.reward_blueprints`.
//!
//! # Not (yet) resolved
//!
//! - **Turn-in / dropoff cargo requirements.** The German community's
//!   mission annotator surfaces per-tier cargo requirements ("Abgabe für
//!   43.750 aUEC: Torite 8 SCU"). That data lives in `HaulingOrder_*`
//!   types, not `BlueprintPoolRecord`, and is a separate domain.
//!   Surface later when the hauling model lands.
//! - **Region / system restrictions.** Likewise live on the contract,
//!   not on the pool. Will land when prerequisites get resolved in step 3.

use std::collections::HashMap;

use sc_extract::generated::{
    BlueprintReward, CraftingBlueprint_Base_NonRefPtr, CraftingProcess_BasePtr, DataPools,
    RecordIndex,
};
use sc_extract::svarog_datacore::DataCoreDatabase;
use sc_extract::{Datacore, Guid, LocaleKey, LocaleMap, LocalizedItemCache};

/// A resolved blueprint item — what a contract can award.
///
/// Display name is intentionally absent — resolve via
/// [`BlueprintItem::display_name`] at the call site through the active
/// [`LocaleMap`].
#[derive(Debug, Clone, PartialEq)]
pub struct BlueprintItem {
    /// GUID of the `CraftingBlueprintRecord` root record.
    pub blueprint_record_guid: Guid,
    /// GUID of the `EntityClassDefinition` that the blueprint crafts —
    /// the actual item (weapon, attachment, …) the player receives.
    /// `None` when the blueprint doesn't resolve through
    /// `CraftingProcess_Creation.entity_class` (non-creation processes
    /// like refining, or a missing reference).
    pub crafted_entity_guid: Option<Guid>,
    /// Fallback `CraftingBlueprint.blueprintName` locale key, used when
    /// the crafted-entity path doesn't resolve. Raw — leading `@`
    /// preserved.
    pub blueprint_name_key: Option<LocaleKey>,
    /// Pick-weight within the pool. Higher = more likely. Engine-side
    /// chance is per-pool; per-item weight is relative.
    pub weight: f32,
}

impl BlueprintItem {
    /// Resolve the player-facing display name through a
    /// [`LocalizedItemCache`] (for the crafted-entity path) and a
    /// [`LocaleMap`] (for the fallback `blueprintName` key).
    ///
    /// Tries two sources in order:
    ///
    /// 1. **Crafted entity's display name** (preferred). Looks up
    ///    `cache.name_key(crafted_entity_guid)` and resolves through
    ///    `locale`.
    /// 2. **`CraftingBlueprint.blueprintName`** (fallback). Resolves the
    ///    stored [`Self::blueprint_name_key`] through `locale`.
    ///
    /// CIG localization placeholders (`<= PLACEHOLDER =>` etc.) are
    /// treated as unresolved so the caller falls through to the next
    /// source. Returns `None` when neither path produces real text.
    pub fn display_name<'a>(
        &self,
        cache: &LocalizedItemCache,
        locale: &'a LocaleMap,
    ) -> Option<&'a str> {
        if let Some(entity_guid) = self.crafted_entity_guid
            && let Some(key) = cache.name_key(&entity_guid)
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

/// A resolved `BlueprintPoolRecord`.
#[derive(Debug, Clone)]
pub struct BlueprintPool {
    /// GUID of the `BlueprintPoolRecord` root record.
    pub guid: Guid,
    /// Record name (`BlueprintPoolRecord.foo`, stripped prefix), useful
    /// for debug / census output. Empty when the record has no name.
    pub name: String,
    /// Items in the pool with their weights. Order is locale-independent
    /// (descending weight, then `blueprint_record_guid`); UIs that want
    /// alphabetical order resolve display names and re-sort.
    pub items: Vec<BlueprintItem>,
}

/// Lookup from `BlueprintPoolRecord.guid` to resolved [`BlueprintPool`].
#[derive(Debug, Clone, Default)]
pub struct BlueprintPoolRegistry {
    pools: HashMap<Guid, BlueprintPool>,
    /// Running count of unresolved blueprint records — diagnostic for
    /// when the `CraftingBlueprintRecord.blueprint` pointer doesn't
    /// point at anything we can resolve (feature-gated types or DCB
    /// breakage).
    unresolved_blueprint_records: usize,
}

impl BlueprintPoolRegistry {
    /// Build the registry from the current [`Datacore`] and a
    /// [`LocaleMap`] used to resolve blueprint names.
    ///
    /// Walks every root `BlueprintPoolRecord` (as seen by the
    /// generator's `RecordIndex`). For each pool:
    ///
    /// 1. Resolve the record's display name via the raw svarog record
    ///    (`BlueprintPoolRecord.<name>`).
    /// 2. For each `BlueprintReward` entry, follow
    ///    `blueprint_record` → `CraftingBlueprintRecord.blueprint` →
    ///    `CraftingBlueprint.blueprintName` → `LocaleMap`.
    ///
    /// Unresolvable entries (empty locale key, feature-gated record
    /// type) stay in the pool with `display_name = ""` — we never
    /// silently drop them, and counters are exposed via
    /// [`Self::unresolved_blueprint_records`] and
    /// [`Self::missing_locale_names`] for diagnostics.
    /// Build the registry from the current [`Datacore`].
    ///
    /// Walks every root `BlueprintPoolRecord` (as seen by the
    /// generator's `RecordIndex`). For each pool:
    ///
    /// 1. Resolve the record's name via the raw svarog record
    ///    (`BlueprintPoolRecord.<name>`).
    /// 2. For each `BlueprintReward` entry, capture
    ///    `crafted_entity_guid` and the fallback `blueprint_name_key`
    ///    so callers can resolve text at render time.
    ///
    /// Locale-independent — display-name resolution happens at the
    /// call site via [`BlueprintItem::display_name`].
    pub fn build(datacore: &Datacore) -> Self {
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

            let mut items: Vec<BlueprintItem> = Vec::new();
            for reward_handle in &pool.blueprint_rewards {
                let Some(reward) = reward_handle.get(pools) else {
                    continue;
                };
                let resolved = resolve_blueprint_reward(
                    records,
                    pools,
                    reward,
                    &mut unresolved_blueprint_records,
                );
                items.push(resolved);
            }

            // Stable, locale-independent order: descending weight (most
            // likely first), then by blueprint-record GUID. UIs that
            // want alphabetical order resolve display names and re-sort
            // post-hoc.
            items.sort_by(|a, b| {
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
                    items,
                },
            );
        }

        Self {
            pools: out,
            unresolved_blueprint_records,
        }
    }

    /// Number of pools in the registry.
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

    /// Total number of blueprint records we couldn't resolve to a
    /// concrete `CraftingBlueprintRecord` (either the record was
    /// missing, or its nested `blueprint` pointer landed on `Unknown`).
    pub fn unresolved_blueprint_records(&self) -> usize {
        self.unresolved_blueprint_records
    }
}

/// Walk `BlueprintReward → CraftingBlueprintRecord → CraftingBlueprint
/// → CraftingProcess_Creation.entity_class → DisplayNameCache` to
/// produce a fully resolved [`BlueprintItem`].
///
/// Two display-name sources are tried in order:
///
/// 1. **Crafted entity's display name** (preferred). The crafted item's
///    `EntityClassDefinition` has the player-facing name that appears
///    in the inventory / shop (`"Arclight Pistol"`,
///    `"Prism Laser Shotgun"`). Resolved via
///    [`sc_extract::DisplayNameCache`] which is already built on the
///    snapshot.
/// 2. **`CraftingBlueprint.blueprintName`** (fallback). Sometimes a
///    useful label when the crafted-entity path doesn't resolve, but
///    these keys are frequently CIG localization placeholders
///    (`<= PLACEHOLDER =>`) so we only use it when the primary path
///    fails and the text isn't a known placeholder.
fn resolve_blueprint_reward(
    records: &RecordIndex,
    pools: &DataPools,
    reward: &BlueprintReward,
    unresolved: &mut usize,
) -> BlueprintItem {
    let mut item = BlueprintItem {
        blueprint_record_guid: reward.blueprint_record.unwrap_or_default(),
        crafted_entity_guid: None,
        blueprint_name_key: None,
        weight: reward.weight,
    };

    let Some(record_guid) = reward.blueprint_record else {
        *unresolved += 1;
        return item;
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

    if !bp.blueprint_name.is_empty() {
        item.blueprint_name_key = Some(bp.blueprint_name.clone());
    }

    item
}

/// Pull the crafted-entity GUID out of a `CraftingProcess_*` variant.
/// Only `CraftingProcess_Creation` is meaningful for contract
/// blueprint rewards — other variants (Refining, Repair, Upgrade,
/// Dismantle) describe different crafting workflows and don't apply
/// to mission drops in 4.7 LIVE.
fn extract_creation_entity(
    process: &Option<CraftingProcess_BasePtr>,
    pools: &DataPools,
) -> Option<Guid> {
    match process.as_ref()? {
        CraftingProcess_BasePtr::CraftingProcess_Creation(h) => h.get(pools)?.entity_class,
        _ => None,
    }
}

/// CIG ships localization entries that haven't been written yet as
/// `<= PLACEHOLDER =>`. Treat those as unresolved so we can fall
/// through to a better source or leave the field empty.
fn is_placeholder(text: &str) -> bool {
    text.contains("PLACEHOLDER") || text == "<= PLACEHOLDER =>"
}

// Silence unused-import warning when feature combinations don't
// actually touch DataCoreDatabase here.
#[allow(dead_code)]
fn _unused_db_anchor(_db: &DataCoreDatabase) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_registry_reports_zero() {
        let reg = BlueprintPoolRegistry::default();
        assert_eq!(reg.len(), 0);
        assert!(reg.is_empty());
    }

    #[test]
    fn get_returns_none_on_missing_pool() {
        let reg = BlueprintPoolRegistry::default();
        assert!(reg.get(&Guid::default()).is_none());
    }
}
