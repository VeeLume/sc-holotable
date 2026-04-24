//! Contract merging — stage 4 of the pipeline.
//!
//! Groups `ExpandedContract`s by `(title, description, reward_signature)`
//! into `Contract` entries. Sub-contract tiers that share the parent's
//! text and rewards collapse into a single `Contract` with
//! `variations: Vec<Variation>`. Siblings that share title/description
//! but diverge on rewards stay separate — and get flagged via
//! `title_siblings` populated with their GUIDs, so consumers
//! annotating the shared title can compare fields and mark mixed
//! rewards as non-guaranteed. See [`find_bp_conflicts`] for the
//! blueprint-specific helper.
//!
//! Target: ~1,497 `Contract`s from ~4,590 `ExpandedContract`s (SCMDB
//! catalog size). The sub-contract duplication that's visible in raw
//! expansion output collapses naturally here.

use std::collections::{BTreeMap, HashMap, HashSet};

use sc_extract::Guid;

use crate::expand::{
    Availability, BlueprintReward, ContractOrigin, EncounterGroup, ExpandedContract, HandlerKind,
    ItemReward, OtherReward, PrereqView, RepReward, RewardAmount, ScripReward,
};

/// The merged-contract output of stage 4. Each `Contract` represents
/// one logical mission after collapsing same-text + same-reward siblings.
#[derive(Debug, Clone)]
pub struct Contract {
    /// Canonical contract id (the first expansion's id in the merge group,
    /// preferring non-sub-contract origins where available).
    pub id: Guid,
    /// Canonical debug name.
    pub debug_name: String,

    /// Owning `ContractGenerator` — same across all variations in a
    /// merge group (they must come from the same generator to share the
    /// reward signature, except for coincidence).
    pub generator_id: Guid,
    pub handler_kind: HandlerKind,
    pub handler_debug_name: String,

    /// Resolved title / description text. `None` only when no
    /// expansion in the group had text resolved — in that case the
    /// merge group is keyed purely on rewards and a synthetic
    /// `(debug_name, handler_debug_name)` tuple to prevent unrelated
    /// text-less contracts from collapsing.
    pub title: Option<String>,
    pub description: Option<String>,
    pub has_runtime_substitution: bool,

    pub shareable: bool,
    pub illegal_flag: bool,

    /// Effective availability. Taken from the first variation; the
    /// merge rule guarantees once_only / reaccept flags agree across
    /// variations that share the merge key (cooldown numbers might
    /// differ in rare cases — v1 picks the first).
    pub availability: Availability,

    /// Rewards — identical across all variations by construction.
    pub reward_uec: RewardAmount,
    pub reward_scrip: Vec<ScripReward>,
    pub reward_rep: Vec<RepReward>,
    pub reward_items: Vec<ItemReward>,
    pub reward_other: Vec<OtherReward>,
    pub blueprint_reward: Option<BlueprintReward>,

    /// Prerequisites shared by every variation. Per-variation extras
    /// live on `Variation::extra_prerequisites`.
    pub prerequisites: Vec<PrereqView>,

    /// Encounters — taken from the first variation (v1 assumes they
    /// agree within a merge group; the census shows they typically do).
    pub encounters: Vec<EncounterGroup>,

    /// The underlying `ExpandedContract`s that collapsed into this
    /// entry. Length ≥ 1. Variations typically differ only in
    /// `extra_prerequisites` (e.g. different `Locality` GUIDs per
    /// location tier).
    pub variations: Vec<Variation>,

    /// Union of every variation's `mission_span`. GUIDs point at
    /// `MissionLocality` records resolvable through
    /// [`crate::LocalityRegistry`]. Order follows first-seen across
    /// variations. Empty when no variation has a `Locality` prereq
    /// (handler-level untethered contracts, e.g. some List /
    /// LinearSeries intro flows).
    ///
    /// Use this to explain reward divergence across
    /// [`title_siblings`][Self::title_siblings] — two Adagio Pyro
    /// contracts with the same title but different blueprint pools
    /// typically carry disjoint `mission_span`s (Region A/B vs C/D).
    pub mission_span: Vec<Guid>,

