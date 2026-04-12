// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `missionscenarios`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MissionScenarioCyclePhase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioCyclePhase {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `duration_seconds` (UInt32)
    #[serde(default)]
    pub duration_seconds: u32,
}

impl Pooled for MissionScenarioCyclePhase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.mission_scenario_cycle_phase }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.mission_scenario_cycle_phase }
}

impl<'a> Extract<'a> for MissionScenarioCyclePhase {
    const TYPE_NAME: &'static str = "MissionScenarioCyclePhase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            duration_seconds: inst.get_u32("duration_seconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionScenarioCycle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioCycle {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `phases` (Class (array))
    #[serde(default)]
    pub phases: Vec<Handle<MissionScenarioCyclePhase>>,
}

impl Pooled for MissionScenarioCycle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.mission_scenario_cycle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.mission_scenario_cycle }
}

impl<'a> Extract<'a> for MissionScenarioCycle {
    const TYPE_NAME: &'static str = "MissionScenarioCycle";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            phases: inst.get_array("phases")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionScenarioCyclePhase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionScenarioCyclePhase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionScenarioScheduleConstraint`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioScheduleConstraint {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `epoch` (Int32)
    #[serde(default)]
    pub epoch: i32,
}

impl Pooled for MissionScenarioScheduleConstraint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.mission_scenario_schedule_constraint }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.mission_scenario_schedule_constraint }
}

impl<'a> Extract<'a> for MissionScenarioScheduleConstraint {
    const TYPE_NAME: &'static str = "MissionScenarioScheduleConstraint";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            epoch: inst.get_i32("epoch").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionScenarioScheduleRecurrence`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioScheduleRecurrence {
    /// `cron` (String)
    #[serde(default)]
    pub cron: String,
    /// `duration_seconds` (UInt32)
    #[serde(default)]
    pub duration_seconds: u32,
}

impl Pooled for MissionScenarioScheduleRecurrence {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.mission_scenario_schedule_recurrence }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.mission_scenario_schedule_recurrence }
}

impl<'a> Extract<'a> for MissionScenarioScheduleRecurrence {
    const TYPE_NAME: &'static str = "MissionScenarioScheduleRecurrence";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            cron: inst.get_str("cron").map(String::from).unwrap_or_default(),
            duration_seconds: inst.get_u32("duration_seconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionScenarioSchedule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenarioSchedule {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `start_constraint` (StrongPointer)
    #[serde(default)]
    pub start_constraint: Option<Handle<MissionScenarioScheduleConstraint>>,
    /// `end_constraint` (StrongPointer)
    #[serde(default)]
    pub end_constraint: Option<Handle<MissionScenarioScheduleConstraint>>,
    /// `recurrence` (StrongPointer)
    #[serde(default)]
    pub recurrence: Option<Handle<MissionScenarioScheduleRecurrence>>,
}

impl Pooled for MissionScenarioSchedule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.mission_scenario_schedule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.mission_scenario_schedule }
}

impl<'a> Extract<'a> for MissionScenarioSchedule {
    const TYPE_NAME: &'static str = "MissionScenarioSchedule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            start_constraint: match inst.get("start_constraint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionScenarioScheduleConstraint>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionScenarioScheduleConstraint>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            end_constraint: match inst.get("end_constraint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionScenarioScheduleConstraint>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionScenarioScheduleConstraint>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            recurrence: match inst.get("recurrence") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionScenarioScheduleRecurrence>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionScenarioScheduleRecurrence>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionScenario`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionScenario {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `variables` (StrongPointer (array))
    #[serde(default)]
    pub variables: Vec<Handle<MissionVariableBase>>,
    /// `cycles` (Class (array))
    #[serde(default)]
    pub cycles: Vec<Handle<MissionScenarioCycle>>,
    /// `schedule` (StrongPointer)
    #[serde(default)]
    pub schedule: Option<Handle<MissionScenarioSchedule>>,
    /// `auto_create` (Boolean)
    #[serde(default)]
    pub auto_create: bool,
    /// `track_progress` (Boolean)
    #[serde(default)]
    pub track_progress: bool,
}

