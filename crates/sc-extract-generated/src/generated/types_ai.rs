// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `AIPerceptionProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPerceptionProfile {
    /// DCB field: `meterSettings` (Class)
    #[serde(default)]
    pub meter_settings: Option<Handle<SAIPerceptionMeterSettings>>,
    /// DCB field: `audioSettings` (Class)
    #[serde(default)]
    pub audio_settings: Option<Handle<SAIPerceptionAudioSettings>>,
    /// DCB field: `visualSettings` (Class)
    #[serde(default)]
    pub visual_settings: Option<Handle<SAIPerceptionVisualSettings>>,
}

impl Pooled for AIPerceptionProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiperception_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiperception_profile }
}

impl<'a> Extract<'a> for AIPerceptionProfile {
    const TYPE_NAME: &'static str = "AIPerceptionProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            meter_settings: match inst.get("meterSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAIPerceptionMeterSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAIPerceptionMeterSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_settings: match inst.get("audioSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAIPerceptionAudioSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAIPerceptionAudioSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            visual_settings: match inst.get("visualSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAIPerceptionVisualSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAIPerceptionVisualSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AIMercyTimerSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMercyTimerSettings {
    /// DCB field: `activationThreshold` (Single)
    #[serde(default)]
    pub activation_threshold: f32,
    /// DCB field: `coolDownTimeSeconds` (Single)
    #[serde(default)]
    pub cool_down_time_seconds: f32,
    /// DCB field: `durationSeconds` (Single)
    #[serde(default)]
    pub duration_seconds: f32,
}

impl Pooled for AIMercyTimerSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aimercy_timer_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aimercy_timer_settings }
}

impl<'a> Extract<'a> for AIMercyTimerSettings {
    const TYPE_NAME: &'static str = "AIMercyTimerSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            activation_threshold: inst.get_f32("activationThreshold").unwrap_or_default(),
            cool_down_time_seconds: inst.get_f32("coolDownTimeSeconds").unwrap_or_default(),
            duration_seconds: inst.get_f32("durationSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIVisualFieldParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIVisualFieldParams {
    /// DCB field: `sightRange` (Single)
    #[serde(default)]
    pub sight_range: f32,
    /// DCB field: `sixthSenseRange` (Single)
    #[serde(default)]
    pub sixth_sense_range: f32,
    /// DCB field: `clampDistanceForHorizontalFOV` (Single)
    #[serde(default)]
    pub clamp_distance_for_horizontal_fov: f32,
    /// DCB field: `clampDistanceForVerticalFOV` (Single)
    #[serde(default)]
    pub clamp_distance_for_vertical_fov: f32,
    /// DCB field: `FOVHorizontal` (Single)
    #[serde(default)]
    pub fovhorizontal: f32,
    /// DCB field: `FOVVertical` (Single)
    #[serde(default)]
    pub fovvertical: f32,
    /// DCB field: `PrimaryFOVHorizontal` (Single)
    #[serde(default)]
    pub primary_fovhorizontal: f32,
    /// DCB field: `PrimaryFOVVertical` (Single)
    #[serde(default)]
    pub primary_fovvertical: f32,
}

impl Pooled for AIVisualFieldParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aivisual_field_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aivisual_field_params }
}

impl<'a> Extract<'a> for AIVisualFieldParams {
    const TYPE_NAME: &'static str = "AIVisualFieldParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sight_range: inst.get_f32("sightRange").unwrap_or_default(),
            sixth_sense_range: inst.get_f32("sixthSenseRange").unwrap_or_default(),
            clamp_distance_for_horizontal_fov: inst.get_f32("clampDistanceForHorizontalFOV").unwrap_or_default(),
            clamp_distance_for_vertical_fov: inst.get_f32("clampDistanceForVerticalFOV").unwrap_or_default(),
            fovhorizontal: inst.get_f32("FOVHorizontal").unwrap_or_default(),
            fovvertical: inst.get_f32("FOVVertical").unwrap_or_default(),
            primary_fovhorizontal: inst.get_f32("PrimaryFOVHorizontal").unwrap_or_default(),
            primary_fovvertical: inst.get_f32("PrimaryFOVVertical").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIContextualVisualFieldProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIContextualVisualFieldProfile {
    /// DCB field: `defaultProfile` (Reference)
    #[serde(default)]
    pub default_profile: Option<CigGuid>,
    /// DCB field: `awarenessLevelOverrides` (Reference)
    #[serde(default)]
    pub awareness_level_overrides: Option<CigGuid>,
}

impl Pooled for AIContextualVisualFieldProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aicontextual_visual_field_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aicontextual_visual_field_profile }
}

