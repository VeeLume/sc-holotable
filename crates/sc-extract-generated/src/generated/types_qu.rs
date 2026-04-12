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

/// DCB type: `QueueingBehaviour`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueingBehaviour {
    /// DCB field: `canInterrupt` (Boolean)
    #[serde(default)]
    pub can_interrupt: bool,
    /// DCB field: `canBeInterrupted` (Boolean)
    #[serde(default)]
    pub can_be_interrupted: bool,
    /// DCB field: `canBeQueued` (Boolean)
    #[serde(default)]
    pub can_be_queued: bool,
}

impl Pooled for QueueingBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.queueing_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.queueing_behaviour }
}

impl<'a> Extract<'a> for QueueingBehaviour {
    const TYPE_NAME: &'static str = "QueueingBehaviour";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            can_interrupt: inst.get_bool("canInterrupt").unwrap_or_default(),
            can_be_interrupted: inst.get_bool("canBeInterrupted").unwrap_or_default(),
            can_be_queued: inst.get_bool("canBeQueued").unwrap_or_default(),
        }
    }
}

/// DCB type: `Quat`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quat {
    /// DCB field: `Rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Ang3>>,
}

impl Pooled for Quat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quat }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quat }
}

impl<'a> Extract<'a> for Quat {
    const TYPE_NAME: &'static str = "Quat";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rotation: match inst.get("Rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `QuatT`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuatT {
    /// DCB field: `Rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Ang3>>,
    /// DCB field: `Position` (Class)
    #[serde(default)]
    pub position: Option<Handle<Vec3>>,
}

impl Pooled for QuatT {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quat_t }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quat_t }
}

impl<'a> Extract<'a> for QuatT {
    const TYPE_NAME: &'static str = "QuatT";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rotation: match inst.get("Rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            position: match inst.get("Position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `QuantumDriveSplineRollbackParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveSplineRollbackParams {
    /// DCB field: `targetAlignmentToBeginRollBack` (Single)
    #[serde(default)]
    pub target_alignment_to_begin_roll_back: f32,
    /// DCB field: `fullRotationDistance` (Single)
    #[serde(default)]
    pub full_rotation_distance: f32,
}

impl Pooled for QuantumDriveSplineRollbackParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_spline_rollback_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_spline_rollback_params }
}

impl<'a> Extract<'a> for QuantumDriveSplineRollbackParams {
    const TYPE_NAME: &'static str = "QuantumDriveSplineRollbackParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            target_alignment_to_begin_roll_back: inst.get_f32("targetAlignmentToBeginRollBack").unwrap_or_default(),
            full_rotation_distance: inst.get_f32("fullRotationDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumDriveSplineFXNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveSplineFXNode {
    /// DCB field: `normAge` (Single)
    #[serde(default)]
    pub norm_age: f32,
    /// DCB field: `splitSegment` (Boolean)
    #[serde(default)]
    pub split_segment: bool,
    /// DCB field: `cornerPoint` (Boolean)
    #[serde(default)]
    pub corner_point: bool,
}

impl Pooled for QuantumDriveSplineFXNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_spline_fxnode }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_spline_fxnode }
}

impl<'a> Extract<'a> for QuantumDriveSplineFXNode {
    const TYPE_NAME: &'static str = "QuantumDriveSplineFXNode";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            norm_age: inst.get_f32("normAge").unwrap_or_default(),
            split_segment: inst.get_bool("splitSegment").unwrap_or_default(),
            corner_point: inst.get_bool("cornerPoint").unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumDriveSplineFXParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveSplineFXParams {
    /// DCB field: `splineParticleEffect` (Class)
    #[serde(default)]
    pub spline_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// DCB field: `startNodeParams` (Class)
    #[serde(default)]
    pub start_node_params: Option<Handle<QuantumDriveSplineFXNode>>,
    /// DCB field: `segmentNodeParams` (Class)
    #[serde(default)]
    pub segment_node_params: Option<Handle<QuantumDriveSplineFXNode>>,
    /// DCB field: `numSegmentNodes` (Int32)
    #[serde(default)]
    pub num_segment_nodes: i32,
    /// DCB field: `endNodeParams` (Class)
    #[serde(default)]
    pub end_node_params: Option<Handle<QuantumDriveSplineFXNode>>,
}

impl Pooled for QuantumDriveSplineFXParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_spline_fxparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_spline_fxparams }
}

