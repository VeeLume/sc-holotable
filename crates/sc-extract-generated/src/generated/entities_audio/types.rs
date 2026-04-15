// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-audio`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SAudioGroupControllerComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SAudioGroupControllerComponentParams {
    /// `powerLevelRtpc` (String)
    pub power_level_rtpc: String,
    /// `powerOnDefaultSwitchState` (String)
    pub power_on_default_switch_state: String,
    /// `powerOnAuxiliarySwitchState` (String)
    pub power_on_auxiliary_switch_state: String,
    /// `powerOnEmergencySwitchState` (String)
    pub power_on_emergency_switch_state: String,
    /// `powerOffSwitchState` (String)
    pub power_off_switch_state: String,
}

impl Pooled for SAudioGroupControllerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_audio.saudio_group_controller_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_audio.saudio_group_controller_component_params }
}

impl<'a> Extract<'a> for SAudioGroupControllerComponentParams {
    const TYPE_NAME: &'static str = "SAudioGroupControllerComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            power_level_rtpc: inst.get_str("powerLevelRtpc").map(String::from).unwrap_or_default(),
            power_on_default_switch_state: inst.get_str("powerOnDefaultSwitchState").map(String::from).unwrap_or_default(),
            power_on_auxiliary_switch_state: inst.get_str("powerOnAuxiliarySwitchState").map(String::from).unwrap_or_default(),
            power_on_emergency_switch_state: inst.get_str("powerOnEmergencySwitchState").map(String::from).unwrap_or_default(),
            power_off_switch_state: inst.get_str("powerOffSwitchState").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioAreaAmbienceComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct AudioAreaAmbienceComponentParams {
    /// `isEnabled` (Boolean)
    pub is_enabled: bool,
    /// `audioPlayTrigger` (Class)
    pub audio_play_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `audioStopTrigger` (Class)
    pub audio_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `areaFadeRTPC` (Class)
    pub area_fade_rtpc: Option<Handle<AudioRtpc>>,
    /// `audioEnvironment` (String)
    pub audio_environment: String,
    /// `audioSwitch` (String)
    pub audio_switch: String,
    /// `audioSwitchStateInside` (String)
    pub audio_switch_state_inside: String,
    /// `audioSwitchStateNear` (String)
    pub audio_switch_state_near: String,
    /// `audioSwitchStateFar` (String)
    pub audio_switch_state_far: String,
    /// `fadeDistance` (Single)
    pub fade_distance: f32,
    /// `environmentFadeDistance` (Single)
    pub environment_fade_distance: f32,
    /// `audioSignature` (Single)
    pub audio_signature: f32,
    /// `positionFadeDistance` (Single)
    pub position_fade_distance: f32,
    /// `pitch` (Single)
    pub pitch: f32,
    /// `volume` (Single)
    pub volume: f32,
    /// `atsVolumeOffset` (Single)
    pub ats_volume_offset: f32,
    /// `isManagedAudioObject` (Boolean)
    pub is_managed_audio_object: bool,
}

impl Pooled for AudioAreaAmbienceComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_audio.audio_area_ambience_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_audio.audio_area_ambience_component_params }
}

