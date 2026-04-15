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

/// Pool storage for the `ifcs` feature.
#[derive(Default)]
pub struct IfcsPools {
    pub espparams: Vec<Option<ESPParams>>,
    pub sifcsesp_params: Vec<Option<SIFCSEspParams>>,
    pub sifcsesp: Vec<Option<SIFCSEsp>>,
    pub sifcsgame_mode_physics_damping: Vec<Option<SIFCSGameModePhysicsDamping>>,
    pub sifcsgame_mode_params: Vec<Option<SIFCSGameModeParams>>,
}
