// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ammoparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ProjectileParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectileParams {
    /// `detonationParams` (StrongPointer)
    #[serde(default)]
    pub detonation_params: Option<Handle<ProjectileDetonationParams>>,
    /// `proximityTriggerParams` (StrongPointer)
    #[serde(default)]
    pub proximity_trigger_params: Option<Handle<ProjectileProximityTriggerParams>>,
}

impl Pooled for ProjectileParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.projectile_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.projectile_params }
}

impl<'a> Extract<'a> for ProjectileParams {
    const TYPE_NAME: &'static str = "ProjectileParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            detonation_params: match inst.get("detonationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProjectileDetonationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProjectileDetonationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            proximity_trigger_params: match inst.get("proximityTriggerParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProjectileProximityTriggerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProjectileProximityTriggerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProjectileSpawnedEntityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectileSpawnedEntityParams {
    /// `entityToSpawnOnCollision` (Reference)
    #[serde(default)]
    pub entity_to_spawn_on_collision: Option<CigGuid>,
    /// `oneShot` (Boolean)
    #[serde(default)]
    pub one_shot: bool,
}

impl Pooled for ProjectileSpawnedEntityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.projectile_spawned_entity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.projectile_spawned_entity_params }
}

impl<'a> Extract<'a> for ProjectileSpawnedEntityParams {
    const TYPE_NAME: &'static str = "ProjectileSpawnedEntityParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_to_spawn_on_collision: inst.get("entityToSpawnOnCollision").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            one_shot: inst.get_bool("oneShot").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProjectileDetonationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectileDetonationParams {
    /// `armTime` (Single)
    #[serde(default)]
    pub arm_time: f32,
    /// `safeDistance` (Single)
    #[serde(default)]
    pub safe_distance: f32,
    /// `destructDelay` (Single)
    #[serde(default)]
    pub destruct_delay: f32,
    /// `explodeOnImpact` (Boolean)
    #[serde(default)]
    pub explode_on_impact: bool,
    /// `explodeOnFinalImpact` (Boolean)
    #[serde(default)]
    pub explode_on_final_impact: bool,
    /// `explodeOnExpire` (Boolean)
    #[serde(default)]
    pub explode_on_expire: bool,
    /// `explodeOnTargetRange` (Boolean)
    #[serde(default)]
    pub explode_on_target_range: bool,
    /// `explosionParams` (Class)
    #[serde(default)]
    pub explosion_params: Option<Handle<ExplosionParams>>,
    /// `spawnedEntityParams` (Class)
    #[serde(default)]
    pub spawned_entity_params: Option<Handle<ProjectileSpawnedEntityParams>>,
}

impl Pooled for ProjectileDetonationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.projectile_detonation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.projectile_detonation_params }
}

impl<'a> Extract<'a> for ProjectileDetonationParams {
    const TYPE_NAME: &'static str = "ProjectileDetonationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            arm_time: inst.get_f32("armTime").unwrap_or_default(),
            safe_distance: inst.get_f32("safeDistance").unwrap_or_default(),
            destruct_delay: inst.get_f32("destructDelay").unwrap_or_default(),
            explode_on_impact: inst.get_bool("explodeOnImpact").unwrap_or_default(),
            explode_on_final_impact: inst.get_bool("explodeOnFinalImpact").unwrap_or_default(),
            explode_on_expire: inst.get_bool("explodeOnExpire").unwrap_or_default(),
            explode_on_target_range: inst.get_bool("explodeOnTargetRange").unwrap_or_default(),
            explosion_params: match inst.get("explosionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ExplosionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ExplosionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spawned_entity_params: match inst.get("spawnedEntityParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProjectileSpawnedEntityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProjectileSpawnedEntityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProjectileProximityTriggerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectileProximityTriggerParams {
    /// `armTime` (Single)
    #[serde(default)]
    pub arm_time: f32,
    /// `safeDistance` (Single)
    #[serde(default)]
    pub safe_distance: f32,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
}

impl Pooled for ProjectileProximityTriggerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.projectile_proximity_trigger_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.projectile_proximity_trigger_params }
}

impl<'a> Extract<'a> for ProjectileProximityTriggerParams {
    const TYPE_NAME: &'static str = "ProjectileProximityTriggerParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            arm_time: inst.get_f32("armTime").unwrap_or_default(),
            safe_distance: inst.get_f32("safeDistance").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
        }
    }
}

/// DCB type: `GeometryTransformParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryTransformParams {
    /// `initAngles` (Class)
    #[serde(default)]
    pub init_angles: Option<Handle<Vec3>>,
    /// `randomAngles` (Class)
    #[serde(default)]
    pub random_angles: Option<Handle<Vec3>>,
    /// `rotationRate` (Class)
    #[serde(default)]
    pub rotation_rate: Option<Handle<Vec3>>,
    /// `randomRotation` (Class)
    #[serde(default)]
    pub random_rotation: Option<Handle<Vec3>>,
    /// `randomScale` (Class)
    #[serde(default)]
    pub random_scale: Option<Handle<Range>>,
}

impl Pooled for GeometryTransformParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.geometry_transform_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.geometry_transform_params }
}

