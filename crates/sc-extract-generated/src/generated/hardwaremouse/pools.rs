// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `hardwaremouse` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HardwaremousePools {
    #[serde(default)]
    pub virtual_cursor_params: Vec<Option<VirtualCursorParams>>,
    #[serde(default)]
    pub virtual_cursor_hover_friction_params: Vec<Option<VirtualCursorHoverFrictionParams>>,
    #[serde(default)]
    pub virtual_cursor_wheel_params: Vec<Option<VirtualCursorWheelParams>>,
    #[serde(default)]
    pub hardware_mouse_params: Vec<Option<HardwareMouseParams>>,
}