impl<'a> Extract<'a> for QuantumDriveSplineFXParams {
    const TYPE_NAME: &'static str = "QuantumDriveSplineFXParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spline_particle_effect: match inst.get("splineParticleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            start_node_params: match inst.get("startNodeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveSplineFXNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveSplineFXNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            segment_node_params: match inst.get("segmentNodeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveSplineFXNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveSplineFXNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            num_segment_nodes: inst.get_i32("numSegmentNodes").unwrap_or_default(),
            end_node_params: match inst.get("endNodeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveSplineFXNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveSplineFXNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `QuantumDriveSplineTraversalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveSplineTraversalParams {
    /// DCB field: `splineFX` (Class)
    #[serde(default)]
    pub spline_fx: Option<Handle<QuantumDriveSplineFXParams>>,
    /// DCB field: `rollUnderFullRotationDistance` (Single)
    #[serde(default)]
    pub roll_under_full_rotation_distance: f32,
    /// DCB field: `rollUnderStartPhase` (EnumChoice)
    #[serde(default)]
    pub roll_under_start_phase: String,
    /// DCB field: `arrivalRadiusScalar` (Single)
    #[serde(default)]
    pub arrival_radius_scalar: f32,
    /// DCB field: `nearAlignmentThreshold` (Single)
    #[serde(default)]
    pub near_alignment_threshold: f32,
    /// DCB field: `nearTensionScalar` (Single)
    #[serde(default)]
    pub near_tension_scalar: f32,
    /// DCB field: `midTensionScalar` (Single)
    #[serde(default)]
    pub mid_tension_scalar: f32,
    /// DCB field: `farTensionScalar` (Single)
    #[serde(default)]
    pub far_tension_scalar: f32,
    /// DCB field: `originNearTensionScalar` (Single)
    #[serde(default)]
    pub origin_near_tension_scalar: f32,
    /// DCB field: `originMidTensionScalar` (Single)
    #[serde(default)]
    pub origin_mid_tension_scalar: f32,
    /// DCB field: `originFarTensionScalar` (Single)
    #[serde(default)]
    pub origin_far_tension_scalar: f32,
    /// DCB field: `targetNearTensionScalar` (Single)
    #[serde(default)]
    pub target_near_tension_scalar: f32,
    /// DCB field: `targetMidTensionScalar` (Single)
    #[serde(default)]
    pub target_mid_tension_scalar: f32,
    /// DCB field: `targetFarTensionScalar` (Single)
    #[serde(default)]
    pub target_far_tension_scalar: f32,
    /// DCB field: `angleDerivedTensionExponent` (Single)
    #[serde(default)]
    pub angle_derived_tension_exponent: f32,
    /// DCB field: `tensionModifiersExponent` (Single)
    #[serde(default)]
    pub tension_modifiers_exponent: f32,
    /// DCB field: `baseTensionMin` (Single)
    #[serde(default)]
    pub base_tension_min: f32,
    /// DCB field: `baseTensionMax` (Single)
    #[serde(default)]
    pub base_tension_max: f32,
    /// DCB field: `tangentPlanetScalar` (Single)
    #[serde(default)]
    pub tangent_planet_scalar: f32,
    /// DCB field: `maxAlignmentToUseTangentDirection` (Single)
    #[serde(default)]
    pub max_alignment_to_use_tangent_direction: f32,
    /// DCB field: `nearEndingTargetOrientationRatio` (Single)
    #[serde(default)]
    pub near_ending_target_orientation_ratio: f32,
    /// DCB field: `midEndingTargetOrientationRatio` (Single)
    #[serde(default)]
    pub mid_ending_target_orientation_ratio: f32,
    /// DCB field: `farEndingTargetOrientationRatio` (Single)
    #[serde(default)]
    pub far_ending_target_orientation_ratio: f32,
    /// DCB field: `nearArrivalOrientationThreshold` (Single)
    #[serde(default)]
    pub near_arrival_orientation_threshold: f32,
    /// DCB field: `midDerivedTargetPositionAlignment` (Single)
    #[serde(default)]
    pub mid_derived_target_position_alignment: f32,
    /// DCB field: `farDerivedTargetPositionAlignment` (Single)
    #[serde(default)]
    pub far_derived_target_position_alignment: f32,
    /// DCB field: `rotationBehavior` (EnumChoice)
    #[serde(default)]
    pub rotation_behavior: String,
    /// DCB field: `rollbackParams` (Class)
    #[serde(default)]
    pub rollback_params: Option<Handle<QuantumDriveSplineRollbackParams>>,
}

impl Pooled for QuantumDriveSplineTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_spline_traversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_spline_traversal_params }
}

