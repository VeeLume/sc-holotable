// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globalquantumdriveparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `QuantumDriveSplineRollbackParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveSplineRollbackParams {
    /// `targetAlignmentToBeginRollBack` (Single)
    #[serde(default)]
    pub target_alignment_to_begin_roll_back: f32,
    /// `fullRotationDistance` (Single)
    #[serde(default)]
    pub full_rotation_distance: f32,
}

impl Pooled for QuantumDriveSplineRollbackParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_spline_rollback_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_spline_rollback_params }
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
    /// `normAge` (Single)
    #[serde(default)]
    pub norm_age: f32,
    /// `splitSegment` (Boolean)
    #[serde(default)]
    pub split_segment: bool,
    /// `cornerPoint` (Boolean)
    #[serde(default)]
    pub corner_point: bool,
}

impl Pooled for QuantumDriveSplineFXNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_spline_fxnode }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_spline_fxnode }
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
    /// `splineParticleEffect` (Class)
    #[serde(default)]
    pub spline_particle_effect: Option<Handle<GlobalResourceParticle>>,
    /// `startNodeParams` (Class)
    #[serde(default)]
    pub start_node_params: Option<Handle<QuantumDriveSplineFXNode>>,
    /// `segmentNodeParams` (Class)
    #[serde(default)]
    pub segment_node_params: Option<Handle<QuantumDriveSplineFXNode>>,
    /// `numSegmentNodes` (Int32)
    #[serde(default)]
    pub num_segment_nodes: i32,
    /// `endNodeParams` (Class)
    #[serde(default)]
    pub end_node_params: Option<Handle<QuantumDriveSplineFXNode>>,
}

impl Pooled for QuantumDriveSplineFXParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_spline_fxparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_spline_fxparams }
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
    /// `splineFX` (Class)
    #[serde(default)]
    pub spline_fx: Option<Handle<QuantumDriveSplineFXParams>>,
    /// `rollUnderFullRotationDistance` (Single)
    #[serde(default)]
    pub roll_under_full_rotation_distance: f32,
    /// `rollUnderStartPhase` (EnumChoice)
    #[serde(default)]
    pub roll_under_start_phase: String,
    /// `arrivalRadiusScalar` (Single)
    #[serde(default)]
    pub arrival_radius_scalar: f32,
    /// `nearAlignmentThreshold` (Single)
    #[serde(default)]
    pub near_alignment_threshold: f32,
    /// `nearTensionScalar` (Single)
    #[serde(default)]
    pub near_tension_scalar: f32,
    /// `midTensionScalar` (Single)
    #[serde(default)]
    pub mid_tension_scalar: f32,
    /// `farTensionScalar` (Single)
    #[serde(default)]
    pub far_tension_scalar: f32,
    /// `originNearTensionScalar` (Single)
    #[serde(default)]
    pub origin_near_tension_scalar: f32,
    /// `originMidTensionScalar` (Single)
    #[serde(default)]
    pub origin_mid_tension_scalar: f32,
    /// `originFarTensionScalar` (Single)
    #[serde(default)]
    pub origin_far_tension_scalar: f32,
    /// `targetNearTensionScalar` (Single)
    #[serde(default)]
    pub target_near_tension_scalar: f32,
    /// `targetMidTensionScalar` (Single)
    #[serde(default)]
    pub target_mid_tension_scalar: f32,
    /// `targetFarTensionScalar` (Single)
    #[serde(default)]
    pub target_far_tension_scalar: f32,
    /// `angleDerivedTensionExponent` (Single)
    #[serde(default)]
    pub angle_derived_tension_exponent: f32,
    /// `tensionModifiersExponent` (Single)
    #[serde(default)]
    pub tension_modifiers_exponent: f32,
    /// `baseTensionMin` (Single)
    #[serde(default)]
    pub base_tension_min: f32,
    /// `baseTensionMax` (Single)
    #[serde(default)]
    pub base_tension_max: f32,
    /// `tangentPlanetScalar` (Single)
    #[serde(default)]
    pub tangent_planet_scalar: f32,
    /// `maxAlignmentToUseTangentDirection` (Single)
    #[serde(default)]
    pub max_alignment_to_use_tangent_direction: f32,
    /// `nearEndingTargetOrientationRatio` (Single)
    #[serde(default)]
    pub near_ending_target_orientation_ratio: f32,
    /// `midEndingTargetOrientationRatio` (Single)
    #[serde(default)]
    pub mid_ending_target_orientation_ratio: f32,
    /// `farEndingTargetOrientationRatio` (Single)
    #[serde(default)]
    pub far_ending_target_orientation_ratio: f32,
    /// `nearArrivalOrientationThreshold` (Single)
    #[serde(default)]
    pub near_arrival_orientation_threshold: f32,
    /// `midDerivedTargetPositionAlignment` (Single)
    #[serde(default)]
    pub mid_derived_target_position_alignment: f32,
    /// `farDerivedTargetPositionAlignment` (Single)
    #[serde(default)]
    pub far_derived_target_position_alignment: f32,
    /// `rotationBehavior` (EnumChoice)
    #[serde(default)]
    pub rotation_behavior: String,
    /// `rollbackParams` (Class)
    #[serde(default)]
    pub rollback_params: Option<Handle<QuantumDriveSplineRollbackParams>>,
}