impl<'a> Extract<'a> for GeometryTransformParams {
    const TYPE_NAME: &'static str = "GeometryTransformParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            init_angles: match inst.get("initAngles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_angles: match inst.get("randomAngles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_rate: match inst.get("rotationRate") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_rotation: match inst.get("randomRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_scale: match inst.get("randomScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AmmoParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmoParams {
    /// `spawnType` (EnumChoice)
    #[serde(default)]
    pub spawn_type: String,
    /// `size` (Byte)
    #[serde(default)]
    pub size: u32,
    /// `ammoCategory` (EnumChoice)
    #[serde(default)]
    pub ammo_category: String,
    /// `UIIconType` (String)
    #[serde(default)]
    pub uiicon_type: String,
    /// `hitPoints` (Single)
    #[serde(default)]
    pub hit_points: f32,
    /// `lifetime` (Single)
    #[serde(default)]
    pub lifetime: f32,
    /// `showtime` (Single)
    #[serde(default)]
    pub showtime: f32,
    /// `inheritVelocity` (Single)
    #[serde(default)]
    pub inherit_velocity: f32,
    /// `bulletType` (Int32)
    #[serde(default)]
    pub bullet_type: i32,
    /// `speed` (Single)
    #[serde(default)]
    pub speed: f32,
    /// `impulseScale` (Single)
    #[serde(default)]
    pub impulse_scale: f32,
    /// `noBulletHits` (Boolean)
    #[serde(default)]
    pub no_bullet_hits: bool,
    /// `quietRemoval` (Boolean)
    #[serde(default)]
    pub quiet_removal: bool,
    /// `whizSound` (Class)
    #[serde(default)]
    pub whiz_sound: Option<Handle<GlobalResourceAudio>>,
    /// `whizSoundDistance` (Single)
    #[serde(default)]
    pub whiz_sound_distance: f32,
    /// `ricochetSound` (Class)
    #[serde(default)]
    pub ricochet_sound: Option<Handle<GlobalResourceAudio>>,
    /// `useInConvergence` (Boolean)
    #[serde(default)]
    pub use_in_convergence: bool,
    /// `projectileLoopStart` (Class)
    #[serde(default)]
    pub projectile_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `projectileLoopStop` (Class)
    #[serde(default)]
    pub projectile_loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `shotsPerAudioLoop` (UInt32)
    #[serde(default)]
    pub shots_per_audio_loop: u32,
    /// `trailParticles` (Class)
    #[serde(default)]
    pub trail_particles: Option<Handle<GlobalResourceParticle>>,
    /// `geometryResourceParams` (StrongPointer)
    #[serde(default)]
    pub geometry_resource_params: Option<Handle<SGeometryResourceParams>>,
    /// `geometryTransformParams` (StrongPointer)
    #[serde(default)]
    pub geometry_transform_params: Option<Handle<GeometryTransformParams>>,
    /// `physicsControllerParams` (StrongPointer)
    #[serde(default)]
    pub physics_controller_params: Option<Handle<SEntityPhysicsControllerParams>>,
    /// `lightPoolParams` (StrongPointer)
    #[serde(default)]
    pub light_pool_params: Option<Handle<PooledLightData>>,
    /// `projectileParams` (StrongPointer)
    #[serde(default)]
    pub projectile_params: Option<Handle<ProjectileParams>>,
    /// `radarObjectParams` (StrongPointer)
    #[serde(default)]
    pub radar_object_params: Option<Handle<SSCSignatureSystemParams>>,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
}

impl Pooled for AmmoParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.ammo_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.ammo_params }
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

/// DCB type: `SSoftbodyGeometryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSoftbodyGeometryParams {
    /// `Geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `SimulationParams` (Class)
    #[serde(default)]
    pub simulation_params: Option<Handle<SEntitySoftExPhysicsControllerParams>>,
}

impl Pooled for SSoftbodyGeometryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.ssoftbody_geometry_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.ssoftbody_geometry_params }
}

