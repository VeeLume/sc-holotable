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

/// DCB type: `ProceduralIdleToMoveParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralIdleToMoveParams {
    /// DCB field: `maxMovementSpeed` (Single)
    #[serde(default)]
    pub max_movement_speed: f32,
    /// DCB field: `maxHipTilt` (Single)
    #[serde(default)]
    pub max_hip_tilt: f32,
    /// DCB field: `maxHipVerticalOffset` (Single)
    #[serde(default)]
    pub max_hip_vertical_offset: f32,
    /// DCB field: `maxHipHorizontalOffset` (Single)
    #[serde(default)]
    pub max_hip_horizontal_offset: f32,
    /// DCB field: `maxSpineBend` (Single)
    #[serde(default)]
    pub max_spine_bend: f32,
    /// DCB field: `tiltDuration` (Single)
    #[serde(default)]
    pub tilt_duration: f32,
    /// DCB field: `tiltRestorationDuration` (Single)
    #[serde(default)]
    pub tilt_restoration_duration: f32,
}

impl Pooled for ProceduralIdleToMoveParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_idle_to_move_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_idle_to_move_params }
}

impl<'a> Extract<'a> for ProceduralIdleToMoveParams {
    const TYPE_NAME: &'static str = "ProceduralIdleToMoveParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_movement_speed: inst.get_f32("maxMovementSpeed").unwrap_or_default(),
            max_hip_tilt: inst.get_f32("maxHipTilt").unwrap_or_default(),
            max_hip_vertical_offset: inst.get_f32("maxHipVerticalOffset").unwrap_or_default(),
            max_hip_horizontal_offset: inst.get_f32("maxHipHorizontalOffset").unwrap_or_default(),
            max_spine_bend: inst.get_f32("maxSpineBend").unwrap_or_default(),
            tilt_duration: inst.get_f32("tiltDuration").unwrap_or_default(),
            tilt_restoration_duration: inst.get_f32("tiltRestorationDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProjectileParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectileParams {
    /// DCB field: `detonationParams` (StrongPointer)
    #[serde(default)]
    pub detonation_params: Option<Handle<ProjectileDetonationParams>>,
    /// DCB field: `proximityTriggerParams` (StrongPointer)
    #[serde(default)]
    pub proximity_trigger_params: Option<Handle<ProjectileProximityTriggerParams>>,
}

impl Pooled for ProjectileParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.projectile_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.projectile_params }
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
    /// DCB field: `entityToSpawnOnCollision` (Reference)
    #[serde(default)]
    pub entity_to_spawn_on_collision: Option<CigGuid>,
    /// DCB field: `oneShot` (Boolean)
    #[serde(default)]
    pub one_shot: bool,
}

impl Pooled for ProjectileSpawnedEntityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.projectile_spawned_entity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.projectile_spawned_entity_params }
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
    /// DCB field: `armTime` (Single)
    #[serde(default)]
    pub arm_time: f32,
    /// DCB field: `safeDistance` (Single)
    #[serde(default)]
    pub safe_distance: f32,
    /// DCB field: `destructDelay` (Single)
    #[serde(default)]
    pub destruct_delay: f32,
    /// DCB field: `explodeOnImpact` (Boolean)
    #[serde(default)]
    pub explode_on_impact: bool,
    /// DCB field: `explodeOnFinalImpact` (Boolean)
    #[serde(default)]
    pub explode_on_final_impact: bool,
    /// DCB field: `explodeOnExpire` (Boolean)
    #[serde(default)]
    pub explode_on_expire: bool,
    /// DCB field: `explodeOnTargetRange` (Boolean)
    #[serde(default)]
    pub explode_on_target_range: bool,
    /// DCB field: `explosionParams` (Class)
    #[serde(default)]
    pub explosion_params: Option<Handle<ExplosionParams>>,
    /// DCB field: `spawnedEntityParams` (Class)
    #[serde(default)]
    pub spawned_entity_params: Option<Handle<ProjectileSpawnedEntityParams>>,
}

impl Pooled for ProjectileDetonationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.projectile_detonation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.projectile_detonation_params }
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
    /// DCB field: `armTime` (Single)
    #[serde(default)]
    pub arm_time: f32,
    /// DCB field: `safeDistance` (Single)
    #[serde(default)]
    pub safe_distance: f32,
    /// DCB field: `radius` (Single)
    #[serde(default)]
    pub radius: f32,
}

