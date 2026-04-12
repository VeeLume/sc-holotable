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

/// DCB type: `PingSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingSharedParams {
    /// DCB field: `blobLifeTime` (Class)
    #[serde(default)]
    pub blob_life_time: Option<Handle<PingBlobLifeTime>>,
    /// DCB field: `blobOffsetScalar` (Single)
    #[serde(default)]
    pub blob_offset_scalar: f32,
    /// DCB field: `blobSizeScalar` (Single)
    #[serde(default)]
    pub blob_size_scalar: f32,
    /// DCB field: `blobScaleFov` (Single)
    #[serde(default)]
    pub blob_scale_fov: f32,
    /// DCB field: `blobScaleMinPixels` (Single)
    #[serde(default)]
    pub blob_scale_min_pixels: f32,
    /// DCB field: `blobScaleMaxPixels` (Single)
    #[serde(default)]
    pub blob_scale_max_pixels: f32,
    /// DCB field: `blobScaleFixedResolution` (Single)
    #[serde(default)]
    pub blob_scale_fixed_resolution: f32,
    /// DCB field: `contactLifeTime` (Class)
    #[serde(default)]
    pub contact_life_time: Option<Handle<PingContactLifeTime>>,
    /// DCB field: `extendedLifeTimeParams` (StrongPointer)
    #[serde(default)]
    pub extended_life_time_params: Option<Handle<PingExtendedLifeTimeParams>>,
    /// DCB field: `pingWaveAcceleration` (Single)
    #[serde(default)]
    pub ping_wave_acceleration: f32,
    /// DCB field: `pingWaveJerk` (Single)
    #[serde(default)]
    pub ping_wave_jerk: f32,
    /// DCB field: `pingWaveJerkDistance` (Single)
    #[serde(default)]
    pub ping_wave_jerk_distance: f32,
    /// DCB field: `useADSMode` (Boolean)
    #[serde(default)]
    pub use_adsmode: bool,
    /// DCB field: `ADSFovFocusAngleMultiplier` (Single)
    #[serde(default)]
    pub adsfov_focus_angle_multiplier: f32,
    /// DCB field: `blobVFXParams` (StrongPointer)
    #[serde(default)]
    pub blob_vfxparams: Option<Handle<BlobVFXSharedParams>>,
    /// DCB field: `pingSFXParams` (StrongPointer)
    #[serde(default)]
    pub ping_sfxparams: Option<Handle<PingSFXSharedParams>>,
    /// DCB field: `pingVFXParams` (StrongPointer)
    #[serde(default)]
    pub ping_vfxparams: Option<Handle<PingVFXSharedParams>>,
    /// DCB field: `pingTypeParams` (StrongPointer)
    #[serde(default)]
    pub ping_type_params: Option<Handle<PingTypeParams>>,
}

impl Pooled for PingSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pi.ping_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pi.ping_shared_params }
}

