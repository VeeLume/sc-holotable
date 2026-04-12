// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `Fidget`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fidget {
    /// DCB field: `FragmentTags` (String (array))
    #[serde(default)]
    pub fragment_tags: Vec<String>,
    /// DCB field: `IntervalMin` (Single)
    #[serde(default)]
    pub interval_min: f32,
    /// DCB field: `IntervalMax` (Single)
    #[serde(default)]
    pub interval_max: f32,
}

impl Pooled for Fidget {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fidget }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fidget }
}

impl<'a> Extract<'a> for Fidget {
    const TYPE_NAME: &'static str = "Fidget";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fragment_tags: inst.get_array("FragmentTags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            interval_min: inst.get_f32("IntervalMin").unwrap_or_default(),
            interval_max: inst.get_f32("IntervalMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `FidgetConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FidgetConfig {
    /// DCB field: `FragmentId` (String)
    #[serde(default)]
    pub fragment_id: String,
    /// DCB field: `Fidgets` (Class (array))
    #[serde(default)]
    pub fidgets: Vec<Handle<Fidget>>,
    /// DCB field: `RepeatTime` (Single)
    #[serde(default)]
    pub repeat_time: f32,
    /// DCB field: `BreakTime` (Single)
    #[serde(default)]
    pub break_time: f32,
}

impl Pooled for FidgetConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fidget_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fidget_config }
}

impl<'a> Extract<'a> for FidgetConfig {
    const TYPE_NAME: &'static str = "FidgetConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fragment_id: inst.get_str("FragmentId").map(String::from).unwrap_or_default(),
            fidgets: inst.get_array("Fidgets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Fidget>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Fidget>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            repeat_time: inst.get_f32("RepeatTime").unwrap_or_default(),
            break_time: inst.get_f32("BreakTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardSurfaceProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardSurfaceProperties {
    /// DCB field: `albedoTintStart` (Class)
    #[serde(default)]
    pub albedo_tint_start: Option<Handle<SRGBA8>>,
    /// DCB field: `albedoTintEnd` (Class)
    #[serde(default)]
    pub albedo_tint_end: Option<Handle<SRGBA8>>,
    /// DCB field: `specularTint` (Class)
    #[serde(default)]
    pub specular_tint: Option<Handle<SRGBA8>>,
    /// DCB field: `smoothnessTint` (Class)
    #[serde(default)]
    pub smoothness_tint: Option<Handle<SRGBA8>>,
    /// DCB field: `edgesIntensity` (Single)
    #[serde(default)]
    pub edges_intensity: f32,
    /// DCB field: `embersIntensity` (Single)
    #[serde(default)]
    pub embers_intensity: f32,
    /// DCB field: `burnSharpness` (Single)
    #[serde(default)]
    pub burn_sharpness: f32,
}

impl Pooled for FireHazardSurfaceProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_surface_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_surface_properties }
}

impl<'a> Extract<'a> for FireHazardSurfaceProperties {
    const TYPE_NAME: &'static str = "FireHazardSurfaceProperties";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            albedo_tint_start: match inst.get("albedoTintStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            albedo_tint_end: match inst.get("albedoTintEnd") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            specular_tint: match inst.get("specularTint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            smoothness_tint: match inst.get("smoothnessTint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
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
    /// DCB field: `fireUnderlyingGlow` (Single)
    #[serde(default)]
    pub fire_underlying_glow: f32,
    /// DCB field: `fireIntensityMultiplier` (Single)
    #[serde(default)]
    pub fire_intensity_multiplier: f32,
    /// DCB field: `fireFlowmapMovementSpeed` (Single)
    #[serde(default)]
    pub fire_flowmap_movement_speed: f32,
    /// DCB field: `fireSwirlSpeed` (Single)
    #[serde(default)]
    pub fire_swirl_speed: f32,
    /// DCB field: `fireMovementSpeed` (Single)
    #[serde(default)]
    pub fire_movement_speed: f32,
    /// DCB field: `maximumLuminance` (Single)
    #[serde(default)]
    pub maximum_luminance: f32,
}