impl Pooled for ProjectileProximityTriggerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.projectile_proximity_trigger_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.projectile_proximity_trigger_params }
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

/// DCB type: `ProceduralPlanetAudioTagAndEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioTagAndEvent {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `movementRtpc` (Class)
    #[serde(default)]
    pub movement_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `loopStart` (Class)
    #[serde(default)]
    pub loop_start: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `loopStop` (Class)
    #[serde(default)]
    pub loop_stop: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for ProceduralPlanetAudioTagAndEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_planet_audio_tag_and_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_planet_audio_tag_and_event }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioTagAndEvent {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioTagAndEvent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            movement_rtpc: match inst.get("movementRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loop_start: match inst.get("loopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loop_stop: match inst.get("loopStop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProceduralPlanetAudioTagAndRtpc`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioTagAndRtpc {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `rtpc` (Class)
    #[serde(default)]
    pub rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for ProceduralPlanetAudioTagAndRtpc {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_planet_audio_tag_and_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_planet_audio_tag_and_rtpc }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioTagAndRtpc {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioTagAndRtpc";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rtpc: match inst.get("rtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProceduralPlanetAudioTagAndEventsDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioTagAndEventsDef {
    /// DCB field: `tagAndEvents` (Class (array))
    #[serde(default)]
    pub tag_and_events: Vec<Handle<ProceduralPlanetAudioTagAndEvent>>,
}

impl Pooled for ProceduralPlanetAudioTagAndEventsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_planet_audio_tag_and_events_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_planet_audio_tag_and_events_def }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioTagAndEventsDef {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioTagAndEventsDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag_and_events: inst.get_array("tagAndEvents")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralPlanetAudioTagAndEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralPlanetAudioTagAndEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralPlanetAudioAlgorithm`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioAlgorithm {
    /// DCB field: `countRtpcs` (Class (array))
    #[serde(default)]
    pub count_rtpcs: Vec<Handle<AudioRtpc>>,
}

impl Pooled for ProceduralPlanetAudioAlgorithm {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_planet_audio_algorithm }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_planet_audio_algorithm }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioAlgorithm {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioAlgorithm";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            count_rtpcs: inst.get_array("countRtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralPlanetAudioDisturbanceDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioDisturbanceDef {
    /// DCB field: `byActor` (Boolean)
    #[serde(default)]
    pub by_actor: bool,
    /// DCB field: `byWheeledVehicle` (Boolean)
    #[serde(default)]
    pub by_wheeled_vehicle: bool,
    /// DCB field: `bySpaceship` (Boolean)
    #[serde(default)]
    pub by_spaceship: bool,
    /// DCB field: `byOther` (Boolean)
    #[serde(default)]
    pub by_other: bool,
    /// DCB field: `idleTimeThreshold` (Single)
    #[serde(default)]
    pub idle_time_threshold: f32,
    /// DCB field: `enterSound` (Class)
    #[serde(default)]
    pub enter_sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `disturbedLoopStart` (Class)
    #[serde(default)]
    pub disturbed_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `disturbedLoopStop` (Class)
    #[serde(default)]
    pub disturbed_loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `redisturbSound` (Class)
    #[serde(default)]
    pub redisturb_sound: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `exitSound` (Class)
    #[serde(default)]
    pub exit_sound: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for ProceduralPlanetAudioDisturbanceDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_planet_audio_disturbance_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_planet_audio_disturbance_def }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioDisturbanceDef {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioDisturbanceDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            by_actor: inst.get_bool("byActor").unwrap_or_default(),
            by_wheeled_vehicle: inst.get_bool("byWheeledVehicle").unwrap_or_default(),
            by_spaceship: inst.get_bool("bySpaceship").unwrap_or_default(),
            by_other: inst.get_bool("byOther").unwrap_or_default(),
            idle_time_threshold: inst.get_f32("idleTimeThreshold").unwrap_or_default(),
            enter_sound: match inst.get("enterSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            disturbed_loop_start: match inst.get("disturbedLoopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            disturbed_loop_stop: match inst.get("disturbedLoopStop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            redisturb_sound: match inst.get("redisturbSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exit_sound: match inst.get("exitSound") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProceduralPlanetAudioDisturbanceList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioDisturbanceList {
    /// DCB field: `disturbances` (Class (array))
    #[serde(default)]
    pub disturbances: Vec<Handle<ProceduralPlanetAudioDisturbanceDef>>,
}

impl Pooled for ProceduralPlanetAudioDisturbanceList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_planet_audio_disturbance_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_planet_audio_disturbance_list }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioDisturbanceList {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioDisturbanceList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            disturbances: inst.get_array("disturbances")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralPlanetAudioDisturbanceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralPlanetAudioDisturbanceDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralPlanetAudioEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioEntry {
    /// DCB field: `audioName` (String)
    #[serde(default)]
    pub audio_name: String,
    /// DCB field: `listenerMovementThreshold` (Single)
    #[serde(default)]
    pub listener_movement_threshold: f32,
    /// DCB field: `algorithm` (StrongPointer)
    #[serde(default)]
    pub algorithm: Option<Handle<ProceduralPlanetAudioAlgorithm>>,
    /// DCB field: `disturbances` (WeakPointer (array))
    #[serde(default)]
    pub disturbances: Vec<Handle<ProceduralPlanetAudioDisturbanceList>>,
}

impl Pooled for ProceduralPlanetAudioEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_planet_audio_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_planet_audio_entry }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioEntry {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_name: inst.get_str("audioName").map(String::from).unwrap_or_default(),
            listener_movement_threshold: inst.get_f32("listenerMovementThreshold").unwrap_or_default(),
            algorithm: match inst.get("algorithm") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralPlanetAudioAlgorithm>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProceduralPlanetAudioAlgorithm>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            disturbances: inst.get_array("disturbances")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralPlanetAudioDisturbanceList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralPlanetAudioDisturbanceList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralPlanetAudioData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioData {
    /// DCB field: `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<ProceduralPlanetAudioEntry>>,
    /// DCB field: `pressureRtpc` (Class)
    #[serde(default)]
    pub pressure_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `temperatureRtpc` (Class)
    #[serde(default)]
    pub temperature_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `humidityRtpc` (Class)
    #[serde(default)]
    pub humidity_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `breathabilityRtpc` (Class)
    #[serde(default)]
    pub breathability_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `firstRoomIsPlanetRoomRtpc` (Class)
    #[serde(default)]
    pub first_room_is_planet_room_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `effectTagMovementRtpcs` (Class (array))
    #[serde(default)]
    pub effect_tag_movement_rtpcs: Vec<Handle<ProceduralPlanetAudioTagAndRtpc>>,
    /// DCB field: `disturbanceLists` (Class (array))
    #[serde(default)]
    pub disturbance_lists: Vec<Handle<ProceduralPlanetAudioDisturbanceList>>,
    /// DCB field: `disturbanceVelocityRtpc` (Class)
    #[serde(default)]
    pub disturbance_velocity_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for ProceduralPlanetAudioData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_planet_audio_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_planet_audio_data }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioData {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: inst.get_array("entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralPlanetAudioEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralPlanetAudioEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            pressure_rtpc: match inst.get("pressureRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            temperature_rtpc: match inst.get("temperatureRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            humidity_rtpc: match inst.get("humidityRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            breathability_rtpc: match inst.get("breathabilityRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            first_room_is_planet_room_rtpc: match inst.get("firstRoomIsPlanetRoomRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            effect_tag_movement_rtpcs: inst.get_array("effectTagMovementRtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralPlanetAudioTagAndRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralPlanetAudioTagAndRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            disturbance_lists: inst.get_array("disturbanceLists")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralPlanetAudioDisturbanceList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralPlanetAudioDisturbanceList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            disturbance_velocity_rtpc: match inst.get("disturbanceVelocityRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProceduralPlanetAudioRiverData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioRiverData {
    /// DCB field: `riverLoopStart` (Class)
    #[serde(default)]
    pub river_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `riverFlowSpeedRtpc` (Class)
    #[serde(default)]
    pub river_flow_speed_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `riverWidthRtpc` (Class)
    #[serde(default)]
    pub river_width_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `distanceFromRiverEdgeRtpc` (Class)
    #[serde(default)]
    pub distance_from_river_edge_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for ProceduralPlanetAudioRiverData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_planet_audio_river_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_planet_audio_river_data }
}

impl<'a> Extract<'a> for ProceduralPlanetAudioRiverData {
    const TYPE_NAME: &'static str = "ProceduralPlanetAudioRiverData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            river_loop_start: match inst.get("riverLoopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            river_flow_speed_rtpc: match inst.get("riverFlowSpeedRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            river_width_rtpc: match inst.get("riverWidthRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance_from_river_edge_rtpc: match inst.get("distanceFromRiverEdgeRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProcBreathingCurve`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingCurve {
    /// DCB field: `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for ProcBreathingCurve {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proc_breathing_curve }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proc_breathing_curve }
}

impl<'a> Extract<'a> for ProcBreathingCurve {
    const TYPE_NAME: &'static str = "ProcBreathingCurve";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProcBreathingCurveDatabase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingCurveDatabase {
    /// DCB field: `breathAnimationCurves` (Reference (array))
    #[serde(default)]
    pub breath_animation_curves: Vec<CigGuid>,
}

impl Pooled for ProcBreathingCurveDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proc_breathing_curve_database }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proc_breathing_curve_database }
}

impl<'a> Extract<'a> for ProcBreathingCurveDatabase {
    const TYPE_NAME: &'static str = "ProcBreathingCurveDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            breath_animation_curves: inst.get_array("breathAnimationCurves")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProcBreathingGraph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingGraph {
    /// DCB field: `magnitude` (Single)
    #[serde(default)]
    pub magnitude: f32,
    /// DCB field: `magnitudeADS` (Single)
    #[serde(default)]
    pub magnitude_ads: f32,
    /// DCB field: `curveX` (Reference)
    #[serde(default)]
    pub curve_x: Option<CigGuid>,
    /// DCB field: `curveY` (Reference)
    #[serde(default)]
    pub curve_y: Option<CigGuid>,
    /// DCB field: `curveZ` (Reference)
    #[serde(default)]
    pub curve_z: Option<CigGuid>,
}

impl Pooled for ProcBreathingGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proc_breathing_graph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proc_breathing_graph }
}

impl<'a> Extract<'a> for ProcBreathingGraph {
    const TYPE_NAME: &'static str = "ProcBreathingGraph";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            magnitude: inst.get_f32("magnitude").unwrap_or_default(),
            magnitude_ads: inst.get_f32("magnitudeADS").unwrap_or_default(),
            curve_x: inst.get("curveX").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            curve_y: inst.get("curveY").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            curve_z: inst.get("curveZ").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ProcBreathingGraphEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingGraphEntry {
    /// DCB field: `joint` (String)
    #[serde(default)]
    pub joint: String,
    /// DCB field: `additionalScale` (Single)
    #[serde(default)]
    pub additional_scale: f32,
    /// DCB field: `relativeToShoulder` (Boolean)
    #[serde(default)]
    pub relative_to_shoulder: bool,
    /// DCB field: `firstPersonOnly` (Boolean)
    #[serde(default)]
    pub first_person_only: bool,
    /// DCB field: `translation` (Class)
    #[serde(default)]
    pub translation: Option<Handle<ProcBreathingGraph>>,
    /// DCB field: `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<ProcBreathingGraph>>,
}

impl Pooled for ProcBreathingGraphEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proc_breathing_graph_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proc_breathing_graph_entry }
}

impl<'a> Extract<'a> for ProcBreathingGraphEntry {
    const TYPE_NAME: &'static str = "ProcBreathingGraphEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            joint: inst.get_str("joint").map(String::from).unwrap_or_default(),
            additional_scale: inst.get_f32("additionalScale").unwrap_or_default(),
            relative_to_shoulder: inst.get_bool("relativeToShoulder").unwrap_or_default(),
            first_person_only: inst.get_bool("firstPersonOnly").unwrap_or_default(),
            translation: match inst.get("translation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProcBreathingGraph>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProcBreathingGraph>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProcBreathingGraph>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProcBreathingGraph>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProcBreathingExertion`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingExertion {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `maxStamina` (Single)
    #[serde(default)]
    pub max_stamina: f32,
    /// DCB field: `maxFullness` (Single)
    #[serde(default)]
    pub max_fullness: f32,
    /// DCB field: `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<ProcBreathingGraphEntry>>,
}

impl Pooled for ProcBreathingExertion {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proc_breathing_exertion }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proc_breathing_exertion }
}

