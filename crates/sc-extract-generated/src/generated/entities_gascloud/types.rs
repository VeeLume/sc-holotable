// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-gascloud`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `GasCloudLightNoiseParams`
pub struct GasCloudLightNoiseParams {
    /// `frequency` (Single)
    pub frequency: f32,
    /// `octaves` (Single)
    pub octaves: f32,
    /// `exponent` (Single)
    pub exponent: f32,
    /// `effect` (Single)
    pub effect: f32,
}

impl Pooled for GasCloudLightNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.gas_cloud_light_noise_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.gas_cloud_light_noise_params
    }
}

impl<'a> Extract<'a> for GasCloudLightNoiseParams {
    const TYPE_NAME: &'static str = "GasCloudLightNoiseParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            octaves: inst.get_f32("octaves").unwrap_or_default(),
            exponent: inst.get_f32("exponent").unwrap_or_default(),
            effect: inst.get_f32("effect").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudLightShadowParams`
pub struct GasCloudLightShadowParams {
    /// `active` (Boolean)
    pub active: bool,
    /// `resolution` (UInt32)
    pub resolution: u32,
    /// `opaqueShadows` (Boolean)
    pub opaque_shadows: bool,
}

impl Pooled for GasCloudLightShadowParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.gas_cloud_light_shadow_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.gas_cloud_light_shadow_params
    }
}

impl<'a> Extract<'a> for GasCloudLightShadowParams {
    const TYPE_NAME: &'static str = "GasCloudLightShadowParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            active: inst.get_bool("active").unwrap_or_default(),
            resolution: inst.get_u32("resolution").unwrap_or_default(),
            opaque_shadows: inst.get_bool("opaqueShadows").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudLightFadeParams`
pub struct GasCloudLightFadeParams {
    /// `minDistance` (Single)
    pub min_distance: f32,
    /// `minFade` (Single)
    pub min_fade: f32,
    /// `maxDistance` (Single)
    pub max_distance: f32,
    /// `maxFade` (Single)
    pub max_fade: f32,
}

impl Pooled for GasCloudLightFadeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.gas_cloud_light_fade_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.gas_cloud_light_fade_params
    }
}