impl<'a> Extract<'a> for SSoftbodyGeometryParams {
    const TYPE_NAME: &'static str = "SSoftbodyGeometryParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            simulation_params: match inst.get("SimulationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntitySoftExPhysicsControllerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntitySoftExPhysicsControllerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGeometryDataParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryDataParams {
    /// `Geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `Material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
    /// `SimulationGeometry` (StrongPointer)
    #[serde(default)]
    pub simulation_geometry: Option<Handle<SSoftbodyGeometryParams>>,
    /// `Palette` (Class)
    #[serde(default)]
    pub palette: Option<Handle<TintPaletteRef>>,
    /// `Slot` (EnumChoice)
    #[serde(default)]
    pub slot: String,
    /// `MaterialAttachments` (String)
    #[serde(default)]
    pub material_attachments: String,
    /// `ProxyCDFPath` (String)
    #[serde(default)]
    pub proxy_cdfpath: String,
    /// `ModifiersPath` (String)
    #[serde(default)]
    pub modifiers_path: String,
    /// `AttachFlags` (UInt32)
    #[serde(default)]
    pub attach_flags: u32,
    /// `DeformerType` (EnumChoice)
    #[serde(default)]
    pub deformer_type: String,
    /// `ProtosBShapeExclude` (UInt16)
    #[serde(default)]
    pub protos_bshape_exclude: u32,
    /// `VisTP` (Boolean)
    #[serde(default)]
    pub vis_tp: bool,
    /// `VisFP` (Boolean)
    #[serde(default)]
    pub vis_fp: bool,
    /// `VisShadow` (Boolean)
    #[serde(default)]
    pub vis_shadow: bool,
    /// `VisSecondaryViews` (Boolean)
    #[serde(default)]
    pub vis_secondary_views: bool,
    /// `WrinkleMap` (Boolean)
    #[serde(default)]
    pub wrinkle_map: bool,
    /// `EnableDecalProjection` (Boolean)
    #[serde(default)]
    pub enable_decal_projection: bool,
    /// `BBoxJoint` (String)
    #[serde(default)]
    pub bbox_joint: String,
    /// `BBoxRadius` (Single)
    #[serde(default)]
    pub bbox_radius: f32,
    /// `Wear` (Single)
    #[serde(default)]
    pub wear: f32,
    /// `Dirt` (Single)
    #[serde(default)]
    pub dirt: f32,
    /// `Interference` (Single)
    #[serde(default)]
    pub interference: f32,
    /// `Damage` (Single)
    #[serde(default)]
    pub damage: f32,
    /// `RenderLayer` (EnumChoice)
    #[serde(default)]
    pub render_layer: String,
    /// `VisAreaMode` (EnumChoice)
    #[serde(default)]
    pub vis_area_mode: String,
    /// `SunShadowMode` (EnumChoice)
    #[serde(default)]
    pub sun_shadow_mode: String,
    /// `viewDistRatio` (Reference)
    #[serde(default)]
    pub view_dist_ratio: Option<CigGuid>,
    /// `LodRatio` (Byte)
    #[serde(default)]
    pub lod_ratio: u32,
}

impl Pooled for SGeometryDataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sgeometry_data_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sgeometry_data_params }
}

impl<'a> Extract<'a> for SGeometryDataParams {
    const TYPE_NAME: &'static str = "SGeometryDataParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("Material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            simulation_geometry: match inst.get("SimulationGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSoftbodyGeometryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSoftbodyGeometryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            palette: match inst.get("Palette") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintPaletteRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintPaletteRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            slot: inst.get_str("Slot").map(String::from).unwrap_or_default(),
            material_attachments: inst.get_str("MaterialAttachments").map(String::from).unwrap_or_default(),
            proxy_cdfpath: inst.get_str("ProxyCDFPath").map(String::from).unwrap_or_default(),
            modifiers_path: inst.get_str("ModifiersPath").map(String::from).unwrap_or_default(),
            attach_flags: inst.get_u32("AttachFlags").unwrap_or_default(),
            deformer_type: inst.get_str("DeformerType").map(String::from).unwrap_or_default(),
            protos_bshape_exclude: inst.get_u32("ProtosBShapeExclude").unwrap_or_default(),
            vis_tp: inst.get_bool("VisTP").unwrap_or_default(),
            vis_fp: inst.get_bool("VisFP").unwrap_or_default(),
            vis_shadow: inst.get_bool("VisShadow").unwrap_or_default(),
            vis_secondary_views: inst.get_bool("VisSecondaryViews").unwrap_or_default(),
            wrinkle_map: inst.get_bool("WrinkleMap").unwrap_or_default(),
            enable_decal_projection: inst.get_bool("EnableDecalProjection").unwrap_or_default(),
            bbox_joint: inst.get_str("BBoxJoint").map(String::from).unwrap_or_default(),
            bbox_radius: inst.get_f32("BBoxRadius").unwrap_or_default(),
            wear: inst.get_f32("Wear").unwrap_or_default(),
            dirt: inst.get_f32("Dirt").unwrap_or_default(),
            interference: inst.get_f32("Interference").unwrap_or_default(),
            damage: inst.get_f32("Damage").unwrap_or_default(),
            render_layer: inst.get_str("RenderLayer").map(String::from).unwrap_or_default(),
            vis_area_mode: inst.get_str("VisAreaMode").map(String::from).unwrap_or_default(),
            sun_shadow_mode: inst.get_str("SunShadowMode").map(String::from).unwrap_or_default(),
            view_dist_ratio: inst.get("viewDistRatio").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            lod_ratio: inst.get_u32("LodRatio").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGeometryNodeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryNodeParams {
    /// `Tags` (String)
    #[serde(default)]
    pub tags: String,
    /// `Geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<SGeometryDataParams>>,
    /// `ScaleMultiplier` (Single)
    #[serde(default)]
    pub scale_multiplier: f32,
    /// `SubGeometry` (Class (array))
    #[serde(default)]
    pub sub_geometry: Vec<Handle<SGeometryNodeParams>>,
}

impl Pooled for SGeometryNodeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sgeometry_node_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sgeometry_node_params }
}

