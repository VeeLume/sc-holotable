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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `MotionConnection`
pub struct MotionConnection {
    /// `waitUntillFinished` (Boolean)
    pub wait_untill_finished: bool,
    /// `delaySeconds` (Single)
    pub delay_seconds: f32,
    /// `waitForEvent` (String)
    pub wait_for_event: String,
    /// `nextState` (WeakPointer)
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
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MotionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MotionState`
pub struct MotionState {
    /// `type` (EnumChoice)
    pub r#type: MotionStateType,
    /// `mannequinTags` (String)
    pub mannequin_tags: String,
    /// `mannequinFragment` (String)
    pub mannequin_fragment: String,
    /// `motionTypeFP` (EnumChoice)
    pub motion_type_fp: MotionControlType,
    /// `motionTypeTP` (EnumChoice)
    pub motion_type_tp: MotionControlType,
    /// `motionTypeRemote` (EnumChoice)
    pub motion_type_remote: MotionControlType,
    /// `connections` (Class (array))
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
            r#type: MotionStateType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            mannequin_tags: inst.get_str("mannequinTags").map(String::from).unwrap_or_default(),
            mannequin_fragment: inst.get_str("mannequinFragment").map(String::from).unwrap_or_default(),
            motion_type_fp: MotionControlType::from_dcb_str(inst.get_str("motionTypeFP").unwrap_or("")),
            motion_type_tp: MotionControlType::from_dcb_str(inst.get_str("motionTypeTP").unwrap_or("")),
            motion_type_remote: MotionControlType::from_dcb_str(inst.get_str("motionTypeRemote").unwrap_or("")),
            connections: inst.get_array("connections")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MotionConnection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<MotionConnection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralIdleToMoveParams`
pub struct ProceduralIdleToMoveParams {
    /// `maxMovementSpeed` (Single)
    pub max_movement_speed: f32,
    /// `maxHipTilt` (Single)
    pub max_hip_tilt: f32,
    /// `maxHipVerticalOffset` (Single)
    pub max_hip_vertical_offset: f32,
    /// `maxHipHorizontalOffset` (Single)
    pub max_hip_horizontal_offset: f32,
    /// `maxSpineBend` (Single)
    pub max_spine_bend: f32,
    /// `tiltDuration` (Single)
    pub tilt_duration: f32,
    /// `tiltRestorationDuration` (Single)
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
pub struct MotionSmoothingParams {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `maxJitterRating` (Single)
    pub max_jitter_rating: f32,
    /// `jitterSensitivity` (Single)
    pub jitter_sensitivity: f32,
    /// `jitterDetectionThreshold` (Single)
    pub jitter_detection_threshold: f32,
    /// `jitterDecayDuration` (Single)
    pub jitter_decay_duration: f32,
    /// `speedSmoothingDuration` (Single)
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
pub struct MotionJukeParams {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `jukeTriggerAngle` (Single)
    pub juke_trigger_angle: f32,
    /// `jukeDetectionDuration` (Single)
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
pub struct MotionTurnParams {
    /// `rightYawTrigger` (Single)
    pub right_yaw_trigger: f32,
    /// `leftYawTrigger` (Single)
    pub left_yaw_trigger: f32,
    /// `turnTriggerOffset` (Single)
    pub turn_trigger_offset: f32,
    /// `targetOffset` (Single)
    pub target_offset: f32,
    /// `snapOffset` (Single)
    pub snap_offset: f32,
    /// `maxTurnSpeed` (Single)
    pub max_turn_speed: f32,
    /// `delayIdleSeconds` (Single)
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
pub struct MotionTurnSetupFiltered {
    /// `filterName` (String)
    pub filter_name: String,
    /// `filterByState` (EnumChoice)
    pub filter_by_state: ActorStateFilterByState,
    /// `filterByMotionSpeed` (EnumChoice)
    pub filter_by_motion_speed: ActorStateFilterByMotionSpeed,
    /// `filterByPoseState` (EnumChoice)
    pub filter_by_pose_state: ActorStateFilterByPoseState,
    /// `filterByStanceState` (EnumChoice)
    pub filter_by_stance_state: ActorStateFilterByStanceState,
    /// `filterByAimStanceState` (EnumChoice)
    pub filter_by_aim_stance_state: ActorStateFilterByAimStanceState,
    /// `filterByLeanState` (EnumChoice)
    pub filter_by_lean_state: ActorStateFilterByLeanState,
    /// `filterByHeldItemType` (EnumChoice)
    pub filter_by_held_item_type: ActorStateFilterByHeldItemType,
    /// `filterBySkeleton` (EnumChoice)
    pub filter_by_skeleton: ActorStateFilterBySkeleton,
    /// `filterByCharacterType` (EnumChoice)
    pub filter_by_character_type: ActorStateFilterByCharacterType,
    /// `filterByRestrainedState` (EnumChoice)
    pub filter_by_restrained_state: EActorStateFilterByBoolState,
    /// `filterByPlayerCamera` (EnumChoice)
    pub filter_by_player_camera: EActorStateFilterByPlayerCamera,
    /// `filterByAimingRestriction` (EnumChoice)
    pub filter_by_aiming_restriction: EActorStateFilterByAimingRestriction,
    /// `filterByLocomotionSet` (EnumChoice)
    pub filter_by_locomotion_set: ActorStateFilterByLocomotionSet,
    /// `params` (Class)
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
            filter_by_state: ActorStateFilterByState::from_dcb_str(inst.get_str("filterByState").unwrap_or("")),
            filter_by_motion_speed: ActorStateFilterByMotionSpeed::from_dcb_str(inst.get_str("filterByMotionSpeed").unwrap_or("")),
            filter_by_pose_state: ActorStateFilterByPoseState::from_dcb_str(inst.get_str("filterByPoseState").unwrap_or("")),
            filter_by_stance_state: ActorStateFilterByStanceState::from_dcb_str(inst.get_str("filterByStanceState").unwrap_or("")),
            filter_by_aim_stance_state: ActorStateFilterByAimStanceState::from_dcb_str(inst.get_str("filterByAimStanceState").unwrap_or("")),
            filter_by_lean_state: ActorStateFilterByLeanState::from_dcb_str(inst.get_str("filterByLeanState").unwrap_or("")),
            filter_by_held_item_type: ActorStateFilterByHeldItemType::from_dcb_str(inst.get_str("filterByHeldItemType").unwrap_or("")),
            filter_by_skeleton: ActorStateFilterBySkeleton::from_dcb_str(inst.get_str("filterBySkeleton").unwrap_or("")),
            filter_by_character_type: ActorStateFilterByCharacterType::from_dcb_str(inst.get_str("filterByCharacterType").unwrap_or("")),
            filter_by_restrained_state: EActorStateFilterByBoolState::from_dcb_str(inst.get_str("filterByRestrainedState").unwrap_or("")),
            filter_by_player_camera: EActorStateFilterByPlayerCamera::from_dcb_str(inst.get_str("filterByPlayerCamera").unwrap_or("")),
            filter_by_aiming_restriction: EActorStateFilterByAimingRestriction::from_dcb_str(inst.get_str("filterByAimingRestriction").unwrap_or("")),
            filter_by_locomotion_set: ActorStateFilterByLocomotionSet::from_dcb_str(inst.get_str("filterByLocomotionSet").unwrap_or("")),
            params: match inst.get("params") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionTurnParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MotionTurnSetupList`
pub struct MotionTurnSetupList {
    /// `setupList` (Class (array))
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
                        Value::ClassRef(r) => Some(b.alloc_nested::<MotionTurnSetupFiltered>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MotionFootPinningParams`
pub struct MotionFootPinningParams {
    /// `footShiftDuration` (Single)
    pub foot_shift_duration: f32,
    /// `autoRepositionDistance` (Single)
    pub auto_reposition_distance: f32,
    /// `animationDuration` (Single)
    pub animation_duration: f32,
    /// `animationWeight` (Single)
    pub animation_weight: f32,
    /// `animationBlendInDuration` (Single)
    pub animation_blend_in_duration: f32,
    /// `animationBlendOutDuration` (Single)
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
pub struct MotionGraph {
    /// `motionStates` (Class (array))
    pub motion_states: Vec<Handle<MotionState>>,
    /// `motionSmoothingConfig` (Class)
    pub motion_smoothing_config: Option<Handle<MotionSmoothingParams>>,
    /// `jukeConfig` (Class)
    pub juke_config: Option<Handle<MotionJukeParams>>,
    /// `idleToMoveProcParamsForward` (Class)
    pub idle_to_move_proc_params_forward: Option<Handle<ProceduralIdleToMoveParams>>,
    /// `idleToMoveProcParamsBack` (Class)
    pub idle_to_move_proc_params_back: Option<Handle<ProceduralIdleToMoveParams>>,
    /// `idleToMoveProcParamsRight` (Class)
    pub idle_to_move_proc_params_right: Option<Handle<ProceduralIdleToMoveParams>>,
    /// `idleToMoveProcParamsLeft` (Class)
    pub idle_to_move_proc_params_left: Option<Handle<ProceduralIdleToMoveParams>>,
    /// `turnConfig` (Class)
    pub turn_config: Option<Handle<MotionTurnSetupList>>,
    /// `footPinningParams` (Class)
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
                        Value::ClassRef(r) => Some(b.alloc_nested::<MotionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            motion_smoothing_config: match inst.get("motionSmoothingConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionSmoothingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            juke_config: match inst.get("jukeConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionJukeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            idle_to_move_proc_params_forward: match inst.get("idleToMoveProcParamsForward") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            idle_to_move_proc_params_back: match inst.get("idleToMoveProcParamsBack") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            idle_to_move_proc_params_right: match inst.get("idleToMoveProcParamsRight") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            idle_to_move_proc_params_left: match inst.get("idleToMoveProcParamsLeft") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProceduralIdleToMoveParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            turn_config: match inst.get("turnConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionTurnSetupList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            foot_pinning_params: match inst.get("footPinningParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MotionFootPinningParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCProneMotionGraphDef`
pub struct SCProneMotionGraphDef {
    /// `turnTriggerYawThreshold` (Single)
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

