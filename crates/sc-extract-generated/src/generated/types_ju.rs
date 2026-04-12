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

/// DCB type: `JumpPointVibrationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpPointVibrationParams {
    /// DCB field: `openStrength` (Single)
    #[serde(default)]
    pub open_strength: f32,
    /// DCB field: `closeStrength` (Single)
    #[serde(default)]
    pub close_strength: f32,
}

impl Pooled for JumpPointVibrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_point_vibration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_point_vibration_params }
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
    /// DCB field: `tuningCurve` (Class)
    #[serde(default)]
    pub tuning_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `distortionCurve` (Class)
    #[serde(default)]
    pub distortion_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `turbulenceCurve` (Class)
    #[serde(default)]
    pub turbulence_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `gustCurve` (Class)
    #[serde(default)]
    pub gust_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `boostCurve` (Class)
    #[serde(default)]
    pub boost_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `postExitStrength` (Single)
    #[serde(default)]
    pub post_exit_strength: f32,
    /// DCB field: `postFailStrength` (Single)
    #[serde(default)]
    pub post_fail_strength: f32,
    /// DCB field: `onStartedEnteringTunnelStrength` (Single)
    #[serde(default)]
    pub on_started_entering_tunnel_strength: f32,
    /// DCB field: `enteredTunnelStrength` (Single)
    #[serde(default)]
    pub entered_tunnel_strength: f32,
    /// DCB field: `enablePostTunedVibrations` (Boolean)
    #[serde(default)]
    pub enable_post_tuned_vibrations: bool,
    /// DCB field: `jumpDriveNormalSuppressionIncreaseTime` (Single)
    #[serde(default)]
    pub jump_drive_normal_suppression_increase_time: f32,
    /// DCB field: `jumpDriveNormalSuppressionDecayTime` (Single)
    #[serde(default)]
    pub jump_drive_normal_suppression_decay_time: f32,
    /// DCB field: `jumpDriveNormalSuppressionCurve` (Class)
    #[serde(default)]
    pub jump_drive_normal_suppression_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for JumpDriveVibrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_vibration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_vibration_params }
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
    /// DCB field: `enteringCurve` (Class)
    #[serde(default)]
    pub entering_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `wallProximityCurve` (Class)
    #[serde(default)]
    pub wall_proximity_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `offAxisCurve` (Class)
    #[serde(default)]
    pub off_axis_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `failingCurve` (Class)
    #[serde(default)]
    pub failing_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for JumpTunnelVibrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_tunnel_vibration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_tunnel_vibration_params }
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
    /// DCB field: `jumpPoint` (Class)
    #[serde(default)]
    pub jump_point: Option<Handle<JumpPointVibrationParams>>,
    /// DCB field: `jumpDrive` (Class)
    #[serde(default)]
    pub jump_drive: Option<Handle<JumpDriveVibrationParams>>,
    /// DCB field: `jumpTunnel` (Class)
    #[serde(default)]
    pub jump_tunnel: Option<Handle<JumpTunnelVibrationParams>>,
    /// DCB field: `movementNormal` (Class)
    #[serde(default)]
    pub movement_normal: Option<Handle<SVibrationSuppression>>,
    /// DCB field: `movementAfterburner` (Class)
    #[serde(default)]
    pub movement_afterburner: Option<Handle<SVibrationSuppression>>,
}

impl Pooled for JumpSystemVibrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_system_vibration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_system_vibration_params }
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

/// DCB type: `JumpFallLandConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpFallLandConfig {
    /// DCB field: `jumpNodes` (StrongPointer (array))
    #[serde(default)]
    pub jump_nodes: Vec<Handle<ActorJumpNode>>,
    /// DCB field: `fallNodes` (StrongPointer (array))
    #[serde(default)]
    pub fall_nodes: Vec<Handle<ActorFallNode>>,
    /// DCB field: `landingNodes` (StrongPointer (array))
    #[serde(default)]
    pub landing_nodes: Vec<Handle<ActorLandingNode>>,
    /// DCB field: `fallOverlayNodes` (StrongPointer (array))
    #[serde(default)]
    pub fall_overlay_nodes: Vec<Handle<ActorFallOverlayNode>>,
    /// DCB field: `variantConfigNodes` (StrongPointer (array))
    #[serde(default)]
    pub variant_config_nodes: Vec<Handle<ActorJumpFallLandVariantConfigNode>>,
}

impl Pooled for JumpFallLandConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_fall_land_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_fall_land_config }
}

