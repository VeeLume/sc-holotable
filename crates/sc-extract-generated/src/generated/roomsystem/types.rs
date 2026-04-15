// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `roomsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `FireHazardSurfaceProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardSurfaceProperties {
    /// `albedoTintStart` (Class)
    #[serde(default)]
    pub albedo_tint_start: Option<Handle<SRGBA8>>,
    /// `albedoTintEnd` (Class)
    #[serde(default)]
    pub albedo_tint_end: Option<Handle<SRGBA8>>,
    /// `specularTint` (Class)
    #[serde(default)]
    pub specular_tint: Option<Handle<SRGBA8>>,
    /// `smoothnessTint` (Class)
    #[serde(default)]
    pub smoothness_tint: Option<Handle<SRGBA8>>,
    /// `edgesIntensity` (Single)
    #[serde(default)]
    pub edges_intensity: f32,
    /// `embersIntensity` (Single)
    #[serde(default)]
    pub embers_intensity: f32,
    /// `burnSharpness` (Single)
    #[serde(default)]
    pub burn_sharpness: f32,
}

impl Pooled for FireHazardSurfaceProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_surface_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_surface_properties }
}

impl<'a> Extract<'a> for FireHazardSurfaceProperties {
    const TYPE_NAME: &'static str = "FireHazardSurfaceProperties";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            albedo_tint_start: match inst.get("albedoTintStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            albedo_tint_end: match inst.get("albedoTintEnd") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            specular_tint: match inst.get("specularTint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            smoothness_tint: match inst.get("smoothnessTint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            edges_intensity: inst.get_f32("edgesIntensity").unwrap_or_default(),
            embers_intensity: inst.get_f32("embersIntensity").unwrap_or_default(),
            burn_sharpness: inst.get_f32("burnSharpness").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardFireProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardFireProperties {
    /// `fireUnderlyingGlow` (Single)
    #[serde(default)]
    pub fire_underlying_glow: f32,
    /// `fireIntensityMultiplier` (Single)
    #[serde(default)]
    pub fire_intensity_multiplier: f32,
    /// `fireFlowmapMovementSpeed` (Single)
    #[serde(default)]
    pub fire_flowmap_movement_speed: f32,
    /// `fireSwirlSpeed` (Single)
    #[serde(default)]
    pub fire_swirl_speed: f32,
    /// `fireMovementSpeed` (Single)
    #[serde(default)]
    pub fire_movement_speed: f32,
    /// `maximumLuminance` (Single)
    #[serde(default)]
    pub maximum_luminance: f32,
}

impl Pooled for FireHazardFireProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_fire_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_fire_properties }
}

impl<'a> Extract<'a> for FireHazardFireProperties {
    const TYPE_NAME: &'static str = "FireHazardFireProperties";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fire_underlying_glow: inst.get_f32("fireUnderlyingGlow").unwrap_or_default(),
            fire_intensity_multiplier: inst.get_f32("fireIntensityMultiplier").unwrap_or_default(),
            fire_flowmap_movement_speed: inst.get_f32("fireFlowmapMovementSpeed").unwrap_or_default(),
            fire_swirl_speed: inst.get_f32("fireSwirlSpeed").unwrap_or_default(),
            fire_movement_speed: inst.get_f32("fireMovementSpeed").unwrap_or_default(),
            maximum_luminance: inst.get_f32("maximumLuminance").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardAfterglowProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardAfterglowProperties {
    /// `maximumTemperature` (Single)
    #[serde(default)]
    pub maximum_temperature: f32,
    /// `temperatureMultiplier` (Single)
    #[serde(default)]
    pub temperature_multiplier: f32,
    /// `burnMaskEdgesStart` (Single)
    #[serde(default)]
    pub burn_mask_edges_start: f32,
    /// `burnMaskEdgesEnd` (Single)
    #[serde(default)]
    pub burn_mask_edges_end: f32,
    /// `incandescenceStrength` (Single)
    #[serde(default)]
    pub incandescence_strength: f32,
    /// `edgesGlowStrength` (Single)
    #[serde(default)]
    pub edges_glow_strength: f32,
    /// `edgesPerimeterStrength` (Single)
    #[serde(default)]
    pub edges_perimeter_strength: f32,
    /// `edgesEmbersStrength` (Single)
    #[serde(default)]
    pub edges_embers_strength: f32,
    /// `maximumLuminance` (Single)
    #[serde(default)]
    pub maximum_luminance: f32,
}

impl Pooled for FireHazardAfterglowProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_afterglow_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_afterglow_properties }
}

impl<'a> Extract<'a> for FireHazardAfterglowProperties {
    const TYPE_NAME: &'static str = "FireHazardAfterglowProperties";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            maximum_temperature: inst.get_f32("maximumTemperature").unwrap_or_default(),
            temperature_multiplier: inst.get_f32("temperatureMultiplier").unwrap_or_default(),
            burn_mask_edges_start: inst.get_f32("burnMaskEdgesStart").unwrap_or_default(),
            burn_mask_edges_end: inst.get_f32("burnMaskEdgesEnd").unwrap_or_default(),
            incandescence_strength: inst.get_f32("incandescenceStrength").unwrap_or_default(),
            edges_glow_strength: inst.get_f32("edgesGlowStrength").unwrap_or_default(),
            edges_perimeter_strength: inst.get_f32("edgesPerimeterStrength").unwrap_or_default(),
            edges_embers_strength: inst.get_f32("edgesEmbersStrength").unwrap_or_default(),
            maximum_luminance: inst.get_f32("maximumLuminance").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardPermanentEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardPermanentEffects {
    /// `burnSize` (Single)
    #[serde(default)]
    pub burn_size: f32,
    /// `heightmapSize` (Single)
    #[serde(default)]
    pub heightmap_size: f32,
    /// `fireSize` (Single)
    #[serde(default)]
    pub fire_size: f32,
    /// `triplanarDitherRepeat` (Single)
    #[serde(default)]
    pub triplanar_dither_repeat: f32,
    /// `triplanarDitherMaxAngle` (Single)
    #[serde(default)]
    pub triplanar_dither_max_angle: f32,
    /// `breakupSize` (Single)
    #[serde(default)]
    pub breakup_size: f32,
    /// `fire` (Class)
    #[serde(default)]
    pub fire: Option<Handle<FireHazardFireProperties>>,
    /// `afterglow` (Class)
    #[serde(default)]
    pub afterglow: Option<Handle<FireHazardAfterglowProperties>>,
    /// `surfaces` (Class)
    #[serde(default)]
    pub surfaces: Option<Handle<FireHazardSurfaceProperties>>,
}

impl Pooled for FireHazardPermanentEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_permanent_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_permanent_effects }
}

