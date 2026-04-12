// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-stanceinfo`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ActorStanceSpeeds`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStanceSpeeds {
    /// `defaultSpeed` (Single)
    #[serde(default)]
    pub default_speed: f32,
    /// `walkSlowSpeed` (Single)
    #[serde(default)]
    pub walk_slow_speed: f32,
    /// `walkMidSpeed` (Single)
    #[serde(default)]
    pub walk_mid_speed: f32,
    /// `walkFastSpeed` (Single)
    #[serde(default)]
    pub walk_fast_speed: f32,
    /// `runSlowSpeed` (Single)
    #[serde(default)]
    pub run_slow_speed: f32,
    /// `runFastSpeed` (Single)
    #[serde(default)]
    pub run_fast_speed: f32,
    /// `sprintSpeed` (Single)
    #[serde(default)]
    pub sprint_speed: f32,
    /// `greenZoneWalkSpeed` (Single)
    #[serde(default)]
    pub green_zone_walk_speed: f32,
    /// `greenZoneSprintSpeed` (Single)
    #[serde(default)]
    pub green_zone_sprint_speed: f32,
    /// `aimDownSightSpeed` (Single)
    #[serde(default)]
    pub aim_down_sight_speed: f32,
    /// `leanSpeed` (Single)
    #[serde(default)]
    pub lean_speed: f32,
    /// `conversationSpeed` (Single)
    #[serde(default)]
    pub conversation_speed: f32,
    /// `defaultLinearAcceleration` (Single)
    #[serde(default)]
    pub default_linear_acceleration: f32,
    /// `defaultRotationSpeed` (Single)
    #[serde(default)]
    pub default_rotation_speed: f32,
    /// `defaultRotationSmoothDuration` (Single)
    #[serde(default)]
    pub default_rotation_smooth_duration: f32,
}

impl Pooled for ActorStanceSpeeds {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_stanceinfo.actor_stance_speeds }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_stanceinfo.actor_stance_speeds }
}

impl<'a> Extract<'a> for ActorStanceSpeeds {
    const TYPE_NAME: &'static str = "ActorStanceSpeeds";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_speed: inst.get_f32("defaultSpeed").unwrap_or_default(),
            walk_slow_speed: inst.get_f32("walkSlowSpeed").unwrap_or_default(),
            walk_mid_speed: inst.get_f32("walkMidSpeed").unwrap_or_default(),
            walk_fast_speed: inst.get_f32("walkFastSpeed").unwrap_or_default(),
            run_slow_speed: inst.get_f32("runSlowSpeed").unwrap_or_default(),
            run_fast_speed: inst.get_f32("runFastSpeed").unwrap_or_default(),
            sprint_speed: inst.get_f32("sprintSpeed").unwrap_or_default(),
            green_zone_walk_speed: inst.get_f32("greenZoneWalkSpeed").unwrap_or_default(),
            green_zone_sprint_speed: inst.get_f32("greenZoneSprintSpeed").unwrap_or_default(),
            aim_down_sight_speed: inst.get_f32("aimDownSightSpeed").unwrap_or_default(),
            lean_speed: inst.get_f32("leanSpeed").unwrap_or_default(),
            conversation_speed: inst.get_f32("conversationSpeed").unwrap_or_default(),
            default_linear_acceleration: inst.get_f32("defaultLinearAcceleration").unwrap_or_default(),
            default_rotation_speed: inst.get_f32("defaultRotationSpeed").unwrap_or_default(),
            default_rotation_smooth_duration: inst.get_f32("defaultRotationSmoothDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStanceDimensions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStanceDimensions {
    /// `heightCollider` (Single)
    #[serde(default)]
    pub height_collider: f32,
    /// `groundContactEps` (Single)
    #[serde(default)]
    pub ground_contact_eps: f32,
    /// `groundTraceSpreadSizes` (Class)
    #[serde(default)]
    pub ground_trace_spread_sizes: Option<Handle<Vec2>>,
    /// `pivot` (Class)
    #[serde(default)]
    pub pivot: Option<Handle<Vec3>>,
    /// `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
    /// `capsuleAxis` (Class)
    #[serde(default)]
    pub capsule_axis: Option<Handle<Vec3>>,
    /// `viewOffset` (Class)
    #[serde(default)]
    pub view_offset: Option<Handle<Vec3>>,
    /// `weaponOffset` (Class)
    #[serde(default)]
    pub weapon_offset: Option<Handle<Vec3>>,
    /// `headStabilization` (Class)
    #[serde(default)]
    pub head_stabilization: Option<Handle<Vec3>>,
    /// `upAlignMode` (EnumChoice)
    #[serde(default)]
    pub up_align_mode: String,
    /// `canPerch` (Boolean)
    #[serde(default)]
    pub can_perch: bool,
    /// `extraDefs` (StrongPointer (array))
    #[serde(default)]
    pub extra_defs: Vec<Handle<SActorStanceDimensionsExtraDef>>,
}

impl Pooled for ActorStanceDimensions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_stanceinfo.actor_stance_dimensions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_stanceinfo.actor_stance_dimensions }
}

