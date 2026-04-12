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

/// Pool storage for the `sglobaltractorbeamparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SglobaltractorbeamparamsPools {
    #[serde(default)]
    pub stractor_beam_holo_visual_params: Vec<Option<STractorBeamHoloVisualParams>>,
    #[serde(default)]
    pub stractor_beam_outline_visual_params: Vec<Option<STractorBeamOutlineVisualParams>>,
    #[serde(default)]
    pub sglobal_tractor_beam_params: Vec<Option<SGlobalTractorBeamParams>>,
}