    /// GUIDs of other `Contract`s in the index that share this
    /// contract's `(title, description)` pair. Empty when the title is
    /// unique. Ignores the `(None, None)` case since those can't share
    /// a displayed title anyway.
    ///
    /// Consumers annotating shared titles use this directly — see
    /// [`find_bp_conflicts`] for the blueprint-specific workflow, and
    /// apply the same pattern for scrip / rep / encounter comparisons.
    pub title_siblings: Vec<Guid>,
}

/// One expansion that merged into a [`Contract`]. Holds the diff from
/// the canonical fields.
#[derive(Debug, Clone)]
pub struct Variation {
    pub expansion_id: Guid,
    pub origin: ContractOrigin,
    /// Prerequisites unique to this variation after subtracting the
    /// group's common prereqs. For the Mission08 pattern, this holds
    /// the variation's `Locality` GUID (the planet-specific pickup).
    pub extra_prerequisites: Vec<PrereqView>,
}

// ── Entry point ─────────────────────────────────────────────────────────────

/// Collapse a list of `ExpandedContract`s into one `Contract` per
/// distinct merge key. Order of output is stable: groups iterate in
/// first-seen order (matching `ExpandedContract` input order).
pub fn merge_expansions(expansions: Vec<ExpandedContract>) -> Vec<Contract> {
    let mut groups: Vec<(MergeKey, Vec<ExpandedContract>)> = Vec::new();
    let mut by_key: HashMap<MergeKey, usize> = HashMap::new();

    for e in expansions {
        let key = MergeKey::from(&e);
        if let Some(&idx) = by_key.get(&key) {
            groups[idx].1.push(e);
        } else {
            by_key.insert(key.clone(), groups.len());
            groups.push((key, vec![e]));
        }
    }

    let mut contracts: Vec<Contract> = groups
        .into_iter()
        .map(|(_key, group)| collapse_group(group))
        .collect();

    compute_title_siblings(&mut contracts);
    contracts
}

// ── Merge key ───────────────────────────────────────────────────────────────

/// The tuple `(title, description, reward_signature)` that decides
/// whether two expansions merge. When `title` is `None`, falls back to
/// including `(debug_name, handler_debug_name)` so unrelated text-less
/// contracts don't collapse onto each other.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MergeKey {
    title: Option<String>,
    description: Option<String>,
    reward_sig: String,
    /// When both `title` and `description` are `None`, carries a
    /// synthetic identifier so text-less expansions only merge within
    /// the same handler + contract debug-name.
    text_fallback: Option<(String, String)>,
}

impl MergeKey {
    fn from(e: &ExpandedContract) -> Self {
        let title = e.title.clone();
        let description = e.description.clone();
        let text_fallback = if title.is_none() && description.is_none() {
            Some((e.handler_debug_name.clone(), e.debug_name.clone()))
        } else {
            None
        };
        Self {
            title,
            description,
            reward_sig: reward_signature(e),
            text_fallback,
        }
    }
}

