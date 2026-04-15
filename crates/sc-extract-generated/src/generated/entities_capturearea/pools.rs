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

/// Pool storage for the `entities-capturearea` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesCaptureareaPools {
    #[serde(default)]
    pub area_outdoor_material_params: Vec<Option<AreaOutdoorMaterialParams>>,
    #[serde(default)]
    pub capture_area_uiparams: Vec<Option<CaptureAreaUIParams>>,
}
