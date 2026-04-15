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

/// Pool storage for the `entities-audio` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesAudioPools {
    #[serde(default)]
    pub saudio_group_controller_component_params: Vec<Option<SAudioGroupControllerComponentParams>>,
    #[serde(default)]
    pub audio_area_ambience_component_params: Vec<Option<AudioAreaAmbienceComponentParams>>,
    #[serde(default)]
    pub audio_trigger_spot_params: Vec<Option<AudioTriggerSpotParams>>,
    #[serde(default)]
    pub entity_audio_controller_rtpc_subscriber_params: Vec<Option<EntityAudioControllerRtpcSubscriberParams>>,
    #[serde(default)]
    pub entity_component_audio_environment_feedback_point_params: Vec<Option<EntityComponentAudioEnvironmentFeedbackPointParams>>,
    #[serde(default)]
    pub entity_component_vehicle_audio_point_params: Vec<Option<EntityComponentVehicleAudioPointParams>>,
    #[serde(default)]
    pub music_area_component_params: Vec<Option<MusicAreaComponentParams>>,
    #[serde(default)]
    pub audio_spline_params: Vec<Option<AudioSplineParams>>,
    #[serde(default)]
    pub entity_component_vibration_audio_point_params: Vec<Option<EntityComponentVibrationAudioPointParams>>,
}
