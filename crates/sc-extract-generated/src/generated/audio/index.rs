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
use crate::Handle;
use std::collections::HashMap;
use svarog_common::CigGuid;

/// Record index for the `audio` feature.
#[derive(Default)]
pub struct AudioIndex {
    pub audio_breath_style_condition_list: HashMap<CigGuid, Handle<AudioBreathStyleConditionList>>,
    pub entity_audio_controller_type_params:
        HashMap<CigGuid, Handle<EntityAudioControllerTypeParams>>,
    pub entity_audio_controller_manager_params:
        HashMap<CigGuid, Handle<EntityAudioControllerManagerParams>>,
    pub audio_whitelist: HashMap<CigGuid, Handle<AudioWhitelist>>,
    pub audio_environment: HashMap<CigGuid, Handle<AudioEnvironment>>,
    pub audio_budget_definition: HashMap<CigGuid, Handle<AudioBudgetDefinition>>,
    pub audio_game_context_globals: HashMap<CigGuid, Handle<AudioGameContextGlobals>>,
    pub audio_game_context_setup: HashMap<CigGuid, Handle<AudioGameContextSetup>>,
    pub surface_audio_properties_definition:
        HashMap<CigGuid, Handle<SurfaceAudioPropertiesDefinition>>,
    pub global_audio_settings: HashMap<CigGuid, Handle<GlobalAudioSettings>>,
    pub audio_tag_action_list: HashMap<CigGuid, Handle<AudioTagActionList>>,
    pub audio_value_output_setup: HashMap<CigGuid, Handle<AudioValueOutputSetup>>,
    pub cockpit_response: HashMap<CigGuid, Handle<CockpitResponse>>,
    pub cockpit_responses: HashMap<CigGuid, Handle<CockpitResponses>>,
    pub audio_allegiance_switches: HashMap<CigGuid, Handle<AudioAllegianceSwitches>>,
    pub audio_signal_list: HashMap<CigGuid, Handle<AudioSignalList>>,
    pub tag_to_audio_rtpc_list: HashMap<CigGuid, Handle<TagToAudioRtpcList>>,
    pub gpuparticle_audio_list: HashMap<CigGuid, Handle<GPUParticleAudioList>>,
    pub comms_audio_effect: HashMap<CigGuid, Handle<CommsAudioEffect>>,
    pub ship_computer_preset_list: HashMap<CigGuid, Handle<ShipComputerPresetList>>,
    pub ship_computer_preset: HashMap<CigGuid, Handle<ShipComputerPreset>>,
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
