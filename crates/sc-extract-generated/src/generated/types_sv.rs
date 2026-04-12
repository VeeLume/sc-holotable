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

/// DCB type: `SVibrationDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVibrationDef {
    /// DCB field: `vibrationTypes` (Class)
    #[serde(default)]
    pub vibration_types: Option<Handle<VibrationTypeData>>,
    /// DCB field: `listensToPhysics` (Boolean)
    #[serde(default)]
    pub listens_to_physics: bool,
    /// DCB field: `listensToDamage` (Boolean)
    #[serde(default)]
    pub listens_to_damage: bool,
    /// DCB field: `listensToHits` (Boolean)
    #[serde(default)]
    pub listens_to_hits: bool,
}

impl Pooled for SVibrationDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sv.svibration_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sv.svibration_def }
}

impl<'a> Extract<'a> for SVibrationDef {
    const TYPE_NAME: &'static str = "SVibrationDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vibration_types: match inst.get("vibrationTypes") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VibrationTypeData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VibrationTypeData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            listens_to_physics: inst.get_bool("listensToPhysics").unwrap_or_default(),
            listens_to_damage: inst.get_bool("listensToDamage").unwrap_or_default(),
            listens_to_hits: inst.get_bool("listensToHits").unwrap_or_default(),
        }
    }
}

/// DCB type: `SVibrationSuppression`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVibrationSuppression {
    /// DCB field: `increaseTime` (Single)
    #[serde(default)]
    pub increase_time: f32,
    /// DCB field: `decayTime` (Single)
    #[serde(default)]
    pub decay_time: f32,
    /// DCB field: `suppressionCurve` (Class)
    #[serde(default)]
    pub suppression_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SVibrationSuppression {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sv.svibration_suppression }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sv.svibration_suppression }
}

impl<'a> Extract<'a> for SVibrationSuppression {
    const TYPE_NAME: &'static str = "SVibrationSuppression";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            increase_time: inst.get_f32("increaseTime").unwrap_or_default(),
            decay_time: inst.get_f32("decayTime").unwrap_or_default(),
            suppression_curve: match inst.get("suppressionCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SVibrationReferenceData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVibrationReferenceData {
    /// DCB field: `referenceValue` (Single)
    #[serde(default)]
    pub reference_value: f32,
    /// DCB field: `referenceMapping` (Class)
    #[serde(default)]
    pub reference_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for SVibrationReferenceData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sv.svibration_reference_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sv.svibration_reference_data }
}

