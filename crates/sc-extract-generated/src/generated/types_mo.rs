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

/// DCB type: `MovementSystemAdditionalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSystemAdditionalParams {
}

impl Pooled for MovementSystemAdditionalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.movement_system_additional_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.movement_system_additional_params }
}

impl<'a> Extract<'a> for MovementSystemAdditionalParams {
    const TYPE_NAME: &'static str = "MovementSystemAdditionalParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MovementSystemAdditionalParamsRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSystemAdditionalParamsRecord {
    /// DCB field: `params` (StrongPointer)
    #[serde(default)]
    pub params: Option<Handle<MovementSystemAdditionalParams>>,
}

impl Pooled for MovementSystemAdditionalParamsRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.movement_system_additional_params_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.movement_system_additional_params_record }
}

impl<'a> Extract<'a> for MovementSystemAdditionalParamsRecord {
    const TYPE_NAME: &'static str = "MovementSystemAdditionalParamsRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params: match inst.get("params") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MovementSystemAdditionalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MovementSystemAdditionalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MotionConnection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionConnection {
    /// DCB field: `waitUntillFinished` (Boolean)
    #[serde(default)]
    pub wait_untill_finished: bool,
    /// DCB field: `delaySeconds` (Single)
    #[serde(default)]
    pub delay_seconds: f32,
    /// DCB field: `waitForEvent` (String)
    #[serde(default)]
    pub wait_for_event: String,
    /// DCB field: `nextState` (WeakPointer)
    #[serde(default)]
    pub next_state: Option<Handle<MotionState>>,
}

impl Pooled for MotionConnection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.motion_connection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.motion_connection }
}

impl<'a> Extract<'a> for MotionConnection {
    const TYPE_NAME: &'static str = "MotionConnection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wait_untill_finished: inst.get_bool("waitUntillFinished").unwrap_or_default(),
            delay_seconds: inst.get_f32("delaySeconds").unwrap_or_default(),
            wait_for_event: inst.get_str("waitForEvent").map(String::from).unwrap_or_default(),
            next_state: match inst.get("nextState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MotionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MotionState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionState {
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `mannequinTags` (String)
    #[serde(default)]
    pub mannequin_tags: String,
    /// DCB field: `mannequinFragment` (String)
    #[serde(default)]
    pub mannequin_fragment: String,
    /// DCB field: `motionTypeFP` (EnumChoice)
    #[serde(default)]
    pub motion_type_fp: String,
    /// DCB field: `motionTypeTP` (EnumChoice)
    #[serde(default)]
    pub motion_type_tp: String,
    /// DCB field: `motionTypeRemote` (EnumChoice)
    #[serde(default)]
    pub motion_type_remote: String,
    /// DCB field: `connections` (Class (array))
    #[serde(default)]
    pub connections: Vec<Handle<MotionConnection>>,
}

impl Pooled for MotionState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.motion_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.motion_state }
}

impl<'a> Extract<'a> for MotionState {
    const TYPE_NAME: &'static str = "MotionState";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            mannequin_tags: inst.get_str("mannequinTags").map(String::from).unwrap_or_default(),
            mannequin_fragment: inst.get_str("mannequinFragment").map(String::from).unwrap_or_default(),
            motion_type_fp: inst.get_str("motionTypeFP").map(String::from).unwrap_or_default(),
            motion_type_tp: inst.get_str("motionTypeTP").map(String::from).unwrap_or_default(),
            motion_type_remote: inst.get_str("motionTypeRemote").map(String::from).unwrap_or_default(),
            connections: inst.get_array("connections")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MotionConnection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MotionConnection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MotionSmoothingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionSmoothingParams {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `maxJitterRating` (Single)
    #[serde(default)]
    pub max_jitter_rating: f32,
    /// DCB field: `jitterSensitivity` (Single)
    #[serde(default)]
    pub jitter_sensitivity: f32,
    /// DCB field: `jitterDetectionThreshold` (Single)
    #[serde(default)]
    pub jitter_detection_threshold: f32,
    /// DCB field: `jitterDecayDuration` (Single)
    #[serde(default)]
    pub jitter_decay_duration: f32,
    /// DCB field: `speedSmoothingDuration` (Single)
    #[serde(default)]
    pub speed_smoothing_duration: f32,
}

impl Pooled for MotionSmoothingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.motion_smoothing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.motion_smoothing_params }
}

