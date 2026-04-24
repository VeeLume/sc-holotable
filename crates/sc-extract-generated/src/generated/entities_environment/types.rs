// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-environment`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `PlanetNavigationVolumeParams`
/// Inherits from: `DataForgeComponentParams`
pub struct PlanetNavigationVolumeParams {
}

impl Pooled for PlanetNavigationVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.planet_navigation_volume_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.planet_navigation_volume_params }
}

impl<'a> Extract<'a> for PlanetNavigationVolumeParams {
    const TYPE_NAME: &'static str = "PlanetNavigationVolumeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AsteroidFieldShapeLinkParams`
pub struct AsteroidFieldShapeLinkParams {
    /// `shapeLinkName` (String (array))
    pub shape_link_name: Vec<String>,
    /// `rngSeed` (Int32)
    pub rng_seed: i32,
    /// `viewDistRatio` (Byte)
    pub view_dist_ratio: u32,
    /// `lodRatio` (Byte)
    pub lod_ratio: u32,
    /// `composition` (Reference)
    pub composition: Option<CigGuid>,
    /// `densityScale` (Single)
    pub density_scale: f32,
    /// `noiseAmplitude` (Single)
    pub noise_amplitude: f32,
    /// `noiseRoughness` (Single)
    pub noise_roughness: f32,
    /// `noiseGranularity` (Single)
    pub noise_granularity: f32,
    /// `falloffCeiling` (Single)
    pub falloff_ceiling: f32,
    /// `shapeStrength` (Single)
    pub shape_strength: f32,
    /// `noiseFrequencyX` (Single)
    pub noise_frequency_x: f32,
    /// `noiseFrequencyY` (Single)
    pub noise_frequency_y: f32,
    /// `noiseFrequencyZ` (Single)
    pub noise_frequency_z: f32,
}

impl Pooled for AsteroidFieldShapeLinkParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.asteroid_field_shape_link_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.asteroid_field_shape_link_params }
}

impl<'a> Extract<'a> for AsteroidFieldShapeLinkParams {
    const TYPE_NAME: &'static str = "AsteroidFieldShapeLinkParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            shape_link_name: inst.get_array("shapeLinkName")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            rng_seed: inst.get_i32("rngSeed").unwrap_or_default(),
            view_dist_ratio: inst.get_u32("viewDistRatio").unwrap_or_default(),
            lod_ratio: inst.get_u32("lodRatio").unwrap_or_default(),
            composition: inst.get("composition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            density_scale: inst.get_f32("densityScale").unwrap_or_default(),
            noise_amplitude: inst.get_f32("noiseAmplitude").unwrap_or_default(),
            noise_roughness: inst.get_f32("noiseRoughness").unwrap_or_default(),
            noise_granularity: inst.get_f32("noiseGranularity").unwrap_or_default(),
            falloff_ceiling: inst.get_f32("falloffCeiling").unwrap_or_default(),
            shape_strength: inst.get_f32("shapeStrength").unwrap_or_default(),
            noise_frequency_x: inst.get_f32("noiseFrequencyX").unwrap_or_default(),
            noise_frequency_y: inst.get_f32("noiseFrequencyY").unwrap_or_default(),
            noise_frequency_z: inst.get_f32("noiseFrequencyZ").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAsteroidClusterComponentParams`
/// Inherits from: `SAsteroidFieldComponentParams`
pub struct SAsteroidClusterComponentParams {
    /// `rngSeed` (Int32)
    pub rng_seed: i32,
    /// `viewDistRatio` (Byte)
    pub view_dist_ratio: u32,
    /// `lodRatio` (Byte)
    pub lod_ratio: u32,
    /// `composition` (String)
    pub composition: String,
    /// `noiseAmplitude` (Single)
    pub noise_amplitude: f32,
    /// `noiseRoughness` (Single)
    pub noise_roughness: f32,
    /// `noiseGranularity` (Single)
    pub noise_granularity: f32,
    /// `densityScale` (Single)
    pub density_scale: f32,
    /// `widthKm` (Double)
    pub width_km: f64,
    /// `lengthKm` (Double)
    pub length_km: f64,
    /// `heightKm` (Double)
    pub height_km: f64,
    /// `noiseFrequencyX` (Single)
    pub noise_frequency_x: f32,
    /// `noiseFrequencyY` (Single)
    pub noise_frequency_y: f32,
    /// `noiseFrequencyZ` (Single)
    pub noise_frequency_z: f32,
}

impl Pooled for SAsteroidClusterComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sasteroid_cluster_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sasteroid_cluster_component_params }
}

impl<'a> Extract<'a> for SAsteroidClusterComponentParams {
    const TYPE_NAME: &'static str = "SAsteroidClusterComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rng_seed: inst.get_i32("rngSeed").unwrap_or_default(),
            view_dist_ratio: inst.get_u32("viewDistRatio").unwrap_or_default(),
            lod_ratio: inst.get_u32("lodRatio").unwrap_or_default(),
            composition: inst.get_str("composition").map(String::from).unwrap_or_default(),
            noise_amplitude: inst.get_f32("noiseAmplitude").unwrap_or_default(),
            noise_roughness: inst.get_f32("noiseRoughness").unwrap_or_default(),
            noise_granularity: inst.get_f32("noiseGranularity").unwrap_or_default(),
            density_scale: inst.get_f32("densityScale").unwrap_or_default(),
            width_km: inst.get_f64("widthKm").unwrap_or_default(),
            length_km: inst.get_f64("lengthKm").unwrap_or_default(),
            height_km: inst.get_f64("heightKm").unwrap_or_default(),
            noise_frequency_x: inst.get_f32("noiseFrequencyX").unwrap_or_default(),
            noise_frequency_y: inst.get_f32("noiseFrequencyY").unwrap_or_default(),
            noise_frequency_z: inst.get_f32("noiseFrequencyZ").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAsteroidRingComponentParams`
/// Inherits from: `SAsteroidFieldComponentParams`
pub struct SAsteroidRingComponentParams {
    /// `rngSeed` (Int32)
    pub rng_seed: i32,
    /// `viewDistRatio` (Byte)
    pub view_dist_ratio: u32,
    /// `lodRatio` (Byte)
    pub lod_ratio: u32,
    /// `composition` (String)
    pub composition: String,
    /// `noiseAmplitude` (Single)
    pub noise_amplitude: f32,
    /// `noiseRoughness` (Single)
    pub noise_roughness: f32,
    /// `noiseGranularity` (Single)
    pub noise_granularity: f32,
    /// `densityScale` (Single)
    pub density_scale: f32,
    /// `innerRadiusKm` (Double)
    pub inner_radius_km: f64,
    /// `outerRadiusKm` (Double)
    pub outer_radius_km: f64,
    /// `depthKm` (Double)
    pub depth_km: f64,
    /// `noiseFrequencyRadial` (Single)
    pub noise_frequency_radial: f32,
}

impl Pooled for SAsteroidRingComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sasteroid_ring_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sasteroid_ring_component_params }
}