impl<'a> Extract<'a> for AudioAreaAmbienceComponentParams {
    const TYPE_NAME: &'static str = "AudioAreaAmbienceComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            is_enabled: inst.get_bool("isEnabled").unwrap_or_default(),
            audio_play_trigger: match inst.get("audioPlayTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_stop_trigger: match inst.get("audioStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            area_fade_rtpc: match inst.get("areaFadeRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_environment: inst.get_str("audioEnvironment").map(String::from).unwrap_or_default(),
            audio_switch: inst.get_str("audioSwitch").map(String::from).unwrap_or_default(),
            audio_switch_state_inside: inst.get_str("audioSwitchStateInside").map(String::from).unwrap_or_default(),
            audio_switch_state_near: inst.get_str("audioSwitchStateNear").map(String::from).unwrap_or_default(),
            audio_switch_state_far: inst.get_str("audioSwitchStateFar").map(String::from).unwrap_or_default(),
            fade_distance: inst.get_f32("fadeDistance").unwrap_or_default(),
            environment_fade_distance: inst.get_f32("environmentFadeDistance").unwrap_or_default(),
            audio_signature: inst.get_f32("audioSignature").unwrap_or_default(),
            position_fade_distance: inst.get_f32("positionFadeDistance").unwrap_or_default(),
            pitch: inst.get_f32("pitch").unwrap_or_default(),
            volume: inst.get_f32("volume").unwrap_or_default(),
            ats_volume_offset: inst.get_f32("atsVolumeOffset").unwrap_or_default(),
            is_managed_audio_object: inst.get_bool("isManagedAudioObject").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioTriggerSpotParams`
/// Inherits from: `DataForgeComponentParams`
pub struct AudioTriggerSpotParams {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `audioTriggerPlayTriggerName` (Class)
    pub audio_trigger_play_trigger_name: Option<Handle<GlobalResourceAudio>>,
    /// `audioTriggerStopTriggerName` (Class)
    pub audio_trigger_stop_trigger_name: Option<Handle<GlobalResourceAudio>>,
    /// `attenuationScalingFactor` (Single)
    pub attenuation_scaling_factor: f32,
    /// `volume` (Single)
    pub volume: f32,
    /// `pitch` (Single)
    pub pitch: f32,
    /// `lowPassFilter` (Single)
    pub low_pass_filter: f32,
    /// `highPassFilter` (Single)
    pub high_pass_filter: f32,
    /// `volumeRandomRange` (Single)
    pub volume_random_range: f32,
    /// `pitchRandomRange` (Single)
    pub pitch_random_range: f32,
    /// `playRandom` (Boolean)
    pub play_random: bool,
    /// `minDelay` (Single)
    pub min_delay: f32,
    /// `maxDelay` (Single)
    pub max_delay: f32,
    /// `delayIsFromEnd` (Boolean)
    pub delay_is_from_end: bool,
    /// `radiusRandom` (Single)
    pub radius_random: f32,
    /// `playOnX` (Boolean)
    pub play_on_x: bool,
    /// `playOnY` (Boolean)
    pub play_on_y: bool,
    /// `playOnZ` (Boolean)
    pub play_on_z: bool,
    /// `audioRTPCUserRtpc1Name` (Class)
    pub audio_rtpcuser_rtpc1_name: Option<Handle<AudioRtpc>>,
    /// `userRtpc1Value` (Single)
    pub user_rtpc1_value: f32,
    /// `audioRTPCUserRtpc2Name` (Class)
    pub audio_rtpcuser_rtpc2_name: Option<Handle<AudioRtpc>>,
    /// `userRtpc2Value` (Single)
    pub user_rtpc2_value: f32,
    /// `audioRTPCUserRtpc3Name` (Class)
    pub audio_rtpcuser_rtpc3_name: Option<Handle<AudioRtpc>>,
    /// `userRtpc3Value` (Single)
    pub user_rtpc3_value: f32,
    /// `voiceStealingRule` (Int32)
    pub voice_stealing_rule: i32,
    /// `voiceCount` (Int32)
    pub voice_count: i32,
    /// `multiPositionType` (Int32)
    pub multi_position_type: i32,
    /// `audioRTPCAreaFadeRtpc` (Class)
    pub audio_rtpcarea_fade_rtpc: Option<Handle<AudioRtpc>>,
    /// `enablePropagationAndSoundsim` (Boolean)
    pub enable_propagation_and_soundsim: bool,
    /// `commsChannel` (Reference)
    pub comms_channel: Option<CigGuid>,
}

impl Pooled for AudioTriggerSpotParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_audio.audio_trigger_spot_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_audio.audio_trigger_spot_params }
}

impl<'a> Extract<'a> for AudioTriggerSpotParams {
    const TYPE_NAME: &'static str = "AudioTriggerSpotParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            audio_trigger_play_trigger_name: match inst.get("audioTriggerPlayTriggerName") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_trigger_stop_trigger_name: match inst.get("audioTriggerStopTriggerName") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            attenuation_scaling_factor: inst.get_f32("attenuationScalingFactor").unwrap_or_default(),
            volume: inst.get_f32("volume").unwrap_or_default(),
            pitch: inst.get_f32("pitch").unwrap_or_default(),
            low_pass_filter: inst.get_f32("lowPassFilter").unwrap_or_default(),
            high_pass_filter: inst.get_f32("highPassFilter").unwrap_or_default(),
            volume_random_range: inst.get_f32("volumeRandomRange").unwrap_or_default(),
            pitch_random_range: inst.get_f32("pitchRandomRange").unwrap_or_default(),
            play_random: inst.get_bool("playRandom").unwrap_or_default(),
            min_delay: inst.get_f32("minDelay").unwrap_or_default(),
            max_delay: inst.get_f32("maxDelay").unwrap_or_default(),
            delay_is_from_end: inst.get_bool("delayIsFromEnd").unwrap_or_default(),
            radius_random: inst.get_f32("radiusRandom").unwrap_or_default(),
            play_on_x: inst.get_bool("playOnX").unwrap_or_default(),
            play_on_y: inst.get_bool("playOnY").unwrap_or_default(),
            play_on_z: inst.get_bool("playOnZ").unwrap_or_default(),
            audio_rtpcuser_rtpc1_name: match inst.get("audioRTPCUserRtpc1Name") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            user_rtpc1_value: inst.get_f32("userRtpc1Value").unwrap_or_default(),
            audio_rtpcuser_rtpc2_name: match inst.get("audioRTPCUserRtpc2Name") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            user_rtpc2_value: inst.get_f32("userRtpc2Value").unwrap_or_default(),
            audio_rtpcuser_rtpc3_name: match inst.get("audioRTPCUserRtpc3Name") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            user_rtpc3_value: inst.get_f32("userRtpc3Value").unwrap_or_default(),
            voice_stealing_rule: inst.get_i32("voiceStealingRule").unwrap_or_default(),
            voice_count: inst.get_i32("voiceCount").unwrap_or_default(),
            multi_position_type: inst.get_i32("multiPositionType").unwrap_or_default(),
            audio_rtpcarea_fade_rtpc: match inst.get("audioRTPCAreaFadeRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            enable_propagation_and_soundsim: inst.get_bool("enablePropagationAndSoundsim").unwrap_or_default(),
            comms_channel: inst.get("commsChannel").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `EntityAudioControllerRtpcSubscriberParams`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityAudioControllerRtpcSubscriberParams {
    /// `rtpcs` (Class (array))
    pub rtpcs: Vec<Handle<AudioRtpc>>,
    /// `rtpcLists` (Reference (array))
    pub rtpc_lists: Vec<CigGuid>,
}

impl Pooled for EntityAudioControllerRtpcSubscriberParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_audio.entity_audio_controller_rtpc_subscriber_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_audio.entity_audio_controller_rtpc_subscriber_params }
}

impl<'a> Extract<'a> for EntityAudioControllerRtpcSubscriberParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerRtpcSubscriberParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rtpcs: inst.get_array("rtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            rtpc_lists: inst.get_array("rtpcLists")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityComponentAudioEnvironmentFeedbackPointParams`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentAudioEnvironmentFeedbackPointParams {
    /// `feedbackPointDef` (Reference)
    pub feedback_point_def: Option<CigGuid>,
}

impl Pooled for EntityComponentAudioEnvironmentFeedbackPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_audio.entity_component_audio_environment_feedback_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_audio.entity_component_audio_environment_feedback_point_params }
}

impl<'a> Extract<'a> for EntityComponentAudioEnvironmentFeedbackPointParams {
    const TYPE_NAME: &'static str = "EntityComponentAudioEnvironmentFeedbackPointParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            feedback_point_def: inst.get("feedbackPointDef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `EntityComponentVehicleAudioPointParams`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentVehicleAudioPointParams {
    /// `loopStart` (Class)
    pub loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `loopStop` (Class)
    pub loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `soundSimOnOneshot` (Class)
    pub sound_sim_on_oneshot: Option<Handle<GlobalResourceAudio>>,
    /// `soundSimOffOneshot` (Class)
    pub sound_sim_off_oneshot: Option<Handle<GlobalResourceAudio>>,
    /// `shipAudioOnOneshot` (Class)
    pub ship_audio_on_oneshot: Option<Handle<GlobalResourceAudio>>,
    /// `shipAudioOffOneshot` (Class)
    pub ship_audio_off_oneshot: Option<Handle<GlobalResourceAudio>>,
    /// `rtpcs` (Class (array))
    pub rtpcs: Vec<Handle<AudioRtpc>>,
    /// `rtpcLists` (Reference (array))
    pub rtpc_lists: Vec<CigGuid>,
}

impl Pooled for EntityComponentVehicleAudioPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_audio.entity_component_vehicle_audio_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_audio.entity_component_vehicle_audio_point_params }
}