impl Pooled for QuantumDriveSplineTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_spline_traversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_spline_traversal_params }
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
    /// `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// `screenTimer` (Single)
    #[serde(default)]
    pub screen_timer: f32,
    /// `hurryScreenTimer` (Single)
    #[serde(default)]
    pub hurry_screen_timer: f32,
    /// `blocking` (Boolean)
    #[serde(default)]
    pub blocking: bool,
}

impl Pooled for QuantumDriveNotification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_notification }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_notification }
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
    /// `calibrationStarted` (Class)
    #[serde(default)]
    pub calibration_started: Option<Handle<QuantumDriveNotification>>,
    /// `calibrationCompleted` (Class)
    #[serde(default)]
    pub calibration_completed: Option<Handle<QuantumDriveNotification>>,
    /// `calibrationFailed` (Class)
    #[serde(default)]
    pub calibration_failed: Option<Handle<QuantumDriveNotification>>,
    /// `playerNameToken` (String)
    #[serde(default)]
    pub player_name_token: String,
    /// `selectedDestinationToken` (String)
    #[serde(default)]
    pub selected_destination_token: String,
}

impl Pooled for QuantumDriveNotifications {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_notifications }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_notifications }
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
    /// `spoolStartAudioTrigger` (Class)
    #[serde(default)]
    pub spool_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `spoolStopAudioTrigger` (Class)
    #[serde(default)]
    pub spool_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `spoolCancelAudioTrigger` (Class)
    #[serde(default)]
    pub spool_cancel_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `spoolFailAudioTrigger` (Class)
    #[serde(default)]
    pub spool_fail_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `alignStartAudioTrigger` (Class)
    #[serde(default)]
    pub align_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `alignStopAudioTrigger` (Class)
    #[serde(default)]
    pub align_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `preRampUpStartAudioTrigger` (Class)
    #[serde(default)]
    pub pre_ramp_up_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `preRampUpStopAudioTrigger` (Class)
    #[serde(default)]
    pub pre_ramp_up_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rampUpStartAudioTrigger` (Class)
    #[serde(default)]
    pub ramp_up_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rampUpStopAudioTrigger` (Class)
    #[serde(default)]
    pub ramp_up_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `flightInProgressStartAudioTrigger` (Class)
    #[serde(default)]
    pub flight_in_progress_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `flightInProgressStopAudioTrigger` (Class)
    #[serde(default)]
    pub flight_in_progress_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rampDownStartAudioTrigger` (Class)
    #[serde(default)]
    pub ramp_down_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rampDownStopAudioTrigger` (Class)
    #[serde(default)]
    pub ramp_down_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `postRampDownStartAudioTrigger` (Class)
    #[serde(default)]
    pub post_ramp_down_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `postRampDownStopAudioTrigger` (Class)
    #[serde(default)]
    pub post_ramp_down_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `coolDownStartAudioTrigger` (Class)
    #[serde(default)]
    pub cool_down_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `coolDownStopAudioTrigger` (Class)
    #[serde(default)]
    pub cool_down_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `spoolTimeRTPC` (Class)
    #[serde(default)]
    pub spool_time_rtpc: Option<Handle<AudioRtpc>>,
    /// `fuelExpendedRTPC` (Class)
    #[serde(default)]
    pub fuel_expended_rtpc: Option<Handle<AudioRtpc>>,
    /// `timeSpentTravellingRTPC` (Class)
    #[serde(default)]
    pub time_spent_travelling_rtpc: Option<Handle<AudioRtpc>>,
    /// `shortRangeRTPC` (Class)
    #[serde(default)]
    pub short_range_rtpc: Option<Handle<AudioRtpc>>,
    /// `totalTravelTimeRTPC` (Class)
    #[serde(default)]
    pub total_travel_time_rtpc: Option<Handle<AudioRtpc>>,
    /// `speedNormalizedCurTripRTPC` (Class)
    #[serde(default)]
    pub speed_normalized_cur_trip_rtpc: Option<Handle<AudioRtpc>>,
    /// `speedNormalizedOverallRTPC` (Class)
    #[serde(default)]
    pub speed_normalized_overall_rtpc: Option<Handle<AudioRtpc>>,
    /// `uiStartAudioTrigger` (Class)
    #[serde(default)]
    pub ui_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiStopAudioTrigger` (Class)
    #[serde(default)]
    pub ui_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiSpoolStartAudioTrigger` (Class)
    #[serde(default)]
    pub ui_spool_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiSpoolCancelAudioTrigger` (Class)
    #[serde(default)]
    pub ui_spool_cancel_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiSpoolDoneAudioTrigger` (Class)
    #[serde(default)]
    pub ui_spool_done_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiSpoolStartAudioTriggerLoop` (Class)
    #[serde(default)]
    pub ui_spool_start_audio_trigger_loop: Option<Handle<GlobalResourceAudio>>,
    /// `uiSpoolStopAudioTriggerLoop` (Class)
    #[serde(default)]
    pub ui_spool_stop_audio_trigger_loop: Option<Handle<GlobalResourceAudio>>,
    /// `uiCalibrationRunStartAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_run_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiCalibrationRunStopAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_run_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiCalibrationWarningStartAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_warning_start_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiCalibrationWarningStopAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_warning_stop_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiCalibrationInterruptAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_interrupt_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiCalibrationBeginAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_begin_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiCalibrationFailAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_fail_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiCalibrationDoneAudioTrigger` (Class)
    #[serde(default)]
    pub ui_calibration_done_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `uiPartyAllReady` (Class)
    #[serde(default)]
    pub ui_party_all_ready: Option<Handle<GlobalResourceAudio>>,
    /// `uiPartyMemberAligned` (Class)
    #[serde(default)]
    pub ui_party_member_aligned: Option<Handle<GlobalResourceAudio>>,
    /// `uiPartyMemberAlignmentLost` (Class)
    #[serde(default)]
    pub ui_party_member_alignment_lost: Option<Handle<GlobalResourceAudio>>,
    /// `uiPartyMemberDrop` (Class)
    #[serde(default)]
    pub ui_party_member_drop: Option<Handle<GlobalResourceAudio>>,
    /// `uiPartyMemberReady` (Class)
    #[serde(default)]
    pub ui_party_member_ready: Option<Handle<GlobalResourceAudio>>,
    /// `uiPartyMemberSpooledUp` (Class)
    #[serde(default)]
    pub ui_party_member_spooled_up: Option<Handle<GlobalResourceAudio>>,
    /// `activateQTAudioSpline` (Reference)
    #[serde(default)]
    pub activate_qtaudio_spline: Option<CigGuid>,
}

impl Pooled for QuantumDriveAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_audio_params }
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
    /// `splineTraversalParams` (Class)
    #[serde(default)]
    pub spline_traversal_params: Option<Handle<QuantumDriveSplineTraversalParams>>,
    /// `notifications` (Class)
    #[serde(default)]
    pub notifications: Option<Handle<QuantumDriveNotifications>>,
    /// `audioParams` (Class)
    #[serde(default)]
    pub audio_params: Option<Handle<QuantumDriveAudioParams>>,
    /// `musicParams` (Class)
    #[serde(default)]
    pub music_params: Option<Handle<QuantumMusicParams>>,
    /// `maxRaycastDist` (Single)
    #[serde(default)]
    pub max_raycast_dist: f32,
    /// `maxLinkingRange` (Single)
    #[serde(default)]
    pub max_linking_range: f32,
    /// `maxIconScaleRange` (Single)
    #[serde(default)]
    pub max_icon_scale_range: f32,
    /// `minimumAltitudeForQuantum` (Single)
    #[serde(default)]
    pub minimum_altitude_for_quantum: f32,
    /// `maximumAtmosphericPressureForQuantum` (Single)
    #[serde(default)]
    pub maximum_atmospheric_pressure_for_quantum: f32,
    /// `interdictionNavPointClass` (Reference)
    #[serde(default)]
    pub interdiction_nav_point_class: Option<CigGuid>,
    /// `scatterScale` (Single)
    #[serde(default)]
    pub scatter_scale: f32,
    /// `inputDelay` (Single)
    #[serde(default)]
    pub input_delay: f32,
}

impl Pooled for QuantumDriveGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_global_params }
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

/// DCB type: `QuantumMusicEventBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMusicEventBase {
    /// `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for QuantumMusicEventBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_music_event_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_music_event_base }
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
/// Inherits from: `QuantumMusicEventBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveStateMusicMap {
    /// `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
    /// `state` (EnumChoice)
    #[serde(default)]
    pub state: String,
}

impl Pooled for QuantumDriveStateMusicMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_state_music_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_state_music_map }
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
/// Inherits from: `QuantumMusicEventBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumStateMusicMap {
    /// `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
    /// `state` (EnumChoice)
    #[serde(default)]
    pub state: String,
}

impl Pooled for QuantumStateMusicMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_state_music_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_state_music_map }
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
/// Inherits from: `QuantumMusicEventBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumDriveLocTypeMusicMap {
    /// `musicLogicEvent` (Reference)
    #[serde(default)]
    pub music_logic_event: Option<CigGuid>,
    /// `musicWwiseEvent` (Class)
    #[serde(default)]
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for QuantumDriveLocTypeMusicMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_drive_loc_type_music_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_drive_loc_type_music_map }
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
    /// `spoolStart` (Class)
    #[serde(default)]
    pub spool_start: Option<Handle<QuantumMusicEventBase>>,
    /// `pinch` (Class)
    #[serde(default)]
    pub pinch: Option<Handle<QuantumMusicEventBase>>,
    /// `entryFlash` (Class)
    #[serde(default)]
    pub entry_flash: Option<Handle<QuantumMusicEventBase>>,
    /// `trailStart` (Class)
    #[serde(default)]
    pub trail_start: Option<Handle<QuantumMusicEventBase>>,
    /// `travelStart` (Class)
    #[serde(default)]
    pub travel_start: Option<Handle<QuantumMusicEventBase>>,
    /// `travelEnd` (Class)
    #[serde(default)]
    pub travel_end: Option<Handle<QuantumMusicEventBase>>,
    /// `exitTrigger` (Class)
    #[serde(default)]
    pub exit_trigger: Option<Handle<QuantumMusicEventBase>>,
}