impl<'a> Extract<'a> for MotionSmoothingParams {
    const TYPE_NAME: &'static str = "MotionSmoothingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            max_jitter_rating: inst.get_f32("maxJitterRating").unwrap_or_default(),
            jitter_sensitivity: inst.get_f32("jitterSensitivity").unwrap_or_default(),
            jitter_detection_threshold: inst.get_f32("jitterDetectionThreshold").unwrap_or_default(),
            jitter_decay_duration: inst.get_f32("jitterDecayDuration").unwrap_or_default(),
            speed_smoothing_duration: inst.get_f32("speedSmoothingDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `MotionJukeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionJukeParams {
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// DCB field: `jukeTriggerAngle` (Single)
    #[serde(default)]
    pub juke_trigger_angle: f32,
    /// DCB field: `jukeDetectionDuration` (Single)
    #[serde(default)]
    pub juke_detection_duration: f32,
}

impl Pooled for MotionJukeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.motion_juke_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.motion_juke_params }
}

impl<'a> Extract<'a> for MotionJukeParams {
    const TYPE_NAME: &'static str = "MotionJukeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            juke_trigger_angle: inst.get_f32("jukeTriggerAngle").unwrap_or_default(),
            juke_detection_duration: inst.get_f32("jukeDetectionDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `MotionTurnParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionTurnParams {
    /// DCB field: `rightYawTrigger` (Single)
    #[serde(default)]
    pub right_yaw_trigger: f32,
    /// DCB field: `leftYawTrigger` (Single)
    #[serde(default)]
    pub left_yaw_trigger: f32,
    /// DCB field: `turnTriggerOffset` (Single)
    #[serde(default)]
    pub turn_trigger_offset: f32,
    /// DCB field: `targetOffset` (Single)
    #[serde(default)]
    pub target_offset: f32,
    /// DCB field: `snapOffset` (Single)
    #[serde(default)]
    pub snap_offset: f32,
    /// DCB field: `maxTurnSpeed` (Single)
    #[serde(default)]
    pub max_turn_speed: f32,
    /// DCB field: `delayIdleSeconds` (Single)
    #[serde(default)]
    pub delay_idle_seconds: f32,
}

impl Pooled for MotionTurnParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.motion_turn_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.motion_turn_params }
}

impl<'a> Extract<'a> for MotionTurnParams {
    const TYPE_NAME: &'static str = "MotionTurnParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            right_yaw_trigger: inst.get_f32("rightYawTrigger").unwrap_or_default(),
            left_yaw_trigger: inst.get_f32("leftYawTrigger").unwrap_or_default(),
            turn_trigger_offset: inst.get_f32("turnTriggerOffset").unwrap_or_default(),
            target_offset: inst.get_f32("targetOffset").unwrap_or_default(),
            snap_offset: inst.get_f32("snapOffset").unwrap_or_default(),
            max_turn_speed: inst.get_f32("maxTurnSpeed").unwrap_or_default(),
            delay_idle_seconds: inst.get_f32("delayIdleSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `MotionTurnSetupFiltered`
///
/// Inherits from: `ActorMotionStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionTurnSetupFiltered {
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
    /// DCB field: `filterByLocomotionSet` (EnumChoice)
    #[serde(default)]
    pub filter_by_locomotion_set: String,
    /// DCB field: `params` (Class)
    #[serde(default)]
    pub params: Option<Handle<MotionTurnParams>>,
}

impl Pooled for MotionTurnSetupFiltered {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.motion_turn_setup_filtered }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.motion_turn_setup_filtered }
}

impl<'a> Extract<'a> for MotionTurnSetupFiltered {
    const TYPE_NAME: &'static str = "MotionTurnSetupFiltered";
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
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionTurnParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MotionTurnParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MotionTurnSetupList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionTurnSetupList {
    /// DCB field: `setupList` (Class (array))
    #[serde(default)]
    pub setup_list: Vec<Handle<MotionTurnSetupFiltered>>,
}

impl Pooled for MotionTurnSetupList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.motion_turn_setup_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.motion_turn_setup_list }
}

impl<'a> Extract<'a> for MotionTurnSetupList {
    const TYPE_NAME: &'static str = "MotionTurnSetupList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            setup_list: inst.get_array("setupList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MotionTurnSetupFiltered>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MotionTurnSetupFiltered>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MotionFootPinningParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionFootPinningParams {
    /// DCB field: `footShiftDuration` (Single)
    #[serde(default)]
    pub foot_shift_duration: f32,
    /// DCB field: `autoRepositionDistance` (Single)
    #[serde(default)]
    pub auto_reposition_distance: f32,
    /// DCB field: `animationDuration` (Single)
    #[serde(default)]
    pub animation_duration: f32,
    /// DCB field: `animationWeight` (Single)
    #[serde(default)]
    pub animation_weight: f32,
    /// DCB field: `animationBlendInDuration` (Single)
    #[serde(default)]
    pub animation_blend_in_duration: f32,
    /// DCB field: `animationBlendOutDuration` (Single)
    #[serde(default)]
    pub animation_blend_out_duration: f32,
}