impl<'a> Extract<'a> for PingSharedParams {
    const TYPE_NAME: &'static str = "PingSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            blob_life_time: match inst.get("blobLifeTime") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingBlobLifeTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingBlobLifeTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blob_offset_scalar: inst.get_f32("blobOffsetScalar").unwrap_or_default(),
            blob_size_scalar: inst.get_f32("blobSizeScalar").unwrap_or_default(),
            blob_scale_fov: inst.get_f32("blobScaleFov").unwrap_or_default(),
            blob_scale_min_pixels: inst.get_f32("blobScaleMinPixels").unwrap_or_default(),
            blob_scale_max_pixels: inst.get_f32("blobScaleMaxPixels").unwrap_or_default(),
            blob_scale_fixed_resolution: inst.get_f32("blobScaleFixedResolution").unwrap_or_default(),
            contact_life_time: match inst.get("contactLifeTime") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingContactLifeTime>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingContactLifeTime>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            extended_life_time_params: match inst.get("extendedLifeTimeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingExtendedLifeTimeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingExtendedLifeTimeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_wave_acceleration: inst.get_f32("pingWaveAcceleration").unwrap_or_default(),
            ping_wave_jerk: inst.get_f32("pingWaveJerk").unwrap_or_default(),
            ping_wave_jerk_distance: inst.get_f32("pingWaveJerkDistance").unwrap_or_default(),
            use_adsmode: inst.get_bool("useADSMode").unwrap_or_default(),
            adsfov_focus_angle_multiplier: inst.get_f32("ADSFovFocusAngleMultiplier").unwrap_or_default(),
            blob_vfxparams: match inst.get("blobVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BlobVFXSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BlobVFXSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_sfxparams: match inst.get("pingSFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingSFXSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingSFXSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_vfxparams: match inst.get("pingVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingVFXSharedParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingVFXSharedParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_type_params: match inst.get("pingTypeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingTypeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingTypeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PingBlobLifeTime`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingBlobLifeTime {
    /// DCB field: `lifeTimeMin` (Single)
    #[serde(default)]
    pub life_time_min: f32,
    /// DCB field: `lifeTimeMax` (Single)
    #[serde(default)]
    pub life_time_max: f32,
}

impl Pooled for PingBlobLifeTime {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pi.ping_blob_life_time }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pi.ping_blob_life_time }
}

impl<'a> Extract<'a> for PingBlobLifeTime {
    const TYPE_NAME: &'static str = "PingBlobLifeTime";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            life_time_min: inst.get_f32("lifeTimeMin").unwrap_or_default(),
            life_time_max: inst.get_f32("lifeTimeMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `PingExtendedLifeTimeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingExtendedLifeTimeParams {
    /// DCB field: `extendedContactTypes` (Reference (array))
    #[serde(default)]
    pub extended_contact_types: Vec<CigGuid>,
    /// DCB field: `extendedLifeTimeDuration` (Single)
    #[serde(default)]
    pub extended_life_time_duration: f32,
    /// DCB field: `extendForOccludedContacts` (Boolean)
    #[serde(default)]
    pub extend_for_occluded_contacts: bool,
    /// DCB field: `disableQuickScanHighlightOnOcclusion` (Boolean)
    #[serde(default)]
    pub disable_quick_scan_highlight_on_occlusion: bool,
    /// DCB field: `resetDetectionOnDeath` (Boolean)
    #[serde(default)]
    pub reset_detection_on_death: bool,
    /// DCB field: `extendForDeadContacts` (Boolean)
    #[serde(default)]
    pub extend_for_dead_contacts: bool,
}

impl Pooled for PingExtendedLifeTimeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pi.ping_extended_life_time_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pi.ping_extended_life_time_params }
}

impl<'a> Extract<'a> for PingExtendedLifeTimeParams {
    const TYPE_NAME: &'static str = "PingExtendedLifeTimeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            extended_contact_types: inst.get_array("extendedContactTypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            extended_life_time_duration: inst.get_f32("extendedLifeTimeDuration").unwrap_or_default(),
            extend_for_occluded_contacts: inst.get_bool("extendForOccludedContacts").unwrap_or_default(),
            disable_quick_scan_highlight_on_occlusion: inst.get_bool("disableQuickScanHighlightOnOcclusion").unwrap_or_default(),
            reset_detection_on_death: inst.get_bool("resetDetectionOnDeath").unwrap_or_default(),
            extend_for_dead_contacts: inst.get_bool("extendForDeadContacts").unwrap_or_default(),
        }
    }
}

/// DCB type: `PingContactLifeTime`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingContactLifeTime {
    /// DCB field: `lifeTimeMin` (Single)
    #[serde(default)]
    pub life_time_min: f32,
    /// DCB field: `lifeTimeMax` (Single)
    #[serde(default)]
    pub life_time_max: f32,
}

impl Pooled for PingContactLifeTime {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pi.ping_contact_life_time }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pi.ping_contact_life_time }
}

impl<'a> Extract<'a> for PingContactLifeTime {
    const TYPE_NAME: &'static str = "PingContactLifeTime";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            life_time_min: inst.get_f32("lifeTimeMin").unwrap_or_default(),
            life_time_max: inst.get_f32("lifeTimeMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `PingSFXSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingSFXSharedParams {
    /// DCB field: `invokePing` (Class)
    #[serde(default)]
    pub invoke_ping: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `invokePingCharged` (Class)
    #[serde(default)]
    pub invoke_ping_charged: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `invokePingBlocked` (Class)
    #[serde(default)]
    pub invoke_ping_blocked: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `invokePingCooldown` (Class)
    #[serde(default)]
    pub invoke_ping_cooldown: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for PingSFXSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pi.ping_sfxshared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pi.ping_sfxshared_params }
}

