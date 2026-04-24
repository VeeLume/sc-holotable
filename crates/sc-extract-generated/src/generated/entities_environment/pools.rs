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

/// Pool storage for the `entities-environment` feature.
#[derive(Default)]
pub struct EntitiesEnvironmentPools {
    pub planet_navigation_volume_params: Vec<Option<PlanetNavigationVolumeParams>>,
    pub asteroid_field_shape_link_params: Vec<Option<AsteroidFieldShapeLinkParams>>,
    pub sasteroid_cluster_component_params: Vec<Option<SAsteroidClusterComponentParams>>,
    pub sasteroid_ring_component_params: Vec<Option<SAsteroidRingComponentParams>>,
    pub sasteroid_gas_cloud_component_params: Vec<Option<SAsteroidGasCloudComponentParams>>,
    pub sasteroid_shape_component_params: Vec<Option<SAsteroidShapeComponentParams>>,
    pub fog_volume_component_params: Vec<Option<FogVolumeComponentParams>>,
    pub harvestable_provider_params: Vec<Option<HarvestableProviderParams>>,
    pub sentity_component_managed_entity_region_params: Vec<Option<SEntityComponentManagedEntityRegionParams>>,
    pub sentity_component_managed_entity_zone_params: Vec<Option<SEntityComponentManagedEntityZoneParams>>,
    pub procedural_planet_audio_component_params: Vec<Option<ProceduralPlanetAudioComponentParams>>,
    pub planet_ocean_audio_component_params: Vec<Option<PlanetOceanAudioComponentParams>>,
    pub default_sphere_geom: Vec<Option<DefaultSphereGeom>>,
    pub sphere_field_geom: Vec<Option<SphereFieldGeom>>,
    pub box_field_geom: Vec<Option<BoxFieldGeom>>,
    pub cylinder_field_geom: Vec<Option<CylinderFieldGeom>>,
    pub capsule_field_geom: Vec<Option<CapsuleFieldGeom>>,
    pub torus_field_geom: Vec<Option<TorusFieldGeom>>,
    pub mesh_field_geom: Vec<Option<MeshFieldGeom>>,
    pub texture3_dfield_geom: Vec<Option<Texture3DFieldGeom>>,
    pub quantum_obstacle_params: Vec<Option<QuantumObstacleParams>>,
    pub solar_system_component_params: Vec<Option<SolarSystemComponentParams>>,
    pub sentity_component_noisy_smoothing_modification_object_params: Vec<Option<SEntityComponentNoisySmoothingModificationObjectParams>>,
    pub sentity_component_planet_area_params: Vec<Option<SEntityComponentPlanetAreaParams>>,
    pub sentity_component_push_pull_modification_object_params: Vec<Option<SEntityComponentPushPullModificationObjectParams>>,
    pub sentity_component_rectangle_modification_object_params: Vec<Option<SEntityComponentRectangleModificationObjectParams>>,
    pub sentity_component_smoothing_modification_object_params: Vec<Option<SEntityComponentSmoothingModificationObjectParams>>,
    pub gas_cloud_sun_shadow_params: Vec<Option<GasCloudSunShadowParams>>,
    pub gas_cloud_vdbdata_params: Vec<Option<GasCloudVDBDataParams>>,
    pub gas_cloud_fade_sphere_params: Vec<Option<GasCloudFadeSphereParams>>,
    pub gas_cloud_fade_cube_params: Vec<Option<GasCloudFadeCubeParams>>,
    pub gas_cloud_fade_params: Vec<Option<GasCloudFadeParams>>,
    pub gas_cloud_vdbshaping_params: Vec<Option<GasCloudVDBShapingParams>>,
    pub gas_cloud_vdbedge_albedo_control_params: Vec<Option<GasCloudVDBEdgeAlbedoControlParams>>,
    pub gas_cloud_vdblighting_params: Vec<Option<GasCloudVDBLightingParams>>,
    pub gas_cloud_vdbgame_play_params: Vec<Option<GasCloudVDBGamePlayParams>>,
    pub gas_cloud_vdbparams: Vec<Option<GasCloudVDBParams>>,
    pub procedural_entity_audio_params: Vec<Option<ProceduralEntityAudioParams>>,
    pub planet_weather_params: Vec<Option<PlanetWeatherParams>>,
    pub planet_room_params: Vec<Option<PlanetRoomParams>>,
    pub planet_atmosphere_params: Vec<Option<PlanetAtmosphereParams>>,
    pub procedural_entity_params: Vec<Option<ProceduralEntityParams>>,
    pub asteroid_state_ref: Vec<Option<AsteroidStateRef>>,
    pub atmosphere_state_multi_ref: Vec<Option<AtmosphereStateMultiRef>>,
    pub radiation_state: Vec<Option<RadiationState>>,
}
