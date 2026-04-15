// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `turret`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `STurretHealthModifierDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STurretHealthModifierDef {
    /// `damageMovementModifier` (Class)
    #[serde(default)]
    pub damage_movement_modifier: Option<Handle<BezierCurve>>,
}

impl Pooled for STurretHealthModifierDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.turret.sturret_health_modifier_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.turret.sturret_health_modifier_def }
}

impl<'a> Extract<'a> for STurretHealthModifierDef {
    const TYPE_NAME: &'static str = "STurretHealthModifierDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_movement_modifier: match inst.get("damageMovementModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `STurretESP`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STurretESP {
    /// `triggerZoneRampInCurve` (Class)
    #[serde(default)]
    pub trigger_zone_ramp_in_curve: Option<Handle<BezierCurve>>,
    /// `maxTrackingStrength` (Single)
    #[serde(default)]
    pub max_tracking_strength: f32,
    /// `distanceFalloffStart` (Single)
    #[serde(default)]
    pub distance_falloff_start: f32,
    /// `distanceFalloffEnd` (Single)
    #[serde(default)]
    pub distance_falloff_end: f32,
    /// `outerZoneDeg` (Single)
    #[serde(default)]
    pub outer_zone_deg: f32,
    /// `innerZoneRatio` (Single)
    #[serde(default)]
    pub inner_zone_ratio: f32,
    /// `adsZoneMinSizeDeg` (Single)
    #[serde(default)]
    pub ads_zone_min_size_deg: f32,
    /// `inputScalerMin` (Single)
    #[serde(default)]
    pub input_scaler_min: f32,
    /// `inputScalerMax` (Single)
    #[serde(default)]
    pub input_scaler_max: f32,
    /// `allowWithRelativeMouseModes` (Boolean)
    #[serde(default)]
    pub allow_with_relative_mouse_modes: bool,
}

impl Pooled for STurretESP {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.turret.sturret_esp }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.turret.sturret_esp }
}

impl<'a> Extract<'a> for STurretESP {
    const TYPE_NAME: &'static str = "STurretESP";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_zone_ramp_in_curve: match inst.get("triggerZoneRampInCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            max_tracking_strength: inst.get_f32("maxTrackingStrength").unwrap_or_default(),
            distance_falloff_start: inst.get_f32("distanceFalloffStart").unwrap_or_default(),
            distance_falloff_end: inst.get_f32("distanceFalloffEnd").unwrap_or_default(),
            outer_zone_deg: inst.get_f32("outerZoneDeg").unwrap_or_default(),
            inner_zone_ratio: inst.get_f32("innerZoneRatio").unwrap_or_default(),
            ads_zone_min_size_deg: inst.get_f32("adsZoneMinSizeDeg").unwrap_or_default(),
            input_scaler_min: inst.get_f32("inputScalerMin").unwrap_or_default(),
            input_scaler_max: inst.get_f32("inputScalerMax").unwrap_or_default(),
            allow_with_relative_mouse_modes: inst.get_bool("allowWithRelativeMouseModes").unwrap_or_default(),
        }
    }
}

/// DCB type: `STurretGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STurretGlobalParams {
    /// `pointerModeAllowed` (Boolean)
    #[serde(default)]
    pub pointer_mode_allowed: bool,
    /// `pointerModeInputSmoothing` (Single)
    #[serde(default)]
    pub pointer_mode_input_smoothing: f32,
    /// `pointerModeInnerAngle` (Single)
    #[serde(default)]
    pub pointer_mode_inner_angle: f32,
    /// `pointerModeInnerAngleMaxSpeedModifier` (Single)
    #[serde(default)]
    pub pointer_mode_inner_angle_max_speed_modifier: f32,
    /// `pointerModeInnerAngleTurretSmoothing` (Single)
    #[serde(default)]
    pub pointer_mode_inner_angle_turret_smoothing: f32,
    /// `pointerModeMiddleAngle` (Single)
    #[serde(default)]
    pub pointer_mode_middle_angle: f32,
    /// `pointerModeOuterAngle` (Single)
    #[serde(default)]
    pub pointer_mode_outer_angle: f32,
    /// `pointerModeOuterAngleTurretSmoothing` (Single)
    #[serde(default)]
    pub pointer_mode_outer_angle_turret_smoothing: f32,
    /// `pointerModeMaxDegPerSec` (Single)
    #[serde(default)]
    pub pointer_mode_max_deg_per_sec: f32,
    /// `relativeInputAllowed` (Boolean)
    #[serde(default)]
    pub relative_input_allowed: bool,
    /// `relativeInputSmoothing` (Single)
    #[serde(default)]
    pub relative_input_smoothing: f32,
}

impl Pooled for STurretGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.turret.sturret_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.turret.sturret_global_params }
}

impl<'a> Extract<'a> for STurretGlobalParams {
    const TYPE_NAME: &'static str = "STurretGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            pointer_mode_allowed: inst.get_bool("pointerModeAllowed").unwrap_or_default(),
            pointer_mode_input_smoothing: inst.get_f32("pointerModeInputSmoothing").unwrap_or_default(),
            pointer_mode_inner_angle: inst.get_f32("pointerModeInnerAngle").unwrap_or_default(),
            pointer_mode_inner_angle_max_speed_modifier: inst.get_f32("pointerModeInnerAngleMaxSpeedModifier").unwrap_or_default(),
            pointer_mode_inner_angle_turret_smoothing: inst.get_f32("pointerModeInnerAngleTurretSmoothing").unwrap_or_default(),
            pointer_mode_middle_angle: inst.get_f32("pointerModeMiddleAngle").unwrap_or_default(),
            pointer_mode_outer_angle: inst.get_f32("pointerModeOuterAngle").unwrap_or_default(),
            pointer_mode_outer_angle_turret_smoothing: inst.get_f32("pointerModeOuterAngleTurretSmoothing").unwrap_or_default(),
            pointer_mode_max_deg_per_sec: inst.get_f32("pointerModeMaxDegPerSec").unwrap_or_default(),
            relative_input_allowed: inst.get_bool("relativeInputAllowed").unwrap_or_default(),
            relative_input_smoothing: inst.get_f32("relativeInputSmoothing").unwrap_or_default(),
        }
    }
}