impl<'a> Extract<'a> for SGeometryNodeParams {
    const TYPE_NAME: &'static str = "SGeometryNodeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_str("Tags").map(String::from).unwrap_or_default(),
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryDataParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryDataParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scale_multiplier: inst.get_f32("ScaleMultiplier").unwrap_or_default(),
            sub_geometry: inst.get_array("SubGeometry")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SGeometryNodeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SGeometryNodeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SMaterialNodeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMaterialNodeParams {
    /// `Tags` (String)
    #[serde(default)]
    pub tags: String,
    /// `Material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
    /// `Palette` (Class)
    #[serde(default)]
    pub palette: Option<Handle<TintPaletteRef>>,
    /// `materialVariants` (Class (array))
    #[serde(default)]
    pub material_variants: Vec<Handle<SMaterialNodeParams>>,
}

impl Pooled for SMaterialNodeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.smaterial_node_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.smaterial_node_params }
}

impl<'a> Extract<'a> for SMaterialNodeParams {
    const TYPE_NAME: &'static str = "SMaterialNodeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_str("Tags").map(String::from).unwrap_or_default(),
            material: match inst.get("Material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            palette: match inst.get("Palette") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintPaletteRef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintPaletteRef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material_variants: inst.get_array("materialVariants")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SMaterialNodeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SMaterialNodeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SGeometryMeshsetupParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryMeshsetupParams {
    /// `visibilityTags` (Class)
    #[serde(default)]
    pub visibility_tags: Option<Handle<TagList>>,
}

impl Pooled for SGeometryMeshsetupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sgeometry_meshsetup_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sgeometry_meshsetup_params }
}

impl<'a> Extract<'a> for SGeometryMeshsetupParams {
    const TYPE_NAME: &'static str = "SGeometryMeshsetupParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            visibility_tags: match inst.get("visibilityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGeometryResourceParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryResourceParams {
    /// `ModelTag` (StrongPointer)
    #[serde(default)]
    pub model_tag: Option<Handle<SGeometryModelTagBase>>,
    /// `cacheResources` (Boolean)
    #[serde(default)]
    pub cache_resources: bool,
    /// `meshsetup` (Class)
    #[serde(default)]
    pub meshsetup: Option<Handle<SGeometryMeshsetupParams>>,
    /// `Geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<SGeometryNodeParams>>,
    /// `Material` (Class)
    #[serde(default)]
    pub material: Option<Handle<SMaterialNodeParams>>,
    /// `rootOverridePaint` (Boolean)
    #[serde(default)]
    pub root_override_paint: bool,
    /// `inheritModelTagFromHost` (Boolean)
    #[serde(default)]
    pub inherit_model_tag_from_host: bool,
}

impl Pooled for SGeometryResourceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sgeometry_resource_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sgeometry_resource_params }
}

impl<'a> Extract<'a> for SGeometryResourceParams {
    const TYPE_NAME: &'static str = "SGeometryResourceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            model_tag: match inst.get("ModelTag") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryModelTagBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryModelTagBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cache_resources: inst.get_bool("cacheResources").unwrap_or_default(),
            meshsetup: match inst.get("meshsetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryMeshsetupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryMeshsetupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGeometryNodeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGeometryNodeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("Material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMaterialNodeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMaterialNodeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            root_override_paint: inst.get_bool("rootOverridePaint").unwrap_or_default(),
            inherit_model_tag_from_host: inst.get_bool("inheritModelTagFromHost").unwrap_or_default(),
        }
    }
}

/// DCB type: `PooledLightData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PooledLightData {
    /// `flareName` (String)
    #[serde(default)]
    pub flare_name: String,
    /// `flareScale` (Single)
    #[serde(default)]
    pub flare_scale: f32,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `diffuseColor` (Class)
    #[serde(default)]
    pub diffuse_color: Option<Handle<RGB>>,
    /// `diffuseMultiplier` (Single)
    #[serde(default)]
    pub diffuse_multiplier: f32,
    /// `specularMultiplier` (Single)
    #[serde(default)]
    pub specular_multiplier: f32,
    /// `attenuationBulbSize` (Single)
    #[serde(default)]
    pub attenuation_bulb_size: f32,
    /// `animSpeed` (Single)
    #[serde(default)]
    pub anim_speed: f32,
    /// `rampTime` (Single)
    #[serde(default)]
    pub ramp_time: f32,
    /// `fake` (Boolean)
    #[serde(default)]
    pub fake: bool,
    /// `autoClip` (Boolean)
    #[serde(default)]
    pub auto_clip: bool,
    /// `style` (Byte)
    #[serde(default)]
    pub style: u32,
    /// `animPhase` (Byte)
    #[serde(default)]
    pub anim_phase: u32,
    /// `flareLensOpticsFrustumAngle` (Byte)
    #[serde(default)]
    pub flare_lens_optics_frustum_angle: u32,
}

impl Pooled for PooledLightData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.pooled_light_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.pooled_light_data }
}