impl<'a> Extract<'a> for FireHazardPermanentEffects {
    const TYPE_NAME: &'static str = "FireHazardPermanentEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            burn_size: inst.get_f32("burnSize").unwrap_or_default(),
            heightmap_size: inst.get_f32("heightmapSize").unwrap_or_default(),
            fire_size: inst.get_f32("fireSize").unwrap_or_default(),
            triplanar_dither_repeat: inst.get_f32("triplanarDitherRepeat").unwrap_or_default(),
            triplanar_dither_max_angle: inst.get_f32("triplanarDitherMaxAngle").unwrap_or_default(),
            breakup_size: inst.get_f32("breakupSize").unwrap_or_default(),
            fire: match inst.get("fire") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardFireProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            afterglow: match inst.get("afterglow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardAfterglowProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            surfaces: match inst.get("surfaces") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardSurfaceProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FireHazardSpawnParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardSpawnParams {
    /// `floorMaxAngle` (Single)
    #[serde(default)]
    pub floor_max_angle: f32,
    /// `wallsMaxAngle` (Single)
    #[serde(default)]
    pub walls_max_angle: f32,
    /// `ceilingMaxAngle` (Single)
    #[serde(default)]
    pub ceiling_max_angle: f32,
    /// `lodTransitionDistance` (Single)
    #[serde(default)]
    pub lod_transition_distance: f32,
    /// `lodEffectReferenceVolume` (Single)
    #[serde(default)]
    pub lod_effect_reference_volume: f32,
}

impl Pooled for FireHazardSpawnParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_spawn_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_spawn_params }
}

impl<'a> Extract<'a> for FireHazardSpawnParams {
    const TYPE_NAME: &'static str = "FireHazardSpawnParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            floor_max_angle: inst.get_f32("floorMaxAngle").unwrap_or_default(),
            walls_max_angle: inst.get_f32("wallsMaxAngle").unwrap_or_default(),
            ceiling_max_angle: inst.get_f32("ceilingMaxAngle").unwrap_or_default(),
            lod_transition_distance: inst.get_f32("lodTransitionDistance").unwrap_or_default(),
            lod_effect_reference_volume: inst.get_f32("lodEffectReferenceVolume").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardFogNoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardFogNoiseParams {
    /// `noiseFrequency` (Class)
    #[serde(default)]
    pub noise_frequency: Option<Handle<Vec3>>,
    /// `noiseScale` (Single)
    #[serde(default)]
    pub noise_scale: f32,
    /// `noiseSpeed` (Single)
    #[serde(default)]
    pub noise_speed: f32,
    /// `noiseTexture` (EnumChoice)
    #[serde(default)]
    pub noise_texture: FireHazardFogNoiseTextures,
}

impl Pooled for FireHazardFogNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_fog_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_fog_noise_params }
}

impl<'a> Extract<'a> for FireHazardFogNoiseParams {
    const TYPE_NAME: &'static str = "FireHazardFogNoiseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            noise_frequency: match inst.get("noiseFrequency") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            noise_scale: inst.get_f32("noiseScale").unwrap_or_default(),
            noise_speed: inst.get_f32("noiseSpeed").unwrap_or_default(),
            noise_texture: FireHazardFogNoiseTextures::from_dcb_str(inst.get_str("noiseTexture").unwrap_or("")),
        }
    }
}

/// DCB type: `FireHazardFogParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardFogParams {
    /// `softEdges` (Single)
    #[serde(default)]
    pub soft_edges: f32,
    /// `densityOffset` (Single)
    #[serde(default)]
    pub density_offset: f32,
    /// `densityMultiplier` (Single)
    #[serde(default)]
    pub density_multiplier: f32,
    /// `maximumDistance` (Single)
    #[serde(default)]
    pub maximum_distance: f32,
    /// `saturationDensity` (Single)
    #[serde(default)]
    pub saturation_density: f32,
    /// `falloffSharpness` (Single)
    #[serde(default)]
    pub falloff_sharpness: f32,
    /// `noiseVolumeOffset` (Single)
    #[serde(default)]
    pub noise_volume_offset: f32,
    /// `noise` (Class)
    #[serde(default)]
    pub noise: Option<Handle<FireHazardFogNoiseParams>>,
}

impl Pooled for FireHazardFogParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_fog_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_fog_params }
}

impl<'a> Extract<'a> for FireHazardFogParams {
    const TYPE_NAME: &'static str = "FireHazardFogParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            soft_edges: inst.get_f32("softEdges").unwrap_or_default(),
            density_offset: inst.get_f32("densityOffset").unwrap_or_default(),
            density_multiplier: inst.get_f32("densityMultiplier").unwrap_or_default(),
            maximum_distance: inst.get_f32("maximumDistance").unwrap_or_default(),
            saturation_density: inst.get_f32("saturationDensity").unwrap_or_default(),
            falloff_sharpness: inst.get_f32("falloffSharpness").unwrap_or_default(),
            noise_volume_offset: inst.get_f32("noiseVolumeOffset").unwrap_or_default(),
            noise: match inst.get("noise") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardFogNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FireHazardGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalParams {
    /// `update` (Class)
    #[serde(default)]
    pub update: Option<Handle<FireHazardGlobalUpdate>>,
    /// `ignition` (Class)
    #[serde(default)]
    pub ignition: Option<Handle<FireHazardGlobalIgnition>>,
    /// `propagation` (Class)
    #[serde(default)]
    pub propagation: Option<Handle<FireHazardGlobalPropagation>>,
    /// `smoke` (Class)
    #[serde(default)]
    pub smoke: Option<Handle<FireHazardGlobalSmokeParams>>,
    /// `damageToHealth` (Class)
    #[serde(default)]
    pub damage_to_health: Option<Handle<FireHazardGlobalDamageToHealthParams>>,
    /// `extinguishing` (Class)
    #[serde(default)]
    pub extinguishing: Option<Handle<FireHazardGlobalExtinguishing>>,
    /// `defaultEffects` (Class)
    #[serde(default)]
    pub default_effects: Option<Handle<FireHazardGlobalDefaultEffects>>,
    /// `lightParams` (Class)
    #[serde(default)]
    pub light_params: Option<Handle<FireHazardGlobalLightParams>>,
    /// `roomConnector` (Class)
    #[serde(default)]
    pub room_connector: Option<Handle<FireHazardGlobalRoomConnectorParams>>,
    /// `particleSpawning` (Class)
    #[serde(default)]
    pub particle_spawning: Option<Handle<FireHazardSpawnParams>>,
    /// `fog` (Class)
    #[serde(default)]
    pub fog: Option<Handle<FireHazardFogParams>>,
    /// `permanentEffects` (Class)
    #[serde(default)]
    pub permanent_effects: Option<Handle<FireHazardPermanentEffects>>,
}

impl Pooled for FireHazardGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_params }
}