impl<'a> Extract<'a> for JumpFallLandConfig {
    const TYPE_NAME: &'static str = "JumpFallLandConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            jump_nodes: inst.get_array("jumpNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorJumpNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorJumpNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            fall_nodes: inst.get_array("fallNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorFallNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorFallNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            landing_nodes: inst.get_array("landingNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorLandingNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorLandingNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            fall_overlay_nodes: inst.get_array("fallOverlayNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorFallOverlayNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorFallOverlayNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            variant_config_nodes: inst.get_array("variantConfigNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorJumpFallLandVariantConfigNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorJumpFallLandVariantConfigNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpDriveUIConeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveUIConeParams {
    /// DCB field: `range` (Single)
    #[serde(default)]
    pub range: f32,
    /// DCB field: `angle` (Single)
    #[serde(default)]
    pub angle: f32,
    /// DCB field: `lookAtAngle` (Single)
    #[serde(default)]
    pub look_at_angle: f32,
}

impl Pooled for JumpDriveUIConeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_uicone_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_uicone_params }
}

impl<'a> Extract<'a> for JumpDriveUIConeParams {
    const TYPE_NAME: &'static str = "JumpDriveUIConeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            range: inst.get_f32("range").unwrap_or_default(),
            angle: inst.get_f32("angle").unwrap_or_default(),
            look_at_angle: inst.get_f32("lookAtAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpTunnelCameraEffectParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTunnelCameraEffectParam {
    /// DCB field: `referenceValue` (Single)
    #[serde(default)]
    pub reference_value: f32,
}

impl Pooled for JumpTunnelCameraEffectParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_tunnel_camera_effect_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_tunnel_camera_effect_param }
}

impl<'a> Extract<'a> for JumpTunnelCameraEffectParam {
    const TYPE_NAME: &'static str = "JumpTunnelCameraEffectParam";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reference_value: inst.get_f32("referenceValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpTunnelCameraEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTunnelCameraEffects {
    /// DCB field: `blur` (Class)
    #[serde(default)]
    pub blur: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// DCB field: `bloom` (Class)
    #[serde(default)]
    pub bloom: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// DCB field: `chromaticAberation` (Class)
    #[serde(default)]
    pub chromatic_aberation: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// DCB field: `shutterSpeed` (Class)
    #[serde(default)]
    pub shutter_speed: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// DCB field: `fov` (Class)
    #[serde(default)]
    pub fov: Option<Handle<JumpTunnelCameraEffectParam>>,
    /// DCB field: `fovScale` (Class)
    #[serde(default)]
    pub fov_scale: Option<Handle<JumpTunnelCameraEffectParam>>,
}

impl Pooled for JumpTunnelCameraEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_tunnel_camera_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_tunnel_camera_effects }
}

impl<'a> Extract<'a> for JumpTunnelCameraEffects {
    const TYPE_NAME: &'static str = "JumpTunnelCameraEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            blur: match inst.get("blur") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bloom: match inst.get("bloom") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            chromatic_aberation: match inst.get("chromaticAberation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shutter_speed: match inst.get("shutterSpeed") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fov: match inst.get("fov") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fov_scale: match inst.get("fovScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpTunnelCameraEffectParam>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveVelocityStrengthParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveVelocityStrengthParams {
    /// DCB field: `maxStrengthAngle` (Single)
    #[serde(default)]
    pub max_strength_angle: f32,
    /// DCB field: `minStrengthAngle` (Single)
    #[serde(default)]
    pub min_strength_angle: f32,
}

impl Pooled for JumpDriveVelocityStrengthParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_velocity_strength_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_velocity_strength_params }
}

impl<'a> Extract<'a> for JumpDriveVelocityStrengthParams {
    const TYPE_NAME: &'static str = "JumpDriveVelocityStrengthParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_strength_angle: inst.get_f32("maxStrengthAngle").unwrap_or_default(),
            min_strength_angle: inst.get_f32("minStrengthAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpDriveFlightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveFlightParams {
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<SJumpDriveFlightRotationParams>>,
    /// DCB field: `linear` (Class)
    #[serde(default)]
    pub linear: Option<Handle<SJumpDriveFlightLinearParams>>,
    /// DCB field: `steering` (Class)
    #[serde(default)]
    pub steering: Option<Handle<SJumpDriveFlightSteeringParams>>,
    /// DCB field: `afterburner` (Class)
    #[serde(default)]
    pub afterburner: Option<Handle<SVehicleAfterburnerParams>>,
    /// DCB field: `turbulence` (Class)
    #[serde(default)]
    pub turbulence: Option<Handle<SJumpDriveFlightTurbulenceParams>>,
    /// DCB field: `respoolTime` (Single)
    #[serde(default)]
    pub respool_time: f32,
    /// DCB field: `exitRecoverySpeed` (Single)
    #[serde(default)]
    pub exit_recovery_speed: f32,
}

impl Pooled for JumpDriveFlightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_flight_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_flight_params }
}

