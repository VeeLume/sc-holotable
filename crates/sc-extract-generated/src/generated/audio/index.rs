// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `audio` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AudioIndex {
    #[serde(default)]
    pub audio_breath_style_condition: HashMap<CigGuid, Handle<AudioBreathStyleCondition>>,
    #[serde(default)]
    pub audio_breath_style_condition_list: HashMap<CigGuid, Handle<AudioBreathStyleConditionList>>,
    #[serde(default)]
    pub audio_breath_style: HashMap<CigGuid, Handle<AudioBreathStyle>>,
    #[serde(default)]
    pub audio_breath_style_suite: HashMap<CigGuid, Handle<AudioBreathStyleSuite>>,
    #[serde(default)]
    pub audio_breath_definition: HashMap<CigGuid, Handle<AudioBreathDefinition>>,
    #[serde(default)]
    pub audio_breath_interrupt: HashMap<CigGuid, Handle<AudioBreathInterrupt>>,
    #[serde(default)]
    pub breathing_trigger_def: HashMap<CigGuid, Handle<BreathingTriggerDef>>,
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
    pub audio_environment_feedback_zone_setup: HashMap<CigGuid, Handle<AudioEnvironmentFeedbackZoneSetup>>,
    #[serde(default)]
    pub audio_environment_feedback_point_def: HashMap<CigGuid, Handle<AudioEnvironmentFeedbackPointDef>>,
    #[serde(default)]
    pub audio_hit_listener_definition: HashMap<CigGuid, Handle<AudioHitListenerDefinition>>,
    #[serde(default)]
    pub wheel_audio_surface_map: HashMap<CigGuid, Handle<WheelAudioSurfaceMap>>,
    #[serde(default)]
    pub procedural_planet_audio_tag_and_events_def: HashMap<CigGuid, Handle<ProceduralPlanetAudioTagAndEventsDef>>,
    #[serde(default)]
    pub procedural_planet_audio_data: HashMap<CigGuid, Handle<ProceduralPlanetAudioData>>,
    #[serde(default)]
    pub procedural_planet_audio_river_data: HashMap<CigGuid, Handle<ProceduralPlanetAudioRiverData>>,
    #[serde(default)]
    pub planet_ocean_audio_data: HashMap<CigGuid, Handle<PlanetOceanAudioData>>,
    #[serde(default)]
    pub vibration_audio_point_def: HashMap<CigGuid, Handle<VibrationAudioPointDef>>,
    #[serde(default)]
    pub audio_signal_list: HashMap<CigGuid, Handle<AudioSignalList>>,
    #[serde(default)]
    pub tag_to_audio_rtpc_list: HashMap<CigGuid, Handle<TagToAudioRtpcList>>,
    #[serde(default)]
    pub gpuparticle_audio_list: HashMap<CigGuid, Handle<GPUParticleAudioList>>,
    #[serde(default)]
    pub comms_audio_effect: HashMap<CigGuid, Handle<CommsAudioEffect>>,
    #[serde(default)]
    pub ship_computer_definition: HashMap<CigGuid, Handle<ShipComputerDefinition>>,
    #[serde(default)]
    pub sscsignature_system_audio_ruleset: HashMap<CigGuid, Handle<SSCSignatureSystemAudioRuleset>>,
    #[serde(default)]
    pub ship_computer_preset_list: HashMap<CigGuid, Handle<ShipComputerPresetList>>,
    #[serde(default)]
    pub ship_computer_preset: HashMap<CigGuid, Handle<ShipComputerPreset>>,
    #[serde(default)]
    pub transit_station_announcements: HashMap<CigGuid, Handle<TransitStationAnnouncements>>,
    #[serde(default)]
    pub uiaudio_definition: HashMap<CigGuid, Handle<UIAudioDefinition>>,
    #[serde(default)]
    pub video_comms_audio_params: HashMap<CigGuid, Handle<VideoCommsAudioParams>>,
}

impl AudioIndex {
    pub fn len(&self) -> usize {
        self.audio_breath_style_condition.len()
            + self.audio_breath_style_condition_list.len()
            + self.audio_breath_style.len()
            + self.audio_breath_style_suite.len()
            + self.audio_breath_definition.len()
            + self.audio_breath_interrupt.len()
            + self.breathing_trigger_def.len()
            + self.entity_audio_controller_type_params.len()
            + self.entity_audio_controller_manager_params.len()
            + self.audio_whitelist.len()
            + self.audio_environment.len()
            + self.audio_budget_definition.len()
            + self.audio_game_context_globals.len()
            + self.audio_game_context_setup.len()
            + self.surface_audio_properties_definition.len()
            + self.global_audio_settings.len()
            + self.audio_tag_action_list.len()
            + self.audio_value_output_setup.len()
            + self.cockpit_response.len()
            + self.cockpit_responses.len()
            + self.audio_allegiance_switches.len()
            + self.audio_environment_feedback_zone_setup.len()
            + self.audio_environment_feedback_point_def.len()
            + self.audio_hit_listener_definition.len()
            + self.wheel_audio_surface_map.len()
            + self.procedural_planet_audio_tag_and_events_def.len()
            + self.procedural_planet_audio_data.len()
            + self.procedural_planet_audio_river_data.len()
            + self.planet_ocean_audio_data.len()
            + self.vibration_audio_point_def.len()
            + self.audio_signal_list.len()
            + self.tag_to_audio_rtpc_list.len()
            + self.gpuparticle_audio_list.len()
            + self.comms_audio_effect.len()
            + self.ship_computer_definition.len()
            + self.sscsignature_system_audio_ruleset.len()
            + self.ship_computer_preset_list.len()
            + self.ship_computer_preset.len()
            + self.transit_station_announcements.len()
            + self.uiaudio_definition.len()
            + self.video_comms_audio_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