impl<'a> Extract<'a> for SVibrationReferenceData {
    const TYPE_NAME: &'static str = "SVibrationReferenceData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reference_value: inst.get_f32("referenceValue").unwrap_or_default(),
            reference_mapping: match inst.get("referenceMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SVibrationVehicleDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVibrationVehicleDef {
    /// DCB field: `listensToVehicleHitEvents` (Boolean)
    #[serde(default)]
    pub listens_to_vehicle_hit_events: bool,
    /// DCB field: `vehicleHitDamageMaxRatio` (Single)
    #[serde(default)]
    pub vehicle_hit_damage_max_ratio: f32,
    /// DCB field: `vehicleHitSuppressionDecayTime` (Single)
    #[serde(default)]
    pub vehicle_hit_suppression_decay_time: f32,
    /// DCB field: `vehicleHitSuppressionCurve` (Class)
    #[serde(default)]
    pub vehicle_hit_suppression_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `vehicleHitDamageVibrationMapping` (Class)
    #[serde(default)]
    pub vehicle_hit_damage_vibration_mapping: Option<Handle<BezierCurve>>,
    /// DCB field: `dragModifier` (Single)
    #[serde(default)]
    pub drag_modifier: f32,
    /// DCB field: `liftModifier` (Single)
    #[serde(default)]
    pub lift_modifier: f32,
    /// DCB field: `masterModeDragModifier` (Single)
    #[serde(default)]
    pub master_mode_drag_modifier: f32,
    /// DCB field: `thrusterModifier` (Single)
    #[serde(default)]
    pub thruster_modifier: f32,
    /// DCB field: `thrusterNormalSuppressionIncreaseTime` (Single)
    #[serde(default)]
    pub thruster_normal_suppression_increase_time: f32,
    /// DCB field: `thrusterNormalSuppressionDecayTime` (Single)
    #[serde(default)]
    pub thruster_normal_suppression_decay_time: f32,
    /// DCB field: `thrusterNormalSuppressionCurve` (Class)
    #[serde(default)]
    pub thruster_normal_suppression_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `thrusterBoostRumbleStrength` (Single)
    #[serde(default)]
    pub thruster_boost_rumble_strength: f32,
    /// DCB field: `thrusterBoostSuppressionIncreaseTime` (Single)
    #[serde(default)]
    pub thruster_boost_suppression_increase_time: f32,
    /// DCB field: `thrusterBoostSuppressionDecayTime` (Single)
    #[serde(default)]
    pub thruster_boost_suppression_decay_time: f32,
    /// DCB field: `thrusterBoostSuppressionCurve` (Class)
    #[serde(default)]
    pub thruster_boost_suppression_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `electricalChargeModifier` (Single)
    #[serde(default)]
    pub electrical_charge_modifier: f32,
    /// DCB field: `deactivateWindCheckOnPlanets` (Boolean)
    #[serde(default)]
    pub deactivate_wind_check_on_planets: bool,
    /// DCB field: `magLaunchInputModifier` (Single)
    #[serde(default)]
    pub mag_launch_input_modifier: f32,
    /// DCB field: `jumpSystemVibrations` (Class)
    #[serde(default)]
    pub jump_system_vibrations: Option<Handle<JumpSystemVibrationParams>>,
    /// DCB field: `thrusterNormal` (Class)
    #[serde(default)]
    pub thruster_normal: Option<Handle<SVibrationSuppression>>,
    /// DCB field: `thrusterAfterburner` (Class)
    #[serde(default)]
    pub thruster_afterburner: Option<Handle<SVibrationSuppression>>,
    /// DCB field: `thrusterAfterburnerRumbleRatio` (Single)
    #[serde(default)]
    pub thruster_afterburner_rumble_ratio: f32,
    /// DCB field: `thrusterRawAngularAccelerationRatio` (Single)
    #[serde(default)]
    pub thruster_raw_angular_acceleration_ratio: f32,
    /// DCB field: `aerodynamicsLinAccelerationVibration` (Class)
    #[serde(default)]
    pub aerodynamics_lin_acceleration_vibration: Option<Handle<SVibrationReferenceData>>,
    /// DCB field: `aerodynamicsAngleOfAttackVibration` (Class)
    #[serde(default)]
    pub aerodynamics_angle_of_attack_vibration: Option<Handle<SVibrationReferenceData>>,
    /// DCB field: `externalVibration` (Class)
    #[serde(default)]
    pub external_vibration: Option<Handle<SVibrationReferenceData>>,
    /// DCB field: `windVibration` (Class)
    #[serde(default)]
    pub wind_vibration: Option<Handle<SVibrationReferenceData>>,
}

impl Pooled for SVibrationVehicleDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sv.svibration_vehicle_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sv.svibration_vehicle_def }
}