impl<'a> Extract<'a> for JumpDriveFlightParams {
    const TYPE_NAME: &'static str = "JumpDriveFlightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightRotationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightRotationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear: match inst.get("linear") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightLinearParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightLinearParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            steering: match inst.get("steering") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightSteeringParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightSteeringParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            afterburner: match inst.get("afterburner") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVehicleAfterburnerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVehicleAfterburnerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turbulence: match inst.get("turbulence") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            respool_time: inst.get_f32("respoolTime").unwrap_or_default(),
            exit_recovery_speed: inst.get_f32("exitRecoverySpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpTunnelForcesParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTunnelForcesParams {
    /// DCB field: `angularAccelerationLimit` (Single)
    #[serde(default)]
    pub angular_acceleration_limit: f32,
    /// DCB field: `angularAccelerationDecay` (Single)
    #[serde(default)]
    pub angular_acceleration_decay: f32,
    /// DCB field: `angularSpeedLimit` (Single)
    #[serde(default)]
    pub angular_speed_limit: f32,
    /// DCB field: `angularCorrectionForceCurve` (Class)
    #[serde(default)]
    pub angular_correction_force_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `useAngularForces` (Boolean)
    #[serde(default)]
    pub use_angular_forces: bool,
    /// DCB field: `linearAccelerationLimit` (Single)
    #[serde(default)]
    pub linear_acceleration_limit: f32,
    /// DCB field: `linearAccelerationDecay` (Single)
    #[serde(default)]
    pub linear_acceleration_decay: f32,
    /// DCB field: `linearSpeedLimit` (Single)
    #[serde(default)]
    pub linear_speed_limit: f32,
    /// DCB field: `useWallRepelForces` (Boolean)
    #[serde(default)]
    pub use_wall_repel_forces: bool,
}

impl Pooled for JumpTunnelForcesParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_tunnel_forces_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_tunnel_forces_params }
}

impl<'a> Extract<'a> for JumpTunnelForcesParams {
    const TYPE_NAME: &'static str = "JumpTunnelForcesParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            angular_acceleration_limit: inst.get_f32("angularAccelerationLimit").unwrap_or_default(),
            angular_acceleration_decay: inst.get_f32("angularAccelerationDecay").unwrap_or_default(),
            angular_speed_limit: inst.get_f32("angularSpeedLimit").unwrap_or_default(),
            angular_correction_force_curve: match inst.get("angularCorrectionForceCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            use_angular_forces: inst.get_bool("useAngularForces").unwrap_or_default(),
            linear_acceleration_limit: inst.get_f32("linearAccelerationLimit").unwrap_or_default(),
            linear_acceleration_decay: inst.get_f32("linearAccelerationDecay").unwrap_or_default(),
            linear_speed_limit: inst.get_f32("linearSpeedLimit").unwrap_or_default(),
            use_wall_repel_forces: inst.get_bool("useWallRepelForces").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpDriveStateAudioMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveStateAudioMap {
    /// DCB field: `enterStateLoop` (Class)
    #[serde(default)]
    pub enter_state_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `enterStateOneShot` (Class)
    #[serde(default)]
    pub enter_state_one_shot: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `exitStateLoop` (Class)
    #[serde(default)]
    pub exit_state_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `jumpDriveState` (EnumChoice)
    #[serde(default)]
    pub jump_drive_state: String,
}

impl Pooled for JumpDriveStateAudioMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_state_audio_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_state_audio_map }
}

