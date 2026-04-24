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

/// Pool storage for the `audio` feature.
#[derive(Default)]
pub struct AudioPools {
    pub audio_breath_style_condition_list: Vec<Option<AudioBreathStyleConditionList>>,
    pub entity_audio_controller_type_params: Vec<Option<EntityAudioControllerTypeParams>>,
    pub entity_audio_controller_type_management_params: Vec<Option<EntityAudioControllerTypeManagementParams>>,
    pub entity_audio_controller_manager_params: Vec<Option<EntityAudioControllerManagerParams>>,
    pub audio_whitelist: Vec<Option<AudioWhitelist>>,
    pub audio_environment: Vec<Option<AudioEnvironment>>,
    pub audio_one_shot_manager_budget_entry: Vec<Option<AudioOneShotManagerBudgetEntry>>,
    pub audio_budget_definition: Vec<Option<AudioBudgetDefinition>>,
    pub audio_game_context_globals: Vec<Option<AudioGameContextGlobals>>,
    pub audio_game_context: Vec<Option<AudioGameContext>>,
    pub audio_game_context_setup: Vec<Option<AudioGameContextSetup>>,
    pub surface_audio_properties_definition: Vec<Option<SurfaceAudioPropertiesDefinition>>,
    pub surface_audio_properties: Vec<Option<SurfaceAudioProperties>>,
    pub global_audio_settings: Vec<Option<GlobalAudioSettings>>,
    pub audio_tag_action: Vec<Option<AudioTagAction>>,
    pub audio_tag_action_list: Vec<Option<AudioTagActionList>>,
    pub audio_value_output_behaviour: Vec<Option<AudioValueOutputBehaviour>>,
    pub audio_value_output_behaviour_camera: Vec<Option<AudioValueOutputBehaviourCamera>>,
    pub audio_value_output_behaviour_microphone: Vec<Option<AudioValueOutputBehaviourMicrophone>>,
    pub audio_value_output: Vec<Option<AudioValueOutput>>,
    pub audio_value_output_setup: Vec<Option<AudioValueOutputSetup>>,
    pub queueing_behaviour: Vec<Option<QueueingBehaviour>>,
    pub cockpit_response_variation: Vec<Option<CockpitResponseVariation>>,
    pub cockpit_response: Vec<Option<CockpitResponse>>,
    pub cockpit_responses: Vec<Option<CockpitResponses>>,
    pub cockpit_rule_base: Vec<Option<CockpitRuleBase>>,
    pub cockpit_rule_float: Vec<Option<CockpitRuleFloat>>,
    pub cockpit_rule_string: Vec<Option<CockpitRuleString>>,
    pub audio_allegiance_switches: Vec<Option<AudioAllegianceSwitches>>,
    pub audio_signal_list: Vec<Option<AudioSignalList>>,
    pub tag_to_audio_rtpc_list: Vec<Option<TagToAudioRtpcList>>,
    pub audio_signal_rtpc: Vec<Option<AudioSignalRtpc>>,
    pub audio_signal: Vec<Option<AudioSignal>>,
    pub tag_to_audio_rtpc: Vec<Option<TagToAudioRtpc>>,
    pub gpuparticle_audio_list: Vec<Option<GPUParticleAudioList>>,
    pub comms_audio_effect: Vec<Option<CommsAudioEffect>>,
    pub ship_computer_preset_list: Vec<Option<ShipComputerPresetList>>,
    pub ship_computer_preset: Vec<Option<ShipComputerPreset>>,
    pub video_comms_audio_params: Vec<Option<VideoCommsAudioParams>>,
}
