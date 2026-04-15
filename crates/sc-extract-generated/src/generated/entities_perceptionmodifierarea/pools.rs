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

/// Pool storage for the `entities-perceptionmodifierarea` feature.
#[derive(Default)]
pub struct EntitiesPerceptionmodifierareaPools {
    pub light_status_multiplier: Vec<Option<LightStatusMultiplier>>,
    pub action_area_audio_noise_extension: Vec<Option<ActionAreaAudioNoiseExtension>>,
    pub action_area_light_extension: Vec<Option<ActionAreaLightExtension>>,
}