impl<'a> Extract<'a> for FireHazardGlobalParams {
    const TYPE_NAME: &'static str = "FireHazardGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            update: match inst.get("update") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalUpdate>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            ignition: match inst.get("ignition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalIgnition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            propagation: match inst.get("propagation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalPropagation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            smoke: match inst.get("smoke") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalSmokeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            damage_to_health: match inst.get("damageToHealth") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalDamageToHealthParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            extinguishing: match inst.get("extinguishing") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalExtinguishing>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            default_effects: match inst.get("defaultEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalDefaultEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_params: match inst.get("lightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            room_connector: match inst.get("roomConnector") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalRoomConnectorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            particle_spawning: match inst.get("particleSpawning") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardSpawnParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            fog: match inst.get("fog") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardFogParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            permanent_effects: match inst.get("permanentEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardPermanentEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FireHazardGlobalUpdate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalUpdate {
    /// `fixedFPS` (Int32)
    #[serde(default)]
    pub fixed_fps: i32,
}

impl Pooled for FireHazardGlobalUpdate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_update }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_update }
}

impl<'a> Extract<'a> for FireHazardGlobalUpdate {
    const TYPE_NAME: &'static str = "FireHazardGlobalUpdate";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fixed_fps: inst.get_i32("fixedFPS").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardGlobalIgnition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalIgnition {
    /// `globalIgnitionChanceMultiplier` (Single)
    #[serde(default)]
    pub global_ignition_chance_multiplier: f32,
    /// `explosionChanceMultiplier` (Single)
    #[serde(default)]
    pub explosion_chance_multiplier: f32,
    /// `projectileChanceMultiplier` (Single)
    #[serde(default)]
    pub projectile_chance_multiplier: f32,
    /// `collisionChanceMultiplier` (Single)
    #[serde(default)]
    pub collision_chance_multiplier: f32,
    /// `fallbackChanceMultiplier` (Single)
    #[serde(default)]
    pub fallback_chance_multiplier: f32,
    /// `globalFlashIgnitionThresholdMultiplier` (Single)
    #[serde(default)]
    pub global_flash_ignition_threshold_multiplier: f32,
    /// `damageTypeIgnitionModifiers` (Single)
    #[serde(default)]
    pub damage_type_ignition_modifiers: f32,
}

impl Pooled for FireHazardGlobalIgnition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_ignition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_ignition }
}

impl<'a> Extract<'a> for FireHazardGlobalIgnition {
    const TYPE_NAME: &'static str = "FireHazardGlobalIgnition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            global_ignition_chance_multiplier: inst.get_f32("globalIgnitionChanceMultiplier").unwrap_or_default(),
            explosion_chance_multiplier: inst.get_f32("explosionChanceMultiplier").unwrap_or_default(),
            projectile_chance_multiplier: inst.get_f32("projectileChanceMultiplier").unwrap_or_default(),
            collision_chance_multiplier: inst.get_f32("collisionChanceMultiplier").unwrap_or_default(),
            fallback_chance_multiplier: inst.get_f32("fallbackChanceMultiplier").unwrap_or_default(),
            global_flash_ignition_threshold_multiplier: inst.get_f32("globalFlashIgnitionThresholdMultiplier").unwrap_or_default(),
            damage_type_ignition_modifiers: inst.get_f32("damageTypeIgnitionModifiers").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardGlobalPropagation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalPropagation {
    /// `fireMassThreshold` (Single)
    #[serde(default)]
    pub fire_mass_threshold: f32,
    /// `radiativeFactor` (Single)
    #[serde(default)]
    pub radiative_factor: f32,
    /// `minimumHeatIntensityOutput` (Single)
    #[serde(default)]
    pub minimum_heat_intensity_output: f32,
    /// `maximumRadiationRadius` (Single)
    #[serde(default)]
    pub maximum_radiation_radius: f32,
    /// `maximumConvectionDistance` (Single)
    #[serde(default)]
    pub maximum_convection_distance: f32,
    /// `standardConvectionTemperature` (Single)
    #[serde(default)]
    pub standard_convection_temperature: f32,
    /// `surfaceAreaBurnRateCurveVoxels` (Single)
    #[serde(default)]
    pub surface_area_burn_rate_curve_voxels: f32,
    /// `surfaceAreaBurnRateCurveEntities` (Single)
    #[serde(default)]
    pub surface_area_burn_rate_curve_entities: f32,
    /// `oxygenPressureRange` (Class)
    #[serde(default)]
    pub oxygen_pressure_range: Option<Handle<Range>>,
}

impl Pooled for FireHazardGlobalPropagation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_propagation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_propagation }
}