impl<'a> Extract<'a> for AIContextualVisualFieldProfile {
    const TYPE_NAME: &'static str = "AIContextualVisualFieldProfile";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_profile: inst.get("defaultProfile").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            awareness_level_overrides: inst.get("awarenessLevelOverrides").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AIVisualFieldProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIVisualFieldProfile {
    /// DCB field: `profile` (Class)
    #[serde(default)]
    pub profile: Option<Handle<AIContextualVisualFieldProfile>>,
}

impl Pooled for AIVisualFieldProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aivisual_field_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aivisual_field_profile }
}

impl<'a> Extract<'a> for AIVisualFieldProfile {
    const TYPE_NAME: &'static str = "AIVisualFieldProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            profile: match inst.get("profile") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIContextualVisualFieldProfile>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIContextualVisualFieldProfile>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AIObservableFilterFlags`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIObservableFilterFlags {
    /// DCB field: `typeFlags` (EnumChoice (array))
    #[serde(default)]
    pub type_flags: Vec<String>,
    /// DCB field: `statusFlags` (EnumChoice (array))
    #[serde(default)]
    pub status_flags: Vec<String>,
}

impl Pooled for AIObservableFilterFlags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiobservable_filter_flags }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiobservable_filter_flags }
}

impl<'a> Extract<'a> for AIObservableFilterFlags {
    const TYPE_NAME: &'static str = "AIObservableFilterFlags";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            type_flags: inst.get_array("typeFlags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            status_flags: inst.get_array("statusFlags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIObservableFilters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIObservableFilters {
    /// DCB field: `typesAndStatus` (Reference)
    #[serde(default)]
    pub types_and_status: Option<CigGuid>,
}

impl Pooled for AIObservableFilters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiobservable_filters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiobservable_filters }
}

impl<'a> Extract<'a> for AIObservableFilters {
    const TYPE_NAME: &'static str = "AIObservableFilters";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types_and_status: inst.get("typesAndStatus").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AIObservableFiltersProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIObservableFiltersProfile {
    /// DCB field: `observableFilters` (Class (array))
    #[serde(default)]
    pub observable_filters: Vec<Handle<AIObservableFilters>>,
}

impl Pooled for AIObservableFiltersProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiobservable_filters_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiobservable_filters_profile }
}

impl<'a> Extract<'a> for AIObservableFiltersProfile {
    const TYPE_NAME: &'static str = "AIObservableFiltersProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            observable_filters: inst.get_array("observableFilters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIObservableFilters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIObservableFilters>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AITargetableSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITargetableSettings {
    /// DCB field: `suggestedAttackerCapacity` (Single)
    #[serde(default)]
    pub suggested_attacker_capacity: f32,
    /// DCB field: `maxIndividualIncomingMissileSize` (Int32)
    #[serde(default)]
    pub max_individual_incoming_missile_size: i32,
    /// DCB field: `maxTotalIncomingMissileSize` (Int32)
    #[serde(default)]
    pub max_total_incoming_missile_size: i32,
    /// DCB field: `maxAttackerShootingTokens` (Int32)
    #[serde(default)]
    pub max_attacker_shooting_tokens: i32,
    /// DCB field: `targetableByGrenadesCooldown` (Single)
    #[serde(default)]
    pub targetable_by_grenades_cooldown: f32,
}

impl Pooled for AITargetableSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aitargetable_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aitargetable_settings }
}