/// Canonical serialisation of the reward set — stringly-typed for simple
/// hashing. Deterministic ordering: scrip / rep / items sorted by GUID,
/// `other` sorted by discriminant name.
fn reward_signature(e: &ExpandedContract) -> String {
    let mut s = String::new();
    // UEC.
    match &e.reward_uec {
        RewardAmount::None => s.push_str("uec:none|"),
        RewardAmount::Calculated => s.push_str("uec:calc|"),
        RewardAmount::Fixed(n) => s.push_str(&format!("uec:fix{n}|")),
    }
    // Scrip.
    let mut scrip: Vec<(Guid, i32)> = e
        .reward_scrip
        .iter()
        .map(|r| (r.currency_guid, r.amount))
        .collect();
    scrip.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()).then(a.1.cmp(&b.1)));
    for (g, n) in &scrip {
        s.push_str(&format!("sc:{g}={n}|"));
    }
    // Rep — amount may be None (CalculatedReputation).
    let mut rep: Vec<(Option<Guid>, Option<Guid>, Option<i32>)> = e
        .reward_rep
        .iter()
        .map(|r| (r.faction, r.scope, r.amount))
        .collect();
    rep.sort_by(|a, b| {
        format!("{:?}{:?}{:?}", a.0, a.1, a.2).cmp(&format!("{:?}{:?}{:?}", b.0, b.1, b.2))
    });
    for (f, sc, a) in &rep {
        s.push_str(&format!(
            "rep:{}/{}={}",
            f.map(|g| g.to_string()).unwrap_or_default(),
            sc.map(|g| g.to_string()).unwrap_or_default(),
            a.map(|n| n.to_string()).unwrap_or_else(|| "calc".into()),
        ));
        s.push('|');
    }
    // Items.
    let mut items: Vec<(Guid, i32)> = e
        .reward_items
        .iter()
        .map(|r| (r.entity_class, r.amount))
        .collect();
    items.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()).then(a.1.cmp(&b.1)));
    for (g, n) in &items {
        s.push_str(&format!("it:{g}={n}|"));
    }
    // Blueprint pool (just the pool guid + chance).
    if let Some(bp) = &e.blueprint_reward {
        s.push_str(&format!("bp:{}@{:.3}|", bp.pool_guid, bp.chance));
    }
    // Other rewards — collapse to discriminant names.
    let mut other_kinds: Vec<&'static str> = e
        .reward_other
        .iter()
        .map(|o| match o {
            OtherReward::BadgeAward => "Badge",
            OtherReward::ScenarioProgress => "ScenProg",
            OtherReward::JournalEntry => "Journal",
            OtherReward::CompletionTags => "CompletionTags",
            OtherReward::CompletionBounty => "CompletionBounty",
            OtherReward::ItemsWeighting => "ItemsWeighting",
            OtherReward::Reward => "Reward",
            OtherReward::RefundBuyIn(_) => "RefundBuyIn",
            OtherReward::Unknown { .. } => "Unknown",
        })
        .collect();
    other_kinds.sort();
    for k in &other_kinds {
        s.push_str(&format!("o:{k}|"));
    }
    s
}

// ── Group collapsing ────────────────────────────────────────────────────────

fn collapse_group(mut group: Vec<ExpandedContract>) -> Contract {
    // Canonical pick: prefer a non-sub-contract origin (the parent
    // contract's id is more stable than any single tier's id).
    // Fallback: first element.
    let canonical_idx = group
        .iter()
        .position(|e| !matches!(e.origin, ContractOrigin::SubContract { .. }))
        .unwrap_or(0);
    group.swap(0, canonical_idx);
    let first = &group[0];

    // Base prerequisites = prereqs that appear in every variation.
    // Each variation's extra_prerequisites = its prereqs minus the base.
    // The comparison is by value (PrereqView doesn't impl Eq directly
    // because of f32 in CrimeStat, so use a stringified signature).
    let base_prereqs = compute_common_prereqs(&group);
    let base_sigs: std::collections::HashSet<String> =
        base_prereqs.iter().map(prereq_signature).collect();

    let variations: Vec<Variation> = group
        .iter()
        .map(|e| {
            let extras: Vec<PrereqView> = e
                .prerequisites
                .iter()
                .filter(|p| !base_sigs.contains(&prereq_signature(p)))
                .cloned()
                .collect();
            Variation {
                expansion_id: e.id,
                origin: e.origin,
                extra_prerequisites: extras,
            }
        })
        .collect();

    // Union of per-variation mission_span, first-seen wins.
    let mut seen_span: HashSet<Guid> = HashSet::new();
    let mut mission_span: Vec<Guid> = Vec::new();
    for e in &group {
        for g in &e.mission_span {
            if seen_span.insert(*g) {
                mission_span.push(*g);
            }
        }
    }

    Contract {
        id: first.id,
        debug_name: first.debug_name.clone(),
        generator_id: first.generator_id,
        handler_kind: first.handler_kind,
        handler_debug_name: first.handler_debug_name.clone(),
        title: first.title.clone(),
        description: first.description.clone(),
        has_runtime_substitution: first.has_runtime_substitution,
        shareable: first.shareable,
        illegal_flag: first.illegal_flag,
        availability: first.availability.clone(),
        reward_uec: first.reward_uec,
        reward_scrip: first.reward_scrip.clone(),
        reward_rep: first.reward_rep.clone(),
        reward_items: first.reward_items.clone(),
        reward_other: first.reward_other.clone(),
        blueprint_reward: first.blueprint_reward.clone(),
        prerequisites: base_prereqs,
        encounters: first.encounters.clone(),
        variations,
        mission_span,
        title_siblings: Vec::new(),
    }
}