impl<'a> Extract<'a> for ProcBreathingExertion {
    const TYPE_NAME: &'static str = "ProcBreathingExertion";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            max_stamina: inst.get_f32("maxStamina").unwrap_or_default(),
            max_fullness: inst.get_f32("maxFullness").unwrap_or_default(),
            entries: inst.get_array("entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProcBreathingGraphEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProcBreathingGraphEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProcBreathingHoldBreathNoise`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingHoldBreathNoise {
    /// DCB field: `noiseAng` (Single)
    #[serde(default)]
    pub noise_ang: f32,
    /// DCB field: `noiseSpeed` (Single)
    #[serde(default)]
    pub noise_speed: f32,
}

impl Pooled for ProcBreathingHoldBreathNoise {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proc_breathing_hold_breath_noise }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proc_breathing_hold_breath_noise }
}

impl<'a> Extract<'a> for ProcBreathingHoldBreathNoise {
    const TYPE_NAME: &'static str = "ProcBreathingHoldBreathNoise";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            noise_ang: inst.get_f32("noiseAng").unwrap_or_default(),
            noise_speed: inst.get_f32("noiseSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProcBreathingSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcBreathingSetup {
    /// DCB field: `exertionGraphs` (Class (array))
    #[serde(default)]
    pub exertion_graphs: Vec<Handle<ProcBreathingExertion>>,
    /// DCB field: `holdBreathNoise` (Class)
    #[serde(default)]
    pub hold_breath_noise: Option<Handle<ProcBreathingHoldBreathNoise>>,
}

impl Pooled for ProcBreathingSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proc_breathing_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proc_breathing_setup }
}

