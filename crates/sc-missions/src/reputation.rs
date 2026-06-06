//! Reputation registries — faction-reputation and standing-rank GUID → name.
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

use std::collections::HashMap;

use sc_extract::{Datacore, Guid, LocaleKey};

/// One resolved reputation faction — the "Faction" axis a consumer groups /
/// filters missions by (SCMDB's faction column).
#[derive(Debug, Clone)]
pub struct FactionRep {
    /// `FactionReputation.name` — stable record name
    /// (`"FactionReputation_Lawful_LingFamilyHauling"`). Locale-independent;
    /// useful as a grouping key and diagnostic.
    pub record_name: String,
    /// `FactionReputation.displayName` — resolve against a `LocaleMap` for the
    /// player-facing name (`"Ling Family Hauling"`).
    pub display_name_key: LocaleKey,
}

/// `FactionReputation` GUID → [`FactionRep`]. Requires the `reputation`
/// sc-extract feature (the records are gated behind it).
#[derive(Debug, Clone, Default)]
pub struct FactionReputations {
    by_guid: HashMap<Guid, FactionRep>,
}

impl FactionReputations {
    pub fn build(datacore: &Datacore) -> Self {
        let pools = &datacore.records().pools;
        let mut by_guid = HashMap::new();
        for (guid, handle) in &datacore.records().records.multi_feature.faction_reputation {
            if let Some(fr) = handle.get(pools) {
                by_guid.insert(
                    *guid,
                    FactionRep {
                        record_name: fr.name.clone(),
                        display_name_key: fr.display_name.clone(),
                    },
                );
            }
        }
        Self { by_guid }
    }

    pub fn get(&self, guid: &Guid) -> Option<&FactionRep> {
        self.by_guid.get(guid)
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
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
                        gated: s.gated,
                    },
                );
            }
        }
        Self { by_guid }
    }

    pub fn get(&self, guid: &Guid) -> Option<&Standing> {
        self.by_guid.get(guid)
    }

    pub fn len(&self) -> usize {
        self.by_guid.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_guid.is_empty()
    }
}