impl<'a> Extract<'a> for FireHazardGlobalPropagation {
    const TYPE_NAME: &'static str = "FireHazardGlobalPropagation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fire_mass_threshold: inst.get_f32("fireMassThreshold").unwrap_or_default(),
            radiative_factor: inst.get_f32("radiativeFactor").unwrap_or_default(),
            minimum_heat_intensity_output: inst.get_f32("minimumHeatIntensityOutput").unwrap_or_default(),
            maximum_radiation_radius: inst.get_f32("maximumRadiationRadius").unwrap_or_default(),
            maximum_convection_distance: inst.get_f32("maximumConvectionDistance").unwrap_or_default(),
            standard_convection_temperature: inst.get_f32("standardConvectionTemperature").unwrap_or_default(),
            surface_area_burn_rate_curve_voxels: inst.get_f32("surfaceAreaBurnRateCurveVoxels").unwrap_or_default(),
            surface_area_burn_rate_curve_entities: inst.get_f32("surfaceAreaBurnRateCurveEntities").unwrap_or_default(),
            oxygen_pressure_range: match inst.get("oxygenPressureRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FireHazardGlobalSmokeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalSmokeParams {
    /// `defaultSmokeComposition` (Reference)
    #[serde(default)]
    pub default_smoke_composition: Option<CigGuid>,
    /// `dynamicPart` (Single)
    #[serde(default)]
    pub dynamic_part: f32,
    /// `particulateMatter` (Reference)
    #[serde(default)]
    pub particulate_matter: Option<CigGuid>,
    /// `waterVapor` (Reference)
    #[serde(default)]
    pub water_vapor: Option<CigGuid>,
}

impl Pooled for FireHazardGlobalSmokeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_smoke_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_smoke_params }
}

impl<'a> Extract<'a> for FireHazardGlobalSmokeParams {
    const TYPE_NAME: &'static str = "FireHazardGlobalSmokeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_smoke_composition: inst.get("defaultSmokeComposition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            dynamic_part: inst.get_f32("dynamicPart").unwrap_or_default(),
            particulate_matter: inst.get("particulateMatter").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            water_vapor: inst.get("waterVapor").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `FireHazardGlobalDamageToHealthParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalDamageToHealthParams {
    /// `minimumDamageTemperature` (Single)
    #[serde(default)]
    pub minimum_damage_temperature: f32,
    /// `baseDamage` (Single)
    #[serde(default)]
    pub base_damage: f32,
    /// `referenceHealth` (Single)
    #[serde(default)]
    pub reference_health: f32,
    /// `curveAngle` (Single)
    #[serde(default)]
    pub curve_angle: f32,
    /// `curveOffset` (Single)
    #[serde(default)]
    pub curve_offset: f32,
}

impl Pooled for FireHazardGlobalDamageToHealthParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_damage_to_health_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_damage_to_health_params }
}

impl<'a> Extract<'a> for FireHazardGlobalDamageToHealthParams {
    const TYPE_NAME: &'static str = "FireHazardGlobalDamageToHealthParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            minimum_damage_temperature: inst.get_f32("minimumDamageTemperature").unwrap_or_default(),
            base_damage: inst.get_f32("baseDamage").unwrap_or_default(),
            reference_health: inst.get_f32("referenceHealth").unwrap_or_default(),
            curve_angle: inst.get_f32("curveAngle").unwrap_or_default(),
            curve_offset: inst.get_f32("curveOffset").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardGlobalExtinguishing`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalExtinguishing {
    /// `oxygenReduction` (Single)
    #[serde(default)]
    pub oxygen_reduction: f32,
    /// `heatTransferMultiplier` (Single)
    #[serde(default)]
    pub heat_transfer_multiplier: f32,
    /// `radiationAbsorption` (Single)
    #[serde(default)]
    pub radiation_absorption: f32,
    /// `vectorFieldPathSpray` (String)
    #[serde(default)]
    pub vector_field_path_spray: String,
    /// `vectorFieldPathSphere` (String)
    #[serde(default)]
    pub vector_field_path_sphere: String,
}

impl Pooled for FireHazardGlobalExtinguishing {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_extinguishing }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_extinguishing }
}

impl<'a> Extract<'a> for FireHazardGlobalExtinguishing {
    const TYPE_NAME: &'static str = "FireHazardGlobalExtinguishing";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            oxygen_reduction: inst.get_f32("oxygenReduction").unwrap_or_default(),
            heat_transfer_multiplier: inst.get_f32("heatTransferMultiplier").unwrap_or_default(),
            radiation_absorption: inst.get_f32("radiationAbsorption").unwrap_or_default(),
            vector_field_path_spray: inst.get_str("vectorFieldPathSpray").map(String::from).unwrap_or_default(),
            vector_field_path_sphere: inst.get_str("vectorFieldPathSphere").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardGlobalDefaultEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalDefaultEffects {
    /// `voxelFireEffect` (String)
    #[serde(default)]
    pub voxel_fire_effect: String,
    /// `voxelFlashIgnitionEffect` (Class)
    #[serde(default)]
    pub voxel_flash_ignition_effect: Option<Handle<GlobalResourceParticle>>,
    /// `entityFireEffect` (Class)
    #[serde(default)]
    pub entity_fire_effect: Option<Handle<GlobalResourceParticle>>,
    /// `entitySmokeEffect` (Class)
    #[serde(default)]
    pub entity_smoke_effect: Option<Handle<GlobalResourceParticle>>,
    /// `defaultSprayExtinguishingEffect` (Class)
    #[serde(default)]
    pub default_spray_extinguishing_effect: Option<Handle<GlobalResourceParticle>>,
    /// `defaultSphereExtinguishingEffect` (Class)
    #[serde(default)]
    pub default_sphere_extinguishing_effect: Option<Handle<GlobalResourceParticle>>,
    /// `maximumSmokeEmission` (Single)
    #[serde(default)]
    pub maximum_smoke_emission: f32,
    /// `entityReferenceSize` (Single)
    #[serde(default)]
    pub entity_reference_size: f32,
    /// `entityMinimumSize` (Single)
    #[serde(default)]
    pub entity_minimum_size: f32,
}

impl Pooled for FireHazardGlobalDefaultEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_default_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_default_effects }
}

