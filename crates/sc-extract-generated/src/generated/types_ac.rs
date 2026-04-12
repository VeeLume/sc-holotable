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

/// DCB type: `ActivityBehaviorRequestCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityBehaviorRequestCondition {
}

impl Pooled for ActivityBehaviorRequestCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.activity_behavior_request_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.activity_behavior_request_condition }
}

impl<'a> Extract<'a> for ActivityBehaviorRequestCondition {
    const TYPE_NAME: &'static str = "ActivityBehaviorRequestCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ActivityBehaviorRequest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityBehaviorRequest {
    /// DCB field: `requestName` (String)
    #[serde(default)]
    pub request_name: String,
    /// DCB field: `condition` (StrongPointer)
    #[serde(default)]
    pub condition: Option<Handle<ActivityBehaviorRequestCondition>>,
}

impl Pooled for ActivityBehaviorRequest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.activity_behavior_request }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.activity_behavior_request }
}

impl<'a> Extract<'a> for ActivityBehaviorRequest {
    const TYPE_NAME: &'static str = "ActivityBehaviorRequest";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            request_name: inst.get_str("requestName").map(String::from).unwrap_or_default(),
            condition: match inst.get("condition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActivityBehaviorRequestCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActivityBehaviorRequestCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActivityDataRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityDataRecord {
    /// DCB field: `activityBehaviorRequests` (StrongPointer (array))
    #[serde(default)]
    pub activity_behavior_requests: Vec<Handle<ActivityBehaviorRequest>>,
}

impl Pooled for ActivityDataRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.activity_data_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.activity_data_record }
}

impl<'a> Extract<'a> for ActivityDataRecord {
    const TYPE_NAME: &'static str = "ActivityDataRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            activity_behavior_requests: inst.get_array("activityBehaviorRequests")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActivityBehaviorRequest>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActivityBehaviorRequest>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActivityData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityData {
    /// DCB field: `dataRecords` (StrongPointer (array))
    #[serde(default)]
    pub data_records: Vec<Handle<ActivityDataRecord>>,
}

impl Pooled for ActivityData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.activity_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.activity_data }
}

impl<'a> Extract<'a> for ActivityData {
    const TYPE_NAME: &'static str = "ActivityData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            data_records: inst.get_array("dataRecords")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActivityDataRecord>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActivityDataRecord>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorAbilityComponent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorAbilityComponent {
    /// DCB field: `abilityDefinitions` (Class)
    #[serde(default)]
    pub ability_definitions: Option<Handle<AbilityDefinition>>,
}

impl Pooled for ActorAbilityComponent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_ability_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_ability_component }
}

impl<'a> Extract<'a> for ActorAbilityComponent {
    const TYPE_NAME: &'static str = "ActorAbilityComponent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ability_definitions: match inst.get("abilityDefinitions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AbilityDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AbilityDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorDuckingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorDuckingParams {
    /// DCB field: `heightRefBone` (String)
    #[serde(default)]
    pub height_ref_bone: String,
    /// DCB field: `hipBone` (String)
    #[serde(default)]
    pub hip_bone: String,
    /// DCB field: `headLowpassBone` (String)
    #[serde(default)]
    pub head_lowpass_bone: String,
    /// DCB field: `refBoneDesiredClearance` (Single)
    #[serde(default)]
    pub ref_bone_desired_clearance: f32,
    /// DCB field: `collisionCheckOrigin` (Class)
    #[serde(default)]
    pub collision_check_origin: Option<Handle<Vec3>>,
    /// DCB field: `collisionCheckDistance` (Single)
    #[serde(default)]
    pub collision_check_distance: f32,
    /// DCB field: `minDuckHeight` (Single)
    #[serde(default)]
    pub min_duck_height: f32,
    /// DCB field: `noWeaponPoseParams` (Class)
    #[serde(default)]
    pub no_weapon_pose_params: Option<Handle<DuckPose>>,
    /// DCB field: `stockedPoseParams` (Class)
    #[serde(default)]
    pub stocked_pose_params: Option<Handle<DuckPose>>,
    /// DCB field: `spine` (Class (array))
    #[serde(default)]
    pub spine: Vec<Handle<SpineBone>>,
}

impl Pooled for ActorDuckingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_ducking_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_ducking_params }
}

impl<'a> Extract<'a> for ActorDuckingParams {
    const TYPE_NAME: &'static str = "ActorDuckingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            height_ref_bone: inst.get_str("heightRefBone").map(String::from).unwrap_or_default(),
            hip_bone: inst.get_str("hipBone").map(String::from).unwrap_or_default(),
            head_lowpass_bone: inst.get_str("headLowpassBone").map(String::from).unwrap_or_default(),
            ref_bone_desired_clearance: inst.get_f32("refBoneDesiredClearance").unwrap_or_default(),
            collision_check_origin: match inst.get("collisionCheckOrigin") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            collision_check_distance: inst.get_f32("collisionCheckDistance").unwrap_or_default(),
            min_duck_height: inst.get_f32("minDuckHeight").unwrap_or_default(),
            no_weapon_pose_params: match inst.get("noWeaponPoseParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DuckPose>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DuckPose>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stocked_pose_params: match inst.get("stockedPoseParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DuckPose>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DuckPose>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spine: inst.get_array("spine")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SpineBone>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SpineBone>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorFrostedVisorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorFrostedVisorParams {
    /// DCB field: `suitTemperatureMaxFrost` (Single)
    #[serde(default)]
    pub suit_temperature_max_frost: f32,
    /// DCB field: `suitTemperatureMinFrost` (Single)
    #[serde(default)]
    pub suit_temperature_min_frost: f32,
    /// DCB field: `visorWipeRemainingFrost` (Single)
    #[serde(default)]
    pub visor_wipe_remaining_frost: f32,
    /// DCB field: `visorWipeTime` (Single)
    #[serde(default)]
    pub visor_wipe_time: f32,
    /// DCB field: `visorWipeFrostRegenDelayMin` (Single)
    #[serde(default)]
    pub visor_wipe_frost_regen_delay_min: f32,
    /// DCB field: `visorWipeFrostRegenDelayMax` (Single)
    #[serde(default)]
    pub visor_wipe_frost_regen_delay_max: f32,
    /// DCB field: `visorIsFrostedThreshold` (Single)
    #[serde(default)]
    pub visor_is_frosted_threshold: f32,
}

impl Pooled for ActorFrostedVisorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_frosted_visor_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_frosted_visor_params }
}

impl<'a> Extract<'a> for ActorFrostedVisorParams {
    const TYPE_NAME: &'static str = "ActorFrostedVisorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            suit_temperature_max_frost: inst.get_f32("suitTemperatureMaxFrost").unwrap_or_default(),
            suit_temperature_min_frost: inst.get_f32("suitTemperatureMinFrost").unwrap_or_default(),
            visor_wipe_remaining_frost: inst.get_f32("visorWipeRemainingFrost").unwrap_or_default(),
            visor_wipe_time: inst.get_f32("visorWipeTime").unwrap_or_default(),
            visor_wipe_frost_regen_delay_min: inst.get_f32("visorWipeFrostRegenDelayMin").unwrap_or_default(),
            visor_wipe_frost_regen_delay_max: inst.get_f32("visorWipeFrostRegenDelayMax").unwrap_or_default(),
            visor_is_frosted_threshold: inst.get_f32("visorIsFrostedThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorEnvironmentComponent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorEnvironmentComponent {
    /// DCB field: `temperatureParams` (Class)
    #[serde(default)]
    pub temperature_params: Option<Handle<EnvironmentTemperatureParams>>,
    /// DCB field: `frostedVisorParams` (Class)
    #[serde(default)]
    pub frosted_visor_params: Option<Handle<ActorFrostedVisorParams>>,
    /// DCB field: `geigerCounterSensitivityFactor` (Single)
    #[serde(default)]
    pub geiger_counter_sensitivity_factor: f32,
    /// DCB field: `geigerCounterMinimumFrequencyToActivate` (Single)
    #[serde(default)]
    pub geiger_counter_minimum_frequency_to_activate: f32,
    /// DCB field: `geigerCounterFrequencyLimit` (Single)
    #[serde(default)]
    pub geiger_counter_frequency_limit: f32,
}

impl Pooled for ActorEnvironmentComponent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_environment_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_environment_component }
}

impl<'a> Extract<'a> for ActorEnvironmentComponent {
    const TYPE_NAME: &'static str = "ActorEnvironmentComponent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            temperature_params: match inst.get("temperatureParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EnvironmentTemperatureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EnvironmentTemperatureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            frosted_visor_params: match inst.get("frostedVisorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorFrostedVisorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorFrostedVisorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            geiger_counter_sensitivity_factor: inst.get_f32("geigerCounterSensitivityFactor").unwrap_or_default(),
            geiger_counter_minimum_frequency_to_activate: inst.get_f32("geigerCounterMinimumFrequencyToActivate").unwrap_or_default(),
            geiger_counter_frequency_limit: inst.get_f32("geigerCounterFrequencyLimit").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorGForceComponent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceComponent {
    /// DCB field: `gForceParams` (Class)
    #[serde(default)]
    pub g_force_params: Option<Handle<GForceParams>>,
}

impl Pooled for ActorGForceComponent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_gforce_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_gforce_component }
}

impl<'a> Extract<'a> for ActorGForceComponent {
    const TYPE_NAME: &'static str = "ActorGForceComponent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            g_force_params: match inst.get("gForceParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GForceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GForceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorGForceHeadBobFakeVelocityGData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceHeadBobFakeVelocityGData {
    /// DCB field: `maxLinearSpeed` (Single)
    #[serde(default)]
    pub max_linear_speed: f32,
    /// DCB field: `mappedMaxFakeGs` (Single)
    #[serde(default)]
    pub mapped_max_fake_gs: f32,
    /// DCB field: `enabledForSpaceships` (Boolean)
    #[serde(default)]
    pub enabled_for_spaceships: bool,
    /// DCB field: `enabledForGravlev` (Boolean)
    #[serde(default)]
    pub enabled_for_gravlev: bool,
    /// DCB field: `enabledForGroundVehicles` (Boolean)
    #[serde(default)]
    pub enabled_for_ground_vehicles: bool,
    /// DCB field: `enabledForBoats` (Boolean)
    #[serde(default)]
    pub enabled_for_boats: bool,
    /// DCB field: `mapping` (Class)
    #[serde(default)]
    pub mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for ActorGForceHeadBobFakeVelocityGData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_gforce_head_bob_fake_velocity_gdata }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_gforce_head_bob_fake_velocity_gdata }
}

impl<'a> Extract<'a> for ActorGForceHeadBobFakeVelocityGData {
    const TYPE_NAME: &'static str = "ActorGForceHeadBobFakeVelocityGData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_linear_speed: inst.get_f32("maxLinearSpeed").unwrap_or_default(),
            mapped_max_fake_gs: inst.get_f32("mappedMaxFakeGs").unwrap_or_default(),
            enabled_for_spaceships: inst.get_bool("enabledForSpaceships").unwrap_or_default(),
            enabled_for_gravlev: inst.get_bool("enabledForGravlev").unwrap_or_default(),
            enabled_for_ground_vehicles: inst.get_bool("enabledForGroundVehicles").unwrap_or_default(),
            enabled_for_boats: inst.get_bool("enabledForBoats").unwrap_or_default(),
            mapping: match inst.get("mapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorGForceHeadBobData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceHeadBobData {
    /// DCB field: `maxGforce` (Class)
    #[serde(default)]
    pub max_gforce: Option<Handle<Vec3>>,
    /// DCB field: `undampedFrequencyTranslation` (Class)
    #[serde(default)]
    pub undamped_frequency_translation: Option<Handle<Vec3>>,
    /// DCB field: `dampingRatioTranslation` (Class)
    #[serde(default)]
    pub damping_ratio_translation: Option<Handle<Vec3>>,
    /// DCB field: `maxHeadTranslation` (Class)
    #[serde(default)]
    pub max_head_translation: Option<Handle<Vec3>>,
    /// DCB field: `translationSmoothing` (Single)
    #[serde(default)]
    pub translation_smoothing: f32,
    /// DCB field: `undampedFrequencyRotation` (Class)
    #[serde(default)]
    pub undamped_frequency_rotation: Option<Handle<Vec3>>,
    /// DCB field: `dampingRatioRotation` (Class)
    #[serde(default)]
    pub damping_ratio_rotation: Option<Handle<Vec3>>,
    /// DCB field: `maxHeadRotation` (Class)
    #[serde(default)]
    pub max_head_rotation: Option<Handle<Ang3>>,
    /// DCB field: `maxHeadRotationMagLaunch` (Class)
    #[serde(default)]
    pub max_head_rotation_mag_launch: Option<Handle<Ang3>>,
    /// DCB field: `rotationSmoothing` (Single)
    #[serde(default)]
    pub rotation_smoothing: f32,
    /// DCB field: `headTranslationLimitsPositive` (Class)
    #[serde(default)]
    pub head_translation_limits_positive: Option<Handle<Vec3>>,
    /// DCB field: `headTranslationLimitsNegative` (Class)
    #[serde(default)]
    pub head_translation_limits_negative: Option<Handle<Vec3>>,
    /// DCB field: `rotationalAccelerationInputModifier` (Single)
    #[serde(default)]
    pub rotational_acceleration_input_modifier: f32,
    /// DCB field: `jumpDriveFlightModifierTranslation` (Single)
    #[serde(default)]
    pub jump_drive_flight_modifier_translation: f32,
    /// DCB field: `jumpDriveFlightModifierRotation` (Single)
    #[serde(default)]
    pub jump_drive_flight_modifier_rotation: f32,
    /// DCB field: `fakeVelocityGs` (Class)
    #[serde(default)]
    pub fake_velocity_gs: Option<Handle<ActorGForceHeadBobFakeVelocityGData>>,
    /// DCB field: `quantumAccelerationMaxGs` (Single)
    #[serde(default)]
    pub quantum_acceleration_max_gs: f32,
}

impl Pooled for ActorGForceHeadBobData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_gforce_head_bob_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_gforce_head_bob_data }
}

