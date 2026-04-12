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

/// DCB type: `ExplosionFlashbangParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosionFlashbangParams {
    /// DCB field: `blindAmount` (Single)
    #[serde(default)]
    pub blind_amount: f32,
    /// DCB field: `flashbangBaseTime` (Single)
    #[serde(default)]
    pub flashbang_base_time: f32,
}

impl Pooled for ExplosionFlashbangParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ex.explosion_flashbang_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ex.explosion_flashbang_params }
}

impl<'a> Extract<'a> for ExplosionFlashbangParams {
    const TYPE_NAME: &'static str = "ExplosionFlashbangParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blind_amount: inst.get_f32("blindAmount").unwrap_or_default(),
            flashbang_base_time: inst.get_f32("flashbangBaseTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `ExplosiveFragmentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosiveFragmentParams {
    /// DCB field: `mass` (Single)
    #[serde(default)]
    pub mass: f32,
    /// DCB field: `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// DCB field: `drag` (Single)
    #[serde(default)]
    pub drag: f32,
    /// DCB field: `amount` (Int32)
    #[serde(default)]
    pub amount: i32,
    /// DCB field: `surfaceTypeOverride` (String)
    #[serde(default)]
    pub surface_type_override: String,
}

impl Pooled for ExplosiveFragmentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ex.explosive_fragment_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ex.explosive_fragment_params }
}

impl<'a> Extract<'a> for ExplosiveFragmentParams {
    const TYPE_NAME: &'static str = "ExplosiveFragmentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mass: inst.get_f32("mass").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            drag: inst.get_f32("drag").unwrap_or_default(),
            amount: inst.get_i32("amount").unwrap_or_default(),
            surface_type_override: inst.get_str("surfaceTypeOverride").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ExplosionParams`
///
/// Inherits from: `ExplosionBaseParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosionParams {
    /// DCB field: `friendlyFire` (EnumChoice)
    #[serde(default)]
    pub friendly_fire: String,
    /// DCB field: `minRadius` (Single)
    #[serde(default)]
    pub min_radius: f32,
    /// DCB field: `maxRadius` (Single)
    #[serde(default)]
    pub max_radius: f32,
    /// DCB field: `soundRadius` (Single)
    #[serde(default)]
    pub sound_radius: f32,
    /// DCB field: `minPhysRadius` (Single)
    #[serde(default)]
    pub min_phys_radius: f32,
    /// DCB field: `maxPhysRadius` (Single)
    #[serde(default)]
    pub max_phys_radius: f32,
    /// DCB field: `angle` (Single)
    #[serde(default)]
    pub angle: f32,
    /// DCB field: `angleVertical` (Single)
    #[serde(default)]
    pub angle_vertical: f32,
    /// DCB field: `explosionFront` (Class)
    #[serde(default)]
    pub explosion_front: Option<Handle<Vec3>>,
    /// DCB field: `explosionUp` (Class)
    #[serde(default)]
    pub explosion_up: Option<Handle<Vec3>>,
    /// DCB field: `explosionZone` (EnumChoice)
    #[serde(default)]
    pub explosion_zone: String,
    /// DCB field: `pressure` (Single)
    #[serde(default)]
    pub pressure: f32,
    /// DCB field: `holeSize` (Single)
    #[serde(default)]
    pub hole_size: f32,
    /// DCB field: `terrainHoleSize` (Single)
    #[serde(default)]
    pub terrain_hole_size: f32,
    /// DCB field: `maxblurdist` (Single)
    #[serde(default)]
    pub maxblurdist: f32,
    /// DCB field: `effectScale` (Single)
    #[serde(default)]
    pub effect_scale: f32,
    /// DCB field: `useRandomScale` (Boolean)
    #[serde(default)]
    pub use_random_scale: bool,
    /// DCB field: `effectScaleMin` (Single)
    #[serde(default)]
    pub effect_scale_min: f32,
    /// DCB field: `effectScaleMax` (Single)
    #[serde(default)]
    pub effect_scale_max: f32,
    /// DCB field: `damage` (StrongPointer)
    #[serde(default)]
    pub damage: Option<Handle<DamageBase>>,
    /// DCB field: `flashbangParams` (StrongPointer)
    #[serde(default)]
    pub flashbang_params: Option<Handle<ExplosionFlashbangParams>>,
    /// DCB field: `shockwaveParams` (StrongPointer)
    #[serde(default)]
    pub shockwave_params: Option<Handle<ShockwaveParams>>,
    /// DCB field: `fragmentParams` (StrongPointer)
    #[serde(default)]
    pub fragment_params: Option<Handle<ExplosiveFragmentParams>>,
    /// DCB field: `hitType` (String)
    #[serde(default)]
    pub hit_type: String,
    /// DCB field: `effect` (Class)
    #[serde(default)]
    pub effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `failedEffect` (Class)
    #[serde(default)]
    pub failed_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `customMaterialEffect` (String)
    #[serde(default)]
    pub custom_material_effect: String,
    /// DCB field: `sound` (Class)
    #[serde(default)]
    pub sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `failedSound` (Class)
    #[serde(default)]
    pub failed_sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `materialEffectEntry` (StrongPointer)
    #[serde(default)]
    pub material_effect_entry: Option<Handle<MaterialEffectEntry>>,
    /// DCB field: `Offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// DCB field: `Direction` (Class)
    #[serde(default)]
    pub direction: Option<Handle<Vec3>>,
    /// DCB field: `particleStrength` (Single)
    #[serde(default)]
    pub particle_strength: f32,
    /// DCB field: `radarContactType` (Reference)
    #[serde(default)]
    pub radar_contact_type: Option<CigGuid>,
}

impl Pooled for ExplosionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ex.explosion_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ex.explosion_params }
}

