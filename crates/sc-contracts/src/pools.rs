//! Precomputed mission groupings.
//!
//! [`MissionPools`] is built once at [`crate::MissionIndex`]
//! construction and exposed as a public field on the index. Each
//! grouping axis is a `HashMap<key, Vec<Guid>>` of mission IDs
//! sharing that key.
//!
//! Divergence — *what* differs across a pool's members — lives as
//! opt-in helper methods on [`crate::MissionIndex`] (see the
//! `*_mixed` / `*_consistent` accessors). Consumers call only what
//! they read.
//!
//! Replaces v1's cluster API + `find_bp_conflicts` helper, which
//! went away with the merge step in phase 4 of the v2 redesign.

use std::collections::HashMap;

use sc_extract::{Guid, LocaleKey};

use crate::expand::Mission;

/// Precomputed groupings keyed off the most common consumer axes.
///
/// Each value is a `Vec<Guid>` of [`Mission`] ids — look up the
/// actual row via [`crate::MissionIndex::get`].
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
    /// Build all grouping axes from the contract list. O(n) per axis.
    /// Cheap: ~4,590 expansion rows × 2 keys yields ~9k HashMap inserts.
    pub fn build(contracts: &[Mission]) -> Self {
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
// `MissionIndex`.

/// True if two `ScripReward` slices represent the same multiset of
/// (currency, amount) pairs in the same order.
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

/// Conservative encounter-shape comparison: matches when the two
/// encounter lists have the same length, each pair shares the same
/// kind (Ships / Npcs / Entities / Unknown) + same `variable_name` +
/// same phase count + same per-phase name + same per-phase slot
/// count. Doesn't dive into per-slot tag comparison — real-world
/// members that share a title but differ on encounter spawns are
/// exactly the case the test wants to flag.
pub(crate) fn encounters_shape_eq(
    a: &[crate::expand::Encounter],
    b: &[crate::expand::Encounter],
) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(x, y)| match (x, y) {
        (crate::expand::Encounter::Ships(xs), crate::expand::Encounter::Ships(ys)) => {
            xs.variable_name == ys.variable_name
                && phases_eq(&xs.phases, &ys.phases, |s1, s2| s1.len() == s2.len())
        }
        (crate::expand::Encounter::Npcs(xs), crate::expand::Encounter::Npcs(ys)) => {
            xs.variable_name == ys.variable_name
                && phases_eq(&xs.phases, &ys.phases, |s1, s2| s1.len() == s2.len())
        }
        (crate::expand::Encounter::Entities(xs), crate::expand::Encounter::Entities(ys)) => {
            xs.variable_name == ys.variable_name
                && phases_eq(&xs.phases, &ys.phases, |s1, s2| s1.len() == s2.len())
        }
        (
            crate::expand::Encounter::Unknown { variable_name: a, .. },
            crate::expand::Encounter::Unknown { variable_name: b, .. },
        ) => a == b,
        _ => false,
    })
}

fn phases_eq<S>(
    a: &[crate::expand::EncounterPhase<S>],
    b: &[crate::expand::EncounterPhase<S>],
    slots_eq: impl Fn(&[S], &[S]) -> bool,
) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b.iter())
            .all(|(x, y)| x.name == y.name && slots_eq(&x.slots, &y.slots))
}