impl<'a> Extract<'a> for EntityComponentVehicleAudioPointParams {
    const TYPE_NAME: &'static str = "EntityComponentVehicleAudioPointParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loop_start: match inst.get("loopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            loop_stop: match inst.get("loopStop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            sound_sim_on_oneshot: match inst.get("soundSimOnOneshot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            sound_sim_off_oneshot: match inst.get("soundSimOffOneshot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_audio_on_oneshot: match inst.get("shipAudioOnOneshot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ship_audio_off_oneshot: match inst.get("shipAudioOffOneshot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rtpcs: inst.get_array("rtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            rtpc_lists: inst.get_array("rtpcLists")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MusicAreaComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct MusicAreaComponentParams {
    /// `onEnterWwiseEvent` (Class)
    pub on_enter_wwise_event: Option<Handle<GlobalResourceAudio>>,
    /// `onLeaveWwiseEvent` (Class)
    pub on_leave_wwise_event: Option<Handle<GlobalResourceAudio>>,
    /// `onEnterCueSwitch` (Reference)
    pub on_enter_cue_switch: Option<CigGuid>,
    /// `radius` (Single)
    pub radius: f32,
    /// `maxRetriggerCount` (Int32)
    pub max_retrigger_count: i32,
    /// `retriggerDelaySecs` (Single)
    pub retrigger_delay_secs: f32,
    /// `retriggerDelayOnlyOnEnter` (Boolean)
    pub retrigger_delay_only_on_enter: bool,
    /// `stopMusicOnExit` (Boolean)
    pub stop_music_on_exit: bool,
    /// `bounds` (Class)
    pub bounds: Option<Handle<Vec3>>,
    /// `onEnterSecondarySwitch` (Reference)
    pub on_enter_secondary_switch: Option<CigGuid>,
    /// `onLeaveSecondarySwitch` (Reference)
    pub on_leave_secondary_switch: Option<CigGuid>,
    /// `distToCentreParameter` (Reference)
    pub dist_to_centre_parameter: Option<CigGuid>,
    /// `distToCentreRtpc` (Class)
    pub dist_to_centre_rtpc: Option<Handle<AudioRtpc>>,
    /// `useAreaComponent` (Boolean)
    pub use_area_component: bool,
}