impl<'a> Extract<'a> for SAsteroidRingComponentParams {
    const TYPE_NAME: &'static str = "SAsteroidRingComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rng_seed: inst.get_i32("rngSeed").unwrap_or_default(),
            view_dist_ratio: inst.get_u32("viewDistRatio").unwrap_or_default(),
            lod_ratio: inst.get_u32("lodRatio").unwrap_or_default(),
            composition: inst.get_str("composition").map(String::from).unwrap_or_default(),
            noise_amplitude: inst.get_f32("noiseAmplitude").unwrap_or_default(),
            noise_roughness: inst.get_f32("noiseRoughness").unwrap_or_default(),
            noise_granularity: inst.get_f32("noiseGranularity").unwrap_or_default(),
            density_scale: inst.get_f32("densityScale").unwrap_or_default(),
            inner_radius_km: inst.get_f64("innerRadiusKm").unwrap_or_default(),
            outer_radius_km: inst.get_f64("outerRadiusKm").unwrap_or_default(),
            depth_km: inst.get_f64("depthKm").unwrap_or_default(),
            noise_frequency_radial: inst.get_f32("noiseFrequencyRadial").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAsteroidGasCloudComponentParams`
/// Inherits from: `SAsteroidFieldComponentParams`
pub struct SAsteroidGasCloudComponentParams {
    /// `rngSeed` (Int32)
    pub rng_seed: i32,
    /// `viewDistRatio` (Byte)
    pub view_dist_ratio: u32,
    /// `lodRatio` (Byte)
    pub lod_ratio: u32,
    /// `composition` (String)
    pub composition: String,
    /// `noiseAmplitude` (Single)
    pub noise_amplitude: f32,
    /// `noiseRoughness` (Single)
    pub noise_roughness: f32,
    /// `noiseGranularity` (Single)
    pub noise_granularity: f32,
    /// `densityScale` (Single)
    pub density_scale: f32,
    /// `densityExponent` (Single)
    pub density_exponent: f32,
    /// `densityMinimum` (Single)
    pub density_minimum: f32,
    /// `densityMaximum` (Single)
    pub density_maximum: f32,
}

impl Pooled for SAsteroidGasCloudComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sasteroid_gas_cloud_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sasteroid_gas_cloud_component_params }
}

impl<'a> Extract<'a> for SAsteroidGasCloudComponentParams {
    const TYPE_NAME: &'static str = "SAsteroidGasCloudComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rng_seed: inst.get_i32("rngSeed").unwrap_or_default(),
            view_dist_ratio: inst.get_u32("viewDistRatio").unwrap_or_default(),
            lod_ratio: inst.get_u32("lodRatio").unwrap_or_default(),
            composition: inst.get_str("composition").map(String::from).unwrap_or_default(),
            noise_amplitude: inst.get_f32("noiseAmplitude").unwrap_or_default(),
            noise_roughness: inst.get_f32("noiseRoughness").unwrap_or_default(),
            noise_granularity: inst.get_f32("noiseGranularity").unwrap_or_default(),
            density_scale: inst.get_f32("densityScale").unwrap_or_default(),
            density_exponent: inst.get_f32("densityExponent").unwrap_or_default(),
            density_minimum: inst.get_f32("densityMinimum").unwrap_or_default(),
            density_maximum: inst.get_f32("densityMaximum").unwrap_or_default(),
        }
    }
}

/// DCB type: `SAsteroidShapeComponentParams`
/// Inherits from: `SAsteroidFieldComponentParams`
pub struct SAsteroidShapeComponentParams {
    /// `rngSeed` (Int32)
    pub rng_seed: i32,
    /// `viewDistRatio` (Byte)
    pub view_dist_ratio: u32,
    /// `lodRatio` (Byte)
    pub lod_ratio: u32,
    /// `composition` (String)
    pub composition: String,
    /// `noiseAmplitude` (Single)
    pub noise_amplitude: f32,
    /// `noiseRoughness` (Single)
    pub noise_roughness: f32,
    /// `noiseGranularity` (Single)
    pub noise_granularity: f32,
    /// `densityScale` (Single)
    pub density_scale: f32,
    /// `noiseFrequencyX` (Single)
    pub noise_frequency_x: f32,
    /// `noiseFrequencyY` (Single)
    pub noise_frequency_y: f32,
    /// `noiseFrequencyZ` (Single)
    pub noise_frequency_z: f32,
    /// `shapeLinks` (Class (array))
    pub shape_links: Vec<Handle<AsteroidFieldShapeLinkParams>>,
    /// `exclusionShapeLinks` (String (array))
    pub exclusion_shape_links: Vec<String>,
}

impl Pooled for SAsteroidShapeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sasteroid_shape_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sasteroid_shape_component_params }
}

impl<'a> Extract<'a> for SAsteroidShapeComponentParams {
    const TYPE_NAME: &'static str = "SAsteroidShapeComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rng_seed: inst.get_i32("rngSeed").unwrap_or_default(),
            view_dist_ratio: inst.get_u32("viewDistRatio").unwrap_or_default(),
            lod_ratio: inst.get_u32("lodRatio").unwrap_or_default(),
            composition: inst.get_str("composition").map(String::from).unwrap_or_default(),
            noise_amplitude: inst.get_f32("noiseAmplitude").unwrap_or_default(),
            noise_roughness: inst.get_f32("noiseRoughness").unwrap_or_default(),
            noise_granularity: inst.get_f32("noiseGranularity").unwrap_or_default(),
            density_scale: inst.get_f32("densityScale").unwrap_or_default(),
            noise_frequency_x: inst.get_f32("noiseFrequencyX").unwrap_or_default(),
            noise_frequency_y: inst.get_f32("noiseFrequencyY").unwrap_or_default(),
            noise_frequency_z: inst.get_f32("noiseFrequencyZ").unwrap_or_default(),
            shape_links: inst.get_array("shapeLinks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AsteroidFieldShapeLinkParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AsteroidFieldShapeLinkParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            exclusion_shape_links: inst.get_array("exclusionShapeLinks")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FogVolumeComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct FogVolumeComponentParams {
    /// `active` (Boolean)
    pub active: bool,
    /// `volumeType` (Int32)
    pub volume_type: i32,
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
    /// `color` (Class)
    pub color: Option<Handle<RGB>>,
    /// `hdrDynamic` (Single)
    pub hdr_dynamic: f32,
    /// `useGlobalFogColor` (Boolean)
    pub use_global_fog_color: bool,
    /// `globalDensity` (Single)
    pub global_density: f32,
    /// `densityOffset` (Single)
    pub density_offset: f32,
    /// `fallOffDirLong` (Single)
    pub fall_off_dir_long: f32,
    /// `fallOffDirLati` (Single)
    pub fall_off_dir_lati: f32,
    /// `fallOffShift` (Single)
    pub fall_off_shift: f32,
    /// `fallOffScale` (Single)
    pub fall_off_scale: f32,
    /// `softEdges` (Single)
    pub soft_edges: f32,
    /// `rampStart` (Single)
    pub ramp_start: f32,
    /// `rampEnd` (Single)
    pub ramp_end: f32,
    /// `rampInfluence` (Single)
    pub ramp_influence: f32,
    /// `densityNoiseScale` (Single)
    pub density_noise_scale: f32,
    /// `densityNoiseOffset` (Single)
    pub density_noise_offset: f32,
    /// `densityNoiseTimeFrequency` (Single)
    pub density_noise_time_frequency: f32,
    /// `densityNoiseSize` (Class)
    pub density_noise_size: Option<Handle<Vec3>>,
    /// `densityNoiseWindInfluence` (Single)
    pub density_noise_wind_influence: f32,
    /// `affectsThisAreaOnly` (Boolean)
    pub affects_this_area_only: bool,
    /// `maxDistance` (Single)
    pub max_distance: f32,
}

impl Pooled for FogVolumeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.fog_volume_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.fog_volume_component_params }
}

impl<'a> Extract<'a> for FogVolumeComponentParams {
    const TYPE_NAME: &'static str = "FogVolumeComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            active: inst.get_bool("active").unwrap_or_default(),
            volume_type: inst.get_i32("volumeType").unwrap_or_default(),
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hdr_dynamic: inst.get_f32("hdrDynamic").unwrap_or_default(),
            use_global_fog_color: inst.get_bool("useGlobalFogColor").unwrap_or_default(),
            global_density: inst.get_f32("globalDensity").unwrap_or_default(),
            density_offset: inst.get_f32("densityOffset").unwrap_or_default(),
            fall_off_dir_long: inst.get_f32("fallOffDirLong").unwrap_or_default(),
            fall_off_dir_lati: inst.get_f32("fallOffDirLati").unwrap_or_default(),
            fall_off_shift: inst.get_f32("fallOffShift").unwrap_or_default(),
            fall_off_scale: inst.get_f32("fallOffScale").unwrap_or_default(),
            soft_edges: inst.get_f32("softEdges").unwrap_or_default(),
            ramp_start: inst.get_f32("rampStart").unwrap_or_default(),
            ramp_end: inst.get_f32("rampEnd").unwrap_or_default(),
            ramp_influence: inst.get_f32("rampInfluence").unwrap_or_default(),
            density_noise_scale: inst.get_f32("densityNoiseScale").unwrap_or_default(),
            density_noise_offset: inst.get_f32("densityNoiseOffset").unwrap_or_default(),
            density_noise_time_frequency: inst.get_f32("densityNoiseTimeFrequency").unwrap_or_default(),
            density_noise_size: match inst.get("densityNoiseSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            density_noise_wind_influence: inst.get_f32("densityNoiseWindInfluence").unwrap_or_default(),
            affects_this_area_only: inst.get_bool("affectsThisAreaOnly").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `HarvestableProviderParams`
/// Inherits from: `DataForgeComponentParams`
pub struct HarvestableProviderParams {
    /// `preset` (Reference)
    pub preset: Option<CigGuid>,
}

impl Pooled for HarvestableProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.harvestable_provider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.harvestable_provider_params }
}

impl<'a> Extract<'a> for HarvestableProviderParams {
    const TYPE_NAME: &'static str = "HarvestableProviderParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            preset: inst.get("preset").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SEntityComponentManagedEntityRegionParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentManagedEntityRegionParams {
}

impl Pooled for SEntityComponentManagedEntityRegionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sentity_component_managed_entity_region_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sentity_component_managed_entity_region_params }
}

impl<'a> Extract<'a> for SEntityComponentManagedEntityRegionParams {
    const TYPE_NAME: &'static str = "SEntityComponentManagedEntityRegionParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityComponentManagedEntityZoneParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentManagedEntityZoneParams {
}

impl Pooled for SEntityComponentManagedEntityZoneParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sentity_component_managed_entity_zone_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sentity_component_managed_entity_zone_params }
}

impl<'a> Extract<'a> for SEntityComponentManagedEntityZoneParams {
    const TYPE_NAME: &'static str = "SEntityComponentManagedEntityZoneParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ProceduralPlanetAudioComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct ProceduralPlanetAudioComponentParams {
    /// `planetAudioData` (Reference)
    pub planet_audio_data: Option<CigGuid>,
    /// `riverAudioData` (Reference)
    pub river_audio_data: Option<CigGuid>,
}

impl Pooled for ProceduralPlanetAudioComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.procedural_planet_audio_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.procedural_planet_audio_component_params }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioComponentParams {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            planet_audio_data: inst.get("planetAudioData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            river_audio_data: inst.get("riverAudioData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `PlanetOceanAudioComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct PlanetOceanAudioComponentParams {
    /// `planetOceanAudioData` (Reference)
    pub planet_ocean_audio_data: Option<CigGuid>,
}

impl Pooled for PlanetOceanAudioComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.planet_ocean_audio_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.planet_ocean_audio_component_params }
}

