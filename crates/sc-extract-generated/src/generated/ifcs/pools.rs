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

/// Pool storage for the `ifcs` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IfcsPools {
    #[serde(default)]
    pub espparams: Vec<Option<ESPParams>>,
    #[serde(default)]
    pub sifcsesp_params: Vec<Option<SIFCSEspParams>>,
    #[serde(default)]
    pub sifcsesp: Vec<Option<SIFCSEsp>>,
    #[serde(default)]
    pub sifcsgame_mode_physics_damping: Vec<Option<SIFCSGameModePhysicsDamping>>,
    #[serde(default)]
    pub sifcsgame_mode_params: Vec<Option<SIFCSGameModeParams>>,
}
