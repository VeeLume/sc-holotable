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

/// Pool storage for the `entities-lights` feature.
#[derive(Default)]
pub struct EntitiesLightsPools {
    pub slight_flicker_volume_component_params: Vec<Option<SLightFlickerVolumeComponentParams>>,
    pub slight_flicker_filtering_params: Vec<Option<SLightFlickerFilteringParams>>,
    pub slight_flicker_anim_params: Vec<Option<SLightFlickerAnimParams>>,
    pub light_audio_component_params: Vec<Option<LightAudioComponentParams>>,
    pub light_box_component_params: Vec<Option<LightBoxComponentParams>>,
}