impl<'a> Extract<'a> for ProcBreathingSetup {
    const TYPE_NAME: &'static str = "ProcBreathingSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            exertion_graphs: inst.get_array("exertionGraphs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProcBreathingExertion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProcBreathingExertion>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            hold_breath_noise: match inst.get("holdBreathNoise") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProcBreathingHoldBreathNoise>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProcBreathingHoldBreathNoise>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ProcAimBaseJointTypeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcAimBaseJointTypeConfig {
    /// DCB field: `jointName` (String)
    #[serde(default)]
    pub joint_name: String,
}

impl Pooled for ProcAimBaseJointTypeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proc_aim_base_joint_type_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proc_aim_base_joint_type_config }
}

impl<'a> Extract<'a> for ProcAimBaseJointTypeConfig {
    const TYPE_NAME: &'static str = "ProcAimBaseJointTypeConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            joint_name: inst.get_str("jointName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ProcAimRigConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcAimRigConfig {
    /// DCB field: `rigName` (String)
    #[serde(default)]
    pub rig_name: String,
    /// DCB field: `aimJointName` (String)
    #[serde(default)]
    pub aim_joint_name: String,
    /// DCB field: `aimJointDirection` (Class)
    #[serde(default)]
    pub aim_joint_direction: Option<Handle<Vec3>>,
    /// DCB field: `aimTargetSmoothDuration` (Single)
    #[serde(default)]
    pub aim_target_smooth_duration: f32,
    /// DCB field: `aimTargetClampAngleDeg` (Single)
    #[serde(default)]
    pub aim_target_clamp_angle_deg: f32,
    /// DCB field: `aimAngleBlendRange` (Class)
    #[serde(default)]
    pub aim_angle_blend_range: Option<Handle<Range>>,
    /// DCB field: `rotationJoints` (StrongPointer (array))
    #[serde(default)]
    pub rotation_joints: Vec<Handle<ProcAimBaseJointTypeConfig>>,
}

