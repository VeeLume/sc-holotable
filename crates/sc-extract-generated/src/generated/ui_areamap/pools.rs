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

/// Pool storage for the `ui-areamap` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiAreamapPools {
    #[serde(default)]
    pub area_map_camera_params: Vec<Option<AreaMapCameraParams>>,
    #[serde(default)]
    pub area_map_params: Vec<Option<AreaMapParams>>,
    #[serde(default)]
    pub font_support_params: Vec<Option<FontSupportParams>>,
    #[serde(default)]
    pub sentity_component_location_data_params: Vec<Option<SEntityComponentLocationDataParams>>,
}