/// Intersection of prerequisites across the group — the ones that
/// every variation carries. Order from the first variation.
fn compute_common_prereqs(group: &[ExpandedContract]) -> Vec<PrereqView> {
    if group.is_empty() {
        return Vec::new();
    }
    let first = &group[0];
    if group.len() == 1 {
        return first.prerequisites.clone();
    }

    // Count occurrences of each prereq signature across all variations.
    let n = group.len();
    let mut counts: HashMap<String, usize> = HashMap::new();
    for e in group {
        // Dedup within a single variation so duplicates don't skew counts.
        let mut seen = std::collections::HashSet::new();
        for p in &e.prerequisites {
            let sig = prereq_signature(p);
            if seen.insert(sig.clone()) {
                *counts.entry(sig).or_default() += 1;
            }
        }
    }

    // Keep prereqs that appear in every variation, in first-variation
    // order, deduped.
    let mut seen = std::collections::HashSet::new();
    let mut common = Vec::new();
    for p in &first.prerequisites {
        let sig = prereq_signature(p);
        if counts.get(&sig).copied().unwrap_or(0) == n && seen.insert(sig) {
            common.push(p.clone());
        }
    }
    common
}

fn prereq_signature(p: &PrereqView) -> String {
    match p {
        PrereqView::Locality { locality } => format!(
            "locty:{}",
            locality.map(|g| g.to_string()).unwrap_or_default()
        ),
        PrereqView::Location { location } => format!(
            "loc:{}",
            location.map(|g| g.to_string()).unwrap_or_default()
        ),
        PrereqView::LocationProperty {
            variable_name,
            extended_text_token,
            level,
        } => format!("locprop:{variable_name}:{extended_text_token}:{level:?}"),
        PrereqView::CrimeStat {
            min,
            max,
            jurisdiction_override,
            include_when_sharing,
        } => format!(
            "cs:{min}-{max}:{}:{include_when_sharing}",
            jurisdiction_override.map(|g| g.to_string()).unwrap_or_default()
        ),
        PrereqView::Reputation {
            faction,
            scope,
            exclude,
            min_standing,
            max_standing,
            include_when_sharing,
        } => format!(
            "rep:{}/{}/min={}/max={}/excl={exclude}/share={include_when_sharing}",
            faction.map(|g| g.to_string()).unwrap_or_default(),
            scope.map(|g| g.to_string()).unwrap_or_default(),
            min_standing.map(|g| g.to_string()).unwrap_or_default(),
            max_standing.map(|g| g.to_string()).unwrap_or_default(),
        ),
        PrereqView::CompletedContractTags {
            required_tags,
            required_count,
            excluded_tags,
            excluded_count,
            include_when_sharing,
        } => {
            let mut req: Vec<String> = required_tags.iter().map(|g| g.to_string()).collect();
            req.sort();
            let mut exc: Vec<String> = excluded_tags.iter().map(|g| g.to_string()).collect();
            exc.sort();
            format!(
                "tags:req={}:count={required_count}:exc={}:count={excluded_count}:share={include_when_sharing}",
                req.join(","),
                exc.join(","),
            )
        }
        PrereqView::Unknown {
            struct_index,
            instance_index,
        } => format!("unk:{struct_index}:{instance_index}"),
    }
}

