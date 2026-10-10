//! Event ladders — the tiered progress tracks of a limited-time event
//! (RSI Discovery Month, Clean Air, …).
//!
//! An event is a `ScenarioProgress` record. Each of its ladders
//! (`STierProgressions`) counts points one of two ways:
//!
//! - [`LadderKind::Overall`] (`CompletionType_GlobalProgression`) — the
//!   event total, fed by every contract's
//!   `ContractResult_ScenarioProgress.PointsToAward`
//!   ([`crate::EventPoints::points`]). These points split across the party
//!   when the contract says so ([`crate::EventPoints::split_for_party`]).
//! - [`LadderKind::Category`] (`CompletionType_CompletionTag`) — a category
//!   track (Transport / Collection / Defense), fed by the contract's
//!   `ContractResult_CompletionTags` count for the ladder's tag
//!   ([`crate::EventPoints::ladders`]). Completion tags carry no split flag:
//!   every party member earns them in full.
//!
//! `LocaleMap`-free like the other registries: ladder names are
//! [`LocaleKey`]s.

use std::collections::HashMap;

use sc_extract::generated::{CompletionTypeBasePtr, DataPools, EAwardId, ScenarioProgress};
use sc_extract::{Datacore, Guid, LocaleKey, RecordCollection};

/// One event — a `ScenarioProgress` record and its ladders.
#[derive(Debug, Clone)]
pub struct Event {
    /// The `ScenarioProgress` record GUID ([`crate::EventPoints::event`]).
    pub id: Guid,
    /// Record name without the `ScenarioProgress.` prefix
    /// (`Iasi_ScenarioProgress`).
    pub record_name: String,
    pub ladders: Vec<Ladder>,
}

/// One tiered progress track of an event.
#[derive(Debug, Clone)]
pub struct Ladder {
    /// The faction block the ladder sits under (`faction_iasi`).
    pub faction: Option<Guid>,
    /// `progressionText` — the ladder's label in the journal
    /// (`@iasi_Journal_Collection` → "Collection").
    pub text_key: LocaleKey,
    pub kind: LadderKind,
    /// Ascending by [`LadderTier::min_points`], as authored.
    pub tiers: Vec<LadderTier>,
}

/// What a ladder counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LadderKind {
    /// The event total (`CompletionType_GlobalProgression`).
    Overall,
    /// A category track, counting this completion tag
    /// (`CompletionType_CompletionTag`).
    Category(Guid),
}

/// One reward tier of a ladder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LadderTier {
    /// Points needed to reach the tier.
    pub min_points: i32,
    /// `badgeToAward` — the `EAwardId` the tier grants
    /// (`R_PU_IASI_COLLECTION_1`), as its DCB string.
    pub badge: String,
}

/// `ScenarioProgress` GUID → [`Event`].
#[derive(Debug, Clone, Default)]
pub struct Events {
    by_guid: HashMap<Guid, Event>,
}

impl Events {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(datacore: &Datacore) -> Self {
        let pools = &datacore.records().pools;
        let db = datacore.db();
        let mut by_guid = HashMap::new();
        for (guid, handle) in &datacore.records().records.multi_feature.scenario_progress {
            let Some(progress) = handle.get(pools) else {
                continue;
            };
            let record_name = db
                .record(guid)
                .and_then(|r| r.name().map(String::from))
                .map(|n| {
                    n.strip_prefix("ScenarioProgress.")
                        .unwrap_or(&n)
                        .to_string()
                })
                .unwrap_or_default();
            by_guid.insert(
                *guid,
                Event {
                    id: *guid,
                    record_name,
                    ladders: ladders(progress, pools),
                },
            );
        }
        Self { by_guid }
    }
}

impl RecordCollection for Events {
    type Item = Event;

    fn get(&self, id: &Guid) -> Option<&Event> {
        self.by_guid.get(id)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Event)> + '_ {
        self.by_guid.iter()
    }
}

/// Every ladder of a `ScenarioProgress`, across its faction blocks.
fn ladders(progress: &ScenarioProgress, pools: &DataPools) -> Vec<Ladder> {
    let mut out = Vec::new();
    for tiers_h in &progress.faction_reward_tiers {
        let Some(block) = tiers_h.get(pools) else {
            continue;
        };
        for prog_h in &block.tier_progressions {
            let Some(prog) = prog_h.get(pools) else {
                continue;
            };
            let kind = match prog.completion_type.as_ref() {
                Some(CompletionTypeBasePtr::CompletionType_GlobalProgression(_)) => {
                    LadderKind::Overall
                }
                Some(CompletionTypeBasePtr::CompletionType_CompletionTag(h)) => {
                    match h.get(pools).and_then(|c| c.completion_tags) {
                        Some(tag) => LadderKind::Category(tag),
                        None => continue,
                    }
                }
                // No completion type, or one this build doesn't model
                // (`DeliveredSCU` is dormant): nothing we can attribute
                // points to.
                _ => continue,
            };
            let tiers = prog
                .tier_rewards
                .iter()
                .filter_map(|h| h.get(pools))
                .map(|t| LadderTier {
                    min_points: t.min_points,
                    badge: award_id(&t.badge_to_award),
                })
                .collect();
            out.push(Ladder {
                faction: block.faction,
                text_key: prog.progression_text.clone(),
                kind,
                tiers,
            });
        }
    }
    out
}

/// The category-ladder tags of one event — the completion tags whose
/// counts are event points.
pub(crate) fn category_tags(progress: &ScenarioProgress, pools: &DataPools) -> Vec<Guid> {
    ladders(progress, pools)
        .into_iter()
        .filter_map(|l| match l.kind {
            LadderKind::Category(tag) => Some(tag),
            LadderKind::Overall => None,
        })
        .collect()
}

/// The DCB string of an `EAwardId` (generated enums keep unknown values
/// as `Unrecognized`; known ones are their variant name).
fn award_id(id: &EAwardId) -> String {
    match id {
        EAwardId::Unrecognized(s) => s.clone(),
        known => format!("{known:?}"),
    }
}