impl Pooled for FireHazardFireProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_fire_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_fire_properties }
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
    /// DCB field: `maximumTemperature` (Single)
    #[serde(default)]
    pub maximum_temperature: f32,
    /// DCB field: `temperatureMultiplier` (Single)
    #[serde(default)]
    pub temperature_multiplier: f32,
    /// DCB field: `burnMaskEdgesStart` (Single)
    #[serde(default)]
    pub burn_mask_edges_start: f32,
    /// DCB field: `burnMaskEdgesEnd` (Single)
    #[serde(default)]
    pub burn_mask_edges_end: f32,
    /// DCB field: `incandescenceStrength` (Single)
    #[serde(default)]
    pub incandescence_strength: f32,
    /// DCB field: `edgesGlowStrength` (Single)
    #[serde(default)]
    pub edges_glow_strength: f32,
    /// DCB field: `edgesPerimeterStrength` (Single)
    #[serde(default)]
    pub edges_perimeter_strength: f32,
    /// DCB field: `edgesEmbersStrength` (Single)
    #[serde(default)]
    pub edges_embers_strength: f32,
    /// DCB field: `maximumLuminance` (Single)
    #[serde(default)]
    pub maximum_luminance: f32,
}

impl Pooled for FireHazardAfterglowProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_afterglow_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_afterglow_properties }
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
    /// DCB field: `burnSize` (Single)
    #[serde(default)]
    pub burn_size: f32,
    /// DCB field: `heightmapSize` (Single)
    #[serde(default)]
    pub heightmap_size: f32,
    /// DCB field: `fireSize` (Single)
    #[serde(default)]
    pub fire_size: f32,
    /// DCB field: `triplanarDitherRepeat` (Single)
    #[serde(default)]
    pub triplanar_dither_repeat: f32,
    /// DCB field: `triplanarDitherMaxAngle` (Single)
    #[serde(default)]
    pub triplanar_dither_max_angle: f32,
    /// DCB field: `breakupSize` (Single)
    #[serde(default)]
    pub breakup_size: f32,
    /// DCB field: `fire` (Class)
    #[serde(default)]
    pub fire: Option<Handle<FireHazardFireProperties>>,
    /// DCB field: `afterglow` (Class)
    #[serde(default)]
    pub afterglow: Option<Handle<FireHazardAfterglowProperties>>,
    /// DCB field: `surfaces` (Class)
    #[serde(default)]
    pub surfaces: Option<Handle<FireHazardSurfaceProperties>>,
}

impl Pooled for FireHazardPermanentEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_permanent_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_permanent_effects }
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardFireProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            afterglow: match inst.get("afterglow") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardAfterglowProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardAfterglowProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            surfaces: match inst.get("surfaces") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardSurfaceProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardSurfaceProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FireHazardSpawnParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardSpawnParams {
    /// DCB field: `floorMaxAngle` (Single)
    #[serde(default)]
    pub floor_max_angle: f32,
    /// DCB field: `wallsMaxAngle` (Single)
    #[serde(default)]
    pub walls_max_angle: f32,
    /// DCB field: `ceilingMaxAngle` (Single)
    #[serde(default)]
    pub ceiling_max_angle: f32,
    /// DCB field: `lodTransitionDistance` (Single)
    #[serde(default)]
    pub lod_transition_distance: f32,
    /// DCB field: `lodEffectReferenceVolume` (Single)
    #[serde(default)]
    pub lod_effect_reference_volume: f32,
}

impl Pooled for FireHazardSpawnParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_spawn_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_spawn_params }
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
    /// DCB field: `noiseFrequency` (Class)
    #[serde(default)]
    pub noise_frequency: Option<Handle<Vec3>>,
    /// DCB field: `noiseScale` (Single)
    #[serde(default)]
    pub noise_scale: f32,
    /// DCB field: `noiseSpeed` (Single)
    #[serde(default)]
    pub noise_speed: f32,
    /// DCB field: `noiseTexture` (EnumChoice)
    #[serde(default)]
    pub noise_texture: String,
}

