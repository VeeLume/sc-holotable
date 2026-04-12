// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `motionstatemachine`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MotionConnection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionConnection {
    /// `waitUntillFinished` (Boolean)
    #[serde(default)]
    pub wait_untill_finished: bool,
    /// `delaySeconds` (Single)
    #[serde(default)]
    pub delay_seconds: f32,
    /// `waitForEvent` (String)
    #[serde(default)]
    pub wait_for_event: String,
    /// `nextState` (WeakPointer)
    #[serde(default)]
    pub next_state: Option<Handle<MotionState>>,
}

impl Pooled for MotionConnection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.motion_connection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.motion_connection }
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
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// `mannequinTags` (String)
    #[serde(default)]
    pub mannequin_tags: String,
    /// `mannequinFragment` (String)
    #[serde(default)]
    pub mannequin_fragment: String,
    /// `motionTypeFP` (EnumChoice)
    #[serde(default)]
    pub motion_type_fp: String,
    /// `motionTypeTP` (EnumChoice)
    #[serde(default)]
    pub motion_type_tp: String,
    /// `motionTypeRemote` (EnumChoice)
    #[serde(default)]
    pub motion_type_remote: String,
    /// `connections` (Class (array))
    #[serde(default)]
    pub connections: Vec<Handle<MotionConnection>>,
}

impl Pooled for MotionState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.motion_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.motion_state }
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

/// DCB type: `ProceduralIdleToMoveParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralIdleToMoveParams {
    /// `maxMovementSpeed` (Single)
    #[serde(default)]
    pub max_movement_speed: f32,
    /// `maxHipTilt` (Single)
    #[serde(default)]
    pub max_hip_tilt: f32,
    /// `maxHipVerticalOffset` (Single)
    #[serde(default)]
    pub max_hip_vertical_offset: f32,
    /// `maxHipHorizontalOffset` (Single)
    #[serde(default)]
    pub max_hip_horizontal_offset: f32,
    /// `maxSpineBend` (Single)
    #[serde(default)]
    pub max_spine_bend: f32,
    /// `tiltDuration` (Single)
    #[serde(default)]
    pub tilt_duration: f32,
    /// `tiltRestorationDuration` (Single)
    #[serde(default)]
    pub tilt_restoration_duration: f32,
}

impl Pooled for ProceduralIdleToMoveParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.procedural_idle_to_move_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.procedural_idle_to_move_params }
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

/// DCB type: `MotionSmoothingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionSmoothingParams {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `maxJitterRating` (Single)
    #[serde(default)]
    pub max_jitter_rating: f32,
    /// `jitterSensitivity` (Single)
    #[serde(default)]
    pub jitter_sensitivity: f32,
    /// `jitterDetectionThreshold` (Single)
    #[serde(default)]
    pub jitter_detection_threshold: f32,
    /// `jitterDecayDuration` (Single)
    #[serde(default)]
    pub jitter_decay_duration: f32,
    /// `speedSmoothingDuration` (Single)
    #[serde(default)]
    pub speed_smoothing_duration: f32,
}

impl Pooled for MotionSmoothingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.motion_smoothing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.motion_smoothing_params }
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
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `jukeTriggerAngle` (Single)
    #[serde(default)]
    pub juke_trigger_angle: f32,
    /// `jukeDetectionDuration` (Single)
    #[serde(default)]
    pub juke_detection_duration: f32,
}

impl Pooled for MotionJukeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.motion_juke_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.motion_juke_params }
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
    /// `rightYawTrigger` (Single)
    #[serde(default)]
    pub right_yaw_trigger: f32,
    /// `leftYawTrigger` (Single)
    #[serde(default)]
    pub left_yaw_trigger: f32,
    /// `turnTriggerOffset` (Single)
    #[serde(default)]
    pub turn_trigger_offset: f32,
    /// `targetOffset` (Single)
    #[serde(default)]
    pub target_offset: f32,
    /// `snapOffset` (Single)
    #[serde(default)]
    pub snap_offset: f32,
    /// `maxTurnSpeed` (Single)
    #[serde(default)]
    pub max_turn_speed: f32,
    /// `delayIdleSeconds` (Single)
    #[serde(default)]
    pub delay_idle_seconds: f32,
}