impl<'a> Extract<'a> for FireHazardGlobalDefaultEffects {
    const TYPE_NAME: &'static str = "FireHazardGlobalDefaultEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            voxel_fire_effect: inst.get_str("voxelFireEffect").map(String::from).unwrap_or_default(),
            voxel_flash_ignition_effect: match inst.get("voxelFlashIgnitionEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            entity_fire_effect: match inst.get("entityFireEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            entity_smoke_effect: match inst.get("entitySmokeEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            default_spray_extinguishing_effect: match inst.get("defaultSprayExtinguishingEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            default_sphere_extinguishing_effect: match inst.get("defaultSphereExtinguishingEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            maximum_smoke_emission: inst.get_f32("maximumSmokeEmission").unwrap_or_default(),
            entity_reference_size: inst.get_f32("entityReferenceSize").unwrap_or_default(),
            entity_minimum_size: inst.get_f32("entityMinimumSize").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardGlobalLightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalLightParams {
    /// `lightColor` (Class)
    #[serde(default)]
    pub light_color: Option<Handle<RGB>>,
    /// `maxLights` (Int32)
    #[serde(default)]
    pub max_lights: i32,
    /// `teleportCooldown` (Single)
    #[serde(default)]
    pub teleport_cooldown: f32,
    /// `selectionRadius` (Single)
    #[serde(default)]
    pub selection_radius: f32,
    /// `animMoveSpeed` (Single)
    #[serde(default)]
    pub anim_move_speed: f32,
    /// `animGrowSpeed` (Single)
    #[serde(default)]
    pub anim_grow_speed: f32,
    /// `animShrinkSpeed` (Single)
    #[serde(default)]
    pub anim_shrink_speed: f32,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `bulbRadiusRange` (Class)
    #[serde(default)]
    pub bulb_radius_range: Option<Handle<Range>>,
    /// `lightIntensityRange` (Class)
    #[serde(default)]
    pub light_intensity_range: Option<Handle<Range>>,
    /// `specularMultiplier` (Single)
    #[serde(default)]
    pub specular_multiplier: f32,
    /// `fogMultiplier` (Single)
    #[serde(default)]
    pub fog_multiplier: f32,
    /// `fogOcclusionFalloff` (Single)
    #[serde(default)]
    pub fog_occlusion_falloff: f32,
    /// `lightAnimStyle` (Byte)
    #[serde(default)]
    pub light_anim_style: u32,
    /// `lightAnimSpeed` (Single)
    #[serde(default)]
    pub light_anim_speed: f32,
    /// `softness` (Single)
    #[serde(default)]
    pub softness: f32,
    /// `maxHeatReleaseRate` (Single)
    #[serde(default)]
    pub max_heat_release_rate: f32,
    /// `lightTemperature` (Single)
    #[serde(default)]
    pub light_temperature: f32,
    /// `useTemperature` (Boolean)
    #[serde(default)]
    pub use_temperature: bool,
    /// `affectsFog` (Boolean)
    #[serde(default)]
    pub affects_fog: bool,
    /// `shadowMap` (Boolean)
    #[serde(default)]
    pub shadow_map: bool,
}

impl Pooled for FireHazardGlobalLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_light_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_light_params }
}

impl<'a> Extract<'a> for FireHazardGlobalLightParams {
    const TYPE_NAME: &'static str = "FireHazardGlobalLightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_color: match inst.get("lightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            max_lights: inst.get_i32("maxLights").unwrap_or_default(),
            teleport_cooldown: inst.get_f32("teleportCooldown").unwrap_or_default(),
            selection_radius: inst.get_f32("selectionRadius").unwrap_or_default(),
            anim_move_speed: inst.get_f32("animMoveSpeed").unwrap_or_default(),
            anim_grow_speed: inst.get_f32("animGrowSpeed").unwrap_or_default(),
            anim_shrink_speed: inst.get_f32("animShrinkSpeed").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            bulb_radius_range: match inst.get("bulbRadiusRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            light_intensity_range: match inst.get("lightIntensityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            specular_multiplier: inst.get_f32("specularMultiplier").unwrap_or_default(),
            fog_multiplier: inst.get_f32("fogMultiplier").unwrap_or_default(),
            fog_occlusion_falloff: inst.get_f32("fogOcclusionFalloff").unwrap_or_default(),
            light_anim_style: inst.get_u32("lightAnimStyle").unwrap_or_default(),
            light_anim_speed: inst.get_f32("lightAnimSpeed").unwrap_or_default(),
            softness: inst.get_f32("softness").unwrap_or_default(),
            max_heat_release_rate: inst.get_f32("maxHeatReleaseRate").unwrap_or_default(),
            light_temperature: inst.get_f32("lightTemperature").unwrap_or_default(),
            use_temperature: inst.get_bool("useTemperature").unwrap_or_default(),
            affects_fog: inst.get_bool("affectsFog").unwrap_or_default(),
            shadow_map: inst.get_bool("shadowMap").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardGlobalRoomConnectorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalRoomConnectorParams {
    /// `igniteThreshold` (Single)
    #[serde(default)]
    pub ignite_threshold: f32,
    /// `maxVolumeRadius` (Single)
    #[serde(default)]
    pub max_volume_radius: f32,
    /// `vectorFieldPath` (String)
    #[serde(default)]
    pub vector_field_path: String,
    /// `vectorFieldRotation` (Single)
    #[serde(default)]
    pub vector_field_rotation: f32,
    /// `vectorFieldRadius` (Single)
    #[serde(default)]
    pub vector_field_radius: f32,
    /// `vectorFieldFalloff` (Single)
    #[serde(default)]
    pub vector_field_falloff: f32,
    /// `vectorFieldThreshold` (Single)
    #[serde(default)]
    pub vector_field_threshold: f32,
    /// `vectorFieldMaxHeatReleaseRateDiff` (Single)
    #[serde(default)]
    pub vector_field_max_heat_release_rate_diff: f32,
    /// `vectorFieldMaxStrength` (Single)
    #[serde(default)]
    pub vector_field_max_strength: f32,
    /// `vectorFieldDepth` (Single)
    #[serde(default)]
    pub vector_field_depth: f32,
}

impl Pooled for FireHazardGlobalRoomConnectorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.fire_hazard_global_room_connector_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.fire_hazard_global_room_connector_params }
}

impl<'a> Extract<'a> for FireHazardGlobalRoomConnectorParams {
    const TYPE_NAME: &'static str = "FireHazardGlobalRoomConnectorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            ignite_threshold: inst.get_f32("igniteThreshold").unwrap_or_default(),
            max_volume_radius: inst.get_f32("maxVolumeRadius").unwrap_or_default(),
            vector_field_path: inst.get_str("vectorFieldPath").map(String::from).unwrap_or_default(),
            vector_field_rotation: inst.get_f32("vectorFieldRotation").unwrap_or_default(),
            vector_field_radius: inst.get_f32("vectorFieldRadius").unwrap_or_default(),
            vector_field_falloff: inst.get_f32("vectorFieldFalloff").unwrap_or_default(),
            vector_field_threshold: inst.get_f32("vectorFieldThreshold").unwrap_or_default(),
            vector_field_max_heat_release_rate_diff: inst.get_f32("vectorFieldMaxHeatReleaseRateDiff").unwrap_or_default(),
            vector_field_max_strength: inst.get_f32("vectorFieldMaxStrength").unwrap_or_default(),
            vector_field_depth: inst.get_f32("vectorFieldDepth").unwrap_or_default(),
        }
    }
}