impl Pooled for FireHazardFogNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_fog_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_fog_noise_params }
}

impl<'a> Extract<'a> for FireHazardFogNoiseParams {
    const TYPE_NAME: &'static str = "FireHazardFogNoiseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            noise_frequency: match inst.get("noiseFrequency") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise_scale: inst.get_f32("noiseScale").unwrap_or_default(),
            noise_speed: inst.get_f32("noiseSpeed").unwrap_or_default(),
            noise_texture: inst.get_str("noiseTexture").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `FireHazardFogParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardFogParams {
    /// DCB field: `softEdges` (Single)
    #[serde(default)]
    pub soft_edges: f32,
    /// DCB field: `densityOffset` (Single)
    #[serde(default)]
    pub density_offset: f32,
    /// DCB field: `densityMultiplier` (Single)
    #[serde(default)]
    pub density_multiplier: f32,
    /// DCB field: `maximumDistance` (Single)
    #[serde(default)]
    pub maximum_distance: f32,
    /// DCB field: `saturationDensity` (Single)
    #[serde(default)]
    pub saturation_density: f32,
    /// DCB field: `falloffSharpness` (Single)
    #[serde(default)]
    pub falloff_sharpness: f32,
    /// DCB field: `noiseVolumeOffset` (Single)
    #[serde(default)]
    pub noise_volume_offset: f32,
    /// DCB field: `noise` (Class)
    #[serde(default)]
    pub noise: Option<Handle<FireHazardFogNoiseParams>>,
}

impl Pooled for FireHazardFogParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_fog_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_fog_params }
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardFogNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FireHazardGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalParams {
    /// DCB field: `update` (Class)
    #[serde(default)]
    pub update: Option<Handle<FireHazardGlobalUpdate>>,
    /// DCB field: `ignition` (Class)
    #[serde(default)]
    pub ignition: Option<Handle<FireHazardGlobalIgnition>>,
    /// DCB field: `propagation` (Class)
    #[serde(default)]
    pub propagation: Option<Handle<FireHazardGlobalPropagation>>,
    /// DCB field: `smoke` (Class)
    #[serde(default)]
    pub smoke: Option<Handle<FireHazardGlobalSmokeParams>>,
    /// DCB field: `damageToHealth` (Class)
    #[serde(default)]
    pub damage_to_health: Option<Handle<FireHazardGlobalDamageToHealthParams>>,
    /// DCB field: `extinguishing` (Class)
    #[serde(default)]
    pub extinguishing: Option<Handle<FireHazardGlobalExtinguishing>>,
    /// DCB field: `defaultEffects` (Class)
    #[serde(default)]
    pub default_effects: Option<Handle<FireHazardGlobalDefaultEffects>>,
    /// DCB field: `lightParams` (Class)
    #[serde(default)]
    pub light_params: Option<Handle<FireHazardGlobalLightParams>>,
    /// DCB field: `roomConnector` (Class)
    #[serde(default)]
    pub room_connector: Option<Handle<FireHazardGlobalRoomConnectorParams>>,
    /// DCB field: `particleSpawning` (Class)
    #[serde(default)]
    pub particle_spawning: Option<Handle<FireHazardSpawnParams>>,
    /// DCB field: `fog` (Class)
    #[serde(default)]
    pub fog: Option<Handle<FireHazardFogParams>>,
    /// DCB field: `permanentEffects` (Class)
    #[serde(default)]
    pub permanent_effects: Option<Handle<FireHazardPermanentEffects>>,
}

impl Pooled for FireHazardGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_params }
}

impl<'a> Extract<'a> for FireHazardGlobalParams {
    const TYPE_NAME: &'static str = "FireHazardGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            update: match inst.get("update") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalUpdate>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardGlobalUpdate>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ignition: match inst.get("ignition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalIgnition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardGlobalIgnition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            propagation: match inst.get("propagation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalPropagation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardGlobalPropagation>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            smoke: match inst.get("smoke") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalSmokeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardGlobalSmokeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damage_to_health: match inst.get("damageToHealth") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalDamageToHealthParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardGlobalDamageToHealthParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            extinguishing: match inst.get("extinguishing") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalExtinguishing>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardGlobalExtinguishing>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_effects: match inst.get("defaultEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalDefaultEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardGlobalDefaultEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            light_params: match inst.get("lightParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalLightParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardGlobalLightParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_connector: match inst.get("roomConnector") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardGlobalRoomConnectorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardGlobalRoomConnectorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            particle_spawning: match inst.get("particleSpawning") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardSpawnParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardSpawnParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fog: match inst.get("fog") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardFogParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardFogParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            permanent_effects: match inst.get("permanentEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FireHazardPermanentEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FireHazardPermanentEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FireHazardGlobalUpdate`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalUpdate {
    /// DCB field: `fixedFPS` (Int32)
    #[serde(default)]
    pub fixed_fps: i32,
}

impl Pooled for FireHazardGlobalUpdate {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_update }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_update }
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
    /// DCB field: `globalIgnitionChanceMultiplier` (Single)
    #[serde(default)]
    pub global_ignition_chance_multiplier: f32,
    /// DCB field: `explosionChanceMultiplier` (Single)
    #[serde(default)]
    pub explosion_chance_multiplier: f32,
    /// DCB field: `projectileChanceMultiplier` (Single)
    #[serde(default)]
    pub projectile_chance_multiplier: f32,
    /// DCB field: `collisionChanceMultiplier` (Single)
    #[serde(default)]
    pub collision_chance_multiplier: f32,
    /// DCB field: `fallbackChanceMultiplier` (Single)
    #[serde(default)]
    pub fallback_chance_multiplier: f32,
    /// DCB field: `globalFlashIgnitionThresholdMultiplier` (Single)
    #[serde(default)]
    pub global_flash_ignition_threshold_multiplier: f32,
    /// DCB field: `damageTypeIgnitionModifiers` (Single)
    #[serde(default)]
    pub damage_type_ignition_modifiers: f32,
}