impl<'a> Extract<'a> for QuantumDriveSplineTraversalParams {
    const TYPE_NAME: &'static str = "QuantumDriveSplineTraversalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spline_fx: match inst.get("splineFX") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveSplineFXParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveSplineFXParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll_under_full_rotation_distance: inst.get_f32("rollUnderFullRotationDistance").unwrap_or_default(),
            roll_under_start_phase: inst.get_str("rollUnderStartPhase").map(String::from).unwrap_or_default(),
            arrival_radius_scalar: inst.get_f32("arrivalRadiusScalar").unwrap_or_default(),
            near_alignment_threshold: inst.get_f32("nearAlignmentThreshold").unwrap_or_default(),
            near_tension_scalar: inst.get_f32("nearTensionScalar").unwrap_or_default(),
            mid_tension_scalar: inst.get_f32("midTensionScalar").unwrap_or_default(),
            far_tension_scalar: inst.get_f32("farTensionScalar").unwrap_or_default(),
            origin_near_tension_scalar: inst.get_f32("originNearTensionScalar").unwrap_or_default(),
            origin_mid_tension_scalar: inst.get_f32("originMidTensionScalar").unwrap_or_default(),
            origin_far_tension_scalar: inst.get_f32("originFarTensionScalar").unwrap_or_default(),
            target_near_tension_scalar: inst.get_f32("targetNearTensionScalar").unwrap_or_default(),
            target_mid_tension_scalar: inst.get_f32("targetMidTensionScalar").unwrap_or_default(),
            target_far_tension_scalar: inst.get_f32("targetFarTensionScalar").unwrap_or_default(),
            angle_derived_tension_exponent: inst.get_f32("angleDerivedTensionExponent").unwrap_or_default(),
            tension_modifiers_exponent: inst.get_f32("tensionModifiersExponent").unwrap_or_default(),
            base_tension_min: inst.get_f32("baseTensionMin").unwrap_or_default(),
            base_tension_max: inst.get_f32("baseTensionMax").unwrap_or_default(),
            tangent_planet_scalar: inst.get_f32("tangentPlanetScalar").unwrap_or_default(),
            max_alignment_to_use_tangent_direction: inst.get_f32("maxAlignmentToUseTangentDirection").unwrap_or_default(),
            near_ending_target_orientation_ratio: inst.get_f32("nearEndingTargetOrientationRatio").unwrap_or_default(),
            mid_ending_target_orientation_ratio: inst.get_f32("midEndingTargetOrientationRatio").unwrap_or_default(),
            far_ending_target_orientation_ratio: inst.get_f32("farEndingTargetOrientationRatio").unwrap_or_default(),
            near_arrival_orientation_threshold: inst.get_f32("nearArrivalOrientationThreshold").unwrap_or_default(),
            mid_derived_target_position_alignment: inst.get_f32("midDerivedTargetPositionAlignment").unwrap_or_default(),
            far_derived_target_position_alignment: inst.get_f32("farDerivedTargetPositionAlignment").unwrap_or_default(),
            rotation_behavior: inst.get_str("rotationBehavior").map(String::from).unwrap_or_default(),
            rollback_params: match inst.get("rollbackParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveSplineRollbackParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveSplineRollbackParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `QuantumDriveNotification`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveNotification {
    /// DCB field: `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// DCB field: `screenTimer` (Single)
    #[serde(default)]
    pub screen_timer: f32,
    /// DCB field: `hurryScreenTimer` (Single)
    #[serde(default)]
    pub hurry_screen_timer: f32,
    /// DCB field: `blocking` (Boolean)
    #[serde(default)]
    pub blocking: bool,
}

impl Pooled for QuantumDriveNotification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_notification }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_notification }
}

