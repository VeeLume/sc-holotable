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

/// Pool storage for the `missionscenarios` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MissionscenariosPools {
    #[serde(default)]
    pub mission_scenario_cycle_phase: Vec<Option<MissionScenarioCyclePhase>>,
    #[serde(default)]
    pub mission_scenario_cycle: Vec<Option<MissionScenarioCycle>>,
    #[serde(default)]
    pub mission_scenario_schedule_constraint: Vec<Option<MissionScenarioScheduleConstraint>>,
    #[serde(default)]
    pub mission_scenario_schedule_recurrence: Vec<Option<MissionScenarioScheduleRecurrence>>,
    #[serde(default)]
    pub mission_scenario_schedule: Vec<Option<MissionScenarioSchedule>>,
    #[serde(default)]
    pub mission_scenario: Vec<Option<MissionScenario>>,
    #[serde(default)]
    pub sresource_reward_multiplier: Vec<Option<SResourceRewardMultiplier>>,
    #[serde(default)]
    pub stier_progressions: Vec<Option<STierProgressions>>,
    #[serde(default)]
    pub stier_reward: Vec<Option<STierReward>>,
    #[serde(default)]
    pub sreward_periodical: Vec<Option<SRewardPeriodical>>,
    #[serde(default)]
    pub sscenario_progress_rewards_tiers: Vec<Option<SScenarioProgressRewardsTiers>>,
    #[serde(default)]
    pub scenario_progress: Vec<Option<ScenarioProgress>>,
    #[serde(default)]
    pub completion_type_base: Vec<Option<CompletionTypeBase>>,
}
