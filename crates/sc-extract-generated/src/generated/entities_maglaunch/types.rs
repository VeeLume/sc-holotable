// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-maglaunch`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SMagLaunchRecoveryNoiseParams`
pub struct SMagLaunchRecoveryNoiseParams {
    /// `maxOffset` (Single)
    pub max_offset: f32,
    /// `frequencies` (Class)
    pub frequencies: Option<Handle<Vec3>>,
    /// `multiplier` (Single)
    pub multiplier: f32,
    /// `hurstIndex` (Single)
    pub hurst_index: f32,
    /// `threshold` (Single)
    pub threshold: f32,
}

impl Pooled for SMagLaunchRecoveryNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_maglaunch.smag_launch_recovery_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_maglaunch.smag_launch_recovery_noise_params }
}

impl<'a> Extract<'a> for SMagLaunchRecoveryNoiseParams {
    const TYPE_NAME: &'static str = "SMagLaunchRecoveryNoiseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_offset: inst.get_f32("maxOffset").unwrap_or_default(),
            frequencies: match inst.get("frequencies") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            multiplier: inst.get_f32("multiplier").unwrap_or_default(),
            hurst_index: inst.get_f32("hurstIndex").unwrap_or_default(),
            threshold: inst.get_f32("threshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `SMagLaunchMotionParams`
pub struct SMagLaunchMotionParams {
    /// `linAccelerationMax` (Single)
    pub lin_acceleration_max: f32,
    /// `linAccelerationJerkTime` (Single)
    pub lin_acceleration_jerk_time: f32,
    /// `linAccelerationDecay` (Single)
    pub lin_acceleration_decay: f32,
    /// `linSpeedMax` (Single)
    pub lin_speed_max: f32,
    /// `angAccelerationMax` (Single)
    pub ang_acceleration_max: f32,
    /// `angAccelerationJerkTime` (Single)
    pub ang_acceleration_jerk_time: f32,
    /// `angAccelerationDecay` (Single)
    pub ang_acceleration_decay: f32,
    /// `angSpeedMax` (Single)
    pub ang_speed_max: f32,
}

impl Pooled for SMagLaunchMotionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_maglaunch.smag_launch_motion_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_maglaunch.smag_launch_motion_params }
}

impl<'a> Extract<'a> for SMagLaunchMotionParams {
    const TYPE_NAME: &'static str = "SMagLaunchMotionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            lin_acceleration_max: inst.get_f32("linAccelerationMax").unwrap_or_default(),
            lin_acceleration_jerk_time: inst.get_f32("linAccelerationJerkTime").unwrap_or_default(),
            lin_acceleration_decay: inst.get_f32("linAccelerationDecay").unwrap_or_default(),
            lin_speed_max: inst.get_f32("linSpeedMax").unwrap_or_default(),
            ang_acceleration_max: inst.get_f32("angAccelerationMax").unwrap_or_default(),
            ang_acceleration_jerk_time: inst.get_f32("angAccelerationJerkTime").unwrap_or_default(),
            ang_acceleration_decay: inst.get_f32("angAccelerationDecay").unwrap_or_default(),
            ang_speed_max: inst.get_f32("angSpeedMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `MagLaunchParams`
/// Inherits from: `DataForgeComponentParams`
pub struct MagLaunchParams {
    /// `liftMotion` (Class)
    pub lift_motion: Option<Handle<SMagLaunchMotionParams>>,
    /// `launchMotion` (Class)
    pub launch_motion: Option<Handle<SMagLaunchMotionParams>>,
    /// `magLiftSpoolUpTime` (Single)
    pub mag_lift_spool_up_time: f32,
    /// `maxTimeWaitAfterLift` (Single)
    pub max_time_wait_after_lift: f32,
    /// `launchLength` (Single)
    pub launch_length: f32,
    /// `landingGearAutoRaise` (Boolean)
    pub landing_gear_auto_raise: bool,
    /// `landingGearAutoRaiseHoverRatio` (Single)
    pub landing_gear_auto_raise_hover_ratio: f32,
    /// `applyNoise` (Boolean)
    pub apply_noise: bool,
    /// `hoverNoiseLin` (Class)
    pub hover_noise_lin: Option<Handle<SMagLaunchRecoveryNoiseParams>>,
    /// `hoverNoiseAng` (Class)
    pub hover_noise_ang: Option<Handle<SMagLaunchRecoveryNoiseParams>>,
}

impl Pooled for MagLaunchParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_maglaunch.mag_launch_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_maglaunch.mag_launch_params }
}

impl<'a> Extract<'a> for MagLaunchParams {
    const TYPE_NAME: &'static str = "MagLaunchParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lift_motion: match inst.get("liftMotion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMagLaunchMotionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            launch_motion: match inst.get("launchMotion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMagLaunchMotionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            mag_lift_spool_up_time: inst.get_f32("magLiftSpoolUpTime").unwrap_or_default(),
            max_time_wait_after_lift: inst.get_f32("maxTimeWaitAfterLift").unwrap_or_default(),
            launch_length: inst.get_f32("launchLength").unwrap_or_default(),
            landing_gear_auto_raise: inst.get_bool("landingGearAutoRaise").unwrap_or_default(),
            landing_gear_auto_raise_hover_ratio: inst.get_f32("landingGearAutoRaiseHoverRatio").unwrap_or_default(),
            apply_noise: inst.get_bool("applyNoise").unwrap_or_default(),
            hover_noise_lin: match inst.get("hoverNoiseLin") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMagLaunchRecoveryNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hover_noise_ang: match inst.get("hoverNoiseAng") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMagLaunchRecoveryNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

