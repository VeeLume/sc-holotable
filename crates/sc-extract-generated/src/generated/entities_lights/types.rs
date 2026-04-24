// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-lights`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SLightFlickerVolumeComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SLightFlickerVolumeComponentParams {
    /// `waveControlParams` (StrongPointer)
    pub wave_control_params: Option<LightFlickerWaveParamsPtr>,
    /// `animParams` (Class)
    pub anim_params: Option<Handle<SLightFlickerAnimParams>>,
    /// `randomSeed` (UInt32)
    pub random_seed: u32,
    /// `searchRadius` (Single)
    pub search_radius: f32,
    /// `wavePasses` (Int32)
    pub wave_passes: i32,
    /// `filteringParams` (Class)
    pub filtering_params: Option<Handle<SLightFlickerFilteringParams>>,
}

impl Pooled for SLightFlickerVolumeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_lights.slight_flicker_volume_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_lights.slight_flicker_volume_component_params }
}

impl<'a> Extract<'a> for SLightFlickerVolumeComponentParams {
    const TYPE_NAME: &'static str = "SLightFlickerVolumeComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wave_control_params: match inst.get("waveControlParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(LightFlickerWaveParamsPtr::from_ref(b, r)),
                _ => None,
            },
            anim_params: match inst.get("animParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SLightFlickerAnimParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            random_seed: inst.get_u32("randomSeed").unwrap_or_default(),
            search_radius: inst.get_f32("searchRadius").unwrap_or_default(),
            wave_passes: inst.get_i32("wavePasses").unwrap_or_default(),
            filtering_params: match inst.get("filteringParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SLightFlickerFilteringParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SLightFlickerFilteringParams`
pub struct SLightFlickerFilteringParams {
    /// `ignoreOnlyFogLights` (Boolean)
    pub ignore_only_fog_lights: bool,
    /// `filterByVisArea` (Boolean)
    pub filter_by_vis_area: bool,
    /// `ignoreEnvironmentProbes` (Boolean)
    pub ignore_environment_probes: bool,
}

impl Pooled for SLightFlickerFilteringParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_lights.slight_flicker_filtering_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_lights.slight_flicker_filtering_params }
}

impl<'a> Extract<'a> for SLightFlickerFilteringParams {
    const TYPE_NAME: &'static str = "SLightFlickerFilteringParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            ignore_only_fog_lights: inst.get_bool("ignoreOnlyFogLights").unwrap_or_default(),
            filter_by_vis_area: inst.get_bool("filterByVisArea").unwrap_or_default(),
            ignore_environment_probes: inst.get_bool("ignoreEnvironmentProbes").unwrap_or_default(),
        }
    }
}

/// DCB type: `SLightFlickerAnimParams`
pub struct SLightFlickerAnimParams {
    /// `minAnimTime` (Single)
    pub min_anim_time: f32,
    /// `maxAnimTime` (Single)
    pub max_anim_time: f32,
    /// `minSpeedMultiplier` (Single)
    pub min_speed_multiplier: f32,
    /// `maxSpeedMultiplier` (Single)
    pub max_speed_multiplier: f32,
    /// `lightStyles` (Int32 (array))
    pub light_styles: Vec<i32>,
    /// `endColor` (Class)
    pub end_color: Option<Handle<SRGB8>>,
    /// `restoreOriginalAfterTime` (Single)
    pub restore_original_after_time: f32,
    /// `keepActiveAfterWaveFinished` (Boolean)
    pub keep_active_after_wave_finished: bool,
    /// `trackviewTime` (Single)
    pub trackview_time: f32,
}

impl Pooled for SLightFlickerAnimParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_lights.slight_flicker_anim_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_lights.slight_flicker_anim_params }
}

impl<'a> Extract<'a> for SLightFlickerAnimParams {
    const TYPE_NAME: &'static str = "SLightFlickerAnimParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_anim_time: inst.get_f32("minAnimTime").unwrap_or_default(),
            max_anim_time: inst.get_f32("maxAnimTime").unwrap_or_default(),
            min_speed_multiplier: inst.get_f32("minSpeedMultiplier").unwrap_or_default(),
            max_speed_multiplier: inst.get_f32("maxSpeedMultiplier").unwrap_or_default(),
            light_styles: inst.get_array("lightStyles")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
            end_color: match inst.get("endColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            restore_original_after_time: inst.get_f32("restoreOriginalAfterTime").unwrap_or_default(),
            keep_active_after_wave_finished: inst.get_bool("keepActiveAfterWaveFinished").unwrap_or_default(),
            trackview_time: inst.get_f32("trackviewTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `LightAudioComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct LightAudioComponentParams {
    /// `enableAudio` (Boolean)
    pub enable_audio: bool,
    /// `playTrigger` (Class)
    pub play_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `stopTrigger` (Class)
    pub stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `lightGroupStateDefaultPlayTrigger` (Class)
    pub light_group_state_default_play_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `lightGroupStateDefaultStopTrigger` (Class)
    pub light_group_state_default_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `lightGroupStateAuxiliaryPlayTrigger` (Class)
    pub light_group_state_auxiliary_play_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `lightGroupStateAuxiliaryStopTrigger` (Class)
    pub light_group_state_auxiliary_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `lightGroupStateEmergencyPlayTrigger` (Class)
    pub light_group_state_emergency_play_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `lightGroupStateEmergencyStopTrigger` (Class)
    pub light_group_state_emergency_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `lightGroupStateCinematicPlayTrigger` (Class)
    pub light_group_state_cinematic_play_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `lightGroupStateCinematicStopTrigger` (Class)
    pub light_group_state_cinematic_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `luminanceRtpc` (Class)
    pub luminance_rtpc: Option<Handle<AudioRtpc>>,
    /// `attenuationScale` (Single)
    pub attenuation_scale: f32,
    /// `volume_db` (Single)
    pub volume_db: f32,
}

impl Pooled for LightAudioComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_lights.light_audio_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_lights.light_audio_component_params }
}

impl<'a> Extract<'a> for LightAudioComponentParams {
    const TYPE_NAME: &'static str = "LightAudioComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_audio: inst.get_bool("enableAudio").unwrap_or_default(),
            play_trigger: match inst.get("playTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            stop_trigger: match inst.get("stopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_group_state_default_play_trigger: match inst.get("lightGroupStateDefaultPlayTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_group_state_default_stop_trigger: match inst.get("lightGroupStateDefaultStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_group_state_auxiliary_play_trigger: match inst.get("lightGroupStateAuxiliaryPlayTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_group_state_auxiliary_stop_trigger: match inst.get("lightGroupStateAuxiliaryStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_group_state_emergency_play_trigger: match inst.get("lightGroupStateEmergencyPlayTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_group_state_emergency_stop_trigger: match inst.get("lightGroupStateEmergencyStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_group_state_cinematic_play_trigger: match inst.get("lightGroupStateCinematicPlayTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_group_state_cinematic_stop_trigger: match inst.get("lightGroupStateCinematicStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            luminance_rtpc: match inst.get("luminanceRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            attenuation_scale: inst.get_f32("attenuationScale").unwrap_or_default(),
            volume_db: inst.get_f32("volume_db").unwrap_or_default(),
        }
    }
}

/// DCB type: `LightBoxComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct LightBoxComponentParams {
}

impl Pooled for LightBoxComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_lights.light_box_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_lights.light_box_component_params }
}

impl<'a> Extract<'a> for LightBoxComponentParams {
    const TYPE_NAME: &'static str = "LightBoxComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