impl Pooled for ProcAimRigConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proc_aim_rig_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proc_aim_rig_config }
}

impl<'a> Extract<'a> for ProcAimRigConfig {
    const TYPE_NAME: &'static str = "ProcAimRigConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rig_name: inst.get_str("rigName").map(String::from).unwrap_or_default(),
            aim_joint_name: inst.get_str("aimJointName").map(String::from).unwrap_or_default(),
            aim_joint_direction: match inst.get("aimJointDirection") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            aim_target_smooth_duration: inst.get_f32("aimTargetSmoothDuration").unwrap_or_default(),
            aim_target_clamp_angle_deg: inst.get_f32("aimTargetClampAngleDeg").unwrap_or_default(),
            aim_angle_blend_range: match inst.get("aimAngleBlendRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_joints: inst.get_array("rotationJoints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProcAimBaseJointTypeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProcAimBaseJointTypeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralAimRigRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralAimRigRecord {
    /// DCB field: `aimLayer` (Int32)
    #[serde(default)]
    pub aim_layer: i32,
    /// DCB field: `aimRigs` (Class (array))
    #[serde(default)]
    pub aim_rigs: Vec<Handle<ProcAimRigConfig>>,
}

impl Pooled for ProceduralAimRigRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_aim_rig_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_aim_rig_record }
}

