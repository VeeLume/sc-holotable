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

/// Pool storage for the `entities-capturearea` feature.
#[derive(Default)]
pub struct EntitiesCaptureareaPools {
    pub area_outdoor_material_params: Vec<Option<AreaOutdoorMaterialParams>>,
    pub capture_area_uiparams: Vec<Option<CaptureAreaUIParams>>,
}
