//! Locale-key clustering — group merged [`Contract`]s by their
//! `title_key` or `description_key` and report which fields diverge
//! across cluster members.
//!
//! Why this lives here, in plain English: any tool that *patches the
//! INI* (sc-langpatch is the driving consumer) writes its annotation
//! into a single locale key that's shared by every variation pointing
//! at it. A title tag like `[BP]` is only safe to add to a key when
//! every member of that key's cluster actually has a blueprint reward;
//! otherwise the patch lies for some variants. The information block
//! sc-langpatch appends to a description has the same constraint, but
//! per-axis (rewards / region / encounters / …): each line of the
//! block needs to be uniform across everyone resolving that
//! description key, or it's misinformation.
//!
//! [`cluster_by_title_key`] / [`cluster_by_description_key`] return
//! the membership; [`ClusterDivergence`] tells the consumer which
//! axes are uniform vs mixed. The divergence struct carries both
//! title-relevant and description-relevant axes — the consumer reads
//! whichever half matches what they're patching.

use std::collections::{HashMap, HashSet};

use sc_extract::{Guid, LocaleKey};

use crate::merge::Contract;

/// One locale-key cluster — every [`Contract`] sharing the cluster
/// key, plus per-axis divergence flags computed across the members.
#[derive(Debug, Clone)]
pub struct KeyCluster<'a> {
    /// The shared `title_key` or `description_key` (depending on which
    /// grouping fn produced the cluster).
    pub key: &'a LocaleKey,
    /// Resolved text for the key, picked from the first member that
    /// has it. `None` when no member resolved (e.g. locale missing the
    /// key).
    pub resolved_text: Option<&'a str>,
    /// Contracts sharing `key`. Length ≥ 1; single-member clusters are
    /// included so the consumer can decide whether to filter.
    pub members: Vec<&'a Contract>,
    /// Which fields diverge across `members`. Empty (`Default`) when
    /// the cluster has one member or every axis is uniform.
    pub divergence: ClusterDivergence,
}

/// Per-axis divergence flags across cluster members. Each `true`
/// means at least two members disagree on the named field.
///
/// Axes are split by which key they inform. A title-cluster consumer
/// reads the title-relevant half to decide tag safety; a description-
/// cluster consumer reads the description-relevant half to decide
/// info-block safety. Both halves are populated regardless of which
/// grouping produced the cluster — it's up to the caller which to use.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClusterDivergence {
    // ── Title-relevant: a `true` means a binary tag is unsafe ──────
    /// At least one member has a blueprint reward and at least one
    /// doesn't. → `[BP]` needs the mixed marker (`[BP*]`).
    pub blueprint_mixed: bool,
    /// `availability.once_only` not uniform → `[Uniq]` is unsafe.
    pub once_only_mixed: bool,
    /// `shareable` not uniform → `[Solo]` (inverse) is unsafe.
    pub shareable_mixed: bool,
    /// `illegal_flag` not uniform.
    pub illegal_mixed: bool,

    // ── Description-relevant: a `true` means an info-block line is unsafe ──
    /// `reward_uec` (None / Calculated / Fixed(n)) not uniform.
    pub reward_uec: bool,
    /// Scrip rewards differ in count, currency GUID, or amount.
    pub reward_scrip: bool,
    /// Reputation rewards differ.
    pub reward_rep: bool,
    /// Among members that have a blueprint reward, the pool GUIDs
    /// differ. Implies the BP block can't name a single pool.
    pub blueprint_pool: bool,
    /// `mission_span` (Vec<Guid> of localities) differs as a set.
    pub mission_span: bool,
    /// Encounter group count or shape differs. v1 is conservative —
    /// flagged true if anything looks different.
    pub encounters: bool,
    /// `availability.cooldowns` not uniform.
    pub cooldowns: bool,

    // ── Cross-cutting ─────────────────────────────────────────────
    /// At least one member's title or description carries a
    /// `~mission(...)` runtime-substitution marker. Doesn't itself
    /// imply tag/block unsafety, but is essential context (the engine
    /// fills the marker per-spawn, so static analysis is incomplete).
    pub has_runtime_substitution: bool,
    /// `handler_kind` not uniform across members. Rare in real data
    /// but legal — flagged because patches that key off handler shape
    /// (e.g. Career-only annotations) become misinformation.
    pub handler_kind_mixed: bool,
}

impl ClusterDivergence {
    /// True if any title-relevant axis diverges. Drives `[BP*]` /
    /// tag-omission rules in title patchers.
    pub fn any_title(&self) -> bool {
        self.blueprint_mixed || self.once_only_mixed || self.shareable_mixed || self.illegal_mixed
    }

    /// True if any description-relevant axis diverges. Drives
    /// "varies by variant" / line-omission rules in description
    /// patchers.
    pub fn any_description(&self) -> bool {
        self.reward_uec
            || self.reward_scrip
            || self.reward_rep
            || self.blueprint_pool
            || self.mission_span
            || self.encounters
            || self.cooldowns
    }