impl<'a> Extract<'a> for GasCloudLightFadeParams {
    const TYPE_NAME: &'static str = "GasCloudLightFadeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_distance: inst.get_f32("minDistance").unwrap_or_default(),
            min_fade: inst.get_f32("minFade").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            max_fade: inst.get_f32("maxFade").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudLightAudioParams`
pub struct GasCloudLightAudioParams {
    /// `gasCloudLoop` (Class)
    pub gas_cloud_loop: Option<Handle<GlobalResourceAudio>>,
    /// `gasCloudNormIntensityRtpc` (Class)
    pub gas_cloud_norm_intensity_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for GasCloudLightAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.gas_cloud_light_audio_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.gas_cloud_light_audio_params
    }
}

impl<'a> Extract<'a> for GasCloudLightAudioParams {
    const TYPE_NAME: &'static str = "GasCloudLightAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            gas_cloud_loop: match inst.get("gasCloudLoop") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            gas_cloud_norm_intensity_rtpc: match inst.get("gasCloudNormIntensityRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `GasCloudLightParams`
/// Inherits from: `DataForgeComponentParams`
pub struct GasCloudLightParams {
    /// `active` (Boolean)
    pub active: bool,
    /// `lightType` (EnumChoice)
    pub light_type: EGasCloudLightType,
    /// `color` (Class)
    pub color: Option<Handle<RGB>>,
    /// `intensity` (Single)
    pub intensity: f32,
    /// `innerRadius` (Single)
    pub inner_radius: f32,
    /// `outerRadius` (Single)
    pub outer_radius: f32,
    /// `affectsFog` (Boolean)
    pub affects_fog: bool,
    /// `affectsObjects` (Boolean)
    pub affects_objects: bool,
    /// `specular` (Boolean)
    pub specular: bool,
    /// `projectorParams` (Class)
    pub projector_params: Option<Handle<LightProjectorParams>>,
    /// `shadow` (Class)
    pub shadow: Option<Handle<GasCloudLightShadowParams>>,
    /// `fade` (Class)
    pub fade: Option<Handle<GasCloudLightFadeParams>>,
    /// `noise` (Class)
    pub noise: Option<Handle<GasCloudLightNoiseParams>>,
    /// `audio` (Class)
    pub audio: Option<Handle<GasCloudLightAudioParams>>,
}

impl Pooled for GasCloudLightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.gas_cloud_light_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.gas_cloud_light_params
    }
}

impl<'a> Extract<'a> for GasCloudLightParams {
    const TYPE_NAME: &'static str = "GasCloudLightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            active: inst.get_bool("active").unwrap_or_default(),
            light_type: EGasCloudLightType::from_dcb_str(inst.get_str("lightType").unwrap_or("")),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            intensity: inst.get_f32("intensity").unwrap_or_default(),
            inner_radius: inst.get_f32("innerRadius").unwrap_or_default(),
            outer_radius: inst.get_f32("outerRadius").unwrap_or_default(),
            affects_fog: inst.get_bool("affectsFog").unwrap_or_default(),
            affects_objects: inst.get_bool("affectsObjects").unwrap_or_default(),
            specular: inst.get_bool("specular").unwrap_or_default(),
            projector_params: match inst.get("projectorParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<LightProjectorParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            shadow: match inst.get("shadow") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GasCloudLightShadowParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            fade: match inst.get("fade") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GasCloudLightFadeParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            noise: match inst.get("noise") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GasCloudLightNoiseParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            audio: match inst.get("audio") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GasCloudLightAudioParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `GasCloudOverrideSphereVolumeParams`
pub struct GasCloudOverrideSphereVolumeParams {
    /// `radius` (Single)
    pub radius: f32,
    /// `fade` (Single)
    pub fade: f32,
}

impl Pooled for GasCloudOverrideSphereVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_gascloud
            .gas_cloud_override_sphere_volume_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_gascloud
            .gas_cloud_override_sphere_volume_params
    }
}

impl<'a> Extract<'a> for GasCloudOverrideSphereVolumeParams {
    const TYPE_NAME: &'static str = "GasCloudOverrideSphereVolumeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            radius: inst.get_f32("radius").unwrap_or_default(),
            fade: inst.get_f32("fade").unwrap_or_default(),
        }
    }
}

/// DCB type: `GasCloudOverrideCubeVolumeParams`
pub struct GasCloudOverrideCubeVolumeParams {
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

impl Pooled for GasCloudOverrideCubeVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_gascloud
            .gas_cloud_override_cube_volume_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_gascloud
            .gas_cloud_override_cube_volume_params
    }
}

impl<'a> Extract<'a> for GasCloudOverrideCubeVolumeParams {
    const TYPE_NAME: &'static str = "GasCloudOverrideCubeVolumeParams";
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

/// DCB type: `GasCloudOverrideVolumeParams`
/// Inherits from: `DataForgeComponentParams`
pub struct GasCloudOverrideVolumeParams {
    /// `volumeType` (EnumChoice)
    pub volume_type: EGasCloudOverrideVolumeType,
    /// `sphereVolume` (Class)
    pub sphere_volume: Option<Handle<GasCloudOverrideSphereVolumeParams>>,
    /// `cubeVolume` (Class)
    pub cube_volume: Option<Handle<GasCloudOverrideCubeVolumeParams>>,
    /// `overrideAlbedo` (Boolean)
    pub override_albedo: bool,
    /// `overrideLightLeak` (Boolean)
    pub override_light_leak: bool,
    /// `overrideDensity` (Boolean)
    pub override_density: bool,
    /// `albedo` (Class)
    pub albedo: Option<Handle<RGB>>,
    /// `lightLeakColor` (Class)
    pub light_leak_color: Option<Handle<RGB>>,
    /// `lightLeakScale` (Single)
    pub light_leak_scale: f32,
    /// `density` (Single)
    pub density: f32,
}

impl Pooled for GasCloudOverrideVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.gas_cloud_override_volume_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.gas_cloud_override_volume_params
    }
}

impl<'a> Extract<'a> for GasCloudOverrideVolumeParams {
    const TYPE_NAME: &'static str = "GasCloudOverrideVolumeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            volume_type: EGasCloudOverrideVolumeType::from_dcb_str(
                inst.get_str("volumeType").unwrap_or(""),
            ),
            sphere_volume: match inst.get("sphereVolume") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GasCloudOverrideSphereVolumeParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            cube_volume: match inst.get("cubeVolume") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GasCloudOverrideCubeVolumeParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            override_albedo: inst.get_bool("overrideAlbedo").unwrap_or_default(),
            override_light_leak: inst.get_bool("overrideLightLeak").unwrap_or_default(),
            override_density: inst.get_bool("overrideDensity").unwrap_or_default(),
            albedo: match inst.get("albedo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            light_leak_color: match inst.get("lightLeakColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            light_leak_scale: inst.get_f32("lightLeakScale").unwrap_or_default(),
            density: inst.get_f32("density").unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityLinkTargetingParams`
pub struct EntityLinkTargetingParams {
    /// `useAsSurfaceTargetEllipsoid` (Boolean)
    pub use_as_surface_target_ellipsoid: bool,
    /// `surfaceTargetEllipsoidScale` (Class)
    pub surface_target_ellipsoid_scale: Option<Handle<Vec3>>,
    /// `defaultToArea` (Boolean)
    pub default_to_area: bool,
}

impl Pooled for EntityLinkTargetingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.entity_link_targeting_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.entity_link_targeting_params
    }
}