// ── Post-merge: title siblings ──────────────────────────────────────────────

/// Compute `Contract.title_siblings` for every contract. Groups
/// contracts by `(title, description)` (skipping the `(None, None)`
/// case since it can't share a displayed title) and back-fills each
/// contract's `title_siblings` with the GUIDs of the other Contracts
/// in its text-group.
fn compute_title_siblings(contracts: &mut [Contract]) {
    // Build the text → contract-indices index.
    let mut by_text: HashMap<(Option<String>, Option<String>), Vec<usize>> = HashMap::new();
    for (i, c) in contracts.iter().enumerate() {
        if c.title.is_none() && c.description.is_none() {
            continue;
        }
        by_text
            .entry((c.title.clone(), c.description.clone()))
            .or_default()
            .push(i);
    }

    for indices in by_text.values() {
        if indices.len() < 2 {
            continue;
        }
        for &i in indices {
            let siblings: Vec<Guid> = indices
                .iter()
                .filter(|&&j| j != i)
                .map(|&j| contracts[j].id)
                .collect();
            contracts[i].title_siblings = siblings;
        }
    }
}

// ── Conflict-finder helpers ────────────────────────────────────────────────

/// One sibling group that disagrees on blueprint rewards — same
/// `(title, description)` resolves to different blueprint pool GUIDs
/// (or some variants have a BP and others don't).
///
/// Returned by [`find_bp_conflicts`]. Consumers annotating mission text
/// with blueprint info render `[BP]*` (mixed) on any member contract,
/// and can list the distinct pools for disambiguation.
#[derive(Debug, Clone)]
pub struct BpConflictGroup {
    pub title: String,
    pub description: Option<String>,
    pub members: Vec<BpConflictMember>,
    /// Count of distinct blueprint-pool GUIDs across the group. `None`
    /// (no blueprint) counts as a distinct slot — so "2 distinct pools"
    /// can mean two pools, or one pool + "no BP" variants.
    pub distinct_pool_count: usize,
    /// True when at least one member has no blueprint and at least one
    /// other does. The most important case for sc-langpatch-style
    /// annotation: the title promises a reward that doesn't apply to
    /// every in-game instance.
    pub has_mixed_presence: bool,
}

#[derive(Debug, Clone)]
pub struct BpConflictMember {
    pub contract_id: Guid,
    pub debug_name: String,
    pub handler_debug_name: String,
    pub handler_kind: HandlerKind,
    /// `None` when this variant has no blueprint reward.
    pub blueprint_pool: Option<Guid>,
    pub pool_name: Option<String>,
    pub item_count: usize,
}