impl<'a> Extract<'a> for ActorGForceHeadBobData {
    const TYPE_NAME: &'static str = "ActorGForceHeadBobData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_gforce: match inst.get("maxGforce") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            undamped_frequency_translation: match inst.get("undampedFrequencyTranslation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damping_ratio_translation: match inst.get("dampingRatioTranslation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_head_translation: match inst.get("maxHeadTranslation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            translation_smoothing: inst.get_f32("translationSmoothing").unwrap_or_default(),
            undamped_frequency_rotation: match inst.get("undampedFrequencyRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damping_ratio_rotation: match inst.get("dampingRatioRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_head_rotation: match inst.get("maxHeadRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_head_rotation_mag_launch: match inst.get("maxHeadRotationMagLaunch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_smoothing: inst.get_f32("rotationSmoothing").unwrap_or_default(),
            head_translation_limits_positive: match inst.get("headTranslationLimitsPositive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_translation_limits_negative: match inst.get("headTranslationLimitsNegative") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotational_acceleration_input_modifier: inst.get_f32("rotationalAccelerationInputModifier").unwrap_or_default(),
            jump_drive_flight_modifier_translation: inst.get_f32("jumpDriveFlightModifierTranslation").unwrap_or_default(),
            jump_drive_flight_modifier_rotation: inst.get_f32("jumpDriveFlightModifierRotation").unwrap_or_default(),
            fake_velocity_gs: match inst.get("fakeVelocityGs") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorGForceHeadBobFakeVelocityGData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorGForceHeadBobFakeVelocityGData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            quantum_acceleration_max_gs: inst.get_f32("quantumAccelerationMaxGs").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorGForceHeadBob`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceHeadBob {
    /// DCB field: `headBobData` (Class)
    #[serde(default)]
    pub head_bob_data: Option<Handle<ActorGForceHeadBobData>>,
    /// DCB field: `headBobDataLegacy` (Class)
    #[serde(default)]
    pub head_bob_data_legacy: Option<Handle<ActorGForceHeadBobData>>,
}

impl Pooled for ActorGForceHeadBob {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_gforce_head_bob }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_gforce_head_bob }
}

impl<'a> Extract<'a> for ActorGForceHeadBob {
    const TYPE_NAME: &'static str = "ActorGForceHeadBob";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            head_bob_data: match inst.get("headBobData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorGForceHeadBobData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorGForceHeadBobData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_bob_data_legacy: match inst.get("headBobDataLegacy") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorGForceHeadBobData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorGForceHeadBobData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorGForceCameraEffectsData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceCameraEffectsData {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `effectsOnlyAppliedForward` (Boolean)
    #[serde(default)]
    pub effects_only_applied_forward: bool,
    /// DCB field: `gForceAngleOuter` (Single)
    #[serde(default)]
    pub g_force_angle_outer: f32,
    /// DCB field: `gForceAngleInner` (Single)
    #[serde(default)]
    pub g_force_angle_inner: f32,
    /// DCB field: `gForceMin` (Single)
    #[serde(default)]
    pub g_force_min: f32,
    /// DCB field: `gForceMax` (Single)
    #[serde(default)]
    pub g_force_max: f32,
    /// DCB field: `gForceFOV` (Single)
    #[serde(default)]
    pub g_force_fov: f32,
    /// DCB field: `focusDistance` (Single)
    #[serde(default)]
    pub focus_distance: f32,
    /// DCB field: `afterburnerEffectMinRatio` (Single)
    #[serde(default)]
    pub afterburner_effect_min_ratio: f32,
    /// DCB field: `genericModifiers` (Class)
    #[serde(default)]
    pub generic_modifiers: Option<Handle<CameraEffectsModifiers>>,
}

impl Pooled for ActorGForceCameraEffectsData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_gforce_camera_effects_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_gforce_camera_effects_data }
}

impl<'a> Extract<'a> for ActorGForceCameraEffectsData {
    const TYPE_NAME: &'static str = "ActorGForceCameraEffectsData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            effects_only_applied_forward: inst.get_bool("effectsOnlyAppliedForward").unwrap_or_default(),
            g_force_angle_outer: inst.get_f32("gForceAngleOuter").unwrap_or_default(),
            g_force_angle_inner: inst.get_f32("gForceAngleInner").unwrap_or_default(),
            g_force_min: inst.get_f32("gForceMin").unwrap_or_default(),
            g_force_max: inst.get_f32("gForceMax").unwrap_or_default(),
            g_force_fov: inst.get_f32("gForceFOV").unwrap_or_default(),
            focus_distance: inst.get_f32("focusDistance").unwrap_or_default(),
            afterburner_effect_min_ratio: inst.get_f32("afterburnerEffectMinRatio").unwrap_or_default(),
            generic_modifiers: match inst.get("genericModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraEffectsModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraEffectsModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorGForceCameraEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorGForceCameraEffects {
    /// DCB field: `cameraEffects` (Class)
    #[serde(default)]
    pub camera_effects: Option<Handle<ActorGForceCameraEffectsData>>,
}

impl Pooled for ActorGForceCameraEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_gforce_camera_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_gforce_camera_effects }
}

impl<'a> Extract<'a> for ActorGForceCameraEffects {
    const TYPE_NAME: &'static str = "ActorGForceCameraEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            camera_effects: match inst.get("cameraEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorGForceCameraEffectsData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorGForceCameraEffectsData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorLocomotionRotateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionRotateParams {
    /// DCB field: `maxDeltaAngleRateNormal` (Single)
    #[serde(default)]
    pub max_delta_angle_rate_normal: f32,
    /// DCB field: `maxDeltaAngleRateCatchup` (Single)
    #[serde(default)]
    pub max_delta_angle_rate_catchup: f32,
    /// DCB field: `maxDeltaAngleRateCombat` (Single)
    #[serde(default)]
    pub max_delta_angle_rate_combat: f32,
    /// DCB field: `maxDeltaAngleRateExactPositioning` (Single)
    #[serde(default)]
    pub max_delta_angle_rate_exact_positioning: f32,
    /// DCB field: `turnSpeedCurveDefault` (Reference)
    #[serde(default)]
    pub turn_speed_curve_default: Option<CigGuid>,
}

impl Pooled for ActorLocomotionRotateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_locomotion_rotate_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_locomotion_rotate_params }
}

impl<'a> Extract<'a> for ActorLocomotionRotateParams {
    const TYPE_NAME: &'static str = "ActorLocomotionRotateParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_delta_angle_rate_normal: inst.get_f32("maxDeltaAngleRateNormal").unwrap_or_default(),
            max_delta_angle_rate_catchup: inst.get_f32("maxDeltaAngleRateCatchup").unwrap_or_default(),
            max_delta_angle_rate_combat: inst.get_f32("maxDeltaAngleRateCombat").unwrap_or_default(),
            max_delta_angle_rate_exact_positioning: inst.get_f32("maxDeltaAngleRateExactPositioning").unwrap_or_default(),
            turn_speed_curve_default: inst.get("turnSpeedCurveDefault").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ActorLocomotionTurnOnSpotParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionTurnOnSpotParams {
    /// DCB field: `minAngle` (Single)
    #[serde(default)]
    pub min_angle: f32,
    /// DCB field: `minAngleForTurnWithoutDelay` (Single)
    #[serde(default)]
    pub min_angle_for_turn_without_delay: f32,
    /// DCB field: `maxDelayTime` (Single)
    #[serde(default)]
    pub max_delay_time: f32,
    /// DCB field: `requeueNormTime` (Single)
    #[serde(default)]
    pub requeue_norm_time: f32,
}

impl Pooled for ActorLocomotionTurnOnSpotParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_locomotion_turn_on_spot_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_locomotion_turn_on_spot_params }
}

impl<'a> Extract<'a> for ActorLocomotionTurnOnSpotParams {
    const TYPE_NAME: &'static str = "ActorLocomotionTurnOnSpotParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_angle: inst.get_f32("minAngle").unwrap_or_default(),
            min_angle_for_turn_without_delay: inst.get_f32("minAngleForTurnWithoutDelay").unwrap_or_default(),
            max_delay_time: inst.get_f32("maxDelayTime").unwrap_or_default(),
            requeue_norm_time: inst.get_f32("requeueNormTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLocomotionSharpTurnParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionSharpTurnParams {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `enableForWalking` (Boolean)
    #[serde(default)]
    pub enable_for_walking: bool,
    /// DCB field: `enableForRunning` (Boolean)
    #[serde(default)]
    pub enable_for_running: bool,
    /// DCB field: `enableForSprinting` (Boolean)
    #[serde(default)]
    pub enable_for_sprinting: bool,
    /// DCB field: `enableForWeapon` (Boolean)
    #[serde(default)]
    pub enable_for_weapon: bool,
    /// DCB field: `enableForWeaponWalking` (Boolean)
    #[serde(default)]
    pub enable_for_weapon_walking: bool,
    /// DCB field: `enableForWeaponRunning` (Boolean)
    #[serde(default)]
    pub enable_for_weapon_running: bool,
    /// DCB field: `enableForWeaponSprinting` (Boolean)
    #[serde(default)]
    pub enable_for_weapon_sprinting: bool,
    /// DCB field: `enableForNW` (Boolean)
    #[serde(default)]
    pub enable_for_nw: bool,
    /// DCB field: `enableForNoWeaponWalking` (Boolean)
    #[serde(default)]
    pub enable_for_no_weapon_walking: bool,
    /// DCB field: `enableForNoWeaponRunning` (Boolean)
    #[serde(default)]
    pub enable_for_no_weapon_running: bool,
    /// DCB field: `enableForNoWeaponSprinting` (Boolean)
    #[serde(default)]
    pub enable_for_no_weapon_sprinting: bool,
    /// DCB field: `enableOnGaitChange` (Boolean)
    #[serde(default)]
    pub enable_on_gait_change: bool,
    /// DCB field: `minActivationAngleDefault` (Single)
    #[serde(default)]
    pub min_activation_angle_default: f32,
    /// DCB field: `minActivationAngleWalk` (Single)
    #[serde(default)]
    pub min_activation_angle_walk: f32,
}

impl Pooled for ActorLocomotionSharpTurnParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_locomotion_sharp_turn_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_locomotion_sharp_turn_params }
}

impl<'a> Extract<'a> for ActorLocomotionSharpTurnParams {
    const TYPE_NAME: &'static str = "ActorLocomotionSharpTurnParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            enable_for_walking: inst.get_bool("enableForWalking").unwrap_or_default(),
            enable_for_running: inst.get_bool("enableForRunning").unwrap_or_default(),
            enable_for_sprinting: inst.get_bool("enableForSprinting").unwrap_or_default(),
            enable_for_weapon: inst.get_bool("enableForWeapon").unwrap_or_default(),
            enable_for_weapon_walking: inst.get_bool("enableForWeaponWalking").unwrap_or_default(),
            enable_for_weapon_running: inst.get_bool("enableForWeaponRunning").unwrap_or_default(),
            enable_for_weapon_sprinting: inst.get_bool("enableForWeaponSprinting").unwrap_or_default(),
            enable_for_nw: inst.get_bool("enableForNW").unwrap_or_default(),
            enable_for_no_weapon_walking: inst.get_bool("enableForNoWeaponWalking").unwrap_or_default(),
            enable_for_no_weapon_running: inst.get_bool("enableForNoWeaponRunning").unwrap_or_default(),
            enable_for_no_weapon_sprinting: inst.get_bool("enableForNoWeaponSprinting").unwrap_or_default(),
            enable_on_gait_change: inst.get_bool("enableOnGaitChange").unwrap_or_default(),
            min_activation_angle_default: inst.get_f32("minActivationAngleDefault").unwrap_or_default(),
            min_activation_angle_walk: inst.get_f32("minActivationAngleWalk").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLocomotionAvoidanceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionAvoidanceParams {
    /// DCB field: `enableForceStrafe` (Boolean)
    #[serde(default)]
    pub enable_force_strafe: bool,
    /// DCB field: `enableAvoidanceTwist` (Boolean)
    #[serde(default)]
    pub enable_avoidance_twist: bool,
    /// DCB field: `twistAllowedHalfFOV` (Single)
    #[serde(default)]
    pub twist_allowed_half_fov: f32,
    /// DCB field: `minTwistDistance` (Single)
    #[serde(default)]
    pub min_twist_distance: f32,
    /// DCB field: `maxTwistDistance` (Single)
    #[serde(default)]
    pub max_twist_distance: f32,
    /// DCB field: `blendWeightAtMinTwist` (Single)
    #[serde(default)]
    pub blend_weight_at_min_twist: f32,
    /// DCB field: `blendWeightAtMaxTwist` (Single)
    #[serde(default)]
    pub blend_weight_at_max_twist: f32,
    /// DCB field: `twistBlendTime` (Single)
    #[serde(default)]
    pub twist_blend_time: f32,
    /// DCB field: `twistBlendOutTime` (Single)
    #[serde(default)]
    pub twist_blend_out_time: f32,
    /// DCB field: `maxDistanceFromPath` (Single)
    #[serde(default)]
    pub max_distance_from_path: f32,
}

impl Pooled for ActorLocomotionAvoidanceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_locomotion_avoidance_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_locomotion_avoidance_params }
}

impl<'a> Extract<'a> for ActorLocomotionAvoidanceParams {
    const TYPE_NAME: &'static str = "ActorLocomotionAvoidanceParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable_force_strafe: inst.get_bool("enableForceStrafe").unwrap_or_default(),
            enable_avoidance_twist: inst.get_bool("enableAvoidanceTwist").unwrap_or_default(),
            twist_allowed_half_fov: inst.get_f32("twistAllowedHalfFOV").unwrap_or_default(),
            min_twist_distance: inst.get_f32("minTwistDistance").unwrap_or_default(),
            max_twist_distance: inst.get_f32("maxTwistDistance").unwrap_or_default(),
            blend_weight_at_min_twist: inst.get_f32("blendWeightAtMinTwist").unwrap_or_default(),
            blend_weight_at_max_twist: inst.get_f32("blendWeightAtMaxTwist").unwrap_or_default(),
            twist_blend_time: inst.get_f32("twistBlendTime").unwrap_or_default(),
            twist_blend_out_time: inst.get_f32("twistBlendOutTime").unwrap_or_default(),
            max_distance_from_path: inst.get_f32("maxDistanceFromPath").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLocomotionPersonality`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionPersonality {
    /// DCB field: `locomotionStyleMannequinTag` (String)
    #[serde(default)]
    pub locomotion_style_mannequin_tag: String,
    /// DCB field: `basePlayerMovementMannequinAction` (Reference)
    #[serde(default)]
    pub base_player_movement_mannequin_action: Option<CigGuid>,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<ActorLocomotionRotateParams>>,
    /// DCB field: `turnOnSpotNormal` (Class)
    #[serde(default)]
    pub turn_on_spot_normal: Option<Handle<ActorLocomotionTurnOnSpotParams>>,
    /// DCB field: `turnOnSpotCombat` (Class)
    #[serde(default)]
    pub turn_on_spot_combat: Option<Handle<ActorLocomotionTurnOnSpotParams>>,
    /// DCB field: `sharpTurns` (Class)
    #[serde(default)]
    pub sharp_turns: Option<Handle<ActorLocomotionSharpTurnParams>>,
    /// DCB field: `avoidance` (Class)
    #[serde(default)]
    pub avoidance: Option<Handle<ActorLocomotionAvoidanceParams>>,
    /// DCB field: `fidget` (Reference)
    #[serde(default)]
    pub fidget: Option<CigGuid>,
    /// DCB field: `slope` (Class)
    #[serde(default)]
    pub slope: Option<Handle<SActorLocomotionFeatureDef_Slope>>,
    /// DCB field: `submergedCreatureConfig` (StrongPointer)
    #[serde(default)]
    pub submerged_creature_config: Option<Handle<SActorLocomotionSubmergedCreatureParams>>,
}

impl Pooled for ActorLocomotionPersonality {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_locomotion_personality }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_locomotion_personality }
}

impl<'a> Extract<'a> for ActorLocomotionPersonality {
    const TYPE_NAME: &'static str = "ActorLocomotionPersonality";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            locomotion_style_mannequin_tag: inst.get_str("locomotionStyleMannequinTag").map(String::from).unwrap_or_default(),
            base_player_movement_mannequin_action: inst.get("basePlayerMovementMannequinAction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionRotateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionRotateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turn_on_spot_normal: match inst.get("turnOnSpotNormal") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionTurnOnSpotParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionTurnOnSpotParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turn_on_spot_combat: match inst.get("turnOnSpotCombat") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionTurnOnSpotParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionTurnOnSpotParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sharp_turns: match inst.get("sharpTurns") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionSharpTurnParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionSharpTurnParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            avoidance: match inst.get("avoidance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionAvoidanceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionAvoidanceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fidget: inst.get("fidget").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            slope: match inst.get("slope") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorLocomotionFeatureDef_Slope>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorLocomotionFeatureDef_Slope>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            submerged_creature_config: match inst.get("submergedCreatureConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorLocomotionSubmergedCreatureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorLocomotionSubmergedCreatureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorBaseStanceMovementModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorBaseStanceMovementModifiers {
    /// DCB field: `strafeSpeedScale` (Single)
    #[serde(default)]
    pub strafe_speed_scale: f32,
    /// DCB field: `strafeForwardSpeedScale` (Single)
    #[serde(default)]
    pub strafe_forward_speed_scale: f32,
    /// DCB field: `backwardSpeedScale` (Single)
    #[serde(default)]
    pub backward_speed_scale: f32,
    /// DCB field: `turnSpeedScale` (Single)
    #[serde(default)]
    pub turn_speed_scale: f32,
    /// DCB field: `turnMaxAngleVelocity` (Single)
    #[serde(default)]
    pub turn_max_angle_velocity: f32,
    /// DCB field: `groundSlopeSpeedScale` (Single)
    #[serde(default)]
    pub ground_slope_speed_scale: f32,
}

impl Pooled for ActorBaseStanceMovementModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_base_stance_movement_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_base_stance_movement_modifiers }
}

impl<'a> Extract<'a> for ActorBaseStanceMovementModifiers {
    const TYPE_NAME: &'static str = "ActorBaseStanceMovementModifiers";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            strafe_speed_scale: inst.get_f32("strafeSpeedScale").unwrap_or_default(),
            strafe_forward_speed_scale: inst.get_f32("strafeForwardSpeedScale").unwrap_or_default(),
            backward_speed_scale: inst.get_f32("backwardSpeedScale").unwrap_or_default(),
            turn_speed_scale: inst.get_f32("turnSpeedScale").unwrap_or_default(),
            turn_max_angle_velocity: inst.get_f32("turnMaxAngleVelocity").unwrap_or_default(),
            ground_slope_speed_scale: inst.get_f32("groundSlopeSpeedScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorBaseMovementModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorBaseMovementModifiers {
    /// DCB field: `standing` (Class)
    #[serde(default)]
    pub standing: Option<Handle<ActorBaseStanceMovementModifiers>>,
    /// DCB field: `crouched` (Class)
    #[serde(default)]
    pub crouched: Option<Handle<ActorBaseStanceMovementModifiers>>,
    /// DCB field: `prone` (Class)
    #[serde(default)]
    pub prone: Option<Handle<ActorBaseStanceMovementModifiers>>,
    /// DCB field: `swimming` (Class)
    #[serde(default)]
    pub swimming: Option<Handle<ActorBaseStanceMovementModifiers>>,
    /// DCB field: `zeroGSurfaceTraversal` (Class)
    #[serde(default)]
    pub zero_gsurface_traversal: Option<Handle<ActorBaseStanceMovementModifiers>>,
    /// DCB field: `other` (Class)
    #[serde(default)]
    pub other: Option<Handle<ActorBaseStanceMovementModifiers>>,
}

impl Pooled for ActorBaseMovementModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_base_movement_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_base_movement_modifiers }
}

impl<'a> Extract<'a> for ActorBaseMovementModifiers {
    const TYPE_NAME: &'static str = "ActorBaseMovementModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            standing: match inst.get("standing") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            crouched: match inst.get("crouched") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            prone: match inst.get("prone") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            swimming: match inst.get("swimming") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            zero_gsurface_traversal: match inst.get("zeroGSurfaceTraversal") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            other: match inst.get("other") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorBaseStanceMovementModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorForceMovementModifierConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorForceMovementModifierConfig {
    /// DCB field: `forceSmoothDuration` (Single)
    #[serde(default)]
    pub force_smooth_duration: f32,
    /// DCB field: `minAngleForSlowdown` (Single)
    #[serde(default)]
    pub min_angle_for_slowdown: f32,
    /// DCB field: `maxAngleForSlowdown` (Single)
    #[serde(default)]
    pub max_angle_for_slowdown: f32,
    /// DCB field: `minMagnitudeForSlowdown` (Single)
    #[serde(default)]
    pub min_magnitude_for_slowdown: f32,
    /// DCB field: `maxMagnitudeForSlowdown` (Single)
    #[serde(default)]
    pub max_magnitude_for_slowdown: f32,
    /// DCB field: `forceSlowdownSpeedMultiplier` (Single)
    #[serde(default)]
    pub force_slowdown_speed_multiplier: f32,
    /// DCB field: `minMagnitudeToDisableSprint` (Single)
    #[serde(default)]
    pub min_magnitude_to_disable_sprint: f32,
    /// DCB field: `maxAngleToDisableSprint` (Single)
    #[serde(default)]
    pub max_angle_to_disable_sprint: f32,
}

impl Pooled for ActorForceMovementModifierConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_force_movement_modifier_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_force_movement_modifier_config }
}

