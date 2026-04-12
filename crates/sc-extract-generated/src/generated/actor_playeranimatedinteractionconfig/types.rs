// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-playeranimatedinteractionconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `WalkToAlignParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkToAlignParams {
    /// `reachDistance` (Single)
    #[serde(default)]
    pub reach_distance: f32,
    /// `minDistance` (Single)
    #[serde(default)]
    pub min_distance: f32,
    /// `maxWalkToTakeTime` (Single)
    #[serde(default)]
    pub max_walk_to_take_time: f32,
    /// `maxStuckTakeTime` (Single)
    #[serde(default)]
    pub max_stuck_take_time: f32,
    /// `minimumLookAtTargetDistance` (Single)
    #[serde(default)]
    pub minimum_look_at_target_distance: f32,
}

impl Pooled for WalkToAlignParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playeranimatedinteractionconfig.walk_to_align_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playeranimatedinteractionconfig.walk_to_align_params }
}

impl<'a> Extract<'a> for WalkToAlignParams {
    const TYPE_NAME: &'static str = "WalkToAlignParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reach_distance: inst.get_f32("reachDistance").unwrap_or_default(),
            min_distance: inst.get_f32("minDistance").unwrap_or_default(),
            max_walk_to_take_time: inst.get_f32("maxWalkToTakeTime").unwrap_or_default(),
            max_stuck_take_time: inst.get_f32("maxStuckTakeTime").unwrap_or_default(),
            minimum_look_at_target_distance: inst.get_f32("minimumLookAtTargetDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimatedAction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedAction {
    /// `name` (Class)
    #[serde(default)]
    pub name: Option<Handle<InputAction>>,
    /// `playerActionAnimType` (EnumChoice)
    #[serde(default)]
    pub player_action_anim_type: String,
}

impl Pooled for AnimatedAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playeranimatedinteractionconfig.animated_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playeranimatedinteractionconfig.animated_action }
}

impl<'a> Extract<'a> for AnimatedAction {
    const TYPE_NAME: &'static str = "AnimatedAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: match inst.get("name") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_action_anim_type: inst.get_str("playerActionAnimType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionWalkingRequestParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionWalkingRequestParams {
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// `speed` (Single)
    #[serde(default)]
    pub speed: f32,
}

impl Pooled for PlayerAnimatedInteractionWalkingRequestParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playeranimatedinteractionconfig.player_animated_interaction_walking_request_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playeranimatedinteractionconfig.player_animated_interaction_walking_request_params }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionWalkingRequestParams {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionWalkingRequestParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            speed: inst.get_f32("speed").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionFiltered`
/// Inherits from: `ActorMotionStateFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionFiltered {
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
    /// `filterByLocomotionSet` (EnumChoice)
    #[serde(default)]
    pub filter_by_locomotion_set: String,
    /// `params` (Class)
    #[serde(default)]
    pub params: Option<Handle<PlayerAnimatedInteractionWalkingRequestParams>>,
}

impl Pooled for PlayerAnimatedInteractionFiltered {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playeranimatedinteractionconfig.player_animated_interaction_filtered }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playeranimatedinteractionconfig.player_animated_interaction_filtered }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionFiltered {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionFiltered";
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
            filter_by_locomotion_set: inst.get_str("filterByLocomotionSet").map(String::from).unwrap_or_default(),
            params: match inst.get("params") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerAnimatedInteractionWalkingRequestParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerAnimatedInteractionWalkingRequestParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerAnimatedInteractionConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimatedInteractionConfig {
    /// `walkToAlignParams` (Class)
    #[serde(default)]
    pub walk_to_align_params: Option<Handle<WalkToAlignParams>>,
    /// `PlayerAnimatedInteractionStanceConfigs` (Class (array))
    #[serde(default)]
    pub player_animated_interaction_stance_configs: Vec<Handle<PlayerAnimatedInteractionFiltered>>,
    /// `AnimActionList` (Class (array))
    #[serde(default)]
    pub anim_action_list: Vec<Handle<AnimatedAction>>,
}

impl Pooled for PlayerAnimatedInteractionConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playeranimatedinteractionconfig.player_animated_interaction_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playeranimatedinteractionconfig.player_animated_interaction_config }
}

impl<'a> Extract<'a> for PlayerAnimatedInteractionConfig {
    const TYPE_NAME: &'static str = "PlayerAnimatedInteractionConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            walk_to_align_params: match inst.get("walkToAlignParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WalkToAlignParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WalkToAlignParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_animated_interaction_stance_configs: inst.get_array("PlayerAnimatedInteractionStanceConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerAnimatedInteractionFiltered>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlayerAnimatedInteractionFiltered>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            anim_action_list: inst.get_array("AnimActionList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AnimatedAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AnimatedAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

