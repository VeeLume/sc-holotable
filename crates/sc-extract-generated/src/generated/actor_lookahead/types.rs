// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-lookahead`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ActorLookAheadPoint`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLookAheadPoint {
    /// `limitYawMinMax` (Class)
    #[serde(default)]
    pub limit_yaw_min_max: Option<Handle<Vec2>>,
    /// `limitPitchMinMax` (Class)
    #[serde(default)]
    pub limit_pitch_min_max: Option<Handle<Vec2>>,
    /// `thresholdYawMinMax` (Class)
    #[serde(default)]
    pub threshold_yaw_min_max: Option<Handle<Vec2>>,
    /// `thresholdPitchMinMax` (Class)
    #[serde(default)]
    pub threshold_pitch_min_max: Option<Handle<Vec2>>,
    /// `multiplierYaw` (Single)
    #[serde(default)]
    pub multiplier_yaw: f32,
    /// `multiplierPitch` (Single)
    #[serde(default)]
    pub multiplier_pitch: f32,
    /// `defaultWeight` (Single)
    #[serde(default)]
    pub default_weight: f32,
    /// `statusSwapTime` (Single)
    #[serde(default)]
    pub status_swap_time: f32,
    /// `respectsDampeningZone` (Boolean)
    #[serde(default)]
    pub respects_dampening_zone: bool,
    /// `dampeningZoneSize` (Single)
    #[serde(default)]
    pub dampening_zone_size: f32,
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
}

impl Pooled for ActorLookAheadPoint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_lookahead.actor_look_ahead_point }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_lookahead.actor_look_ahead_point }
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
    /// `inputThreshold` (Single)
    #[serde(default)]
    pub input_threshold: f32,
    /// `outputMaxRollAngle` (Single)
    #[serde(default)]
    pub output_max_roll_angle: f32,
    /// `angleMap` (Class)
    #[serde(default)]
    pub angle_map: Option<Handle<BezierCurve>>,
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
}

impl Pooled for ActorLookAheadRoll {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_lookahead.actor_look_ahead_roll }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_lookahead.actor_look_ahead_roll }
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
    /// `trackingThresholdNormal` (Single)
    #[serde(default)]
    pub tracking_threshold_normal: f32,
    /// `trackingThresholdExtended` (Single)
    #[serde(default)]
    pub tracking_threshold_extended: f32,
    /// `trackingThresholdGracePeriod` (Single)
    #[serde(default)]
    pub tracking_threshold_grace_period: f32,
}

impl Pooled for ActorLookAheadTargetTracking {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_lookahead.actor_look_ahead_target_tracking }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_lookahead.actor_look_ahead_target_tracking }
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
    /// `lookAheadPoints` (Class)
    #[serde(default)]
    pub look_ahead_points: Option<Handle<ActorLookAheadPoint>>,
    /// `lookAheadRolls` (Class)
    #[serde(default)]
    pub look_ahead_rolls: Option<Handle<ActorLookAheadRoll>>,
    /// `undampedFrequency` (Class)
    #[serde(default)]
    pub undamped_frequency: Option<Handle<Vec3>>,
    /// `dampingRatio` (Class)
    #[serde(default)]
    pub damping_ratio: Option<Handle<Vec3>>,
    /// `vehicleVelocityTranslationLengthMax` (Single)
    #[serde(default)]
    pub vehicle_velocity_translation_length_max: f32,
    /// `vehicleVelocityTranslationWeightModifier` (Class)
    #[serde(default)]
    pub vehicle_velocity_translation_weight_modifier: Option<Handle<BezierCurve>>,
    /// `horizonAlignRollDampeningMaxAngle` (Single)
    #[serde(default)]
    pub horizon_align_roll_dampening_max_angle: f32,
    /// `horizonAlignRollDampening` (Class)
    #[serde(default)]
    pub horizon_align_roll_dampening: Option<Handle<BezierCurve>>,
    /// `yawPitchInputDivergenceThreshold` (Single)
    #[serde(default)]
    pub yaw_pitch_input_divergence_threshold: f32,
    /// `jumpPointSplineLookAheadDistance` (Single)
    #[serde(default)]
    pub jump_point_spline_look_ahead_distance: f32,
    /// `dampeningZoneCurve` (Class)
    #[serde(default)]
    pub dampening_zone_curve: Option<Handle<BezierCurve>>,
    /// `targetTracking` (Class)
    #[serde(default)]
    pub target_tracking: Option<Handle<ActorLookAheadTargetTracking>>,
    /// `customPointWeight` (Single)
    #[serde(default)]
    pub custom_point_weight: f32,
}

impl Pooled for ActorLookAheadVehicle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_lookahead.actor_look_ahead_vehicle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_lookahead.actor_look_ahead_vehicle }
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

