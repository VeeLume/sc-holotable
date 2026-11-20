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

/// Pool storage for the `aimotive` feature.
#[derive(Default)]
pub struct AimotivePools {
    pub condition_audio_intensity_heard: Vec<Option<Condition_AudioIntensityHeard>>,
    pub condition_stance: Vec<Option<Condition_Stance>>,
    pub condition_target_is_ship: Vec<Option<Condition_TargetIsShip>>,
}