impl<'a> Extract<'a> for JumpDriveStateAudioMap {
    const TYPE_NAME: &'static str = "JumpDriveStateAudioMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enter_state_loop: match inst.get("enterStateLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enter_state_one_shot: match inst.get("enterStateOneShot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exit_state_loop: match inst.get("exitStateLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jump_drive_state: inst.get_str("jumpDriveState").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpDriveAudioMovementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveAudioMovementParams {
    /// DCB field: `shipLinearAccelerationRL` (Class)
    #[serde(default)]
    pub ship_linear_acceleration_rl: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipLinearAccelerationFB` (Class)
    #[serde(default)]
    pub ship_linear_acceleration_fb: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipLinearAccelerationUD` (Class)
    #[serde(default)]
    pub ship_linear_acceleration_ud: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipAngularAccelerationPitch` (Class)
    #[serde(default)]
    pub ship_angular_acceleration_pitch: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipAngularAccelerationRoll` (Class)
    #[serde(default)]
    pub ship_angular_acceleration_roll: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipAngularAccelerationYaw` (Class)
    #[serde(default)]
    pub ship_angular_acceleration_yaw: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipAngularTurbulencePitch` (Class)
    #[serde(default)]
    pub ship_angular_turbulence_pitch: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipAngularTurbulenceRoll` (Class)
    #[serde(default)]
    pub ship_angular_turbulence_roll: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipAngularTurbulenceYaw` (Class)
    #[serde(default)]
    pub ship_angular_turbulence_yaw: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipLinearVelocityRL` (Class)
    #[serde(default)]
    pub ship_linear_velocity_rl: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipLinearVelocityFB` (Class)
    #[serde(default)]
    pub ship_linear_velocity_fb: Option<Handle<AudioRtpc>>,
    /// DCB field: `shipLinearVelocityUD` (Class)
    #[serde(default)]
    pub ship_linear_velocity_ud: Option<Handle<AudioRtpc>>,
    /// DCB field: `tunnelLinearAccelerationRL` (Class)
    #[serde(default)]
    pub tunnel_linear_acceleration_rl: Option<Handle<AudioRtpc>>,
    /// DCB field: `tunnelLinearAccelerationFB` (Class)
    #[serde(default)]
    pub tunnel_linear_acceleration_fb: Option<Handle<AudioRtpc>>,
    /// DCB field: `tunnelLinearAccelerationUD` (Class)
    #[serde(default)]
    pub tunnel_linear_acceleration_ud: Option<Handle<AudioRtpc>>,
    /// DCB field: `tunnelAngularAccelerationPitch` (Class)
    #[serde(default)]
    pub tunnel_angular_acceleration_pitch: Option<Handle<AudioRtpc>>,
    /// DCB field: `tunnelAngularAccelerationRoll` (Class)
    #[serde(default)]
    pub tunnel_angular_acceleration_roll: Option<Handle<AudioRtpc>>,
    /// DCB field: `tunnelAngularAccelerationYaw` (Class)
    #[serde(default)]
    pub tunnel_angular_acceleration_yaw: Option<Handle<AudioRtpc>>,
    /// DCB field: `playerInputPitch` (Class)
    #[serde(default)]
    pub player_input_pitch: Option<Handle<AudioRtpc>>,
    /// DCB field: `playerInputRoll` (Class)
    #[serde(default)]
    pub player_input_roll: Option<Handle<AudioRtpc>>,
    /// DCB field: `playerInputYaw` (Class)
    #[serde(default)]
    pub player_input_yaw: Option<Handle<AudioRtpc>>,
    /// DCB field: `playerInputStrafeRL` (Class)
    #[serde(default)]
    pub player_input_strafe_rl: Option<Handle<AudioRtpc>>,
    /// DCB field: `playerInputStrafeFB` (Class)
    #[serde(default)]
    pub player_input_strafe_fb: Option<Handle<AudioRtpc>>,
    /// DCB field: `playerInputStrafeUD` (Class)
    #[serde(default)]
    pub player_input_strafe_ud: Option<Handle<AudioRtpc>>,
    /// DCB field: `afterburnerRequestedRtpc` (Class)
    #[serde(default)]
    pub afterburner_requested_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for JumpDriveAudioMovementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_audio_movement_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_audio_movement_params }
}

