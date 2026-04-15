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

/// Pool storage for the `entities-lights` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesLightsPools {
    #[serde(default)]
    pub slight_flicker_volume_component_params: Vec<Option<SLightFlickerVolumeComponentParams>>,
    #[serde(default)]
    pub slight_flicker_filtering_params: Vec<Option<SLightFlickerFilteringParams>>,
    #[serde(default)]
    pub slight_flicker_anim_params: Vec<Option<SLightFlickerAnimParams>>,
    #[serde(default)]
    pub light_audio_component_params: Vec<Option<LightAudioComponentParams>>,
    #[serde(default)]
    pub light_box_component_params: Vec<Option<LightBoxComponentParams>>,
}