impl<'a> Extract<'a> for AITargetableSettings {
    const TYPE_NAME: &'static str = "AITargetableSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            suggested_attacker_capacity: inst.get_f32("suggestedAttackerCapacity").unwrap_or_default(),
            max_individual_incoming_missile_size: inst.get_i32("maxIndividualIncomingMissileSize").unwrap_or_default(),
            max_total_incoming_missile_size: inst.get_i32("maxTotalIncomingMissileSize").unwrap_or_default(),
            max_attacker_shooting_tokens: inst.get_i32("maxAttackerShootingTokens").unwrap_or_default(),
            targetable_by_grenades_cooldown: inst.get_f32("targetableByGrenadesCooldown").unwrap_or_default(),
        }
    }
}

/// DCB type: `AISpecialRangedAttackConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISpecialRangedAttackConfig {
    /// DCB field: `attackName` (String)
    #[serde(default)]
    pub attack_name: String,
    /// DCB field: `maxElevationDeg` (Single)
    #[serde(default)]
    pub max_elevation_deg: f32,
    /// DCB field: `verticalAttackFragmentTag` (String)
    #[serde(default)]
    pub vertical_attack_fragment_tag: String,
}

impl Pooled for AISpecialRangedAttackConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aispecial_ranged_attack_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aispecial_ranged_attack_config }
}

impl<'a> Extract<'a> for AISpecialRangedAttackConfig {
    const TYPE_NAME: &'static str = "AISpecialRangedAttackConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attack_name: inst.get_str("attackName").map(String::from).unwrap_or_default(),
            max_elevation_deg: inst.get_f32("maxElevationDeg").unwrap_or_default(),
            vertical_attack_fragment_tag: inst.get_str("verticalAttackFragmentTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AIAvailableSpecialRangedAttacksConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAvailableSpecialRangedAttacksConfig {
    /// DCB field: `availableSpecialRangedAttacks` (Reference (array))
    #[serde(default)]
    pub available_special_ranged_attacks: Vec<CigGuid>,
}

impl Pooled for AIAvailableSpecialRangedAttacksConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiavailable_special_ranged_attacks_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiavailable_special_ranged_attacks_config }
}

impl<'a> Extract<'a> for AIAvailableSpecialRangedAttacksConfig {
    const TYPE_NAME: &'static str = "AIAvailableSpecialRangedAttacksConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            available_special_ranged_attacks: inst.get_array("availableSpecialRangedAttacks")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIFireDisciplineSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFireDisciplineSettings {
    /// DCB field: `fireConeMinAngle` (Single)
    #[serde(default)]
    pub fire_cone_min_angle: f32,
    /// DCB field: `fireConeMaxAngle` (Single)
    #[serde(default)]
    pub fire_cone_max_angle: f32,
}

impl Pooled for AIFireDisciplineSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aifire_discipline_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aifire_discipline_settings }
}

impl<'a> Extract<'a> for AIFireDisciplineSettings {
    const TYPE_NAME: &'static str = "AIFireDisciplineSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fire_cone_min_angle: inst.get_f32("fireConeMinAngle").unwrap_or_default(),
            fire_cone_max_angle: inst.get_f32("fireConeMaxAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMotiveActionDetails`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotiveActionDetails {
    /// DCB field: `detailsId` (String)
    #[serde(default)]
    pub details_id: String,
}

impl Pooled for AIMotiveActionDetails {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aimotive_action_details }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aimotive_action_details }
}

impl<'a> Extract<'a> for AIMotiveActionDetails {
    const TYPE_NAME: &'static str = "AIMotiveActionDetails";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            details_id: inst.get_str("detailsId").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMotiveAction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotiveAction {
    /// DCB field: `id` (Guid)
    #[serde(default)]
    pub id: CigGuid,
    /// DCB field: `actionName` (String)
    #[serde(default)]
    pub action_name: String,
    /// DCB field: `conditions` (StrongPointer (array))
    #[serde(default)]
    pub conditions: Vec<Handle<AIMotiveCondition>>,
    /// DCB field: `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// DCB field: `actionDetails` (StrongPointer (array))
    #[serde(default)]
    pub action_details: Vec<Handle<AIMotiveActionDetails>>,
}