impl<'a> Extract<'a> for PooledLightData {
    const TYPE_NAME: &'static str = "PooledLightData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            flare_name: inst.get_str("flareName").map(String::from).unwrap_or_default(),
            flare_scale: inst.get_f32("flareScale").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            diffuse_color: match inst.get("diffuseColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            diffuse_multiplier: inst.get_f32("diffuseMultiplier").unwrap_or_default(),
            specular_multiplier: inst.get_f32("specularMultiplier").unwrap_or_default(),
            attenuation_bulb_size: inst.get_f32("attenuationBulbSize").unwrap_or_default(),
            anim_speed: inst.get_f32("animSpeed").unwrap_or_default(),
            ramp_time: inst.get_f32("rampTime").unwrap_or_default(),
            fake: inst.get_bool("fake").unwrap_or_default(),
            auto_clip: inst.get_bool("autoClip").unwrap_or_default(),
            style: inst.get_u32("style").unwrap_or_default(),
            anim_phase: inst.get_u32("animPhase").unwrap_or_default(),
            flare_lens_optics_frustum_angle: inst.get_u32("flareLensOpticsFrustumAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityPhysicsControllerParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityPhysicsControllerParams {
    /// `PhysType` (StrongPointer)
    #[serde(default)]
    pub phys_type: Option<Handle<SEntityBasePhysicsControllerParams>>,
}

impl Pooled for SEntityPhysicsControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sentity_physics_controller_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sentity_physics_controller_params }
}

impl<'a> Extract<'a> for SEntityPhysicsControllerParams {
    const TYPE_NAME: &'static str = "SEntityPhysicsControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            phys_type: match inst.get("PhysType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityBasePhysicsControllerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityBasePhysicsControllerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGameCollisionClass`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGameCollisionClass {
    /// `gameCollisionClass` (EnumChoice)
    #[serde(default)]
    pub game_collision_class: String,
    /// `gameIgnoreCollisionClass` (EnumChoice)
    #[serde(default)]
    pub game_ignore_collision_class: String,
}

impl Pooled for SGameCollisionClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sgame_collision_class }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sgame_collision_class }
}

impl<'a> Extract<'a> for SGameCollisionClass {
    const TYPE_NAME: &'static str = "SGameCollisionClass";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            game_collision_class: inst.get_str("gameCollisionClass").map(String::from).unwrap_or_default(),
            game_ignore_collision_class: inst.get_str("gameIgnoreCollisionClass").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SBreakablePhysicsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBreakablePhysicsParams {
    /// `maxSimultaneousCracks` (Int32)
    #[serde(default)]
    pub max_simultaneous_cracks: i32,
    /// `maxPushForce` (Single)
    #[serde(default)]
    pub max_push_force: f32,
    /// `maxPullForce` (Single)
    #[serde(default)]
    pub max_pull_force: f32,
    /// `maxShiftForce` (Single)
    #[serde(default)]
    pub max_shift_force: f32,
    /// `maxTwistTorque` (Single)
    #[serde(default)]
    pub max_twist_torque: f32,
    /// `maxBendTorque` (Single)
    #[serde(default)]
    pub max_bend_torque: f32,
    /// `crackWeaken` (Single)
    #[serde(default)]
    pub crack_weaken: f32,
}

impl Pooled for SBreakablePhysicsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sbreakable_physics_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sbreakable_physics_params }
}

impl<'a> Extract<'a> for SBreakablePhysicsParams {
    const TYPE_NAME: &'static str = "SBreakablePhysicsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_simultaneous_cracks: inst.get_i32("maxSimultaneousCracks").unwrap_or_default(),
            max_push_force: inst.get_f32("maxPushForce").unwrap_or_default(),
            max_pull_force: inst.get_f32("maxPullForce").unwrap_or_default(),
            max_shift_force: inst.get_f32("maxShiftForce").unwrap_or_default(),
            max_twist_torque: inst.get_f32("maxTwistTorque").unwrap_or_default(),
            max_bend_torque: inst.get_f32("maxBendTorque").unwrap_or_default(),
            crack_weaken: inst.get_f32("crackWeaken").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityBasePhysicsControllerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityBasePhysicsControllerParams {
    /// `Mass` (Single)
    #[serde(default)]
    pub mass: f32,
    /// `compoundingAllowed` (Boolean)
    #[serde(default)]
    pub compounding_allowed: bool,
    /// `breakableParams` (StrongPointer)
    #[serde(default)]
    pub breakable_params: Option<Handle<SBreakablePhysicsParams>>,
    /// `gameCollisionClass` (StrongPointer)
    #[serde(default)]
    pub game_collision_class: Option<Handle<SGameCollisionClass>>,
    /// `spawnBoxScale` (Single)
    #[serde(default)]
    pub spawn_box_scale: f32,
}

impl Pooled for SEntityBasePhysicsControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sentity_base_physics_controller_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sentity_base_physics_controller_params }
}

