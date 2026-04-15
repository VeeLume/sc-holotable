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

/// Pool storage for the `roomsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomsystemPools {
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
    pub apparent_temperature_params: Vec<Option<ApparentTemperatureParams>>,
    #[serde(default)]
    pub global_gas_params: Vec<Option<GlobalGasParams>>,
    #[serde(default)]
    pub global_room_state_params: Vec<Option<GlobalRoomStateParams>>,
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
    pub radiation_behavior_asteroid_design_curve_surface_radiation_params: Vec<Option<RadiationBehavior_AsteroidDesignCurveSurfaceRadiationParams>>,
    #[serde(default)]
    pub radiation_behavior_asteroid_inverse_square_surface_radiation_params: Vec<Option<RadiationBehavior_AsteroidInverseSquareSurfaceRadiationParams>>,
    #[serde(default)]
    pub weather_effects_atmosphere_property: Vec<Option<WeatherEffects_Atmosphere_Property>>,
    #[serde(default)]
    pub weather_effects_atmosphere_multi_property_value: Vec<Option<WeatherEffects_Atmosphere_MultiPropertyValue>>,
    #[serde(default)]
    pub weather_effects_atmosphere_gas_cloud_optical_density: Vec<Option<WeatherEffects_Atmosphere_GasCloudOpticalDensity>>,
}
