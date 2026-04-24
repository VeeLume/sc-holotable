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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `VirtualCursorParams`
pub struct VirtualCursorParams {
    /// `accelerationEnabled` (Boolean)
    pub acceleration_enabled: bool,
    /// `accelerationMax` (Single)
    pub acceleration_max: f32,
    /// `accelerationRate` (Single)
    pub acceleration_rate: f32,
    /// `accelerationDeadZone` (Single)
    pub acceleration_dead_zone: f32,
    /// `sensitivity` (Single)
    pub sensitivity: f32,
    /// `sensitivityPower` (Single)
    pub sensitivity_power: f32,
    /// `deadZone` (Single)
    pub dead_zone: f32,
    /// `consoleCursorEnable` (Boolean)
    pub console_cursor_enable: bool,
    /// `consoleCursorScale` (Single)
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
pub struct VirtualCursorHoverFrictionParams {
    /// `maxFriction` (Single)
    pub max_friction: f32,
    /// `elementSizeTreshold` (Single)
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
pub struct VirtualCursorWheelParams {
    /// `speed` (Single)
    pub speed: f32,
    /// `timeMax` (Single)
    pub time_max: f32,
    /// `timeMultiplier` (Single)
    pub time_multiplier: f32,
    /// `linearTime` (Single)
    pub linear_time: f32,
    /// `secondStepDelay` (Single)
    pub second_step_delay: f32,
    /// `deadZone` (Single)
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
pub struct HardwareMouseParams {
    /// `cursor` (Class)
    pub cursor: Option<Handle<VirtualCursorParams>>,
    /// `hoverFriction` (Class)
    pub hover_friction: Option<Handle<VirtualCursorHoverFrictionParams>>,
    /// `wheel` (Class)
    pub wheel: Option<Handle<VirtualCursorWheelParams>>,
    /// `enableDPadNavigation` (Boolean)
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
                _ => None,
            },
            hover_friction: match inst.get("hoverFriction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VirtualCursorHoverFrictionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            wheel: match inst.get("wheel") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VirtualCursorWheelParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            enable_dpad_navigation: inst.get_bool("enableDPadNavigation").unwrap_or_default(),
        }
    }
}