impl Pooled for MissionScenario {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.mission_scenario }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.mission_scenario }
}

impl<'a> Extract<'a> for MissionScenario {
    const TYPE_NAME: &'static str = "MissionScenario";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            variables: inst.get_array("variables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionVariableBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionVariableBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            cycles: inst.get_array("cycles")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionScenarioCycle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionScenarioCycle>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            schedule: match inst.get("schedule") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionScenarioSchedule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionScenarioSchedule>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            auto_create: inst.get_bool("auto_create").unwrap_or_default(),
            track_progress: inst.get_bool("track_progress").unwrap_or_default(),
        }
    }
}

/// DCB type: `SResourceRewardMultiplier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SResourceRewardMultiplier {
    /// `resource` (Reference)
    #[serde(default)]
    pub resource: Option<CigGuid>,
    /// `UECPerSCU` (Int32)
    #[serde(default)]
    pub uecper_scu: i32,
}

impl Pooled for SResourceRewardMultiplier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.sresource_reward_multiplier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.sresource_reward_multiplier }
}

impl<'a> Extract<'a> for SResourceRewardMultiplier {
    const TYPE_NAME: &'static str = "SResourceRewardMultiplier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            resource: inst.get("resource").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            uecper_scu: inst.get_i32("UECPerSCU").unwrap_or_default(),
        }
    }
}

/// DCB type: `STierProgressions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STierProgressions {
    /// `progressionText` (Locale)
    #[serde(default)]
    pub progression_text: String,
    /// `minMultiplierThreshold` (Single)
    #[serde(default)]
    pub min_multiplier_threshold: f32,
    /// `showAsEqualPartsOnProgression` (Boolean)
    #[serde(default)]
    pub show_as_equal_parts_on_progression: bool,
    /// `lastTierDisplayOnProgression` (Boolean)
    #[serde(default)]
    pub last_tier_display_on_progression: bool,
    /// `progressionColor` (String)
    #[serde(default)]
    pub progression_color: String,
    /// `completionType` (StrongPointer)
    #[serde(default)]
    pub completion_type: Option<Handle<CompletionTypeBase>>,
    /// `periodMissionScenario` (Reference)
    #[serde(default)]
    pub period_mission_scenario: Option<CigGuid>,
    /// `tierRewards` (Class (array))
    #[serde(default)]
    pub tier_rewards: Vec<Handle<STierReward>>,
}

impl Pooled for STierProgressions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.stier_progressions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.stier_progressions }
}

impl<'a> Extract<'a> for STierProgressions {
    const TYPE_NAME: &'static str = "STierProgressions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            progression_text: inst.get_str("progressionText").map(String::from).unwrap_or_default(),
            min_multiplier_threshold: inst.get_f32("minMultiplierThreshold").unwrap_or_default(),
            show_as_equal_parts_on_progression: inst.get_bool("showAsEqualPartsOnProgression").unwrap_or_default(),
            last_tier_display_on_progression: inst.get_bool("lastTierDisplayOnProgression").unwrap_or_default(),
            progression_color: inst.get_str("progressionColor").map(String::from).unwrap_or_default(),
            completion_type: match inst.get("completionType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CompletionTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CompletionTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            period_mission_scenario: inst.get("periodMissionScenario").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            tier_rewards: inst.get_array("tierRewards")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<STierReward>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<STierReward>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `STierReward`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STierReward {
    /// `minPoints` (Int32)
    #[serde(default)]
    pub min_points: i32,
    /// `badgeToAward` (EnumChoice)
    #[serde(default)]
    pub badge_to_award: String,
    /// `globalPointsMultiplier` (Int32)
    #[serde(default)]
    pub global_points_multiplier: i32,
    /// `commsNotification` (Reference)
    #[serde(default)]
    pub comms_notification: Option<CigGuid>,
}

impl Pooled for STierReward {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.stier_reward }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.stier_reward }
}

