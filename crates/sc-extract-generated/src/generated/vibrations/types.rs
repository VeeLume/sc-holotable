// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `vibrations`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `VibrationTypeData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationTypeData {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `fallOffTime` (Single)
    #[serde(default)]
    pub fall_off_time: f32,
}

impl Pooled for VibrationTypeData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vibrations.vibration_type_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vibrations.vibration_type_data }
}

impl<'a> Extract<'a> for VibrationTypeData {
    const TYPE_NAME: &'static str = "VibrationTypeData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            fall_off_time: inst.get_f32("fallOffTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `SVibrationDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVibrationDef {
    /// `vibrationTypes` (Class)
    #[serde(default)]
    pub vibration_types: Option<Handle<VibrationTypeData>>,
    /// `listensToPhysics` (Boolean)
    #[serde(default)]
    pub listens_to_physics: bool,
    /// `listensToDamage` (Boolean)
    #[serde(default)]
    pub listens_to_damage: bool,
    /// `listensToHits` (Boolean)
    #[serde(default)]
    pub listens_to_hits: bool,
}

impl Pooled for SVibrationDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vibrations.svibration_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vibrations.svibration_def }
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
    /// `increaseTime` (Single)
    #[serde(default)]
    pub increase_time: f32,
    /// `decayTime` (Single)
    #[serde(default)]
    pub decay_time: f32,
    /// `suppressionCurve` (Class)
    #[serde(default)]
    pub suppression_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SVibrationSuppression {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vibrations.svibration_suppression }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vibrations.svibration_suppression }
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

/// DCB type: `JumpPointVibrationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpPointVibrationParams {
    /// `openStrength` (Single)
    #[serde(default)]
    pub open_strength: f32,
    /// `closeStrength` (Single)
    #[serde(default)]
    pub close_strength: f32,
}

impl Pooled for JumpPointVibrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vibrations.jump_point_vibration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vibrations.jump_point_vibration_params }
}

impl<'a> Extract<'a> for JumpPointVibrationParams {
    const TYPE_NAME: &'static str = "JumpPointVibrationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            open_strength: inst.get_f32("openStrength").unwrap_or_default(),
            close_strength: inst.get_f32("closeStrength").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpDriveVibrationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveVibrationParams {
    /// `tuningCurve` (Class)
    #[serde(default)]
    pub tuning_curve: Option<Handle<BezierCurve>>,
    /// `distortionCurve` (Class)
    #[serde(default)]
    pub distortion_curve: Option<Handle<BezierCurve>>,
    /// `turbulenceCurve` (Class)
    #[serde(default)]
    pub turbulence_curve: Option<Handle<BezierCurve>>,
    /// `gustCurve` (Class)
    #[serde(default)]
    pub gust_curve: Option<Handle<BezierCurve>>,
    /// `boostCurve` (Class)
    #[serde(default)]
    pub boost_curve: Option<Handle<BezierCurve>>,
    /// `postExitStrength` (Single)
    #[serde(default)]
    pub post_exit_strength: f32,
    /// `postFailStrength` (Single)
    #[serde(default)]
    pub post_fail_strength: f32,
    /// `onStartedEnteringTunnelStrength` (Single)
    #[serde(default)]
    pub on_started_entering_tunnel_strength: f32,
    /// `enteredTunnelStrength` (Single)
    #[serde(default)]
    pub entered_tunnel_strength: f32,
    /// `enablePostTunedVibrations` (Boolean)
    #[serde(default)]
    pub enable_post_tuned_vibrations: bool,
    /// `jumpDriveNormalSuppressionIncreaseTime` (Single)
    #[serde(default)]
    pub jump_drive_normal_suppression_increase_time: f32,
    /// `jumpDriveNormalSuppressionDecayTime` (Single)
    #[serde(default)]
    pub jump_drive_normal_suppression_decay_time: f32,
    /// `jumpDriveNormalSuppressionCurve` (Class)
    #[serde(default)]
    pub jump_drive_normal_suppression_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for JumpDriveVibrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vibrations.jump_drive_vibration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vibrations.jump_drive_vibration_params }
}

