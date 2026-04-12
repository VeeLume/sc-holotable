// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-actormovables`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SMovableLimits`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMovableLimits {
    /// `veloctiyForwardClamp` (Single)
    #[serde(default)]
    pub veloctiy_forward_clamp: f32,
    /// `veloctiyBackwardClamp` (Single)
    #[serde(default)]
    pub veloctiy_backward_clamp: f32,
    /// `veloctiySideClamp` (Single)
    #[serde(default)]
    pub veloctiy_side_clamp: f32,
    /// `maxYawSpeed` (Single)
    #[serde(default)]
    pub max_yaw_speed: f32,
    /// `maxLinearAcceleration` (Single)
    #[serde(default)]
    pub max_linear_acceleration: f32,
    /// `maxAngularAcceleration` (Single)
    #[serde(default)]
    pub max_angular_acceleration: f32,
    /// `linearAccelerationEasingPower` (Single)
    #[serde(default)]
    pub linear_acceleration_easing_power: f32,
    /// `linearAccelerationEasingMinSpeed` (Single)
    #[serde(default)]
    pub linear_acceleration_easing_min_speed: f32,
    /// `linearAccelerationEasingMaxSpeed` (Single)
    #[serde(default)]
    pub linear_acceleration_easing_max_speed: f32,
    /// `angularAccelerationEasingPower` (Single)
    #[serde(default)]
    pub angular_acceleration_easing_power: f32,
    /// `angularAccelerationEasingMinSpeed` (Single)
    #[serde(default)]
    pub angular_acceleration_easing_min_speed: f32,
    /// `angularAccelerationEasingMaxSpeed` (Single)
    #[serde(default)]
    pub angular_acceleration_easing_max_speed: f32,
    /// `rotLinModifier` (Single)
    #[serde(default)]
    pub rot_lin_modifier: f32,
    /// `lateralDamping` (Single)
    #[serde(default)]
    pub lateral_damping: f32,
    /// `yawDamping` (Single)
    #[serde(default)]
    pub yaw_damping: f32,
    /// `leanMaxSlopeAngle` (Single)
    #[serde(default)]
    pub lean_max_slope_angle: f32,
    /// `leanMultiplier` (Single)
    #[serde(default)]
    pub lean_multiplier: f32,
    /// `allowSprint` (Boolean)
    #[serde(default)]
    pub allow_sprint: bool,
    /// `velocitySprintClamp` (Single)
    #[serde(default)]
    pub velocity_sprint_clamp: f32,
}

impl Pooled for SMovableLimits {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_actormovables.smovable_limits }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_actormovables.smovable_limits }
}

impl<'a> Extract<'a> for SMovableLimits {
    const TYPE_NAME: &'static str = "SMovableLimits";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            veloctiy_forward_clamp: inst.get_f32("veloctiyForwardClamp").unwrap_or_default(),
            veloctiy_backward_clamp: inst.get_f32("veloctiyBackwardClamp").unwrap_or_default(),
            veloctiy_side_clamp: inst.get_f32("veloctiySideClamp").unwrap_or_default(),
            max_yaw_speed: inst.get_f32("maxYawSpeed").unwrap_or_default(),
            max_linear_acceleration: inst.get_f32("maxLinearAcceleration").unwrap_or_default(),
            max_angular_acceleration: inst.get_f32("maxAngularAcceleration").unwrap_or_default(),
            linear_acceleration_easing_power: inst.get_f32("linearAccelerationEasingPower").unwrap_or_default(),
            linear_acceleration_easing_min_speed: inst.get_f32("linearAccelerationEasingMinSpeed").unwrap_or_default(),
            linear_acceleration_easing_max_speed: inst.get_f32("linearAccelerationEasingMaxSpeed").unwrap_or_default(),
            angular_acceleration_easing_power: inst.get_f32("angularAccelerationEasingPower").unwrap_or_default(),
            angular_acceleration_easing_min_speed: inst.get_f32("angularAccelerationEasingMinSpeed").unwrap_or_default(),
            angular_acceleration_easing_max_speed: inst.get_f32("angularAccelerationEasingMaxSpeed").unwrap_or_default(),
            rot_lin_modifier: inst.get_f32("rotLinModifier").unwrap_or_default(),
            lateral_damping: inst.get_f32("lateralDamping").unwrap_or_default(),
            yaw_damping: inst.get_f32("yawDamping").unwrap_or_default(),
            lean_max_slope_angle: inst.get_f32("leanMaxSlopeAngle").unwrap_or_default(),
            lean_multiplier: inst.get_f32("leanMultiplier").unwrap_or_default(),
            allow_sprint: inst.get_bool("allowSprint").unwrap_or_default(),
            velocity_sprint_clamp: inst.get_f32("velocitySprintClamp").unwrap_or_default(),
        }
    }
}

