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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `FlashObjectBindingGroup`
pub struct FlashObjectBindingGroup {
    /// `variableObjects` (Class (array))
    pub variable_objects: Vec<Handle<FlashVariableObject>>,
}

impl Pooled for FlashObjectBindingGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_flashobjectbindinggroups.flash_object_binding_group
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_flashobjectbindinggroups.flash_object_binding_group
    }
}

impl<'a> Extract<'a> for FlashObjectBindingGroup {
    const TYPE_NAME: &'static str = "FlashObjectBindingGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variable_objects: inst
                .get_array("variableObjects")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<FlashVariableObject>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<FlashVariableObject>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FlashVariableObject`
pub struct FlashVariableObject {
    /// `linkName` (String)
    pub link_name: String,
    /// `setAlpha` (Boolean)
    pub set_alpha: bool,
    /// `alpha` (Single)
    pub alpha: f32,
    /// `setCurrentFrame` (Boolean)
    pub set_current_frame: bool,
    /// `currentFrame` (UInt32)
    pub current_frame: u32,
    /// `modeX` (EnumChoice)
    pub mode_x: FlashValueUpdateMode,
    /// `modeY` (EnumChoice)
    pub mode_y: FlashValueUpdateMode,
    /// `modeZ` (EnumChoice)
    pub mode_z: FlashValueUpdateMode,
    /// `x` (Single)
    pub x: f32,
    /// `y` (Single)
    pub y: f32,
    /// `z` (Single)
    pub z: f32,
    /// `attachX` (Single)
    pub attach_x: f32,
    /// `attachY` (Single)
    pub attach_y: f32,
    /// `modeRotationX` (EnumChoice)
    pub mode_rotation_x: FlashValueUpdateMode,
    /// `modeRotationY` (EnumChoice)
    pub mode_rotation_y: FlashValueUpdateMode,
    /// `modeRotationZ` (EnumChoice)
    pub mode_rotation_z: FlashValueUpdateMode,
    /// `rotationX` (Single)
    pub rotation_x: f32,
    /// `rotationY` (Single)
    pub rotation_y: f32,
    /// `rotationZ` (Single)
    pub rotation_z: f32,
    /// `setScaleX` (Boolean)
    pub set_scale_x: bool,
    /// `setScaleY` (Boolean)
    pub set_scale_y: bool,
    /// `setScaleZ` (Boolean)
    pub set_scale_z: bool,
    /// `scaleX` (Single)
    pub scale_x: f32,
    /// `scaleY` (Single)
    pub scale_y: f32,
    /// `scaleZ` (Single)
    pub scale_z: f32,
    /// `setVisible` (Boolean)
    pub set_visible: bool,
    /// `visible` (Boolean)
    pub visible: bool,
    /// `setPerspFOV` (Boolean)
    pub set_persp_fov: bool,
    /// `perspFOV` (Single)
    pub persp_fov: f32,
    /// `fix2dCoordinates` (Boolean)
    pub fix2d_coordinates: bool,
}

impl Pooled for FlashVariableObject {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_flashobjectbindinggroups.flash_variable_object
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_flashobjectbindinggroups.flash_variable_object
    }
}

impl<'a> Extract<'a> for FlashVariableObject {
    const TYPE_NAME: &'static str = "FlashVariableObject";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            link_name: inst
                .get_str("linkName")
                .map(String::from)
                .unwrap_or_default(),
            set_alpha: inst.get_bool("setAlpha").unwrap_or_default(),
            alpha: inst.get_f32("alpha").unwrap_or_default(),
            set_current_frame: inst.get_bool("setCurrentFrame").unwrap_or_default(),
            current_frame: inst.get_u32("currentFrame").unwrap_or_default(),
            mode_x: FlashValueUpdateMode::from_dcb_str(inst.get_str("modeX").unwrap_or("")),
            mode_y: FlashValueUpdateMode::from_dcb_str(inst.get_str("modeY").unwrap_or("")),
            mode_z: FlashValueUpdateMode::from_dcb_str(inst.get_str("modeZ").unwrap_or("")),
            x: inst.get_f32("x").unwrap_or_default(),
            y: inst.get_f32("y").unwrap_or_default(),
            z: inst.get_f32("z").unwrap_or_default(),
            attach_x: inst.get_f32("attachX").unwrap_or_default(),
            attach_y: inst.get_f32("attachY").unwrap_or_default(),
            mode_rotation_x: FlashValueUpdateMode::from_dcb_str(
                inst.get_str("modeRotationX").unwrap_or(""),
            ),
            mode_rotation_y: FlashValueUpdateMode::from_dcb_str(
                inst.get_str("modeRotationY").unwrap_or(""),
            ),
            mode_rotation_z: FlashValueUpdateMode::from_dcb_str(
                inst.get_str("modeRotationZ").unwrap_or(""),
            ),
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