impl<'a> Extract<'a> for ProceduralAimRigRecord {
    const TYPE_NAME: &'static str = "ProceduralAimRigRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aim_layer: inst.get_i32("aimLayer").unwrap_or_default(),
            aim_rigs: inst.get_array("aimRigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProcAimRigConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProcAimRigConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralAnimationBone`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralAnimationBone {
    /// DCB field: `bone` (EnumChoice)
    #[serde(default)]
    pub bone: String,
    /// DCB field: `chainLength` (Int32)
    #[serde(default)]
    pub chain_length: i32,
    /// DCB field: `layer` (Int32)
    #[serde(default)]
    pub layer: i32,
    /// DCB field: `operation` (EnumChoice)
    #[serde(default)]
    pub operation: String,
    /// DCB field: `relativeTo` (EnumChoice)
    #[serde(default)]
    pub relative_to: String,
    /// DCB field: `values` (Class)
    #[serde(default)]
    pub values: Option<Handle<Vec3>>,
    /// DCB field: `delay` (Single)
    #[serde(default)]
    pub delay: f32,
}

impl Pooled for ProceduralAnimationBone {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_animation_bone }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_animation_bone }
}

impl<'a> Extract<'a> for ProceduralAnimationBone {
    const TYPE_NAME: &'static str = "ProceduralAnimationBone";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bone: inst.get_str("bone").map(String::from).unwrap_or_default(),
            chain_length: inst.get_i32("chainLength").unwrap_or_default(),
            layer: inst.get_i32("layer").unwrap_or_default(),
            operation: inst.get_str("operation").map(String::from).unwrap_or_default(),
            relative_to: inst.get_str("relativeTo").map(String::from).unwrap_or_default(),
            values: match inst.get("values") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delay: inst.get_f32("delay").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralAnimationSequence`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralAnimationSequence {
    /// DCB field: `duration` (Single)
    #[serde(default)]
    pub duration: f32,
    /// DCB field: `animationCurve` (Class)
    #[serde(default)]
    pub animation_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `boneAnimations` (Class (array))
    #[serde(default)]
    pub bone_animations: Vec<Handle<ProceduralAnimationBone>>,
}

impl Pooled for ProceduralAnimationSequence {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_animation_sequence }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_animation_sequence }
}

impl<'a> Extract<'a> for ProceduralAnimationSequence {
    const TYPE_NAME: &'static str = "ProceduralAnimationSequence";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            duration: inst.get_f32("duration").unwrap_or_default(),
            animation_curve: match inst.get("animationCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bone_animations: inst.get_array("boneAnimations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralAnimationBone>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralAnimationBone>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralAnimation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralAnimation {
    /// DCB field: `sequences` (Class (array))
    #[serde(default)]
    pub sequences: Vec<Handle<ProceduralAnimationSequence>>,
}

impl Pooled for ProceduralAnimation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_animation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_animation }
}

impl<'a> Extract<'a> for ProceduralAnimation {
    const TYPE_NAME: &'static str = "ProceduralAnimation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sequences: inst.get_array("sequences")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralAnimationSequence>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralAnimationSequence>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralLandingFilter`
///
/// Inherits from: `ActorStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLandingFilter {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// DCB field: `filterByLandingStrength` (EnumChoice)
    #[serde(default)]
    pub filter_by_landing_strength: String,
    /// DCB field: `animationRecord` (Reference)
    #[serde(default)]
    pub animation_record: Option<CigGuid>,
}

impl Pooled for ProceduralLandingFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_landing_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_landing_filter }
}

