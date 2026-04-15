// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `hardwaremouse` feature.
#[derive(Default)]
pub struct HardwaremousePools {
    pub virtual_cursor_params: Vec<Option<VirtualCursorParams>>,
    pub virtual_cursor_hover_friction_params: Vec<Option<VirtualCursorHoverFrictionParams>>,
    pub virtual_cursor_wheel_params: Vec<Option<VirtualCursorWheelParams>>,
    pub hardware_mouse_params: Vec<Option<HardwareMouseParams>>,
}
