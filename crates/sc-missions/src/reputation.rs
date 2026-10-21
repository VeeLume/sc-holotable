//! Reputation registries — faction-reputation, scope and standing-rank GUID
//! → name.
//!
//! Contracts reference reputation **factions** (the giving / gating faction,
//! e.g. `FactionReputation_Lawful_LingFamilyHauling`) on their reward and
//! prerequisite records, and reputation **standing tiers**
//! (`ReputationStanding_FactionRep_Rank0..6`) on rep prerequisites. Both are
//! bare `Reference` GUIDs in the contract graph; these registries resolve them
//! to a stable record name + a localizable display-name key.
//!
//! Following the crate idiom (see [`crate::RewardCurrencies`] /
//! [`crate::BlueprintPools`]), the registries are **`LocaleMap`-free**: they
//! store the [`LocaleKey`], and the consumer resolves it against the active
//! locale at the call site. Built once from a [`Datacore`]; carried on
//! [`crate::Missions`].
//!
//! The gRPC `ReputationService` names the same records (verified on LIVE
//! 4.10): its `entity` `urn:sc:reputations:org:geid:<n>` is
//! `FactionReputation.GEID` ([`FactionReputations::by_geid`]), its
//! `standings_id` an `SReputationScopeParams` record GUID
//! ([`ReputationScopes`]) and its `standing.id` an
//! `SReputationStandingParams` record GUID ([`ReputationStandings`]).

use std::collections::HashMap;

use sc_extract::{Datacore, Guid, LocaleKey, RecordCollection};

/// One resolved reputation faction — the "Faction" axis a consumer groups /
/// filters missions by (SCMDB's faction column).
#[derive(Debug, Clone)]
pub struct FactionRep {
    /// The record name (`"FactionReputation_Lawful_LingFamilyHauling"`).
    /// Locale-independent; useful as a grouping key and diagnostic. (The
    /// `name` field reads `(Default)` on every faction in LIVE 4.10.)
    pub record_name: String,
    /// `FactionReputation.displayName` — resolve against a `LocaleMap` for the
    /// player-facing name (`"Ling Family Hauling"`).
    pub display_name_key: LocaleKey,
    /// `FactionReputation.GEID`: the org id the reputation service sends
    /// (`urn:sc:reputations:org:geid:199998949491` is Covalex).
    pub geid: u64,
}

/// `FactionReputation` GUID → [`FactionRep`]. Requires the `reputation`
/// sc-extract feature (the records are gated behind it).
#[derive(Debug, Clone, Default)]
pub struct FactionReputations {
    by_guid: HashMap<Guid, FactionRep>,
    by_geid: HashMap<u64, Guid>,
}

impl FactionReputations {
    pub fn build(datacore: &Datacore) -> Self {
        let pools = &datacore.records().pools;
        let mut by_guid = HashMap::new();
        let mut by_geid = HashMap::new();
        for (guid, handle) in &datacore.records().records.multi_feature.faction_reputation {
            if let Some(fr) = handle.get(pools) {
                if fr.geid != 0 {
                    by_geid.insert(fr.geid, *guid);
                }
                by_guid.insert(
                    *guid,
                    FactionRep {
                        record_name: record_name(datacore, guid),
                        display_name_key: fr.display_name.clone(),
                        geid: fr.geid,
                    },
                );
            }
        }
        Self { by_guid, by_geid }
    }

    /// The faction with `FactionReputation.GEID` `geid`.
    pub fn by_geid(&self, geid: u64) -> Option<&FactionRep> {
        self.by_geid.get(&geid).and_then(|g| self.by_guid.get(g))
    }

    /// The record GUID of the faction with GEID `geid`.
    pub fn guid_by_geid(&self, geid: u64) -> Option<Guid> {
        self.by_geid.get(&geid).copied()
    }
}

impl RecordCollection for FactionReputations {
    type Item = FactionRep;

    fn get(&self, guid: &Guid) -> Option<&FactionRep> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &FactionRep)> + '_ {
        self.by_guid.iter()
    }
}

/// One resolved reputation standing tier (`Rank0`..`Rank6` etc.) — the bound
/// on a [`crate::PrereqView::Reputation`] min/max-standing reference.
#[derive(Debug, Clone)]
pub struct Standing {
    /// `SReputationStandingParams.name` — stable record name
    /// (`"ReputationStanding_FactionRep_Rank2"`).
    pub record_name: String,
    /// `displayName` — resolve against a `LocaleMap` for the player-facing
    /// tier name.
    pub display_name_key: LocaleKey,
    /// `minReputation` — the numeric reputation threshold this tier begins at.
    /// Lets a consumer order tiers without parsing the record name.
    pub min_reputation: i64,
    /// `driftReputation` / `driftTimeHours`: how reputation drifts while
    /// in this tier.
    pub drift_reputation: i64,
    pub drift_time_hours: f32,
    /// `gated` — whether reaching this tier unlocks gated content.
    pub gated: bool,
}