impl<'a> Extract<'a> for PlanetOceanAudioComponentParams {
    const TYPE_NAME: &'static str = "PlanetOceanAudioComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            planet_ocean_audio_data: inst.get("planetOceanAudioData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `DefaultSphereGeom`
pub struct DefaultSphereGeom {
    /// `disable` (Boolean)
    pub disable: bool,
    /// `radiusInflation` (Single)
    pub radius_inflation: f32,
}

impl Pooled for DefaultSphereGeom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.default_sphere_geom }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.default_sphere_geom }
}

impl<'a> Extract<'a> for DefaultSphereGeom {
    const TYPE_NAME: &'static str = "DefaultSphereGeom";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            disable: inst.get_bool("disable").unwrap_or_default(),
            radius_inflation: inst.get_f32("radiusInflation").unwrap_or_default(),
        }
    }
}

/// DCB type: `SphereFieldGeom`
pub struct SphereFieldGeom {
    /// `center` (Class)
    pub center: Option<Handle<Vec3>>,
    /// `R` (Single)
    pub r: f32,
}

impl Pooled for SphereFieldGeom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sphere_field_geom }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sphere_field_geom }
}

impl<'a> Extract<'a> for SphereFieldGeom {
    const TYPE_NAME: &'static str = "SphereFieldGeom";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            center: match inst.get("center") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            r: inst.get_f32("R").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoxFieldGeom`
pub struct BoxFieldGeom {
    /// `center` (Class)
    pub center: Option<Handle<Vec3>>,
    /// `oriented` (Boolean)
    pub oriented: bool,
    /// `basis` (Class)
    pub basis: Option<Handle<Quat>>,
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
}

impl Pooled for BoxFieldGeom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.box_field_geom }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.box_field_geom }
}

