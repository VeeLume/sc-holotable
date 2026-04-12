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

/// Pool storage for the `weaponproceduralanimation` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WeaponproceduralanimationPools {
    #[serde(default)]
    pub weapon_procedural_clips_set_up: Vec<Option<WeaponProceduralClipsSetUp>>,
    #[serde(default)]
    pub weapon_procedural_animation: Vec<Option<WeaponProceduralAnimation>>,
}