impl Pooled for MotionFootPinningParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.motion_foot_pinning_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.motion_foot_pinning_params }
}

impl<'a> Extract<'a> for MotionFootPinningParams {
    const TYPE_NAME: &'static str = "MotionFootPinningParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            foot_shift_duration: inst.get_f32("footShiftDuration").unwrap_or_default(),
            auto_reposition_distance: inst.get_f32("autoRepositionDistance").unwrap_or_default(),
            animation_duration: inst.get_f32("animationDuration").unwrap_or_default(),
            animation_weight: inst.get_f32("animationWeight").unwrap_or_default(),
            animation_blend_in_duration: inst.get_f32("animationBlendInDuration").unwrap_or_default(),
            animation_blend_out_duration: inst.get_f32("animationBlendOutDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `MotionGraph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionGraph {
    /// DCB field: `motionStates` (Class (array))
    #[serde(default)]
    pub motion_states: Vec<Handle<MotionState>>,
    /// DCB field: `motionSmoothingConfig` (Class)
    #[serde(default)]
    pub motion_smoothing_config: Option<Handle<MotionSmoothingParams>>,
    /// DCB field: `jukeConfig` (Class)
    #[serde(default)]
    pub juke_config: Option<Handle<MotionJukeParams>>,
    /// DCB field: `idleToMoveProcParamsForward` (Class)
    #[serde(default)]
    pub idle_to_move_proc_params_forward: Option<Handle<ProceduralIdleToMoveParams>>,
    /// DCB field: `idleToMoveProcParamsBack` (Class)
    #[serde(default)]
    pub idle_to_move_proc_params_back: Option<Handle<ProceduralIdleToMoveParams>>,
    /// DCB field: `idleToMoveProcParamsRight` (Class)
    #[serde(default)]
    pub idle_to_move_proc_params_right: Option<Handle<ProceduralIdleToMoveParams>>,
    /// DCB field: `idleToMoveProcParamsLeft` (Class)
    #[serde(default)]
    pub idle_to_move_proc_params_left: Option<Handle<ProceduralIdleToMoveParams>>,
    /// DCB field: `turnConfig` (Class)
    #[serde(default)]
    pub turn_config: Option<Handle<MotionTurnSetupList>>,
    /// DCB field: `footPinningParams` (Class)
    #[serde(default)]
    pub foot_pinning_params: Option<Handle<MotionFootPinningParams>>,
}

impl Pooled for MotionGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.motion_graph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.motion_graph }
}

impl<'a> Extract<'a> for MotionGraph {
    const TYPE_NAME: &'static str = "MotionGraph";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            motion_states: inst.get_array("motionStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MotionState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MotionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            motion_smoothing_config: match inst.get("motionSmoothingConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionSmoothingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MotionSmoothingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            juke_config: match inst.get("jukeConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionJukeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MotionJukeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            idle_to_move_proc_params_forward: match inst.get("idleToMoveProcParamsForward") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            idle_to_move_proc_params_back: match inst.get("idleToMoveProcParamsBack") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            idle_to_move_proc_params_right: match inst.get("idleToMoveProcParamsRight") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            idle_to_move_proc_params_left: match inst.get("idleToMoveProcParamsLeft") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turn_config: match inst.get("turnConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionTurnSetupList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MotionTurnSetupList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            foot_pinning_params: match inst.get("footPinningParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionFootPinningParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MotionFootPinningParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MovementSpeedOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSpeedOverride {
    /// DCB field: `speedCategory` (EnumChoice)
    #[serde(default)]
    pub speed_category: String,
    /// DCB field: `speedOverride` (Single)
    #[serde(default)]
    pub speed_override: f32,
}

impl Pooled for MovementSpeedOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.movement_speed_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.movement_speed_override }
}

impl<'a> Extract<'a> for MovementSpeedOverride {
    const TYPE_NAME: &'static str = "MovementSpeedOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            speed_category: inst.get_str("speedCategory").map(String::from).unwrap_or_default(),
            speed_override: inst.get_f32("speedOverride").unwrap_or_default(),
        }
    }
}

/// DCB type: `ModuleDeclarationType_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclarationType_Base {
}

impl Pooled for ModuleDeclarationType_Base {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.module_declaration_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.module_declaration_type_base }
}

