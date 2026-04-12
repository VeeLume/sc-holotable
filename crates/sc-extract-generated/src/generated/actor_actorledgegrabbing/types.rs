// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-actorledgegrabbing`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `LedgeNearbyParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgeNearbyParams {
    /// `maxDistanceIdle` (Single)
    #[serde(default)]
    pub max_distance_idle: f32,
    /// `maxDistanceMovement` (Single)
    #[serde(default)]
    pub max_distance_movement: f32,
    /// `maxDistanceInAirIdle` (Single)
    #[serde(default)]
    pub max_distance_in_air_idle: f32,
    /// `maxDistanceInAirMovement` (Single)
    #[serde(default)]
    pub max_distance_in_air_movement: f32,
    /// `minHeight` (Single)
    #[serde(default)]
    pub min_height: f32,
    /// `maxHeightFromGround` (Single)
    #[serde(default)]
    pub max_height_from_ground: f32,
    /// `maxHeightInAir` (Single)
    #[serde(default)]
    pub max_height_in_air: f32,
    /// `maxSearchAngleDeg` (Single)
    #[serde(default)]
    pub max_search_angle_deg: f32,
    /// `inAirInvalidUpperSearchEdgeHeight` (Single)
    #[serde(default)]
    pub in_air_invalid_upper_search_edge_height: f32,
    /// `inAirInvalidUpperSearchEdgeDepth` (Single)
    #[serde(default)]
    pub in_air_invalid_upper_search_edge_depth: f32,
    /// `searchDir` (Class)
    #[serde(default)]
    pub search_dir: Option<Handle<Vec3>>,
}

impl Pooled for LedgeNearbyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorledgegrabbing.ledge_nearby_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorledgegrabbing.ledge_nearby_params }
}

impl<'a> Extract<'a> for LedgeNearbyParams {
    const TYPE_NAME: &'static str = "LedgeNearbyParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_distance_idle: inst.get_f32("maxDistanceIdle").unwrap_or_default(),
            max_distance_movement: inst.get_f32("maxDistanceMovement").unwrap_or_default(),
            max_distance_in_air_idle: inst.get_f32("maxDistanceInAirIdle").unwrap_or_default(),
            max_distance_in_air_movement: inst.get_f32("maxDistanceInAirMovement").unwrap_or_default(),
            min_height: inst.get_f32("minHeight").unwrap_or_default(),
            max_height_from_ground: inst.get_f32("maxHeightFromGround").unwrap_or_default(),
            max_height_in_air: inst.get_f32("maxHeightInAir").unwrap_or_default(),
            max_search_angle_deg: inst.get_f32("maxSearchAngleDeg").unwrap_or_default(),
            in_air_invalid_upper_search_edge_height: inst.get_f32("inAirInvalidUpperSearchEdgeHeight").unwrap_or_default(),
            in_air_invalid_upper_search_edge_depth: inst.get_f32("inAirInvalidUpperSearchEdgeDepth").unwrap_or_default(),
            search_dir: match inst.get("searchDir") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LedgeTransitionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgeTransitionParams {
    /// `minHeight` (Single)
    #[serde(default)]
    pub min_height: f32,
    /// `lowMaxHeight` (Single)
    #[serde(default)]
    pub low_max_height: f32,
    /// `mediumMaxHeight` (Single)
    #[serde(default)]
    pub medium_max_height: f32,
    /// `highMaxHeight` (Single)
    #[serde(default)]
    pub high_max_height: f32,
    /// `ultraMaxHeight` (Single)
    #[serde(default)]
    pub ultra_max_height: f32,
    /// `lowMinWarpDepth` (Single)
    #[serde(default)]
    pub low_min_warp_depth: f32,
    /// `mediumMinWarpDepth` (Single)
    #[serde(default)]
    pub medium_min_warp_depth: f32,
    /// `highMinWarpDepth` (Single)
    #[serde(default)]
    pub high_min_warp_depth: f32,
    /// `ultraMinWarpDepth` (Single)
    #[serde(default)]
    pub ultra_min_warp_depth: f32,
    /// `walkSlowTimeScale` (Single)
    #[serde(default)]
    pub walk_slow_time_scale: f32,
    /// `walkFastTimeScale` (Single)
    #[serde(default)]
    pub walk_fast_time_scale: f32,
    /// `runSlowTimeScale` (Single)
    #[serde(default)]
    pub run_slow_time_scale: f32,
    /// `runFastTimeScale` (Single)
    #[serde(default)]
    pub run_fast_time_scale: f32,
    /// `sprintTimeScale` (Single)
    #[serde(default)]
    pub sprint_time_scale: f32,
}

impl Pooled for LedgeTransitionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorledgegrabbing.ledge_transition_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorledgegrabbing.ledge_transition_params }
}

impl<'a> Extract<'a> for LedgeTransitionParams {
    const TYPE_NAME: &'static str = "LedgeTransitionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_height: inst.get_f32("minHeight").unwrap_or_default(),
            low_max_height: inst.get_f32("lowMaxHeight").unwrap_or_default(),
            medium_max_height: inst.get_f32("mediumMaxHeight").unwrap_or_default(),
            high_max_height: inst.get_f32("highMaxHeight").unwrap_or_default(),
            ultra_max_height: inst.get_f32("ultraMaxHeight").unwrap_or_default(),
            low_min_warp_depth: inst.get_f32("lowMinWarpDepth").unwrap_or_default(),
            medium_min_warp_depth: inst.get_f32("mediumMinWarpDepth").unwrap_or_default(),
            high_min_warp_depth: inst.get_f32("highMinWarpDepth").unwrap_or_default(),
            ultra_min_warp_depth: inst.get_f32("ultraMinWarpDepth").unwrap_or_default(),
            walk_slow_time_scale: inst.get_f32("walkSlowTimeScale").unwrap_or_default(),
            walk_fast_time_scale: inst.get_f32("walkFastTimeScale").unwrap_or_default(),
            run_slow_time_scale: inst.get_f32("runSlowTimeScale").unwrap_or_default(),
            run_fast_time_scale: inst.get_f32("runFastTimeScale").unwrap_or_default(),
            sprint_time_scale: inst.get_f32("sprintTimeScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `LedgeGrabbingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgeGrabbingParams {
    /// `ledgeNearbyParams` (Class)
    #[serde(default)]
    pub ledge_nearby_params: Option<Handle<LedgeNearbyParams>>,
    /// `vaultTransitionParams` (Class)
    #[serde(default)]
    pub vault_transition_params: Option<Handle<LedgeTransitionParams>>,
    /// `mantleTransitionParams` (Class)
    #[serde(default)]
    pub mantle_transition_params: Option<Handle<LedgeTransitionParams>>,
}

impl Pooled for LedgeGrabbingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorledgegrabbing.ledge_grabbing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorledgegrabbing.ledge_grabbing_params }
}

impl<'a> Extract<'a> for LedgeGrabbingParams {
    const TYPE_NAME: &'static str = "LedgeGrabbingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ledge_nearby_params: match inst.get("ledgeNearbyParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LedgeNearbyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LedgeNearbyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vault_transition_params: match inst.get("vaultTransitionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LedgeTransitionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LedgeTransitionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mantle_transition_params: match inst.get("mantleTransitionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LedgeTransitionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LedgeTransitionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