impl<'a> Extract<'a> for QuantumDriveNotification {
    const TYPE_NAME: &'static str = "QuantumDriveNotification";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
            screen_timer: inst.get_f32("screenTimer").unwrap_or_default(),
            hurry_screen_timer: inst.get_f32("hurryScreenTimer").unwrap_or_default(),
            blocking: inst.get_bool("blocking").unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumDriveNotifications`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveNotifications {
    /// DCB field: `calibrationStarted` (Class)
    #[serde(default)]
    pub calibration_started: Option<Handle<QuantumDriveNotification>>,
    /// DCB field: `calibrationCompleted` (Class)
    #[serde(default)]
    pub calibration_completed: Option<Handle<QuantumDriveNotification>>,
    /// DCB field: `calibrationFailed` (Class)
    #[serde(default)]
    pub calibration_failed: Option<Handle<QuantumDriveNotification>>,
    /// DCB field: `playerNameToken` (String)
    #[serde(default)]
    pub player_name_token: String,
    /// DCB field: `selectedDestinationToken` (String)
    #[serde(default)]
    pub selected_destination_token: String,
}

impl Pooled for QuantumDriveNotifications {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_notifications }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_notifications }
}

impl<'a> Extract<'a> for QuantumDriveNotifications {
    const TYPE_NAME: &'static str = "QuantumDriveNotifications";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            calibration_started: match inst.get("calibrationStarted") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveNotification>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveNotification>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            calibration_completed: match inst.get("calibrationCompleted") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveNotification>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveNotification>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            calibration_failed: match inst.get("calibrationFailed") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveNotification>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveNotification>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_name_token: inst.get_str("playerNameToken").map(String::from).unwrap_or_default(),
            selected_destination_token: inst.get_str("selectedDestinationToken").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumDriveAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveAudioParams {
    /// DCB field: `spoolStartAudioTrigger` (Class)
    #[serde(default)]
    pub spool_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `spoolStopAudioTrigger` (Class)
    #[serde(default)]
    pub spool_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `spoolCancelAudioTrigger` (Class)
    #[serde(default)]
    pub spool_cancel_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `spoolFailAudioTrigger` (Class)
    #[serde(default)]
    pub spool_fail_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `alignStartAudioTrigger` (Class)
    #[serde(default)]
    pub align_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `alignStopAudioTrigger` (Class)
    #[serde(default)]
    pub align_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `preRampUpStartAudioTrigger` (Class)
    #[serde(default)]
    pub pre_ramp_up_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `preRampUpStopAudioTrigger` (Class)
    #[serde(default)]
    pub pre_ramp_up_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `rampUpStartAudioTrigger` (Class)
    #[serde(default)]
    pub ramp_up_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `rampUpStopAudioTrigger` (Class)
    #[serde(default)]
    pub ramp_up_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `flightInProgressStartAudioTrigger` (Class)
    #[serde(default)]
    pub flight_in_progress_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `flightInProgressStopAudioTrigger` (Class)
    #[serde(default)]
    pub flight_in_progress_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `rampDownStartAudioTrigger` (Class)
    #[serde(default)]
    pub ramp_down_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `rampDownStopAudioTrigger` (Class)
    #[serde(default)]
    pub ramp_down_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `postRampDownStartAudioTrigger` (Class)
    #[serde(default)]
    pub post_ramp_down_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `postRampDownStopAudioTrigger` (Class)
    #[serde(default)]
    pub post_ramp_down_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `coolDownStartAudioTrigger` (Class)
    #[serde(default)]
    pub cool_down_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `coolDownStopAudioTrigger` (Class)
    #[serde(default)]
    pub cool_down_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `spoolTimeRTPC` (Class)
    #[serde(default)]
    pub spool_time_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `fuelExpendedRTPC` (Class)
    #[serde(default)]
    pub fuel_expended_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `timeSpentTravellingRTPC` (Class)
    #[serde(default)]
    pub time_spent_travelling_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `shortRangeRTPC` (Class)
    #[serde(default)]
    pub short_range_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `totalTravelTimeRTPC` (Class)
    #[serde(default)]
    pub total_travel_time_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `speedNormalizedCurTripRTPC` (Class)
    #[serde(default)]
    pub speed_normalized_cur_trip_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `speedNormalizedOverallRTPC` (Class)
    #[serde(default)]
    pub speed_normalized_overall_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `uiStartAudioTrigger` (Class)
    #[serde(default)]
    pub ui_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiStopAudioTrigger` (Class)
    #[serde(default)]
    pub ui_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiSpoolStartAudioTrigger` (Class)
    #[serde(default)]
    pub ui_spool_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiSpoolCancelAudioTrigger` (Class)
    #[serde(default)]
    pub ui_spool_cancel_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiSpoolDoneAudioTrigger` (Class)
    #[serde(default)]
    pub ui_spool_done_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiSpoolStartAudioTriggerLoop` (Class)
    #[serde(default)]
    pub ui_spool_start_audio_trigger_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiSpoolStopAudioTriggerLoop` (Class)
    #[serde(default)]
    pub ui_spool_stop_audio_trigger_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiCalibrationRunStartAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_run_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiCalibrationRunStopAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_run_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiCalibrationWarningStartAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_warning_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiCalibrationWarningStopAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_warning_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiCalibrationInterruptAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_interrupt_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiCalibrationBeginAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_begin_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiCalibrationFailAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_fail_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiCalibrationDoneAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_done_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiPartyAllReady` (Class)
    #[serde(default)]
    pub ui_party_all_ready: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiPartyMemberAligned` (Class)
    #[serde(default)]
    pub ui_party_member_aligned: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiPartyMemberAlignmentLost` (Class)
    #[serde(default)]
    pub ui_party_member_alignment_lost: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiPartyMemberDrop` (Class)
    #[serde(default)]
    pub ui_party_member_drop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiPartyMemberReady` (Class)
    #[serde(default)]
    pub ui_party_member_ready: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `uiPartyMemberSpooledUp` (Class)
    #[serde(default)]
    pub ui_party_member_spooled_up: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `activateQTAudioSpline` (Reference)
    #[serde(default)]
    pub activate_qtaudio_spline: Option<CigGuid>,
}