impl<'a> Extract<'a> for SVibrationVehicleDef {
    const TYPE_NAME: &'static str = "SVibrationVehicleDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            listens_to_vehicle_hit_events: inst.get_bool("listensToVehicleHitEvents").unwrap_or_default(),
            vehicle_hit_damage_max_ratio: inst.get_f32("vehicleHitDamageMaxRatio").unwrap_or_default(),
            vehicle_hit_suppression_decay_time: inst.get_f32("vehicleHitSuppressionDecayTime").unwrap_or_default(),
            vehicle_hit_suppression_curve: match inst.get("vehicleHitSuppressionCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vehicle_hit_damage_vibration_mapping: match inst.get("vehicleHitDamageVibrationMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drag_modifier: inst.get_f32("dragModifier").unwrap_or_default(),
            lift_modifier: inst.get_f32("liftModifier").unwrap_or_default(),
            master_mode_drag_modifier: inst.get_f32("masterModeDragModifier").unwrap_or_default(),
            thruster_modifier: inst.get_f32("thrusterModifier").unwrap_or_default(),
            thruster_normal_suppression_increase_time: inst.get_f32("thrusterNormalSuppressionIncreaseTime").unwrap_or_default(),
            thruster_normal_suppression_decay_time: inst.get_f32("thrusterNormalSuppressionDecayTime").unwrap_or_default(),
            thruster_normal_suppression_curve: match inst.get("thrusterNormalSuppressionCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            thruster_boost_rumble_strength: inst.get_f32("thrusterBoostRumbleStrength").unwrap_or_default(),
            thruster_boost_suppression_increase_time: inst.get_f32("thrusterBoostSuppressionIncreaseTime").unwrap_or_default(),
            thruster_boost_suppression_decay_time: inst.get_f32("thrusterBoostSuppressionDecayTime").unwrap_or_default(),
            thruster_boost_suppression_curve: match inst.get("thrusterBoostSuppressionCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            electrical_charge_modifier: inst.get_f32("electricalChargeModifier").unwrap_or_default(),
            deactivate_wind_check_on_planets: inst.get_bool("deactivateWindCheckOnPlanets").unwrap_or_default(),
            mag_launch_input_modifier: inst.get_f32("magLaunchInputModifier").unwrap_or_default(),
            jump_system_vibrations: match inst.get("jumpSystemVibrations") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpSystemVibrationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpSystemVibrationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            thruster_normal: match inst.get("thrusterNormal") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVibrationSuppression>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVibrationSuppression>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            thruster_afterburner: match inst.get("thrusterAfterburner") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVibrationSuppression>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVibrationSuppression>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            thruster_afterburner_rumble_ratio: inst.get_f32("thrusterAfterburnerRumbleRatio").unwrap_or_default(),
            thruster_raw_angular_acceleration_ratio: inst.get_f32("thrusterRawAngularAccelerationRatio").unwrap_or_default(),
            aerodynamics_lin_acceleration_vibration: match inst.get("aerodynamicsLinAccelerationVibration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVibrationReferenceData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVibrationReferenceData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            aerodynamics_angle_of_attack_vibration: match inst.get("aerodynamicsAngleOfAttackVibration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVibrationReferenceData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVibrationReferenceData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            external_vibration: match inst.get("externalVibration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVibrationReferenceData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVibrationReferenceData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wind_vibration: match inst.get("windVibration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVibrationReferenceData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVibrationReferenceData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SViewDistanceRatioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SViewDistanceRatioParams {
    /// DCB field: `viewDistRatio` (Int32)
    #[serde(default)]
    pub view_dist_ratio: i32,
}

impl Pooled for SViewDistanceRatioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sv.sview_distance_ratio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sv.sview_distance_ratio_params }
}

impl<'a> Extract<'a> for SViewDistanceRatioParams {
    const TYPE_NAME: &'static str = "SViewDistanceRatioParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            view_dist_ratio: inst.get_i32("viewDistRatio").unwrap_or_default(),
        }
    }
}

/// DCB type: `SVehicleHudParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVehicleHudParams {
    /// DCB field: `altitudeTape` (Class)
    #[serde(default)]
    pub altitude_tape: Option<Handle<SHudTapeParams>>,
    /// DCB field: `radarAltimeterWidgetThreshold` (Single)
    #[serde(default)]
    pub radar_altimeter_widget_threshold: f32,
    /// DCB field: `compassTape` (Class)
    #[serde(default)]
    pub compass_tape: Option<Handle<SHudTapeParams>>,
}

impl Pooled for SVehicleHudParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sv.svehicle_hud_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sv.svehicle_hud_params }
}

impl<'a> Extract<'a> for SVehicleHudParams {
    const TYPE_NAME: &'static str = "SVehicleHudParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            altitude_tape: match inst.get("altitudeTape") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHudTapeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHudTapeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            radar_altimeter_widget_threshold: inst.get_f32("radarAltimeterWidgetThreshold").unwrap_or_default(),
            compass_tape: match inst.get("compassTape") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHudTapeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHudTapeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SVehicleAiDamageModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVehicleAiDamageModifiers {
    /// DCB field: `modifiers` (Single)
    #[serde(default)]
    pub modifiers: f32,
}

impl Pooled for SVehicleAiDamageModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sv.svehicle_ai_damage_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sv.svehicle_ai_damage_modifiers }
}

impl<'a> Extract<'a> for SVehicleAiDamageModifiers {
    const TYPE_NAME: &'static str = "SVehicleAiDamageModifiers";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            modifiers: inst.get_f32("modifiers").unwrap_or_default(),
        }
    }
}

