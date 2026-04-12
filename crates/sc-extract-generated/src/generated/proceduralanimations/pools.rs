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

/// Pool storage for the `proceduralanimations` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProceduralanimationsPools {
    #[serde(default)]
    pub procedural_animation_bone: Vec<Option<ProceduralAnimationBone>>,
    #[serde(default)]
    pub procedural_animation_sequence: Vec<Option<ProceduralAnimationSequence>>,
    #[serde(default)]
    pub procedural_animation: Vec<Option<ProceduralAnimation>>,
}
