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

/// Pool storage for the `hudparams` feature.
#[derive(Default)]
pub struct HudparamsPools {
    pub starget_selector_color_highlighting: Vec<Option<STargetSelectorColorHighlighting>>,
    pub starget_selector_hud_params: Vec<Option<STargetSelectorHudParams>>,
    pub sprojected_pitch_ladder_params: Vec<Option<SProjectedPitchLadderParams>>,
    pub sprojected_yaw_line_params: Vec<Option<SProjectedYawLineParams>>,
    pub sprojected_display_params: Vec<Option<SProjectedDisplayParams>>,
    pub shud_tape_params: Vec<Option<SHudTapeParams>>,
    pub sprojected_hud_params: Vec<Option<SProjectedHudParams>>,
    pub svehicle_hud_params: Vec<Option<SVehicleHudParams>>,
    pub saimable_gimbal_mode_labels: Vec<Option<SAimableGimbalModeLabels>>,
    pub saimable_controller_hud_params: Vec<Option<SAimableControllerHudParams>>,
}