impl<'a> Extract<'a> for BoxFieldGeom {
    const TYPE_NAME: &'static str = "BoxFieldGeom";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            center: match inst.get("center") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            oriented: inst.get_bool("oriented").unwrap_or_default(),
            basis: match inst.get("basis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Quat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CylinderFieldGeom`
pub struct CylinderFieldGeom {
    /// `center` (Class)
    pub center: Option<Handle<Vec3>>,
    /// `axis` (Class)
    pub axis: Option<Handle<Vec3>>,
    /// `radius` (Single)
    pub radius: f32,
    /// `hh` (Single)
    pub hh: f32,
}

impl Pooled for CylinderFieldGeom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.cylinder_field_geom }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.cylinder_field_geom }
}

impl<'a> Extract<'a> for CylinderFieldGeom {
    const TYPE_NAME: &'static str = "CylinderFieldGeom";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            center: match inst.get("center") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            axis: match inst.get("axis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            radius: inst.get_f32("radius").unwrap_or_default(),
            hh: inst.get_f32("hh").unwrap_or_default(),
        }
    }
}

/// DCB type: `CapsuleFieldGeom`
pub struct CapsuleFieldGeom {
    /// `center` (Class)
    pub center: Option<Handle<Vec3>>,
    /// `axis` (Class)
    pub axis: Option<Handle<Vec3>>,
    /// `radius` (Single)
    pub radius: f32,
    /// `hh` (Single)
    pub hh: f32,
}

impl Pooled for CapsuleFieldGeom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.capsule_field_geom }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.capsule_field_geom }
}

impl<'a> Extract<'a> for CapsuleFieldGeom {
    const TYPE_NAME: &'static str = "CapsuleFieldGeom";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            center: match inst.get("center") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            axis: match inst.get("axis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            radius: inst.get_f32("radius").unwrap_or_default(),
            hh: inst.get_f32("hh").unwrap_or_default(),
        }
    }
}

/// DCB type: `TorusFieldGeom`
pub struct TorusFieldGeom {
    /// `center` (Class)
    pub center: Option<Handle<Vec3>>,
    /// `axis` (Class)
    pub axis: Option<Handle<Vec3>>,
    /// `R` (Single)
    pub r: f32,
    /// `r` (Single)
    pub r_2: f32,
}

impl Pooled for TorusFieldGeom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.torus_field_geom }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.torus_field_geom }
}

impl<'a> Extract<'a> for TorusFieldGeom {
    const TYPE_NAME: &'static str = "TorusFieldGeom";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            center: match inst.get("center") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            axis: match inst.get("axis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            r: inst.get_f32("R").unwrap_or_default(),
            r_2: inst.get_f32("r").unwrap_or_default(),
        }
    }
}

/// DCB type: `MeshFieldGeom`
pub struct MeshFieldGeom {
    /// `geometry` (Class)
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
}

impl Pooled for MeshFieldGeom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.mesh_field_geom }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.mesh_field_geom }
}

impl<'a> Extract<'a> for MeshFieldGeom {
    const TYPE_NAME: &'static str = "MeshFieldGeom";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Texture3DFieldGeom`
pub struct Texture3DFieldGeom {
    /// `texture3D` (String)
    pub texture3_d: String,
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
    /// `posOffset` (Class)
    pub pos_offset: Option<Handle<Vec3>>,
    /// `rotOffset` (Class)
    pub rot_offset: Option<Handle<Ang3>>,
}

impl Pooled for Texture3DFieldGeom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.texture3_dfield_geom }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.texture3_dfield_geom }
}

impl<'a> Extract<'a> for Texture3DFieldGeom {
    const TYPE_NAME: &'static str = "Texture3DFieldGeom";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            texture3_d: inst.get_str("texture3D").map(String::from).unwrap_or_default(),
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            pos_offset: match inst.get("posOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rot_offset: match inst.get("rotOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `QuantumObstacleParams`
/// Inherits from: `DataForgeComponentParams`
pub struct QuantumObstacleParams {
    /// `defaultSphereFieldGeometry` (Class)
    pub default_sphere_field_geometry: Option<Handle<DefaultSphereGeom>>,
    /// `sphereFieldGeometries` (Class (array))
    pub sphere_field_geometries: Vec<Handle<SphereFieldGeom>>,
    /// `boxFieldGeometries` (Class (array))
    pub box_field_geometries: Vec<Handle<BoxFieldGeom>>,
    /// `cylinderFieldGeometries` (Class (array))
    pub cylinder_field_geometries: Vec<Handle<CylinderFieldGeom>>,
    /// `capsuleFieldGeometries` (Class (array))
    pub capsule_field_geometries: Vec<Handle<CapsuleFieldGeom>>,
    /// `torusFieldGeometries` (Class (array))
    pub torus_field_geometries: Vec<Handle<TorusFieldGeom>>,
    /// `meshFieldGeometries` (Class (array))
    pub mesh_field_geometries: Vec<Handle<MeshFieldGeom>>,
    /// `texture3DFieldGeometries` (Class (array))
    pub texture3_dfield_geometries: Vec<Handle<Texture3DFieldGeom>>,
}

impl Pooled for QuantumObstacleParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.quantum_obstacle_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.quantum_obstacle_params }
}

impl<'a> Extract<'a> for QuantumObstacleParams {
    const TYPE_NAME: &'static str = "QuantumObstacleParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_sphere_field_geometry: match inst.get("defaultSphereFieldGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DefaultSphereGeom>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            sphere_field_geometries: inst.get_array("sphereFieldGeometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SphereFieldGeom>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SphereFieldGeom>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            box_field_geometries: inst.get_array("boxFieldGeometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BoxFieldGeom>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<BoxFieldGeom>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            cylinder_field_geometries: inst.get_array("cylinderFieldGeometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CylinderFieldGeom>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<CylinderFieldGeom>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            capsule_field_geometries: inst.get_array("capsuleFieldGeometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CapsuleFieldGeom>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<CapsuleFieldGeom>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            torus_field_geometries: inst.get_array("torusFieldGeometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TorusFieldGeom>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TorusFieldGeom>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            mesh_field_geometries: inst.get_array("meshFieldGeometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeshFieldGeom>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<MeshFieldGeom>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            texture3_dfield_geometries: inst.get_array("texture3DFieldGeometries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Texture3DFieldGeom>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<Texture3DFieldGeom>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SolarSystemComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SolarSystemComponentParams {
}