impl<'a> Extract<'a> for ExplosionParams {
    const TYPE_NAME: &'static str = "ExplosionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            friendly_fire: inst.get_str("friendlyFire").map(String::from).unwrap_or_default(),
            min_radius: inst.get_f32("minRadius").unwrap_or_default(),
            max_radius: inst.get_f32("maxRadius").unwrap_or_default(),
            sound_radius: inst.get_f32("soundRadius").unwrap_or_default(),
            min_phys_radius: inst.get_f32("minPhysRadius").unwrap_or_default(),
            max_phys_radius: inst.get_f32("maxPhysRadius").unwrap_or_default(),
            angle: inst.get_f32("angle").unwrap_or_default(),
            angle_vertical: inst.get_f32("angleVertical").unwrap_or_default(),
            explosion_front: match inst.get("explosionFront") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            explosion_up: match inst.get("explosionUp") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            explosion_zone: inst.get_str("explosionZone").map(String::from).unwrap_or_default(),
            pressure: inst.get_f32("pressure").unwrap_or_default(),
            hole_size: inst.get_f32("holeSize").unwrap_or_default(),
            terrain_hole_size: inst.get_f32("terrainHoleSize").unwrap_or_default(),
            maxblurdist: inst.get_f32("maxblurdist").unwrap_or_default(),
            effect_scale: inst.get_f32("effectScale").unwrap_or_default(),
            use_random_scale: inst.get_bool("useRandomScale").unwrap_or_default(),
            effect_scale_min: inst.get_f32("effectScaleMin").unwrap_or_default(),
            effect_scale_max: inst.get_f32("effectScaleMax").unwrap_or_default(),
            damage: match inst.get("damage") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            flashbang_params: match inst.get("flashbangParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosionFlashbangParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosionFlashbangParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shockwave_params: match inst.get("shockwaveParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ShockwaveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ShockwaveParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fragment_params: match inst.get("fragmentParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosiveFragmentParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosiveFragmentParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hit_type: inst.get_str("hitType").map(String::from).unwrap_or_default(),
            effect: match inst.get("effect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            failed_effect: match inst.get("failedEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            custom_material_effect: inst.get_str("customMaterialEffect").map(String::from).unwrap_or_default(),
            sound: match inst.get("sound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            failed_sound: match inst.get("failedSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material_effect_entry: match inst.get("materialEffectEntry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MaterialEffectEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MaterialEffectEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset: match inst.get("Offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            direction: match inst.get("Direction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            particle_strength: inst.get_f32("particleStrength").unwrap_or_default(),
            radar_contact_type: inst.get("radarContactType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ExplosiveOrdnancePingVFX`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosiveOrdnancePingVFX {
    /// DCB field: `pingSphereGeometry` (Class)
    #[serde(default)]
    pub ping_sphere_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `pingMaterial` (Class)
    #[serde(default)]
    pub ping_material: Option<Handle<GlobalResourceMaterial>>,
    /// DCB field: `pingColor` (Class)
    #[serde(default)]
    pub ping_color: Option<Handle<RGB>>,
    /// DCB field: `pingBrightness` (Single)
    #[serde(default)]
    pub ping_brightness: f32,
}

impl Pooled for ExplosiveOrdnancePingVFX {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ex.explosive_ordnance_ping_vfx }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ex.explosive_ordnance_ping_vfx }
}

impl<'a> Extract<'a> for ExplosiveOrdnancePingVFX {
    const TYPE_NAME: &'static str = "ExplosiveOrdnancePingVFX";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ping_sphere_geometry: match inst.get("pingSphereGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_material: match inst.get("pingMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_color: match inst.get("pingColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ping_brightness: inst.get_f32("pingBrightness").unwrap_or_default(),
        }
    }
}

/// DCB type: `ExplosiveOrdnancePingGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplosiveOrdnancePingGlobalParams {
    /// DCB field: `vfxGhostPingParams` (Class)
    #[serde(default)]
    pub vfx_ghost_ping_params: Option<Handle<ExplosiveOrdnancePingVFX>>,
    /// DCB field: `vfxDesiredPingParams` (Class)
    #[serde(default)]
    pub vfx_desired_ping_params: Option<Handle<ExplosiveOrdnancePingVFX>>,
    /// DCB field: `vfxPredictedPingParams` (Class)
    #[serde(default)]
    pub vfx_predicted_ping_params: Option<Handle<ExplosiveOrdnancePingVFX>>,
}

impl Pooled for ExplosiveOrdnancePingGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ex.explosive_ordnance_ping_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ex.explosive_ordnance_ping_global_params }
}

impl<'a> Extract<'a> for ExplosiveOrdnancePingGlobalParams {
    const TYPE_NAME: &'static str = "ExplosiveOrdnancePingGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vfx_ghost_ping_params: match inst.get("vfxGhostPingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vfx_desired_ping_params: match inst.get("vfxDesiredPingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vfx_predicted_ping_params: match inst.get("vfxPredictedPingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosiveOrdnancePingVFX>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

