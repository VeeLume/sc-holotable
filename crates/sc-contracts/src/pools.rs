//! Precomputed mission groupings.
//!
//! [`MissionPools`] holds the same data v1 derived through
//! [`crate::cluster_by_title_key`] / [`crate::cluster_by_description_key`]
//! but as direct fields on [`crate::ContractIndex`] — built once at
//! index construction, read by direct map lookup. Each grouping axis
//! is a `HashMap<key, Vec<Guid>>` of contract IDs sharing that key.
//!
//! Divergence — *what* differs across a pool's members — lives as
//! opt-in helper methods on [`crate::ContractIndex`] (see the
//! `*_mixed` / `*_consistent` accessors). Consumers call only what
//! they read; the bag-of-bools `ClusterDivergence` shape is not
//! computed eagerly.
//!
//! # Phasing
//!
//! v1's `find_bp_conflicts` becomes a one-liner over `pools.title_key
//! + index.blueprint_mixed`. The cluster API stays available
//! (deprecated) until consumers migrate.

use std::collections::HashMap;

use sc_extract::{Guid, LocaleKey};

use crate::merge::Contract;

/// Precomputed groupings keyed off the most common consumer axes.
///
/// Each value is a `Vec<Guid>` of canonical [`Contract`] ids — look up
/// the actual contract via [`crate::ContractIndex::get`].
///
/// More axes are non-breaking additions: a future `by_locality` field
/// can land alongside without changing existing fields.
#[derive(Debug, Clone, Default)]
pub struct MissionPools {
    /// Contracts grouped by `title_key`. Contracts whose title key is
    /// `None` are not included.
    pub title_key: HashMap<LocaleKey, Vec<Guid>>,
    /// Contracts grouped by `description_key`. Contracts whose
    /// description key is `None` are not included.
    pub description_key: HashMap<LocaleKey, Vec<Guid>>,
}

impl MissionPools {
    /// Build all grouping axes from the merged contract list. O(n) per
    /// axis. ~14k HashMap inserts on SC 4.7 LIVE; cheap.
    pub fn build(contracts: &[Contract]) -> Self {
        let mut pools = Self::default();
        for c in contracts {
            if let Some(key) = c.title_key.as_ref() {
                pools.title_key.entry(key.clone()).or_default().push(c.id);
            }
            if let Some(key) = c.description_key.as_ref() {
                pools
                    .description_key
                    .entry(key.clone())
                    .or_default()
                    .push(c.id);
            }
        }
        pools
    }
}

// ── Divergence comparators ─────────────────────────────────────────────────
//
// Free functions used by the `*_mixed` / `*_consistent` methods on
// `ContractIndex`. Lifted out of `clusters.rs::compute_divergence` so
// both APIs share one implementation; clusters.rs reads them too.

/// True if two `ScripReward` slices represent the same multiset of
/// (currency, amount) pairs in the same order. Order matters here
/// because the merge guarantees a single canonical order across
/// expansions sharing rewards; consumers grouping on a different axis
/// (e.g. title only) may legitimately see reordered rewards across
/// members.
pub(crate) fn scrip_eq(a: &[crate::expand::ScripReward], b: &[crate::expand::ScripReward]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .all(|(x, y)| x.currency_guid == y.currency_guid && x.amount == y.amount)
}

/// Set-equality for two GUID slices.
pub(crate) fn guid_set_eq(a: &[Guid], b: &[Guid]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let sa: std::collections::HashSet<&Guid> = a.iter().collect();
    let sb: std::collections::HashSet<&Guid> = b.iter().collect();
    sa == sb
}

/// Conservative encounter-shape comparison: matches when the two have
/// the same number of groups, and each group has the same
/// `variable_name` + same wave count + same per-wave name + same per-
/// wave slot count. Doesn't dive into per-slot tag comparison —
/// real-world members that share a title but differ on encounter
/// spawns are exactly the case the test wants to flag.
pub(crate) fn encounters_shape_eq(
    a: &[crate::expand::EncounterGroup],
    b: &[crate::expand::EncounterGroup],
) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(x, y)| {
        x.variable_name == y.variable_name
            && x.waves.len() == y.waves.len()
            && x.waves
                .iter()
                .zip(y.waves.iter())
                .all(|(wx, wy)| wx.name == wy.name && wx.slots.len() == wy.slots.len())
    })
}
