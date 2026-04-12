// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `audio` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AudioPools {
    #[serde(default)]
    pub saudio_breath_parameters: Vec<Option<SAudioBreathParameters>>,
    #[serde(default)]
    pub misted_breath_params: Vec<Option<MistedBreathParams>>,
    #[serde(default)]
    pub stance_breath_modifier: Vec<Option<StanceBreathModifier>>,
    #[serde(default)]
    pub hold_exhale_duration: Vec<Option<HoldExhaleDuration>>,
    #[serde(default)]
    pub breath_duration_params: Vec<Option<BreathDurationParams>>,
    #[serde(default)]
    pub audio_breath_style_condition: Vec<Option<AudioBreathStyleCondition>>,
    #[serde(default)]
    pub audio_breath_style_condition_list: Vec<Option<AudioBreathStyleConditionList>>,
    #[serde(default)]
    pub audio_breath_style_transition_node: Vec<Option<AudioBreathStyleTransitionNode>>,
    #[serde(default)]
    pub actor_breathing_style_startup: Vec<Option<ActorBreathingStyleStartup>>,
    #[serde(default)]
    pub audio_breath_style: Vec<Option<AudioBreathStyle>>,
    #[serde(default)]
    pub audio_breath_style_base_node: Vec<Option<AudioBreathStyleBaseNode>>,
    #[serde(default)]
    pub audio_breath_style_node: Vec<Option<AudioBreathStyleNode>>,
    #[serde(default)]
    pub audio_breath_style_suite: Vec<Option<AudioBreathStyleSuite>>,
    #[serde(default)]
    pub audio_breath_definition: Vec<Option<AudioBreathDefinition>>,
    #[serde(default)]
    pub audio_breath_interrupt: Vec<Option<AudioBreathInterrupt>>,
    #[serde(default)]
    pub breathing_trigger_def: Vec<Option<BreathingTriggerDef>>,
    #[serde(default)]
    pub tag_trigger: Vec<Option<TagTrigger>>,
    #[serde(default)]
    pub sentity_audio_controller_params: Vec<Option<SEntityAudioControllerParams>>,
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
    pub audio_allegiance_switches: Vec<Option<AudioAllegianceSwitches>>,
    #[serde(default)]
    pub audio_environment_feedback_zone_process: Vec<Option<AudioEnvironmentFeedbackZoneProcess>>,
    #[serde(default)]
    pub audio_environment_feedback_zone_setup: Vec<Option<AudioEnvironmentFeedbackZoneSetup>>,
    #[serde(default)]
    pub audio_environment_movement_rtpc_behavior: Vec<Option<AudioEnvironmentMovementRtpcBehavior>>,
    #[serde(default)]
    pub audio_environment_feedback_tag_and_event: Vec<Option<AudioEnvironmentFeedbackTagAndEvent>>,
    #[serde(default)]
    pub audio_environment_feedback_point_def: Vec<Option<AudioEnvironmentFeedbackPointDef>>,
    #[serde(default)]
    pub audio_hit_listener_trigger: Vec<Option<AudioHitListenerTrigger>>,
    #[serde(default)]
    pub audio_hit_type_definition: Vec<Option<AudioHitTypeDefinition>>,
    #[serde(default)]
    pub audio_hit_listener_definition: Vec<Option<AudioHitListenerDefinition>>,
    #[serde(default)]
    pub wheel_audio_params: Vec<Option<WheelAudioParams>>,
    #[serde(default)]
    pub wheel_audio_surface_mapping: Vec<Option<WheelAudioSurfaceMapping>>,
    #[serde(default)]
    pub wheel_audio_surface_map: Vec<Option<WheelAudioSurfaceMap>>,
    #[serde(default)]
    pub procedural_planet_audio_tag_and_event: Vec<Option<ProceduralPlanetAudioTagAndEvent>>,
    #[serde(default)]
    pub procedural_planet_audio_tag_and_rtpc: Vec<Option<ProceduralPlanetAudioTagAndRtpc>>,
    #[serde(default)]
    pub procedural_planet_audio_tag_and_events_def: Vec<Option<ProceduralPlanetAudioTagAndEventsDef>>,
    #[serde(default)]
    pub procedural_planet_audio_algorithm: Vec<Option<ProceduralPlanetAudioAlgorithm>>,
    #[serde(default)]
    pub procedural_planet_audio_disturbance_def: Vec<Option<ProceduralPlanetAudioDisturbanceDef>>,
    #[serde(default)]
    pub procedural_planet_audio_disturbance_list: Vec<Option<ProceduralPlanetAudioDisturbanceList>>,
    #[serde(default)]
    pub procedural_planet_audio_entry: Vec<Option<ProceduralPlanetAudioEntry>>,
    #[serde(default)]
    pub procedural_planet_audio_data: Vec<Option<ProceduralPlanetAudioData>>,
    #[serde(default)]
    pub procedural_planet_audio_river_data: Vec<Option<ProceduralPlanetAudioRiverData>>,
    #[serde(default)]
    pub planet_ocean_audio_checkpoint: Vec<Option<PlanetOceanAudioCheckpoint>>,
    #[serde(default)]
    pub planet_ocean_depth_assignment: Vec<Option<PlanetOceanDepthAssignment>>,
    #[serde(default)]
    pub planet_ocean_audio_data: Vec<Option<PlanetOceanAudioData>>,
    #[serde(default)]
    pub vibration_audio_entry: Vec<Option<VibrationAudioEntry>>,
    #[serde(default)]
    pub vibration_audio_point_def: Vec<Option<VibrationAudioPointDef>>,
    #[serde(default)]
    pub audio_signal_list: Vec<Option<AudioSignalList>>,
    #[serde(default)]
    pub tag_to_audio_rtpc_list: Vec<Option<TagToAudioRtpcList>>,
    #[serde(default)]
    pub audio_signal_rtpc: Vec<Option<AudioSignalRtpc>>,
    #[serde(default)]
    pub audio_signal: Vec<Option<AudioSignal>>,
    #[serde(default)]
    pub audio_rtpc_with_default: Vec<Option<AudioRtpcWithDefault>>,
    #[serde(default)]
    pub audio_switch: Vec<Option<AudioSwitch>>,
    #[serde(default)]
    pub srtpc_behaviour: Vec<Option<SRtpcBehaviour>>,
    #[serde(default)]
    pub audio_rtpc_with_behaviour: Vec<Option<AudioRtpcWithBehaviour>>,
    #[serde(default)]
    pub tag_to_audio_rtpc: Vec<Option<TagToAudioRtpc>>,
    #[serde(default)]
    pub gpuparticle_audio_list: Vec<Option<GPUParticleAudioList>>,
    #[serde(default)]
    pub comms_audio_effect: Vec<Option<CommsAudioEffect>>,
    #[serde(default)]
    pub ship_computer_definition: Vec<Option<ShipComputerDefinition>>,
    #[serde(default)]
    pub sscsignature_system_audio_modifier: Vec<Option<SSCSignatureSystemAudioModifier>>,
    #[serde(default)]
    pub sscsignature_system_audio_sub_rule: Vec<Option<SSCSignatureSystemAudioSubRule>>,
    #[serde(default)]
    pub sscsignature_system_audio_rule: Vec<Option<SSCSignatureSystemAudioRule>>,
    #[serde(default)]
    pub sscsignature_system_audio_ruleset: Vec<Option<SSCSignatureSystemAudioRuleset>>,
    #[serde(default)]
    pub ship_computer_preset_list: Vec<Option<ShipComputerPresetList>>,
    #[serde(default)]
    pub ship_computer_preset: Vec<Option<ShipComputerPreset>>,
    #[serde(default)]
    pub transit_station_announcements: Vec<Option<TransitStationAnnouncements>>,
    #[serde(default)]
    pub transit_station_announcement: Vec<Option<TransitStationAnnouncement>>,
    #[serde(default)]
    pub uiaudio_event: Vec<Option<UIAudioEvent>>,
    #[serde(default)]
    pub uiaudio_parameter: Vec<Option<UIAudioParameter>>,
    #[serde(default)]
    pub uidialogue_event: Vec<Option<UIDialogueEvent>>,
    #[serde(default)]
    pub uiaudio_definition: Vec<Option<UIAudioDefinition>>,
    #[serde(default)]
    pub video_comms_audio_params: Vec<Option<VideoCommsAudioParams>>,
}