    /// True if any axis (excluding the runtime-substitution context
    /// flag) diverges.
    pub fn any(&self) -> bool {
        self.any_title() || self.any_description() || self.handler_kind_mixed
    }
}

// ── Entry points ────────────────────────────────────────────────────────────

/// Group `contracts` by `title_key`. Contracts with no `title_key`
/// are skipped. Output ordering: clusters with any divergence first,
/// then by member count descending, then by key alphabetical.
///
/// **Deprecated:** Phase 3 of the v2 redesign. Use
/// `index.pools.title_key` for the grouping and the divergence helpers
/// on [`crate::ContractIndex`] (`blueprint_mixed`,
/// `rewards_uec_consistent`, …) for the per-axis flags. The cluster
/// API is kept working until consumers migrate; it will go away in a
/// later release.
#[deprecated(
    note = "use ContractIndex.pools.title_key + divergence methods (blueprint_mixed, rewards_uec_consistent, ...)"
)]
pub fn cluster_by_title_key(contracts: &[Contract]) -> Vec<KeyCluster<'_>> {
    cluster_by(
        contracts,
        |c| c.title_key.as_ref(),
        |c| c.title.as_deref(),
    )
}

/// Group `contracts` by `description_key`. Same shape as
/// [`cluster_by_title_key`]; contracts without a description key are
/// skipped.
///
/// **Deprecated:** see [`cluster_by_title_key`].
#[deprecated(
    note = "use ContractIndex.pools.description_key + divergence methods (blueprint_mixed, rewards_uec_consistent, ...)"
)]
pub fn cluster_by_description_key(contracts: &[Contract]) -> Vec<KeyCluster<'_>> {
    cluster_by(
        contracts,
        |c| c.description_key.as_ref(),
        |c| c.description.as_deref(),
    )
}

fn cluster_by<'a, FK, FT>(
    contracts: &'a [Contract],
    get_key: FK,
    get_text: FT,
) -> Vec<KeyCluster<'a>>
where
    FK: Fn(&Contract) -> Option<&LocaleKey>,
    FT: Fn(&Contract) -> Option<&str>,
{
    let mut groups: HashMap<&LocaleKey, Vec<&Contract>> = HashMap::new();
    for c in contracts {
        if let Some(k) = get_key(c) {
            groups.entry(k).or_default().push(c);
        }
    }

    let mut clusters: Vec<KeyCluster<'_>> = groups
        .into_iter()
        .map(|(key, members)| {
            let resolved_text = members.iter().find_map(|c| get_text(c));
            let divergence = compute_divergence(&members);
            KeyCluster {
                key,
                resolved_text,
                members,
                divergence,
            }
        })
        .collect();

    clusters.sort_by(|a, b| {
        // Divergent clusters first (most actionable).
        b.divergence
            .any()
            .cmp(&a.divergence.any())
            .then_with(|| b.members.len().cmp(&a.members.len()))
            .then_with(|| a.key.as_str().cmp(b.key.as_str()))
    });
    clusters
}

// ── Divergence computation ──────────────────────────────────────────────────

fn compute_divergence(members: &[&Contract]) -> ClusterDivergence {
    let mut d = ClusterDivergence::default();
    if members.is_empty() {
        return d;
    }
    if members.len() == 1 {
        d.has_runtime_substitution = members[0].has_runtime_substitution;
        return d;
    }

    let first = members[0];
    let mut bp_present = 0usize;
    let mut bp_pools: HashSet<Guid> = HashSet::new();

    for &c in members {
        if c.has_runtime_substitution {
            d.has_runtime_substitution = true;
        }
        if c.handler_kind != first.handler_kind {
            d.handler_kind_mixed = true;
        }
        if c.availability.once_only != first.availability.once_only {
            d.once_only_mixed = true;
        }
        if c.shareable != first.shareable {
            d.shareable_mixed = true;
        }
        if c.illegal_flag != first.illegal_flag {
            d.illegal_mixed = true;
        }
        if c.rewards.uec != first.rewards.uec {
            d.reward_uec = true;
        }
        if !scrip_eq(&c.rewards.scrip, &first.rewards.scrip) {
            d.reward_scrip = true;
        }
        if c.rewards.reputation != first.rewards.reputation {
            d.reward_rep = true;
        }
        if !guid_set_eq(&c.mission_span, &first.mission_span) {
            d.mission_span = true;
        }
        if !encounters_shape_eq(&c.encounters, &first.encounters) {
            d.encounters = true;
        }
        if c.availability.cooldowns != first.availability.cooldowns {
            d.cooldowns = true;
        }

        if let Some(bp) = &c.rewards.blueprint {
            bp_present += 1;
            bp_pools.insert(bp.pool_guid);
        }
    }

    if bp_present > 0 && bp_present < members.len() {
        d.blueprint_mixed = true;
    }
    if bp_pools.len() > 1 {
        d.blueprint_pool = true;
    }

    d
}

// Comparator helpers live in pools.rs so the divergence methods on
// ContractIndex share one implementation.
use crate::pools::{encounters_shape_eq, guid_set_eq, scrip_eq};