impl Pooled for MotionTurnParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.motion_turn_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.motion_turn_params }
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
/// Inherits from: `ActorMotionStateFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionTurnSetupFiltered {
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
    pub params: Option<Handle<MotionTurnParams>>,
}

impl Pooled for MotionTurnSetupFiltered {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.motion_turn_setup_filtered }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.motion_turn_setup_filtered }
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
    /// `setupList` (Class (array))
    #[serde(default)]
    pub setup_list: Vec<Handle<MotionTurnSetupFiltered>>,
}

impl Pooled for MotionTurnSetupList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.motion_turn_setup_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.motion_turn_setup_list }
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
    /// `footShiftDuration` (Single)
    #[serde(default)]
    pub foot_shift_duration: f32,
    /// `autoRepositionDistance` (Single)
    #[serde(default)]
    pub auto_reposition_distance: f32,
    /// `animationDuration` (Single)
    #[serde(default)]
    pub animation_duration: f32,
    /// `animationWeight` (Single)
    #[serde(default)]
    pub animation_weight: f32,
    /// `animationBlendInDuration` (Single)
    #[serde(default)]
    pub animation_blend_in_duration: f32,
    /// `animationBlendOutDuration` (Single)
    #[serde(default)]
    pub animation_blend_out_duration: f32,
}

impl Pooled for MotionFootPinningParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.motion_foot_pinning_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.motion_foot_pinning_params }
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
    /// `motionStates` (Class (array))
    #[serde(default)]
    pub motion_states: Vec<Handle<MotionState>>,
    /// `motionSmoothingConfig` (Class)
    #[serde(default)]
    pub motion_smoothing_config: Option<Handle<MotionSmoothingParams>>,
    /// `jukeConfig` (Class)
    #[serde(default)]
    pub juke_config: Option<Handle<MotionJukeParams>>,
    /// `idleToMoveProcParamsForward` (Class)
    #[serde(default)]
    pub idle_to_move_proc_params_forward: Option<Handle<ProceduralIdleToMoveParams>>,
    /// `idleToMoveProcParamsBack` (Class)
    #[serde(default)]
    pub idle_to_move_proc_params_back: Option<Handle<ProceduralIdleToMoveParams>>,
    /// `idleToMoveProcParamsRight` (Class)
    #[serde(default)]
    pub idle_to_move_proc_params_right: Option<Handle<ProceduralIdleToMoveParams>>,
    /// `idleToMoveProcParamsLeft` (Class)
    #[serde(default)]
    pub idle_to_move_proc_params_left: Option<Handle<ProceduralIdleToMoveParams>>,
    /// `turnConfig` (Class)
    #[serde(default)]
    pub turn_config: Option<Handle<MotionTurnSetupList>>,
    /// `footPinningParams` (Class)
    #[serde(default)]
    pub foot_pinning_params: Option<Handle<MotionFootPinningParams>>,
}

impl Pooled for MotionGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.motion_graph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.motion_graph }
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

/// DCB type: `SCProneMotionGraphDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCProneMotionGraphDef {
    /// `turnTriggerYawThreshold` (Single)
    #[serde(default)]
    pub turn_trigger_yaw_threshold: f32,
}

impl Pooled for SCProneMotionGraphDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.scprone_motion_graph_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.scprone_motion_graph_def }
}

impl<'a> Extract<'a> for SCProneMotionGraphDef {
    const TYPE_NAME: &'static str = "SCProneMotionGraphDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            turn_trigger_yaw_threshold: inst.get_f32("turnTriggerYawThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `IMannequinActionDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IMannequinActionDef {
}

impl Pooled for IMannequinActionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.imannequin_action_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.imannequin_action_def }
}

impl<'a> Extract<'a> for IMannequinActionDef {
    const TYPE_NAME: &'static str = "IMannequinActionDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SMannequinActionDefRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMannequinActionDefRecord {
    /// `actionDef` (StrongPointer)
    #[serde(default)]
    pub action_def: Option<Handle<IMannequinActionDef>>,
}

impl Pooled for SMannequinActionDefRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.motionstatemachine.smannequin_action_def_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.motionstatemachine.smannequin_action_def_record }
}

impl<'a> Extract<'a> for SMannequinActionDefRecord {
    const TYPE_NAME: &'static str = "SMannequinActionDefRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            action_def: match inst.get("actionDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<IMannequinActionDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<IMannequinActionDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