impl<'a> Extract<'a> for ActorForceMovementModifierConfig {
    const TYPE_NAME: &'static str = "ActorForceMovementModifierConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            force_smooth_duration: inst.get_f32("forceSmoothDuration").unwrap_or_default(),
            min_angle_for_slowdown: inst.get_f32("minAngleForSlowdown").unwrap_or_default(),
            max_angle_for_slowdown: inst.get_f32("maxAngleForSlowdown").unwrap_or_default(),
            min_magnitude_for_slowdown: inst.get_f32("minMagnitudeForSlowdown").unwrap_or_default(),
            max_magnitude_for_slowdown: inst.get_f32("maxMagnitudeForSlowdown").unwrap_or_default(),
            force_slowdown_speed_multiplier: inst.get_f32("forceSlowdownSpeedMultiplier").unwrap_or_default(),
            min_magnitude_to_disable_sprint: inst.get_f32("minMagnitudeToDisableSprint").unwrap_or_default(),
            max_angle_to_disable_sprint: inst.get_f32("maxAngleToDisableSprint").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorExternalForceMovementModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorExternalForceMovementModifiers {
    /// DCB field: `gForce` (Class)
    #[serde(default)]
    pub g_force: Option<Handle<ActorForceMovementModifierConfig>>,
    /// DCB field: `wind` (Class)
    #[serde(default)]
    pub wind: Option<Handle<ActorForceMovementModifierConfig>>,
}

impl Pooled for ActorExternalForceMovementModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_external_force_movement_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_external_force_movement_modifiers }
}

impl<'a> Extract<'a> for ActorExternalForceMovementModifiers {
    const TYPE_NAME: &'static str = "ActorExternalForceMovementModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            g_force: match inst.get("gForce") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorForceMovementModifierConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorForceMovementModifierConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wind: match inst.get("wind") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorForceMovementModifierConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorForceMovementModifierConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorEnvironmentalModifierConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorEnvironmentalModifierConfig {
    /// DCB field: `minAffectedRatioForSlowdown` (Single)
    #[serde(default)]
    pub min_affected_ratio_for_slowdown: f32,
    /// DCB field: `environmentalSlowdownSpeedMultiplier` (Single)
    #[serde(default)]
    pub environmental_slowdown_speed_multiplier: f32,
    /// DCB field: `minAffectedRatioToDisableSprint` (Single)
    #[serde(default)]
    pub min_affected_ratio_to_disable_sprint: f32,
}

impl Pooled for ActorEnvironmentalModifierConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_environmental_modifier_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_environmental_modifier_config }
}

impl<'a> Extract<'a> for ActorEnvironmentalModifierConfig {
    const TYPE_NAME: &'static str = "ActorEnvironmentalModifierConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_affected_ratio_for_slowdown: inst.get_f32("minAffectedRatioForSlowdown").unwrap_or_default(),
            environmental_slowdown_speed_multiplier: inst.get_f32("environmentalSlowdownSpeedMultiplier").unwrap_or_default(),
            min_affected_ratio_to_disable_sprint: inst.get_f32("minAffectedRatioToDisableSprint").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorEnvironmentalMovementModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorEnvironmentalMovementModifiers {
    /// DCB field: `water` (Class)
    #[serde(default)]
    pub water: Option<Handle<ActorEnvironmentalModifierConfig>>,
}

impl Pooled for ActorEnvironmentalMovementModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_environmental_movement_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_environmental_movement_modifiers }
}

impl<'a> Extract<'a> for ActorEnvironmentalMovementModifiers {
    const TYPE_NAME: &'static str = "ActorEnvironmentalMovementModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            water: match inst.get("water") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorEnvironmentalModifierConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorEnvironmentalModifierConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorMovementModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorMovementModifiers {
    /// DCB field: `baseModifiers` (Class)
    #[serde(default)]
    pub base_modifiers: Option<Handle<ActorBaseMovementModifiers>>,
    /// DCB field: `externalForceModifiers` (Class)
    #[serde(default)]
    pub external_force_modifiers: Option<Handle<ActorExternalForceMovementModifiers>>,
    /// DCB field: `environmentalModifiers` (Class)
    #[serde(default)]
    pub environmental_modifiers: Option<Handle<ActorEnvironmentalMovementModifiers>>,
}

impl Pooled for ActorMovementModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_movement_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_movement_modifiers }
}

impl<'a> Extract<'a> for ActorMovementModifiers {
    const TYPE_NAME: &'static str = "ActorMovementModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_modifiers: match inst.get("baseModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorBaseMovementModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorBaseMovementModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            external_force_modifiers: match inst.get("externalForceModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorExternalForceMovementModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorExternalForceMovementModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            environmental_modifiers: match inst.get("environmentalModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorEnvironmentalMovementModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorEnvironmentalMovementModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorMovementSetTransition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorMovementSetTransition {
    /// DCB field: `movementSet` (EnumChoice)
    #[serde(default)]
    pub movement_set: String,
    /// DCB field: `condition` (EnumChoice)
    #[serde(default)]
    pub condition: String,
    /// DCB field: `startDelay` (Single)
    #[serde(default)]
    pub start_delay: f32,
    /// DCB field: `endDelay` (Single)
    #[serde(default)]
    pub end_delay: f32,
}

impl Pooled for ActorMovementSetTransition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_movement_set_transition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_movement_set_transition }
}

impl<'a> Extract<'a> for ActorMovementSetTransition {
    const TYPE_NAME: &'static str = "ActorMovementSetTransition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            movement_set: inst.get_str("movementSet").map(String::from).unwrap_or_default(),
            condition: inst.get_str("condition").map(String::from).unwrap_or_default(),
            start_delay: inst.get_f32("startDelay").unwrap_or_default(),
            end_delay: inst.get_f32("endDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorMovementSetsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorMovementSetsConfig {
    /// DCB field: `transitions` (Class (array))
    #[serde(default)]
    pub transitions: Vec<Handle<ActorMovementSetTransition>>,
}

impl Pooled for ActorMovementSetsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_movement_sets_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_movement_sets_config }
}

impl<'a> Extract<'a> for ActorMovementSetsConfig {
    const TYPE_NAME: &'static str = "ActorMovementSetsConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            transitions: inst.get_array("transitions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorMovementSetTransition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorMovementSetTransition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorProceduralRecoilSetup`
///
/// Inherits from: `ActorStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorProceduralRecoilSetup {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// DCB field: `actorProceduralRecoilModifiers` (Reference)
    #[serde(default)]
    pub actor_procedural_recoil_modifiers: Option<CigGuid>,
}

impl Pooled for ActorProceduralRecoilSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_procedural_recoil_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_procedural_recoil_setup }
}

impl<'a> Extract<'a> for ActorProceduralRecoilSetup {
    const TYPE_NAME: &'static str = "ActorProceduralRecoilSetup";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filter_name: inst.get_str("filterName").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            filter_by_motion_speed: inst.get_str("filterByMotionSpeed").map(String::from).unwrap_or_default(),
            filter_by_pose_state: inst.get_str("filterByPoseState").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_lean_state: inst.get_str("filterByLeanState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
            filter_by_character_type: inst.get_str("filterByCharacterType").map(String::from).unwrap_or_default(),
            filter_by_restrained_state: inst.get_str("filterByRestrainedState").map(String::from).unwrap_or_default(),
            filter_by_player_camera: inst.get_str("filterByPlayerCamera").map(String::from).unwrap_or_default(),
            filter_by_aiming_restriction: inst.get_str("filterByAimingRestriction").map(String::from).unwrap_or_default(),
            actor_procedural_recoil_modifiers: inst.get("actorProceduralRecoilModifiers").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ActorProceduralRecoilConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorProceduralRecoilConfig {
    /// DCB field: `actorProceduralRecoilSetup` (Class (array))
    #[serde(default)]
    pub actor_procedural_recoil_setup: Vec<Handle<ActorProceduralRecoilSetup>>,
}

impl Pooled for ActorProceduralRecoilConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_procedural_recoil_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_procedural_recoil_config }
}

impl<'a> Extract<'a> for ActorProceduralRecoilConfig {
    const TYPE_NAME: &'static str = "ActorProceduralRecoilConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actor_procedural_recoil_setup: inst.get_array("actorProceduralRecoilSetup")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorProceduralRecoilSetup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorProceduralRecoilSetup>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorProceduralRecoilModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorProceduralRecoilModifiers {
    /// DCB field: `actorProceduralHandsRecoilModifiers` (Class)
    #[serde(default)]
    pub actor_procedural_hands_recoil_modifiers: Option<Handle<SActorProceduralHandsRecoilModifiers>>,
    /// DCB field: `actorProceduralAimRecoilModifiers` (Class)
    #[serde(default)]
    pub actor_procedural_aim_recoil_modifiers: Option<Handle<SActorProceduralAimRecoilModifiers>>,
    /// DCB field: `actorProceduralBodyRecoilModifiers` (Class)
    #[serde(default)]
    pub actor_procedural_body_recoil_modifiers: Option<Handle<SActorProceduralBodyRecoilModifiers>>,
    /// DCB field: `actorProceduralHeadRecoilModifiers` (Class)
    #[serde(default)]
    pub actor_procedural_head_recoil_modifiers: Option<Handle<SActorProceduralHeadRecoilModifiers>>,
}

impl Pooled for ActorProceduralRecoilModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_procedural_recoil_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_procedural_recoil_modifiers }
}

impl<'a> Extract<'a> for ActorProceduralRecoilModifiers {
    const TYPE_NAME: &'static str = "ActorProceduralRecoilModifiers";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actor_procedural_hands_recoil_modifiers: match inst.get("actorProceduralHandsRecoilModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralHandsRecoilModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralHandsRecoilModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_procedural_aim_recoil_modifiers: match inst.get("actorProceduralAimRecoilModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralAimRecoilModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralAimRecoilModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_procedural_body_recoil_modifiers: match inst.get("actorProceduralBodyRecoilModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralBodyRecoilModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralBodyRecoilModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_procedural_head_recoil_modifiers: match inst.get("actorProceduralHeadRecoilModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralHeadRecoilModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralHeadRecoilModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorFootJointPairDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorFootJointPairDef {
    /// DCB field: `leftFootJoint` (String)
    #[serde(default)]
    pub left_foot_joint: String,
    /// DCB field: `rightFootJoint` (String)
    #[serde(default)]
    pub right_foot_joint: String,
}

impl Pooled for ActorFootJointPairDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_foot_joint_pair_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_foot_joint_pair_def }
}

impl<'a> Extract<'a> for ActorFootJointPairDef {
    const TYPE_NAME: &'static str = "ActorFootJointPairDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            left_foot_joint: inst.get_str("leftFootJoint").map(String::from).unwrap_or_default(),
            right_foot_joint: inst.get_str("rightFootJoint").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorMeleeDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorMeleeDef {
    /// DCB field: `headJoint` (String)
    #[serde(default)]
    pub head_joint: String,
}

impl Pooled for ActorMeleeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_melee_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_melee_def }
}

impl<'a> Extract<'a> for ActorMeleeDef {
    const TYPE_NAME: &'static str = "ActorMeleeDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            head_joint: inst.get_str("headJoint").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorSkeletonConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSkeletonConfig {
    /// DCB field: `locomotionAnimSyncConfig` (Class)
    #[serde(default)]
    pub locomotion_anim_sync_config: Option<Handle<LocomotionAnimSyncConfig>>,
    /// DCB field: `ragdollRecoveryConfig` (Class)
    #[serde(default)]
    pub ragdoll_recovery_config: Option<Handle<RagdollRecoveryConfig>>,
    /// DCB field: `estimatedCyclePhaseFootConfig` (Class)
    #[serde(default)]
    pub estimated_cycle_phase_foot_config: Option<Handle<ActorFootJointPairDef>>,
    /// DCB field: `preciseCyclePhaseFootConfig` (Class)
    #[serde(default)]
    pub precise_cycle_phase_foot_config: Option<Handle<ActorFootJointPairDef>>,
    /// DCB field: `meleeConfig` (Class)
    #[serde(default)]
    pub melee_config: Option<Handle<ActorMeleeDef>>,
}

impl Pooled for ActorSkeletonConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_skeleton_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_skeleton_config }
}

impl<'a> Extract<'a> for ActorSkeletonConfig {
    const TYPE_NAME: &'static str = "ActorSkeletonConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            locomotion_anim_sync_config: match inst.get("locomotionAnimSyncConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LocomotionAnimSyncConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LocomotionAnimSyncConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ragdoll_recovery_config: match inst.get("ragdollRecoveryConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RagdollRecoveryConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RagdollRecoveryConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            estimated_cycle_phase_foot_config: match inst.get("estimatedCyclePhaseFootConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorFootJointPairDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorFootJointPairDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            precise_cycle_phase_foot_config: match inst.get("preciseCyclePhaseFootConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorFootJointPairDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorFootJointPairDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            melee_config: match inst.get("meleeConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorMeleeDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorMeleeDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorSlidingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSlidingParams {
    /// DCB field: `maxSlideTime` (Single)
    #[serde(default)]
    pub max_slide_time: f32,
    /// DCB field: `minSlideStartSpeed` (Single)
    #[serde(default)]
    pub min_slide_start_speed: f32,
    /// DCB field: `minDistanceToObstacleForSlide` (Single)
    #[serde(default)]
    pub min_distance_to_obstacle_for_slide: f32,
    /// DCB field: `slideDeceleration` (Single)
    #[serde(default)]
    pub slide_deceleration: f32,
    /// DCB field: `minSlideStopSpeed` (Single)
    #[serde(default)]
    pub min_slide_stop_speed: f32,
    /// DCB field: `yawBreakAngle` (Single)
    #[serde(default)]
    pub yaw_break_angle: f32,
    /// DCB field: `slideInertia` (Single)
    #[serde(default)]
    pub slide_inertia: f32,
    /// DCB field: `slideDropDuration` (Single)
    #[serde(default)]
    pub slide_drop_duration: f32,
    /// DCB field: `useCurveForSlideSpeed` (Boolean)
    #[serde(default)]
    pub use_curve_for_slide_speed: bool,
    /// DCB field: `slideSpeedCurve` (Class)
    #[serde(default)]
    pub slide_speed_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `speedCameraEffectsRecord` (Reference)
    #[serde(default)]
    pub speed_camera_effects_record: Option<CigGuid>,
}

impl Pooled for ActorSlidingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_sliding_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_sliding_params }
}

impl<'a> Extract<'a> for ActorSlidingParams {
    const TYPE_NAME: &'static str = "ActorSlidingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_slide_time: inst.get_f32("maxSlideTime").unwrap_or_default(),
            min_slide_start_speed: inst.get_f32("minSlideStartSpeed").unwrap_or_default(),
            min_distance_to_obstacle_for_slide: inst.get_f32("minDistanceToObstacleForSlide").unwrap_or_default(),
            slide_deceleration: inst.get_f32("slideDeceleration").unwrap_or_default(),
            min_slide_stop_speed: inst.get_f32("minSlideStopSpeed").unwrap_or_default(),
            yaw_break_angle: inst.get_f32("yawBreakAngle").unwrap_or_default(),
            slide_inertia: inst.get_f32("slideInertia").unwrap_or_default(),
            slide_drop_duration: inst.get_f32("slideDropDuration").unwrap_or_default(),
            use_curve_for_slide_speed: inst.get_bool("useCurveForSlideSpeed").unwrap_or_default(),
            slide_speed_curve: match inst.get("slideSpeedCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            speed_camera_effects_record: inst.get("speedCameraEffectsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ActorSpeedCameraEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSpeedCameraEffects {
    /// DCB field: `speedBoostMin` (Single)
    #[serde(default)]
    pub speed_boost_min: f32,
    /// DCB field: `speedBoostMax` (Single)
    #[serde(default)]
    pub speed_boost_max: f32,
    /// DCB field: `speedBoostFOV` (Single)
    #[serde(default)]
    pub speed_boost_fov: f32,
    /// DCB field: `focusDistance` (Single)
    #[serde(default)]
    pub focus_distance: f32,
    /// DCB field: `genericModifiers` (Class)
    #[serde(default)]
    pub generic_modifiers: Option<Handle<CameraEffectsModifiers>>,
}

impl Pooled for ActorSpeedCameraEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_speed_camera_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_speed_camera_effects }
}

impl<'a> Extract<'a> for ActorSpeedCameraEffects {
    const TYPE_NAME: &'static str = "ActorSpeedCameraEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            speed_boost_min: inst.get_f32("speedBoostMin").unwrap_or_default(),
            speed_boost_max: inst.get_f32("speedBoostMax").unwrap_or_default(),
            speed_boost_fov: inst.get_f32("speedBoostFOV").unwrap_or_default(),
            focus_distance: inst.get_f32("focusDistance").unwrap_or_default(),
            generic_modifiers: match inst.get("genericModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraEffectsModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraEffectsModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActionStaminaCosts`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionStaminaCosts {
    /// DCB field: `actionCategory` (EnumChoice)
    #[serde(default)]
    pub action_category: String,
    /// DCB field: `instantStaminaCost` (StrongPointer)
    #[serde(default)]
    pub instant_stamina_cost: Option<Handle<StaminaCost>>,
    /// DCB field: `blockedStaminaCost` (StrongPointer)
    #[serde(default)]
    pub blocked_stamina_cost: Option<Handle<StaminaCost>>,
    /// DCB field: `staminaToStart` (Single)
    #[serde(default)]
    pub stamina_to_start: f32,
}

impl Pooled for ActionStaminaCosts {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.action_stamina_costs }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.action_stamina_costs }
}

