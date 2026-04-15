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

/// Pool storage for the `ui-radardisplay_config_radar` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiRadardisplay_config_radarPools {
    #[serde(default)]
    pub radar_plate_config: Vec<Option<RadarPlate_Config>>,
    #[serde(default)]
    pub radar_display_entry_effects_config: Vec<Option<RadarDisplayEntryEffects_Config>>,
    #[serde(default)]
    pub radar_display_config: Vec<Option<RadarDisplay_Config>>,
}