impl<'a> Extract<'a> for ModuleDeclarationType_Base {
    const TYPE_NAME: &'static str = "ModuleDeclarationType_Base";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ModuleDeclaration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclaration {
    /// DCB field: `module` (String)
    #[serde(default)]
    pub module: String,
    /// DCB field: `moduleType` (StrongPointer)
    #[serde(default)]
    pub module_type: Option<Handle<ModuleDeclarationType_Base>>,
    /// DCB field: `properties` (Class (array))
    #[serde(default)]
    pub properties: Vec<Handle<MissionProperty>>,
}

impl Pooled for ModuleDeclaration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.module_declaration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.module_declaration }
}

impl<'a> Extract<'a> for ModuleDeclaration {
    const TYPE_NAME: &'static str = "ModuleDeclaration";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            module: inst.get_str("module").map(String::from).unwrap_or_default(),
            module_type: match inst.get("moduleType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ModuleDeclarationType_Base>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ModuleDeclarationType_Base>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            properties: inst.get_array("properties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionProperty>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionProperty>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `mobiGlasApp`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct mobiGlasApp {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `bindingsNamespace` (String)
    #[serde(default)]
    pub bindings_namespace: String,
    /// DCB field: `appCanvas` (Reference)
    #[serde(default)]
    pub app_canvas: Option<CigGuid>,
    /// DCB field: `homeCanvas` (Reference)
    #[serde(default)]
    pub home_canvas: Option<CigGuid>,
    /// DCB field: `displayIcon` (String)
    #[serde(default)]
    pub display_icon: String,
    /// DCB field: `displayIcon3d` (String)
    #[serde(default)]
    pub display_icon3d: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `appColor` (Class)
    #[serde(default)]
    pub app_color: Option<Handle<SRGBA8>>,
    /// DCB field: `isHidden` (Boolean)
    #[serde(default)]
    pub is_hidden: bool,
    /// DCB field: `defaultAppData` (Reference (array))
    #[serde(default)]
    pub default_app_data: Vec<CigGuid>,
    /// DCB field: `appParams` (StrongPointer)
    #[serde(default)]
    pub app_params: Option<Handle<SMobiGlasAppParamsBase>>,
    /// DCB field: `hostingApps` (Reference (array))
    #[serde(default)]
    pub hosting_apps: Vec<CigGuid>,
    /// DCB field: `legacyApp` (Boolean)
    #[serde(default)]
    pub legacy_app: bool,
}

impl Pooled for mobiGlasApp {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.mobi_glas_app }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.mobi_glas_app }
}

impl<'a> Extract<'a> for mobiGlasApp {
    const TYPE_NAME: &'static str = "mobiGlasApp";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            bindings_namespace: inst.get_str("bindingsNamespace").map(String::from).unwrap_or_default(),
            app_canvas: inst.get("appCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            home_canvas: inst.get("homeCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_icon: inst.get_str("displayIcon").map(String::from).unwrap_or_default(),
            display_icon3d: inst.get_str("displayIcon3d").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            app_color: match inst.get("appColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            is_hidden: inst.get_bool("isHidden").unwrap_or_default(),
            default_app_data: inst.get_array("defaultAppData")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            app_params: match inst.get("appParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMobiGlasAppParamsBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMobiGlasAppParamsBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hosting_apps: inst.get_array("hostingApps")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            legacy_app: inst.get_bool("legacyApp").unwrap_or_default(),
        }
    }
}

/// DCB type: `MoveViewRestrictionPenalty`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveViewRestrictionPenalty {
    /// DCB field: `restrictedMotionPenalty` (Single)
    #[serde(default)]
    pub restricted_motion_penalty: f32,
    /// DCB field: `restrictedViewPenalty` (Single)
    #[serde(default)]
    pub restricted_view_penalty: f32,
}

impl Pooled for MoveViewRestrictionPenalty {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.move_view_restriction_penalty }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.move_view_restriction_penalty }
}

impl<'a> Extract<'a> for MoveViewRestrictionPenalty {
    const TYPE_NAME: &'static str = "MoveViewRestrictionPenalty";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            restricted_motion_penalty: inst.get_f32("restrictedMotionPenalty").unwrap_or_default(),
            restricted_view_penalty: inst.get_f32("restrictedViewPenalty").unwrap_or_default(),
        }
    }
}

/// DCB type: `MoveViewRestrictionWeighting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveViewRestrictionWeighting {
    /// DCB field: `HelmetWeighting` (Single)
    #[serde(default)]
    pub helmet_weighting: f32,
    /// DCB field: `CoreWeighting` (Single)
    #[serde(default)]
    pub core_weighting: f32,
    /// DCB field: `LegsWeighting` (Single)
    #[serde(default)]
    pub legs_weighting: f32,
    /// DCB field: `ArmsWeighting` (Single)
    #[serde(default)]
    pub arms_weighting: f32,
}

