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

/// DCB type: `AmmoParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmoParams {
    /// DCB field: `spawnType` (EnumChoice)
    #[serde(default)]
    pub spawn_type: String,
    /// DCB field: `size` (Byte)
    #[serde(default)]
    pub size: u32,
    /// DCB field: `ammoCategory` (EnumChoice)
    #[serde(default)]
    pub ammo_category: String,
    /// DCB field: `UIIconType` (String)
    #[serde(default)]
    pub uiicon_type: String,
    /// DCB field: `hitPoints` (Single)
    #[serde(default)]
    pub hit_points: f32,
    /// DCB field: `lifetime` (Single)
    #[serde(default)]
    pub lifetime: f32,
    /// DCB field: `showtime` (Single)
    #[serde(default)]
    pub showtime: f32,
    /// DCB field: `inheritVelocity` (Single)
    #[serde(default)]
    pub inherit_velocity: f32,
    /// DCB field: `bulletType` (Int32)
    #[serde(default)]
    pub bullet_type: i32,
    /// DCB field: `speed` (Single)
    #[serde(default)]
    pub speed: f32,
    /// DCB field: `impulseScale` (Single)
    #[serde(default)]
    pub impulse_scale: f32,
    /// DCB field: `noBulletHits` (Boolean)
    #[serde(default)]
    pub no_bullet_hits: bool,
    /// DCB field: `quietRemoval` (Boolean)
    #[serde(default)]
    pub quiet_removal: bool,
    /// DCB field: `whizSound` (Class)
    #[serde(default)]
    pub whiz_sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `whizSoundDistance` (Single)
    #[serde(default)]
    pub whiz_sound_distance: f32,
    /// DCB field: `ricochetSound` (Class)
    #[serde(default)]
    pub ricochet_sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `useInConvergence` (Boolean)
    #[serde(default)]
    pub use_in_convergence: bool,
    /// DCB field: `projectileLoopStart` (Class)
    #[serde(default)]
    pub projectile_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `projectileLoopStop` (Class)
    #[serde(default)]
    pub projectile_loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `shotsPerAudioLoop` (UInt32)
    #[serde(default)]
    pub shots_per_audio_loop: u32,
    /// DCB field: `trailParticles` (Class)
    #[serde(default)]
    pub trail_particles: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `geometryResourceParams` (StrongPointer)
    #[serde(default)]
    pub geometry_resource_params: Option<Handle<SGeometryResourceParams>>,
    /// DCB field: `geometryTransformParams` (StrongPointer)
    #[serde(default)]
    pub geometry_transform_params: Option<Handle<GeometryTransformParams>>,
    /// DCB field: `physicsControllerParams` (StrongPointer)
    #[serde(default)]
    pub physics_controller_params: Option<Handle<SEntityPhysicsControllerParams>>,
    /// DCB field: `lightPoolParams` (StrongPointer)
    #[serde(default)]
    pub light_pool_params: Option<Handle<PooledLightData>>,
    /// DCB field: `projectileParams` (StrongPointer)
    #[serde(default)]
    pub projectile_params: Option<Handle<ProjectileParams>>,
    /// DCB field: `radarObjectParams` (StrongPointer)
    #[serde(default)]
    pub radar_object_params: Option<Handle<SSCSignatureSystemParams>>,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
}

impl Pooled for AmmoParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_am.ammo_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_am.ammo_params }
}

impl<'a> Extract<'a> for AmmoParams {
    const TYPE_NAME: &'static str = "AmmoParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spawn_type: inst.get_str("spawnType").map(String::from).unwrap_or_default(),
            size: inst.get_u32("size").unwrap_or_default(),
            ammo_category: inst.get_str("ammoCategory").map(String::from).unwrap_or_default(),
            uiicon_type: inst.get_str("UIIconType").map(String::from).unwrap_or_default(),
            hit_points: inst.get_f32("hitPoints").unwrap_or_default(),
            lifetime: inst.get_f32("lifetime").unwrap_or_default(),
            showtime: inst.get_f32("showtime").unwrap_or_default(),
            inherit_velocity: inst.get_f32("inheritVelocity").unwrap_or_default(),
            bullet_type: inst.get_i32("bulletType").unwrap_or_default(),
            speed: inst.get_f32("speed").unwrap_or_default(),
            impulse_scale: inst.get_f32("impulseScale").unwrap_or_default(),
            no_bullet_hits: inst.get_bool("noBulletHits").unwrap_or_default(),
            quiet_removal: inst.get_bool("quietRemoval").unwrap_or_default(),
            whiz_sound: match inst.get("whizSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            whiz_sound_distance: inst.get_f32("whizSoundDistance").unwrap_or_default(),
            ricochet_sound: match inst.get("ricochetSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            use_in_convergence: inst.get_bool("useInConvergence").unwrap_or_default(),
            projectile_loop_start: match inst.get("projectileLoopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            projectile_loop_stop: match inst.get("projectileLoopStop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shots_per_audio_loop: inst.get_u32("shotsPerAudioLoop").unwrap_or_default(),
            trail_particles: match inst.get("trailParticles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            geometry_resource_params: match inst.get("geometryResourceParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryResourceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryResourceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            geometry_transform_params: match inst.get("geometryTransformParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GeometryTransformParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GeometryTransformParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            physics_controller_params: match inst.get("physicsControllerParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityPhysicsControllerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityPhysicsControllerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            light_pool_params: match inst.get("lightPoolParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PooledLightData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PooledLightData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            projectile_params: match inst.get("projectileParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProjectileParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProjectileParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            radar_object_params: match inst.get("radarObjectParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureSystemParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureSystemParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
        }
    }
}

