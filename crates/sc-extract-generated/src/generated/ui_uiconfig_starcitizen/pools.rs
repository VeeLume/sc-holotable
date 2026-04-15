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

/// Pool storage for the `ui-uiconfig_starcitizen` feature.
#[derive(Default)]
pub struct UiUiconfig_starcitizenPools {
    pub uiconfig: Vec<Option<UIConfig>>,
    pub fpsreticle_config: Vec<Option<FPSReticle_Config>>,
    pub evareticle_config: Vec<Option<EVAReticle_Config>>,
    pub flash_palette: Vec<Option<Flash_Palette>>,
    pub flash_palette_entry: Vec<Option<Flash_PaletteEntry>>,
    pub uistate_color: Vec<Option<UIStateColor>>,
    pub uistate_color_threshold: Vec<Option<UIStateColor_Threshold>>,
}