impl Pooled for QuantumDriveAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_audio_params }
}

impl<'a> Extract<'a> for QuantumDriveAudioParams {
    const TYPE_NAME: &'static str = "QuantumDriveAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spool_start_audio_trigger: match inst.get("spoolStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spool_stop_audio_trigger: match inst.get("spoolStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spool_cancel_audio_trigger: match inst.get("spoolCancelAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spool_fail_audio_trigger: match inst.get("spoolFailAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            align_start_audio_trigger: match inst.get("alignStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            align_stop_audio_trigger: match inst.get("alignStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pre_ramp_up_start_audio_trigger: match inst.get("preRampUpStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pre_ramp_up_stop_audio_trigger: match inst.get("preRampUpStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ramp_up_start_audio_trigger: match inst.get("rampUpStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ramp_up_stop_audio_trigger: match inst.get("rampUpStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            flight_in_progress_start_audio_trigger: match inst.get("flightInProgressStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            flight_in_progress_stop_audio_trigger: match inst.get("flightInProgressStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ramp_down_start_audio_trigger: match inst.get("rampDownStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ramp_down_stop_audio_trigger: match inst.get("rampDownStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            post_ramp_down_start_audio_trigger: match inst.get("postRampDownStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            post_ramp_down_stop_audio_trigger: match inst.get("postRampDownStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cool_down_start_audio_trigger: match inst.get("coolDownStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cool_down_stop_audio_trigger: match inst.get("coolDownStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spool_time_rtpc: match inst.get("spoolTimeRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fuel_expended_rtpc: match inst.get("fuelExpendedRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            time_spent_travelling_rtpc: match inst.get("timeSpentTravellingRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            short_range_rtpc: match inst.get("shortRangeRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            total_travel_time_rtpc: match inst.get("totalTravelTimeRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            speed_normalized_cur_trip_rtpc: match inst.get("speedNormalizedCurTripRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            speed_normalized_overall_rtpc: match inst.get("speedNormalizedOverallRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_start_audio_trigger: match inst.get("uiStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_stop_audio_trigger: match inst.get("uiStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_spool_start_audio_trigger: match inst.get("uiSpoolStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_spool_cancel_audio_trigger: match inst.get("uiSpoolCancelAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_spool_done_audio_trigger: match inst.get("uiSpoolDoneAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_spool_start_audio_trigger_loop: match inst.get("uiSpoolStartAudioTriggerLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_spool_stop_audio_trigger_loop: match inst.get("uiSpoolStopAudioTriggerLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_calibration_run_start_audio_trigger: match inst.get("uiCalibrationRunStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_calibration_run_stop_audio_trigger: match inst.get("uiCalibrationRunStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_calibration_warning_start_audio_trigger: match inst.get("uiCalibrationWarningStartAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_calibration_warning_stop_audio_trigger: match inst.get("uiCalibrationWarningStopAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_calibration_interrupt_audio_trigger: match inst.get("uiCalibrationInterruptAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_calibration_begin_audio_trigger: match inst.get("uiCalibrationBeginAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_calibration_fail_audio_trigger: match inst.get("uiCalibrationFailAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_calibration_done_audio_trigger: match inst.get("uiCalibrationDoneAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_party_all_ready: match inst.get("uiPartyAllReady") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_party_member_aligned: match inst.get("uiPartyMemberAligned") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_party_member_alignment_lost: match inst.get("uiPartyMemberAlignmentLost") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_party_member_drop: match inst.get("uiPartyMemberDrop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_party_member_ready: match inst.get("uiPartyMemberReady") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ui_party_member_spooled_up: match inst.get("uiPartyMemberSpooledUp") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            activate_qtaudio_spline: inst.get("activateQTAudioSpline").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `QuantumDriveGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveGlobalParams {
    /// DCB field: `splineTraversalParams` (Class)
    #[serde(default)]
    pub spline_traversal_params: Option<Handle<QuantumDriveSplineTraversalParams>>,
    /// DCB field: `notifications` (Class)
    #[serde(default)]
    pub notifications: Option<Handle<QuantumDriveNotifications>>,
    /// DCB field: `audioParams` (Class)
    #[serde(default)]
    pub audio_params: Option<Handle<QuantumDriveAudioParams>>,
    /// DCB field: `musicParams` (Class)
    #[serde(default)]
    pub music_params: Option<Handle<QuantumMusicParams>>,
    /// DCB field: `maxRaycastDist` (Single)
    #[serde(default)]
    pub max_raycast_dist: f32,
    /// DCB field: `maxLinkingRange` (Single)
    #[serde(default)]
    pub max_linking_range: f32,
    /// DCB field: `maxIconScaleRange` (Single)
    #[serde(default)]
    pub max_icon_scale_range: f32,
    /// DCB field: `minimumAltitudeForQuantum` (Single)
    #[serde(default)]
    pub minimum_altitude_for_quantum: f32,
    /// DCB field: `maximumAtmosphericPressureForQuantum` (Single)
    #[serde(default)]
    pub maximum_atmospheric_pressure_for_quantum: f32,
    /// DCB field: `interdictionNavPointClass` (Reference)
    #[serde(default)]
    pub interdiction_nav_point_class: Option<CigGuid>,
    /// DCB field: `scatterScale` (Single)
    #[serde(default)]
    pub scatter_scale: f32,
    /// DCB field: `inputDelay` (Single)
    #[serde(default)]
    pub input_delay: f32,
}