impl Pooled for MusicAreaComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_audio.music_area_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_audio.music_area_component_params }
}

impl<'a> Extract<'a> for MusicAreaComponentParams {
    const TYPE_NAME: &'static str = "MusicAreaComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            on_enter_wwise_event: match inst.get("onEnterWwiseEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            on_leave_wwise_event: match inst.get("onLeaveWwiseEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            on_enter_cue_switch: inst.get("onEnterCueSwitch").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            radius: inst.get_f32("radius").unwrap_or_default(),
            max_retrigger_count: inst.get_i32("maxRetriggerCount").unwrap_or_default(),
            retrigger_delay_secs: inst.get_f32("retriggerDelaySecs").unwrap_or_default(),
            retrigger_delay_only_on_enter: inst.get_bool("retriggerDelayOnlyOnEnter").unwrap_or_default(),
            stop_music_on_exit: inst.get_bool("stopMusicOnExit").unwrap_or_default(),
            bounds: match inst.get("bounds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            on_enter_secondary_switch: inst.get("onEnterSecondarySwitch").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            on_leave_secondary_switch: inst.get("onLeaveSecondarySwitch").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            dist_to_centre_parameter: inst.get("distToCentreParameter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            dist_to_centre_rtpc: match inst.get("distToCentreRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            use_area_component: inst.get_bool("useAreaComponent").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioSplineParams`
/// Inherits from: `DataForgeComponentParams`
pub struct AudioSplineParams {
    /// `activatedRoute` (Boolean)
    pub activated_route: bool,
    /// `moveTowardsListener` (Boolean)
    pub move_towards_listener: bool,
    /// `traversalDuration` (Single)
    pub traversal_duration: f32,
    /// `audioRTPCDistanceTravelled` (Class)
    pub audio_rtpcdistance_travelled: Option<Handle<AudioRtpc>>,
    /// `activationTag` (Reference)
    pub activation_tag: Option<CigGuid>,
}

impl Pooled for AudioSplineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_audio.audio_spline_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_audio.audio_spline_params }
}

impl<'a> Extract<'a> for AudioSplineParams {
    const TYPE_NAME: &'static str = "AudioSplineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            activated_route: inst.get_bool("activatedRoute").unwrap_or_default(),
            move_towards_listener: inst.get_bool("moveTowardsListener").unwrap_or_default(),
            traversal_duration: inst.get_f32("traversalDuration").unwrap_or_default(),
            audio_rtpcdistance_travelled: match inst.get("audioRTPCDistanceTravelled") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            activation_tag: inst.get("activationTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `EntityComponentVibrationAudioPointParams`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentVibrationAudioPointParams {
    /// `pointDefinition` (Reference)
    pub point_definition: Option<CigGuid>,
}

impl Pooled for EntityComponentVibrationAudioPointParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_audio.entity_component_vibration_audio_point_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_audio.entity_component_vibration_audio_point_params }
}

impl<'a> Extract<'a> for EntityComponentVibrationAudioPointParams {
    const TYPE_NAME: &'static str = "EntityComponentVibrationAudioPointParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            point_definition: inst.get("pointDefinition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

