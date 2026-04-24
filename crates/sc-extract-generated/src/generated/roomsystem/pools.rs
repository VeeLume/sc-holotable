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

/// Pool storage for the `roomsystem` feature.
#[derive(Default)]
pub struct RoomsystemPools {
    pub fire_hazard_surface_properties: Vec<Option<FireHazardSurfaceProperties>>,
    pub fire_hazard_fire_properties: Vec<Option<FireHazardFireProperties>>,
    pub fire_hazard_afterglow_properties: Vec<Option<FireHazardAfterglowProperties>>,
    pub fire_hazard_permanent_effects: Vec<Option<FireHazardPermanentEffects>>,
    pub fire_hazard_spawn_params: Vec<Option<FireHazardSpawnParams>>,
    pub fire_hazard_fog_noise_params: Vec<Option<FireHazardFogNoiseParams>>,
    pub fire_hazard_fog_params: Vec<Option<FireHazardFogParams>>,
    pub fire_hazard_global_params: Vec<Option<FireHazardGlobalParams>>,
    pub fire_hazard_global_update: Vec<Option<FireHazardGlobalUpdate>>,
    pub fire_hazard_global_ignition: Vec<Option<FireHazardGlobalIgnition>>,
    pub fire_hazard_global_propagation: Vec<Option<FireHazardGlobalPropagation>>,
    pub fire_hazard_global_smoke_params: Vec<Option<FireHazardGlobalSmokeParams>>,
    pub fire_hazard_global_damage_to_health_params:
        Vec<Option<FireHazardGlobalDamageToHealthParams>>,
    pub fire_hazard_global_extinguishing: Vec<Option<FireHazardGlobalExtinguishing>>,
    pub fire_hazard_global_default_effects: Vec<Option<FireHazardGlobalDefaultEffects>>,
    pub fire_hazard_global_light_params: Vec<Option<FireHazardGlobalLightParams>>,
    pub fire_hazard_global_room_connector_params: Vec<Option<FireHazardGlobalRoomConnectorParams>>,
    pub lightning_behavior: Vec<Option<LightningBehavior>>,
    pub lightning_behavior_effect: Vec<Option<LightningBehavior_Effect>>,
    pub lightning_target_mode: Vec<Option<LightningTargetMode>>,
    pub apparent_temperature_params: Vec<Option<ApparentTemperatureParams>>,
    pub global_gas_params: Vec<Option<GlobalGasParams>>,
    pub global_room_state_params: Vec<Option<GlobalRoomStateParams>>,
    pub electrical_state_template_internal: Vec<Option<ElectricalStateTemplateInternal>>,
    pub electrical_state_template: Vec<Option<ElectricalStateTemplate>>,
    pub electrical_calculation_property_range: Vec<Option<ElectricalCalculationPropertyRange>>,
    pub behavior_electrical_vehicle_effect_params:
        Vec<Option<Behavior_ElectricalVehicleEffectParams>>,
    pub electrical_behavior: Vec<Option<ElectricalBehavior>>,
    pub radiation_behavior_asteroid_design_curve_surface_radiation_params:
        Vec<Option<RadiationBehavior_AsteroidDesignCurveSurfaceRadiationParams>>,
    pub radiation_behavior_asteroid_inverse_square_surface_radiation_params:
        Vec<Option<RadiationBehavior_AsteroidInverseSquareSurfaceRadiationParams>>,
    pub weather_effects_atmosphere_property: Vec<Option<WeatherEffects_Atmosphere_Property>>,
    pub weather_effects_atmosphere_multi_property_value:
        Vec<Option<WeatherEffects_Atmosphere_MultiPropertyValue>>,
    pub weather_effects_atmosphere_gas_cloud_optical_density:
        Vec<Option<WeatherEffects_Atmosphere_GasCloudOpticalDensity>>,
}
