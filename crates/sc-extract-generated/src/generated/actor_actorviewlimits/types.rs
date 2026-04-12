// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-actorviewlimits`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ActorAimLimitsStateFilter`
/// Inherits from: `ActorStateFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorAimLimitsStateFilter {
    /// `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// `aimLimits` (Class)
    #[serde(default)]
    pub aim_limits: Option<Handle<ActorViewLimits>>,
}

impl Pooled for ActorAimLimitsStateFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorviewlimits.actor_aim_limits_state_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorviewlimits.actor_aim_limits_state_filter }
}

impl<'a> Extract<'a> for ActorAimLimitsStateFilter {
    const TYPE_NAME: &'static str = "ActorAimLimitsStateFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
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
            aim_limits: match inst.get("aimLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorViewLimits>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorViewLimits>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorLookLimitsStateFilter`
/// Inherits from: `ActorStateFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLookLimitsStateFilter {
    /// `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// `lookLimits` (Class)
    #[serde(default)]
    pub look_limits: Option<Handle<ActorViewLimits>>,
}

impl Pooled for ActorLookLimitsStateFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorviewlimits.actor_look_limits_state_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorviewlimits.actor_look_limits_state_filter }
}

impl<'a> Extract<'a> for ActorLookLimitsStateFilter {
    const TYPE_NAME: &'static str = "ActorLookLimitsStateFilter";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
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
            look_limits: match inst.get("lookLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorViewLimits>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorViewLimits>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorLookLimits`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLookLimits {
    /// `lookLimitsStateFilters` (Class (array))
    #[serde(default)]
    pub look_limits_state_filters: Vec<Handle<ActorLookLimitsStateFilter>>,
}

impl Pooled for ActorLookLimits {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorviewlimits.actor_look_limits }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorviewlimits.actor_look_limits }
}

impl<'a> Extract<'a> for ActorLookLimits {
    const TYPE_NAME: &'static str = "ActorLookLimits";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            look_limits_state_filters: inst.get_array("lookLimitsStateFilters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorLookLimitsStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorLookLimitsStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorAimLimits`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorAimLimits {
    /// `aimLimitsStateFilters` (Class (array))
    #[serde(default)]
    pub aim_limits_state_filters: Vec<Handle<ActorAimLimitsStateFilter>>,
}

impl Pooled for ActorAimLimits {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorviewlimits.actor_aim_limits }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorviewlimits.actor_aim_limits }
}

impl<'a> Extract<'a> for ActorAimLimits {
    const TYPE_NAME: &'static str = "ActorAimLimits";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aim_limits_state_filters: inst.get_array("aimLimitsStateFilters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorAimLimitsStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorAimLimitsStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