impl Pooled for AIMotiveAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aimotive_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aimotive_action }
}

impl<'a> Extract<'a> for AIMotiveAction {
    const TYPE_NAME: &'static str = "AIMotiveAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            action_name: inst.get_str("actionName").map(String::from).unwrap_or_default(),
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMotiveCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMotiveCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            priority: inst.get_i32("priority").unwrap_or_default(),
            action_details: inst.get_array("actionDetails")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMotiveActionDetails>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMotiveActionDetails>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMotiveCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotiveCondition {
    /// DCB field: `id` (Guid)
    #[serde(default)]
    pub id: CigGuid,
    /// DCB field: `interrupt` (Boolean)
    #[serde(default)]
    pub interrupt: bool,
    /// DCB field: `minSatisfactionDuration` (Single)
    #[serde(default)]
    pub min_satisfaction_duration: f32,
}

impl Pooled for AIMotiveCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aimotive_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aimotive_condition }
}

impl<'a> Extract<'a> for AIMotiveCondition {
    const TYPE_NAME: &'static str = "AIMotiveCondition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            interrupt: inst.get_bool("interrupt").unwrap_or_default(),
            min_satisfaction_duration: inst.get_f32("minSatisfactionDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMotive`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotive {
    /// DCB field: `id` (Guid)
    #[serde(default)]
    pub id: CigGuid,
    /// DCB field: `actionDefs` (Class (array))
    #[serde(default)]
    pub action_defs: Vec<Handle<AIMotiveAction>>,
}

impl Pooled for AIMotive {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aimotive }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aimotive }
}

impl<'a> Extract<'a> for AIMotive {
    const TYPE_NAME: &'static str = "AIMotive";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            action_defs: inst.get_array("actionDefs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMotiveAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMotiveAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMotiveList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMotiveList {
    /// DCB field: `motiveList` (StrongPointer (array))
    #[serde(default)]
    pub motive_list: Vec<Handle<AIMotive>>,
}

impl Pooled for AIMotiveList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aimotive_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aimotive_list }
}

impl<'a> Extract<'a> for AIMotiveList {
    const TYPE_NAME: &'static str = "AIMotiveList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            motive_list: inst.get_array("motiveList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMotive>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMotive>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProfile {
    /// DCB field: `character` (Class)
    #[serde(default)]
    pub character: Option<Handle<CharacterSkills>>,
    /// DCB field: `seatOperator` (Class)
    #[serde(default)]
    pub seat_operator: Option<Handle<SeatOperatorSkills>>,
    /// DCB field: `tacticSelectionScores` (Class)
    #[serde(default)]
    pub tactic_selection_scores: Option<Handle<TacticScoringProfile>>,
    /// DCB field: `sharedTacticParams` (Class)
    #[serde(default)]
    pub shared_tactic_params: Option<Handle<SharedTacticParams>>,
    /// DCB field: `shootingParams` (Class)
    #[serde(default)]
    pub shooting_params: Option<Handle<ShootingParams>>,
}

impl Pooled for AIProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiprofile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiprofile }
}