impl Pooled for SolarSystemComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.solar_system_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.solar_system_component_params }
}

impl<'a> Extract<'a> for SolarSystemComponentParams {
    const TYPE_NAME: &'static str = "SolarSystemComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SEntityComponentNoisySmoothingModificationObjectParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentNoisySmoothingModificationObjectParams {
    /// `smoothingParams` (Class)
    pub smoothing_params: Option<Handle<SEntityComponentSmoothingModificationObjectParams>>,
    /// `noiseAmount` (Single)
    pub noise_amount: f32,
    /// `noiseFrequency` (Single)
    pub noise_frequency: f32,
    /// `seed` (Int32)
    pub seed: i32,
}

impl Pooled for SEntityComponentNoisySmoothingModificationObjectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sentity_component_noisy_smoothing_modification_object_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sentity_component_noisy_smoothing_modification_object_params }
}

impl<'a> Extract<'a> for SEntityComponentNoisySmoothingModificationObjectParams {
    const TYPE_NAME: &'static str = "SEntityComponentNoisySmoothingModificationObjectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            smoothing_params: match inst.get("smoothingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityComponentSmoothingModificationObjectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            noise_amount: inst.get_f32("noiseAmount").unwrap_or_default(),
            noise_frequency: inst.get_f32("noiseFrequency").unwrap_or_default(),
            seed: inst.get_i32("seed").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityComponentPlanetAreaParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentPlanetAreaParams {
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
    /// `areaType` (EnumChoice)
    pub area_type: PlanetAreaType,
    /// `lodLevel` (Int32)
    pub lod_level: i32,
    /// `bOverride` (Boolean)
    pub b_override: bool,
    /// `areaFitting` (EnumChoice)
    pub area_fitting: PlanetAreaFitting,
}

impl Pooled for SEntityComponentPlanetAreaParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sentity_component_planet_area_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sentity_component_planet_area_params }
}

impl<'a> Extract<'a> for SEntityComponentPlanetAreaParams {
    const TYPE_NAME: &'static str = "SEntityComponentPlanetAreaParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            area_type: PlanetAreaType::from_dcb_str(inst.get_str("areaType").unwrap_or("")),
            lod_level: inst.get_i32("lodLevel").unwrap_or_default(),
            b_override: inst.get_bool("bOverride").unwrap_or_default(),
            area_fitting: PlanetAreaFitting::from_dcb_str(inst.get_str("areaFitting").unwrap_or("")),
        }
    }
}

/// DCB type: `SEntityComponentPushPullModificationObjectParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentPushPullModificationObjectParams {
    /// `size` (Single)
    pub size: f32,
    /// `strength` (Single)
    pub strength: f32,
    /// `pull` (Boolean)
    pub pull: bool,
    /// `steepness` (Single)
    pub steepness: f32,
    /// `elliptical` (Boolean)
    pub elliptical: bool,
    /// `rimRadius` (Single)
    pub rim_radius: f32,
    /// `sortOrder` (Int32)
    pub sort_order: i32,
}

impl Pooled for SEntityComponentPushPullModificationObjectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sentity_component_push_pull_modification_object_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sentity_component_push_pull_modification_object_params }
}

impl<'a> Extract<'a> for SEntityComponentPushPullModificationObjectParams {
    const TYPE_NAME: &'static str = "SEntityComponentPushPullModificationObjectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            size: inst.get_f32("size").unwrap_or_default(),
            strength: inst.get_f32("strength").unwrap_or_default(),
            pull: inst.get_bool("pull").unwrap_or_default(),
            steepness: inst.get_f32("steepness").unwrap_or_default(),
            elliptical: inst.get_bool("elliptical").unwrap_or_default(),
            rim_radius: inst.get_f32("rimRadius").unwrap_or_default(),
            sort_order: inst.get_i32("sortOrder").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityComponentRectangleModificationObjectParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentRectangleModificationObjectParams {
    /// `size` (Class)
    pub size: Option<Handle<Vec3>>,
    /// `rollOff` (Single)
    pub roll_off: f32,
    /// `strength` (Single)
    pub strength: f32,
    /// `dishEffect` (Single)
    pub dish_effect: f32,
    /// `sortOrder` (Int32)
    pub sort_order: i32,
}

impl Pooled for SEntityComponentRectangleModificationObjectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sentity_component_rectangle_modification_object_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sentity_component_rectangle_modification_object_params }
}

impl<'a> Extract<'a> for SEntityComponentRectangleModificationObjectParams {
    const TYPE_NAME: &'static str = "SEntityComponentRectangleModificationObjectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            roll_off: inst.get_f32("rollOff").unwrap_or_default(),
            strength: inst.get_f32("strength").unwrap_or_default(),
            dish_effect: inst.get_f32("dishEffect").unwrap_or_default(),
            sort_order: inst.get_i32("sortOrder").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityComponentSmoothingModificationObjectParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentSmoothingModificationObjectParams {
    /// `size` (Single)
    pub size: f32,
    /// `rollOff` (Single)
    pub roll_off: f32,
    /// `strength` (Single)
    pub strength: f32,
    /// `sortOrder` (Int32)
    pub sort_order: i32,
}

impl Pooled for SEntityComponentSmoothingModificationObjectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.sentity_component_smoothing_modification_object_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.sentity_component_smoothing_modification_object_params }
}

impl<'a> Extract<'a> for SEntityComponentSmoothingModificationObjectParams {
    const TYPE_NAME: &'static str = "SEntityComponentSmoothingModificationObjectParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            size: inst.get_f32("size").unwrap_or_default(),
            roll_off: inst.get_f32("rollOff").unwrap_or_default(),
            strength: inst.get_f32("strength").unwrap_or_default(),
            sort_order: inst.get_i32("sortOrder").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudSunShadowParams`
pub struct GasCloudSunShadowParams {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `resScaleXY` (Single)
    pub res_scale_xy: f32,
    /// `resScaleZ` (Single)
    pub res_scale_z: f32,
    /// `shadowDistanceScale` (Single)
    pub shadow_distance_scale: f32,
}

impl Pooled for GasCloudSunShadowParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_sun_shadow_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_sun_shadow_params }
}

impl<'a> Extract<'a> for GasCloudSunShadowParams {
    const TYPE_NAME: &'static str = "GasCloudSunShadowParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            res_scale_xy: inst.get_f32("resScaleXY").unwrap_or_default(),
            res_scale_z: inst.get_f32("resScaleZ").unwrap_or_default(),
            shadow_distance_scale: inst.get_f32("shadowDistanceScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudVDBDataParams`
pub struct GasCloudVDBDataParams {
    /// `file` (String)
    pub file: String,
}

impl Pooled for GasCloudVDBDataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_vdbdata_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_vdbdata_params }
}

