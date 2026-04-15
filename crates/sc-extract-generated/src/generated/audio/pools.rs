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

/// Pool storage for the `audio` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AudioPools {
    #[serde(default)]
    pub audio_breath_style_condition_list: Vec<Option<AudioBreathStyleConditionList>>,
    #[serde(default)]
    pub entity_audio_controller_type_params: Vec<Option<EntityAudioControllerTypeParams>>,
    #[serde(default)]
    pub entity_audio_controller_type_management_params: Vec<Option<EntityAudioControllerTypeManagementParams>>,
    #[serde(default)]
    pub entity_audio_controller_manager_params: Vec<Option<EntityAudioControllerManagerParams>>,
    #[serde(default)]
    pub audio_whitelist: Vec<Option<AudioWhitelist>>,
    #[serde(default)]
    pub audio_environment: Vec<Option<AudioEnvironment>>,
    #[serde(default)]
    pub audio_one_shot_manager_budget_entry: Vec<Option<AudioOneShotManagerBudgetEntry>>,
    #[serde(default)]
    pub audio_budget_definition: Vec<Option<AudioBudgetDefinition>>,
    #[serde(default)]
    pub audio_game_context_globals: Vec<Option<AudioGameContextGlobals>>,
    #[serde(default)]
    pub audio_game_context: Vec<Option<AudioGameContext>>,
    #[serde(default)]
    pub audio_game_context_setup: Vec<Option<AudioGameContextSetup>>,
    #[serde(default)]
    pub surface_audio_properties_definition: Vec<Option<SurfaceAudioPropertiesDefinition>>,
    #[serde(default)]
    pub surface_audio_properties: Vec<Option<SurfaceAudioProperties>>,
    #[serde(default)]
    pub global_audio_settings: Vec<Option<GlobalAudioSettings>>,
    #[serde(default)]
    pub audio_tag_action: Vec<Option<AudioTagAction>>,
    #[serde(default)]
    pub audio_tag_action_list: Vec<Option<AudioTagActionList>>,
    #[serde(default)]
    pub audio_value_output_behaviour: Vec<Option<AudioValueOutputBehaviour>>,
    #[serde(default)]
    pub audio_value_output_behaviour_camera: Vec<Option<AudioValueOutputBehaviourCamera>>,
    #[serde(default)]
    pub audio_value_output_behaviour_microphone: Vec<Option<AudioValueOutputBehaviourMicrophone>>,
    #[serde(default)]
    pub audio_value_output: Vec<Option<AudioValueOutput>>,
    #[serde(default)]
    pub audio_value_output_setup: Vec<Option<AudioValueOutputSetup>>,
    #[serde(default)]
    pub queueing_behaviour: Vec<Option<QueueingBehaviour>>,
    #[serde(default)]
    pub cockpit_response_variation: Vec<Option<CockpitResponseVariation>>,
    #[serde(default)]
    pub cockpit_response: Vec<Option<CockpitResponse>>,
    #[serde(default)]
    pub cockpit_responses: Vec<Option<CockpitResponses>>,
    #[serde(default)]
    pub cockpit_rule_base: Vec<Option<CockpitRuleBase>>,
    #[serde(default)]
    pub cockpit_rule_float: Vec<Option<CockpitRuleFloat>>,
    #[serde(default)]
    pub cockpit_rule_string: Vec<Option<CockpitRuleString>>,
    #[serde(default)]
    pub audio_allegiance_switches: Vec<Option<AudioAllegianceSwitches>>,
    #[serde(default)]
    pub audio_signal_list: Vec<Option<AudioSignalList>>,
    #[serde(default)]
    pub tag_to_audio_rtpc_list: Vec<Option<TagToAudioRtpcList>>,
    #[serde(default)]
    pub audio_signal_rtpc: Vec<Option<AudioSignalRtpc>>,
    #[serde(default)]
    pub audio_signal: Vec<Option<AudioSignal>>,
    #[serde(default)]
    pub tag_to_audio_rtpc: Vec<Option<TagToAudioRtpc>>,
    #[serde(default)]
    pub gpuparticle_audio_list: Vec<Option<GPUParticleAudioList>>,
    #[serde(default)]
    pub comms_audio_effect: Vec<Option<CommsAudioEffect>>,
    #[serde(default)]
    pub ship_computer_preset_list: Vec<Option<ShipComputerPresetList>>,
    #[serde(default)]
    pub ship_computer_preset: Vec<Option<ShipComputerPreset>>,
    #[serde(default)]
    pub video_comms_audio_params: Vec<Option<VideoCommsAudioParams>>,
}
