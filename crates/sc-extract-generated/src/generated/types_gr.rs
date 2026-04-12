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

/// DCB type: `GrabCameraControlParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrabCameraControlParams {
    /// DCB field: `responsiveness` (Single)
    #[serde(default)]
    pub responsiveness: f32,
    /// DCB field: `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// DCB field: `rotationSlowdown` (Single)
    #[serde(default)]
    pub rotation_slowdown: f32,
    /// DCB field: `zoomSpeed` (Single)
    #[serde(default)]
    pub zoom_speed: f32,
    /// DCB field: `zoomSlowdown` (Single)
    #[serde(default)]
    pub zoom_slowdown: f32,
    /// DCB field: `minimumZoomDistance` (Single)
    #[serde(default)]
    pub minimum_zoom_distance: f32,
    /// DCB field: `maximumZoomDistance` (Single)
    #[serde(default)]
    pub maximum_zoom_distance: f32,
    /// DCB field: `maximumZoomSpeed` (Single)
    #[serde(default)]
    pub maximum_zoom_speed: f32,
    /// DCB field: `isGrabbableOutOfBounds` (Boolean)
    #[serde(default)]
    pub is_grabbable_out_of_bounds: bool,
    /// DCB field: `grabRotationMode` (EnumChoice)
    #[serde(default)]
    pub grab_rotation_mode: String,
    /// DCB field: `panResponsiveness` (Single)
    #[serde(default)]
    pub pan_responsiveness: f32,
    /// DCB field: `panSpeed` (Single)
    #[serde(default)]
    pub pan_speed: f32,
    /// DCB field: `panSlowdown` (Single)
    #[serde(default)]
    pub pan_slowdown: f32,
}

impl Pooled for GrabCameraControlParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gr.grab_camera_control_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gr.grab_camera_control_params }
}

impl<'a> Extract<'a> for GrabCameraControlParams {
    const TYPE_NAME: &'static str = "GrabCameraControlParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            responsiveness: inst.get_f32("responsiveness").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            rotation_slowdown: inst.get_f32("rotationSlowdown").unwrap_or_default(),
            zoom_speed: inst.get_f32("zoomSpeed").unwrap_or_default(),
            zoom_slowdown: inst.get_f32("zoomSlowdown").unwrap_or_default(),
            minimum_zoom_distance: inst.get_f32("minimumZoomDistance").unwrap_or_default(),
            maximum_zoom_distance: inst.get_f32("maximumZoomDistance").unwrap_or_default(),
            maximum_zoom_speed: inst.get_f32("maximumZoomSpeed").unwrap_or_default(),
            is_grabbable_out_of_bounds: inst.get_bool("isGrabbableOutOfBounds").unwrap_or_default(),
            grab_rotation_mode: inst.get_str("grabRotationMode").map(String::from).unwrap_or_default(),
            pan_responsiveness: inst.get_f32("panResponsiveness").unwrap_or_default(),
            pan_speed: inst.get_f32("panSpeed").unwrap_or_default(),
            pan_slowdown: inst.get_f32("panSlowdown").unwrap_or_default(),
        }
    }
}

/// DCB type: `Grip`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grip {
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
    /// DCB field: `grip` (Class)
    #[serde(default)]
    pub grip: Option<Handle<SGrip>>,
}

impl Pooled for Grip {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gr.grip }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gr.grip }
}

impl<'a> Extract<'a> for Grip {
    const TYPE_NAME: &'static str = "Grip";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            grip: match inst.get("grip") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGrip>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGrip>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