impl<'a> Extract<'a> for ActionStaminaCosts {
    const TYPE_NAME: &'static str = "ActionStaminaCosts";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            action_category: inst.get_str("actionCategory").map(String::from).unwrap_or_default(),
            instant_stamina_cost: match inst.get("instantStaminaCost") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StaminaCost>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StaminaCost>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blocked_stamina_cost: match inst.get("blockedStaminaCost") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StaminaCost>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StaminaCost>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stamina_to_start: inst.get_f32("staminaToStart").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStaminaComponent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStaminaComponent {
    /// DCB field: `oxygenGasParams` (Class)
    #[serde(default)]
    pub oxygen_gas_params: Option<Handle<BreathableOxygenParams>>,
    /// DCB field: `breathableGasesParams` (Class (array))
    #[serde(default)]
    pub breathable_gases_params: Vec<Handle<BreathableGasParams>>,
    /// DCB field: `staminaCostParams` (Class)
    #[serde(default)]
    pub stamina_cost_params: Option<Handle<StaminaCostParams>>,
    /// DCB field: `hudFeedbackParams` (Class)
    #[serde(default)]
    pub hud_feedback_params: Option<Handle<HudFeedbackParams>>,
    /// DCB field: `breathingHelper` (Class)
    #[serde(default)]
    pub breathing_helper: Option<Handle<BreathingHelperParams>>,
    /// DCB field: `npcBreathingParams` (StrongPointer)
    #[serde(default)]
    pub npc_breathing_params: Option<Handle<NpcBreathingParams>>,
}

impl Pooled for ActorStaminaComponent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_stamina_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_stamina_component }
}

impl<'a> Extract<'a> for ActorStaminaComponent {
    const TYPE_NAME: &'static str = "ActorStaminaComponent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            oxygen_gas_params: match inst.get("oxygenGasParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BreathableOxygenParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BreathableOxygenParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            breathable_gases_params: inst.get_array("breathableGasesParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BreathableGasParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BreathableGasParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            stamina_cost_params: match inst.get("staminaCostParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StaminaCostParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StaminaCostParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hud_feedback_params: match inst.get("hudFeedbackParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HudFeedbackParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HudFeedbackParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            breathing_helper: match inst.get("breathingHelper") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BreathingHelperParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BreathingHelperParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_breathing_params: match inst.get("npcBreathingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<NpcBreathingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<NpcBreathingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStanceSpeeds`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStanceSpeeds {
    /// DCB field: `defaultSpeed` (Single)
    #[serde(default)]
    pub default_speed: f32,
    /// DCB field: `walkSlowSpeed` (Single)
    #[serde(default)]
    pub walk_slow_speed: f32,
    /// DCB field: `walkMidSpeed` (Single)
    #[serde(default)]
    pub walk_mid_speed: f32,
    /// DCB field: `walkFastSpeed` (Single)
    #[serde(default)]
    pub walk_fast_speed: f32,
    /// DCB field: `runSlowSpeed` (Single)
    #[serde(default)]
    pub run_slow_speed: f32,
    /// DCB field: `runFastSpeed` (Single)
    #[serde(default)]
    pub run_fast_speed: f32,
    /// DCB field: `sprintSpeed` (Single)
    #[serde(default)]
    pub sprint_speed: f32,
    /// DCB field: `greenZoneWalkSpeed` (Single)
    #[serde(default)]
    pub green_zone_walk_speed: f32,
    /// DCB field: `greenZoneSprintSpeed` (Single)
    #[serde(default)]
    pub green_zone_sprint_speed: f32,
    /// DCB field: `aimDownSightSpeed` (Single)
    #[serde(default)]
    pub aim_down_sight_speed: f32,
    /// DCB field: `leanSpeed` (Single)
    #[serde(default)]
    pub lean_speed: f32,
    /// DCB field: `conversationSpeed` (Single)
    #[serde(default)]
    pub conversation_speed: f32,
    /// DCB field: `defaultLinearAcceleration` (Single)
    #[serde(default)]
    pub default_linear_acceleration: f32,
    /// DCB field: `defaultRotationSpeed` (Single)
    #[serde(default)]
    pub default_rotation_speed: f32,
    /// DCB field: `defaultRotationSmoothDuration` (Single)
    #[serde(default)]
    pub default_rotation_smooth_duration: f32,
}

impl Pooled for ActorStanceSpeeds {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_stance_speeds }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_stance_speeds }
}

impl<'a> Extract<'a> for ActorStanceSpeeds {
    const TYPE_NAME: &'static str = "ActorStanceSpeeds";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_speed: inst.get_f32("defaultSpeed").unwrap_or_default(),
            walk_slow_speed: inst.get_f32("walkSlowSpeed").unwrap_or_default(),
            walk_mid_speed: inst.get_f32("walkMidSpeed").unwrap_or_default(),
            walk_fast_speed: inst.get_f32("walkFastSpeed").unwrap_or_default(),
            run_slow_speed: inst.get_f32("runSlowSpeed").unwrap_or_default(),
            run_fast_speed: inst.get_f32("runFastSpeed").unwrap_or_default(),
            sprint_speed: inst.get_f32("sprintSpeed").unwrap_or_default(),
            green_zone_walk_speed: inst.get_f32("greenZoneWalkSpeed").unwrap_or_default(),
            green_zone_sprint_speed: inst.get_f32("greenZoneSprintSpeed").unwrap_or_default(),
            aim_down_sight_speed: inst.get_f32("aimDownSightSpeed").unwrap_or_default(),
            lean_speed: inst.get_f32("leanSpeed").unwrap_or_default(),
            conversation_speed: inst.get_f32("conversationSpeed").unwrap_or_default(),
            default_linear_acceleration: inst.get_f32("defaultLinearAcceleration").unwrap_or_default(),
            default_rotation_speed: inst.get_f32("defaultRotationSpeed").unwrap_or_default(),
            default_rotation_smooth_duration: inst.get_f32("defaultRotationSmoothDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStanceDimensions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStanceDimensions {
    /// DCB field: `heightCollider` (Single)
    #[serde(default)]
    pub height_collider: f32,
    /// DCB field: `groundContactEps` (Single)
    #[serde(default)]
    pub ground_contact_eps: f32,
    /// DCB field: `groundTraceSpreadSizes` (Class)
    #[serde(default)]
    pub ground_trace_spread_sizes: Option<Handle<Vec2>>,
    /// DCB field: `pivot` (Class)
    #[serde(default)]
    pub pivot: Option<Handle<Vec3>>,
    /// DCB field: `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
    /// DCB field: `capsuleAxis` (Class)
    #[serde(default)]
    pub capsule_axis: Option<Handle<Vec3>>,
    /// DCB field: `viewOffset` (Class)
    #[serde(default)]
    pub view_offset: Option<Handle<Vec3>>,
    /// DCB field: `weaponOffset` (Class)
    #[serde(default)]
    pub weapon_offset: Option<Handle<Vec3>>,
    /// DCB field: `headStabilization` (Class)
    #[serde(default)]
    pub head_stabilization: Option<Handle<Vec3>>,
    /// DCB field: `upAlignMode` (EnumChoice)
    #[serde(default)]
    pub up_align_mode: String,
    /// DCB field: `canPerch` (Boolean)
    #[serde(default)]
    pub can_perch: bool,
    /// DCB field: `extraDefs` (StrongPointer (array))
    #[serde(default)]
    pub extra_defs: Vec<Handle<SActorStanceDimensionsExtraDef>>,
}

impl Pooled for ActorStanceDimensions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_stance_dimensions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_stance_dimensions }
}

impl<'a> Extract<'a> for ActorStanceDimensions {
    const TYPE_NAME: &'static str = "ActorStanceDimensions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            height_collider: inst.get_f32("heightCollider").unwrap_or_default(),
            ground_contact_eps: inst.get_f32("groundContactEps").unwrap_or_default(),
            ground_trace_spread_sizes: match inst.get("groundTraceSpreadSizes") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            capsule_axis: match inst.get("capsuleAxis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            view_offset: match inst.get("viewOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_offset: match inst.get("weaponOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_stabilization: match inst.get("headStabilization") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            up_align_mode: inst.get_str("upAlignMode").map(String::from).unwrap_or_default(),
            can_perch: inst.get_bool("canPerch").unwrap_or_default(),
            extra_defs: inst.get_array("extraDefs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorStanceDimensionsExtraDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorStanceDimensionsExtraDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStanceSpeedsInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStanceSpeedsInfo {
    /// DCB field: `stateFilter` (Class)
    #[serde(default)]
    pub state_filter: Option<Handle<ActorMotionStateFilter>>,
    /// DCB field: `speeds` (Class)
    #[serde(default)]
    pub speeds: Option<Handle<ActorStanceSpeeds>>,
}

impl Pooled for ActorStanceSpeedsInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_stance_speeds_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_stance_speeds_info }
}

impl<'a> Extract<'a> for ActorStanceSpeedsInfo {
    const TYPE_NAME: &'static str = "ActorStanceSpeedsInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_filter: match inst.get("stateFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorMotionStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorMotionStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            speeds: match inst.get("speeds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStanceSpeeds>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStanceSpeeds>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStanceDimensionsInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStanceDimensionsInfo {
    /// DCB field: `stateFilter` (Class)
    #[serde(default)]
    pub state_filter: Option<Handle<ActorMotionStateFilter>>,
    /// DCB field: `dimensions` (Class)
    #[serde(default)]
    pub dimensions: Option<Handle<ActorStanceDimensions>>,
}

impl Pooled for ActorStanceDimensionsInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_stance_dimensions_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_stance_dimensions_info }
}

impl<'a> Extract<'a> for ActorStanceDimensionsInfo {
    const TYPE_NAME: &'static str = "ActorStanceDimensionsInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_filter: match inst.get("stateFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorMotionStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorMotionStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            dimensions: match inst.get("dimensions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStanceDimensions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStanceDimensions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStateFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateFilter {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
}

impl Pooled for ActorStateFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_state_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_state_filter }
}

impl<'a> Extract<'a> for ActorStateFilter {
    const TYPE_NAME: &'static str = "ActorStateFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filter_name: inst.get_str("filterName").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            filter_by_motion_speed: inst.get_str("filterByMotionSpeed").map(String::from).unwrap_or_default(),
            filter_by_pose_state: inst.get_str("filterByPoseState").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_lean_state: inst.get_str("filterByLeanState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
            filter_by_character_type: inst.get_str("filterByCharacterType").map(String::from).unwrap_or_default(),
            filter_by_restrained_state: inst.get_str("filterByRestrainedState").map(String::from).unwrap_or_default(),
            filter_by_player_camera: inst.get_str("filterByPlayerCamera").map(String::from).unwrap_or_default(),
            filter_by_aiming_restriction: inst.get_str("filterByAimingRestriction").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorMotionStateFilter`
///
/// Inherits from: `ActorStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorMotionStateFilter {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// DCB field: `filterByLocomotionSet` (EnumChoice)
    #[serde(default)]
    pub filter_by_locomotion_set: String,
}

impl Pooled for ActorMotionStateFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_motion_state_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_motion_state_filter }
}

impl<'a> Extract<'a> for ActorMotionStateFilter {
    const TYPE_NAME: &'static str = "ActorMotionStateFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filter_name: inst.get_str("filterName").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            filter_by_motion_speed: inst.get_str("filterByMotionSpeed").map(String::from).unwrap_or_default(),
            filter_by_pose_state: inst.get_str("filterByPoseState").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_lean_state: inst.get_str("filterByLeanState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
            filter_by_character_type: inst.get_str("filterByCharacterType").map(String::from).unwrap_or_default(),
            filter_by_restrained_state: inst.get_str("filterByRestrainedState").map(String::from).unwrap_or_default(),
            filter_by_player_camera: inst.get_str("filterByPlayerCamera").map(String::from).unwrap_or_default(),
            filter_by_aiming_restriction: inst.get_str("filterByAimingRestriction").map(String::from).unwrap_or_default(),
            filter_by_locomotion_set: inst.get_str("filterByLocomotionSet").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStateSkeletonFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateSkeletonFilter {
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
}

impl Pooled for ActorStateSkeletonFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_state_skeleton_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_state_skeleton_filter }
}

impl<'a> Extract<'a> for ActorStateSkeletonFilter {
    const TYPE_NAME: &'static str = "ActorStateSkeletonFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorActionHandler_LadderValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorActionHandler_LadderValidationParams {
    /// DCB field: `nearDistance` (Single)
    #[serde(default)]
    pub near_distance: f32,
    /// DCB field: `mountTimeout` (Single)
    #[serde(default)]
    pub mount_timeout: f32,
    /// DCB field: `slideTimeout` (Single)
    #[serde(default)]
    pub slide_timeout: f32,
    /// DCB field: `dismountTimeout` (Single)
    #[serde(default)]
    pub dismount_timeout: f32,
}

impl Pooled for ActorActionHandler_LadderValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_action_handler_ladder_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_action_handler_ladder_validation_params }
}

impl<'a> Extract<'a> for ActorActionHandler_LadderValidationParams {
    const TYPE_NAME: &'static str = "ActorActionHandler_LadderValidationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            near_distance: inst.get_f32("nearDistance").unwrap_or_default(),
            mount_timeout: inst.get_f32("mountTimeout").unwrap_or_default(),
            slide_timeout: inst.get_f32("slideTimeout").unwrap_or_default(),
            dismount_timeout: inst.get_f32("dismountTimeout").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorActionHandler_ValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorActionHandler_ValidationParams {
    /// DCB field: `ladderValidationParams` (Class)
    #[serde(default)]
    pub ladder_validation_params: Option<Handle<ActorActionHandler_LadderValidationParams>>,
}

impl Pooled for ActorActionHandler_ValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_action_handler_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_action_handler_validation_params }
}

impl<'a> Extract<'a> for ActorActionHandler_ValidationParams {
    const TYPE_NAME: &'static str = "ActorActionHandler_ValidationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ladder_validation_params: match inst.get("ladderValidationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorActionHandler_LadderValidationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorActionHandler_LadderValidationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStateData_StateValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateData_StateValidationParams {
    /// DCB field: `stateTimeout` (Single)
    #[serde(default)]
    pub state_timeout: f32,
}

impl Pooled for ActorStateData_StateValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_state_data_state_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_state_data_state_validation_params }
}

impl<'a> Extract<'a> for ActorStateData_StateValidationParams {
    const TYPE_NAME: &'static str = "ActorStateData_StateValidationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_timeout: inst.get_f32("stateTimeout").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStateData_AnimationValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateData_AnimationValidationParams {
    /// DCB field: `stateTimeout` (Single)
    #[serde(default)]
    pub state_timeout: f32,
}

impl Pooled for ActorStateData_AnimationValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_state_data_animation_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_state_data_animation_validation_params }
}

impl<'a> Extract<'a> for ActorStateData_AnimationValidationParams {
    const TYPE_NAME: &'static str = "ActorStateData_AnimationValidationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_timeout: inst.get_f32("stateTimeout").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStateData_JumpFallValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateData_JumpFallValidationParams {
    /// DCB field: `stateTimeout` (Single)
    #[serde(default)]
    pub state_timeout: f32,
}

impl Pooled for ActorStateData_JumpFallValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_state_data_jump_fall_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_state_data_jump_fall_validation_params }
}

impl<'a> Extract<'a> for ActorStateData_JumpFallValidationParams {
    const TYPE_NAME: &'static str = "ActorStateData_JumpFallValidationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_timeout: inst.get_f32("stateTimeout").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStateData_LadderValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateData_LadderValidationParams {
    /// DCB field: `stateTimeout` (Single)
    #[serde(default)]
    pub state_timeout: f32,
    /// DCB field: `divergeDistance` (Single)
    #[serde(default)]
    pub diverge_distance: f32,
}

impl Pooled for ActorStateData_LadderValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_state_data_ladder_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_state_data_ladder_validation_params }
}

impl<'a> Extract<'a> for ActorStateData_LadderValidationParams {
    const TYPE_NAME: &'static str = "ActorStateData_LadderValidationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_timeout: inst.get_f32("stateTimeout").unwrap_or_default(),
            diverge_distance: inst.get_f32("divergeDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStateData_UsableValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateData_UsableValidationParams {
    /// DCB field: `stateTimeout` (Single)
    #[serde(default)]
    pub state_timeout: f32,
}

impl Pooled for ActorStateData_UsableValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_state_data_usable_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_state_data_usable_validation_params }
}