impl<'a> Extract<'a> for AIProfile {
    const TYPE_NAME: &'static str = "AIProfile";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            character: match inst.get("character") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CharacterSkills>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CharacterSkills>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            seat_operator: match inst.get("seatOperator") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SeatOperatorSkills>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SeatOperatorSkills>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tactic_selection_scores: match inst.get("tacticSelectionScores") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TacticScoringProfile>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TacticScoringProfile>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shared_tactic_params: match inst.get("sharedTacticParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SharedTacticParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SharedTacticParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shooting_params: match inst.get("shootingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ShootingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ShootingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Aiming`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aiming {
    /// DCB field: `accuracyConeAngle` (Single)
    #[serde(default)]
    pub accuracy_cone_angle: f32,
    /// DCB field: `maxTargetAngle` (Single)
    #[serde(default)]
    pub max_target_angle: f32,
    /// DCB field: `maxTargetAngleInfluence` (Single)
    #[serde(default)]
    pub max_target_angle_influence: f32,
    /// DCB field: `chanceOfHittingTarget` (Single)
    #[serde(default)]
    pub chance_of_hitting_target: f32,
    /// DCB field: `missileCooldownMin` (Single)
    #[serde(default)]
    pub missile_cooldown_min: f32,
    /// DCB field: `missileCooldownMax` (Single)
    #[serde(default)]
    pub missile_cooldown_max: f32,
    /// DCB field: `countermeasureReactionDelayMin` (Single)
    #[serde(default)]
    pub countermeasure_reaction_delay_min: f32,
    /// DCB field: `countermeasureReactionDelayMax` (Single)
    #[serde(default)]
    pub countermeasure_reaction_delay_max: f32,
    /// DCB field: `countermeasureCooldownMin` (Single)
    #[serde(default)]
    pub countermeasure_cooldown_min: f32,
    /// DCB field: `countermeasureCooldownMax` (Single)
    #[serde(default)]
    pub countermeasure_cooldown_max: f32,
    /// DCB field: `countermeasureLaunchChanceMin` (Single)
    #[serde(default)]
    pub countermeasure_launch_chance_min: f32,
    /// DCB field: `countermeasureLaunchChanceIncreaseRatio` (Single)
    #[serde(default)]
    pub countermeasure_launch_chance_increase_ratio: f32,
    /// DCB field: `flareBurstSizeMultiplierMin` (Single)
    #[serde(default)]
    pub flare_burst_size_multiplier_min: f32,
    /// DCB field: `flareBurstSizeMultiplierMax` (Single)
    #[serde(default)]
    pub flare_burst_size_multiplier_max: f32,
    /// DCB field: `defaultInAimAngleThreshold` (Single)
    #[serde(default)]
    pub default_in_aim_angle_threshold: f32,
    /// DCB field: `disciplinedInAimAngleThreshold` (Single)
    #[serde(default)]
    pub disciplined_in_aim_angle_threshold: f32,
    /// DCB field: `unDisciplinedInAimAngleThreshold` (Single)
    #[serde(default)]
    pub un_disciplined_in_aim_angle_threshold: f32,
}

impl Pooled for Aiming {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiming }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiming }
}

impl<'a> Extract<'a> for Aiming {
    const TYPE_NAME: &'static str = "Aiming";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            accuracy_cone_angle: inst.get_f32("accuracyConeAngle").unwrap_or_default(),
            max_target_angle: inst.get_f32("maxTargetAngle").unwrap_or_default(),
            max_target_angle_influence: inst.get_f32("maxTargetAngleInfluence").unwrap_or_default(),
            chance_of_hitting_target: inst.get_f32("chanceOfHittingTarget").unwrap_or_default(),
            missile_cooldown_min: inst.get_f32("missileCooldownMin").unwrap_or_default(),
            missile_cooldown_max: inst.get_f32("missileCooldownMax").unwrap_or_default(),
            countermeasure_reaction_delay_min: inst.get_f32("countermeasureReactionDelayMin").unwrap_or_default(),
            countermeasure_reaction_delay_max: inst.get_f32("countermeasureReactionDelayMax").unwrap_or_default(),
            countermeasure_cooldown_min: inst.get_f32("countermeasureCooldownMin").unwrap_or_default(),
            countermeasure_cooldown_max: inst.get_f32("countermeasureCooldownMax").unwrap_or_default(),
            countermeasure_launch_chance_min: inst.get_f32("countermeasureLaunchChanceMin").unwrap_or_default(),
            countermeasure_launch_chance_increase_ratio: inst.get_f32("countermeasureLaunchChanceIncreaseRatio").unwrap_or_default(),
            flare_burst_size_multiplier_min: inst.get_f32("flareBurstSizeMultiplierMin").unwrap_or_default(),
            flare_burst_size_multiplier_max: inst.get_f32("flareBurstSizeMultiplierMax").unwrap_or_default(),
            default_in_aim_angle_threshold: inst.get_f32("defaultInAimAngleThreshold").unwrap_or_default(),
            disciplined_in_aim_angle_threshold: inst.get_f32("disciplinedInAimAngleThreshold").unwrap_or_default(),
            un_disciplined_in_aim_angle_threshold: inst.get_f32("unDisciplinedInAimAngleThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `AITimeSinceTargetSeen`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITimeSinceTargetSeen {
    /// DCB field: `decayDelayTimeSeconds` (Single)
    #[serde(default)]
    pub decay_delay_time_seconds: f32,
    /// DCB field: `decayRate` (Single)
    #[serde(default)]
    pub decay_rate: f32,
    /// DCB field: `capTimeSeconds` (Single)
    #[serde(default)]
    pub cap_time_seconds: f32,
    /// DCB field: `accuracyOverTimeCurve` (Class)
    #[serde(default)]
    pub accuracy_over_time_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for AITimeSinceTargetSeen {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aitime_since_target_seen }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aitime_since_target_seen }
}