/// DCB type: `LightningBehavior`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningBehavior {
    /// `effects` (Class (array))
    #[serde(default)]
    pub effects: Vec<Handle<LightningBehavior_Effect>>,
}

impl Pooled for LightningBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.lightning_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.lightning_behavior }
}

impl<'a> Extract<'a> for LightningBehavior {
    const TYPE_NAME: &'static str = "LightningBehavior";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            effects: inst.get_array("effects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LightningBehavior_Effect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<LightningBehavior_Effect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LightningBehavior_Effect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningBehavior_Effect {
    /// `lightningEffect` (String)
    #[serde(default)]
    pub lightning_effect: String,
    /// `targetModes` (StrongPointer (array))
    #[serde(default)]
    pub target_modes: Vec<LightningTargetModePtr>,
    /// `audio` (Class)
    #[serde(default)]
    pub audio: Option<Handle<LightningStrikeAudio>>,
}

impl Pooled for LightningBehavior_Effect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.lightning_behavior_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.lightning_behavior_effect }
}

impl<'a> Extract<'a> for LightningBehavior_Effect {
    const TYPE_NAME: &'static str = "LightningBehavior_Effect";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lightning_effect: inst.get_str("lightningEffect").map(String::from).unwrap_or_default(),
            target_modes: inst.get_array("targetModes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(LightningTargetModePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            audio: match inst.get("audio") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LightningStrikeAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LightningTargetMode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningTargetMode {
    /// `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
}

impl Pooled for LightningTargetMode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.lightning_target_mode }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.lightning_target_mode }
}

impl<'a> Extract<'a> for LightningTargetMode {
    const TYPE_NAME: &'static str = "LightningTargetMode";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
        }
    }
}

/// DCB type: `ApparentTemperatureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApparentTemperatureParams {
    /// `heatIndexBlendThreshold` (Double)
    #[serde(default)]
    pub heat_index_blend_threshold: f64,
    /// `heatIndexHardThreshold` (Double)
    #[serde(default)]
    pub heat_index_hard_threshold: f64,
    /// `windChillBlendThreshold` (Double)
    #[serde(default)]
    pub wind_chill_blend_threshold: f64,
    /// `windChillHardThreshold` (Double)
    #[serde(default)]
    pub wind_chill_hard_threshold: f64,
    /// `maxPressureForScaling` (Double)
    #[serde(default)]
    pub max_pressure_for_scaling: f64,
    /// `defaultTemperatureForPressureScaling` (Double)
    #[serde(default)]
    pub default_temperature_for_pressure_scaling: f64,
}

impl Pooled for ApparentTemperatureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.apparent_temperature_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.apparent_temperature_params }
}

impl<'a> Extract<'a> for ApparentTemperatureParams {
    const TYPE_NAME: &'static str = "ApparentTemperatureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            heat_index_blend_threshold: inst.get_f64("heatIndexBlendThreshold").unwrap_or_default(),
            heat_index_hard_threshold: inst.get_f64("heatIndexHardThreshold").unwrap_or_default(),
            wind_chill_blend_threshold: inst.get_f64("windChillBlendThreshold").unwrap_or_default(),
            wind_chill_hard_threshold: inst.get_f64("windChillHardThreshold").unwrap_or_default(),
            max_pressure_for_scaling: inst.get_f64("maxPressureForScaling").unwrap_or_default(),
            default_temperature_for_pressure_scaling: inst.get_f64("defaultTemperatureForPressureScaling").unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalGasParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalGasParams {
    /// `transferRatePerAtmosphereSquareMetre` (Double)
    #[serde(default)]
    pub transfer_rate_per_atmosphere_square_metre: f64,
    /// `minimumTransferRate` (Double)
    #[serde(default)]
    pub minimum_transfer_rate: f64,
    /// `mixRatePerSquareMetre` (Double)
    #[serde(default)]
    pub mix_rate_per_square_metre: f64,
    /// `mixAbsPressureDifferenceThreshold` (Double)
    #[serde(default)]
    pub mix_abs_pressure_difference_threshold: f64,
    /// `massAdditionRatePerCubicMetre_MatchingGases` (Double)
    #[serde(default)]
    pub mass_addition_rate_per_cubic_metre_matching_gases: f64,
    /// `massSubtractionRatePerCubicMetre_MatchingGases` (Double)
    #[serde(default)]
    pub mass_subtraction_rate_per_cubic_metre_matching_gases: f64,
    /// `massSubtractionRatePerCubicMetre_ForeignGases` (Double)
    #[serde(default)]
    pub mass_subtraction_rate_per_cubic_metre_foreign_gases: f64,
    /// `resourceCostPerKilogramCorrected` (Double)
    #[serde(default)]
    pub resource_cost_per_kilogram_corrected: f64,
    /// `thermalEnergyCorrectionRatePerCubicMetre` (Double)
    #[serde(default)]
    pub thermal_energy_correction_rate_per_cubic_metre: f64,
    /// `resourceCostPerJoule` (Double)
    #[serde(default)]
    pub resource_cost_per_joule: f64,
    /// `humidityCorrectionRate` (Double)
    #[serde(default)]
    pub humidity_correction_rate: f64,
    /// `resourceCostPerHumidity` (Double)
    #[serde(default)]
    pub resource_cost_per_humidity: f64,
    /// `apparentTemperatureParams` (Class)
    #[serde(default)]
    pub apparent_temperature_params: Option<Handle<ApparentTemperatureParams>>,
}

impl Pooled for GlobalGasParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.global_gas_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.global_gas_params }
}

