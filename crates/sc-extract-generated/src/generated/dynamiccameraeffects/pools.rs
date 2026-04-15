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

/// Pool storage for the `dynamiccameraeffects` feature.
#[derive(Default)]
pub struct DynamiccameraeffectsPools {
    pub dynamic_camera_effects_list: Vec<Option<DynamicCameraEffectsList>>,
    pub constant_dofpos_weights: Vec<Option<ConstantDOFPosWeights>>,
    pub constant_dofweights: Vec<Option<ConstantDOFWeights>>,
    pub constant_dofgrid: Vec<Option<ConstantDOFGrid>>,
    pub constant_dofglobal_data: Vec<Option<ConstantDOFGlobalData>>,
}