impl<'a> Extract<'a> for JumpDriveVibrationParams {
    const TYPE_NAME: &'static str = "JumpDriveVibrationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tuning_curve: match inst.get("tuningCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distortion_curve: match inst.get("distortionCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turbulence_curve: match inst.get("turbulenceCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            gust_curve: match inst.get("gustCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            boost_curve: match inst.get("boostCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            post_exit_strength: inst.get_f32("postExitStrength").unwrap_or_default(),
            post_fail_strength: inst.get_f32("postFailStrength").unwrap_or_default(),
            on_started_entering_tunnel_strength: inst.get_f32("onStartedEnteringTunnelStrength").unwrap_or_default(),
            entered_tunnel_strength: inst.get_f32("enteredTunnelStrength").unwrap_or_default(),
            enable_post_tuned_vibrations: inst.get_bool("enablePostTunedVibrations").unwrap_or_default(),
            jump_drive_normal_suppression_increase_time: inst.get_f32("jumpDriveNormalSuppressionIncreaseTime").unwrap_or_default(),
            jump_drive_normal_suppression_decay_time: inst.get_f32("jumpDriveNormalSuppressionDecayTime").unwrap_or_default(),
            jump_drive_normal_suppression_curve: match inst.get("jumpDriveNormalSuppressionCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpTunnelVibrationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTunnelVibrationParams {
    /// `enteringCurve` (Class)
    #[serde(default)]
    pub entering_curve: Option<Handle<BezierCurve>>,
    /// `wallProximityCurve` (Class)
    #[serde(default)]
    pub wall_proximity_curve: Option<Handle<BezierCurve>>,
    /// `offAxisCurve` (Class)
    #[serde(default)]
    pub off_axis_curve: Option<Handle<BezierCurve>>,
    /// `failingCurve` (Class)
    #[serde(default)]
    pub failing_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for JumpTunnelVibrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vibrations.jump_tunnel_vibration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vibrations.jump_tunnel_vibration_params }
}

impl<'a> Extract<'a> for JumpTunnelVibrationParams {
    const TYPE_NAME: &'static str = "JumpTunnelVibrationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entering_curve: match inst.get("enteringCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wall_proximity_curve: match inst.get("wallProximityCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            off_axis_curve: match inst.get("offAxisCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            failing_curve: match inst.get("failingCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpSystemVibrationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpSystemVibrationParams {
    /// `jumpPoint` (Class)
    #[serde(default)]
    pub jump_point: Option<Handle<JumpPointVibrationParams>>,
    /// `jumpDrive` (Class)
    #[serde(default)]
    pub jump_drive: Option<Handle<JumpDriveVibrationParams>>,
    /// `jumpTunnel` (Class)
    #[serde(default)]
    pub jump_tunnel: Option<Handle<JumpTunnelVibrationParams>>,
    /// `movementNormal` (Class)
    #[serde(default)]
    pub movement_normal: Option<Handle<SVibrationSuppression>>,
    /// `movementAfterburner` (Class)
    #[serde(default)]
    pub movement_afterburner: Option<Handle<SVibrationSuppression>>,
}

impl Pooled for JumpSystemVibrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vibrations.jump_system_vibration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vibrations.jump_system_vibration_params }
}