impl<'a> Extract<'a> for GasCloudVDBDataParams {
    const TYPE_NAME: &'static str = "GasCloudVDBDataParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            file: inst.get_str("file").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudFadeSphereParams`
pub struct GasCloudFadeSphereParams {
    /// `radius` (Single)
    pub radius: f32,
    /// `fade` (Single)
    pub fade: f32,
}

impl Pooled for GasCloudFadeSphereParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_fade_sphere_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_fade_sphere_params }
}

impl<'a> Extract<'a> for GasCloudFadeSphereParams {
    const TYPE_NAME: &'static str = "GasCloudFadeSphereParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            radius: inst.get_f32("radius").unwrap_or_default(),
            fade: inst.get_f32("fade").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudFadeCubeParams`
pub struct GasCloudFadeCubeParams {
    /// `sizeX` (Single)
    pub size_x: f32,
    /// `sizeY` (Single)
    pub size_y: f32,
    /// `sizeZ` (Single)
    pub size_z: f32,
    /// `fadeX` (Single)
    pub fade_x: f32,
    /// `fadeY` (Single)
    pub fade_y: f32,
    /// `fadeZ` (Single)
    pub fade_z: f32,
}

impl Pooled for GasCloudFadeCubeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_fade_cube_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_fade_cube_params }
}

impl<'a> Extract<'a> for GasCloudFadeCubeParams {
    const TYPE_NAME: &'static str = "GasCloudFadeCubeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            size_x: inst.get_f32("sizeX").unwrap_or_default(),
            size_y: inst.get_f32("sizeY").unwrap_or_default(),
            size_z: inst.get_f32("sizeZ").unwrap_or_default(),
            fade_x: inst.get_f32("fadeX").unwrap_or_default(),
            fade_y: inst.get_f32("fadeY").unwrap_or_default(),
            fade_z: inst.get_f32("fadeZ").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudFadeParams`
pub struct GasCloudFadeParams {
    /// `volumeType` (EnumChoice)
    pub volume_type: EGasCloudFadeVolumeType,
    /// `sphereParams` (Class)
    pub sphere_params: Option<Handle<GasCloudFadeSphereParams>>,
    /// `cubeParams` (Class)
    pub cube_params: Option<Handle<GasCloudFadeCubeParams>>,
    /// `preview` (Boolean)
    pub preview: bool,
}

impl Pooled for GasCloudFadeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_fade_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_fade_params }
}

impl<'a> Extract<'a> for GasCloudFadeParams {
    const TYPE_NAME: &'static str = "GasCloudFadeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            volume_type: EGasCloudFadeVolumeType::from_dcb_str(inst.get_str("volumeType").unwrap_or("")),
            sphere_params: match inst.get("sphereParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GasCloudFadeSphereParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            cube_params: match inst.get("cubeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GasCloudFadeCubeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            preview: inst.get_bool("preview").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudVDBShapingParams`
pub struct GasCloudVDBShapingParams {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `extinctionToOpticalDensity` (Single)
    pub extinction_to_optical_density: f32,
    /// `shapeNoiseTileFactor` (Single)
    pub shape_noise_tile_factor: f32,
    /// `shapeNoiseMipBias` (Single)
    pub shape_noise_mip_bias: f32,
    /// `shapeFactorControl` (Single)
    pub shape_factor_control: f32,
    /// `shapeNoiseMin` (Single)
    pub shape_noise_min: f32,
    /// `shapeNoiseMax` (Single)
    pub shape_noise_max: f32,
    /// `shapeNoiseErosionScale` (Single)
    pub shape_noise_erosion_scale: f32,
    /// `shapeNoiseErosionDensityBoost` (Single)
    pub shape_noise_erosion_density_boost: f32,
    /// `erosionNoiseTileFactor` (Single)
    pub erosion_noise_tile_factor: f32,
    /// `erosionNoiseMipBias` (Single)
    pub erosion_noise_mip_bias: f32,
    /// `erosionScale` (Single)
    pub erosion_scale: f32,
    /// `erosionDensityBoost` (Single)
    pub erosion_density_boost: f32,
    /// `erosionHiFreqScale` (Single)
    pub erosion_hi_freq_scale: f32,
    /// `erosionHiFreqDensityBoost` (Single)
    pub erosion_hi_freq_density_boost: f32,
    /// `vdbDensityScale` (Single)
    pub vdb_density_scale: f32,
    /// `vdbDensityBias` (Single)
    pub vdb_density_bias: f32,
}

impl Pooled for GasCloudVDBShapingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_vdbshaping_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_vdbshaping_params }
}

impl<'a> Extract<'a> for GasCloudVDBShapingParams {
    const TYPE_NAME: &'static str = "GasCloudVDBShapingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            extinction_to_optical_density: inst.get_f32("extinctionToOpticalDensity").unwrap_or_default(),
            shape_noise_tile_factor: inst.get_f32("shapeNoiseTileFactor").unwrap_or_default(),
            shape_noise_mip_bias: inst.get_f32("shapeNoiseMipBias").unwrap_or_default(),
            shape_factor_control: inst.get_f32("shapeFactorControl").unwrap_or_default(),
            shape_noise_min: inst.get_f32("shapeNoiseMin").unwrap_or_default(),
            shape_noise_max: inst.get_f32("shapeNoiseMax").unwrap_or_default(),
            shape_noise_erosion_scale: inst.get_f32("shapeNoiseErosionScale").unwrap_or_default(),
            shape_noise_erosion_density_boost: inst.get_f32("shapeNoiseErosionDensityBoost").unwrap_or_default(),
            erosion_noise_tile_factor: inst.get_f32("erosionNoiseTileFactor").unwrap_or_default(),
            erosion_noise_mip_bias: inst.get_f32("erosionNoiseMipBias").unwrap_or_default(),
            erosion_scale: inst.get_f32("erosionScale").unwrap_or_default(),
            erosion_density_boost: inst.get_f32("erosionDensityBoost").unwrap_or_default(),
            erosion_hi_freq_scale: inst.get_f32("erosionHiFreqScale").unwrap_or_default(),
            erosion_hi_freq_density_boost: inst.get_f32("erosionHiFreqDensityBoost").unwrap_or_default(),
            vdb_density_scale: inst.get_f32("vdbDensityScale").unwrap_or_default(),
            vdb_density_bias: inst.get_f32("vdbDensityBias").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudVDBEdgeAlbedoControlParams`
pub struct GasCloudVDBEdgeAlbedoControlParams {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `amplitude` (Single)
    pub amplitude: f32,
    /// `densityOffset` (Single)
    pub density_offset: f32,
    /// `densityRamp` (Single)
    pub density_ramp: f32,
    /// `densityFalloff` (Single)
    pub density_falloff: f32,
    /// `environmentSensitivityGlobal` (Single)
    pub environment_sensitivity_global: f32,
    /// `environmentSensitivityLocal` (Single)
    pub environment_sensitivity_local: f32,
}

impl Pooled for GasCloudVDBEdgeAlbedoControlParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_vdbedge_albedo_control_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_vdbedge_albedo_control_params }
}

