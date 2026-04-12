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

/// Pool storage for the `ui-radar3dpresets` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiRadar3dpresetsPools {
    #[serde(default)]
    pub uidata_bank_display3_dspace_dust_params: Vec<Option<UIDataBankDisplay3DSpaceDustParams>>,
    #[serde(default)]
    pub uidata_bank_display3_dinterpolate_params: Vec<Option<UIDataBankDisplay3DInterpolateParams>>,
    #[serde(default)]
    pub uidata_bank_display3_dparams: Vec<Option<UIDataBankDisplay3DParams>>,
    #[serde(default)]
    pub uiworld_display_path_state_params: Vec<Option<UIWorldDisplayPathStateParams>>,
    #[serde(default)]
    pub uiworld_display_path_line_params: Vec<Option<UIWorldDisplayPathLineParams>>,
    #[serde(default)]
    pub uiworld_display_path_params: Vec<Option<UIWorldDisplayPathParams>>,
    #[serde(default)]
    pub radar_display3_dpreset: Vec<Option<RadarDisplay3DPreset>>,
}
