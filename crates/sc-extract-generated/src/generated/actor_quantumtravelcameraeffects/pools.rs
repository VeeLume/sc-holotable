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

/// Pool storage for the `actor-quantumtravelcameraeffects` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorQuantumtravelcameraeffectsPools {
    #[serde(default)]
    pub squantum_camera_state_mapping_def: Vec<Option<SQuantumCameraStateMappingDef>>,
    #[serde(default)]
    pub squantum_camera_state_effects_def: Vec<Option<SQuantumCameraStateEffectsDef>>,
    #[serde(default)]
    pub squantum_camera_effects_def: Vec<Option<SQuantumCameraEffectsDef>>,
}