impl<'a> Extract<'a> for SEntityBasePhysicsControllerParams {
    const TYPE_NAME: &'static str = "SEntityBasePhysicsControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mass: inst.get_f32("Mass").unwrap_or_default(),
            compounding_allowed: inst.get_bool("compoundingAllowed").unwrap_or_default(),
            breakable_params: match inst.get("breakableParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBreakablePhysicsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBreakablePhysicsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            game_collision_class: match inst.get("gameCollisionClass") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGameCollisionClass>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGameCollisionClass>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spawn_box_scale: inst.get_f32("spawnBoxScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntitySoftExPhysicsControllerParams`
/// Inherits from: `SEntityBasePhysicsControllerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntitySoftExPhysicsControllerParams {
    /// `Mass` (Single)
    #[serde(default)]
    pub mass: f32,
    /// `compoundingAllowed` (Boolean)
    #[serde(default)]
    pub compounding_allowed: bool,
    /// `breakableParams` (StrongPointer)
    #[serde(default)]
    pub breakable_params: Option<Handle<SBreakablePhysicsParams>>,
    /// `gameCollisionClass` (StrongPointer)
    #[serde(default)]
    pub game_collision_class: Option<Handle<SGameCollisionClass>>,
    /// `spawnBoxScale` (Single)
    #[serde(default)]
    pub spawn_box_scale: f32,
    /// `damping` (Single)
    #[serde(default)]
    pub damping: f32,
    /// `stretchStiffness` (Single)
    #[serde(default)]
    pub stretch_stiffness: f32,
    /// `compressStiffness` (Single)
    #[serde(default)]
    pub compress_stiffness: f32,
    /// `bendingStiffness` (Single)
    #[serde(default)]
    pub bending_stiffness: f32,
    /// `cosseratShearXStiffness` (Single)
    #[serde(default)]
    pub cosserat_shear_xstiffness: f32,
    /// `cosseratShearYStiffness` (Single)
    #[serde(default)]
    pub cosserat_shear_ystiffness: f32,
    /// `cosseratStretchStiffness` (Single)
    #[serde(default)]
    pub cosserat_stretch_stiffness: f32,
    /// `cosseratBendXStiffness` (Single)
    #[serde(default)]
    pub cosserat_bend_xstiffness: f32,
    /// `cosseratBendYStiffness` (Single)
    #[serde(default)]
    pub cosserat_bend_ystiffness: f32,
    /// `cosseratTwistingStiffness` (Single)
    #[serde(default)]
    pub cosserat_twisting_stiffness: f32,
    /// `attachmentInfluence` (Single)
    #[serde(default)]
    pub attachment_influence: f32,
    /// `maxDisplacementInfluence` (Single)
    #[serde(default)]
    pub max_displacement_influence: f32,
    /// `maxStretchAttach` (Single)
    #[serde(default)]
    pub max_stretch_attach: f32,
    /// `tetraVolStiffness` (Single)
    #[serde(default)]
    pub tetra_vol_stiffness: f32,
    /// `collisionGap` (Single)
    #[serde(default)]
    pub collision_gap: f32,
    /// `collisionThicknessFactor` (Single)
    #[serde(default)]
    pub collision_thickness_factor: f32,
    /// `staticFriction` (Single)
    #[serde(default)]
    pub static_friction: f32,
    /// `dynamicFriction` (Single)
    #[serde(default)]
    pub dynamic_friction: f32,
    /// `fixedStep` (Single)
    #[serde(default)]
    pub fixed_step: f32,
    /// `totalMass` (Single)
    #[serde(default)]
    pub total_mass: f32,
    /// `relativeDeltaScale` (Single)
    #[serde(default)]
    pub relative_delta_scale: f32,
    /// `bindingOffset` (Single)
    #[serde(default)]
    pub binding_offset: f32,
    /// `enforceLength` (Boolean)
    #[serde(default)]
    pub enforce_length: bool,
    /// `enforceBending` (Boolean)
    #[serde(default)]
    pub enforce_bending: bool,
    /// `enforceCosseratStrechShear` (Boolean)
    #[serde(default)]
    pub enforce_cosserat_strech_shear: bool,
    /// `enforceCosseratBendTwist` (Boolean)
    #[serde(default)]
    pub enforce_cosserat_bend_twist: bool,
    /// `enforceAttachment` (Boolean)
    #[serde(default)]
    pub enforce_attachment: bool,
    /// `enforceMaxDisplacement` (Boolean)
    #[serde(default)]
    pub enforce_max_displacement: bool,
    /// `enforceTetraVol` (Boolean)
    #[serde(default)]
    pub enforce_tetra_vol: bool,
    /// `enableCollisions` (Boolean)
    #[serde(default)]
    pub enable_collisions: bool,
    /// `enableFriction` (Boolean)
    #[serde(default)]
    pub enable_friction: bool,
    /// `enableSelfCollision` (Boolean)
    #[serde(default)]
    pub enable_self_collision: bool,
    /// `normalizeParticleSize` (Boolean)
    #[serde(default)]
    pub normalize_particle_size: bool,
    /// `maxDisplacementRestrictToPositiveHemisphere` (Boolean)
    #[serde(default)]
    pub max_displacement_restrict_to_positive_hemisphere: bool,
    /// `iterations` (Int32)
    #[serde(default)]
    pub iterations: i32,
    /// `gridDim` (Int32)
    #[serde(default)]
    pub grid_dim: i32,
    /// `lift` (Single)
    #[serde(default)]
    pub lift: f32,
    /// `drag` (Single)
    #[serde(default)]
    pub drag: f32,
    /// `windVariance` (Single)
    #[serde(default)]
    pub wind_variance: f32,
    /// `airResistance` (Single)
    #[serde(default)]
    pub air_resistance: f32,
    /// `waterResistance` (Single)
    #[serde(default)]
    pub water_resistance: f32,
    /// `substepMode` (EnumChoice)
    #[serde(default)]
    pub substep_mode: String,
    /// `visualBindingMode` (EnumChoice)
    #[serde(default)]
    pub visual_binding_mode: String,
}

impl Pooled for SEntitySoftExPhysicsControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sentity_soft_ex_physics_controller_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sentity_soft_ex_physics_controller_params }
}