impl<'a> Extract<'a> for GasCloudVDBEdgeAlbedoControlParams {
    const TYPE_NAME: &'static str = "GasCloudVDBEdgeAlbedoControlParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            amplitude: inst.get_f32("amplitude").unwrap_or_default(),
            density_offset: inst.get_f32("densityOffset").unwrap_or_default(),
            density_ramp: inst.get_f32("densityRamp").unwrap_or_default(),
            density_falloff: inst.get_f32("densityFalloff").unwrap_or_default(),
            environment_sensitivity_global: inst.get_f32("environmentSensitivityGlobal").unwrap_or_default(),
            environment_sensitivity_local: inst.get_f32("environmentSensitivityLocal").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudVDBLightingParams`
pub struct GasCloudVDBLightingParams {
    /// `albedo` (Class)
    pub albedo: Option<Handle<SRGB8>>,
    /// `lightLeakColor` (Class)
    pub light_leak_color: Option<Handle<SRGB8>>,
    /// `lightLeakScale` (Single)
    pub light_leak_scale: f32,
    /// `density` (Single)
    pub density: f32,
    /// `inheritParentDensity` (Boolean)
    pub inherit_parent_density: bool,
    /// `rayStepMultiplier` (Single)
    pub ray_step_multiplier: f32,
    /// `fadeParams` (Class)
    pub fade_params: Option<Handle<GasCloudFadeParams>>,
}

impl Pooled for GasCloudVDBLightingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_vdblighting_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_vdblighting_params }
}

impl<'a> Extract<'a> for GasCloudVDBLightingParams {
    const TYPE_NAME: &'static str = "GasCloudVDBLightingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            albedo: match inst.get("albedo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_leak_color: match inst.get("lightLeakColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_leak_scale: inst.get_f32("lightLeakScale").unwrap_or_default(),
            density: inst.get_f32("density").unwrap_or_default(),
            inherit_parent_density: inst.get_bool("inheritParentDensity").unwrap_or_default(),
            ray_step_multiplier: inst.get_f32("rayStepMultiplier").unwrap_or_default(),
            fade_params: match inst.get("fadeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GasCloudFadeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GasCloudVDBGamePlayParams`
pub struct GasCloudVDBGamePlayParams {
    /// `densityMultiplier` (Single)
    pub density_multiplier: f32,
    /// `falloffStepDistance` (Single)
    pub falloff_step_distance: f32,
}

impl Pooled for GasCloudVDBGamePlayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_vdbgame_play_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_vdbgame_play_params }
}

impl<'a> Extract<'a> for GasCloudVDBGamePlayParams {
    const TYPE_NAME: &'static str = "GasCloudVDBGamePlayParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            density_multiplier: inst.get_f32("densityMultiplier").unwrap_or_default(),
            falloff_step_distance: inst.get_f32("falloffStepDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudVDBParams`
/// Inherits from: `DataForgeComponentParams`
pub struct GasCloudVDBParams {
    /// `isChild` (Boolean)
    pub is_child: bool,
    /// `data` (Class)
    pub data: Option<Handle<GasCloudVDBDataParams>>,
    /// `shaping` (Class)
    pub shaping: Option<Handle<GasCloudVDBShapingParams>>,
    /// `edgeAlbedoControl` (Class)
    pub edge_albedo_control: Option<Handle<GasCloudVDBEdgeAlbedoControlParams>>,
    /// `lighting` (Class)
    pub lighting: Option<Handle<GasCloudVDBLightingParams>>,
    /// `shadow` (Class)
    pub shadow: Option<Handle<GasCloudSunShadowParams>>,
    /// `gamePlay` (Class)
    pub game_play: Option<Handle<GasCloudVDBGamePlayParams>>,
}

impl Pooled for GasCloudVDBParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.gas_cloud_vdbparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.gas_cloud_vdbparams }
}

impl<'a> Extract<'a> for GasCloudVDBParams {
    const TYPE_NAME: &'static str = "GasCloudVDBParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            is_child: inst.get_bool("isChild").unwrap_or_default(),
            data: match inst.get("data") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GasCloudVDBDataParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            shaping: match inst.get("shaping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GasCloudVDBShapingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            edge_albedo_control: match inst.get("edgeAlbedoControl") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GasCloudVDBEdgeAlbedoControlParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            lighting: match inst.get("lighting") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GasCloudVDBLightingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            shadow: match inst.get("shadow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GasCloudSunShadowParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            game_play: match inst.get("gamePlay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GasCloudVDBGamePlayParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProceduralEntityAudioParams`
pub struct ProceduralEntityAudioParams {
    /// `biomeSwitch` (Class)
    pub biome_switch: Option<Handle<AudioSwitch>>,
}

impl Pooled for ProceduralEntityAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.procedural_entity_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.procedural_entity_audio_params }
}

impl<'a> Extract<'a> for ProceduralEntityAudioParams {
    const TYPE_NAME: &'static str = "ProceduralEntityAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            biome_switch: match inst.get("biomeSwitch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlanetWeatherParams`
pub struct PlanetWeatherParams {
    /// `maximumWindSpeed` (Single)
    pub maximum_wind_speed: f32,
    /// `airDensity` (Single)
    pub air_density: f32,
    /// `windMapOverride` (String)
    pub wind_map_override: String,
    /// `windMapOffset` (Single)
    pub wind_map_offset: f32,
    /// `windMapRotationSpeed` (Single)
    pub wind_map_rotation_speed: f32,
    /// `windGustStrengthRange` (Single)
    pub wind_gust_strength_range: f32,
    /// `windGustSpeedMultiplier` (Single)
    pub wind_gust_speed_multiplier: f32,
    /// `elevationBeginWindSpeedDropOff` (Single)
    pub elevation_begin_wind_speed_drop_off: f32,
    /// `windGustRepetitionAmount` (UInt16)
    pub wind_gust_repetition_amount: u32,
    /// `enablePlanetaryGroundEffects` (Boolean)
    pub enable_planetary_ground_effects: bool,
}

impl Pooled for PlanetWeatherParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.planet_weather_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.planet_weather_params }
}