impl<'a> Extract<'a> for JumpDriveAudioMovementParams {
    const TYPE_NAME: &'static str = "JumpDriveAudioMovementParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ship_linear_acceleration_rl: match inst.get("shipLinearAccelerationRL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_linear_acceleration_fb: match inst.get("shipLinearAccelerationFB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_linear_acceleration_ud: match inst.get("shipLinearAccelerationUD") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_angular_acceleration_pitch: match inst.get("shipAngularAccelerationPitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_angular_acceleration_roll: match inst.get("shipAngularAccelerationRoll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_angular_acceleration_yaw: match inst.get("shipAngularAccelerationYaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_angular_turbulence_pitch: match inst.get("shipAngularTurbulencePitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_angular_turbulence_roll: match inst.get("shipAngularTurbulenceRoll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_angular_turbulence_yaw: match inst.get("shipAngularTurbulenceYaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_linear_velocity_rl: match inst.get("shipLinearVelocityRL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_linear_velocity_fb: match inst.get("shipLinearVelocityFB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_linear_velocity_ud: match inst.get("shipLinearVelocityUD") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_linear_acceleration_rl: match inst.get("tunnelLinearAccelerationRL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_linear_acceleration_fb: match inst.get("tunnelLinearAccelerationFB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_linear_acceleration_ud: match inst.get("tunnelLinearAccelerationUD") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_angular_acceleration_pitch: match inst.get("tunnelAngularAccelerationPitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_angular_acceleration_roll: match inst.get("tunnelAngularAccelerationRoll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_angular_acceleration_yaw: match inst.get("tunnelAngularAccelerationYaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_input_pitch: match inst.get("playerInputPitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_input_roll: match inst.get("playerInputRoll") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_input_yaw: match inst.get("playerInputYaw") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_input_strafe_rl: match inst.get("playerInputStrafeRL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_input_strafe_fb: match inst.get("playerInputStrafeFB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_input_strafe_ud: match inst.get("playerInputStrafeUD") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            afterburner_requested_rtpc: match inst.get("afterburnerRequestedRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveAudioParams {
    /// DCB field: `stateMap` (Class)
    #[serde(default)]
    pub state_map: Option<Handle<JumpDriveStateAudioMap>>,
    /// DCB field: `tunnelProgressRtpc` (Class)
    #[serde(default)]
    pub tunnel_progress_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `inJumpTunnelRtpc` (Class)
    #[serde(default)]
    pub in_jump_tunnel_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `distanceFromSplineRtpc` (Class)
    #[serde(default)]
    pub distance_from_spline_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `wallImpactSpeedThreshold` (Single)
    #[serde(default)]
    pub wall_impact_speed_threshold: f32,
    /// DCB field: `wallImpactDistanceThreshold` (Single)
    #[serde(default)]
    pub wall_impact_distance_threshold: f32,
    /// DCB field: `passByLightDistanceThreshold` (Single)
    #[serde(default)]
    pub pass_by_light_distance_threshold: f32,
    /// DCB field: `passByLightDistanceNormRtpc` (Class)
    #[serde(default)]
    pub pass_by_light_distance_norm_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `passByLightDotRtpc` (Class)
    #[serde(default)]
    pub pass_by_light_dot_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `tunnelWallImpactSpeedRtpc` (Class)
    #[serde(default)]
    pub tunnel_wall_impact_speed_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `tunnelWallImpactOneShot` (Class)
    #[serde(default)]
    pub tunnel_wall_impact_one_shot: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `startTunnelWallContactLoop` (Class)
    #[serde(default)]
    pub start_tunnel_wall_contact_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `stopTunnelWallContactLoop` (Class)
    #[serde(default)]
    pub stop_tunnel_wall_contact_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `splineVelRtpc` (Class)
    #[serde(default)]
    pub spline_vel_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `distortionRtpc` (Class)
    #[serde(default)]
    pub distortion_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `startSplineCenterLoop` (Class)
    #[serde(default)]
    pub start_spline_center_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `stopSplineCenterLoop` (Class)
    #[serde(default)]
    pub stop_spline_center_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `startShipTunnelMidpointLoop` (Class)
    #[serde(default)]
    pub start_ship_tunnel_midpoint_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `stopShipTunnelMidpointLoop` (Class)
    #[serde(default)]
    pub stop_ship_tunnel_midpoint_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `movementParams` (Class)
    #[serde(default)]
    pub movement_params: Option<Handle<JumpDriveAudioMovementParams>>,
}

impl Pooled for JumpDriveAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_audio_params }
}