impl<'a> Extract<'a> for SEntitySoftExPhysicsControllerParams {
    const TYPE_NAME: &'static str = "SEntitySoftExPhysicsControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mass: inst.get_f32("Mass").unwrap_or_default(),
            compounding_allowed: inst.get_bool("compoundingAllowed").unwrap_or_default(),
            breakable_params: match inst.get("breakableParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBreakablePhysicsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBreakablePhysicsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            game_collision_class: match inst.get("gameCollisionClass") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGameCollisionClass>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGameCollisionClass>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spawn_box_scale: inst.get_f32("spawnBoxScale").unwrap_or_default(),
            damping: inst.get_f32("damping").unwrap_or_default(),
            stretch_stiffness: inst.get_f32("stretchStiffness").unwrap_or_default(),
            compress_stiffness: inst.get_f32("compressStiffness").unwrap_or_default(),
            bending_stiffness: inst.get_f32("bendingStiffness").unwrap_or_default(),
            cosserat_shear_xstiffness: inst.get_f32("cosseratShearXStiffness").unwrap_or_default(),
            cosserat_shear_ystiffness: inst.get_f32("cosseratShearYStiffness").unwrap_or_default(),
            cosserat_stretch_stiffness: inst.get_f32("cosseratStretchStiffness").unwrap_or_default(),
            cosserat_bend_xstiffness: inst.get_f32("cosseratBendXStiffness").unwrap_or_default(),
            cosserat_bend_ystiffness: inst.get_f32("cosseratBendYStiffness").unwrap_or_default(),
            cosserat_twisting_stiffness: inst.get_f32("cosseratTwistingStiffness").unwrap_or_default(),
            attachment_influence: inst.get_f32("attachmentInfluence").unwrap_or_default(),
            max_displacement_influence: inst.get_f32("maxDisplacementInfluence").unwrap_or_default(),
            max_stretch_attach: inst.get_f32("maxStretchAttach").unwrap_or_default(),
            tetra_vol_stiffness: inst.get_f32("tetraVolStiffness").unwrap_or_default(),
            collision_gap: inst.get_f32("collisionGap").unwrap_or_default(),
            collision_thickness_factor: inst.get_f32("collisionThicknessFactor").unwrap_or_default(),
            static_friction: inst.get_f32("staticFriction").unwrap_or_default(),
            dynamic_friction: inst.get_f32("dynamicFriction").unwrap_or_default(),
            fixed_step: inst.get_f32("fixedStep").unwrap_or_default(),
            total_mass: inst.get_f32("totalMass").unwrap_or_default(),
            relative_delta_scale: inst.get_f32("relativeDeltaScale").unwrap_or_default(),
            binding_offset: inst.get_f32("bindingOffset").unwrap_or_default(),
            enforce_length: inst.get_bool("enforceLength").unwrap_or_default(),
            enforce_bending: inst.get_bool("enforceBending").unwrap_or_default(),
            enforce_cosserat_strech_shear: inst.get_bool("enforceCosseratStrechShear").unwrap_or_default(),
            enforce_cosserat_bend_twist: inst.get_bool("enforceCosseratBendTwist").unwrap_or_default(),
            enforce_attachment: inst.get_bool("enforceAttachment").unwrap_or_default(),
            enforce_max_displacement: inst.get_bool("enforceMaxDisplacement").unwrap_or_default(),
            enforce_tetra_vol: inst.get_bool("enforceTetraVol").unwrap_or_default(),
            enable_collisions: inst.get_bool("enableCollisions").unwrap_or_default(),
            enable_friction: inst.get_bool("enableFriction").unwrap_or_default(),
            enable_self_collision: inst.get_bool("enableSelfCollision").unwrap_or_default(),
            normalize_particle_size: inst.get_bool("normalizeParticleSize").unwrap_or_default(),
            max_displacement_restrict_to_positive_hemisphere: inst.get_bool("maxDisplacementRestrictToPositiveHemisphere").unwrap_or_default(),
            iterations: inst.get_i32("iterations").unwrap_or_default(),
            grid_dim: inst.get_i32("gridDim").unwrap_or_default(),
            lift: inst.get_f32("lift").unwrap_or_default(),
            drag: inst.get_f32("drag").unwrap_or_default(),
            wind_variance: inst.get_f32("windVariance").unwrap_or_default(),
            air_resistance: inst.get_f32("airResistance").unwrap_or_default(),
            water_resistance: inst.get_f32("waterResistance").unwrap_or_default(),
            substep_mode: inst.get_str("substepMode").map(String::from).unwrap_or_default(),
            visual_binding_mode: inst.get_str("visualBindingMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioParams {
    /// `rulesets` (Reference (array))
    #[serde(default)]
    pub rulesets: Vec<CigGuid>,
}

impl Pooled for SSCSignatureSystemAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sscsignature_system_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sscsignature_system_audio_params }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioParams {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rulesets: inst.get_array("rulesets")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SSCSignatureSystemParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemParams {
    /// `bindingURLPrefix` (String)
    #[serde(default)]
    pub binding_urlprefix: String,
    /// `radarProperties` (StrongPointer)
    #[serde(default)]
    pub radar_properties: Option<Handle<SSCRadarContactProperites>>,
    /// `audioParams` (StrongPointer)
    #[serde(default)]
    pub audio_params: Option<Handle<SSCSignatureSystemAudioParams>>,
    /// `scanCustomData` (Reference)
    #[serde(default)]
    pub scan_custom_data: Option<CigGuid>,
    /// `embeddedScanInfo` (StrongPointer)
    #[serde(default)]
    pub embedded_scan_info: Option<Handle<ScanCustomDataInfo>>,
    /// `scanDisplayLayoutOverride` (Reference)
    #[serde(default)]
    pub scan_display_layout_override: Option<CigGuid>,
    /// `detectionTags` (Reference (array))
    #[serde(default)]
    pub detection_tags: Vec<CigGuid>,
    /// `isOverridden` (Boolean)
    #[serde(default)]
    pub is_overridden: bool,
    /// `overriddenSize` (Class)
    #[serde(default)]
    pub overridden_size: Option<Handle<Vec3>>,
    /// `enableDetectionOnItemPort` (Boolean)
    #[serde(default)]
    pub enable_detection_on_item_port: bool,
    /// `ignoreHighlightWhenDetectorInsideBounds` (Boolean)
    #[serde(default)]
    pub ignore_highlight_when_detector_inside_bounds: bool,
    /// `linkedObjectives` (Reference (array))
    #[serde(default)]
    pub linked_objectives: Vec<CigGuid>,
    /// `ignoreHighlightWhenNoLinkedOrActiveObjectives` (Boolean)
    #[serde(default)]
    pub ignore_highlight_when_no_linked_or_active_objectives: bool,
    /// `priorityBoxoutTag` (Reference)
    #[serde(default)]
    pub priority_boxout_tag: Option<CigGuid>,
    /// `isObjectOfInterest` (Boolean)
    #[serde(default)]
    pub is_object_of_interest: bool,
}

impl Pooled for SSCSignatureSystemParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.sscsignature_system_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.sscsignature_system_params }
}