impl<'a> Extract<'a> for ProceduralLandingFilter {
    const TYPE_NAME: &'static str = "ProceduralLandingFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filter_name: inst.get_str("filterName").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            filter_by_motion_speed: inst.get_str("filterByMotionSpeed").map(String::from).unwrap_or_default(),
            filter_by_pose_state: inst.get_str("filterByPoseState").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_lean_state: inst.get_str("filterByLeanState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
            filter_by_character_type: inst.get_str("filterByCharacterType").map(String::from).unwrap_or_default(),
            filter_by_restrained_state: inst.get_str("filterByRestrainedState").map(String::from).unwrap_or_default(),
            filter_by_player_camera: inst.get_str("filterByPlayerCamera").map(String::from).unwrap_or_default(),
            filter_by_aiming_restriction: inst.get_str("filterByAimingRestriction").map(String::from).unwrap_or_default(),
            filter_by_landing_strength: inst.get_str("filterByLandingStrength").map(String::from).unwrap_or_default(),
            animation_record: inst.get("animationRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ProceduralLandingSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLandingSetup {
    /// DCB field: `landingAnimations` (Class (array))
    #[serde(default)]
    pub landing_animations: Vec<Handle<ProceduralLandingFilter>>,
    /// DCB field: `firstPersonLandingAnimations` (Class (array))
    #[serde(default)]
    pub first_person_landing_animations: Vec<Handle<ProceduralLandingFilter>>,
}

impl Pooled for ProceduralLandingSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_landing_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_landing_setup }
}

impl<'a> Extract<'a> for ProceduralLandingSetup {
    const TYPE_NAME: &'static str = "ProceduralLandingSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            landing_animations: inst.get_array("landingAnimations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralLandingFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralLandingFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            first_person_landing_animations: inst.get_array("firstPersonLandingAnimations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralLandingFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralLandingFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralLayoutNode_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayoutNode_Base {
}

impl Pooled for ProceduralLayoutNode_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_layout_node_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_layout_node_base }
}

impl<'a> Extract<'a> for ProceduralLayoutNode_Base {
    const TYPE_NAME: &'static str = "ProceduralLayoutNode_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ProceduralLayout_SupplementaryElementTagsOptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayout_SupplementaryElementTagsOptions {
    /// DCB field: `SupplementaryElementTags` (Reference (array))
    #[serde(default)]
    pub supplementary_element_tags: Vec<CigGuid>,
    /// DCB field: `MaxElementsToGenerate` (Int32)
    #[serde(default)]
    pub max_elements_to_generate: i32,
}

impl Pooled for ProceduralLayout_SupplementaryElementTagsOptions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_layout_supplementary_element_tags_options }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_layout_supplementary_element_tags_options }
}

impl<'a> Extract<'a> for ProceduralLayout_SupplementaryElementTagsOptions {
    const TYPE_NAME: &'static str = "ProceduralLayout_SupplementaryElementTagsOptions";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            supplementary_element_tags: inst.get_array("SupplementaryElementTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            max_elements_to_generate: inst.get_i32("MaxElementsToGenerate").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralLayout_TagFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayout_TagFilter {
    /// DCB field: `GraphNodeTagsToFilter` (Class (array))
    #[serde(default)]
    pub graph_node_tags_to_filter: Vec<Handle<TagList>>,
    /// DCB field: `TagFilteringMode` (EnumChoice)
    #[serde(default)]
    pub tag_filtering_mode: String,
    /// DCB field: `SupplementaryElementTagsList` (Class (array))
    #[serde(default)]
    pub supplementary_element_tags_list: Vec<Handle<ProceduralLayout_SupplementaryElementTagsOptions>>,
}

impl Pooled for ProceduralLayout_TagFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_layout_tag_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_layout_tag_filter }
}