impl<'a> Extract<'a> for ActorStateData_UsableValidationParams {
    const TYPE_NAME: &'static str = "ActorStateData_UsableValidationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_timeout: inst.get_f32("stateTimeout").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStateData_ValidationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateData_ValidationParams {
    /// DCB field: `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// DCB field: `stateValidationParams` (Class)
    #[serde(default)]
    pub state_validation_params: Option<Handle<ActorStateData_StateValidationParams>>,
    /// DCB field: `animationValidationParams` (Class)
    #[serde(default)]
    pub animation_validation_params: Option<Handle<ActorStateData_AnimationValidationParams>>,
    /// DCB field: `jumpFallValidationParams` (Class)
    #[serde(default)]
    pub jump_fall_validation_params: Option<Handle<ActorStateData_JumpFallValidationParams>>,
    /// DCB field: `ladderValidationParams` (Class)
    #[serde(default)]
    pub ladder_validation_params: Option<Handle<ActorStateData_LadderValidationParams>>,
    /// DCB field: `usableValidationParams` (Class)
    #[serde(default)]
    pub usable_validation_params: Option<Handle<ActorStateData_UsableValidationParams>>,
}

impl Pooled for ActorStateData_ValidationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_state_data_validation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_state_data_validation_params }
}

impl<'a> Extract<'a> for ActorStateData_ValidationParams {
    const TYPE_NAME: &'static str = "ActorStateData_ValidationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            state_validation_params: match inst.get("stateValidationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateData_StateValidationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateData_StateValidationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            animation_validation_params: match inst.get("animationValidationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateData_AnimationValidationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateData_AnimationValidationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jump_fall_validation_params: match inst.get("jumpFallValidationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateData_JumpFallValidationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateData_JumpFallValidationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ladder_validation_params: match inst.get("ladderValidationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateData_LadderValidationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateData_LadderValidationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            usable_validation_params: match inst.get("usableValidationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateData_UsableValidationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateData_UsableValidationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStateValidation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateValidation {
    /// DCB field: `actorActionHandlerValidationParams` (Class)
    #[serde(default)]
    pub actor_action_handler_validation_params: Option<Handle<ActorActionHandler_ValidationParams>>,
    /// DCB field: `actorStateDataValidationParams` (Class)
    #[serde(default)]
    pub actor_state_data_validation_params: Option<Handle<ActorStateData_ValidationParams>>,
}

impl Pooled for ActorStateValidation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_state_validation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_state_validation }
}

impl<'a> Extract<'a> for ActorStateValidation {
    const TYPE_NAME: &'static str = "ActorStateValidation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actor_action_handler_validation_params: match inst.get("actorActionHandlerValidationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorActionHandler_ValidationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorActionHandler_ValidationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_state_data_validation_params: match inst.get("actorStateDataValidationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateData_ValidationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateData_ValidationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStatusData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusData {
    /// DCB field: `actorStatusType` (EnumChoice)
    #[serde(default)]
    pub actor_status_type: String,
    /// DCB field: `statusTrigger` (StrongPointer)
    #[serde(default)]
    pub status_trigger: Option<Handle<StatusTriggerBase>>,
    /// DCB field: `statusEffectTriggers` (StrongPointer (array))
    #[serde(default)]
    pub status_effect_triggers: Vec<Handle<StatusEffectTrigger>>,
}

impl Pooled for ActorStatusData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_data }
}

impl<'a> Extract<'a> for ActorStatusData {
    const TYPE_NAME: &'static str = "ActorStatusData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actor_status_type: inst.get_str("actorStatusType").map(String::from).unwrap_or_default(),
            status_trigger: match inst.get("statusTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusTriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusTriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            status_effect_triggers: inst.get_array("statusEffectTriggers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StatusEffectTrigger>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StatusEffectTrigger>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStatData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatData {
    /// DCB field: `statType` (EnumChoice)
    #[serde(default)]
    pub stat_type: String,
    /// DCB field: `initialValue` (Single)
    #[serde(default)]
    pub initial_value: f32,
    /// DCB field: `minimumStatValue` (Single)
    #[serde(default)]
    pub minimum_stat_value: f32,
    /// DCB field: `maximumStatValue` (Single)
    #[serde(default)]
    pub maximum_stat_value: f32,
    /// DCB field: `decayRate` (Single)
    #[serde(default)]
    pub decay_rate: f32,
    /// DCB field: `baseDecayCooldown` (Single)
    #[serde(default)]
    pub base_decay_cooldown: f32,
    /// DCB field: `initialCooldown` (Single)
    #[serde(default)]
    pub initial_cooldown: f32,
    /// DCB field: `cooldownType` (EnumChoice)
    #[serde(default)]
    pub cooldown_type: String,
    /// DCB field: `decayToCurrentStatus` (Boolean)
    #[serde(default)]
    pub decay_to_current_status: bool,
    /// DCB field: `mutuallyExclusiveStatuses` (Boolean)
    #[serde(default)]
    pub mutually_exclusive_statuses: bool,
    /// DCB field: `remoteClientPredicted` (Boolean)
    #[serde(default)]
    pub remote_client_predicted: bool,
    /// DCB field: `useIdealStatValue` (Boolean)
    #[serde(default)]
    pub use_ideal_stat_value: bool,
    /// DCB field: `linkedStats` (StrongPointer (array))
    #[serde(default)]
    pub linked_stats: Vec<Handle<LinkedStatBase>>,
    /// DCB field: `effectSetup` (StrongPointer)
    #[serde(default)]
    pub effect_setup: Option<Handle<StatusEffectSetupBase>>,
    /// DCB field: `maskedRetriggerSetup` (StrongPointer)
    #[serde(default)]
    pub masked_retrigger_setup: Option<Handle<StatusMaskedRetriggerSetupBase>>,
}

impl Pooled for ActorStatData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_stat_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_stat_data }
}

impl<'a> Extract<'a> for ActorStatData {
    const TYPE_NAME: &'static str = "ActorStatData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stat_type: inst.get_str("statType").map(String::from).unwrap_or_default(),
            initial_value: inst.get_f32("initialValue").unwrap_or_default(),
            minimum_stat_value: inst.get_f32("minimumStatValue").unwrap_or_default(),
            maximum_stat_value: inst.get_f32("maximumStatValue").unwrap_or_default(),
            decay_rate: inst.get_f32("decayRate").unwrap_or_default(),
            base_decay_cooldown: inst.get_f32("baseDecayCooldown").unwrap_or_default(),
            initial_cooldown: inst.get_f32("initialCooldown").unwrap_or_default(),
            cooldown_type: inst.get_str("cooldownType").map(String::from).unwrap_or_default(),
            decay_to_current_status: inst.get_bool("decayToCurrentStatus").unwrap_or_default(),
            mutually_exclusive_statuses: inst.get_bool("mutuallyExclusiveStatuses").unwrap_or_default(),
            remote_client_predicted: inst.get_bool("remoteClientPredicted").unwrap_or_default(),
            use_ideal_stat_value: inst.get_bool("useIdealStatValue").unwrap_or_default(),
            linked_stats: inst.get_array("linkedStats")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LinkedStatBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LinkedStatBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            effect_setup: match inst.get("effectSetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusEffectSetupBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusEffectSetupBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            masked_retrigger_setup: match inst.get("maskedRetriggerSetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusMaskedRetriggerSetupBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusMaskedRetriggerSetupBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStatusPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusPreset {
    /// DCB field: `effectSetup` (Class)
    #[serde(default)]
    pub effect_setup: Option<Handle<StatusEffectSetup>>,
}

impl Pooled for ActorStatusPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_preset }
}

impl<'a> Extract<'a> for ActorStatusPreset {
    const TYPE_NAME: &'static str = "ActorStatusPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effect_setup: match inst.get("effectSetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusEffectSetup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusEffectSetup>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStatusUIWarningEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusUIWarningEntry {
    /// DCB field: `statValue` (Single)
    #[serde(default)]
    pub stat_value: f32,
    /// DCB field: `cooldownValue` (Single)
    #[serde(default)]
    pub cooldown_value: f32,
    /// DCB field: `warningString` (Locale)
    #[serde(default)]
    pub warning_string: String,
}

impl Pooled for ActorStatusUIWarningEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_uiwarning_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_uiwarning_entry }
}

impl<'a> Extract<'a> for ActorStatusUIWarningEntry {
    const TYPE_NAME: &'static str = "ActorStatusUIWarningEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            stat_value: inst.get_f32("statValue").unwrap_or_default(),
            cooldown_value: inst.get_f32("cooldownValue").unwrap_or_default(),
            warning_string: inst.get_str("warningString").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStatusUIWarning`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusUIWarning {
    /// DCB field: `statType` (EnumChoice)
    #[serde(default)]
    pub stat_type: String,
    /// DCB field: `warningDuration` (Single)
    #[serde(default)]
    pub warning_duration: f32,
    /// DCB field: `warningEntries` (Class (array))
    #[serde(default)]
    pub warning_entries: Vec<Handle<ActorStatusUIWarningEntry>>,
}

impl Pooled for ActorStatusUIWarning {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_uiwarning }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_uiwarning }
}

impl<'a> Extract<'a> for ActorStatusUIWarning {
    const TYPE_NAME: &'static str = "ActorStatusUIWarning";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stat_type: inst.get_str("statType").map(String::from).unwrap_or_default(),
            warning_duration: inst.get_f32("warningDuration").unwrap_or_default(),
            warning_entries: inst.get_array("warningEntries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorStatusUIWarningEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorStatusUIWarningEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStatusIncapacitatedUIData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusIncapacitatedUIData {
    /// DCB field: `unconsciousStateWarning` (Locale)
    #[serde(default)]
    pub unconscious_state_warning: String,
    /// DCB field: `downedStateWarning` (Locale)
    #[serde(default)]
    pub downed_state_warning: String,
    /// DCB field: `overdosedStateWarning` (Locale)
    #[serde(default)]
    pub overdosed_state_warning: String,
    /// DCB field: `timeToRevivalString` (Locale)
    #[serde(default)]
    pub time_to_revival_string: String,
    /// DCB field: `timeToDeathString` (Locale)
    #[serde(default)]
    pub time_to_death_string: String,
    /// DCB field: `downedRevivalBeaconInsufficientFunds` (Locale)
    #[serde(default)]
    pub downed_revival_beacon_insufficient_funds: String,
    /// DCB field: `downedRevivalBeaconAvailable` (Locale)
    #[serde(default)]
    pub downed_revival_beacon_available: String,
    /// DCB field: `downedRevivalBeaconRequested` (Locale)
    #[serde(default)]
    pub downed_revival_beacon_requested: String,
    /// DCB field: `downedRevivalBeaconAccepted` (Locale)
    #[serde(default)]
    pub downed_revival_beacon_accepted: String,
    /// DCB field: `downedRevivalBeaconAcceptedGeneric` (Locale)
    #[serde(default)]
    pub downed_revival_beacon_accepted_generic: String,
    /// DCB field: `downedRevivalBeaconAbandoned` (Locale)
    #[serde(default)]
    pub downed_revival_beacon_abandoned: String,
}

impl Pooled for ActorStatusIncapacitatedUIData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_incapacitated_uidata }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_incapacitated_uidata }
}

impl<'a> Extract<'a> for ActorStatusIncapacitatedUIData {
    const TYPE_NAME: &'static str = "ActorStatusIncapacitatedUIData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            unconscious_state_warning: inst.get_str("unconsciousStateWarning").map(String::from).unwrap_or_default(),
            downed_state_warning: inst.get_str("downedStateWarning").map(String::from).unwrap_or_default(),
            overdosed_state_warning: inst.get_str("overdosedStateWarning").map(String::from).unwrap_or_default(),
            time_to_revival_string: inst.get_str("timeToRevivalString").map(String::from).unwrap_or_default(),
            time_to_death_string: inst.get_str("timeToDeathString").map(String::from).unwrap_or_default(),
            downed_revival_beacon_insufficient_funds: inst.get_str("downedRevivalBeaconInsufficientFunds").map(String::from).unwrap_or_default(),
            downed_revival_beacon_available: inst.get_str("downedRevivalBeaconAvailable").map(String::from).unwrap_or_default(),
            downed_revival_beacon_requested: inst.get_str("downedRevivalBeaconRequested").map(String::from).unwrap_or_default(),
            downed_revival_beacon_accepted: inst.get_str("downedRevivalBeaconAccepted").map(String::from).unwrap_or_default(),
            downed_revival_beacon_accepted_generic: inst.get_str("downedRevivalBeaconAcceptedGeneric").map(String::from).unwrap_or_default(),
            downed_revival_beacon_abandoned: inst.get_str("downedRevivalBeaconAbandoned").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStatusUIData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusUIData {
    /// DCB field: `lifeExpectancyWarning` (Locale)
    #[serde(default)]
    pub life_expectancy_warning: String,
    /// DCB field: `downedRescueNotification` (Locale)
    #[serde(default)]
    pub downed_rescue_notification: String,
    /// DCB field: `incapacitatedWarningStrings` (Class)
    #[serde(default)]
    pub incapacitated_warning_strings: Option<Handle<ActorStatusIncapacitatedUIData>>,
    /// DCB field: `actorStatusWarnings` (Class (array))
    #[serde(default)]
    pub actor_status_warnings: Vec<Handle<ActorStatusUIWarning>>,
    /// DCB field: `healthIconData` (Class)
    #[serde(default)]
    pub health_icon_data: Option<Handle<HealthIconData>>,
}

impl Pooled for ActorStatusUIData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_uidata }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_uidata }
}

impl<'a> Extract<'a> for ActorStatusUIData {
    const TYPE_NAME: &'static str = "ActorStatusUIData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            life_expectancy_warning: inst.get_str("lifeExpectancyWarning").map(String::from).unwrap_or_default(),
            downed_rescue_notification: inst.get_str("downedRescueNotification").map(String::from).unwrap_or_default(),
            incapacitated_warning_strings: match inst.get("incapacitatedWarningStrings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStatusIncapacitatedUIData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStatusIncapacitatedUIData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_status_warnings: inst.get_array("actorStatusWarnings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorStatusUIWarning>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorStatusUIWarning>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            health_icon_data: match inst.get("healthIconData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HealthIconData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HealthIconData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActionStatusCosts`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionStatusCosts {
    /// DCB field: `actionCategory` (EnumChoice)
    #[serde(default)]
    pub action_category: String,
    /// DCB field: `instantStatusCost` (Class (array))
    #[serde(default)]
    pub instant_status_cost: Vec<Handle<StatusCost>>,
}

impl Pooled for ActionStatusCosts {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.action_status_costs }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.action_status_costs }
}

impl<'a> Extract<'a> for ActionStatusCosts {
    const TYPE_NAME: &'static str = "ActionStatusCosts";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            action_category: inst.get_str("actionCategory").map(String::from).unwrap_or_default(),
            instant_status_cost: inst.get_array("instantStatusCost")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StatusCost>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StatusCost>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStatusLocalisation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusLocalisation {
    /// DCB field: `hospitalisationReasonNames` (Locale)
    #[serde(default)]
    pub hospitalisation_reason_names: String,
    /// DCB field: `deathReasonNames` (Locale)
    #[serde(default)]
    pub death_reason_names: String,
    /// DCB field: `damageTypeNames` (Locale)
    #[serde(default)]
    pub damage_type_names: String,
}

impl Pooled for ActorStatusLocalisation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_localisation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_localisation }
}

impl<'a> Extract<'a> for ActorStatusLocalisation {
    const TYPE_NAME: &'static str = "ActorStatusLocalisation";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hospitalisation_reason_names: inst.get_str("hospitalisationReasonNames").map(String::from).unwrap_or_default(),
            death_reason_names: inst.get_str("deathReasonNames").map(String::from).unwrap_or_default(),
            damage_type_names: inst.get_str("damageTypeNames").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStatusBuff`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusBuff {
    /// DCB field: `statusEffectType` (EnumChoice)
    #[serde(default)]
    pub status_effect_type: String,
    /// DCB field: `buffApplication` (StrongPointer)
    #[serde(default)]
    pub buff_application: Option<Handle<StatusBuffTypeBase>>,
    /// DCB field: `buffValue` (Single)
    #[serde(default)]
    pub buff_value: f32,
    /// DCB field: `buffDuration` (Single)
    #[serde(default)]
    pub buff_duration: f32,
    /// DCB field: `buffDurationCap` (Single)
    #[serde(default)]
    pub buff_duration_cap: f32,
}

impl Pooled for ActorStatusBuff {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_buff }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_buff }
}

impl<'a> Extract<'a> for ActorStatusBuff {
    const TYPE_NAME: &'static str = "ActorStatusBuff";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            status_effect_type: inst.get_str("statusEffectType").map(String::from).unwrap_or_default(),
            buff_application: match inst.get("buffApplication") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusBuffTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusBuffTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            buff_value: inst.get_f32("buffValue").unwrap_or_default(),
            buff_duration: inst.get_f32("buffDuration").unwrap_or_default(),
            buff_duration_cap: inst.get_f32("buffDurationCap").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStatusAddBuff`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusAddBuff {
    /// DCB field: `buffType` (EnumChoice)
    #[serde(default)]
    pub buff_type: String,
    /// DCB field: `durationOverride` (StrongPointer)
    #[serde(default)]
    pub duration_override: Option<Handle<BuffDurationBase>>,
    /// DCB field: `valueOverride` (StrongPointer)
    #[serde(default)]
    pub value_override: Option<Handle<BuffValueOverride>>,
}

impl Pooled for ActorStatusAddBuff {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_add_buff }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_add_buff }
}

impl<'a> Extract<'a> for ActorStatusAddBuff {
    const TYPE_NAME: &'static str = "ActorStatusAddBuff";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            buff_type: inst.get_str("buffType").map(String::from).unwrap_or_default(),
            duration_override: match inst.get("durationOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuffDurationBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuffDurationBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            value_override: match inst.get("valueOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuffValueOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuffValueOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStatusStatModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusStatModifier {
    /// DCB field: `affectedStatType` (EnumChoice)
    #[serde(default)]
    pub affected_stat_type: String,
}

impl Pooled for ActorStatusStatModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_stat_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_stat_modifier }
}