impl<'a> Extract<'a> for AITimeSinceTargetSeen {
    const TYPE_NAME: &'static str = "AITimeSinceTargetSeen";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            decay_delay_time_seconds: inst.get_f32("decayDelayTimeSeconds").unwrap_or_default(),
            decay_rate: inst.get_f32("decayRate").unwrap_or_default(),
            cap_time_seconds: inst.get_f32("capTimeSeconds").unwrap_or_default(),
            accuracy_over_time_curve: match inst.get("accuracyOverTimeCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AIFormulaScoreModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFormulaScoreModifiers {
    /// DCB field: `exponent` (Single)
    #[serde(default)]
    pub exponent: f32,
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
}

impl Pooled for AIFormulaScoreModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiformula_score_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiformula_score_modifiers }
}

impl<'a> Extract<'a> for AIFormulaScoreModifiers {
    const TYPE_NAME: &'static str = "AIFormulaScoreModifiers";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            exponent: inst.get_f32("exponent").unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

/// DCB type: `AITargetingFormulaSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITargetingFormulaSettings {
    /// DCB field: `onFootRangeMultiplier` (Single)
    #[serde(default)]
    pub on_foot_range_multiplier: f32,
    /// DCB field: `inVehicleRangeMultiplier` (Single)
    #[serde(default)]
    pub in_vehicle_range_multiplier: f32,
    /// DCB field: `selfDefenceMaxHealthMultiplier` (Single)
    #[serde(default)]
    pub self_defence_max_health_multiplier: f32,
    /// DCB field: `protectedMaxHealthMultiplier` (Single)
    #[serde(default)]
    pub protected_max_health_multiplier: f32,
    /// DCB field: `recentDamageDecayFactorPerSecond` (Single)
    #[serde(default)]
    pub recent_damage_decay_factor_per_second: f32,
    /// DCB field: `attackerCapacityScore` (Class)
    #[serde(default)]
    pub attacker_capacity_score: Option<Handle<AIFormulaScoreModifiers>>,
    /// DCB field: `distanceScore` (Class)
    #[serde(default)]
    pub distance_score: Option<Handle<AIFormulaScoreModifiers>>,
    /// DCB field: `selfDefenceScore` (Class)
    #[serde(default)]
    pub self_defence_score: Option<Handle<AIFormulaScoreModifiers>>,
    /// DCB field: `protectionScore` (Class)
    #[serde(default)]
    pub protection_score: Option<Handle<AIFormulaScoreModifiers>>,
}

impl Pooled for AITargetingFormulaSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aitargeting_formula_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aitargeting_formula_settings }
}