impl Pooled for FireHazardGlobalIgnition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_ignition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_ignition }
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
    /// DCB field: `fireMassThreshold` (Single)
    #[serde(default)]
    pub fire_mass_threshold: f32,
    /// DCB field: `radiativeFactor` (Single)
    #[serde(default)]
    pub radiative_factor: f32,
    /// DCB field: `minimumHeatIntensityOutput` (Single)
    #[serde(default)]
    pub minimum_heat_intensity_output: f32,
    /// DCB field: `maximumRadiationRadius` (Single)
    #[serde(default)]
    pub maximum_radiation_radius: f32,
    /// DCB field: `maximumConvectionDistance` (Single)
    #[serde(default)]
    pub maximum_convection_distance: f32,
    /// DCB field: `standardConvectionTemperature` (Single)
    #[serde(default)]
    pub standard_convection_temperature: f32,
    /// DCB field: `surfaceAreaBurnRateCurveVoxels` (Single)
    #[serde(default)]
    pub surface_area_burn_rate_curve_voxels: f32,
    /// DCB field: `surfaceAreaBurnRateCurveEntities` (Single)
    #[serde(default)]
    pub surface_area_burn_rate_curve_entities: f32,
    /// DCB field: `oxygenPressureRange` (Class)
    #[serde(default)]
    pub oxygen_pressure_range: Option<Handle<Range>>,
}

