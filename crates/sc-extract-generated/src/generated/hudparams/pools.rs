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

/// Pool storage for the `hudparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HudparamsPools {
    #[serde(default)]
    pub starget_selector_color_highlighting: Vec<Option<STargetSelectorColorHighlighting>>,
    #[serde(default)]
    pub starget_selector_hud_params: Vec<Option<STargetSelectorHudParams>>,
    #[serde(default)]
    pub sprojected_pitch_ladder_params: Vec<Option<SProjectedPitchLadderParams>>,
    #[serde(default)]
    pub sprojected_yaw_line_params: Vec<Option<SProjectedYawLineParams>>,
    #[serde(default)]
    pub sprojected_display_params: Vec<Option<SProjectedDisplayParams>>,
    #[serde(default)]
    pub shud_tape_params: Vec<Option<SHudTapeParams>>,
    #[serde(default)]
    pub sprojected_hud_params: Vec<Option<SProjectedHudParams>>,
    #[serde(default)]
    pub svehicle_hud_params: Vec<Option<SVehicleHudParams>>,
    #[serde(default)]
    pub saimable_gimbal_mode_labels: Vec<Option<SAimableGimbalModeLabels>>,
    #[serde(default)]
    pub saimable_controller_hud_params: Vec<Option<SAimableControllerHudParams>>,
}