impl<'a> Extract<'a> for AITargetingFormulaSettings {
    const TYPE_NAME: &'static str = "AITargetingFormulaSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            on_foot_range_multiplier: inst.get_f32("onFootRangeMultiplier").unwrap_or_default(),
            in_vehicle_range_multiplier: inst.get_f32("inVehicleRangeMultiplier").unwrap_or_default(),
            self_defence_max_health_multiplier: inst.get_f32("selfDefenceMaxHealthMultiplier").unwrap_or_default(),
            protected_max_health_multiplier: inst.get_f32("protectedMaxHealthMultiplier").unwrap_or_default(),
            recent_damage_decay_factor_per_second: inst.get_f32("recentDamageDecayFactorPerSecond").unwrap_or_default(),
            attacker_capacity_score: match inst.get("attackerCapacityScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance_score: match inst.get("distanceScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            self_defence_score: match inst.get("selfDefenceScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            protection_score: match inst.get("protectionScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AIWaveCollection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIWaveCollection {
    /// DCB field: `waves` (Class (array))
    #[serde(default)]
    pub waves: Vec<Handle<AIWave>>,
}

impl Pooled for AIWaveCollection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiwave_collection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiwave_collection }
}

impl<'a> Extract<'a> for AIWaveCollection {
    const TYPE_NAME: &'static str = "AIWaveCollection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            waves: inst.get_array("waves")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIWave>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIWave>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIWave`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIWave {
    /// DCB field: `id` (Int32)
    #[serde(default)]
    pub id: i32,
    /// DCB field: `textId` (String)
    #[serde(default)]
    pub text_id: String,
    /// DCB field: `staggerTime` (Single)
    #[serde(default)]
    pub stagger_time: f32,
    /// DCB field: `members` (Class (array))
    #[serde(default)]
    pub members: Vec<Handle<AIWaveMember>>,
}

impl Pooled for AIWave {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiwave }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiwave }
}

impl<'a> Extract<'a> for AIWave {
    const TYPE_NAME: &'static str = "AIWave";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_i32("id").unwrap_or_default(),
            text_id: inst.get_str("textId").map(String::from).unwrap_or_default(),
            stagger_time: inst.get_f32("staggerTime").unwrap_or_default(),
            members: inst.get_array("members")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIWaveMember>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIWaveMember>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AIWaveMember`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIWaveMember {
    /// DCB field: `archetype` (String)
    #[serde(default)]
    pub archetype: String,
    /// DCB field: `entityClassDefinition` (Reference)
    #[serde(default)]
    pub entity_class_definition: Option<CigGuid>,
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `vehicleName` (String)
    #[serde(default)]
    pub vehicle_name: String,
    /// DCB field: `amount` (Int32)
    #[serde(default)]
    pub amount: i32,
    /// DCB field: `minAmount` (Int32)
    #[serde(default)]
    pub min_amount: i32,
    /// DCB field: `midAmount` (Int32)
    #[serde(default)]
    pub mid_amount: i32,
    /// DCB field: `maxAmount` (Int32)
    #[serde(default)]
    pub max_amount: i32,
    /// DCB field: `crewManifestOverride` (Reference)
    #[serde(default)]
    pub crew_manifest_override: Option<CigGuid>,
    /// DCB field: `skillsetOverride` (Reference)
    #[serde(default)]
    pub skillset_override: Option<CigGuid>,
    /// DCB field: `cargoManifests` (Reference (array))
    #[serde(default)]
    pub cargo_manifests: Vec<CigGuid>,
    /// DCB field: `aiDamageModifiersOverride` (Reference)
    #[serde(default)]
    pub ai_damage_modifiers_override: Option<CigGuid>,
}

impl Pooled for AIWaveMember {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aiwave_member }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aiwave_member }
}

impl<'a> Extract<'a> for AIWaveMember {
    const TYPE_NAME: &'static str = "AIWaveMember";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            archetype: inst.get_str("archetype").map(String::from).unwrap_or_default(),
            entity_class_definition: inst.get("entityClassDefinition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            vehicle_name: inst.get_str("vehicleName").map(String::from).unwrap_or_default(),
            amount: inst.get_i32("amount").unwrap_or_default(),
            min_amount: inst.get_i32("minAmount").unwrap_or_default(),
            mid_amount: inst.get_i32("midAmount").unwrap_or_default(),
            max_amount: inst.get_i32("maxAmount").unwrap_or_default(),
            crew_manifest_override: inst.get("crewManifestOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            skillset_override: inst.get("skillsetOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            cargo_manifests: inst.get_array("cargoManifests")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            ai_damage_modifiers_override: inst.get("aiDamageModifiersOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AIMeleeAttack`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMeleeAttack {
    /// DCB field: `attackType` (EnumChoice)
    #[serde(default)]
    pub attack_type: String,
    /// DCB field: `globalFragmentTags` (String)
    #[serde(default)]
    pub global_fragment_tags: String,
    /// DCB field: `minDistanceToTarget2d` (Single)
    #[serde(default)]
    pub min_distance_to_target2d: f32,
    /// DCB field: `maxDistanceToTarget2d` (Single)
    #[serde(default)]
    pub max_distance_to_target2d: f32,
    /// DCB field: `minRelativeTargetHeight` (Single)
    #[serde(default)]
    pub min_relative_target_height: f32,
    /// DCB field: `maxRelativeTargetHeight` (Single)
    #[serde(default)]
    pub max_relative_target_height: f32,
    /// DCB field: `maxAngleToTarget` (Single)
    #[serde(default)]
    pub max_angle_to_target: f32,
    /// DCB field: `ignoreAttackObstructionClearance` (Boolean)
    #[serde(default)]
    pub ignore_attack_obstruction_clearance: bool,
}

impl Pooled for AIMeleeAttack {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aimelee_attack }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aimelee_attack }
}

