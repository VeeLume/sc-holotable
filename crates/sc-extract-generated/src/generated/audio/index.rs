// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `audio` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AudioIndex {
    #[serde(default)]
    pub audio_breath_style_condition_list: HashMap<CigGuid, Handle<AudioBreathStyleConditionList>>,
    #[serde(default)]
    pub entity_audio_controller_type_params: HashMap<CigGuid, Handle<EntityAudioControllerTypeParams>>,
    #[serde(default)]
    pub entity_audio_controller_manager_params: HashMap<CigGuid, Handle<EntityAudioControllerManagerParams>>,
    #[serde(default)]
    pub audio_whitelist: HashMap<CigGuid, Handle<AudioWhitelist>>,
    #[serde(default)]
    pub audio_environment: HashMap<CigGuid, Handle<AudioEnvironment>>,
    #[serde(default)]
    pub audio_budget_definition: HashMap<CigGuid, Handle<AudioBudgetDefinition>>,
    #[serde(default)]
    pub audio_game_context_globals: HashMap<CigGuid, Handle<AudioGameContextGlobals>>,
    #[serde(default)]
    pub audio_game_context_setup: HashMap<CigGuid, Handle<AudioGameContextSetup>>,
    #[serde(default)]
    pub surface_audio_properties_definition: HashMap<CigGuid, Handle<SurfaceAudioPropertiesDefinition>>,
    #[serde(default)]
    pub global_audio_settings: HashMap<CigGuid, Handle<GlobalAudioSettings>>,
    #[serde(default)]
    pub audio_tag_action_list: HashMap<CigGuid, Handle<AudioTagActionList>>,
    #[serde(default)]
    pub audio_value_output_setup: HashMap<CigGuid, Handle<AudioValueOutputSetup>>,
    #[serde(default)]
    pub cockpit_response: HashMap<CigGuid, Handle<CockpitResponse>>,
    #[serde(default)]
    pub cockpit_responses: HashMap<CigGuid, Handle<CockpitResponses>>,
    #[serde(default)]
    pub audio_allegiance_switches: HashMap<CigGuid, Handle<AudioAllegianceSwitches>>,
    #[serde(default)]
    pub audio_signal_list: HashMap<CigGuid, Handle<AudioSignalList>>,
    #[serde(default)]
    pub tag_to_audio_rtpc_list: HashMap<CigGuid, Handle<TagToAudioRtpcList>>,
    #[serde(default)]
    pub gpuparticle_audio_list: HashMap<CigGuid, Handle<GPUParticleAudioList>>,
    #[serde(default)]
    pub comms_audio_effect: HashMap<CigGuid, Handle<CommsAudioEffect>>,
    #[serde(default)]
    pub ship_computer_preset_list: HashMap<CigGuid, Handle<ShipComputerPresetList>>,
    #[serde(default)]
    pub ship_computer_preset: HashMap<CigGuid, Handle<ShipComputerPreset>>,
    #[serde(default)]
    pub video_comms_audio_params: HashMap<CigGuid, Handle<VideoCommsAudioParams>>,
}

impl AudioIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.audio_breath_style_condition_list.len();
        total += self.entity_audio_controller_type_params.len();
        total += self.entity_audio_controller_manager_params.len();
        total += self.audio_whitelist.len();
        total += self.audio_environment.len();
        total += self.audio_budget_definition.len();
        total += self.audio_game_context_globals.len();
        total += self.audio_game_context_setup.len();
        total += self.surface_audio_properties_definition.len();
        total += self.global_audio_settings.len();
        total += self.audio_tag_action_list.len();
        total += self.audio_value_output_setup.len();
        total += self.cockpit_response.len();
        total += self.cockpit_responses.len();
        total += self.audio_allegiance_switches.len();
        total += self.audio_signal_list.len();
        total += self.tag_to_audio_rtpc_list.len();
        total += self.gpuparticle_audio_list.len();
        total += self.comms_audio_effect.len();
        total += self.ship_computer_preset_list.len();
        total += self.ship_computer_preset.len();
        total += self.video_comms_audio_params.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
