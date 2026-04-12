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

/// Pool storage for the `dynamiccameraeffects` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DynamiccameraeffectsPools {
    #[serde(default)]
    pub dynamic_camera_effects: Vec<Option<DynamicCameraEffects>>,
    #[serde(default)]
    pub dynamic_camera_effects_renderer_params: Vec<Option<DynamicCameraEffectsRendererParams>>,
    #[serde(default)]
    pub dynamic_camera_effects_list: Vec<Option<DynamicCameraEffectsList>>,
    #[serde(default)]
    pub constant_dofpos_weights: Vec<Option<ConstantDOFPosWeights>>,
    #[serde(default)]
    pub constant_dofweights: Vec<Option<ConstantDOFWeights>>,
    #[serde(default)]
    pub constant_dofgrid: Vec<Option<ConstantDOFGrid>>,
    #[serde(default)]
    pub constant_dofglobal_data: Vec<Option<ConstantDOFGlobalData>>,
}
