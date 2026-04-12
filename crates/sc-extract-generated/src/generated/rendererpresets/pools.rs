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

/// Pool storage for the `rendererpresets` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RendererpresetsPools {
    #[serde(default)]
    pub camera_lens_streak: Vec<Option<CameraLensStreak>>,
    #[serde(default)]
    pub camera_lens_distortion: Vec<Option<CameraLensDistortion>>,
    #[serde(default)]
    pub camera_lens_chromatic_aberration: Vec<Option<CameraLensChromaticAberration>>,
    #[serde(default)]
    pub camera_lens_ghost_instance: Vec<Option<CameraLensGhostInstance>>,
    #[serde(default)]
    pub camera_lens_ghost_set: Vec<Option<CameraLensGhostSet>>,
    #[serde(default)]
    pub camera_lens_params: Vec<Option<CameraLensParams>>,
}
