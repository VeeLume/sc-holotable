// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `ui-hudcolors_shipcolorpalettes` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiHudcolors_shipcolorpalettesPools {
    #[serde(default)]
    pub hud_colors: Vec<Option<HudColors>>,
    #[serde(default)]
    pub hud_color_palette: Vec<Option<HudColor_Palette>>,
    #[serde(default)]
    pub hud_color_entry: Vec<Option<HudColor_Entry>>,
    #[serde(default)]
    pub hud_color_custom_entry: Vec<Option<HudColor_CustomEntry>>,
    #[serde(default)]
    pub hud_color_holo_param: Vec<Option<HudColor_HoloParam>>,
    #[serde(default)]
    pub hud_color_holo_mat_colors: Vec<Option<HudColor_HoloMatColors>>,
    #[serde(default)]
    pub hud_color_holo_mat_textures: Vec<Option<HudColor_HoloMatTextures>>,
}