impl<'a> Extract<'a> for ActorStanceDimensions {
    const TYPE_NAME: &'static str = "ActorStanceDimensions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            height_collider: inst.get_f32("heightCollider").unwrap_or_default(),
            ground_contact_eps: inst.get_f32("groundContactEps").unwrap_or_default(),
            ground_trace_spread_sizes: match inst.get("groundTraceSpreadSizes") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            capsule_axis: match inst.get("capsuleAxis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            view_offset: match inst.get("viewOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_offset: match inst.get("weaponOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_stabilization: match inst.get("headStabilization") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            up_align_mode: inst.get_str("upAlignMode").map(String::from).unwrap_or_default(),
            can_perch: inst.get_bool("canPerch").unwrap_or_default(),
            extra_defs: inst.get_array("extraDefs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorStanceDimensionsExtraDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorStanceDimensionsExtraDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorStanceSpeedsInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStanceSpeedsInfo {
    /// `stateFilter` (Class)
    #[serde(default)]
    pub state_filter: Option<Handle<ActorMotionStateFilter>>,
    /// `speeds` (Class)
    #[serde(default)]
    pub speeds: Option<Handle<ActorStanceSpeeds>>,
}

impl Pooled for ActorStanceSpeedsInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_stanceinfo.actor_stance_speeds_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_stanceinfo.actor_stance_speeds_info }
}

impl<'a> Extract<'a> for ActorStanceSpeedsInfo {
    const TYPE_NAME: &'static str = "ActorStanceSpeedsInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_filter: match inst.get("stateFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorMotionStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorMotionStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            speeds: match inst.get("speeds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStanceSpeeds>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStanceSpeeds>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorStanceDimensionsInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStanceDimensionsInfo {
    /// `stateFilter` (Class)
    #[serde(default)]
    pub state_filter: Option<Handle<ActorMotionStateFilter>>,
    /// `dimensions` (Class)
    #[serde(default)]
    pub dimensions: Option<Handle<ActorStanceDimensions>>,
}

impl Pooled for ActorStanceDimensionsInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_stanceinfo.actor_stance_dimensions_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_stanceinfo.actor_stance_dimensions_info }
}

impl<'a> Extract<'a> for ActorStanceDimensionsInfo {
    const TYPE_NAME: &'static str = "ActorStanceDimensionsInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_filter: match inst.get("stateFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorMotionStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorMotionStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            dimensions: match inst.get("dimensions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStanceDimensions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStanceDimensions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SActorStanceDimensionsExtraDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorStanceDimensionsExtraDef {
}

impl Pooled for SActorStanceDimensionsExtraDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_stanceinfo.sactor_stance_dimensions_extra_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_stanceinfo.sactor_stance_dimensions_extra_def }
}

impl<'a> Extract<'a> for SActorStanceDimensionsExtraDef {
    const TYPE_NAME: &'static str = "SActorStanceDimensionsExtraDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