impl<'a> Extract<'a> for AIMeleeAttack {
    const TYPE_NAME: &'static str = "AIMeleeAttack";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attack_type: inst.get_str("attackType").map(String::from).unwrap_or_default(),
            global_fragment_tags: inst.get_str("globalFragmentTags").map(String::from).unwrap_or_default(),
            min_distance_to_target2d: inst.get_f32("minDistanceToTarget2d").unwrap_or_default(),
            max_distance_to_target2d: inst.get_f32("maxDistanceToTarget2d").unwrap_or_default(),
            min_relative_target_height: inst.get_f32("minRelativeTargetHeight").unwrap_or_default(),
            max_relative_target_height: inst.get_f32("maxRelativeTargetHeight").unwrap_or_default(),
            max_angle_to_target: inst.get_f32("maxAngleToTarget").unwrap_or_default(),
            ignore_attack_obstruction_clearance: inst.get_bool("ignoreAttackObstructionClearance").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMeleeCombatConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMeleeCombatConfig {
    /// DCB field: `meleeAttackCategoryInfo` (Class (array))
    #[serde(default)]
    pub melee_attack_category_info: Vec<Handle<MeleeAttackCategoryInfo>>,
    /// DCB field: `attackCategoryParams` (StrongPointer (array))
    #[serde(default)]
    pub attack_category_params: Vec<Handle<AttackCategoryParamsBase>>,
    /// DCB field: `meleeAttacks` (StrongPointer (array))
    #[serde(default)]
    pub melee_attacks: Vec<Handle<AIMeleeAttack>>,
    /// DCB field: `combos` (Class (array))
    #[serde(default)]
    pub combos: Vec<Handle<MeleeAttackCombo>>,
}

impl Pooled for AIMeleeCombatConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ai.aimelee_combat_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ai.aimelee_combat_config }
}

impl<'a> Extract<'a> for AIMeleeCombatConfig {
    const TYPE_NAME: &'static str = "AIMeleeCombatConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            melee_attack_category_info: inst.get_array("meleeAttackCategoryInfo")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeAttackCategoryInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeAttackCategoryInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            attack_category_params: inst.get_array("attackCategoryParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AttackCategoryParamsBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AttackCategoryParamsBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            melee_attacks: inst.get_array("meleeAttacks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMeleeAttack>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMeleeAttack>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            combos: inst.get_array("combos")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeAttackCombo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeAttackCombo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