impl Pooled for QuantumDriveGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_global_params }
}

impl<'a> Extract<'a> for QuantumDriveGlobalParams {
    const TYPE_NAME: &'static str = "QuantumDriveGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spline_traversal_params: match inst.get("splineTraversalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveSplineTraversalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveSplineTraversalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notifications: match inst.get("notifications") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveNotifications>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveNotifications>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_params: match inst.get("audioParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumDriveAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumDriveAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            music_params: match inst.get("musicParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_raycast_dist: inst.get_f32("maxRaycastDist").unwrap_or_default(),
            max_linking_range: inst.get_f32("maxLinkingRange").unwrap_or_default(),
            max_icon_scale_range: inst.get_f32("maxIconScaleRange").unwrap_or_default(),
            minimum_altitude_for_quantum: inst.get_f32("minimumAltitudeForQuantum").unwrap_or_default(),
            maximum_atmospheric_pressure_for_quantum: inst.get_f32("maximumAtmosphericPressureForQuantum").unwrap_or_default(),
            interdiction_nav_point_class: inst.get("interdictionNavPointClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            scatter_scale: inst.get_f32("scatterScale").unwrap_or_default(),
            input_delay: inst.get_f32("inputDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumDriveEffectSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveEffectSettings {
    /// DCB field: `spoolingEffectFadeInDuration` (Single)
    #[serde(default)]
    pub spooling_effect_fade_in_duration: f32,
    /// DCB field: `spoolingEffectFadeOutDuration` (Single)
    #[serde(default)]
    pub spooling_effect_fade_out_duration: f32,
    /// DCB field: `spoolingEffectMultiplier` (Single)
    #[serde(default)]
    pub spooling_effect_multiplier: f32,
    /// DCB field: `spoolingEffectSpeedInput` (Single)
    #[serde(default)]
    pub spooling_effect_speed_input: f32,
}

impl Pooled for QuantumDriveEffectSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_effect_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_effect_settings }
}