impl<'a> Extract<'a> for GlobalGasParams {
    const TYPE_NAME: &'static str = "GlobalGasParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            transfer_rate_per_atmosphere_square_metre: inst.get_f64("transferRatePerAtmosphereSquareMetre").unwrap_or_default(),
            minimum_transfer_rate: inst.get_f64("minimumTransferRate").unwrap_or_default(),
            mix_rate_per_square_metre: inst.get_f64("mixRatePerSquareMetre").unwrap_or_default(),
            mix_abs_pressure_difference_threshold: inst.get_f64("mixAbsPressureDifferenceThreshold").unwrap_or_default(),
            mass_addition_rate_per_cubic_metre_matching_gases: inst.get_f64("massAdditionRatePerCubicMetre_MatchingGases").unwrap_or_default(),
            mass_subtraction_rate_per_cubic_metre_matching_gases: inst.get_f64("massSubtractionRatePerCubicMetre_MatchingGases").unwrap_or_default(),
            mass_subtraction_rate_per_cubic_metre_foreign_gases: inst.get_f64("massSubtractionRatePerCubicMetre_ForeignGases").unwrap_or_default(),
            resource_cost_per_kilogram_corrected: inst.get_f64("resourceCostPerKilogramCorrected").unwrap_or_default(),
            thermal_energy_correction_rate_per_cubic_metre: inst.get_f64("thermalEnergyCorrectionRatePerCubicMetre").unwrap_or_default(),
            resource_cost_per_joule: inst.get_f64("resourceCostPerJoule").unwrap_or_default(),
            humidity_correction_rate: inst.get_f64("humidityCorrectionRate").unwrap_or_default(),
            resource_cost_per_humidity: inst.get_f64("resourceCostPerHumidity").unwrap_or_default(),
            apparent_temperature_params: match inst.get("apparentTemperatureParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ApparentTemperatureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalRoomStateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalRoomStateParams {
    /// `typeRanges` (Class)
    #[serde(default)]
    pub type_ranges: Option<Handle<Range>>,
    /// `typeDebugColors` (Class)
    #[serde(default)]
    pub type_debug_colors: Option<Handle<RGB>>,
    /// `debugParticles` (Class)
    #[serde(default)]
    pub debug_particles: Option<Handle<GlobalResourceParticle>>,
    /// `defaultSpaceDust` (Class)
    #[serde(default)]
    pub default_space_dust: Option<Handle<WeatherEffects_SpaceLoopEffect>>,
}

impl Pooled for GlobalRoomStateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.global_room_state_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.global_room_state_params }
}

impl<'a> Extract<'a> for GlobalRoomStateParams {
    const TYPE_NAME: &'static str = "GlobalRoomStateParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            type_ranges: match inst.get("typeRanges") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            type_debug_colors: match inst.get("typeDebugColors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            debug_particles: match inst.get("debugParticles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            default_space_dust: match inst.get("defaultSpaceDust") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WeatherEffects_SpaceLoopEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ElectricalStateTemplateInternal`
/// Inherits from: `ElectricalState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalStateTemplateInternal {
    /// `chargeMod` (EnumChoice)
    #[serde(default)]
    pub charge_mod: RoomStateModifyType,
    /// `charge` (Single)
    #[serde(default)]
    pub charge: f32,
}

impl Pooled for ElectricalStateTemplateInternal {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.electrical_state_template_internal }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.electrical_state_template_internal }
}

impl<'a> Extract<'a> for ElectricalStateTemplateInternal {
    const TYPE_NAME: &'static str = "ElectricalStateTemplateInternal";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            charge_mod: RoomStateModifyType::from_dcb_str(inst.get_str("chargeMod").unwrap_or("")),
            charge: inst.get_f32("charge").unwrap_or_default(),
        }
    }
}

/// DCB type: `ElectricalStateTemplate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalStateTemplate {
    /// `state` (Class)
    #[serde(default)]
    pub state: Option<Handle<ElectricalStateTemplateInternal>>,
}

impl Pooled for ElectricalStateTemplate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.electrical_state_template }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.electrical_state_template }
}

impl<'a> Extract<'a> for ElectricalStateTemplate {
    const TYPE_NAME: &'static str = "ElectricalStateTemplate";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: match inst.get("state") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ElectricalStateTemplateInternal>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ElectricalCalculationPropertyRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalCalculationPropertyRange {
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: ElectricalCalculationPropertyType,
    /// `range` (Class)
    #[serde(default)]
    pub range: Option<Handle<Range>>,
}

impl Pooled for ElectricalCalculationPropertyRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.electrical_calculation_property_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.electrical_calculation_property_range }
}

impl<'a> Extract<'a> for ElectricalCalculationPropertyRange {
    const TYPE_NAME: &'static str = "ElectricalCalculationPropertyRange";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: ElectricalCalculationPropertyType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            range: match inst.get("range") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Behavior_ElectricalVehicleEffectParams`
/// Inherits from: `Behavior_VehicleEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior_ElectricalVehicleEffectParams {
    /// `enableEngineTrails` (Boolean)
    #[serde(default)]
    pub enable_engine_trails: bool,
    /// `enableEngineContrails` (Boolean)
    #[serde(default)]
    pub enable_engine_contrails: bool,
    /// `customVehicleEffects` (StrongPointer)
    #[serde(default)]
    pub custom_vehicle_effects: Option<Handle<Behavior_CustomVehicleEffectsPreset>>,
    /// `customVehicleCalculation` (Class)
    #[serde(default)]
    pub custom_vehicle_calculation: Option<Handle<ElectricalCalculationPropertyRange>>,
}

impl Pooled for Behavior_ElectricalVehicleEffectParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.behavior_electrical_vehicle_effect_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.behavior_electrical_vehicle_effect_params }
}

impl<'a> Extract<'a> for Behavior_ElectricalVehicleEffectParams {
    const TYPE_NAME: &'static str = "Behavior_ElectricalVehicleEffectParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_engine_trails: inst.get_bool("enableEngineTrails").unwrap_or_default(),
            enable_engine_contrails: inst.get_bool("enableEngineContrails").unwrap_or_default(),
            custom_vehicle_effects: match inst.get("customVehicleEffects") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Behavior_CustomVehicleEffectsPreset>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            custom_vehicle_calculation: match inst.get("customVehicleCalculation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ElectricalCalculationPropertyRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ElectricalBehavior`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalBehavior {
    /// `lightning` (Class)
    #[serde(default)]
    pub lightning: Option<Handle<LightningBehavior>>,
    /// `vehicleEffects` (Class)
    #[serde(default)]
    pub vehicle_effects: Option<Handle<Behavior_ElectricalVehicleEffectParams>>,
}

impl Pooled for ElectricalBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.electrical_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.electrical_behavior }
}

