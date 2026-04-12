// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `hardwaremouse`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `VirtualCursorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualCursorParams {
    /// `accelerationEnabled` (Boolean)
    #[serde(default)]
    pub acceleration_enabled: bool,
    /// `accelerationMax` (Single)
    #[serde(default)]
    pub acceleration_max: f32,
    /// `accelerationRate` (Single)
    #[serde(default)]
    pub acceleration_rate: f32,
    /// `accelerationDeadZone` (Single)
    #[serde(default)]
    pub acceleration_dead_zone: f32,
    /// `sensitivity` (Single)
    #[serde(default)]
    pub sensitivity: f32,
    /// `sensitivityPower` (Single)
    #[serde(default)]
    pub sensitivity_power: f32,
    /// `deadZone` (Single)
    #[serde(default)]
    pub dead_zone: f32,
    /// `consoleCursorEnable` (Boolean)
    #[serde(default)]
    pub console_cursor_enable: bool,
    /// `consoleCursorScale` (Single)
    #[serde(default)]
    pub console_cursor_scale: f32,
}

impl Pooled for VirtualCursorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hardwaremouse.virtual_cursor_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hardwaremouse.virtual_cursor_params }
}

impl<'a> Extract<'a> for VirtualCursorParams {
    const TYPE_NAME: &'static str = "VirtualCursorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            acceleration_enabled: inst.get_bool("accelerationEnabled").unwrap_or_default(),
            acceleration_max: inst.get_f32("accelerationMax").unwrap_or_default(),
            acceleration_rate: inst.get_f32("accelerationRate").unwrap_or_default(),
            acceleration_dead_zone: inst.get_f32("accelerationDeadZone").unwrap_or_default(),
            sensitivity: inst.get_f32("sensitivity").unwrap_or_default(),
            sensitivity_power: inst.get_f32("sensitivityPower").unwrap_or_default(),
            dead_zone: inst.get_f32("deadZone").unwrap_or_default(),
            console_cursor_enable: inst.get_bool("consoleCursorEnable").unwrap_or_default(),
            console_cursor_scale: inst.get_f32("consoleCursorScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `VirtualCursorHoverFrictionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualCursorHoverFrictionParams {
    /// `maxFriction` (Single)
    #[serde(default)]
    pub max_friction: f32,
    /// `elementSizeTreshold` (Single)
    #[serde(default)]
    pub element_size_treshold: f32,
}

impl Pooled for VirtualCursorHoverFrictionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hardwaremouse.virtual_cursor_hover_friction_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hardwaremouse.virtual_cursor_hover_friction_params }
}

impl<'a> Extract<'a> for VirtualCursorHoverFrictionParams {
    const TYPE_NAME: &'static str = "VirtualCursorHoverFrictionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_friction: inst.get_f32("maxFriction").unwrap_or_default(),
            element_size_treshold: inst.get_f32("elementSizeTreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `VirtualCursorWheelParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualCursorWheelParams {
    /// `speed` (Single)
    #[serde(default)]
    pub speed: f32,
    /// `timeMax` (Single)
    #[serde(default)]
    pub time_max: f32,
    /// `timeMultiplier` (Single)
    #[serde(default)]
    pub time_multiplier: f32,
    /// `linearTime` (Single)
    #[serde(default)]
    pub linear_time: f32,
    /// `secondStepDelay` (Single)
    #[serde(default)]
    pub second_step_delay: f32,
    /// `deadZone` (Single)
    #[serde(default)]
    pub dead_zone: f32,
}

impl Pooled for VirtualCursorWheelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hardwaremouse.virtual_cursor_wheel_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hardwaremouse.virtual_cursor_wheel_params }
}

impl<'a> Extract<'a> for VirtualCursorWheelParams {
    const TYPE_NAME: &'static str = "VirtualCursorWheelParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            speed: inst.get_f32("speed").unwrap_or_default(),
            time_max: inst.get_f32("timeMax").unwrap_or_default(),
            time_multiplier: inst.get_f32("timeMultiplier").unwrap_or_default(),
            linear_time: inst.get_f32("linearTime").unwrap_or_default(),
            second_step_delay: inst.get_f32("secondStepDelay").unwrap_or_default(),
            dead_zone: inst.get_f32("deadZone").unwrap_or_default(),
        }
    }
}

/// DCB type: `HardwareMouseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareMouseParams {
    /// `cursor` (Class)
    #[serde(default)]
    pub cursor: Option<Handle<VirtualCursorParams>>,
    /// `hoverFriction` (Class)
    #[serde(default)]
    pub hover_friction: Option<Handle<VirtualCursorHoverFrictionParams>>,
    /// `wheel` (Class)
    #[serde(default)]
    pub wheel: Option<Handle<VirtualCursorWheelParams>>,
    /// `enableDPadNavigation` (Boolean)
    #[serde(default)]
    pub enable_dpad_navigation: bool,
}

impl Pooled for HardwareMouseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hardwaremouse.hardware_mouse_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hardwaremouse.hardware_mouse_params }
}

impl<'a> Extract<'a> for HardwareMouseParams {
    const TYPE_NAME: &'static str = "HardwareMouseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            cursor: match inst.get("cursor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VirtualCursorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VirtualCursorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hover_friction: match inst.get("hoverFriction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VirtualCursorHoverFrictionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VirtualCursorHoverFrictionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wheel: match inst.get("wheel") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VirtualCursorWheelParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VirtualCursorWheelParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable_dpad_navigation: inst.get_bool("enableDPadNavigation").unwrap_or_default(),
        }
    }
}