impl<'a> Extract<'a> for QuantumDriveEffectSettings {
    const TYPE_NAME: &'static str = "QuantumDriveEffectSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            spooling_effect_fade_in_duration: inst.get_f32("spoolingEffectFadeInDuration").unwrap_or_default(),
            spooling_effect_fade_out_duration: inst.get_f32("spoolingEffectFadeOutDuration").unwrap_or_default(),
            spooling_effect_multiplier: inst.get_f32("spoolingEffectMultiplier").unwrap_or_default(),
            spooling_effect_speed_input: inst.get_f32("spoolingEffectSpeedInput").unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumMusicEventBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMusicEventBase {
    /// DCB field: `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// DCB field: `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for QuantumMusicEventBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_music_event_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_music_event_base }
}

impl<'a> Extract<'a> for QuantumMusicEventBase {
    const TYPE_NAME: &'static str = "QuantumMusicEventBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            music_logic_event: inst.get("musicLogicEvent").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            music_wwise_event: match inst.get("musicWwiseEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `QuantumDriveStateMusicMap`
///
/// Inherits from: `QuantumMusicEventBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveStateMusicMap {
    /// DCB field: `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// DCB field: `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `state` (EnumChoice)
    #[serde(default)]
    pub state: String,
}

impl Pooled for QuantumDriveStateMusicMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_state_music_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_state_music_map }
}

impl<'a> Extract<'a> for QuantumDriveStateMusicMap {
    const TYPE_NAME: &'static str = "QuantumDriveStateMusicMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            music_logic_event: inst.get("musicLogicEvent").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            music_wwise_event: match inst.get("musicWwiseEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            state: inst.get_str("state").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumStateMusicMap`
///
/// Inherits from: `QuantumMusicEventBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumStateMusicMap {
    /// DCB field: `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// DCB field: `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `state` (EnumChoice)
    #[serde(default)]
    pub state: String,
}

impl Pooled for QuantumStateMusicMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_state_music_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_state_music_map }
}

impl<'a> Extract<'a> for QuantumStateMusicMap {
    const TYPE_NAME: &'static str = "QuantumStateMusicMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            music_logic_event: inst.get("musicLogicEvent").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            music_wwise_event: match inst.get("musicWwiseEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            state: inst.get_str("state").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumDriveLocTypeMusicMap`
///
/// Inherits from: `QuantumMusicEventBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveLocTypeMusicMap {
    /// DCB field: `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// DCB field: `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for QuantumDriveLocTypeMusicMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_drive_loc_type_music_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_drive_loc_type_music_map }
}

impl<'a> Extract<'a> for QuantumDriveLocTypeMusicMap {
    const TYPE_NAME: &'static str = "QuantumDriveLocTypeMusicMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            music_logic_event: inst.get("musicLogicEvent").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            music_wwise_event: match inst.get("musicWwiseEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumEffectMusic`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumEffectMusic {
    /// DCB field: `spoolStart` (Class)
    #[serde(default)]
    pub spool_start: Option<Handle<QuantumMusicEventBase>>,
    /// DCB field: `pinch` (Class)
    #[serde(default)]
    pub pinch: Option<Handle<QuantumMusicEventBase>>,
    /// DCB field: `entryFlash` (Class)
    #[serde(default)]
    pub entry_flash: Option<Handle<QuantumMusicEventBase>>,
    /// DCB field: `trailStart` (Class)
    #[serde(default)]
    pub trail_start: Option<Handle<QuantumMusicEventBase>>,
    /// DCB field: `travelStart` (Class)
    #[serde(default)]
    pub travel_start: Option<Handle<QuantumMusicEventBase>>,
    /// DCB field: `travelEnd` (Class)
    #[serde(default)]
    pub travel_end: Option<Handle<QuantumMusicEventBase>>,
    /// DCB field: `exitTrigger` (Class)
    #[serde(default)]
    pub exit_trigger: Option<Handle<QuantumMusicEventBase>>,
}

impl Pooled for QuantumEffectMusic {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_effect_music }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_effect_music }
}