/// `SReputationStandingParams` GUID → [`Standing`]. Requires the `reputation`
/// sc-extract feature.
#[derive(Debug, Clone, Default)]
pub struct ReputationStandings {
    by_guid: HashMap<Guid, Standing>,
}

impl ReputationStandings {
    pub fn build(datacore: &Datacore) -> Self {
        let pools = &datacore.records().pools;
        let mut by_guid = HashMap::new();
        for (guid, handle) in &datacore
            .records()
            .records
            .multi_feature
            .sreputation_standing_params
        {
            if let Some(s) = handle.get(pools) {
                by_guid.insert(
                    *guid,
                    Standing {
                        record_name: s.name.clone(),
                        display_name_key: s.display_name.clone(),
                        min_reputation: s.min_reputation,
                        drift_reputation: s.drift_reputation,
                        drift_time_hours: s.drift_time_hours,
                        gated: s.gated,
                    },
                );
            }
        }
        Self { by_guid }
    }
}

impl RecordCollection for ReputationStandings {
    type Item = Standing;

    fn get(&self, guid: &Guid) -> Option<&Standing> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &Standing)> + '_ {
        self.by_guid.iter()
    }
}

/// One reputation scope (`SReputationScopeParams`): a ladder of ranks a
/// faction's reputation is measured on (`FactionReputationScope`,
/// `ReputationScope_Hauling`, ...).
#[derive(Debug, Clone)]
pub struct ReputationScope {
    /// The record name (`"ReputationScope_Hauling"`).
    pub record_name: String,
    /// `scopeName`.
    pub scope_name: String,
    /// `displayName`; many are `<= PLACEHOLDER =>`.
    pub display_name_key: LocaleKey,
    /// `standingMap.reputationCeiling` / `initialReputation`.
    pub ceiling: i64,
    pub initial: i64,
    /// `standingMap.standings`: [`Standing`] GUIDs, lowest
    /// `minReputation` first (the data stores some ladders descending).
    /// Scopes share rank records (Delivery_CitizensForPyro reuses the
    /// Hauling scope's Delivery ranks), so look a rank up by GUID.
    pub ranks: Vec<Guid>,
}

/// `SReputationScopeParams` GUID -> [`ReputationScope`]. Requires the
/// `reputation` sc-extract feature.
#[derive(Debug, Clone, Default)]
pub struct ReputationScopes {
    by_guid: HashMap<Guid, ReputationScope>,
}

impl ReputationScopes {
    pub fn build(datacore: &Datacore) -> Self {
        let store = datacore.records();
        let pools = &store.pools;
        let standings = &store.records.multi_feature.sreputation_standing_params;
        let min = |g: &Guid| {
            standings
                .get(g)
                .and_then(|h| h.get(pools))
                .map_or(i64::MAX, |s| s.min_reputation)
        };
        let mut by_guid = HashMap::new();
        for (guid, handle) in &store.records.multi_feature.sreputation_scope_params {
            let Some(scope) = handle.get(pools) else {
                continue;
            };
            let map = scope.standing_map.and_then(|h| h.get(pools));
            let mut ranks: Vec<Guid> = map.map(|m| m.standings.clone()).unwrap_or_default();
            ranks.sort_by_key(min);
            by_guid.insert(
                *guid,
                ReputationScope {
                    record_name: record_name(datacore, guid),
                    scope_name: scope.scope_name.clone(),
                    display_name_key: scope.display_name.clone(),
                    ceiling: map.map_or(0, |m| m.reputation_ceiling),
                    initial: map.map_or(0, |m| m.initial_reputation),
                    ranks,
                },
            );
        }
        Self { by_guid }
    }
}

impl RecordCollection for ReputationScopes {
    type Item = ReputationScope;

    fn get(&self, guid: &Guid) -> Option<&ReputationScope> {
        self.by_guid.get(guid)
    }

    fn len(&self) -> usize {
        self.by_guid.len()
    }

    fn iter(&self) -> impl Iterator<Item = (&Guid, &ReputationScope)> + '_ {
        self.by_guid.iter()
    }
}

/// A record's name without its type prefix
/// (`FactionReputation.FactionReputation_Battaglia` →
/// `FactionReputation_Battaglia`).
fn record_name(datacore: &Datacore, guid: &Guid) -> String {
    datacore
        .db()
        .record(guid)
        .and_then(|r| r.name())
        .map(|n| n.split_once('.').map_or(n, |(_, rest)| rest).to_string())
        .unwrap_or_default()
}