impl Pooled for FireHazardGlobalPropagation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_propagation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_propagation }
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FireHazardGlobalSmokeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireHazardGlobalSmokeParams {
    /// DCB field: `defaultSmokeComposition` (Reference)
    #[serde(default)]
    pub default_smoke_composition: Option<CigGuid>,
    /// DCB field: `dynamicPart` (Single)
    #[serde(default)]
    pub dynamic_part: f32,
    /// DCB field: `particulateMatter` (Reference)
    #[serde(default)]
    pub particulate_matter: Option<CigGuid>,
    /// DCB field: `waterVapor` (Reference)
    #[serde(default)]
    pub water_vapor: Option<CigGuid>,
}

impl Pooled for FireHazardGlobalSmokeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_smoke_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_smoke_params }
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
    /// DCB field: `minimumDamageTemperature` (Single)
    #[serde(default)]
    pub minimum_damage_temperature: f32,
    /// DCB field: `baseDamage` (Single)
    #[serde(default)]
    pub base_damage: f32,
    /// DCB field: `referenceHealth` (Single)
    #[serde(default)]
    pub reference_health: f32,
    /// DCB field: `curveAngle` (Single)
    #[serde(default)]
    pub curve_angle: f32,
    /// DCB field: `curveOffset` (Single)
    #[serde(default)]
    pub curve_offset: f32,
}

impl Pooled for FireHazardGlobalDamageToHealthParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_damage_to_health_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_damage_to_health_params }
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
    /// DCB field: `oxygenReduction` (Single)
    #[serde(default)]
    pub oxygen_reduction: f32,
    /// DCB field: `heatTransferMultiplier` (Single)
    #[serde(default)]
    pub heat_transfer_multiplier: f32,
    /// DCB field: `radiationAbsorption` (Single)
    #[serde(default)]
    pub radiation_absorption: f32,
    /// DCB field: `vectorFieldPathSpray` (String)
    #[serde(default)]
    pub vector_field_path_spray: String,
    /// DCB field: `vectorFieldPathSphere` (String)
    #[serde(default)]
    pub vector_field_path_sphere: String,
}

impl Pooled for FireHazardGlobalExtinguishing {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_extinguishing }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_extinguishing }
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
    /// DCB field: `voxelFireEffect` (String)
    #[serde(default)]
    pub voxel_fire_effect: String,
    /// DCB field: `voxelFlashIgnitionEffect` (Class)
    #[serde(default)]
    pub voxel_flash_ignition_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `entityFireEffect` (Class)
    #[serde(default)]
    pub entity_fire_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `entitySmokeEffect` (Class)
    #[serde(default)]
    pub entity_smoke_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `defaultSprayExtinguishingEffect` (Class)
    #[serde(default)]
    pub default_spray_extinguishing_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `defaultSphereExtinguishingEffect` (Class)
    #[serde(default)]
    pub default_sphere_extinguishing_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `maximumSmokeEmission` (Single)
    #[serde(default)]
    pub maximum_smoke_emission: f32,
    /// DCB field: `entityReferenceSize` (Single)
    #[serde(default)]
    pub entity_reference_size: f32,
    /// DCB field: `entityMinimumSize` (Single)
    #[serde(default)]
    pub entity_minimum_size: f32,
}

impl Pooled for FireHazardGlobalDefaultEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_default_effects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_default_effects }
}

