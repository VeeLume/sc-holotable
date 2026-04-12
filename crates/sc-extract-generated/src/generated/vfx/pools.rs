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

/// Pool storage for the `vfx` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VfxPools {
    #[serde(default)]
    pub atmospheric_flight_effects: Vec<Option<AtmosphericFlightEffects>>,
    #[serde(default)]
    pub trail_fading_settings: Vec<Option<TrailFadingSettings>>,
    #[serde(default)]
    pub reverse_trails_setting: Vec<Option<ReverseTrailsSetting>>,
    #[serde(default)]
    pub global_engine_trails_setting: Vec<Option<GlobalEngineTrailsSetting>>,
    #[serde(default)]
    pub global_atmospheric_heating_settings: Vec<Option<GlobalAtmosphericHeatingSettings>>,
    #[serde(default)]
    pub global_aerodynamic_trail_settings: Vec<Option<GlobalAerodynamicTrailSettings>>,
    #[serde(default)]
    pub global_environment_effect_settings: Vec<Option<GlobalEnvironmentEffectSettings>>,
    #[serde(default)]
    pub damage_map_damage_types: Vec<Option<DamageMapDamageTypes>>,
    #[serde(default)]
    pub damage_map_damage_form: Vec<Option<DamageMapDamageForm>>,
    #[serde(default)]
    pub damage_map_global_params: Vec<Option<DamageMapGlobalParams>>,
    #[serde(default)]
    pub dematerialize_animation: Vec<Option<DematerializeAnimation>>,
    #[serde(default)]
    pub global_gas_cloud_vdb_gameplay_params: Vec<Option<GlobalGasCloudVDB_GameplayParams>>,
    #[serde(default)]
    pub global_gas_cloud_vdbparams: Vec<Option<GlobalGasCloudVDBParams>>,
    #[serde(default)]
    pub planet_effect_loddistance: Vec<Option<PlanetEffectLODDistance>>,
    #[serde(default)]
    pub global_fog_volume: Vec<Option<GlobalFogVolume>>,
    #[serde(default)]
    pub planet_effect_lod: Vec<Option<PlanetEffectLOD>>,
    #[serde(default)]
    pub quantum_drive_effect_settings: Vec<Option<QuantumDriveEffectSettings>>,
    #[serde(default)]
    pub screen_effects_library: Vec<Option<ScreenEffects_Library>>,
    #[serde(default)]
    pub screen_effects_effect: Vec<Option<ScreenEffects_Effect>>,
    #[serde(default)]
    pub screen_effects_pattern: Vec<Option<ScreenEffects_Pattern>>,
    #[serde(default)]
    pub screen_effects_param: Vec<Option<ScreenEffects_Param>>,
    #[serde(default)]
    pub screen_effects_param_value: Vec<Option<ScreenEffects_ParamValue>>,
    #[serde(default)]
    pub screen_effects_param_strength_behavior: Vec<Option<ScreenEffects_ParamStrengthBehavior>>,
    #[serde(default)]
    pub screen_effects_debug: Vec<Option<ScreenEffects_Debug>>,
    #[serde(default)]
    pub screen_effects_debug_effect: Vec<Option<ScreenEffects_DebugEffect>>,
    #[serde(default)]
    pub screen_effects_debug_param: Vec<Option<ScreenEffects_DebugParam>>,
    #[serde(default)]
    pub shield_type_params: Vec<Option<ShieldTypeParams>>,
    #[serde(default)]
    pub video_comms_shader_params: Vec<Option<VideoCommsShaderParams>>,
    #[serde(default)]
    pub water_interaction_effect_params: Vec<Option<WaterInteractionEffectParams>>,
    #[serde(default)]
    pub water_effects_global_params: Vec<Option<WaterEffectsGlobalParams>>,
}
