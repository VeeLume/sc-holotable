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

/// Pool storage for the `ui-directrtt` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiDirectrttPools {
    #[serde(default)]
    pub direct_rtt_chromatic_aberration_params: Vec<Option<DirectRTT_ChromaticAberrationParams>>,
    #[serde(default)]
    pub direct_rtt_drop_shadow_params: Vec<Option<DirectRTT_DropShadowParams>>,
    #[serde(default)]
    pub direct_rtt_bloom_params: Vec<Option<DirectRTT_BloomParams>>,
    #[serde(default)]
    pub direct_rtt_pixel_grid_params: Vec<Option<DirectRTT_PixelGridParams>>,
    #[serde(default)]
    pub direct_rtt_interference_params: Vec<Option<DirectRTT_InterferenceParams>>,
    #[serde(default)]
    pub direct_rtt_after_tonemapping_params: Vec<Option<DirectRTT_AfterTonemappingParams>>,
}
