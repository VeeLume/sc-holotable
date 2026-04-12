// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `reputation` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReputationPools {
    #[serde(default)]
    pub reputation_reward_base_def: Vec<Option<ReputationRewardBaseDef>>,
    #[serde(default)]
    pub sandbox_infraction_base_def: Vec<Option<SandboxInfractionBaseDef>>,
    #[serde(default)]
    pub sandbox_trigger_manual_params: Vec<Option<SandboxTriggerManualParams>>,
    #[serde(default)]
    pub sandbox_trigger_record: Vec<Option<SandboxTriggerRecord>>,
    #[serde(default)]
    pub sperk_params_base: Vec<Option<SPerkParamsBase>>,
    #[serde(default)]
    pub sperk_reputation_params: Vec<Option<SPerkReputationParams>>,
    #[serde(default)]
    pub sperk_reputation_list_params: Vec<Option<SPerkReputationListParams>>,
    #[serde(default)]
    pub sreputation_standing_params: Vec<Option<SReputationStandingParams>>,
    #[serde(default)]
    pub sreputation_standing_map_params: Vec<Option<SReputationStandingMapParams>>,
    #[serde(default)]
    pub sreputation_scope_context_ui: Vec<Option<SReputationScopeContextUI>>,
    #[serde(default)]
    pub sreputation_context_ui: Vec<Option<SReputationContextUI>>,
    #[serde(default)]
    pub sreputation_state_params: Vec<Option<SReputationStateParams>>,
    #[serde(default)]
    pub sreputation_state_modifier_base: Vec<Option<SReputationStateModifierBase>>,
    #[serde(default)]
    pub sreputation_state_modifier_params: Vec<Option<SReputationStateModifierParams>>,
    #[serde(default)]
    pub sreputation_state_mission_result_modifier_list_params: Vec<Option<SReputationStateMissionResultModifierListParams>>,
    #[serde(default)]
    pub sreputation_state_mission_result_modifier_params: Vec<Option<SReputationStateMissionResultModifierParams>>,
    #[serde(default)]
    pub sreputation_scope_params: Vec<Option<SReputationScopeParams>>,
    #[serde(default)]
    pub sreputation_reward_amount: Vec<Option<SReputationRewardAmount>>,
    #[serde(default)]
    pub sreputation_standing_reward_bonus_params: Vec<Option<SReputationStandingRewardBonusParams>>,
    #[serde(default)]
    pub sreputation_mission_giver_reward_bonus_params: Vec<Option<SReputationMissionGiverRewardBonusParams>>,
    #[serde(default)]
    pub sreputation_mission_reward_bonus_params: Vec<Option<SReputationMissionRewardBonusParams>>,
}