impl<'a> Extract<'a> for PingSFXSharedParams {
    const TYPE_NAME: &'static str = "PingSFXSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            invoke_ping: match inst.get("invokePing") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invoke_ping_charged: match inst.get("invokePingCharged") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invoke_ping_blocked: match inst.get("invokePingBlocked") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invoke_ping_cooldown: match inst.get("invokePingCooldown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PingWaveVFXParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingWaveVFXParams {
    /// DCB field: `waveParticle` (Class)
    #[serde(default)]
    pub wave_particle: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `pulseSphereGeometry` (Class)
    #[serde(default)]
    pub pulse_sphere_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `pulseHalfSphereGeometry` (Class)
    #[serde(default)]
    pub pulse_half_sphere_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `pulseConeGeometry` (Class)
    #[serde(default)]
    pub pulse_cone_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `pulseMaterial` (Class)
    #[serde(default)]
    pub pulse_material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `conePulseMaterial` (Class)
    #[serde(default)]
    pub cone_pulse_material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `visualAcceleration` (Single)
    #[serde(default)]
    pub visual_acceleration: f32,
    /// DCB field: `visualMaxDistance` (Single)
    #[serde(default)]
    pub visual_max_distance: f32,
    /// DCB field: `snapWaveTimescaleToLifetime` (Boolean)
    #[serde(default)]
    pub snap_wave_timescale_to_lifetime: bool,
    /// DCB field: `use360PingwaveEffectForAllLevels` (Boolean)
    #[serde(default)]
    pub use360_pingwave_effect_for_all_levels: bool,
    /// DCB field: `attachPingwaveEffectToHost` (Boolean)
    #[serde(default)]
    pub attach_pingwave_effect_to_host: bool,
}

impl Pooled for PingWaveVFXParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pi.ping_wave_vfxparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pi.ping_wave_vfxparams }
}

impl<'a> Extract<'a> for PingWaveVFXParams {
    const TYPE_NAME: &'static str = "PingWaveVFXParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wave_particle: match inst.get("waveParticle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pulse_sphere_geometry: match inst.get("pulseSphereGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pulse_half_sphere_geometry: match inst.get("pulseHalfSphereGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pulse_cone_geometry: match inst.get("pulseConeGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pulse_material: match inst.get("pulseMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cone_pulse_material: match inst.get("conePulseMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            visual_acceleration: inst.get_f32("visualAcceleration").unwrap_or_default(),
            visual_max_distance: inst.get_f32("visualMaxDistance").unwrap_or_default(),
            snap_wave_timescale_to_lifetime: inst.get_bool("snapWaveTimescaleToLifetime").unwrap_or_default(),
            use360_pingwave_effect_for_all_levels: inst.get_bool("use360PingwaveEffectForAllLevels").unwrap_or_default(),
            attach_pingwave_effect_to_host: inst.get_bool("attachPingwaveEffectToHost").unwrap_or_default(),
        }
    }
}

/// DCB type: `PingVFXSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingVFXSharedParams {
    /// DCB field: `quickPingWaveVFXParams` (StrongPointer)
    #[serde(default)]
    pub quick_ping_wave_vfxparams: Option<Handle<PingWaveVFXParams>>,
    /// DCB field: `chargedPingWaveVFXParams` (StrongPointer)
    #[serde(default)]
    pub charged_ping_wave_vfxparams: Option<Handle<PingWaveVFXParams>>,
    /// DCB field: `blockedPingWaveVFXParams` (StrongPointer)
    #[serde(default)]
    pub blocked_ping_wave_vfxparams: Option<Handle<PingWaveVFXParams>>,
}

impl Pooled for PingVFXSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pi.ping_vfxshared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pi.ping_vfxshared_params }
}

impl<'a> Extract<'a> for PingVFXSharedParams {
    const TYPE_NAME: &'static str = "PingVFXSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            quick_ping_wave_vfxparams: match inst.get("quickPingWaveVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingWaveVFXParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingWaveVFXParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            charged_ping_wave_vfxparams: match inst.get("chargedPingWaveVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingWaveVFXParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingWaveVFXParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blocked_ping_wave_vfxparams: match inst.get("blockedPingWaveVFXParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PingWaveVFXParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PingWaveVFXParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PingTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingTypeParams {
    /// DCB field: `pingChargeUIShowTime` (Single)
    #[serde(default)]
    pub ping_charge_uishow_time: f32,
    /// DCB field: `pingChargeUIJammedTime` (Single)
    #[serde(default)]
    pub ping_charge_uijammed_time: f32,
    /// DCB field: `pingChargeUIHideTime` (Single)
    #[serde(default)]
    pub ping_charge_uihide_time: f32,
    /// DCB field: `pingChargeUIUnavailableTime` (Single)
    #[serde(default)]
    pub ping_charge_uiunavailable_time: f32,
}

impl Pooled for PingTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pi.ping_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pi.ping_type_params }
}

impl<'a> Extract<'a> for PingTypeParams {
    const TYPE_NAME: &'static str = "PingTypeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            ping_charge_uishow_time: inst.get_f32("pingChargeUIShowTime").unwrap_or_default(),
            ping_charge_uijammed_time: inst.get_f32("pingChargeUIJammedTime").unwrap_or_default(),
            ping_charge_uihide_time: inst.get_f32("pingChargeUIHideTime").unwrap_or_default(),
            ping_charge_uiunavailable_time: inst.get_f32("pingChargeUIUnavailableTime").unwrap_or_default(),
        }
    }
}

