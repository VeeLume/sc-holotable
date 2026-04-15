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

/// Pool storage for the `sglobaltractorbeamparams` feature.
#[derive(Default)]
pub struct SglobaltractorbeamparamsPools {
    pub stractor_beam_holo_visual_params: Vec<Option<STractorBeamHoloVisualParams>>,
    pub stractor_beam_outline_visual_params: Vec<Option<STractorBeamOutlineVisualParams>>,
    pub sglobal_tractor_beam_params: Vec<Option<SGlobalTractorBeamParams>>,
}
