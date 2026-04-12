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

/// DCB type: `FlashObjectBindingGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashObjectBindingGroup {
    /// DCB field: `variableObjects` (Class (array))
    #[serde(default)]
    pub variable_objects: Vec<Handle<FlashVariableObject>>,
}

impl Pooled for FlashObjectBindingGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fl.flash_object_binding_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fl.flash_object_binding_group }
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
    /// DCB field: `linkName` (String)
    #[serde(default)]
    pub link_name: String,
    /// DCB field: `setAlpha` (Boolean)
    #[serde(default)]
    pub set_alpha: bool,
    /// DCB field: `alpha` (Single)
    #[serde(default)]
    pub alpha: f32,
    /// DCB field: `setCurrentFrame` (Boolean)
    #[serde(default)]
    pub set_current_frame: bool,
    /// DCB field: `currentFrame` (UInt32)
    #[serde(default)]
    pub current_frame: u32,
    /// DCB field: `modeX` (EnumChoice)
    #[serde(default)]
    pub mode_x: String,
    /// DCB field: `modeY` (EnumChoice)
    #[serde(default)]
    pub mode_y: String,
    /// DCB field: `modeZ` (EnumChoice)
    #[serde(default)]
    pub mode_z: String,
    /// DCB field: `x` (Single)
    #[serde(default)]
    pub x: f32,
    /// DCB field: `y` (Single)
    #[serde(default)]
    pub y: f32,
    /// DCB field: `z` (Single)
    #[serde(default)]
    pub z: f32,
    /// DCB field: `attachX` (Single)
    #[serde(default)]
    pub attach_x: f32,
    /// DCB field: `attachY` (Single)
    #[serde(default)]
    pub attach_y: f32,
    /// DCB field: `modeRotationX` (EnumChoice)
    #[serde(default)]
    pub mode_rotation_x: String,
    /// DCB field: `modeRotationY` (EnumChoice)
    #[serde(default)]
    pub mode_rotation_y: String,
    /// DCB field: `modeRotationZ` (EnumChoice)
    #[serde(default)]
    pub mode_rotation_z: String,
    /// DCB field: `rotationX` (Single)
    #[serde(default)]
    pub rotation_x: f32,
    /// DCB field: `rotationY` (Single)
    #[serde(default)]
    pub rotation_y: f32,
    /// DCB field: `rotationZ` (Single)
    #[serde(default)]
    pub rotation_z: f32,
    /// DCB field: `setScaleX` (Boolean)
    #[serde(default)]
    pub set_scale_x: bool,
    /// DCB field: `setScaleY` (Boolean)
    #[serde(default)]
    pub set_scale_y: bool,
    /// DCB field: `setScaleZ` (Boolean)
    #[serde(default)]
    pub set_scale_z: bool,
    /// DCB field: `scaleX` (Single)
    #[serde(default)]
    pub scale_x: f32,
    /// DCB field: `scaleY` (Single)
    #[serde(default)]
    pub scale_y: f32,
    /// DCB field: `scaleZ` (Single)
    #[serde(default)]
    pub scale_z: f32,
    /// DCB field: `setVisible` (Boolean)
    #[serde(default)]
    pub set_visible: bool,
    /// DCB field: `visible` (Boolean)
    #[serde(default)]
    pub visible: bool,
    /// DCB field: `setPerspFOV` (Boolean)
    #[serde(default)]
    pub set_persp_fov: bool,
    /// DCB field: `perspFOV` (Single)
    #[serde(default)]
    pub persp_fov: f32,
    /// DCB field: `fix2dCoordinates` (Boolean)
    #[serde(default)]
    pub fix2d_coordinates: bool,
}

impl Pooled for FlashVariableObject {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fl.flash_variable_object }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fl.flash_variable_object }
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

/// DCB type: `FloatFactorRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatFactorRange {
    /// DCB field: `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// DCB field: `max` (Single)
    #[serde(default)]
    pub max: f32,
}

impl Pooled for FloatFactorRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fl.float_factor_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fl.float_factor_range }
}

impl<'a> Extract<'a> for FloatFactorRange {
    const TYPE_NAME: &'static str = "FloatFactorRange";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min: inst.get_f32("min").unwrap_or_default(),
            max: inst.get_f32("max").unwrap_or_default(),
        }
    }
}

/// DCB type: `Flash_Palette`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flash_Palette {
    /// DCB field: `Entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<Flash_PaletteEntry>>,
}

impl Pooled for Flash_Palette {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fl.flash_palette }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fl.flash_palette }
}

impl<'a> Extract<'a> for Flash_Palette {
    const TYPE_NAME: &'static str = "Flash_Palette";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: inst.get_array("Entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Flash_PaletteEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Flash_PaletteEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Flash_PaletteEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flash_PaletteEntry {
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `FlashColor` (StrongPointer)
    #[serde(default)]
    pub flash_color: Option<Handle<SRGBA8>>,
}

impl Pooled for Flash_PaletteEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fl.flash_palette_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fl.flash_palette_entry }
}

impl<'a> Extract<'a> for Flash_PaletteEntry {
    const TYPE_NAME: &'static str = "Flash_PaletteEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            flash_color: match inst.get("FlashColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FlightHUDUIMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightHUDUIMessage {
    /// DCB field: `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// DCB field: `priority` (Byte)
    #[serde(default)]
    pub priority: u32,
}

impl Pooled for FlightHUDUIMessage {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fl.flight_huduimessage }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fl.flight_huduimessage }
}

impl<'a> Extract<'a> for FlightHUDUIMessage {
    const TYPE_NAME: &'static str = "FlightHUDUIMessage";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
            priority: inst.get_u32("priority").unwrap_or_default(),
        }
    }
}

/// DCB type: `FlightHUDUIView_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightHUDUIView_Config {
    /// DCB field: `messages` (Class)
    #[serde(default)]
    pub messages: Option<Handle<FlightHUDUIMessage>>,
}

impl Pooled for FlightHUDUIView_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fl.flight_huduiview_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fl.flight_huduiview_config }
}

impl<'a> Extract<'a> for FlightHUDUIView_Config {
    const TYPE_NAME: &'static str = "FlightHUDUIView_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            messages: match inst.get("messages") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FlightHUDUIMessage>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FlightHUDUIMessage>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

