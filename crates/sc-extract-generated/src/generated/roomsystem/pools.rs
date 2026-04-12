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

/// Pool storage for the `roomsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomsystemPools {
    #[serde(default)]
    pub sentity_effect_system_attachment: Vec<Option<SEntityEffectSystem_Attachment>>,
    #[serde(default)]
    pub sentity_effect_system_property_modifier: Vec<Option<SEntityEffectSystem_PropertyModifier>>,
    #[serde(default)]
    pub sentity_effect_system_particle_category: Vec<Option<SEntityEffectSystem_ParticleCategory>>,
    #[serde(default)]
    pub sentity_effect_system_particle_tag_effect: Vec<Option<SEntityEffectSystem_ParticleTagEffect>>,
    #[serde(default)]
    pub sentity_effect_system_particle_trigger_effect: Vec<Option<SEntityEffectSystem_ParticleTriggerEffect>>,
    #[serde(default)]
    pub sentity_effect_system_particle_property_link: Vec<Option<SEntityEffectSystem_ParticlePropertyLink>>,
    #[serde(default)]
    pub fire_hazard_surface_properties: Vec<Option<FireHazardSurfaceProperties>>,
    #[serde(default)]
    pub fire_hazard_fire_properties: Vec<Option<FireHazardFireProperties>>,
    #[serde(default)]
    pub fire_hazard_afterglow_properties: Vec<Option<FireHazardAfterglowProperties>>,
    #[serde(default)]
    pub fire_hazard_permanent_effects: Vec<Option<FireHazardPermanentEffects>>,
    #[serde(default)]
    pub fire_hazard_spawn_params: Vec<Option<FireHazardSpawnParams>>,
    #[serde(default)]
    pub fire_hazard_fog_noise_params: Vec<Option<FireHazardFogNoiseParams>>,
    #[serde(default)]
    pub fire_hazard_fog_params: Vec<Option<FireHazardFogParams>>,
    #[serde(default)]
    pub fire_hazard_global_params: Vec<Option<FireHazardGlobalParams>>,
    #[serde(default)]
    pub fire_hazard_global_update: Vec<Option<FireHazardGlobalUpdate>>,
    #[serde(default)]
    pub fire_hazard_global_ignition: Vec<Option<FireHazardGlobalIgnition>>,
    #[serde(default)]
    pub fire_hazard_global_propagation: Vec<Option<FireHazardGlobalPropagation>>,
    #[serde(default)]
    pub fire_hazard_global_smoke_params: Vec<Option<FireHazardGlobalSmokeParams>>,
    #[serde(default)]
    pub fire_hazard_global_damage_to_health_params: Vec<Option<FireHazardGlobalDamageToHealthParams>>,
    #[serde(default)]
    pub fire_hazard_global_extinguishing: Vec<Option<FireHazardGlobalExtinguishing>>,
    #[serde(default)]
    pub fire_hazard_global_default_effects: Vec<Option<FireHazardGlobalDefaultEffects>>,
    #[serde(default)]
    pub fire_hazard_global_light_params: Vec<Option<FireHazardGlobalLightParams>>,
    #[serde(default)]
    pub fire_hazard_global_room_connector_params: Vec<Option<FireHazardGlobalRoomConnectorParams>>,
    #[serde(default)]
    pub lightning_behavior: Vec<Option<LightningBehavior>>,
    #[serde(default)]
    pub lightning_behavior_effect: Vec<Option<LightningBehavior_Effect>>,
    #[serde(default)]
    pub lightning_target_mode: Vec<Option<LightningTargetMode>>,
    #[serde(default)]
    pub lightning_strike_audio: Vec<Option<LightningStrikeAudio>>,
    #[serde(default)]
    pub gas_params: Vec<Option<GasParams>>,
    #[serde(default)]
    pub sgas_atmosphere_entry_params: Vec<Option<SGasAtmosphereEntryParams>>,
    #[serde(default)]
    pub satmospheric_composition_params: Vec<Option<SAtmosphericCompositionParams>>,
    #[serde(default)]
    pub atmospheric_composition_template: Vec<Option<AtmosphericCompositionTemplate>>,
    #[serde(default)]
    pub apparent_temperature_params: Vec<Option<ApparentTemperatureParams>>,
    #[serde(default)]
    pub global_gas_params: Vec<Option<GlobalGasParams>>,
    #[serde(default)]
    pub global_room_state_params: Vec<Option<GlobalRoomStateParams>>,
    #[serde(default)]
    pub behavior_vehicle_effect_params: Vec<Option<Behavior_VehicleEffectParams>>,
    #[serde(default)]
    pub behavior_custom_vehicle_effects_preset: Vec<Option<Behavior_CustomVehicleEffectsPreset>>,
    #[serde(default)]
    pub asteroid_state_template_internal: Vec<Option<AsteroidStateTemplateInternal>>,
    #[serde(default)]
    pub asteroid_state_template: Vec<Option<AsteroidStateTemplate>>,
    #[serde(default)]
    pub asteroid_behavior: Vec<Option<AsteroidBehavior>>,
    #[serde(default)]
    pub asteroid_behavior_weather_params: Vec<Option<AsteroidBehavior_WeatherParams>>,
    #[serde(default)]
    pub atmosphere_state_template_internal: Vec<Option<AtmosphereStateTemplateInternal>>,
    #[serde(default)]
    pub atmosphere_state_template: Vec<Option<AtmosphereStateTemplate>>,
    #[serde(default)]
    pub atmosphere_state_pressure_template: Vec<Option<AtmosphereStatePressureTemplate>>,
    #[serde(default)]
    pub atmosphere_state_temperature_template: Vec<Option<AtmosphereStateTemperatureTemplate>>,
    #[serde(default)]
    pub atmosphere_state_humidity_template: Vec<Option<AtmosphereStateHumidityTemplate>>,
    #[serde(default)]
    pub aerodynamic_trail_calculation: Vec<Option<AerodynamicTrailCalculation>>,
    #[serde(default)]
    pub behavior_atmosphere_vehicle_effect_params: Vec<Option<Behavior_AtmosphereVehicleEffectParams>>,
    #[serde(default)]
    pub atmosphere_behavior: Vec<Option<AtmosphereBehavior>>,
    #[serde(default)]
    pub atmosphere_behavior_turbulence_params: Vec<Option<AtmosphereBehavior_TurbulenceParams>>,
    #[serde(default)]
    pub atmosphere_behavior_weather_params: Vec<Option<AtmosphereBehavior_WeatherParams>>,
    #[serde(default)]
    pub electrical_state_template_internal: Vec<Option<ElectricalStateTemplateInternal>>,
    #[serde(default)]
    pub electrical_state_template: Vec<Option<ElectricalStateTemplate>>,
    #[serde(default)]
    pub electrical_calculation_property_range: Vec<Option<ElectricalCalculationPropertyRange>>,
    #[serde(default)]
    pub behavior_electrical_vehicle_effect_params: Vec<Option<Behavior_ElectricalVehicleEffectParams>>,
    #[serde(default)]
    pub electrical_behavior: Vec<Option<ElectricalBehavior>>,
    #[serde(default)]
    pub radiation_state_property_params: Vec<Option<RadiationStatePropertyParams>>,
    #[serde(default)]
    pub radiation_state_template_internal: Vec<Option<RadiationStateTemplateInternal>>,
    #[serde(default)]
    pub radiation_state_template: Vec<Option<RadiationStateTemplate>>,
    #[serde(default)]
    pub radiation_behavior: Vec<Option<RadiationBehavior>>,
    #[serde(default)]
    pub radiation_behavior_surface_radiation_params: Vec<Option<RadiationBehavior_SurfaceRadiationParams>>,
    #[serde(default)]
    pub weather_effects_space_loop_effect: Vec<Option<WeatherEffects_SpaceLoopEffect>>,
    #[serde(default)]
    pub weather_effects_asteroid: Vec<Option<WeatherEffects_Asteroid>>,
    #[serde(default)]
    pub weather_effects_atmosphere: Vec<Option<WeatherEffects_Atmosphere>>,
}