impl<'a> Extract<'a> for ActorStatusStatModifier {
    const TYPE_NAME: &'static str = "ActorStatusStatModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            affected_stat_type: inst.get_str("affectedStatType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStatusEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusEffect {
    /// DCB field: `statusEffectValueType` (StrongPointer)
    #[serde(default)]
    pub status_effect_value_type: Option<Handle<BasicStatusEffectApplicationType>>,
    /// DCB field: `affectedStat` (StrongPointer)
    #[serde(default)]
    pub affected_stat: Option<Handle<ActorStatusStatModifier>>,
    /// DCB field: `display` (Locale)
    #[serde(default)]
    pub display: String,
}

impl Pooled for ActorStatusEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_effect }
}

impl<'a> Extract<'a> for ActorStatusEffect {
    const TYPE_NAME: &'static str = "ActorStatusEffect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            status_effect_value_type: match inst.get("statusEffectValueType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BasicStatusEffectApplicationType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BasicStatusEffectApplicationType>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            affected_stat: match inst.get("affectedStat") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStatusStatModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStatusStatModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display: inst.get_str("display").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorSomaticShakeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSomaticShakeConfig {
    /// DCB field: `filter` (Class)
    #[serde(default)]
    pub filter: Option<Handle<ActorStateFilter>>,
    /// DCB field: `weaponOffsetCm` (Single)
    #[serde(default)]
    pub weapon_offset_cm: f32,
    /// DCB field: `weaponRotationAngle` (Single)
    #[serde(default)]
    pub weapon_rotation_angle: f32,
    /// DCB field: `armRotationAngle` (Single)
    #[serde(default)]
    pub arm_rotation_angle: f32,
    /// DCB field: `handRotationAngle` (Single)
    #[serde(default)]
    pub hand_rotation_angle: f32,
}

impl Pooled for ActorSomaticShakeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_somatic_shake_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_somatic_shake_config }
}

impl<'a> Extract<'a> for ActorSomaticShakeConfig {
    const TYPE_NAME: &'static str = "ActorSomaticShakeConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter: match inst.get("filter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_offset_cm: inst.get_f32("weaponOffsetCm").unwrap_or_default(),
            weapon_rotation_angle: inst.get_f32("weaponRotationAngle").unwrap_or_default(),
            arm_rotation_angle: inst.get_f32("armRotationAngle").unwrap_or_default(),
            hand_rotation_angle: inst.get_f32("handRotationAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorShudderConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorShudderConfig {
    /// DCB field: `shuddersEnabled` (Boolean)
    #[serde(default)]
    pub shudders_enabled: bool,
    /// DCB field: `frequencyMin` (Single)
    #[serde(default)]
    pub frequency_min: f32,
    /// DCB field: `frequencyMax` (Single)
    #[serde(default)]
    pub frequency_max: f32,
    /// DCB field: `amplitudeMin` (Single)
    #[serde(default)]
    pub amplitude_min: f32,
    /// DCB field: `amplitudeMax` (Single)
    #[serde(default)]
    pub amplitude_max: f32,
    /// DCB field: `durationMin` (Single)
    #[serde(default)]
    pub duration_min: f32,
    /// DCB field: `durationMax` (Single)
    #[serde(default)]
    pub duration_max: f32,
    /// DCB field: `delayMin` (Single)
    #[serde(default)]
    pub delay_min: f32,
    /// DCB field: `delayMax` (Single)
    #[serde(default)]
    pub delay_max: f32,
}

impl Pooled for ActorShudderConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_shudder_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_shudder_config }
}

impl<'a> Extract<'a> for ActorShudderConfig {
    const TYPE_NAME: &'static str = "ActorShudderConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            shudders_enabled: inst.get_bool("shuddersEnabled").unwrap_or_default(),
            frequency_min: inst.get_f32("frequencyMin").unwrap_or_default(),
            frequency_max: inst.get_f32("frequencyMax").unwrap_or_default(),
            amplitude_min: inst.get_f32("amplitudeMin").unwrap_or_default(),
            amplitude_max: inst.get_f32("amplitudeMax").unwrap_or_default(),
            duration_min: inst.get_f32("durationMin").unwrap_or_default(),
            duration_max: inst.get_f32("durationMax").unwrap_or_default(),
            delay_min: inst.get_f32("delayMin").unwrap_or_default(),
            delay_max: inst.get_f32("delayMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorSomaticShakeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSomaticShakeParams {
    /// DCB field: `frequencyStart` (Single)
    #[serde(default)]
    pub frequency_start: f32,
    /// DCB field: `frequencyEnd` (Single)
    #[serde(default)]
    pub frequency_end: f32,
    /// DCB field: `amplitudeStart` (Single)
    #[serde(default)]
    pub amplitude_start: f32,
    /// DCB field: `amplitudeEnd` (Single)
    #[serde(default)]
    pub amplitude_end: f32,
    /// DCB field: `blendDuration` (Single)
    #[serde(default)]
    pub blend_duration: f32,
    /// DCB field: `breathingInfluence` (Single)
    #[serde(default)]
    pub breathing_influence: f32,
    /// DCB field: `shudder` (Class)
    #[serde(default)]
    pub shudder: Option<Handle<ActorShudderConfig>>,
    /// DCB field: `somaticShakeConfigs` (Class (array))
    #[serde(default)]
    pub somatic_shake_configs: Vec<Handle<ActorSomaticShakeConfig>>,
}

impl Pooled for ActorSomaticShakeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_somatic_shake_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_somatic_shake_params }
}

impl<'a> Extract<'a> for ActorSomaticShakeParams {
    const TYPE_NAME: &'static str = "ActorSomaticShakeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            frequency_start: inst.get_f32("frequencyStart").unwrap_or_default(),
            frequency_end: inst.get_f32("frequencyEnd").unwrap_or_default(),
            amplitude_start: inst.get_f32("amplitudeStart").unwrap_or_default(),
            amplitude_end: inst.get_f32("amplitudeEnd").unwrap_or_default(),
            blend_duration: inst.get_f32("blendDuration").unwrap_or_default(),
            breathing_influence: inst.get_f32("breathingInfluence").unwrap_or_default(),
            shudder: match inst.get("shudder") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorShudderConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorShudderConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            somatic_shake_configs: inst.get_array("somaticShakeConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorSomaticShakeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorSomaticShakeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorSomaticShakingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSomaticShakingParams {
    /// DCB field: `shiveringParams` (Class)
    #[serde(default)]
    pub shivering_params: Option<Handle<ActorSomaticShakeParams>>,
    /// DCB field: `armsLockShakeParams` (Class)
    #[serde(default)]
    pub arms_lock_shake_params: Option<Handle<ActorSomaticShakeParams>>,
}

impl Pooled for ActorSomaticShakingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_somatic_shaking_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_somatic_shaking_params }
}

impl<'a> Extract<'a> for ActorSomaticShakingParams {
    const TYPE_NAME: &'static str = "ActorSomaticShakingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            shivering_params: match inst.get("shiveringParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorSomaticShakeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorSomaticShakeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            arms_lock_shake_params: match inst.get("armsLockShakeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorSomaticShakeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorSomaticShakeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorToxicGasParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorToxicGasParams {
    /// DCB field: `toxicGases` (Class (array))
    #[serde(default)]
    pub toxic_gases: Vec<Handle<ToxicGasDef>>,
}

impl Pooled for ActorToxicGasParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_toxic_gas_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_toxic_gas_params }
}

impl<'a> Extract<'a> for ActorToxicGasParams {
    const TYPE_NAME: &'static str = "ActorToxicGasParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            toxic_gases: inst.get_array("toxicGases")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ToxicGasDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ToxicGasDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStatusGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusGlobalParams {
    /// DCB field: `actorStatusEffects` (Class)
    #[serde(default)]
    pub actor_status_effects: Option<Handle<ActorStatusEffect>>,
    /// DCB field: `actorStatusBuffs` (Class)
    #[serde(default)]
    pub actor_status_buffs: Option<Handle<ActorStatusBuff>>,
    /// DCB field: `actorStatusLocalisation` (Class)
    #[serde(default)]
    pub actor_status_localisation: Option<Handle<ActorStatusLocalisation>>,
    /// DCB field: `medicalItemTierConfig` (Reference)
    #[serde(default)]
    pub medical_item_tier_config: Option<CigGuid>,
    /// DCB field: `drugConsumableType` (Reference)
    #[serde(default)]
    pub drug_consumable_type: Option<CigGuid>,
    /// DCB field: `overdoseRevivalConsumableSubType` (Reference)
    #[serde(default)]
    pub overdose_revival_consumable_sub_type: Option<CigGuid>,
    /// DCB field: `somaticShakeParams` (Class)
    #[serde(default)]
    pub somatic_shake_params: Option<Handle<ActorSomaticShakingParams>>,
    /// DCB field: `statusPlacedSurfaceEffectsTags` (Reference)
    #[serde(default)]
    pub status_placed_surface_effects_tags: Option<CigGuid>,
}

impl Pooled for ActorStatusGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_global_params }
}

impl<'a> Extract<'a> for ActorStatusGlobalParams {
    const TYPE_NAME: &'static str = "ActorStatusGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actor_status_effects: match inst.get("actorStatusEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStatusEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStatusEffect>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_status_buffs: match inst.get("actorStatusBuffs") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStatusBuff>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStatusBuff>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_status_localisation: match inst.get("actorStatusLocalisation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStatusLocalisation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStatusLocalisation>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            medical_item_tier_config: inst.get("medicalItemTierConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            drug_consumable_type: inst.get("drugConsumableType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            overdose_revival_consumable_sub_type: inst.get("overdoseRevivalConsumableSubType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            somatic_shake_params: match inst.get("somaticShakeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorSomaticShakingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorSomaticShakingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            status_placed_surface_effects_tags: inst.get("statusPlacedSurfaceEffectsTags").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ActorStatusComponent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatusComponent {
    /// DCB field: `speedThresholds` (Class (array))
    #[serde(default)]
    pub speed_thresholds: Vec<Handle<SpeedThreshold>>,
    /// DCB field: `actorStats` (Class (array))
    #[serde(default)]
    pub actor_stats: Vec<Handle<ActorStatData>>,
    /// DCB field: `actorStatusPresets` (Class (array))
    #[serde(default)]
    pub actor_status_presets: Vec<Handle<ActorStatusPreset>>,
    /// DCB field: `linkedStatPresets` (Class (array))
    #[serde(default)]
    pub linked_stat_presets: Vec<Handle<LinkedStatSetupPreset>>,
    /// DCB field: `maskedRetriggerPresets` (Class (array))
    #[serde(default)]
    pub masked_retrigger_presets: Vec<Handle<StatusMaskedRetriggerPreset>>,
    /// DCB field: `actorStatusUIData` (Class)
    #[serde(default)]
    pub actor_status_uidata: Option<Handle<ActorStatusUIData>>,
    /// DCB field: `abilityStatusCosts` (Class (array))
    #[serde(default)]
    pub ability_status_costs: Vec<Handle<AbilityStatusCosts>>,
    /// DCB field: `actionStatusCosts` (Class (array))
    #[serde(default)]
    pub action_status_costs: Vec<Handle<ActionStatusCosts>>,
    /// DCB field: `statusEffectAbilityLocks` (Class (array))
    #[serde(default)]
    pub status_effect_ability_locks: Vec<Handle<StatusEffectAbilityLock>>,
    /// DCB field: `consumptionParams` (Class)
    #[serde(default)]
    pub consumption_params: Option<Handle<ConsumableParams>>,
    /// DCB field: `armsLockParams` (Class)
    #[serde(default)]
    pub arms_lock_params: Option<Handle<ArmsLockConfig>>,
    /// DCB field: `driftingConsciousnessParams` (Class)
    #[serde(default)]
    pub drifting_consciousness_params: Option<Handle<DriftingConsciousnessConfig>>,
    /// DCB field: `downedParams` (Class)
    #[serde(default)]
    pub downed_params: Option<Handle<DownedConfig>>,
    /// DCB field: `driftingDrunkParams` (Class)
    #[serde(default)]
    pub drifting_drunk_params: Option<Handle<DriftingDrunkConfig>>,
    /// DCB field: `actorSignatureParams` (Class)
    #[serde(default)]
    pub actor_signature_params: Option<Handle<SignatureParams>>,
    /// DCB field: `bodyResistanceWeights` (Class)
    #[serde(default)]
    pub body_resistance_weights: Option<Handle<ResistanceWeightParams>>,
    /// DCB field: `revivalFadeinParams` (Class)
    #[serde(default)]
    pub revival_fadein_params: Option<Handle<RevivalFadeInParams>>,
    /// DCB field: `globalParams` (Reference)
    #[serde(default)]
    pub global_params: Option<CigGuid>,
    /// DCB field: `BDLSafeLimit` (Single)
    #[serde(default)]
    pub bdlsafe_limit: f32,
    /// DCB field: `toxicGasParams` (StrongPointer)
    #[serde(default)]
    pub toxic_gas_params: Option<Handle<ActorToxicGasParams>>,
    /// DCB field: `hygieneParams` (Class)
    #[serde(default)]
    pub hygiene_params: Option<Handle<HygieneParams>>,
}

impl Pooled for ActorStatusComponent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_status_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_status_component }
}

impl<'a> Extract<'a> for ActorStatusComponent {
    const TYPE_NAME: &'static str = "ActorStatusComponent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            speed_thresholds: inst.get_array("speedThresholds")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SpeedThreshold>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SpeedThreshold>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            actor_stats: inst.get_array("actorStats")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorStatData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorStatData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            actor_status_presets: inst.get_array("actorStatusPresets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorStatusPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorStatusPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            linked_stat_presets: inst.get_array("linkedStatPresets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LinkedStatSetupPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LinkedStatSetupPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            masked_retrigger_presets: inst.get_array("maskedRetriggerPresets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StatusMaskedRetriggerPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StatusMaskedRetriggerPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            actor_status_uidata: match inst.get("actorStatusUIData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStatusUIData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStatusUIData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ability_status_costs: inst.get_array("abilityStatusCosts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AbilityStatusCosts>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AbilityStatusCosts>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            action_status_costs: inst.get_array("actionStatusCosts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActionStatusCosts>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActionStatusCosts>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            status_effect_ability_locks: inst.get_array("statusEffectAbilityLocks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StatusEffectAbilityLock>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StatusEffectAbilityLock>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            consumption_params: match inst.get("consumptionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ConsumableParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConsumableParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            arms_lock_params: match inst.get("armsLockParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ArmsLockConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ArmsLockConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drifting_consciousness_params: match inst.get("driftingConsciousnessParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DriftingConsciousnessConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DriftingConsciousnessConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            downed_params: match inst.get("downedParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DownedConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DownedConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drifting_drunk_params: match inst.get("driftingDrunkParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DriftingDrunkConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DriftingDrunkConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actor_signature_params: match inst.get("actorSignatureParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SignatureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SignatureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            body_resistance_weights: match inst.get("bodyResistanceWeights") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ResistanceWeightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ResistanceWeightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            revival_fadein_params: match inst.get("revivalFadeinParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RevivalFadeInParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RevivalFadeInParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            global_params: inst.get("globalParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            bdlsafe_limit: inst.get_f32("BDLSafeLimit").unwrap_or_default(),
            toxic_gas_params: match inst.get("toxicGasParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorToxicGasParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorToxicGasParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hygiene_params: match inst.get("hygieneParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HygieneParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HygieneParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorZeroGTraversalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorZeroGTraversalParams {
    /// DCB field: `defaultZeroGTraversalParams` (Class)
    #[serde(default)]
    pub default_zero_gtraversal_params: Option<Handle<SCDefaultZeroGTraversalParams>>,
    /// DCB field: `optionalZeroGTraversalParams` (Class (array))
    #[serde(default)]
    pub optional_zero_gtraversal_params: Vec<Handle<SCOptionalZeroGTraversalParams>>,
}

impl Pooled for ActorZeroGTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_zero_gtraversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_zero_gtraversal_params }
}

impl<'a> Extract<'a> for ActorZeroGTraversalParams {
    const TYPE_NAME: &'static str = "ActorZeroGTraversalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_zero_gtraversal_params: match inst.get("defaultZeroGTraversalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCDefaultZeroGTraversalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCDefaultZeroGTraversalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            optional_zero_gtraversal_params: inst.get_array("optionalZeroGTraversalParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCOptionalZeroGTraversalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCOptionalZeroGTraversalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorBreathingStyleStartup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorBreathingStyleStartup {
    /// DCB field: `forceInhaleFirst` (Boolean)
    #[serde(default)]
    pub force_inhale_first: bool,
    /// DCB field: `forceExhaleFirst` (Boolean)
    #[serde(default)]
    pub force_exhale_first: bool,
    /// DCB field: `overrideFirstInhale` (Class)
    #[serde(default)]
    pub override_first_inhale: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `overrideFirstExhale` (Class)
    #[serde(default)]
    pub override_first_exhale: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `firstBreathDurationFromOverride` (Boolean)
    #[serde(default)]
    pub first_breath_duration_from_override: bool,
    /// DCB field: `firstBreathDurationCompensation` (Single)
    #[serde(default)]
    pub first_breath_duration_compensation: f32,
    /// DCB field: `conditions` (Reference (array))
    #[serde(default)]
    pub conditions: Vec<CigGuid>,
}

impl Pooled for ActorBreathingStyleStartup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_breathing_style_startup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_breathing_style_startup }
}