impl<'a> Extract<'a> for EntityLinkTargetingParams {
    const TYPE_NAME: &'static str = "EntityLinkTargetingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            use_as_surface_target_ellipsoid: inst
                .get_bool("useAsSurfaceTargetEllipsoid")
                .unwrap_or_default(),
            surface_target_ellipsoid_scale: match inst.get("surfaceTargetEllipsoidScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            default_to_area: inst.get_bool("defaultToArea").unwrap_or_default(),
        }
    }
}

/// DCB type: `LightningRegionLightningParams`
pub struct LightningRegionLightningParams {
    /// `lightningEffect` (Class)
    pub lightning_effect: Option<Handle<GlobalResourceParticle>>,
    /// `targetingSeed` (UInt32)
    pub targeting_seed: u32,
    /// `appearanceSeed` (Int32)
    pub appearance_seed: i32,
    /// `strength` (Single)
    pub strength: f32,
    /// `strikeArea` (Boolean)
    pub strike_area: bool,
    /// `strikeLinks` (StrongPointer)
    pub strike_links: Option<Handle<EntityLinkTargetingParams>>,
    /// `strikeSurfaces` (Boolean)
    pub strike_surfaces: bool,
    /// `conductiveSurfaceSpawnProbability` (Single)
    pub conductive_surface_spawn_probability: f32,
    /// `internalStrikeProbability` (Single)
    pub internal_strike_probability: f32,
    /// `minimumInterval` (Single)
    pub minimum_interval: f32,
    /// `maximumInterval` (Single)
    pub maximum_interval: f32,
    /// `spawnRadius` (Single)
    pub spawn_radius: f32,
    /// `targetRadius` (Single)
    pub target_radius: f32,
    /// `minimumLength` (Single)
    pub minimum_length: f32,
    /// `maximumLength` (Single)
    pub maximum_length: f32,
    /// `minimumCameraDistance` (Single)
    pub minimum_camera_distance: f32,
    /// `isCascadeTrigger` (Boolean)
    pub is_cascade_trigger: bool,
    /// `cascadeRadius` (Single)
    pub cascade_radius: f32,
    /// `cascadeBuildUpScale` (Single)
    pub cascade_build_up_scale: f32,
    /// `cascadeMinimumInterval` (Single)
    pub cascade_minimum_interval: f32,
    /// `cascadeTriggerMinimumInterval` (Single)
    pub cascade_trigger_minimum_interval: f32,
    /// `localScale` (Class)
    pub local_scale: Option<Handle<Vec3>>,
    /// `scaleMultiplier` (Single)
    pub scale_multiplier: f32,
    /// `audioEmitterTrigger` (Class)
    pub audio_emitter_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `audioTargetTrigger` (Class)
    pub audio_target_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for LightningRegionLightningParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.lightning_region_lightning_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.lightning_region_lightning_params
    }
}