impl<'a> Extract<'a> for JumpDriveAudioParams {
    const TYPE_NAME: &'static str = "JumpDriveAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_map: match inst.get("stateMap") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveStateAudioMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveStateAudioMap>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_progress_rtpc: match inst.get("tunnelProgressRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            in_jump_tunnel_rtpc: match inst.get("inJumpTunnelRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance_from_spline_rtpc: match inst.get("distanceFromSplineRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wall_impact_speed_threshold: inst.get_f32("wallImpactSpeedThreshold").unwrap_or_default(),
            wall_impact_distance_threshold: inst.get_f32("wallImpactDistanceThreshold").unwrap_or_default(),
            pass_by_light_distance_threshold: inst.get_f32("passByLightDistanceThreshold").unwrap_or_default(),
            pass_by_light_distance_norm_rtpc: match inst.get("passByLightDistanceNormRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pass_by_light_dot_rtpc: match inst.get("passByLightDotRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_wall_impact_speed_rtpc: match inst.get("tunnelWallImpactSpeedRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tunnel_wall_impact_one_shot: match inst.get("tunnelWallImpactOneShot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            start_tunnel_wall_contact_loop: match inst.get("startTunnelWallContactLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_tunnel_wall_contact_loop: match inst.get("stopTunnelWallContactLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spline_vel_rtpc: match inst.get("splineVelRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distortion_rtpc: match inst.get("distortionRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            start_spline_center_loop: match inst.get("startSplineCenterLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_spline_center_loop: match inst.get("stopSplineCenterLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            start_ship_tunnel_midpoint_loop: match inst.get("startShipTunnelMidpointLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_ship_tunnel_midpoint_loop: match inst.get("stopShipTunnelMidpointLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            movement_params: match inst.get("movementParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveAudioMovementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveAudioMovementParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveMusicEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveMusicEvent {
    /// DCB field: `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// DCB field: `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for JumpDriveMusicEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_music_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_music_event }
}

impl<'a> Extract<'a> for JumpDriveMusicEvent {
    const TYPE_NAME: &'static str = "JumpDriveMusicEvent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            music_logic_event: inst.get("musicLogicEvent").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            music_wwise_event: match inst.get("musicWwiseEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveStateMusicMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveStateMusicMap {
    /// DCB field: `musicEvent` (Class)
    #[serde(default)]
    pub music_event: Option<Handle<JumpDriveMusicEvent>>,
    /// DCB field: `jumpDriveState` (EnumChoice)
    #[serde(default)]
    pub jump_drive_state: String,
}

impl Pooled for JumpDriveStateMusicMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_state_music_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_state_music_map }
}

impl<'a> Extract<'a> for JumpDriveStateMusicMap {
    const TYPE_NAME: &'static str = "JumpDriveStateMusicMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            music_event: match inst.get("musicEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveMusicEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveMusicEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jump_drive_state: inst.get_str("jumpDriveState").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpDriveMusicParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveMusicParams {
    /// DCB field: `tunnelProgressRtpc` (Class)
    #[serde(default)]
    pub tunnel_progress_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `stateMap` (Class (array))
    #[serde(default)]
    pub state_map: Vec<Handle<JumpDriveStateMusicMap>>,
    /// DCB field: `preArrivalDurationSecs` (Single)
    #[serde(default)]
    pub pre_arrival_duration_secs: f32,
    /// DCB field: `preArrivalMusicEvent` (Class)
    #[serde(default)]
    pub pre_arrival_music_event: Option<Handle<JumpDriveMusicEvent>>,
}

impl Pooled for JumpDriveMusicParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_music_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_music_params }
}

impl<'a> Extract<'a> for JumpDriveMusicParams {
    const TYPE_NAME: &'static str = "JumpDriveMusicParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tunnel_progress_rtpc: match inst.get("tunnelProgressRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            state_map: inst.get_array("stateMap")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<JumpDriveStateMusicMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<JumpDriveStateMusicMap>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            pre_arrival_duration_secs: inst.get_f32("preArrivalDurationSecs").unwrap_or_default(),
            pre_arrival_music_event: match inst.get("preArrivalMusicEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<JumpDriveMusicEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<JumpDriveMusicEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `JumpDriveApproachRingsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveApproachRingsParams {
    /// DCB field: `firstRingDistance` (Single)
    #[serde(default)]
    pub first_ring_distance: f32,
    /// DCB field: `lastRingDistance` (Single)
    #[serde(default)]
    pub last_ring_distance: f32,
    /// DCB field: `largestRingDiameter` (Single)
    #[serde(default)]
    pub largest_ring_diameter: f32,
}

impl Pooled for JumpDriveApproachRingsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_drive_approach_rings_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_drive_approach_rings_params }
}

impl<'a> Extract<'a> for JumpDriveApproachRingsParams {
    const TYPE_NAME: &'static str = "JumpDriveApproachRingsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            first_ring_distance: inst.get_f32("firstRingDistance").unwrap_or_default(),
            last_ring_distance: inst.get_f32("lastRingDistance").unwrap_or_default(),
            largest_ring_diameter: inst.get_f32("largestRingDiameter").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpTravelCameraParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTravelCameraParams {
    /// DCB field: `genericModifiers` (Class)
    #[serde(default)]
    pub generic_modifiers: Option<Handle<CameraEffectsModifiers>>,
    /// DCB field: `cameraEffectParams` (Class)
    #[serde(default)]
    pub camera_effect_params: Option<Handle<GlobalJumpTunnelCameraEffectParams>>,
}

impl Pooled for JumpTravelCameraParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_travel_camera_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_travel_camera_params }
}

impl<'a> Extract<'a> for JumpTravelCameraParams {
    const TYPE_NAME: &'static str = "JumpTravelCameraParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            generic_modifiers: match inst.get("genericModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CameraEffectsModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CameraEffectsModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            camera_effect_params: match inst.get("cameraEffectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalJumpTunnelCameraEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalJumpTunnelCameraEffectParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Jurisdiction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jurisdiction {
    /// DCB field: `subsumptionJurisdiction` (String)
    #[serde(default)]
    pub subsumption_jurisdiction: String,
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `logoPath` (String)
    #[serde(default)]
    pub logo_path: String,
    /// DCB field: `parentJurisdiction` (Reference)
    #[serde(default)]
    pub parent_jurisdiction: Option<CigGuid>,
    /// DCB field: `respectsParentJurisdictionLaws` (Boolean)
    #[serde(default)]
    pub respects_parent_jurisdiction_laws: bool,
    /// DCB field: `infractions` (Class (array))
    #[serde(default)]
    pub infractions: Vec<Handle<Infraction>>,
    /// DCB field: `infractionSets` (Reference (array))
    #[serde(default)]
    pub infraction_sets: Vec<CigGuid>,
    /// DCB field: `journalEntry` (Reference)
    #[serde(default)]
    pub journal_entry: Option<CigGuid>,
    /// DCB field: `maxStolenGoodsPossessionSCU` (Single)
    #[serde(default)]
    pub max_stolen_goods_possession_scu: f32,
    /// DCB field: `prohibitedGoods` (Reference (array))
    #[serde(default)]
    pub prohibited_goods: Vec<CigGuid>,
    /// DCB field: `prohibitedResources` (Reference (array))
    #[serde(default)]
    pub prohibited_resources: Vec<CigGuid>,
    /// DCB field: `controlledSubstanceClasses` (Class)
    #[serde(default)]
    pub controlled_substance_classes: Option<Handle<ControlledSubstanceClass>>,
    /// DCB field: `baseFine` (Int32)
    #[serde(default)]
    pub base_fine: i32,
    /// DCB field: `earlyPaymentPeriod` (Single)
    #[serde(default)]
    pub early_payment_period: f32,
    /// DCB field: `isPrison` (Boolean)
    #[serde(default)]
    pub is_prison: bool,
    /// DCB field: `impoundingDefinitions` (Class (array))
    #[serde(default)]
    pub impounding_definitions: Vec<Handle<ImpoundingDefinition>>,
}

impl Pooled for Jurisdiction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jurisdiction }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jurisdiction }
}