impl<'a> Extract<'a> for ActorBreathingStyleStartup {
    const TYPE_NAME: &'static str = "ActorBreathingStyleStartup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            force_inhale_first: inst.get_bool("forceInhaleFirst").unwrap_or_default(),
            force_exhale_first: inst.get_bool("forceExhaleFirst").unwrap_or_default(),
            override_first_inhale: match inst.get("overrideFirstInhale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            override_first_exhale: match inst.get("overrideFirstExhale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            first_breath_duration_from_override: inst.get_bool("firstBreathDurationFromOverride").unwrap_or_default(),
            first_breath_duration_compensation: inst.get_f32("firstBreathDurationCompensation").unwrap_or_default(),
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorFOVViewParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorFOVViewParams {
    /// DCB field: `globalVerticalFOVLimit` (Class)
    #[serde(default)]
    pub global_vertical_fovlimit: Option<Handle<Vec2>>,
    /// DCB field: `globalFOVLerpSpeed` (Single)
    #[serde(default)]
    pub global_fovlerp_speed: f32,
}

impl Pooled for ActorFOVViewParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_fovview_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_fovview_params }
}

impl<'a> Extract<'a> for ActorFOVViewParams {
    const TYPE_NAME: &'static str = "ActorFOVViewParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            global_vertical_fovlimit: match inst.get("globalVerticalFOVLimit") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            global_fovlerp_speed: inst.get_f32("globalFOVLerpSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLookAheadPoint`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLookAheadPoint {
    /// DCB field: `limitYawMinMax` (Class)
    #[serde(default)]
    pub limit_yaw_min_max: Option<Handle<Vec2>>,
    /// DCB field: `limitPitchMinMax` (Class)
    #[serde(default)]
    pub limit_pitch_min_max: Option<Handle<Vec2>>,
    /// DCB field: `thresholdYawMinMax` (Class)
    #[serde(default)]
    pub threshold_yaw_min_max: Option<Handle<Vec2>>,
    /// DCB field: `thresholdPitchMinMax` (Class)
    #[serde(default)]
    pub threshold_pitch_min_max: Option<Handle<Vec2>>,
    /// DCB field: `multiplierYaw` (Single)
    #[serde(default)]
    pub multiplier_yaw: f32,
    /// DCB field: `multiplierPitch` (Single)
    #[serde(default)]
    pub multiplier_pitch: f32,
    /// DCB field: `defaultWeight` (Single)
    #[serde(default)]
    pub default_weight: f32,
    /// DCB field: `statusSwapTime` (Single)
    #[serde(default)]
    pub status_swap_time: f32,
    /// DCB field: `respectsDampeningZone` (Boolean)
    #[serde(default)]
    pub respects_dampening_zone: bool,
    /// DCB field: `dampeningZoneSize` (Single)
    #[serde(default)]
    pub dampening_zone_size: f32,
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
}

impl Pooled for ActorLookAheadPoint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_look_ahead_point }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_look_ahead_point }
}

impl<'a> Extract<'a> for ActorLookAheadPoint {
    const TYPE_NAME: &'static str = "ActorLookAheadPoint";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            limit_yaw_min_max: match inst.get("limitYawMinMax") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            limit_pitch_min_max: match inst.get("limitPitchMinMax") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            threshold_yaw_min_max: match inst.get("thresholdYawMinMax") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            threshold_pitch_min_max: match inst.get("thresholdPitchMinMax") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            multiplier_yaw: inst.get_f32("multiplierYaw").unwrap_or_default(),
            multiplier_pitch: inst.get_f32("multiplierPitch").unwrap_or_default(),
            default_weight: inst.get_f32("defaultWeight").unwrap_or_default(),
            status_swap_time: inst.get_f32("statusSwapTime").unwrap_or_default(),
            respects_dampening_zone: inst.get_bool("respectsDampeningZone").unwrap_or_default(),
            dampening_zone_size: inst.get_f32("dampeningZoneSize").unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLookAheadRoll`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLookAheadRoll {
    /// DCB field: `inputThreshold` (Single)
    #[serde(default)]
    pub input_threshold: f32,
    /// DCB field: `outputMaxRollAngle` (Single)
    #[serde(default)]
    pub output_max_roll_angle: f32,
    /// DCB field: `angleMap` (Class)
    #[serde(default)]
    pub angle_map: Option<Handle<BezierCurve>>,
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
}

impl Pooled for ActorLookAheadRoll {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_look_ahead_roll }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_look_ahead_roll }
}

impl<'a> Extract<'a> for ActorLookAheadRoll {
    const TYPE_NAME: &'static str = "ActorLookAheadRoll";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input_threshold: inst.get_f32("inputThreshold").unwrap_or_default(),
            output_max_roll_angle: inst.get_f32("outputMaxRollAngle").unwrap_or_default(),
            angle_map: match inst.get("angleMap") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enabled: inst.get_bool("enabled").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLookAheadTargetTracking`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLookAheadTargetTracking {
    /// DCB field: `trackingThresholdNormal` (Single)
    #[serde(default)]
    pub tracking_threshold_normal: f32,
    /// DCB field: `trackingThresholdExtended` (Single)
    #[serde(default)]
    pub tracking_threshold_extended: f32,
    /// DCB field: `trackingThresholdGracePeriod` (Single)
    #[serde(default)]
    pub tracking_threshold_grace_period: f32,
}

impl Pooled for ActorLookAheadTargetTracking {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_look_ahead_target_tracking }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_look_ahead_target_tracking }
}

impl<'a> Extract<'a> for ActorLookAheadTargetTracking {
    const TYPE_NAME: &'static str = "ActorLookAheadTargetTracking";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tracking_threshold_normal: inst.get_f32("trackingThresholdNormal").unwrap_or_default(),
            tracking_threshold_extended: inst.get_f32("trackingThresholdExtended").unwrap_or_default(),
            tracking_threshold_grace_period: inst.get_f32("trackingThresholdGracePeriod").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLookAheadVehicle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLookAheadVehicle {
    /// DCB field: `lookAheadPoints` (Class)
    #[serde(default)]
    pub look_ahead_points: Option<Handle<ActorLookAheadPoint>>,
    /// DCB field: `lookAheadRolls` (Class)
    #[serde(default)]
    pub look_ahead_rolls: Option<Handle<ActorLookAheadRoll>>,
    /// DCB field: `undampedFrequency` (Class)
    #[serde(default)]
    pub undamped_frequency: Option<Handle<Vec3>>,
    /// DCB field: `dampingRatio` (Class)
    #[serde(default)]
    pub damping_ratio: Option<Handle<Vec3>>,
    /// DCB field: `vehicleVelocityTranslationLengthMax` (Single)
    #[serde(default)]
    pub vehicle_velocity_translation_length_max: f32,
    /// DCB field: `vehicleVelocityTranslationWeightModifier` (Class)
    #[serde(default)]
    pub vehicle_velocity_translation_weight_modifier: Option<Handle<BezierCurve>>,
    /// DCB field: `horizonAlignRollDampeningMaxAngle` (Single)
    #[serde(default)]
    pub horizon_align_roll_dampening_max_angle: f32,
    /// DCB field: `horizonAlignRollDampening` (Class)
    #[serde(default)]
    pub horizon_align_roll_dampening: Option<Handle<BezierCurve>>,
    /// DCB field: `yawPitchInputDivergenceThreshold` (Single)
    #[serde(default)]
    pub yaw_pitch_input_divergence_threshold: f32,
    /// DCB field: `jumpPointSplineLookAheadDistance` (Single)
    #[serde(default)]
    pub jump_point_spline_look_ahead_distance: f32,
    /// DCB field: `dampeningZoneCurve` (Class)
    #[serde(default)]
    pub dampening_zone_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `targetTracking` (Class)
    #[serde(default)]
    pub target_tracking: Option<Handle<ActorLookAheadTargetTracking>>,
    /// DCB field: `customPointWeight` (Single)
    #[serde(default)]
    pub custom_point_weight: f32,
}

impl Pooled for ActorLookAheadVehicle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_look_ahead_vehicle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_look_ahead_vehicle }
}

impl<'a> Extract<'a> for ActorLookAheadVehicle {
    const TYPE_NAME: &'static str = "ActorLookAheadVehicle";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            look_ahead_points: match inst.get("lookAheadPoints") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLookAheadPoint>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLookAheadPoint>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            look_ahead_rolls: match inst.get("lookAheadRolls") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLookAheadRoll>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLookAheadRoll>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            undamped_frequency: match inst.get("undampedFrequency") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damping_ratio: match inst.get("dampingRatio") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vehicle_velocity_translation_length_max: inst.get_f32("vehicleVelocityTranslationLengthMax").unwrap_or_default(),
            vehicle_velocity_translation_weight_modifier: match inst.get("vehicleVelocityTranslationWeightModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            horizon_align_roll_dampening_max_angle: inst.get_f32("horizonAlignRollDampeningMaxAngle").unwrap_or_default(),
            horizon_align_roll_dampening: match inst.get("horizonAlignRollDampening") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw_pitch_input_divergence_threshold: inst.get_f32("yawPitchInputDivergenceThreshold").unwrap_or_default(),
            jump_point_spline_look_ahead_distance: inst.get_f32("jumpPointSplineLookAheadDistance").unwrap_or_default(),
            dampening_zone_curve: match inst.get("dampeningZoneCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            target_tracking: match inst.get("targetTracking") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLookAheadTargetTracking>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLookAheadTargetTracking>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            custom_point_weight: inst.get_f32("customPointWeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorDefaultActionsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorDefaultActionsConfig {
    /// DCB field: `versionID` (Int32)
    #[serde(default)]
    pub version_id: i32,
    /// DCB field: `defaultActionsList` (Class (array))
    #[serde(default)]
    pub default_actions_list: Vec<Handle<DefaultActionsEntry>>,
    /// DCB field: `defaultActionsDescriptions` (Class (array))
    #[serde(default)]
    pub default_actions_descriptions: Vec<Handle<DefaultActionDescriptionOverride>>,
}

impl Pooled for ActorDefaultActionsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_default_actions_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_default_actions_config }
}

impl<'a> Extract<'a> for ActorDefaultActionsConfig {
    const TYPE_NAME: &'static str = "ActorDefaultActionsConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            version_id: inst.get_i32("versionID").unwrap_or_default(),
            default_actions_list: inst.get_array("defaultActionsList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionsEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionsEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_actions_descriptions: inst.get_array("defaultActionsDescriptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionDescriptionOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionDescriptionOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLandingNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLandingNode {
    /// DCB field: `configVariant` (EnumChoice)
    #[serde(default)]
    pub config_variant: String,
    /// DCB field: `landingAnimations` (Class (array))
    #[serde(default)]
    pub landing_animations: Vec<Handle<LandingAnimationSetup>>,
    /// DCB field: `proceduralLandingSetup` (Reference)
    #[serde(default)]
    pub procedural_landing_setup: Option<CigGuid>,
}

impl Pooled for ActorLandingNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_landing_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_landing_node }
}

impl<'a> Extract<'a> for ActorLandingNode {
    const TYPE_NAME: &'static str = "ActorLandingNode";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            config_variant: inst.get_str("configVariant").map(String::from).unwrap_or_default(),
            landing_animations: inst.get_array("landingAnimations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LandingAnimationSetup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LandingAnimationSetup>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            procedural_landing_setup: inst.get("proceduralLandingSetup").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ActorFallOverlayNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorFallOverlayNode {
    /// DCB field: `configVariant` (EnumChoice)
    #[serde(default)]
    pub config_variant: String,
    /// DCB field: `arc` (Class)
    #[serde(default)]
    pub arc: Option<Handle<FragmentInfo>>,
    /// DCB field: `loop` (Class)
    #[serde(default)]
    pub r#loop: Option<Handle<FragmentRequiredInfo>>,
    /// DCB field: `outro` (Class)
    #[serde(default)]
    pub outro: Option<Handle<FragmentInfo>>,
    /// DCB field: `transitionFromInterruptDelay` (Single)
    #[serde(default)]
    pub transition_from_interrupt_delay: f32,
    /// DCB field: `transitionFromInterruptDuration` (Single)
    #[serde(default)]
    pub transition_from_interrupt_duration: f32,
}

impl Pooled for ActorFallOverlayNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_fall_overlay_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_fall_overlay_node }
}

impl<'a> Extract<'a> for ActorFallOverlayNode {
    const TYPE_NAME: &'static str = "ActorFallOverlayNode";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            config_variant: inst.get_str("configVariant").map(String::from).unwrap_or_default(),
            arc: match inst.get("arc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FragmentInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FragmentInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            r#loop: match inst.get("loop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FragmentRequiredInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FragmentRequiredInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outro: match inst.get("outro") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FragmentInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FragmentInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            transition_from_interrupt_delay: inst.get_f32("transitionFromInterruptDelay").unwrap_or_default(),
            transition_from_interrupt_duration: inst.get_f32("transitionFromInterruptDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorFallNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorFallNode {
    /// DCB field: `configVariant` (EnumChoice)
    #[serde(default)]
    pub config_variant: String,
    /// DCB field: `intro` (Class)
    #[serde(default)]
    pub intro: Option<Handle<FragmentInfo>>,
    /// DCB field: `loop` (Class)
    #[serde(default)]
    pub r#loop: Option<Handle<FragmentRequiredInfo>>,
    /// DCB field: `outro` (Class)
    #[serde(default)]
    pub outro: Option<Handle<FragmentInfo>>,
    /// DCB field: `introUseHandOverlay` (Boolean)
    #[serde(default)]
    pub intro_use_hand_overlay: bool,
    /// DCB field: `loopUseHandOverlay` (Boolean)
    #[serde(default)]
    pub loop_use_hand_overlay: bool,
    /// DCB field: `outroUseHandOverlay` (Boolean)
    #[serde(default)]
    pub outro_use_hand_overlay: bool,
    /// DCB field: `introAllowTurning` (Boolean)
    #[serde(default)]
    pub intro_allow_turning: bool,
    /// DCB field: `loopAllowTurning` (Boolean)
    #[serde(default)]
    pub loop_allow_turning: bool,
    /// DCB field: `outroAllowTurning` (Boolean)
    #[serde(default)]
    pub outro_allow_turning: bool,
    /// DCB field: `introStanceCheck` (Class)
    #[serde(default)]
    pub intro_stance_check: Option<Handle<StanceCheckConfig>>,
    /// DCB field: `loopStanceCheckDelay` (Single)
    #[serde(default)]
    pub loop_stance_check_delay: f32,
    /// DCB field: `loopStanceCheck` (Class)
    #[serde(default)]
    pub loop_stance_check: Option<Handle<StanceCheckConfig>>,
    /// DCB field: `outroStanceCheck` (Class)
    #[serde(default)]
    pub outro_stance_check: Option<Handle<StanceCheckConfig>>,
    /// DCB field: `loopBlendStartTime` (Single)
    #[serde(default)]
    pub loop_blend_start_time: f32,
    /// DCB field: `loopBlendEndTime` (Single)
    #[serde(default)]
    pub loop_blend_end_time: f32,
    /// DCB field: `outroTimeToLand` (Single)
    #[serde(default)]
    pub outro_time_to_land: f32,
    /// DCB field: `outroMinDelay` (Single)
    #[serde(default)]
    pub outro_min_delay: f32,
    /// DCB field: `outroMinSpeed` (Single)
    #[serde(default)]
    pub outro_min_speed: f32,
    /// DCB field: `fallOverlay` (WeakPointer)
    #[serde(default)]
    pub fall_overlay: Option<Handle<ActorFallOverlayNode>>,
    /// DCB field: `landing` (WeakPointer)
    #[serde(default)]
    pub landing: Option<Handle<ActorLandingNode>>,
    /// DCB field: `alternateLandings` (Class (array))
    #[serde(default)]
    pub alternate_landings: Vec<Handle<LandingSelection>>,
}

impl Pooled for ActorFallNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_fall_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_fall_node }
}

impl<'a> Extract<'a> for ActorFallNode {
    const TYPE_NAME: &'static str = "ActorFallNode";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            config_variant: inst.get_str("configVariant").map(String::from).unwrap_or_default(),
            intro: match inst.get("intro") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FragmentInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FragmentInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            r#loop: match inst.get("loop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FragmentRequiredInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FragmentRequiredInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outro: match inst.get("outro") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FragmentInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FragmentInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            intro_use_hand_overlay: inst.get_bool("introUseHandOverlay").unwrap_or_default(),
            loop_use_hand_overlay: inst.get_bool("loopUseHandOverlay").unwrap_or_default(),
            outro_use_hand_overlay: inst.get_bool("outroUseHandOverlay").unwrap_or_default(),
            intro_allow_turning: inst.get_bool("introAllowTurning").unwrap_or_default(),
            loop_allow_turning: inst.get_bool("loopAllowTurning").unwrap_or_default(),
            outro_allow_turning: inst.get_bool("outroAllowTurning").unwrap_or_default(),
            intro_stance_check: match inst.get("introStanceCheck") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StanceCheckConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StanceCheckConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loop_stance_check_delay: inst.get_f32("loopStanceCheckDelay").unwrap_or_default(),
            loop_stance_check: match inst.get("loopStanceCheck") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StanceCheckConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StanceCheckConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outro_stance_check: match inst.get("outroStanceCheck") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StanceCheckConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StanceCheckConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loop_blend_start_time: inst.get_f32("loopBlendStartTime").unwrap_or_default(),
            loop_blend_end_time: inst.get_f32("loopBlendEndTime").unwrap_or_default(),
            outro_time_to_land: inst.get_f32("outroTimeToLand").unwrap_or_default(),
            outro_min_delay: inst.get_f32("outroMinDelay").unwrap_or_default(),
            outro_min_speed: inst.get_f32("outroMinSpeed").unwrap_or_default(),
            fall_overlay: match inst.get("fallOverlay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorFallOverlayNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorFallOverlayNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            landing: match inst.get("landing") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLandingNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLandingNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            alternate_landings: inst.get_array("alternateLandings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LandingSelection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LandingSelection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorJumpNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorJumpNode {
    /// DCB field: `configVariant` (EnumChoice)
    #[serde(default)]
    pub config_variant: String,
    /// DCB field: `launch` (Class)
    #[serde(default)]
    pub launch: Option<Handle<FragmentInfo>>,
    /// DCB field: `arc` (Class)
    #[serde(default)]
    pub arc: Option<Handle<FragmentInfo>>,
    /// DCB field: `arcUseHandOverlay` (Boolean)
    #[serde(default)]
    pub arc_use_hand_overlay: bool,
    /// DCB field: `arcAllowTurning` (Boolean)
    #[serde(default)]
    pub arc_allow_turning: bool,
    /// DCB field: `durationTags` (Class (array))
    #[serde(default)]
    pub duration_tags: Vec<Handle<DurationTags>>,
    /// DCB field: `fallOverlay` (WeakPointer)
    #[serde(default)]
    pub fall_overlay: Option<Handle<ActorFallOverlayNode>>,
    /// DCB field: `fall` (WeakPointer)
    #[serde(default)]
    pub fall: Option<Handle<ActorFallNode>>,
    /// DCB field: `landing` (WeakPointer)
    #[serde(default)]
    pub landing: Option<Handle<ActorLandingNode>>,
}

impl Pooled for ActorJumpNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_jump_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_jump_node }
}

