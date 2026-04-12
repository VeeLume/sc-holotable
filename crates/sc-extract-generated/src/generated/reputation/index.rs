// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `reputation` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReputationIndex {
    #[serde(default)]
    pub sandbox_trigger_record: HashMap<CigGuid, Handle<SandboxTriggerRecord>>,
    #[serde(default)]
    pub sperk_reputation_list_params: HashMap<CigGuid, Handle<SPerkReputationListParams>>,
    #[serde(default)]
    pub sreputation_standing_params: HashMap<CigGuid, Handle<SReputationStandingParams>>,
    #[serde(default)]
    pub sreputation_context_ui: HashMap<CigGuid, Handle<SReputationContextUI>>,
    #[serde(default)]
    pub sreputation_state_params: HashMap<CigGuid, Handle<SReputationStateParams>>,
    #[serde(default)]
    pub sreputation_state_mission_result_modifier_params: HashMap<CigGuid, Handle<SReputationStateMissionResultModifierParams>>,
    #[serde(default)]
    pub sreputation_scope_params: HashMap<CigGuid, Handle<SReputationScopeParams>>,
    #[serde(default)]
    pub sreputation_reward_amount: HashMap<CigGuid, Handle<SReputationRewardAmount>>,
    #[serde(default)]
    pub sreputation_mission_reward_bonus_params: HashMap<CigGuid, Handle<SReputationMissionRewardBonusParams>>,
}

impl ReputationIndex {
    pub fn len(&self) -> usize {
        self.sandbox_trigger_record.len()
            + self.sperk_reputation_list_params.len()
            + self.sreputation_standing_params.len()
            + self.sreputation_context_ui.len()
            + self.sreputation_state_params.len()
            + self.sreputation_state_mission_result_modifier_params.len()
            + self.sreputation_scope_params.len()
            + self.sreputation_reward_amount.len()
            + self.sreputation_mission_reward_bonus_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