impl<'a> Extract<'a> for Jurisdiction {
    const TYPE_NAME: &'static str = "Jurisdiction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            subsumption_jurisdiction: inst.get_str("subsumptionJurisdiction").map(String::from).unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            logo_path: inst.get_str("logoPath").map(String::from).unwrap_or_default(),
            parent_jurisdiction: inst.get("parentJurisdiction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            respects_parent_jurisdiction_laws: inst.get_bool("respectsParentJurisdictionLaws").unwrap_or_default(),
            infractions: inst.get_array("infractions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Infraction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Infraction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            infraction_sets: inst.get_array("infractionSets")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            journal_entry: inst.get("journalEntry").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            max_stolen_goods_possession_scu: inst.get_f32("maxStolenGoodsPossessionSCU").unwrap_or_default(),
            prohibited_goods: inst.get_array("prohibitedGoods")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            prohibited_resources: inst.get_array("prohibitedResources")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            controlled_substance_classes: match inst.get("controlledSubstanceClasses") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlledSubstanceClass>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlledSubstanceClass>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            base_fine: inst.get_i32("baseFine").unwrap_or_default(),
            early_payment_period: inst.get_f32("earlyPaymentPeriod").unwrap_or_default(),
            is_prison: inst.get_bool("isPrison").unwrap_or_default(),
            impounding_definitions: inst.get_array("impoundingDefinitions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ImpoundingDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ImpoundingDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpFallLandParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpFallLandParams {
    /// DCB field: `useJump` (Boolean)
    #[serde(default)]
    pub use_jump: bool,
    /// DCB field: `jumpHeight` (Single)
    #[serde(default)]
    pub jump_height: f32,
    /// DCB field: `jumpDistance` (Single)
    #[serde(default)]
    pub jump_distance: f32,
}

impl Pooled for JumpFallLandParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_fall_land_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_fall_land_params }
}

impl<'a> Extract<'a> for JumpFallLandParams {
    const TYPE_NAME: &'static str = "JumpFallLandParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            use_jump: inst.get_bool("useJump").unwrap_or_default(),
            jump_height: inst.get_f32("jumpHeight").unwrap_or_default(),
            jump_distance: inst.get_f32("jumpDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpThrusterPackConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpThrusterPackConfig {
    /// DCB field: `useJumpThrusters` (Boolean)
    #[serde(default)]
    pub use_jump_thrusters: bool,
    /// DCB field: `jumpBoosterFuel` (Single)
    #[serde(default)]
    pub jump_booster_fuel: f32,
    /// DCB field: `jumpBoosterFuelConsumptionSpeed` (Single)
    #[serde(default)]
    pub jump_booster_fuel_consumption_speed: f32,
    /// DCB field: `jumpBoosterFuelAirRestoreSpeed` (Single)
    #[serde(default)]
    pub jump_booster_fuel_air_restore_speed: f32,
    /// DCB field: `jumpBoosterFuelGroundRestoreSpeed` (Single)
    #[serde(default)]
    pub jump_booster_fuel_ground_restore_speed: f32,
    /// DCB field: `jumpThrustersImpulseStrenght` (Single)
    #[serde(default)]
    pub jump_thrusters_impulse_strenght: f32,
    /// DCB field: `useLandThrusters` (Boolean)
    #[serde(default)]
    pub use_land_thrusters: bool,
    /// DCB field: `landThrustersImpulseStrenght` (Single)
    #[serde(default)]
    pub land_thrusters_impulse_strenght: f32,
    /// DCB field: `landThrustersMinThresholdDistance` (Single)
    #[serde(default)]
    pub land_thrusters_min_threshold_distance: f32,
    /// DCB field: `landThrustersMaxThresholdDistance` (Single)
    #[serde(default)]
    pub land_thrusters_max_threshold_distance: f32,
    /// DCB field: `landThrustersMinSpeed` (Single)
    #[serde(default)]
    pub land_thrusters_min_speed: f32,
    /// DCB field: `landThrustersMinTime` (Single)
    #[serde(default)]
    pub land_thrusters_min_time: f32,
}

impl Pooled for JumpThrusterPackConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ju.jump_thruster_pack_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ju.jump_thruster_pack_config }
}

