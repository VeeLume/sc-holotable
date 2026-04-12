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

/// Pool storage for the `dynamiclightingrig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DynamiclightingrigPools {
    #[serde(default)]
    pub scdynamic_rig_intensity_params: Vec<Option<SCDynamicRigIntensityParams>>,
    #[serde(default)]
    pub scdynamic_rig_light_params: Vec<Option<SCDynamicRigLightParams>>,
    #[serde(default)]
    pub scdynamic_lighting_rig_global_params: Vec<Option<SCDynamicLightingRigGlobalParams>>,
}