impl<'a> Extract<'a> for SSCSignatureSystemParams {
    const TYPE_NAME: &'static str = "SSCSignatureSystemParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            binding_urlprefix: inst.get_str("bindingURLPrefix").map(String::from).unwrap_or_default(),
            radar_properties: match inst.get("radarProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCRadarContactProperites>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCRadarContactProperites>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_params: match inst.get("audioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureSystemAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureSystemAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scan_custom_data: inst.get("scanCustomData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            embedded_scan_info: match inst.get("embeddedScanInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ScanCustomDataInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ScanCustomDataInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scan_display_layout_override: inst.get("scanDisplayLayoutOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            detection_tags: inst.get_array("detectionTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            is_overridden: inst.get_bool("isOverridden").unwrap_or_default(),
            overridden_size: match inst.get("overriddenSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable_detection_on_item_port: inst.get_bool("enableDetectionOnItemPort").unwrap_or_default(),
            ignore_highlight_when_detector_inside_bounds: inst.get_bool("ignoreHighlightWhenDetectorInsideBounds").unwrap_or_default(),
            linked_objectives: inst.get_array("linkedObjectives")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            ignore_highlight_when_no_linked_or_active_objectives: inst.get_bool("ignoreHighlightWhenNoLinkedOrActiveObjectives").unwrap_or_default(),
            priority_boxout_tag: inst.get("priorityBoxoutTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            is_object_of_interest: inst.get_bool("isObjectOfInterest").unwrap_or_default(),
        }
    }
}

/// DCB type: `TintPaletteSwizzle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintPaletteSwizzle {
    /// `primary` (Int32)
    #[serde(default)]
    pub primary: i32,
    /// `secondary` (Int32)
    #[serde(default)]
    pub secondary: i32,
    /// `tertiary` (Int32)
    #[serde(default)]
    pub tertiary: i32,
}

impl Pooled for TintPaletteSwizzle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.tint_palette_swizzle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.tint_palette_swizzle }
}

impl<'a> Extract<'a> for TintPaletteSwizzle {
    const TYPE_NAME: &'static str = "TintPaletteSwizzle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            primary: inst.get_i32("primary").unwrap_or_default(),
            secondary: inst.get_i32("secondary").unwrap_or_default(),
            tertiary: inst.get_i32("tertiary").unwrap_or_default(),
        }
    }
}

/// DCB type: `TintPaletteRef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TintPaletteRef {
    /// `RootRecord` (Reference)
    #[serde(default)]
    pub root_record: Option<CigGuid>,
    /// `ChildPath` (String)
    #[serde(default)]
    pub child_path: String,
    /// `SwizzleOverride` (StrongPointer)
    #[serde(default)]
    pub swizzle_override: Option<Handle<TintPaletteSwizzle>>,
}

impl Pooled for TintPaletteRef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ammoparams.tint_palette_ref }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ammoparams.tint_palette_ref }
}

impl<'a> Extract<'a> for TintPaletteRef {
    const TYPE_NAME: &'static str = "TintPaletteRef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            root_record: inst.get("RootRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            child_path: inst.get_str("ChildPath").map(String::from).unwrap_or_default(),
            swizzle_override: match inst.get("SwizzleOverride") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TintPaletteSwizzle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TintPaletteSwizzle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

