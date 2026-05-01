//! Top-level [`ContractIndex`] — the one-stop entry point for
//! consumers.
//!
//! A `ContractIndex` owns the merged contract list plus every registry
//! needed to resolve GUIDs the contracts point at. Build it once from
//! a [`sc_extract::Datacore`] + [`sc_extract::LocaleMap`]; carry it
//! around freely; look up by GUID or iterate.
//!
//! ```no_run
//! use sc_contracts::ContractIndex;
//! # fn demo(datacore: &sc_extract::Datacore, locale: &sc_extract::LocaleMap) {
//! let index = ContractIndex::build(datacore, locale);
//! println!("{} contracts ({} with mission span)",
//!     index.contracts.len(),
//!     index.contracts.iter().filter(|c| !c.mission_span.is_empty()).count(),
//! );
//! for contract in &index.contracts {
//!     // …annotate title, render rewards, resolve span via
//!     //    index.localities.get(guid), etc.
//! }
//! # }
//! ```

use std::collections::HashMap;

use sc_extract::{Datacore, Guid, LocaleMap, TagTree};

use crate::blueprints::BlueprintPoolRegistry;
use crate::currency::RewardCurrencyCatalog;
use crate::expand::expand_all;
use crate::locality::{LocalityRegistry, LocationRegistry};
use crate::merge::{Contract, merge_expansions};
use crate::pools::{self, MissionPools};
use crate::ships::ShipRegistry;

/// Bundled output of the four-stage contracts pipeline.
///
/// The registries are kept alongside the contract list because
/// several `Contract` fields hold GUIDs — `mission_span`,
/// `blueprint_reward.pool_guid`, `reward_scrip[i].currency_guid`,
/// encounter candidate GUIDs — that need one of these registries to
/// resolve into user-facing text. Consumers either keep the entire
/// `ContractIndex` or pull specific registries out at build time.
///
/// Fields are public by convention — `ContractIndex` is a plain data
/// bundle, not an opaque handle. If a consumer needs only some
/// registries, it can drop the rest after build.
#[derive(Debug, Clone)]
pub struct ContractIndex {
    /// Merged contracts, post-similarity-collapse. Stable order:
    /// first-seen among expansions (which iterate generators by GUID
    /// map order, handlers / contracts / sub-contracts in
    /// declaration order).
    pub contracts: Vec<Contract>,

    /// Ship entity lookup + spawn-query resolver. Holds every
    /// `EntityClassDefinition` reachable from a contract spawn query
    /// plus the typed classifier for non-ship tags.
    pub ships: ShipRegistry,

    /// Blueprint pool resolver. Maps `BlueprintPoolRecord` GUIDs to
    /// their items + localized item display names.
    pub blueprints: BlueprintPoolRegistry,

    /// Typed currency catalog — identifies which
    /// `EntityClassDefinition`s are scrip vs generic items.
    pub currency: RewardCurrencyCatalog,

    /// Star-map object classifier (system + body + localized names),
    /// driven by parent-chain traversal.
    pub locations: LocationRegistry,

    /// Mission-locality resolver — turns `MissionLocality` GUIDs from
    /// `Contract.mission_span` into [`crate::LocalityView`] entries
    /// with resolved locations + `region_label` summary.
    pub localities: LocalityRegistry,

    /// Tag-tree handle, cloned from the underlying `Datacore` snapshot.
    /// Required by [`crate::TagBag`] classifier methods (factions /
    /// cargo / spawn_identifiers / ai_traits / mission_tags) which
    /// walk tag paths against the live tree on demand. Holding it
    /// here means consumers don't have to thread it separately.
    pub tag_tree: TagTree,

    /// Precomputed groupings of contracts by various consumer-relevant
    /// axes — title key, description key, …. Read fields directly:
    /// `index.pools.title_key.get(&key)`. Pair with the per-axis
    /// divergence helpers ([`Self::blueprint_mixed`] etc.) for the
    /// cluster-API workflow.
    pub pools: MissionPools,

    /// Fast `Guid → index` lookup for [`Self::get`]. Built at
    /// construction; stays in sync with `contracts` because
    /// `ContractIndex` is immutable after `build`.
    by_id: HashMap<Guid, usize>,
}

