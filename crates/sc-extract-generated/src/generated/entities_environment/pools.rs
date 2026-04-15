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

/// Pool storage for the `entities-environment` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesEnvironmentPools {
    #[serde(default)]
    pub planet_navigation_volume_params: Vec<Option<PlanetNavigationVolumeParams>>,
    #[serde(default)]
    pub asteroid_field_shape_link_params: Vec<Option<AsteroidFieldShapeLinkParams>>,
    #[serde(default)]
    pub sasteroid_cluster_component_params: Vec<Option<SAsteroidClusterComponentParams>>,
    #[serde(default)]
    pub sasteroid_ring_component_params: Vec<Option<SAsteroidRingComponentParams>>,
    #[serde(default)]
    pub sasteroid_gas_cloud_component_params: Vec<Option<SAsteroidGasCloudComponentParams>>,
    #[serde(default)]
    pub sasteroid_shape_component_params: Vec<Option<SAsteroidShapeComponentParams>>,
    #[serde(default)]
    pub fog_volume_component_params: Vec<Option<FogVolumeComponentParams>>,
    #[serde(default)]
    pub harvestable_provider_params: Vec<Option<HarvestableProviderParams>>,
    #[serde(default)]
    pub sentity_component_managed_entity_region_params: Vec<Option<SEntityComponentManagedEntityRegionParams>>,
    #[serde(default)]
    pub sentity_component_managed_entity_zone_params: Vec<Option<SEntityComponentManagedEntityZoneParams>>,
    #[serde(default)]
    pub procedural_planet_audio_component_params: Vec<Option<ProceduralPlanetAudioComponentParams>>,
    #[serde(default)]
    pub planet_ocean_audio_component_params: Vec<Option<PlanetOceanAudioComponentParams>>,
    #[serde(default)]
    pub default_sphere_geom: Vec<Option<DefaultSphereGeom>>,
    #[serde(default)]
    pub sphere_field_geom: Vec<Option<SphereFieldGeom>>,
    #[serde(default)]
    pub box_field_geom: Vec<Option<BoxFieldGeom>>,
    #[serde(default)]
    pub cylinder_field_geom: Vec<Option<CylinderFieldGeom>>,
    #[serde(default)]
    pub capsule_field_geom: Vec<Option<CapsuleFieldGeom>>,
    #[serde(default)]
    pub torus_field_geom: Vec<Option<TorusFieldGeom>>,
    #[serde(default)]
    pub mesh_field_geom: Vec<Option<MeshFieldGeom>>,
    #[serde(default)]
    pub texture3_dfield_geom: Vec<Option<Texture3DFieldGeom>>,
    #[serde(default)]
    pub quantum_obstacle_params: Vec<Option<QuantumObstacleParams>>,
    #[serde(default)]
    pub solar_system_component_params: Vec<Option<SolarSystemComponentParams>>,
    #[serde(default)]
    pub sentity_component_noisy_smoothing_modification_object_params: Vec<Option<SEntityComponentNoisySmoothingModificationObjectParams>>,
    #[serde(default)]
    pub sentity_component_planet_area_params: Vec<Option<SEntityComponentPlanetAreaParams>>,
    #[serde(default)]
    pub sentity_component_push_pull_modification_object_params: Vec<Option<SEntityComponentPushPullModificationObjectParams>>,
    #[serde(default)]
    pub sentity_component_rectangle_modification_object_params: Vec<Option<SEntityComponentRectangleModificationObjectParams>>,
    #[serde(default)]
    pub sentity_component_smoothing_modification_object_params: Vec<Option<SEntityComponentSmoothingModificationObjectParams>>,
    #[serde(default)]
    pub gas_cloud_sun_shadow_params: Vec<Option<GasCloudSunShadowParams>>,
    #[serde(default)]
    pub gas_cloud_vdbdata_params: Vec<Option<GasCloudVDBDataParams>>,
    #[serde(default)]
    pub gas_cloud_fade_sphere_params: Vec<Option<GasCloudFadeSphereParams>>,
    #[serde(default)]
    pub gas_cloud_fade_cube_params: Vec<Option<GasCloudFadeCubeParams>>,
    #[serde(default)]
    pub gas_cloud_fade_params: Vec<Option<GasCloudFadeParams>>,
    #[serde(default)]
    pub gas_cloud_vdbshaping_params: Vec<Option<GasCloudVDBShapingParams>>,
    #[serde(default)]
    pub gas_cloud_vdbedge_albedo_control_params: Vec<Option<GasCloudVDBEdgeAlbedoControlParams>>,
    #[serde(default)]
    pub gas_cloud_vdblighting_params: Vec<Option<GasCloudVDBLightingParams>>,
    #[serde(default)]
    pub gas_cloud_vdbgame_play_params: Vec<Option<GasCloudVDBGamePlayParams>>,
    #[serde(default)]
    pub gas_cloud_vdbparams: Vec<Option<GasCloudVDBParams>>,
    #[serde(default)]
    pub procedural_entity_audio_params: Vec<Option<ProceduralEntityAudioParams>>,
    #[serde(default)]
    pub planet_weather_params: Vec<Option<PlanetWeatherParams>>,
    #[serde(default)]
    pub planet_room_params: Vec<Option<PlanetRoomParams>>,
    #[serde(default)]
    pub planet_atmosphere_params: Vec<Option<PlanetAtmosphereParams>>,
    #[serde(default)]
    pub procedural_entity_params: Vec<Option<ProceduralEntityParams>>,
    #[serde(default)]
    pub asteroid_state_ref: Vec<Option<AsteroidStateRef>>,
    #[serde(default)]
    pub atmosphere_state_multi_ref: Vec<Option<AtmosphereStateMultiRef>>,
    #[serde(default)]
    pub radiation_state: Vec<Option<RadiationState>>,
}