impl<'a> Extract<'a> for ElectricalBehavior {
    const TYPE_NAME: &'static str = "ElectricalBehavior";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lightning: match inst.get("lightning") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LightningBehavior>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            vehicle_effects: match inst.get("vehicleEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Behavior_ElectricalVehicleEffectParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadiationBehavior_AsteroidDesignCurveSurfaceRadiationParams`
/// Inherits from: `RadiationBehavior_SurfaceRadiationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationBehavior_AsteroidDesignCurveSurfaceRadiationParams {
    /// `scaleOnLargestAsteroid` (Single)
    #[serde(default)]
    pub scale_on_largest_asteroid: f32,
    /// `radiusRange` (Class)
    #[serde(default)]
    pub radius_range: Option<Handle<Range>>,
    /// `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for RadiationBehavior_AsteroidDesignCurveSurfaceRadiationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.radiation_behavior_asteroid_design_curve_surface_radiation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.radiation_behavior_asteroid_design_curve_surface_radiation_params }
}

impl<'a> Extract<'a> for RadiationBehavior_AsteroidDesignCurveSurfaceRadiationParams {
    const TYPE_NAME: &'static str = "RadiationBehavior_AsteroidDesignCurveSurfaceRadiationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scale_on_largest_asteroid: inst.get_f32("scaleOnLargestAsteroid").unwrap_or_default(),
            radius_range: match inst.get("radiusRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadiationBehavior_AsteroidInverseSquareSurfaceRadiationParams`
/// Inherits from: `RadiationBehavior_SurfaceRadiationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationBehavior_AsteroidInverseSquareSurfaceRadiationParams {
    /// `scaleOnLargestAsteroid` (Single)
    #[serde(default)]
    pub scale_on_largest_asteroid: f32,
    /// `radiusScale` (Single)
    #[serde(default)]
    pub radius_scale: f32,
    /// `intensityScale` (Single)
    #[serde(default)]
    pub intensity_scale: f32,
    /// `intensityCutoff` (Single)
    #[serde(default)]
    pub intensity_cutoff: f32,
}

impl Pooled for RadiationBehavior_AsteroidInverseSquareSurfaceRadiationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.radiation_behavior_asteroid_inverse_square_surface_radiation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.radiation_behavior_asteroid_inverse_square_surface_radiation_params }
}

impl<'a> Extract<'a> for RadiationBehavior_AsteroidInverseSquareSurfaceRadiationParams {
    const TYPE_NAME: &'static str = "RadiationBehavior_AsteroidInverseSquareSurfaceRadiationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            scale_on_largest_asteroid: inst.get_f32("scaleOnLargestAsteroid").unwrap_or_default(),
            radius_scale: inst.get_f32("radiusScale").unwrap_or_default(),
            intensity_scale: inst.get_f32("intensityScale").unwrap_or_default(),
            intensity_cutoff: inst.get_f32("intensityCutoff").unwrap_or_default(),
        }
    }
}

/// DCB type: `WeatherEffects_Atmosphere_Property`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEffects_Atmosphere_Property {
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: AtmosphereStatePropertyType,
    /// `interpolationRange` (Class)
    #[serde(default)]
    pub interpolation_range: Option<Handle<Range>>,
}

impl Pooled for WeatherEffects_Atmosphere_Property {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.weather_effects_atmosphere_property }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.weather_effects_atmosphere_property }
}

impl<'a> Extract<'a> for WeatherEffects_Atmosphere_Property {
    const TYPE_NAME: &'static str = "WeatherEffects_Atmosphere_Property";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: AtmosphereStatePropertyType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            interpolation_range: match inst.get("interpolationRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeatherEffects_Atmosphere_MultiPropertyValue`
/// Inherits from: `WeatherEffects_Atmosphere`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEffects_Atmosphere_MultiPropertyValue {
    /// `spaceLoopEffect` (Class)
    #[serde(default)]
    pub space_loop_effect: Option<Handle<WeatherEffects_SpaceLoopEffect>>,
    /// `causesPuddles` (Boolean)
    #[serde(default)]
    pub causes_puddles: bool,
    /// `properties` (Class (array))
    #[serde(default)]
    pub properties: Vec<Handle<WeatherEffects_Atmosphere_Property>>,
}

impl Pooled for WeatherEffects_Atmosphere_MultiPropertyValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.weather_effects_atmosphere_multi_property_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.weather_effects_atmosphere_multi_property_value }
}

impl<'a> Extract<'a> for WeatherEffects_Atmosphere_MultiPropertyValue {
    const TYPE_NAME: &'static str = "WeatherEffects_Atmosphere_MultiPropertyValue";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            space_loop_effect: match inst.get("spaceLoopEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WeatherEffects_SpaceLoopEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            causes_puddles: inst.get_bool("causesPuddles").unwrap_or_default(),
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<WeatherEffects_Atmosphere_Property>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<WeatherEffects_Atmosphere_Property>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `WeatherEffects_Atmosphere_GasCloudOpticalDensity`
/// Inherits from: `WeatherEffects_Atmosphere`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEffects_Atmosphere_GasCloudOpticalDensity {
    /// `spaceLoopEffect` (Class)
    #[serde(default)]
    pub space_loop_effect: Option<Handle<WeatherEffects_SpaceLoopEffect>>,
    /// `interpolationRange` (Class)
    #[serde(default)]
    pub interpolation_range: Option<Handle<Range>>,
}

impl Pooled for WeatherEffects_Atmosphere_GasCloudOpticalDensity {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.roomsystem.weather_effects_atmosphere_gas_cloud_optical_density }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.roomsystem.weather_effects_atmosphere_gas_cloud_optical_density }
}

impl<'a> Extract<'a> for WeatherEffects_Atmosphere_GasCloudOpticalDensity {
    const TYPE_NAME: &'static str = "WeatherEffects_Atmosphere_GasCloudOpticalDensity";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            space_loop_effect: match inst.get("spaceLoopEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WeatherEffects_SpaceLoopEffect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            interpolation_range: match inst.get("interpolationRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

