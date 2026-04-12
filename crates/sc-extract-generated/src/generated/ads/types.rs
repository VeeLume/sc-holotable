// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ads`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SeatAdsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatAdsDef {
    /// `zoomSpeed` (Single)
    #[serde(default)]
    pub zoom_speed: f32,
    /// `minFovWithTarget` (Single)
    #[serde(default)]
    pub min_fov_with_target: f32,
    /// `minFovWithoutTarget` (Single)
    #[serde(default)]
    pub min_fov_without_target: f32,
    /// `minFovStableZoom` (Single)
    #[serde(default)]
    pub min_fov_stable_zoom: f32,
    /// `crosshairAngle` (Single)
    #[serde(default)]
    pub crosshair_angle: f32,
    /// `maxPitch` (Single)
    #[serde(default)]
    pub max_pitch: f32,
    /// `maxYaw` (Single)
    #[serde(default)]
    pub max_yaw: f32,
    /// `timeToKeepZoomAfterTargetLoss` (Single)
    #[serde(default)]
    pub time_to_keep_zoom_after_target_loss: f32,
    /// `timeToKeepZoomAfterHoverTargetLoss` (Single)
    #[serde(default)]
    pub time_to_keep_zoom_after_hover_target_loss: f32,
    /// `timeToDisallowZoomAfterHoverTargetGain` (Single)
    #[serde(default)]
    pub time_to_disallow_zoom_after_hover_target_gain: f32,
    /// `trackSubtargets` (Boolean)
    #[serde(default)]
    pub track_subtargets: bool,
    /// `useScalingOutsideAds` (Boolean)
    #[serde(default)]
    pub use_scaling_outside_ads: bool,
    /// `allowedForOperatorMode` (Boolean)
    #[serde(default)]
    pub allowed_for_operator_mode: bool,
    /// `recoilFovCurve` (Class)
    #[serde(default)]
    pub recoil_fov_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SeatAdsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ads.seat_ads_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ads.seat_ads_def }
}

impl<'a> Extract<'a> for SeatAdsDef {
    const TYPE_NAME: &'static str = "SeatAdsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            zoom_speed: inst.get_f32("zoomSpeed").unwrap_or_default(),
            min_fov_with_target: inst.get_f32("minFovWithTarget").unwrap_or_default(),
            min_fov_without_target: inst.get_f32("minFovWithoutTarget").unwrap_or_default(),
            min_fov_stable_zoom: inst.get_f32("minFovStableZoom").unwrap_or_default(),
            crosshair_angle: inst.get_f32("crosshairAngle").unwrap_or_default(),
            max_pitch: inst.get_f32("maxPitch").unwrap_or_default(),
            max_yaw: inst.get_f32("maxYaw").unwrap_or_default(),
            time_to_keep_zoom_after_target_loss: inst.get_f32("timeToKeepZoomAfterTargetLoss").unwrap_or_default(),
            time_to_keep_zoom_after_hover_target_loss: inst.get_f32("timeToKeepZoomAfterHoverTargetLoss").unwrap_or_default(),
            time_to_disallow_zoom_after_hover_target_gain: inst.get_f32("timeToDisallowZoomAfterHoverTargetGain").unwrap_or_default(),
            track_subtargets: inst.get_bool("trackSubtargets").unwrap_or_default(),
            use_scaling_outside_ads: inst.get_bool("useScalingOutsideAds").unwrap_or_default(),
            allowed_for_operator_mode: inst.get_bool("allowedForOperatorMode").unwrap_or_default(),
            recoil_fov_curve: match inst.get("recoilFovCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

