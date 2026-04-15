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

/// Pool storage for the `ui-radardisplay_config_radar` feature.
#[derive(Default)]
pub struct UiRadardisplay_config_radarPools {
    pub radar_plate_config: Vec<Option<RadarPlate_Config>>,
    pub radar_display_entry_effects_config: Vec<Option<RadarDisplayEntryEffects_Config>>,
    pub radar_display_config: Vec<Option<RadarDisplay_Config>>,
}