impl<'a> Extract<'a> for LightningRegionLightningParams {
    const TYPE_NAME: &'static str = "LightningRegionLightningParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lightning_effect: match inst.get("lightningEffect") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceParticle>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            targeting_seed: inst.get_u32("targetingSeed").unwrap_or_default(),
            appearance_seed: inst.get_i32("appearanceSeed").unwrap_or_default(),
            strength: inst.get_f32("strength").unwrap_or_default(),
            strike_area: inst.get_bool("strikeArea").unwrap_or_default(),
            strike_links: match inst.get("strikeLinks") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<EntityLinkTargetingParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            strike_surfaces: inst.get_bool("strikeSurfaces").unwrap_or_default(),
            conductive_surface_spawn_probability: inst
                .get_f32("conductiveSurfaceSpawnProbability")
                .unwrap_or_default(),
            internal_strike_probability: inst
                .get_f32("internalStrikeProbability")
                .unwrap_or_default(),
            minimum_interval: inst.get_f32("minimumInterval").unwrap_or_default(),
            maximum_interval: inst.get_f32("maximumInterval").unwrap_or_default(),
            spawn_radius: inst.get_f32("spawnRadius").unwrap_or_default(),
            target_radius: inst.get_f32("targetRadius").unwrap_or_default(),
            minimum_length: inst.get_f32("minimumLength").unwrap_or_default(),
            maximum_length: inst.get_f32("maximumLength").unwrap_or_default(),
            minimum_camera_distance: inst.get_f32("minimumCameraDistance").unwrap_or_default(),
            is_cascade_trigger: inst.get_bool("isCascadeTrigger").unwrap_or_default(),
            cascade_radius: inst.get_f32("cascadeRadius").unwrap_or_default(),
            cascade_build_up_scale: inst.get_f32("cascadeBuildUpScale").unwrap_or_default(),
            cascade_minimum_interval: inst.get_f32("cascadeMinimumInterval").unwrap_or_default(),
            cascade_trigger_minimum_interval: inst
                .get_f32("cascadeTriggerMinimumInterval")
                .unwrap_or_default(),
            local_scale: match inst.get("localScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            scale_multiplier: inst.get_f32("scaleMultiplier").unwrap_or_default(),
            audio_emitter_trigger: match inst.get("audioEmitterTrigger") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            audio_target_trigger: match inst.get("audioTargetTrigger") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `InterferenceParams`
pub struct InterferenceParams {
    /// `fallOffDistance` (Single)
    pub fall_off_distance: f32,
}

impl Pooled for InterferenceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.interference_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.interference_params
    }
}

impl<'a> Extract<'a> for InterferenceParams {
    const TYPE_NAME: &'static str = "InterferenceParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fall_off_distance: inst.get_f32("fallOffDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `LightningRegionParams`
/// Inherits from: `DataForgeComponentParams`
pub struct LightningRegionParams {
    /// `active` (Boolean)
    pub active: bool,
    /// `lightning` (Class)
    pub lightning: Option<Handle<LightningRegionLightningParams>>,
    /// `interference` (StrongPointer)
    pub interference: Option<InterferenceParamsPtr>,
}

impl Pooled for LightningRegionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_gascloud.lightning_region_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_gascloud.lightning_region_params
    }
}

impl<'a> Extract<'a> for LightningRegionParams {
    const TYPE_NAME: &'static str = "LightningRegionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            active: inst.get_bool("active").unwrap_or_default(),
            lightning: match inst.get("lightning") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<LightningRegionLightningParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            interference: match inst.get("interference") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(InterferenceParamsPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}