impl<'a> Extract<'a> for ProceduralLayout_TagFilter {
    const TYPE_NAME: &'static str = "ProceduralLayout_TagFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            graph_node_tags_to_filter: inst.get_array("GraphNodeTagsToFilter")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            tag_filtering_mode: inst.get_str("TagFilteringMode").map(String::from).unwrap_or_default(),
            supplementary_element_tags_list: inst.get_array("SupplementaryElementTagsList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralLayout_SupplementaryElementTagsOptions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralLayout_SupplementaryElementTagsOptions>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralLayoutGraph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralLayoutGraph {
    /// DCB field: `ElementsLibraries` (String (array))
    #[serde(default)]
    pub elements_libraries: Vec<String>,
    /// DCB field: `RoutingElementsLibraries` (String (array))
    #[serde(default)]
    pub routing_elements_libraries: Vec<String>,
    /// DCB field: `ConnectorsLibraries` (String (array))
    #[serde(default)]
    pub connectors_libraries: Vec<String>,
    /// DCB field: `CapsLibraries` (String (array))
    #[serde(default)]
    pub caps_libraries: Vec<String>,
    /// DCB field: `SecondaryElementsLibraries` (String (array))
    #[serde(default)]
    pub secondary_elements_libraries: Vec<String>,
    /// DCB field: `DefaultRoutingElementsTags` (Class (array))
    #[serde(default)]
    pub default_routing_elements_tags: Vec<Handle<TagList>>,
    /// DCB field: `GlobalTagFiltering` (Class (array))
    #[serde(default)]
    pub global_tag_filtering: Vec<Handle<ProceduralLayout_TagFilter>>,
    /// DCB field: `GlobalAddOnElementsTags` (Class (array))
    #[serde(default)]
    pub global_add_on_elements_tags: Vec<Handle<TagList>>,
    /// DCB field: `GlobalAddOnElementsGenerationChance` (Single)
    #[serde(default)]
    pub global_add_on_elements_generation_chance: f32,
    /// DCB field: `TintPalettePath` (String)
    #[serde(default)]
    pub tint_palette_path: String,
    /// DCB field: `Nodes` (StrongPointer (array))
    #[serde(default)]
    pub nodes: Vec<Handle<ProceduralLayoutNode_Base>>,
}

impl Pooled for ProceduralLayoutGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.procedural_layout_graph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.procedural_layout_graph }
}

impl<'a> Extract<'a> for ProceduralLayoutGraph {
    const TYPE_NAME: &'static str = "ProceduralLayoutGraph";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            elements_libraries: inst.get_array("ElementsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            routing_elements_libraries: inst.get_array("RoutingElementsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            connectors_libraries: inst.get_array("ConnectorsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            caps_libraries: inst.get_array("CapsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            secondary_elements_libraries: inst.get_array("SecondaryElementsLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            default_routing_elements_tags: inst.get_array("DefaultRoutingElementsTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_tag_filtering: inst.get_array("GlobalTagFiltering")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralLayout_TagFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralLayout_TagFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_add_on_elements_tags: inst.get_array("GlobalAddOnElementsTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_add_on_elements_generation_chance: inst.get_f32("GlobalAddOnElementsGenerationChance").unwrap_or_default(),
            tint_palette_path: inst.get_str("TintPalettePath").map(String::from).unwrap_or_default(),
            nodes: inst.get_array("Nodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralLayoutNode_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralLayoutNode_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProximityInventoryDetectionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityInventoryDetectionParams {
    /// DCB field: `filterTags` (Class)
    #[serde(default)]
    pub filter_tags: Option<Handle<TagsDNF>>,
    /// DCB field: `zoneQueryRadius` (Single)
    #[serde(default)]
    pub zone_query_radius: f32,
    /// DCB field: `maxHeightStep` (Single)
    #[serde(default)]
    pub max_height_step: f32,
    /// DCB field: `losTargetGroundOffset` (Single)
    #[serde(default)]
    pub los_target_ground_offset: f32,
    /// DCB field: `losPlayerGroundOffsets` (Single (array))
    #[serde(default)]
    pub los_player_ground_offsets: Vec<f32>,
}

impl Pooled for ProximityInventoryDetectionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pr.proximity_inventory_detection_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pr.proximity_inventory_detection_params }
}

impl<'a> Extract<'a> for ProximityInventoryDetectionParams {
    const TYPE_NAME: &'static str = "ProximityInventoryDetectionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            filter_tags: match inst.get("filterTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            zone_query_radius: inst.get_f32("zoneQueryRadius").unwrap_or_default(),
            max_height_step: inst.get_f32("maxHeightStep").unwrap_or_default(),
            los_target_ground_offset: inst.get_f32("losTargetGroundOffset").unwrap_or_default(),
            los_player_ground_offsets: inst.get_array("losPlayerGroundOffsets")
                .map(|arr| arr.filter_map(|v| v.as_f32()).collect())
                .unwrap_or_default(),
        }
    }
}