impl<'a> Extract<'a> for PlanetWeatherParams {
    const TYPE_NAME: &'static str = "PlanetWeatherParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            maximum_wind_speed: inst.get_f32("maximumWindSpeed").unwrap_or_default(),
            air_density: inst.get_f32("airDensity").unwrap_or_default(),
            wind_map_override: inst.get_str("windMapOverride").map(String::from).unwrap_or_default(),
            wind_map_offset: inst.get_f32("windMapOffset").unwrap_or_default(),
            wind_map_rotation_speed: inst.get_f32("windMapRotationSpeed").unwrap_or_default(),
            wind_gust_strength_range: inst.get_f32("windGustStrengthRange").unwrap_or_default(),
            wind_gust_speed_multiplier: inst.get_f32("windGustSpeedMultiplier").unwrap_or_default(),
            elevation_begin_wind_speed_drop_off: inst.get_f32("elevationBeginWindSpeedDropOff").unwrap_or_default(),
            wind_gust_repetition_amount: inst.get_u32("windGustRepetitionAmount").unwrap_or_default(),
            enable_planetary_ground_effects: inst.get_bool("enablePlanetaryGroundEffects").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetRoomParams`
pub struct PlanetRoomParams {
    /// `TemperatureRange` (Single)
    pub temperature_range: f32,
    /// `HumidityRange` (Single)
    pub humidity_range: f32,
    /// `dayNightTemperatureParams` (StrongPointer)
    pub day_night_temperature_params: Option<PlanetDayNightTemperatureBaseParamsPtr>,
}

impl Pooled for PlanetRoomParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.planet_room_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.planet_room_params }
}

impl<'a> Extract<'a> for PlanetRoomParams {
    const TYPE_NAME: &'static str = "PlanetRoomParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            temperature_range: inst.get_f32("TemperatureRange").unwrap_or_default(),
            humidity_range: inst.get_f32("HumidityRange").unwrap_or_default(),
            day_night_temperature_params: match inst.get("dayNightTemperatureParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(PlanetDayNightTemperatureBaseParamsPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlanetAtmosphereParams`
pub struct PlanetAtmosphereParams {
    /// `PressureLinearFalloffInterpolation` (Double)
    pub pressure_linear_falloff_interpolation: f64,
}

impl Pooled for PlanetAtmosphereParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.planet_atmosphere_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.planet_atmosphere_params }
}

impl<'a> Extract<'a> for PlanetAtmosphereParams {
    const TYPE_NAME: &'static str = "PlanetAtmosphereParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            pressure_linear_falloff_interpolation: inst.get_f64("PressureLinearFalloffInterpolation").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralEntityParams`
/// Inherits from: `DataForgeComponentParams`
pub struct ProceduralEntityParams {
    /// `Weather` (Class)
    pub weather: Option<Handle<PlanetWeatherParams>>,
    /// `Room` (Class)
    pub room: Option<Handle<PlanetRoomParams>>,
    /// `Atmosphere` (Class)
    pub atmosphere: Option<Handle<PlanetAtmosphereParams>>,
    /// `AudioParams` (Class)
    pub audio_params: Option<Handle<ProceduralEntityAudioParams>>,
}

impl Pooled for ProceduralEntityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.procedural_entity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.procedural_entity_params }
}

impl<'a> Extract<'a> for ProceduralEntityParams {
    const TYPE_NAME: &'static str = "ProceduralEntityParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weather: match inst.get("Weather") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlanetWeatherParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            room: match inst.get("Room") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlanetRoomParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            atmosphere: match inst.get("Atmosphere") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlanetAtmosphereParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_params: match inst.get("AudioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralEntityAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AsteroidStateRef`
/// Inherits from: `AsteroidStateBase`
pub struct AsteroidStateRef {
    /// `stateTemplate` (Reference)
    pub state_template: Option<CigGuid>,
}

impl Pooled for AsteroidStateRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.asteroid_state_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.asteroid_state_ref }
}

impl<'a> Extract<'a> for AsteroidStateRef {
    const TYPE_NAME: &'static str = "AsteroidStateRef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_template: inst.get("stateTemplate").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AtmosphereStateMultiRef`
/// Inherits from: `AtmosphereStateBase`
pub struct AtmosphereStateMultiRef {
    /// `pressureTemplate` (Reference)
    pub pressure_template: Option<CigGuid>,
    /// `temperatureTemplate` (Reference)
    pub temperature_template: Option<CigGuid>,
    /// `humidityTemplate` (Reference)
    pub humidity_template: Option<CigGuid>,
}

impl Pooled for AtmosphereStateMultiRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.atmosphere_state_multi_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.atmosphere_state_multi_ref }
}

impl<'a> Extract<'a> for AtmosphereStateMultiRef {
    const TYPE_NAME: &'static str = "AtmosphereStateMultiRef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            pressure_template: inst.get("pressureTemplate").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            temperature_template: inst.get("temperatureTemplate").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            humidity_template: inst.get("humidityTemplate").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `RadiationState`
/// Inherits from: `RadiationStateBase`
pub struct RadiationState {
    /// `distortionMod` (EnumChoice)
    pub distortion_mod: RoomStateModifyType,
    /// `distortion` (Single)
    pub distortion: f32,
    /// `IR` (Class)
    pub ir: Option<Handle<RadiationStatePropertyParams>>,
    /// `EM` (Class)
    pub em: Option<Handle<RadiationStatePropertyParams>>,
    /// `CS` (Class)
    pub cs: Option<Handle<RadiationStatePropertyParams>>,
    /// `hazardousRadiationMod` (EnumChoice)
    pub hazardous_radiation_mod: RoomStateModifyType,
    /// `hazardousRadiationRate` (Single)
    pub hazardous_radiation_rate: f32,
}

impl Pooled for RadiationState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_environment.radiation_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_environment.radiation_state }
}

impl<'a> Extract<'a> for RadiationState {
    const TYPE_NAME: &'static str = "RadiationState";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            distortion_mod: RoomStateModifyType::from_dcb_str(inst.get_str("distortionMod").unwrap_or("")),
            distortion: inst.get_f32("distortion").unwrap_or_default(),
            ir: match inst.get("IR") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadiationStatePropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            em: match inst.get("EM") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadiationStatePropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            cs: match inst.get("CS") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RadiationStatePropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hazardous_radiation_mod: RoomStateModifyType::from_dcb_str(inst.get_str("hazardousRadiationMod").unwrap_or("")),
            hazardous_radiation_rate: inst.get_f32("hazardousRadiationRate").unwrap_or_default(),
        }
    }
}