impl Pooled for QuantumEffectMusic {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_effect_music }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_effect_music }
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
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `minDistance` (Double)
    #[serde(default)]
    pub min_distance: f64,
    /// `categoryEvent` (Class)
    #[serde(default)]
    pub category_event: Option<Handle<QuantumMusicEventBase>>,
    /// `preArrivalDurationSecs` (Single)
    #[serde(default)]
    pub pre_arrival_duration_secs: f32,
}

impl Pooled for QuantumMusicTripCategory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_music_trip_category }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_music_trip_category }
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
    /// `enableMusic` (Boolean)
    #[serde(default)]
    pub enable_music: bool,
    /// `stateMap` (Class (array))
    #[serde(default)]
    pub state_map: Vec<Handle<QuantumDriveStateMusicMap>>,
    /// `stateMapNew` (Class (array))
    #[serde(default)]
    pub state_map_new: Vec<Handle<QuantumStateMusicMap>>,
    /// `effectMusic` (Class)
    #[serde(default)]
    pub effect_music: Option<Handle<QuantumEffectMusic>>,
    /// `tripCategory` (Class (array))
    #[serde(default)]
    pub trip_category: Vec<Handle<QuantumMusicTripCategory>>,
    /// `preArrivalDurationSecs` (Single)
    #[serde(default)]
    pub pre_arrival_duration_secs: f32,
    /// `preArrivalMusicEvent` (Class)
    #[serde(default)]
    pub pre_arrival_music_event: Option<Handle<QuantumMusicEventBase>>,
    /// `onStopUseMusicEvent` (Class)
    #[serde(default)]
    pub on_stop_use_music_event: Option<Handle<QuantumMusicEventBase>>,
    /// `journeyProgressRTPC` (Class)
    #[serde(default)]
    pub journey_progress_rtpc: Option<Handle<AudioRtpc>>,
    /// `locationTypeMap` (Class (array))
    #[serde(default)]
    pub location_type_map: Vec<Handle<QuantumDriveLocTypeMusicMap>>,
}

impl Pooled for QuantumMusicParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalquantumdriveparams.quantum_music_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalquantumdriveparams.quantum_music_params }
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

