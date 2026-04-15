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

/// Pool storage for the `ui-directrtt` feature.
#[derive(Default)]
pub struct UiDirectrttPools {
    pub direct_rtt_chromatic_aberration_params: Vec<Option<DirectRTT_ChromaticAberrationParams>>,
    pub direct_rtt_drop_shadow_params: Vec<Option<DirectRTT_DropShadowParams>>,
    pub direct_rtt_bloom_params: Vec<Option<DirectRTT_BloomParams>>,
    pub direct_rtt_pixel_grid_params: Vec<Option<DirectRTT_PixelGridParams>>,
    pub direct_rtt_interference_params: Vec<Option<DirectRTT_InterferenceParams>>,
    pub direct_rtt_after_tonemapping_params: Vec<Option<DirectRTT_AfterTonemappingParams>>,
}
