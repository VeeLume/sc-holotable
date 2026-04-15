// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `reputation` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReputationPools {
    #[serde(default)]
    pub sandbox_infraction_definition_trigger: Vec<Option<SandboxInfractionDefinitionTrigger>>,
    #[serde(default)]
    pub sreputation_state_params: Vec<Option<SReputationStateParams>>,
    #[serde(default)]
    pub sreputation_state_modifier_increment: Vec<Option<SReputationStateModifierIncrement>>,
    #[serde(default)]
    pub sreputation_state_modifier_set: Vec<Option<SReputationStateModifierSet>>,
    #[serde(default)]
    pub sreputation_state_modifier_set_bool: Vec<Option<SReputationStateModifierSetBool>>,
    #[serde(default)]
    pub sreputation_state_modifier_set_to_state: Vec<Option<SReputationStateModifierSetToState>>,
    #[serde(default)]
    pub sreputation_state_modifier_params: Vec<Option<SReputationStateModifierParams>>,
    #[serde(default)]
    pub sreputation_state_mission_result_modifier_list_params: Vec<Option<SReputationStateMissionResultModifierListParams>>,
    #[serde(default)]
    pub sreputation_state_mission_result_modifier_params: Vec<Option<SReputationStateMissionResultModifierParams>>,
}