impl<'a> Extract<'a> for STierReward {
    const TYPE_NAME: &'static str = "STierReward";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_points: inst.get_i32("minPoints").unwrap_or_default(),
            badge_to_award: inst.get_str("badgeToAward").map(String::from).unwrap_or_default(),
            global_points_multiplier: inst.get_i32("globalPointsMultiplier").unwrap_or_default(),
            comms_notification: inst.get("commsNotification").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SRewardPeriodical`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRewardPeriodical {
    /// `timePeriodText` (Locale)
    #[serde(default)]
    pub time_period_text: String,
    /// `minPoints` (Int32)
    #[serde(default)]
    pub min_points: i32,
    /// `globalPointsMultiplier` (Single)
    #[serde(default)]
    pub global_points_multiplier: f32,
    /// `minMultiplierThreshold` (Single)
    #[serde(default)]
    pub min_multiplier_threshold: f32,
    /// `completionTags` (Reference)
    #[serde(default)]
    pub completion_tags: Option<CigGuid>,
    /// `periodMissionScenario` (Reference)
    #[serde(default)]
    pub period_mission_scenario: Option<CigGuid>,
}

impl Pooled for SRewardPeriodical {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.sreward_periodical }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.sreward_periodical }
}

impl<'a> Extract<'a> for SRewardPeriodical {
    const TYPE_NAME: &'static str = "SRewardPeriodical";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            time_period_text: inst.get_str("timePeriodText").map(String::from).unwrap_or_default(),
            min_points: inst.get_i32("minPoints").unwrap_or_default(),
            global_points_multiplier: inst.get_f32("globalPointsMultiplier").unwrap_or_default(),
            min_multiplier_threshold: inst.get_f32("minMultiplierThreshold").unwrap_or_default(),
            completion_tags: inst.get("completionTags").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            period_mission_scenario: inst.get("periodMissionScenario").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SScenarioProgressRewardsTiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SScenarioProgressRewardsTiers {
    /// `faction` (Reference)
    #[serde(default)]
    pub faction: Option<CigGuid>,
    /// `convertTiersPointsToPercent` (Boolean)
    #[serde(default)]
    pub convert_tiers_points_to_percent: bool,
    /// `tierProgressions` (Class (array))
    #[serde(default)]
    pub tier_progressions: Vec<Handle<STierProgressions>>,
    /// `rewardPeriodical` (Class (array))
    #[serde(default)]
    pub reward_periodical: Vec<Handle<SRewardPeriodical>>,
}

impl Pooled for SScenarioProgressRewardsTiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.sscenario_progress_rewards_tiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.sscenario_progress_rewards_tiers }
}

impl<'a> Extract<'a> for SScenarioProgressRewardsTiers {
    const TYPE_NAME: &'static str = "SScenarioProgressRewardsTiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            faction: inst.get("faction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            convert_tiers_points_to_percent: inst.get_bool("convertTiersPointsToPercent").unwrap_or_default(),
            tier_progressions: inst.get_array("tierProgressions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<STierProgressions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<STierProgressions>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            reward_periodical: inst.get_array("rewardPeriodical")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SRewardPeriodical>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SRewardPeriodical>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ScenarioProgress`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioProgress {
    /// `multiplierRewards` (Class (array))
    #[serde(default)]
    pub multiplier_rewards: Vec<Handle<SResourceRewardMultiplier>>,
    /// `factionRewardTiers` (Class (array))
    #[serde(default)]
    pub faction_reward_tiers: Vec<Handle<SScenarioProgressRewardsTiers>>,
}

impl Pooled for ScenarioProgress {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.scenario_progress }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.scenario_progress }
}

impl<'a> Extract<'a> for ScenarioProgress {
    const TYPE_NAME: &'static str = "ScenarioProgress";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            multiplier_rewards: inst.get_array("multiplierRewards")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SResourceRewardMultiplier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SResourceRewardMultiplier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            faction_reward_tiers: inst.get_array("factionRewardTiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SScenarioProgressRewardsTiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SScenarioProgressRewardsTiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CompletionTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionTypeBase {
}

impl Pooled for CompletionTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.missionscenarios.completion_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.missionscenarios.completion_type_base }
}

impl<'a> Extract<'a> for CompletionTypeBase {
    const TYPE_NAME: &'static str = "CompletionTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