/// DCB type: `SVehicleAfterburnerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVehicleAfterburnerParams {
    /// DCB field: `preDelayTime` (Single)
    #[serde(default)]
    pub pre_delay_time: f32,
    /// DCB field: `rampUpTime` (Single)
    #[serde(default)]
    pub ramp_up_time: f32,
    /// DCB field: `rampDownTime` (Single)
    #[serde(default)]
    pub ramp_down_time: f32,
    /// DCB field: `thresholdRatio` (Single)
    #[serde(default)]
    pub threshold_ratio: f32,
    /// DCB field: `capacity` (Single)
    #[serde(default)]
    pub capacity: f32,
    /// DCB field: `costPerSec` (Single)
    #[serde(default)]
    pub cost_per_sec: f32,
    /// DCB field: `assignmentCostModifier` (Class)
    #[serde(default)]
    pub assignment_cost_modifier: Option<Handle<BezierCurve>>,
    /// DCB field: `regenPerSec` (Single)
    #[serde(default)]
    pub regen_per_sec: f32,
    /// DCB field: `assignmentRegenModifier` (Class)
    #[serde(default)]
    pub assignment_regen_modifier: Option<Handle<BezierCurve>>,
    /// DCB field: `regenDelayAfterUse` (Single)
    #[serde(default)]
    pub regen_delay_after_use: f32,
    /// DCB field: `linearAccelMultiplierPositive` (Class)
    #[serde(default)]
    pub linear_accel_multiplier_positive: Option<Handle<Vec3>>,
    /// DCB field: `linearAccelMultiplierNegative` (Class)
    #[serde(default)]
    pub linear_accel_multiplier_negative: Option<Handle<Vec3>>,
    /// DCB field: `assignmentModifierLin` (Class)
    #[serde(default)]
    pub assignment_modifier_lin: Option<Handle<BezierCurve>>,
    /// DCB field: `angularAccelMultiplier` (Class)
    #[serde(default)]
    pub angular_accel_multiplier: Option<Handle<Vec3>>,
    /// DCB field: `assignmentModifierAng` (Class)
    #[serde(default)]
    pub assignment_modifier_ang: Option<Handle<BezierCurve>>,
    /// DCB field: `allowTriggerDuringRampdown` (Boolean)
    #[serde(default)]
    pub allow_trigger_during_rampdown: bool,
    /// DCB field: `enableAntiSpam` (Boolean)
    #[serde(default)]
    pub enable_anti_spam: bool,
    /// DCB field: `startAtFullCapacity` (Boolean)
    #[serde(default)]
    pub start_at_full_capacity: bool,
}

impl Pooled for SVehicleAfterburnerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sv.svehicle_afterburner_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sv.svehicle_afterburner_params }
}

impl<'a> Extract<'a> for SVehicleAfterburnerParams {
    const TYPE_NAME: &'static str = "SVehicleAfterburnerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pre_delay_time: inst.get_f32("preDelayTime").unwrap_or_default(),
            ramp_up_time: inst.get_f32("rampUpTime").unwrap_or_default(),
            ramp_down_time: inst.get_f32("rampDownTime").unwrap_or_default(),
            threshold_ratio: inst.get_f32("thresholdRatio").unwrap_or_default(),
            capacity: inst.get_f32("capacity").unwrap_or_default(),
            cost_per_sec: inst.get_f32("costPerSec").unwrap_or_default(),
            assignment_cost_modifier: match inst.get("assignmentCostModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            regen_per_sec: inst.get_f32("regenPerSec").unwrap_or_default(),
            assignment_regen_modifier: match inst.get("assignmentRegenModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            regen_delay_after_use: inst.get_f32("regenDelayAfterUse").unwrap_or_default(),
            linear_accel_multiplier_positive: match inst.get("linearAccelMultiplierPositive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear_accel_multiplier_negative: match inst.get("linearAccelMultiplierNegative") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            assignment_modifier_lin: match inst.get("assignmentModifierLin") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angular_accel_multiplier: match inst.get("angularAccelMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            assignment_modifier_ang: match inst.get("assignmentModifierAng") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            allow_trigger_during_rampdown: inst.get_bool("allowTriggerDuringRampdown").unwrap_or_default(),
            enable_anti_spam: inst.get_bool("enableAntiSpam").unwrap_or_default(),
            start_at_full_capacity: inst.get_bool("startAtFullCapacity").unwrap_or_default(),
        }
    }
}

/// DCB type: `SVecWithNoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVecWithNoiseParams {
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// DCB field: `noise` (Class)
    #[serde(default)]
    pub noise: Option<Handle<SHeadRecoilNoiseParams>>,
}

impl Pooled for SVecWithNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sv.svec_with_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sv.svec_with_noise_params }
}

impl<'a> Extract<'a> for SVecWithNoiseParams {
    const TYPE_NAME: &'static str = "SVecWithNoiseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise: match inst.get("noise") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHeadRecoilNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHeadRecoilNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