impl<'a> Extract<'a> for JumpThrusterPackConfig {
    const TYPE_NAME: &'static str = "JumpThrusterPackConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            use_jump_thrusters: inst.get_bool("useJumpThrusters").unwrap_or_default(),
            jump_booster_fuel: inst.get_f32("jumpBoosterFuel").unwrap_or_default(),
            jump_booster_fuel_consumption_speed: inst.get_f32("jumpBoosterFuelConsumptionSpeed").unwrap_or_default(),
            jump_booster_fuel_air_restore_speed: inst.get_f32("jumpBoosterFuelAirRestoreSpeed").unwrap_or_default(),
            jump_booster_fuel_ground_restore_speed: inst.get_f32("jumpBoosterFuelGroundRestoreSpeed").unwrap_or_default(),
            jump_thrusters_impulse_strenght: inst.get_f32("jumpThrustersImpulseStrenght").unwrap_or_default(),
            use_land_thrusters: inst.get_bool("useLandThrusters").unwrap_or_default(),
            land_thrusters_impulse_strenght: inst.get_f32("landThrustersImpulseStrenght").unwrap_or_default(),
            land_thrusters_min_threshold_distance: inst.get_f32("landThrustersMinThresholdDistance").unwrap_or_default(),
            land_thrusters_max_threshold_distance: inst.get_f32("landThrustersMaxThresholdDistance").unwrap_or_default(),
            land_thrusters_min_speed: inst.get_f32("landThrustersMinSpeed").unwrap_or_default(),
            land_thrusters_min_time: inst.get_f32("landThrustersMinTime").unwrap_or_default(),
        }
    }
}

