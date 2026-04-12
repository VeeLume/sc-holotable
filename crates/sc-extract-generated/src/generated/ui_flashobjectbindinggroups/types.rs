// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-flashobjectbindinggroups`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `FlashObjectBindingGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashObjectBindingGroup {
    /// `variableObjects` (Class (array))
    #[serde(default)]
    pub variable_objects: Vec<Handle<FlashVariableObject>>,
}

impl Pooled for FlashObjectBindingGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_flashobjectbindinggroups.flash_object_binding_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_flashobjectbindinggroups.flash_object_binding_group }
}

impl<'a> Extract<'a> for FlashObjectBindingGroup {
    const TYPE_NAME: &'static str = "FlashObjectBindingGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variable_objects: inst.get_array("variableObjects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<FlashVariableObject>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<FlashVariableObject>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FlashVariableObject`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashVariableObject {
    /// `linkName` (String)
    #[serde(default)]
    pub link_name: String,
    /// `setAlpha` (Boolean)
    #[serde(default)]
    pub set_alpha: bool,
    /// `alpha` (Single)
    #[serde(default)]
    pub alpha: f32,
    /// `setCurrentFrame` (Boolean)
    #[serde(default)]
    pub set_current_frame: bool,
    /// `currentFrame` (UInt32)
    #[serde(default)]
    pub current_frame: u32,
    /// `modeX` (EnumChoice)
    #[serde(default)]
    pub mode_x: String,
    /// `modeY` (EnumChoice)
    #[serde(default)]
    pub mode_y: String,
    /// `modeZ` (EnumChoice)
    #[serde(default)]
    pub mode_z: String,
    /// `x` (Single)
    #[serde(default)]
    pub x: f32,
    /// `y` (Single)
    #[serde(default)]
    pub y: f32,
    /// `z` (Single)
    #[serde(default)]
    pub z: f32,
    /// `attachX` (Single)
    #[serde(default)]
    pub attach_x: f32,
    /// `attachY` (Single)
    #[serde(default)]
    pub attach_y: f32,
    /// `modeRotationX` (EnumChoice)
    #[serde(default)]
    pub mode_rotation_x: String,
    /// `modeRotationY` (EnumChoice)
    #[serde(default)]
    pub mode_rotation_y: String,
    /// `modeRotationZ` (EnumChoice)
    #[serde(default)]
    pub mode_rotation_z: String,
    /// `rotationX` (Single)
    #[serde(default)]
    pub rotation_x: f32,
    /// `rotationY` (Single)
    #[serde(default)]
    pub rotation_y: f32,
    /// `rotationZ` (Single)
    #[serde(default)]
    pub rotation_z: f32,
    /// `setScaleX` (Boolean)
    #[serde(default)]
    pub set_scale_x: bool,
    /// `setScaleY` (Boolean)
    #[serde(default)]
    pub set_scale_y: bool,
    /// `setScaleZ` (Boolean)
    #[serde(default)]
    pub set_scale_z: bool,
    /// `scaleX` (Single)
    #[serde(default)]
    pub scale_x: f32,
    /// `scaleY` (Single)
    #[serde(default)]
    pub scale_y: f32,
    /// `scaleZ` (Single)
    #[serde(default)]
    pub scale_z: f32,
    /// `setVisible` (Boolean)
    #[serde(default)]
    pub set_visible: bool,
    /// `visible` (Boolean)
    #[serde(default)]
    pub visible: bool,
    /// `setPerspFOV` (Boolean)
    #[serde(default)]
    pub set_persp_fov: bool,
    /// `perspFOV` (Single)
    #[serde(default)]
    pub persp_fov: f32,
    /// `fix2dCoordinates` (Boolean)
    #[serde(default)]
    pub fix2d_coordinates: bool,
}

impl Pooled for FlashVariableObject {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_flashobjectbindinggroups.flash_variable_object }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_flashobjectbindinggroups.flash_variable_object }
}

impl<'a> Extract<'a> for FlashVariableObject {
    const TYPE_NAME: &'static str = "FlashVariableObject";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            link_name: inst.get_str("linkName").map(String::from).unwrap_or_default(),
            set_alpha: inst.get_bool("setAlpha").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            set_current_frame: inst.get_bool("setCurrentFrame").unwrap_or_default(),
            current_frame: inst.get_u32("currentFrame").unwrap_or_default(),
            mode_x: inst.get_str("modeX").map(String::from).unwrap_or_default(),
            mode_y: inst.get_str("modeY").map(String::from).unwrap_or_default(),
            mode_z: inst.get_str("modeZ").map(String::from).unwrap_or_default(),
            x: inst.get_f32("x").unwrap_or_default(),
            y: inst.get_f32("y").unwrap_or_default(),
            z: inst.get_f32("z").unwrap_or_default(),
            attach_x: inst.get_f32("attachX").unwrap_or_default(),
            attach_y: inst.get_f32("attachY").unwrap_or_default(),
            mode_rotation_x: inst.get_str("modeRotationX").map(String::from).unwrap_or_default(),
            mode_rotation_y: inst.get_str("modeRotationY").map(String::from).unwrap_or_default(),
            mode_rotation_z: inst.get_str("modeRotationZ").map(String::from).unwrap_or_default(),
            rotation_x: inst.get_f32("rotationX").unwrap_or_default(),
            rotation_y: inst.get_f32("rotationY").unwrap_or_default(),
            rotation_z: inst.get_f32("rotationZ").unwrap_or_default(),
            set_scale_x: inst.get_bool("setScaleX").unwrap_or_default(),
            set_scale_y: inst.get_bool("setScaleY").unwrap_or_default(),
            set_scale_z: inst.get_bool("setScaleZ").unwrap_or_default(),
            scale_x: inst.get_f32("scaleX").unwrap_or_default(),
            scale_y: inst.get_f32("scaleY").unwrap_or_default(),
            scale_z: inst.get_f32("scaleZ").unwrap_or_default(),
            set_visible: inst.get_bool("setVisible").unwrap_or_default(),
            visible: inst.get_bool("visible").unwrap_or_default(),
            set_persp_fov: inst.get_bool("setPerspFOV").unwrap_or_default(),
            persp_fov: inst.get_f32("perspFOV").unwrap_or_default(),
            fix2d_coordinates: inst.get_bool("fix2dCoordinates").unwrap_or_default(),
        }
    }
}

