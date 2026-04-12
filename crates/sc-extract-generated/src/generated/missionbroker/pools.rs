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

/// Pool storage for the `missionbroker` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MissionbrokerPools {
    #[serde(default)]
    pub date: Vec<Option<Date>>,
    #[serde(default)]
    pub time: Vec<Option<Time>>,
    #[serde(default)]
    pub date_time: Vec<Option<DateTime>>,
    #[serde(default)]
    pub date_time_schedule: Vec<Option<DateTimeSchedule>>,
    #[serde(default)]
    pub reputation_prerequisite_range: Vec<Option<ReputationPrerequisiteRange>>,
    #[serde(default)]
    pub reputation_prerequisites: Vec<Option<ReputationPrerequisites>>,
    #[serde(default)]
    pub mission_deadline: Vec<Option<MissionDeadline>>,
    #[serde(default)]
    pub mission_reward: Vec<Option<MissionReward>>,
    #[serde(default)]
    pub min_required_missions: Vec<Option<MinRequiredMissions>>,
    #[serde(default)]
    pub mission_broker_entry: Vec<Option<MissionBrokerEntry>>,
    #[serde(default)]
    pub mission_complete_perk_base_def: Vec<Option<MissionCompletePerkBaseDef>>,
    #[serde(default)]
    pub sreputation_amount_params: Vec<Option<SReputationAmountParams>>,
    #[serde(default)]
    pub sreputation_amount_list_params: Vec<Option<SReputationAmountListParams>>,
    #[serde(default)]
    pub sreputation_mission_requirement_expression_element: Vec<Option<SReputationMissionRequirementExpressionElement>>,
    #[serde(default)]
    pub sreputation_mission_requirements_params: Vec<Option<SReputationMissionRequirementsParams>>,
}