impl<'a> Extract<'a> for FireHazardGlobalDefaultEffects {
    const TYPE_NAME: &'static str = "FireHazardGlobalDefaultEffects";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            voxel_fire_effect: inst.get_str("voxelFireEffect").map(String::from).unwrap_or_default(),
            voxel_flash_ignition_effect: match inst.get("voxelFlashIgnitionEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entity_fire_effect: match inst.get("entityFireEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entity_smoke_effect: match inst.get("entitySmokeEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_spray_extinguishing_effect: match inst.get("defaultSprayExtinguishingEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_sphere_extinguishing_effect: match inst.get("defaultSphereExtinguishingEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
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
    /// DCB field: `lightColor` (Class)
    #[serde(default)]
    pub light_color: Option<Handle<RGB>>,
    /// DCB field: `maxLights` (Int32)
    #[serde(default)]
    pub max_lights: i32,
    /// DCB field: `teleportCooldown` (Single)
    #[serde(default)]
    pub teleport_cooldown: f32,
    /// DCB field: `selectionRadius` (Single)
    #[serde(default)]
    pub selection_radius: f32,
    /// DCB field: `animMoveSpeed` (Single)
    #[serde(default)]
    pub anim_move_speed: f32,
    /// DCB field: `animGrowSpeed` (Single)
    #[serde(default)]
    pub anim_grow_speed: f32,
    /// DCB field: `animShrinkSpeed` (Single)
    #[serde(default)]
    pub anim_shrink_speed: f32,
    /// DCB field: `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// DCB field: `bulbRadiusRange` (Class)
    #[serde(default)]
    pub bulb_radius_range: Option<Handle<Range>>,
    /// DCB field: `lightIntensityRange` (Class)
    #[serde(default)]
    pub light_intensity_range: Option<Handle<Range>>,
    /// DCB field: `specularMultiplier` (Single)
    #[serde(default)]
    pub specular_multiplier: f32,
    /// DCB field: `fogMultiplier` (Single)
    #[serde(default)]
    pub fog_multiplier: f32,
    /// DCB field: `fogOcclusionFalloff` (Single)
    #[serde(default)]
    pub fog_occlusion_falloff: f32,
    /// DCB field: `lightAnimStyle` (Byte)
    #[serde(default)]
    pub light_anim_style: u32,
    /// DCB field: `lightAnimSpeed` (Single)
    #[serde(default)]
    pub light_anim_speed: f32,
    /// DCB field: `softness` (Single)
    #[serde(default)]
    pub softness: f32,
    /// DCB field: `maxHeatReleaseRate` (Single)
    #[serde(default)]
    pub max_heat_release_rate: f32,
    /// DCB field: `lightTemperature` (Single)
    #[serde(default)]
    pub light_temperature: f32,
    /// DCB field: `useTemperature` (Boolean)
    #[serde(default)]
    pub use_temperature: bool,
    /// DCB field: `affectsFog` (Boolean)
    #[serde(default)]
    pub affects_fog: bool,
    /// DCB field: `shadowMap` (Boolean)
    #[serde(default)]
    pub shadow_map: bool,
}

impl Pooled for FireHazardGlobalLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_light_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_light_params }
}

impl<'a> Extract<'a> for FireHazardGlobalLightParams {
    const TYPE_NAME: &'static str = "FireHazardGlobalLightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_color: match inst.get("lightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            light_intensity_range: match inst.get("lightIntensityRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
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
    /// DCB field: `igniteThreshold` (Single)
    #[serde(default)]
    pub ignite_threshold: f32,
    /// DCB field: `maxVolumeRadius` (Single)
    #[serde(default)]
    pub max_volume_radius: f32,
    /// DCB field: `vectorFieldPath` (String)
    #[serde(default)]
    pub vector_field_path: String,
    /// DCB field: `vectorFieldRotation` (Single)
    #[serde(default)]
    pub vector_field_rotation: f32,
    /// DCB field: `vectorFieldRadius` (Single)
    #[serde(default)]
    pub vector_field_radius: f32,
    /// DCB field: `vectorFieldFalloff` (Single)
    #[serde(default)]
    pub vector_field_falloff: f32,
    /// DCB field: `vectorFieldThreshold` (Single)
    #[serde(default)]
    pub vector_field_threshold: f32,
    /// DCB field: `vectorFieldMaxHeatReleaseRateDiff` (Single)
    #[serde(default)]
    pub vector_field_max_heat_release_rate_diff: f32,
    /// DCB field: `vectorFieldMaxStrength` (Single)
    #[serde(default)]
    pub vector_field_max_strength: f32,
    /// DCB field: `vectorFieldDepth` (Single)
    #[serde(default)]
    pub vector_field_depth: f32,
}

impl Pooled for FireHazardGlobalRoomConnectorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fi.fire_hazard_global_room_connector_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fi.fire_hazard_global_room_connector_params }
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