impl ContractIndex {
    /// Run all four pipeline stages and return the result.
    ///
    /// On SC 4.7 LIVE this takes ~2–3s in release (dominated by
    /// generator expansion over 4,926 nodes + the ship registry's
    /// two-pass entity scan).
    ///
    /// `datacore` must have the `contracts` + `servicebeacon`
    /// features enabled — the default `sc-contracts` dependency
    /// turns both on.
    pub fn build(datacore: &Datacore, locale: &LocaleMap) -> Self {
        let ships = ShipRegistry::build(datacore);
        let blueprints = BlueprintPoolRegistry::build(datacore, locale);
        let currency = RewardCurrencyCatalog::build(datacore);
        let locations = LocationRegistry::build(datacore, locale);
        let localities = LocalityRegistry::build(datacore, &locations);

        let expansions = expand_all(
            datacore,
            locale,
            &ships,
            &blueprints,
            &currency,
            &localities,
        );
        let contracts = merge_expansions(expansions);

        let by_id = contracts
            .iter()
            .enumerate()
            .map(|(i, c)| (c.id, i))
            .collect();

        let tag_tree = datacore.snapshot().tag_tree.clone();
        let pools = MissionPools::build(&contracts);

        Self {
            contracts,
            ships,
            blueprints,
            currency,
            locations,
            localities,
            tag_tree,
            pools,
            by_id,
        }
    }

    /// Look up a contract by its canonical GUID. Returns `None` if
    /// the GUID isn't a canonical merged-contract id — note that a
    /// `Contract.id` is the parent contract's GUID, not any
    /// individual sub-contract's. Sub-contract lookup goes through
    /// [`Self::get_by_variation`].
    pub fn get(&self, id: Guid) -> Option<&Contract> {
        self.by_id.get(&id).map(|&i| &self.contracts[i])
    }

    /// Look up a contract by any variation's `expansion_id` (i.e.,
    /// by the parent Contract's GUID *or* any sub-contract GUID).
    /// Slower than [`Self::get`] — O(n_variations). Useful when
    /// consumers hold a raw DCB GUID without knowing whether it's a
    /// parent or a sub-contract.
    pub fn get_by_variation(&self, expansion_id: Guid) -> Option<&Contract> {
        if let Some(c) = self.get(expansion_id) {
            return Some(c);
        }
        self.contracts
            .iter()
            .find(|c| c.variations.iter().any(|v| v.expansion_id == expansion_id))
    }

