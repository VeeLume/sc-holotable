// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `missionbroker` feature.
#[derive(Default)]
pub struct MissionbrokerPools {
    pub mission_modifier_law_license: Vec<Option<MissionModifier_LawLicense>>,
    pub mission_modifier_faction_hostility: Vec<Option<MissionModifier_FactionHostility>>,
    pub mission_modifier_hostile_mission: Vec<Option<MissionModifier_HostileMission>>,
    pub mission_modifier_ignore_mission_player_criminality: Vec<Option<MissionModifier_IgnoreMissionPlayerCriminality>>,
    pub sreputation_mission_requirement_expression_and: Vec<Option<SReputationMissionRequirementExpression_And>>,
    pub sreputation_mission_requirement_expression_left_parenthesis: Vec<Option<SReputationMissionRequirementExpression_LeftParenthesis>>,
    pub sreputation_mission_requirement_expression_right_parenthesis: Vec<Option<SReputationMissionRequirementExpression_RightParenthesis>>,
}