impl Pooled for MoveViewRestrictionWeighting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.move_view_restriction_weighting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.move_view_restriction_weighting }
}

impl<'a> Extract<'a> for MoveViewRestrictionWeighting {
    const TYPE_NAME: &'static str = "MoveViewRestrictionWeighting";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            helmet_weighting: inst.get_f32("HelmetWeighting").unwrap_or_default(),
            core_weighting: inst.get_f32("CoreWeighting").unwrap_or_default(),
            legs_weighting: inst.get_f32("LegsWeighting").unwrap_or_default(),
            arms_weighting: inst.get_f32("ArmsWeighting").unwrap_or_default(),
        }
    }
}

/// DCB type: `MovieClipTransformationInterpolatorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieClipTransformationInterpolatorParams {
    /// DCB field: `movieClipName` (String)
    #[serde(default)]
    pub movie_clip_name: String,
    /// DCB field: `transformationInterpolatorParams` (Class)
    #[serde(default)]
    pub transformation_interpolator_params: Option<Handle<TransformationInterpolatorParams>>,
}

impl Pooled for MovieClipTransformationInterpolatorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.movie_clip_transformation_interpolator_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.movie_clip_transformation_interpolator_params }
}

impl<'a> Extract<'a> for MovieClipTransformationInterpolatorParams {
    const TYPE_NAME: &'static str = "MovieClipTransformationInterpolatorParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            movie_clip_name: inst.get_str("movieClipName").map(String::from).unwrap_or_default(),
            transformation_interpolator_params: match inst.get("transformationInterpolatorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TransformationInterpolatorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TransformationInterpolatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MovieClipTransformationInterpolator`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieClipTransformationInterpolator {
    /// DCB field: `interpolationTime` (Single)
    #[serde(default)]
    pub interpolation_time: f32,
    /// DCB field: `movieClipTransformationInterpolatorParams` (Class (array))
    #[serde(default)]
    pub movie_clip_transformation_interpolator_params: Vec<Handle<MovieClipTransformationInterpolatorParams>>,
}

impl Pooled for MovieClipTransformationInterpolator {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.movie_clip_transformation_interpolator }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.movie_clip_transformation_interpolator }
}

impl<'a> Extract<'a> for MovieClipTransformationInterpolator {
    const TYPE_NAME: &'static str = "MovieClipTransformationInterpolator";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interpolation_time: inst.get_f32("interpolationTime").unwrap_or_default(),
            movie_clip_transformation_interpolator_params: inst.get_array("movieClipTransformationInterpolatorParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MovieClipTransformationInterpolatorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MovieClipTransformationInterpolatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MobiGlasAppData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasAppData {
    /// DCB field: `appData` (StrongPointer)
    #[serde(default)]
    pub app_data: Option<Handle<MobiGlasAppDataBase>>,
}

impl Pooled for MobiGlasAppData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.mobi_glas_app_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.mobi_glas_app_data }
}

impl<'a> Extract<'a> for MobiGlasAppData {
    const TYPE_NAME: &'static str = "MobiGlasAppData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            app_data: match inst.get("appData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MobiGlasAppDataBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MobiGlasAppDataBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MobiGlasAppDataBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasAppDataBase {
}

impl Pooled for MobiGlasAppDataBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.mobi_glas_app_data_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.mobi_glas_app_data_base }
}

impl<'a> Extract<'a> for MobiGlasAppDataBase {
    const TYPE_NAME: &'static str = "MobiGlasAppDataBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MobiGlasMissionNote`
///
/// Inherits from: `MobiGlasAppDataBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobiGlasMissionNote {
    /// DCB field: `noteTitle` (Locale)
    #[serde(default)]
    pub note_title: String,
    /// DCB field: `noteContent` (Locale (array))
    #[serde(default)]
    pub note_content: Vec<String>,
    /// DCB field: `appLink` (Class)
    #[serde(default)]
    pub app_link: Option<Handle<SMobiGlasAppLink>>,
}

impl Pooled for MobiGlasMissionNote {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_mo.mobi_glas_mission_note }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_mo.mobi_glas_mission_note }
}

impl<'a> Extract<'a> for MobiGlasMissionNote {
    const TYPE_NAME: &'static str = "MobiGlasMissionNote";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            note_title: inst.get_str("noteTitle").map(String::from).unwrap_or_default(),
            note_content: inst.get_array("noteContent")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            app_link: match inst.get("appLink") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMobiGlasAppLink>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMobiGlasAppLink>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

