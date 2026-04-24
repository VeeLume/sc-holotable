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

/// Pool storage for the `vfx` feature.
#[derive(Default)]
pub struct VfxPools {
    pub atmospheric_flight_effects: Vec<Option<AtmosphericFlightEffects>>,
    pub trail_fading_settings: Vec<Option<TrailFadingSettings>>,
    pub reverse_trails_setting: Vec<Option<ReverseTrailsSetting>>,
    pub global_engine_trails_setting: Vec<Option<GlobalEngineTrailsSetting>>,
    pub global_atmospheric_heating_settings: Vec<Option<GlobalAtmosphericHeatingSettings>>,
    pub global_aerodynamic_trail_settings: Vec<Option<GlobalAerodynamicTrailSettings>>,
    pub global_environment_effect_settings: Vec<Option<GlobalEnvironmentEffectSettings>>,
    pub damage_map_damage_types: Vec<Option<DamageMapDamageTypes>>,
    pub damage_map_damage_form: Vec<Option<DamageMapDamageForm>>,
    pub damage_map_global_params: Vec<Option<DamageMapGlobalParams>>,
    pub dematerialize_animation: Vec<Option<DematerializeAnimation>>,
    pub global_gas_cloud_vdb_gameplay_params: Vec<Option<GlobalGasCloudVDB_GameplayParams>>,
    pub global_gas_cloud_vdbparams: Vec<Option<GlobalGasCloudVDBParams>>,
    pub planet_effect_loddistance: Vec<Option<PlanetEffectLODDistance>>,
    pub global_fog_volume: Vec<Option<GlobalFogVolume>>,
    pub planet_effect_lod: Vec<Option<PlanetEffectLOD>>,
    pub quantum_drive_effect_settings: Vec<Option<QuantumDriveEffectSettings>>,
    pub screen_effects_library: Vec<Option<ScreenEffects_Library>>,
    pub screen_effects_effect: Vec<Option<ScreenEffects_Effect>>,
    pub screen_effects_pattern: Vec<Option<ScreenEffects_Pattern>>,
    pub screen_effects_param: Vec<Option<ScreenEffects_Param>>,
    pub screen_effects_param_strength_behavior: Vec<Option<ScreenEffects_ParamStrengthBehavior>>,
    pub screen_effects_pattern_linear: Vec<Option<ScreenEffects_Pattern_Linear>>,
    pub screen_effects_param_value_float: Vec<Option<ScreenEffects_ParamValue_Float>>,
    pub screen_effects_param_strength_behavior_range_enable:
        Vec<Option<ScreenEffects_ParamStrengthBehavior_RangeEnable>>,
    pub screen_effects_param_strength_behavior_range_fade:
        Vec<Option<ScreenEffects_ParamStrengthBehavior_RangeFade>>,
    pub screen_effects_debug: Vec<Option<ScreenEffects_Debug>>,
    pub screen_effects_debug_effect: Vec<Option<ScreenEffects_DebugEffect>>,
    pub screen_effects_debug_param: Vec<Option<ScreenEffects_DebugParam>>,
    pub video_comms_shader_params: Vec<Option<VideoCommsShaderParams>>,
    pub water_interaction_effect_params: Vec<Option<WaterInteractionEffectParams>>,
    pub water_effects_global_params: Vec<Option<WaterEffectsGlobalParams>>,
}