impl<'a> Extract<'a> for QuantumEffectMusic {
    const TYPE_NAME: &'static str = "QuantumEffectMusic";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spool_start: match inst.get("spoolStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pinch: match inst.get("pinch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entry_flash: match inst.get("entryFlash") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            trail_start: match inst.get("trailStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            travel_start: match inst.get("travelStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            travel_end: match inst.get("travelEnd") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exit_trigger: match inst.get("exitTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `QuantumMusicTripCategory`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMusicTripCategory {
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `minDistance` (Double)
    #[serde(default)]
    pub min_distance: f64,
    /// DCB field: `categoryEvent` (Class)
    #[serde(default)]
    pub category_event: Option<Handle<QuantumMusicEventBase>>,
    /// DCB field: `preArrivalDurationSecs` (Single)
    #[serde(default)]
    pub pre_arrival_duration_secs: f32,
}

impl Pooled for QuantumMusicTripCategory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_music_trip_category }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_music_trip_category }
}

impl<'a> Extract<'a> for QuantumMusicTripCategory {
    const TYPE_NAME: &'static str = "QuantumMusicTripCategory";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            min_distance: inst.get_f64("minDistance").unwrap_or_default(),
            category_event: match inst.get("categoryEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pre_arrival_duration_secs: inst.get_f32("preArrivalDurationSecs").unwrap_or_default(),
        }
    }
}

/// DCB type: `QuantumMusicParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMusicParams {
    /// DCB field: `enableMusic` (Boolean)
    #[serde(default)]
    pub enable_music: bool,
    /// DCB field: `stateMap` (Class (array))
    #[serde(default)]
    pub state_map: Vec<Handle<QuantumDriveStateMusicMap>>,
    /// DCB field: `stateMapNew` (Class (array))
    #[serde(default)]
    pub state_map_new: Vec<Handle<QuantumStateMusicMap>>,
    /// DCB field: `effectMusic` (Class)
    #[serde(default)]
    pub effect_music: Option<Handle<QuantumEffectMusic>>,
    /// DCB field: `tripCategory` (Class (array))
    #[serde(default)]
    pub trip_category: Vec<Handle<QuantumMusicTripCategory>>,
    /// DCB field: `preArrivalDurationSecs` (Single)
    #[serde(default)]
    pub pre_arrival_duration_secs: f32,
    /// DCB field: `preArrivalMusicEvent` (Class)
    #[serde(default)]
    pub pre_arrival_music_event: Option<Handle<QuantumMusicEventBase>>,
    /// DCB field: `onStopUseMusicEvent` (Class)
    #[serde(default)]
    pub on_stop_use_music_event: Option<Handle<QuantumMusicEventBase>>,
    /// DCB field: `journeyProgressRTPC` (Class)
    #[serde(default)]
    pub journey_progress_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `locationTypeMap` (Class (array))
    #[serde(default)]
    pub location_type_map: Vec<Handle<QuantumDriveLocTypeMusicMap>>,
}

impl Pooled for QuantumMusicParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quantum_music_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quantum_music_params }
}

impl<'a> Extract<'a> for QuantumMusicParams {
    const TYPE_NAME: &'static str = "QuantumMusicParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_music: inst.get_bool("enableMusic").unwrap_or_default(),
            state_map: inst.get_array("stateMap")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<QuantumDriveStateMusicMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<QuantumDriveStateMusicMap>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            state_map_new: inst.get_array("stateMapNew")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<QuantumStateMusicMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<QuantumStateMusicMap>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            effect_music: match inst.get("effectMusic") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumEffectMusic>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumEffectMusic>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            trip_category: inst.get_array("tripCategory")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<QuantumMusicTripCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<QuantumMusicTripCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            pre_arrival_duration_secs: inst.get_f32("preArrivalDurationSecs").unwrap_or_default(),
            pre_arrival_music_event: match inst.get("preArrivalMusicEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_stop_use_music_event: match inst.get("onStopUseMusicEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuantumMusicEventBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuantumMusicEventBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            journey_progress_rtpc: match inst.get("journeyProgressRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            location_type_map: inst.get_array("locationTypeMap")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<QuantumDriveLocTypeMusicMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<QuantumDriveLocTypeMusicMap>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `QuickAccessWheelElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAccessWheelElement {
    /// DCB field: `openContextMenuAsFolder` (Boolean)
    #[serde(default)]
    pub open_context_menu_as_folder: bool,
}

impl Pooled for QuickAccessWheelElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_qu.quick_access_wheel_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_qu.quick_access_wheel_element }
}

impl<'a> Extract<'a> for QuickAccessWheelElement {
    const TYPE_NAME: &'static str = "QuickAccessWheelElement";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            open_context_menu_as_folder: inst.get_bool("openContextMenuAsFolder").unwrap_or_default(),
        }
    }
}