/// Find every mission where siblings share a title + description but
/// diverge on blueprint rewards. Result is the input for rendering
/// `[BP]*` / mixed-BP disclaimers on enriched mission text.
///
/// Groups are deduplicated: each conflict appears exactly once (keyed
/// off the lowest contract index among members).
pub fn find_bp_conflicts(contracts: &[Contract]) -> Vec<BpConflictGroup> {
    use std::collections::HashSet;

    let by_id: HashMap<Guid, usize> = contracts
        .iter()
        .enumerate()
        .map(|(i, c)| (c.id, i))
        .collect();

    let mut seen: HashSet<Guid> = HashSet::new();
    let mut out = Vec::new();

    for c in contracts {
        if seen.contains(&c.id) || c.title_siblings.is_empty() {
            continue;
        }

        // Build the full group (self + siblings).
        let mut group_ids: Vec<Guid> = Vec::with_capacity(c.title_siblings.len() + 1);
        group_ids.push(c.id);
        group_ids.extend(c.title_siblings.iter().copied());
        // Mark all as seen so we don't emit the same group from a sibling.
        for id in &group_ids {
            seen.insert(*id);
        }

        // Compute distinct pools across the group.
        let mut distinct_pools: HashSet<Option<Guid>> = HashSet::new();
        let mut has_bp = false;
        let mut has_no_bp = false;
        for id in &group_ids {
            let Some(&idx) = by_id.get(id) else { continue };
            let pool = contracts[idx].blueprint_reward.as_ref().map(|b| b.pool_guid);
            distinct_pools.insert(pool);
            if pool.is_some() {
                has_bp = true;
            } else {
                has_no_bp = true;
            }
        }
        if distinct_pools.len() < 2 {
            continue; // all same pool (or all no BP)
        }

        // Build member records in the order they appear in `contracts`
        // (stable output).
        let mut members: Vec<BpConflictMember> = group_ids
            .iter()
            .filter_map(|id| by_id.get(id).map(|&idx| &contracts[idx]))
            .map(|c| BpConflictMember {
                contract_id: c.id,
                debug_name: c.debug_name.clone(),
                handler_debug_name: c.handler_debug_name.clone(),
                handler_kind: c.handler_kind,
                blueprint_pool: c.blueprint_reward.as_ref().map(|b| b.pool_guid),
                pool_name: c.blueprint_reward.as_ref().map(|b| b.pool_name.clone()),
                item_count: c.blueprint_reward.as_ref().map(|b| b.items.len()).unwrap_or(0),
            })
            .collect();
        members.sort_by(|a, b| a.debug_name.cmp(&b.debug_name));

        out.push(BpConflictGroup {
            title: c.title.clone().unwrap_or_default(),
            description: c.description.clone(),
            members,
            distinct_pool_count: distinct_pools.len(),
            has_mixed_presence: has_bp && has_no_bp,
        });
    }

    // Stable output: sort by title.
    out.sort_by(|a, b| a.title.cmp(&b.title).then(a.description.cmp(&b.description)));
    out
}

// ── Public helpers ──────────────────────────────────────────────────────────

/// Summary statistics for a post-merge `Contract` list. Useful for
/// validating against the SCMDB target and for census output.
pub fn merge_stats(contracts: &[Contract]) -> MergeStats {
    let mut variation_count_histogram: BTreeMap<usize, usize> = BTreeMap::new();
    let mut sibling_count_histogram: BTreeMap<usize, usize> = BTreeMap::new();
    let mut largest_variation = 0usize;
    let mut total_variations = 0usize;
    let mut with_siblings = 0usize;

    for c in contracts {
        let n = c.variations.len();
        *variation_count_histogram.entry(n).or_default() += 1;
        largest_variation = largest_variation.max(n);
        total_variations += n;

        let sib = c.title_siblings.len();
        *sibling_count_histogram.entry(sib).or_default() += 1;
        if sib > 0 {
            with_siblings += 1;
        }
    }

    MergeStats {
        total_contracts: contracts.len(),
        total_variations,
        largest_variation,
        contracts_with_title_siblings: with_siblings,
        variation_count_histogram,
        sibling_count_histogram,
    }
}

#[derive(Debug, Clone)]
pub struct MergeStats {
    pub total_contracts: usize,
    pub total_variations: usize,
    pub largest_variation: usize,
    /// Count of contracts whose `title_siblings` is non-empty — i.e.
    /// at least one other contract shares their `(title, description)`.
    /// These are the ones worth running [`find_bp_conflicts`] (and
    /// analogous helpers) against.
    pub contracts_with_title_siblings: usize,
    pub variation_count_histogram: BTreeMap<usize, usize>,
    /// `sibling_count → number-of-contracts-with-that-many-siblings`.
    pub sibling_count_histogram: BTreeMap<usize, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_yields_empty_output() {
        let out = merge_expansions(Vec::new());
        assert!(out.is_empty());
    }

    #[test]
    fn bp_conflicts_empty_on_empty_input() {
        assert!(find_bp_conflicts(&[]).is_empty());
    }
}
