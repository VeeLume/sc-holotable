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

/// Pool storage for the `entities-audio` feature.
#[derive(Default)]
pub struct EntitiesAudioPools {
    pub saudio_group_controller_component_params: Vec<Option<SAudioGroupControllerComponentParams>>,
    pub audio_area_ambience_component_params: Vec<Option<AudioAreaAmbienceComponentParams>>,
    pub audio_trigger_spot_params: Vec<Option<AudioTriggerSpotParams>>,
    pub entity_audio_controller_rtpc_subscriber_params:
        Vec<Option<EntityAudioControllerRtpcSubscriberParams>>,
    pub entity_component_audio_environment_feedback_point_params:
        Vec<Option<EntityComponentAudioEnvironmentFeedbackPointParams>>,
    pub entity_component_vehicle_audio_point_params:
        Vec<Option<EntityComponentVehicleAudioPointParams>>,
    pub music_area_component_params: Vec<Option<MusicAreaComponentParams>>,
    pub audio_spline_params: Vec<Option<AudioSplineParams>>,
    pub entity_component_vibration_audio_point_params:
        Vec<Option<EntityComponentVibrationAudioPointParams>>,
}