impl<'a> Extract<'a> for ActorJumpNode {
    const TYPE_NAME: &'static str = "ActorJumpNode";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            config_variant: inst.get_str("configVariant").map(String::from).unwrap_or_default(),
            launch: match inst.get("launch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FragmentInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FragmentInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            arc: match inst.get("arc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FragmentInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FragmentInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            arc_use_hand_overlay: inst.get_bool("arcUseHandOverlay").unwrap_or_default(),
            arc_allow_turning: inst.get_bool("arcAllowTurning").unwrap_or_default(),
            duration_tags: inst.get_array("durationTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DurationTags>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DurationTags>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            fall_overlay: match inst.get("fallOverlay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorFallOverlayNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorFallOverlayNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fall: match inst.get("fall") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorFallNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorFallNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            landing: match inst.get("landing") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLandingNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLandingNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorJumpFallLandVariantConfigNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorJumpFallLandVariantConfigNode {
    /// DCB field: `variant` (EnumChoice)
    #[serde(default)]
    pub variant: String,
    /// DCB field: `jump` (WeakPointer)
    #[serde(default)]
    pub jump: Option<Handle<ActorJumpNode>>,
    /// DCB field: `fall` (WeakPointer)
    #[serde(default)]
    pub fall: Option<Handle<ActorFallNode>>,
    /// DCB field: `land` (WeakPointer)
    #[serde(default)]
    pub land: Option<Handle<ActorLandingNode>>,
}

impl Pooled for ActorJumpFallLandVariantConfigNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_jump_fall_land_variant_config_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_jump_fall_land_variant_config_node }
}

impl<'a> Extract<'a> for ActorJumpFallLandVariantConfigNode {
    const TYPE_NAME: &'static str = "ActorJumpFallLandVariantConfigNode";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variant: inst.get_str("variant").map(String::from).unwrap_or_default(),
            jump: match inst.get("jump") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorJumpNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorJumpNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fall: match inst.get("fall") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorFallNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorFallNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            land: match inst.get("land") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLandingNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLandingNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorTargetedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorTargetedParams {
    /// DCB field: `frustumScaleW` (Single)
    #[serde(default)]
    pub frustum_scale_w: f32,
    /// DCB field: `frustumScaleH` (Single)
    #[serde(default)]
    pub frustum_scale_h: f32,
    /// DCB field: `lookFovAngle` (Single)
    #[serde(default)]
    pub look_fov_angle: f32,
    /// DCB field: `frustumFar` (Single)
    #[serde(default)]
    pub frustum_far: f32,
}

impl Pooled for ActorTargetedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_targeted_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_targeted_params }
}

impl<'a> Extract<'a> for ActorTargetedParams {
    const TYPE_NAME: &'static str = "ActorTargetedParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            frustum_scale_w: inst.get_f32("frustumScaleW").unwrap_or_default(),
            frustum_scale_h: inst.get_f32("frustumScaleH").unwrap_or_default(),
            look_fov_angle: inst.get_f32("lookFovAngle").unwrap_or_default(),
            frustum_far: inst.get_f32("frustumFar").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStanceConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStanceConfig {
    /// DCB field: `stanceSpeeds` (Reference (array))
    #[serde(default)]
    pub stance_speeds: Vec<CigGuid>,
    /// DCB field: `stanceDimensions` (Reference (array))
    #[serde(default)]
    pub stance_dimensions: Vec<CigGuid>,
}

impl Pooled for ActorStanceConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_stance_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_stance_config }
}

impl<'a> Extract<'a> for ActorStanceConfig {
    const TYPE_NAME: &'static str = "ActorStanceConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            stance_speeds: inst.get_array("stanceSpeeds")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            stance_dimensions: inst.get_array("stanceDimensions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorSignatureMultiplierGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorSignatureMultiplierGlobalParams {
    /// DCB field: `bodyTemperatureToIRMultiplier` (Single)
    #[serde(default)]
    pub body_temperature_to_irmultiplier: f32,
    /// DCB field: `staminaToIRMultiplier` (Single)
    #[serde(default)]
    pub stamina_to_irmultiplier: f32,
    /// DCB field: `staminaToIRDecayDelay` (Single)
    #[serde(default)]
    pub stamina_to_irdecay_delay: f32,
    /// DCB field: `staminaToIRDecayRate` (Single)
    #[serde(default)]
    pub stamina_to_irdecay_rate: f32,
}

impl Pooled for ActorSignatureMultiplierGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_signature_multiplier_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_signature_multiplier_global_params }
}

impl<'a> Extract<'a> for ActorSignatureMultiplierGlobalParams {
    const TYPE_NAME: &'static str = "ActorSignatureMultiplierGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            body_temperature_to_irmultiplier: inst.get_f32("bodyTemperatureToIRMultiplier").unwrap_or_default(),
            stamina_to_irmultiplier: inst.get_f32("staminaToIRMultiplier").unwrap_or_default(),
            stamina_to_irdecay_delay: inst.get_f32("staminaToIRDecayDelay").unwrap_or_default(),
            stamina_to_irdecay_rate: inst.get_f32("staminaToIRDecayRate").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorRestrainPerAttackerConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRestrainPerAttackerConfig {
    /// DCB field: `attackerSkeleton` (Class)
    #[serde(default)]
    pub attacker_skeleton: Option<Handle<ActorStateSkeletonFilter>>,
    /// DCB field: `frontalQuadrantAngle` (Single)
    #[serde(default)]
    pub frontal_quadrant_angle: f32,
    /// DCB field: `immuneToRestrain` (Boolean)
    #[serde(default)]
    pub immune_to_restrain: bool,
    /// DCB field: `interruptOnHitReaction` (Boolean)
    #[serde(default)]
    pub interrupt_on_hit_reaction: bool,
}

impl Pooled for ActorRestrainPerAttackerConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_restrain_per_attacker_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_restrain_per_attacker_config }
}

impl<'a> Extract<'a> for ActorRestrainPerAttackerConfig {
    const TYPE_NAME: &'static str = "ActorRestrainPerAttackerConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attacker_skeleton: match inst.get("attackerSkeleton") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateSkeletonFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateSkeletonFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            frontal_quadrant_angle: inst.get_f32("frontalQuadrantAngle").unwrap_or_default(),
            immune_to_restrain: inst.get_bool("immuneToRestrain").unwrap_or_default(),
            interrupt_on_hit_reaction: inst.get_bool("interruptOnHitReaction").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorRestrainConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRestrainConfig {
    /// DCB field: `immuneToRestrain` (Boolean)
    #[serde(default)]
    pub immune_to_restrain: bool,
    /// DCB field: `perAttackerConfigs` (Class (array))
    #[serde(default)]
    pub per_attacker_configs: Vec<Handle<ActorRestrainPerAttackerConfig>>,
    /// DCB field: `restrainVisibilityCheckJoints` (String (array))
    #[serde(default)]
    pub restrain_visibility_check_joints: Vec<String>,
}

impl Pooled for ActorRestrainConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_restrain_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_restrain_config }
}

impl<'a> Extract<'a> for ActorRestrainConfig {
    const TYPE_NAME: &'static str = "ActorRestrainConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            immune_to_restrain: inst.get_bool("immuneToRestrain").unwrap_or_default(),
            per_attacker_configs: inst.get_array("perAttackerConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorRestrainPerAttackerConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorRestrainPerAttackerConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            restrain_visibility_check_joints: inst.get_array("restrainVisibilityCheckJoints")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorViewLimits`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorViewLimits {
    /// DCB field: `minYaw` (Single)
    #[serde(default)]
    pub min_yaw: f32,
    /// DCB field: `maxYaw` (Single)
    #[serde(default)]
    pub max_yaw: f32,
    /// DCB field: `minPitch` (Single)
    #[serde(default)]
    pub min_pitch: f32,
    /// DCB field: `maxPitch` (Single)
    #[serde(default)]
    pub max_pitch: f32,
    /// DCB field: `minRoll` (Single)
    #[serde(default)]
    pub min_roll: f32,
    /// DCB field: `maxRoll` (Single)
    #[serde(default)]
    pub max_roll: f32,
    /// DCB field: `useEllipse` (Boolean)
    #[serde(default)]
    pub use_ellipse: bool,
}

impl Pooled for ActorViewLimits {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_view_limits }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_view_limits }
}

impl<'a> Extract<'a> for ActorViewLimits {
    const TYPE_NAME: &'static str = "ActorViewLimits";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_yaw: inst.get_f32("minYaw").unwrap_or_default(),
            max_yaw: inst.get_f32("maxYaw").unwrap_or_default(),
            min_pitch: inst.get_f32("minPitch").unwrap_or_default(),
            max_pitch: inst.get_f32("maxPitch").unwrap_or_default(),
            min_roll: inst.get_f32("minRoll").unwrap_or_default(),
            max_roll: inst.get_f32("maxRoll").unwrap_or_default(),
            use_ellipse: inst.get_bool("useEllipse").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorAimLimitsStateFilter`
///
/// Inherits from: `ActorStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorAimLimitsStateFilter {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// DCB field: `aimLimits` (Class)
    #[serde(default)]
    pub aim_limits: Option<Handle<ActorViewLimits>>,
}

impl Pooled for ActorAimLimitsStateFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_aim_limits_state_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_aim_limits_state_filter }
}

impl<'a> Extract<'a> for ActorAimLimitsStateFilter {
    const TYPE_NAME: &'static str = "ActorAimLimitsStateFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter_name: inst.get_str("filterName").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            filter_by_motion_speed: inst.get_str("filterByMotionSpeed").map(String::from).unwrap_or_default(),
            filter_by_pose_state: inst.get_str("filterByPoseState").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_lean_state: inst.get_str("filterByLeanState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
            filter_by_character_type: inst.get_str("filterByCharacterType").map(String::from).unwrap_or_default(),
            filter_by_restrained_state: inst.get_str("filterByRestrainedState").map(String::from).unwrap_or_default(),
            filter_by_player_camera: inst.get_str("filterByPlayerCamera").map(String::from).unwrap_or_default(),
            filter_by_aiming_restriction: inst.get_str("filterByAimingRestriction").map(String::from).unwrap_or_default(),
            aim_limits: match inst.get("aimLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorViewLimits>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorViewLimits>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorLookLimitsStateFilter`
///
/// Inherits from: `ActorStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLookLimitsStateFilter {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// DCB field: `lookLimits` (Class)
    #[serde(default)]
    pub look_limits: Option<Handle<ActorViewLimits>>,
}

impl Pooled for ActorLookLimitsStateFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_look_limits_state_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_look_limits_state_filter }
}

impl<'a> Extract<'a> for ActorLookLimitsStateFilter {
    const TYPE_NAME: &'static str = "ActorLookLimitsStateFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter_name: inst.get_str("filterName").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            filter_by_motion_speed: inst.get_str("filterByMotionSpeed").map(String::from).unwrap_or_default(),
            filter_by_pose_state: inst.get_str("filterByPoseState").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_lean_state: inst.get_str("filterByLeanState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
            filter_by_character_type: inst.get_str("filterByCharacterType").map(String::from).unwrap_or_default(),
            filter_by_restrained_state: inst.get_str("filterByRestrainedState").map(String::from).unwrap_or_default(),
            filter_by_player_camera: inst.get_str("filterByPlayerCamera").map(String::from).unwrap_or_default(),
            filter_by_aiming_restriction: inst.get_str("filterByAimingRestriction").map(String::from).unwrap_or_default(),
            look_limits: match inst.get("lookLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorViewLimits>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorViewLimits>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorViewLimitPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorViewLimitPreset {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `lookLimits` (Class)
    #[serde(default)]
    pub look_limits: Option<Handle<ActorViewLimits>>,
    /// DCB field: `aimLimits` (Class)
    #[serde(default)]
    pub aim_limits: Option<Handle<ActorViewLimits>>,
}

impl Pooled for ActorViewLimitPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_view_limit_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_view_limit_preset }
}

impl<'a> Extract<'a> for ActorViewLimitPreset {
    const TYPE_NAME: &'static str = "ActorViewLimitPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            look_limits: match inst.get("lookLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorViewLimits>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorViewLimits>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            aim_limits: match inst.get("aimLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorViewLimits>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorViewLimits>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorViewLimitPresetDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorViewLimitPresetDatabase {
    /// DCB field: `presets` (Class (array))
    #[serde(default)]
    pub presets: Vec<Handle<ActorViewLimitPreset>>,
}

impl Pooled for ActorViewLimitPresetDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_view_limit_preset_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_view_limit_preset_database }
}

impl<'a> Extract<'a> for ActorViewLimitPresetDatabase {
    const TYPE_NAME: &'static str = "ActorViewLimitPresetDatabase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            presets: inst.get_array("presets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorViewLimitPreset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorViewLimitPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLookLimits`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLookLimits {
    /// DCB field: `lookLimitsStateFilters` (Class (array))
    #[serde(default)]
    pub look_limits_state_filters: Vec<Handle<ActorLookLimitsStateFilter>>,
}

impl Pooled for ActorLookLimits {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_look_limits }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_look_limits }
}

impl<'a> Extract<'a> for ActorLookLimits {
    const TYPE_NAME: &'static str = "ActorLookLimits";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            look_limits_state_filters: inst.get_array("lookLimitsStateFilters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorLookLimitsStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorLookLimitsStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorAimLimits`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorAimLimits {
    /// DCB field: `aimLimitsStateFilters` (Class (array))
    #[serde(default)]
    pub aim_limits_state_filters: Vec<Handle<ActorAimLimitsStateFilter>>,
}

impl Pooled for ActorAimLimits {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.actor_aim_limits }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.actor_aim_limits }
}

impl<'a> Extract<'a> for ActorAimLimits {
    const TYPE_NAME: &'static str = "ActorAimLimits";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aim_limits_state_filters: inst.get_array("aimLimitsStateFilters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorAimLimitsStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorAimLimitsStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActionRuleDisplayParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRuleDisplayParams {
    /// DCB field: `exclusiveDisplay` (Boolean)
    #[serde(default)]
    pub exclusive_display: bool,
    /// DCB field: `blockedText` (StrongPointer)
    #[serde(default)]
    pub blocked_text: Option<Handle<BlockedTextParams>>,
}

impl Pooled for ActionRuleDisplayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.action_rule_display_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.action_rule_display_params }
}

impl<'a> Extract<'a> for ActionRuleDisplayParams {
    const TYPE_NAME: &'static str = "ActionRuleDisplayParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            exclusive_display: inst.get_bool("exclusiveDisplay").unwrap_or_default(),
            blocked_text: match inst.get("blockedText") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BlockedTextParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BlockedTextParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActionRuleParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRuleParams {
    /// DCB field: `ruleDisplay` (StrongPointer)
    #[serde(default)]
    pub rule_display: Option<Handle<ActionRuleDisplayParams>>,
}

impl Pooled for ActionRuleParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.action_rule_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.action_rule_params }
}

impl<'a> Extract<'a> for ActionRuleParams {
    const TYPE_NAME: &'static str = "ActionRuleParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rule_display: match inst.get("ruleDisplay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActionRuleDisplayParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActionRuleDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActionRuleList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRuleList {
    /// DCB field: `inputAction` (Class)
    #[serde(default)]
    pub input_action: Option<Handle<InputAction>>,
    /// DCB field: `preset` (Reference)
    #[serde(default)]
    pub preset: Option<CigGuid>,
    /// DCB field: `ruleParams` (StrongPointer (array))
    #[serde(default)]
    pub rule_params: Vec<Handle<ActionRuleParams>>,
}

impl Pooled for ActionRuleList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ac.action_rule_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ac.action_rule_list }
}

impl<'a> Extract<'a> for ActionRuleList {
    const TYPE_NAME: &'static str = "ActionRuleList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input_action: match inst.get("inputAction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            preset: inst.get("preset").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rule_params: inst.get_array("ruleParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActionRuleParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActionRuleParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