    /// Convenience — iterate every contract in index order.
    pub fn iter(&self) -> impl Iterator<Item = &Contract> + '_ {
        self.contracts.iter()
    }

    /// Number of merged contracts in the index.
    pub fn len(&self) -> usize {
        self.contracts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty()
    }

    /// Iterate the contracts whose ids are in `ids`, in order. Skips
    /// ids that don't resolve. Use with [`MissionPools`] field values:
    /// `index.iter_pool(ids).filter(...)`.
    pub fn iter_pool<'a>(&'a self, ids: &'a [Guid]) -> impl Iterator<Item = &'a Contract> + 'a {
        ids.iter().filter_map(|id| self.get(*id))
    }

    // ── Divergence helpers ──────────────────────────────────────────────────
    //
    // Per-axis "is this consistent across the pool" predicates. Each
    // returns `true` when at least two members differ on the named
    // axis (the `_mixed` variants) or when all members agree (the
    // `_consistent` variants). Empty / single-member pools return
    // `false` for `_mixed` and `true` for `_consistent` — vacuously
    // not-divergent.

    /// True if some pool members carry a blueprint reward and others
    /// don't. Drives the `[BP*]` mixed-marker decision in title patchers.
    pub fn blueprint_mixed(&self, ids: &[Guid]) -> bool {
        let mut total = 0usize;
        let mut with_bp = 0usize;
        for c in self.iter_pool(ids) {
            total += 1;
            if c.rewards.blueprint.is_some() {
                with_bp += 1;
            }
        }
        with_bp > 0 && with_bp < total
    }

    /// True if pool members disagree on the blueprint pool GUID. (When
    /// every member carries a blueprint but the pool varies, the line
    /// can't name a single pool.)
    pub fn blueprint_pool_consistent(&self, ids: &[Guid]) -> bool {
        let mut pools = std::collections::HashSet::new();
        for c in self.iter_pool(ids) {
            if let Some(bp) = &c.rewards.blueprint {
                pools.insert(bp.pool_guid);
            }
        }
        pools.len() <= 1
    }

    /// True if all pool members agree on the UEC reward shape (None /
    /// Calculated / Fixed(n)).
    pub fn rewards_uec_consistent(&self, ids: &[Guid]) -> bool {
        all_eq(self.iter_pool(ids).map(|c| c.rewards.uec))
    }

    /// True if all pool members carry the same scrip rewards (same
    /// length, same currency_guid + amount per index).
    pub fn rewards_scrip_consistent(&self, ids: &[Guid]) -> bool {
        let mut iter = self.iter_pool(ids);
        let Some(first) = iter.next() else { return true };
        iter.all(|c| pools::scrip_eq(&c.rewards.scrip, &first.rewards.scrip))
    }

    /// True if all pool members agree on reputation rewards.
    pub fn rewards_rep_consistent(&self, ids: &[Guid]) -> bool {
        let mut iter = self.iter_pool(ids);
        let Some(first) = iter.next() else { return true };
        iter.all(|c| c.rewards.reputation == first.rewards.reputation)
    }

    /// True if all pool members agree on availability cooldowns.
    pub fn cooldowns_consistent(&self, ids: &[Guid]) -> bool {
        all_eq(self.iter_pool(ids).map(|c| c.availability.cooldowns.clone()))
    }

    /// True if pool members disagree on once_only.
    pub fn once_only_mixed(&self, ids: &[Guid]) -> bool {
        is_mixed(self.iter_pool(ids).map(|c| c.availability.once_only))
    }

    /// True if pool members disagree on shareable.
    pub fn shareable_mixed(&self, ids: &[Guid]) -> bool {
        is_mixed(self.iter_pool(ids).map(|c| c.shareable))
    }

    /// True if pool members disagree on illegal_flag.
    pub fn illegal_mixed(&self, ids: &[Guid]) -> bool {
        is_mixed(self.iter_pool(ids).map(|c| c.illegal_flag))
    }

    /// True if pool members disagree on handler_kind.
    pub fn handler_kind_mixed(&self, ids: &[Guid]) -> bool {
        is_mixed(self.iter_pool(ids).map(|c| c.handler_kind))
    }

    /// True if pool members disagree on mission_span (set-equality).
    pub fn mission_span_consistent(&self, ids: &[Guid]) -> bool {
        let mut iter = self.iter_pool(ids);
        let Some(first) = iter.next() else { return true };
        iter.all(|c| pools::guid_set_eq(&c.mission_span, &first.mission_span))
    }

    /// True if pool members agree on encounter shape (group count,
    /// per-group variable_name, per-wave name + slot count).
    pub fn encounters_shape_consistent(&self, ids: &[Guid]) -> bool {
        let mut iter = self.iter_pool(ids);
        let Some(first) = iter.next() else { return true };
        iter.all(|c| pools::encounters_shape_eq(&c.encounters, &first.encounters))
    }

    /// True if any pool member has runtime substitution markers
    /// (`~mission(...)`) in its title or description.
    pub fn has_runtime_substitution(&self, ids: &[Guid]) -> bool {
        self.iter_pool(ids).any(|c| c.has_runtime_substitution)
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// True iff every yielded value equals the first.
fn all_eq<T: PartialEq>(mut iter: impl Iterator<Item = T>) -> bool {
    let Some(first) = iter.next() else { return true };
    iter.all(|x| x == first)
}

/// True iff at least two yielded values differ.
fn is_mixed<T: PartialEq>(iter: impl Iterator<Item = T>) -> bool {
    !all_eq(iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_index_shape() {
        // Can't build without a Datacore, but can construct an empty
        // shell to exercise the accessor surface. The real build is
        // covered by the `examples/*` live validation.
        let idx = ContractIndex {
            contracts: Vec::new(),
            ships: ShipRegistry::default(),
            blueprints: BlueprintPoolRegistry::default(),
            currency: RewardCurrencyCatalog::default(),
            locations: LocationRegistry::default(),
            localities: LocalityRegistry::default(),
            tag_tree: TagTree::default(),
            pools: MissionPools::default(),
            by_id: HashMap::new(),
        };
        assert_eq!(idx.len(), 0);
        assert!(idx.is_empty());
        assert!(idx.get(Guid::default()).is_none());
        assert!(idx.get_by_variation(Guid::default()).is_none());
        assert_eq!(idx.iter().count(), 0);
    }
}