impl<'a> Extract<'a> for JumpSystemVibrationParams {
    const TYPE_NAME: &'static str = "JumpSystemVibrationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            jump_point: match inst.get("jumpPoint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpPointVibrationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpPointVibrationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jump_drive: match inst.get("jumpDrive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveVibrationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveVibrationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jump_tunnel: match inst.get("jumpTunnel") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelVibrationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelVibrationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            movement_normal: match inst.get("movementNormal") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVibrationSuppression>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVibrationSuppression>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            movement_afterburner: match inst.get("movementAfterburner") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVibrationSuppression>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVibrationSuppression>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SVibrationReferenceData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVibrationReferenceData {
    /// `referenceValue` (Single)
    #[serde(default)]
    pub reference_value: f32,
    /// `referenceMapping` (Class)
    #[serde(default)]
    pub reference_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for SVibrationReferenceData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vibrations.svibration_reference_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vibrations.svibration_reference_data }
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
    /// `listensToVehicleHitEvents` (Boolean)
    #[serde(default)]
    pub listens_to_vehicle_hit_events: bool,
    /// `vehicleHitDamageMaxRatio` (Single)
    #[serde(default)]
    pub vehicle_hit_damage_max_ratio: f32,
    /// `vehicleHitSuppressionDecayTime` (Single)
    #[serde(default)]
    pub vehicle_hit_suppression_decay_time: f32,
    /// `vehicleHitSuppressionCurve` (Class)
    #[serde(default)]
    pub vehicle_hit_suppression_curve: Option<Handle<BezierCurve>>,
    /// `vehicleHitDamageVibrationMapping` (Class)
    #[serde(default)]
    pub vehicle_hit_damage_vibration_mapping: Option<Handle<BezierCurve>>,
    /// `dragModifier` (Single)
    #[serde(default)]
    pub drag_modifier: f32,
    /// `liftModifier` (Single)
    #[serde(default)]
    pub lift_modifier: f32,
    /// `masterModeDragModifier` (Single)
    #[serde(default)]
    pub master_mode_drag_modifier: f32,
    /// `thrusterModifier` (Single)
    #[serde(default)]
    pub thruster_modifier: f32,
    /// `thrusterNormalSuppressionIncreaseTime` (Single)
    #[serde(default)]
    pub thruster_normal_suppression_increase_time: f32,
    /// `thrusterNormalSuppressionDecayTime` (Single)
    #[serde(default)]
    pub thruster_normal_suppression_decay_time: f32,
    /// `thrusterNormalSuppressionCurve` (Class)
    #[serde(default)]
    pub thruster_normal_suppression_curve: Option<Handle<BezierCurve>>,
    /// `thrusterBoostRumbleStrength` (Single)
    #[serde(default)]
    pub thruster_boost_rumble_strength: f32,
    /// `thrusterBoostSuppressionIncreaseTime` (Single)
    #[serde(default)]
    pub thruster_boost_suppression_increase_time: f32,
    /// `thrusterBoostSuppressionDecayTime` (Single)
    #[serde(default)]
    pub thruster_boost_suppression_decay_time: f32,
    /// `thrusterBoostSuppressionCurve` (Class)
    #[serde(default)]
    pub thruster_boost_suppression_curve: Option<Handle<BezierCurve>>,
    /// `electricalChargeModifier` (Single)
    #[serde(default)]
    pub electrical_charge_modifier: f32,
    /// `deactivateWindCheckOnPlanets` (Boolean)
    #[serde(default)]
    pub deactivate_wind_check_on_planets: bool,
    /// `magLaunchInputModifier` (Single)
    #[serde(default)]
    pub mag_launch_input_modifier: f32,
    /// `jumpSystemVibrations` (Class)
    #[serde(default)]
    pub jump_system_vibrations: Option<Handle<JumpSystemVibrationParams>>,
    /// `thrusterNormal` (Class)
    #[serde(default)]
    pub thruster_normal: Option<Handle<SVibrationSuppression>>,
    /// `thrusterAfterburner` (Class)
    #[serde(default)]
    pub thruster_afterburner: Option<Handle<SVibrationSuppression>>,
    /// `thrusterAfterburnerRumbleRatio` (Single)
    #[serde(default)]
    pub thruster_afterburner_rumble_ratio: f32,
    /// `thrusterRawAngularAccelerationRatio` (Single)
    #[serde(default)]
    pub thruster_raw_angular_acceleration_ratio: f32,
    /// `aerodynamicsLinAccelerationVibration` (Class)
    #[serde(default)]
    pub aerodynamics_lin_acceleration_vibration: Option<Handle<SVibrationReferenceData>>,
    /// `aerodynamicsAngleOfAttackVibration` (Class)
    #[serde(default)]
    pub aerodynamics_angle_of_attack_vibration: Option<Handle<SVibrationReferenceData>>,
    /// `externalVibration` (Class)
    #[serde(default)]
    pub external_vibration: Option<Handle<SVibrationReferenceData>>,
    /// `windVibration` (Class)
    #[serde(default)]
    pub wind_vibration: Option<Handle<SVibrationReferenceData>>,
}

impl Pooled for SVibrationVehicleDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.vibrations.svibration_vehicle_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.vibrations.svibration_vehicle_def }
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

