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

/// Pool storage for the `missionbroker` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MissionbrokerPools {
    #[serde(default)]
    pub mission_modifier_law_license: Vec<Option<MissionModifier_LawLicense>>,
    #[serde(default)]
    pub mission_modifier_faction_hostility: Vec<Option<MissionModifier_FactionHostility>>,
    #[serde(default)]
    pub mission_modifier_hostile_mission: Vec<Option<MissionModifier_HostileMission>>,
    #[serde(default)]
    pub mission_modifier_ignore_mission_player_criminality: Vec<Option<MissionModifier_IgnoreMissionPlayerCriminality>>,
    #[serde(default)]
    pub sreputation_mission_requirement_expression_and: Vec<Option<SReputationMissionRequirementExpression_And>>,
    #[serde(default)]
    pub sreputation_mission_requirement_expression_left_parenthesis: Vec<Option<SReputationMissionRequirementExpression_LeftParenthesis>>,
    #[serde(default)]
    pub sreputation_mission_requirement_expression_right_parenthesis: Vec<Option<SReputationMissionRequirementExpression_RightParenthesis>>,
}
