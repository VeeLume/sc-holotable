// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-targettrackingautozoom`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `TargetTrackingAutoZoomDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetTrackingAutoZoomDef {
    /// `zoomByDistance` (Class)
    #[serde(default)]
    pub zoom_by_distance: Option<Handle<BezierCurve>>,
    /// `referenceDistance` (Single)
    #[serde(default)]
    pub reference_distance: f32,
    /// `zoomAngleMin` (Single)
    #[serde(default)]
    pub zoom_angle_min: f32,
    /// `zoomAngleMax` (Single)
    #[serde(default)]
    pub zoom_angle_max: f32,
    /// `zoomLerpSpeedIn` (Single)
    #[serde(default)]
    pub zoom_lerp_speed_in: f32,
    /// `zoomLerpSpeedOut` (Single)
    #[serde(default)]
    pub zoom_lerp_speed_out: f32,
}

impl Pooled for TargetTrackingAutoZoomDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_targettrackingautozoom.target_tracking_auto_zoom_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_targettrackingautozoom.target_tracking_auto_zoom_def }
}

impl<'a> Extract<'a> for TargetTrackingAutoZoomDef {
    const TYPE_NAME: &'static str = "TargetTrackingAutoZoomDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            zoom_by_distance: match inst.get("zoomByDistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reference_distance: inst.get_f32("referenceDistance").unwrap_or_default(),
            zoom_angle_min: inst.get_f32("zoomAngleMin").unwrap_or_default(),
            zoom_angle_max: inst.get_f32("zoomAngleMax").unwrap_or_default(),
            zoom_lerp_speed_in: inst.get_f32("zoomLerpSpeedIn").unwrap_or_default(),
            zoom_lerp_speed_out: inst.get_f32("zoomLerpSpeedOut").unwrap_or_default(),
        }
    }
}

