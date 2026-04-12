// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `audio`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SAudioBreathParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAudioBreathParameters {
    /// `BreathHeldExhaleMax` (Single)
    #[serde(default)]
    pub breath_held_exhale_max: f32,
}

impl Pooled for SAudioBreathParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.saudio_breath_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.saudio_breath_parameters }
}

impl<'a> Extract<'a> for SAudioBreathParameters {
    const TYPE_NAME: &'static str = "SAudioBreathParameters";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            breath_held_exhale_max: inst.get_f32("BreathHeldExhaleMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `MistedBreathParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MistedBreathParams {
    /// `apparentTemperatureTrigger` (Single)
    #[serde(default)]
    pub apparent_temperature_trigger: f32,
    /// `minStrengthAtTemperature` (Single)
    #[serde(default)]
    pub min_strength_at_temperature: f32,
    /// `maxStrengthAtTemperature` (Single)
    #[serde(default)]
    pub max_strength_at_temperature: f32,
    /// `particleEffect` (Class)
    #[serde(default)]
    pub particle_effect: Option<Handle<GlobalResourceParticle>>,
}

impl Pooled for MistedBreathParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.misted_breath_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.misted_breath_params }
}

impl<'a> Extract<'a> for MistedBreathParams {
    const TYPE_NAME: &'static str = "MistedBreathParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            apparent_temperature_trigger: inst.get_f32("apparentTemperatureTrigger").unwrap_or_default(),
            min_strength_at_temperature: inst.get_f32("minStrengthAtTemperature").unwrap_or_default(),
            max_strength_at_temperature: inst.get_f32("maxStrengthAtTemperature").unwrap_or_default(),
            particle_effect: match inst.get("particleEffect") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StanceBreathModifier`
/// Inherits from: `ActorStateFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StanceBreathModifier {
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
    /// `modifier` (Single)
    #[serde(default)]
    pub modifier: f32,
}

impl Pooled for StanceBreathModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.stance_breath_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.stance_breath_modifier }
}

impl<'a> Extract<'a> for StanceBreathModifier {
    const TYPE_NAME: &'static str = "StanceBreathModifier";
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
            modifier: inst.get_f32("modifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `HoldExhaleDuration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldExhaleDuration {
    /// `breathHeldRatioMin` (Single)
    #[serde(default)]
    pub breath_held_ratio_min: f32,
    /// `breathHeldRatioMax` (Single)
    #[serde(default)]
    pub breath_held_ratio_max: f32,
    /// `duration` (Single)
    #[serde(default)]
    pub duration: f32,
}

impl Pooled for HoldExhaleDuration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.hold_exhale_duration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.hold_exhale_duration }
}

impl<'a> Extract<'a> for HoldExhaleDuration {
    const TYPE_NAME: &'static str = "HoldExhaleDuration";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            breath_held_ratio_min: inst.get_f32("breathHeldRatioMin").unwrap_or_default(),
            breath_held_ratio_max: inst.get_f32("breathHeldRatioMax").unwrap_or_default(),
            duration: inst.get_f32("duration").unwrap_or_default(),
        }
    }
}

/// DCB type: `BreathDurationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathDurationParams {
    /// `defaultDuration` (Single)
    #[serde(default)]
    pub default_duration: f32,
    /// `durationModifier` (Single)
    #[serde(default)]
    pub duration_modifier: f32,
    /// `holdBreathInhaleTime` (Single)
    #[serde(default)]
    pub hold_breath_inhale_time: f32,
    /// `holdBreathExhaleTimes` (Class (array))
    #[serde(default)]
    pub hold_breath_exhale_times: Vec<Handle<HoldExhaleDuration>>,
}

impl Pooled for BreathDurationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.breath_duration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.breath_duration_params }
}

impl<'a> Extract<'a> for BreathDurationParams {
    const TYPE_NAME: &'static str = "BreathDurationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_duration: inst.get_f32("defaultDuration").unwrap_or_default(),
            duration_modifier: inst.get_f32("durationModifier").unwrap_or_default(),
            hold_breath_inhale_time: inst.get_f32("holdBreathInhaleTime").unwrap_or_default(),
            hold_breath_exhale_times: inst.get_array("holdBreathExhaleTimes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<HoldExhaleDuration>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<HoldExhaleDuration>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioBreathStyleCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleCondition {
    /// `whenBreathParameter` (EnumChoice)
    #[serde(default)]
    pub when_breath_parameter: String,
    /// `isAbove` (Single)
    #[serde(default)]
    pub is_above: f32,
    /// `andBelow` (Single)
    #[serde(default)]
    pub and_below: f32,
    /// `andJumpsUpBy` (Single)
    #[serde(default)]
    pub and_jumps_up_by: f32,
    /// `orDropsBy` (Single)
    #[serde(default)]
    pub or_drops_by: f32,
}

impl Pooled for AudioBreathStyleCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_style_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_style_condition }
}

impl<'a> Extract<'a> for AudioBreathStyleCondition {
    const TYPE_NAME: &'static str = "AudioBreathStyleCondition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            when_breath_parameter: inst.get_str("whenBreathParameter").map(String::from).unwrap_or_default(),
            is_above: inst.get_f32("isAbove").unwrap_or_default(),
            and_below: inst.get_f32("andBelow").unwrap_or_default(),
            and_jumps_up_by: inst.get_f32("andJumpsUpBy").unwrap_or_default(),
            or_drops_by: inst.get_f32("orDropsBy").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioBreathStyleConditionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleConditionList {
    /// `list` (Reference (array))
    #[serde(default)]
    pub list: Vec<CigGuid>,
}

impl Pooled for AudioBreathStyleConditionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_style_condition_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_style_condition_list }
}

impl<'a> Extract<'a> for AudioBreathStyleConditionList {
    const TYPE_NAME: &'static str = "AudioBreathStyleConditionList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            list: inst.get_array("list")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioBreathStyleTransitionNode`
/// Inherits from: `AudioBreathStyleBaseNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleTransitionNode {
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `whenConditionsAreTrueFor` (Single)
    #[serde(default)]
    pub when_conditions_are_true_for: f32,
    /// `onNextInhale` (Boolean)
    #[serde(default)]
    pub on_next_inhale: bool,
    /// `onNextExhale` (Boolean)
    #[serde(default)]
    pub on_next_exhale: bool,
    /// `immediately` (Boolean)
    #[serde(default)]
    pub immediately: bool,
    /// `conditions` (Reference (array))
    #[serde(default)]
    pub conditions: Vec<CigGuid>,
    /// `style` (WeakPointer)
    #[serde(default)]
    pub style: Option<Handle<AudioBreathStyleNode>>,
}

impl Pooled for AudioBreathStyleTransitionNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_style_transition_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_style_transition_node }
}

impl<'a> Extract<'a> for AudioBreathStyleTransitionNode {
    const TYPE_NAME: &'static str = "AudioBreathStyleTransitionNode";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            when_conditions_are_true_for: inst.get_f32("whenConditionsAreTrueFor").unwrap_or_default(),
            on_next_inhale: inst.get_bool("onNextInhale").unwrap_or_default(),
            on_next_exhale: inst.get_bool("onNextExhale").unwrap_or_default(),
            immediately: inst.get_bool("immediately").unwrap_or_default(),
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            style: match inst.get("style") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioBreathStyleNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioBreathStyleNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorBreathingStyleStartup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorBreathingStyleStartup {
    /// `forceInhaleFirst` (Boolean)
    #[serde(default)]
    pub force_inhale_first: bool,
    /// `forceExhaleFirst` (Boolean)
    #[serde(default)]
    pub force_exhale_first: bool,
    /// `overrideFirstInhale` (Class)
    #[serde(default)]
    pub override_first_inhale: Option<Handle<GlobalResourceAudio>>,
    /// `overrideFirstExhale` (Class)
    #[serde(default)]
    pub override_first_exhale: Option<Handle<GlobalResourceAudio>>,
    /// `firstBreathDurationFromOverride` (Boolean)
    #[serde(default)]
    pub first_breath_duration_from_override: bool,
    /// `firstBreathDurationCompensation` (Single)
    #[serde(default)]
    pub first_breath_duration_compensation: f32,
    /// `conditions` (Reference (array))
    #[serde(default)]
    pub conditions: Vec<CigGuid>,
}

impl Pooled for ActorBreathingStyleStartup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.actor_breathing_style_startup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.actor_breathing_style_startup }
}

impl<'a> Extract<'a> for ActorBreathingStyleStartup {
    const TYPE_NAME: &'static str = "ActorBreathingStyleStartup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            force_inhale_first: inst.get_bool("forceInhaleFirst").unwrap_or_default(),
            force_exhale_first: inst.get_bool("forceExhaleFirst").unwrap_or_default(),
            override_first_inhale: match inst.get("overrideFirstInhale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            override_first_exhale: match inst.get("overrideFirstExhale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            first_breath_duration_from_override: inst.get_bool("firstBreathDurationFromOverride").unwrap_or_default(),
            first_breath_duration_compensation: inst.get_f32("firstBreathDurationCompensation").unwrap_or_default(),
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioBreathStyle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyle {
    /// `firstBreathCustomisation` (Class (array))
    #[serde(default)]
    pub first_breath_customisation: Vec<Handle<ActorBreathingStyleStartup>>,
    /// `VO2InputParam` (EnumChoice)
    #[serde(default)]
    pub vo2_input_param: String,
    /// `minInputValue` (Single)
    #[serde(default)]
    pub min_input_value: f32,
    /// `maxInputValue` (Single)
    #[serde(default)]
    pub max_input_value: f32,
    /// `InputFallingPredictionTime` (Single)
    #[serde(default)]
    pub input_falling_prediction_time: f32,
    /// `InputRisingPredictionTime` (Single)
    #[serde(default)]
    pub input_rising_prediction_time: f32,
    /// `allowBreathShortening` (Boolean)
    #[serde(default)]
    pub allow_breath_shortening: bool,
    /// `VO2FromInput` (Class)
    #[serde(default)]
    pub vo2_from_input: Option<Handle<BezierCurve>>,
    /// `maxVO2FallRate` (Single)
    #[serde(default)]
    pub max_vo2_fall_rate: f32,
    /// `maxVO2RiseRate` (Single)
    #[serde(default)]
    pub max_vo2_rise_rate: f32,
    /// `durationFromVO2` (Class)
    #[serde(default)]
    pub duration_from_vo2: Option<Handle<BezierCurve>>,
    /// `volumeFromVO2` (Class)
    #[serde(default)]
    pub volume_from_vo2: Option<Handle<BezierCurve>>,
    /// `durationVolumeScaleFromVO2Delta` (Class)
    #[serde(default)]
    pub duration_volume_scale_from_vo2_delta: Option<Handle<BezierCurve>>,
    /// `inhaleExhaleRatioFromVO2Delta` (Class)
    #[serde(default)]
    pub inhale_exhale_ratio_from_vo2_delta: Option<Handle<BezierCurve>>,
    /// `minVolume` (Single)
    #[serde(default)]
    pub min_volume: f32,
    /// `maxVolume` (Single)
    #[serde(default)]
    pub max_volume: f32,
    /// `maxVolumeDrop` (Single)
    #[serde(default)]
    pub max_volume_drop: f32,
    /// `maxVolumeRise` (Single)
    #[serde(default)]
    pub max_volume_rise: f32,
    /// `minDuration` (Single)
    #[serde(default)]
    pub min_duration: f32,
    /// `maxDuration` (Single)
    #[serde(default)]
    pub max_duration: f32,
    /// `maxDurationDrop` (Single)
    #[serde(default)]
    pub max_duration_drop: f32,
    /// `maxDurationRise` (Single)
    #[serde(default)]
    pub max_duration_rise: f32,
    /// `audioEventOnEnter` (Class)
    #[serde(default)]
    pub audio_event_on_enter: Option<Handle<GlobalResourceAudio>>,
    /// `audioEventOnExit` (Class)
    #[serde(default)]
    pub audio_event_on_exit: Option<Handle<GlobalResourceAudio>>,
    /// `audioEvents` (Class)
    #[serde(default)]
    pub audio_events: Option<Handle<GlobalResourceAudio>>,
    /// `landingInterrupt` (Reference)
    #[serde(default)]
    pub landing_interrupt: Option<CigGuid>,
}

impl Pooled for AudioBreathStyle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_style }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_style }
}

impl<'a> Extract<'a> for AudioBreathStyle {
    const TYPE_NAME: &'static str = "AudioBreathStyle";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            first_breath_customisation: inst.get_array("firstBreathCustomisation")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorBreathingStyleStartup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorBreathingStyleStartup>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            vo2_input_param: inst.get_str("VO2InputParam").map(String::from).unwrap_or_default(),
            min_input_value: inst.get_f32("minInputValue").unwrap_or_default(),
            max_input_value: inst.get_f32("maxInputValue").unwrap_or_default(),
            input_falling_prediction_time: inst.get_f32("InputFallingPredictionTime").unwrap_or_default(),
            input_rising_prediction_time: inst.get_f32("InputRisingPredictionTime").unwrap_or_default(),
            allow_breath_shortening: inst.get_bool("allowBreathShortening").unwrap_or_default(),
            vo2_from_input: match inst.get("VO2FromInput") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_vo2_fall_rate: inst.get_f32("maxVO2FallRate").unwrap_or_default(),
            max_vo2_rise_rate: inst.get_f32("maxVO2RiseRate").unwrap_or_default(),
            duration_from_vo2: match inst.get("durationFromVO2") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            volume_from_vo2: match inst.get("volumeFromVO2") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            duration_volume_scale_from_vo2_delta: match inst.get("durationVolumeScaleFromVO2Delta") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inhale_exhale_ratio_from_vo2_delta: match inst.get("inhaleExhaleRatioFromVO2Delta") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_volume: inst.get_f32("minVolume").unwrap_or_default(),
            max_volume: inst.get_f32("maxVolume").unwrap_or_default(),
            max_volume_drop: inst.get_f32("maxVolumeDrop").unwrap_or_default(),
            max_volume_rise: inst.get_f32("maxVolumeRise").unwrap_or_default(),
            min_duration: inst.get_f32("minDuration").unwrap_or_default(),
            max_duration: inst.get_f32("maxDuration").unwrap_or_default(),
            max_duration_drop: inst.get_f32("maxDurationDrop").unwrap_or_default(),
            max_duration_rise: inst.get_f32("maxDurationRise").unwrap_or_default(),
            audio_event_on_enter: match inst.get("audioEventOnEnter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_event_on_exit: match inst.get("audioEventOnExit") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_events: match inst.get("audioEvents") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            landing_interrupt: inst.get("landingInterrupt").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AudioBreathStyleBaseNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleBaseNode {
}

impl Pooled for AudioBreathStyleBaseNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_style_base_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_style_base_node }
}

impl<'a> Extract<'a> for AudioBreathStyleBaseNode {
    const TYPE_NAME: &'static str = "AudioBreathStyleBaseNode";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AudioBreathStyleNode`
/// Inherits from: `AudioBreathStyleBaseNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleNode {
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `style` (Reference)
    #[serde(default)]
    pub style: Option<CigGuid>,
    /// `transitions` (WeakPointer (array))
    #[serde(default)]
    pub transitions: Vec<Handle<AudioBreathStyleTransitionNode>>,
}

impl Pooled for AudioBreathStyleNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_style_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_style_node }
}

impl<'a> Extract<'a> for AudioBreathStyleNode {
    const TYPE_NAME: &'static str = "AudioBreathStyleNode";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            style: inst.get("style").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            transitions: inst.get_array("transitions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioBreathStyleTransitionNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioBreathStyleTransitionNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioBreathStyleSuite`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleSuite {
    /// `initialStyle` (WeakPointer)
    #[serde(default)]
    pub initial_style: Option<Handle<AudioBreathStyleNode>>,
    /// `nodes` (StrongPointer (array))
    #[serde(default)]
    pub nodes: Vec<Handle<AudioBreathStyleBaseNode>>,
}

impl Pooled for AudioBreathStyleSuite {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_style_suite }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_style_suite }
}

impl<'a> Extract<'a> for AudioBreathStyleSuite {
    const TYPE_NAME: &'static str = "AudioBreathStyleSuite";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            initial_style: match inst.get("initialStyle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioBreathStyleNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioBreathStyleNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            nodes: inst.get_array("nodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioBreathStyleBaseNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioBreathStyleBaseNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioBreathDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathDefinition {
    /// `defaultBreathingStyles` (Reference)
    #[serde(default)]
    pub default_breathing_styles: Option<CigGuid>,
    /// `pilotBreathingStyles` (Reference)
    #[serde(default)]
    pub pilot_breathing_styles: Option<CigGuid>,
    /// `params` (Class)
    #[serde(default)]
    pub params: Option<Handle<SAudioBreathParameters>>,
    /// `audioEvents` (Class)
    #[serde(default)]
    pub audio_events: Option<Handle<GlobalResourceAudio>>,
    /// `audioRTPCs` (Class)
    #[serde(default)]
    pub audio_rtpcs: Option<Handle<AudioRtpcWithDefault>>,
    /// `breathVolumeParams` (Class)
    #[serde(default)]
    pub breath_volume_params: Option<Handle<BreathVolumeParams>>,
    /// `breathDurationParams` (Class)
    #[serde(default)]
    pub breath_duration_params: Option<Handle<BreathDurationParams>>,
    /// `holdBreathStylesWhitelist` (Reference (array))
    #[serde(default)]
    pub hold_breath_styles_whitelist: Vec<CigGuid>,
    /// `defaultProcBreathingSetup` (Reference)
    #[serde(default)]
    pub default_proc_breathing_setup: Option<CigGuid>,
    /// `defaultLandingRecord` (Reference)
    #[serde(default)]
    pub default_landing_record: Option<CigGuid>,
    /// `mistedBreathParams` (Class)
    #[serde(default)]
    pub misted_breath_params: Option<Handle<MistedBreathParams>>,
    /// `stanceBreathModifiers` (Class (array))
    #[serde(default)]
    pub stance_breath_modifiers: Vec<Handle<StanceBreathModifier>>,
}

impl Pooled for AudioBreathDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_definition }
}

impl<'a> Extract<'a> for AudioBreathDefinition {
    const TYPE_NAME: &'static str = "AudioBreathDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_breathing_styles: inst.get("defaultBreathingStyles").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pilot_breathing_styles: inst.get("pilotBreathingStyles").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            params: match inst.get("params") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAudioBreathParameters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAudioBreathParameters>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_events: match inst.get("audioEvents") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            audio_rtpcs: match inst.get("audioRTPCs") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpcWithDefault>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpcWithDefault>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            breath_volume_params: match inst.get("breathVolumeParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BreathVolumeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BreathVolumeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            breath_duration_params: match inst.get("breathDurationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BreathDurationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BreathDurationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hold_breath_styles_whitelist: inst.get_array("holdBreathStylesWhitelist")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            default_proc_breathing_setup: inst.get("defaultProcBreathingSetup").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_landing_record: inst.get("defaultLandingRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            misted_breath_params: match inst.get("mistedBreathParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MistedBreathParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MistedBreathParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stance_breath_modifiers: inst.get_array("stanceBreathModifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StanceBreathModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StanceBreathModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioBreathInterrupt`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathInterrupt {
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
    /// `breathEvent` (EnumChoice)
    #[serde(default)]
    pub breath_event: String,
    /// `customEvent` (Class)
    #[serde(default)]
    pub custom_event: Option<Handle<GlobalResourceAudio>>,
    /// `triggerCustomEventOnly` (Boolean)
    #[serde(default)]
    pub trigger_custom_event_only: bool,
}

impl Pooled for AudioBreathInterrupt {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_interrupt }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_interrupt }
}

impl<'a> Extract<'a> for AudioBreathInterrupt {
    const TYPE_NAME: &'static str = "AudioBreathInterrupt";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            breath_event: inst.get_str("breathEvent").map(String::from).unwrap_or_default(),
            custom_event: match inst.get("customEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            trigger_custom_event_only: inst.get_bool("triggerCustomEventOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `BreathingTriggerDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathingTriggerDef {
    /// `audioBreathInterrupt` (Reference)
    #[serde(default)]
    pub audio_breath_interrupt: Option<CigGuid>,
    /// `interruptParam` (Single)
    #[serde(default)]
    pub interrupt_param: f32,
    /// `resumeBreathingWhenAudioEnds` (Boolean)
    #[serde(default)]
    pub resume_breathing_when_audio_ends: bool,
    /// `forceInhaleAfterResume` (Boolean)
    #[serde(default)]
    pub force_inhale_after_resume: bool,
    /// `forceExhaleAfterResume` (Boolean)
    #[serde(default)]
    pub force_exhale_after_resume: bool,
}

impl Pooled for BreathingTriggerDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.breathing_trigger_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.breathing_trigger_def }
}

impl<'a> Extract<'a> for BreathingTriggerDef {
    const TYPE_NAME: &'static str = "BreathingTriggerDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            audio_breath_interrupt: inst.get("audioBreathInterrupt").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            interrupt_param: inst.get_f32("interruptParam").unwrap_or_default(),
            resume_breathing_when_audio_ends: inst.get_bool("resumeBreathingWhenAudioEnds").unwrap_or_default(),
            force_inhale_after_resume: inst.get_bool("forceInhaleAfterResume").unwrap_or_default(),
            force_exhale_after_resume: inst.get_bool("forceExhaleAfterResume").unwrap_or_default(),
        }
    }
}

/// DCB type: `TagTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagTrigger {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `startTrigger` (Class)
    #[serde(default)]
    pub start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `stopTrigger` (Class)
    #[serde(default)]
    pub stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `boneName` (String)
    #[serde(default)]
    pub bone_name: String,
}

impl Pooled for TagTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.tag_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.tag_trigger }
}

impl<'a> Extract<'a> for TagTrigger {
    const TYPE_NAME: &'static str = "TagTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            start_trigger: match inst.get("startTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stop_trigger: match inst.get("stopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bone_name: inst.get_str("boneName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntityAudioControllerParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityAudioControllerParams {
    /// `audioControllerEntityType` (EnumChoice)
    #[serde(default)]
    pub audio_controller_entity_type: String,
    /// `randomSeedCount` (Int32)
    #[serde(default)]
    pub random_seed_count: i32,
    /// `fullLODDistance` (Single)
    #[serde(default)]
    pub full_loddistance: f32,
    /// `fullToLowLODDistance` (Single)
    #[serde(default)]
    pub full_to_low_loddistance: f32,
    /// `offToLowLODDistance` (Single)
    #[serde(default)]
    pub off_to_low_loddistance: f32,
    /// `offLODDistance` (Single)
    #[serde(default)]
    pub off_loddistance: f32,
    /// `occlusionAttenuationScaler` (Single)
    #[serde(default)]
    pub occlusion_attenuation_scaler: f32,
    /// `tags` (Class)
    #[serde(default)]
    pub tags: Option<Handle<TagList>>,
    /// `tagTriggers` (Class (array))
    #[serde(default)]
    pub tag_triggers: Vec<Handle<TagTrigger>>,
}

impl Pooled for SEntityAudioControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.sentity_audio_controller_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.sentity_audio_controller_params }
}

impl<'a> Extract<'a> for SEntityAudioControllerParams {
    const TYPE_NAME: &'static str = "SEntityAudioControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_controller_entity_type: inst.get_str("audioControllerEntityType").map(String::from).unwrap_or_default(),
            random_seed_count: inst.get_i32("randomSeedCount").unwrap_or_default(),
            full_loddistance: inst.get_f32("fullLODDistance").unwrap_or_default(),
            full_to_low_loddistance: inst.get_f32("fullToLowLODDistance").unwrap_or_default(),
            off_to_low_loddistance: inst.get_f32("offToLowLODDistance").unwrap_or_default(),
            off_loddistance: inst.get_f32("offLODDistance").unwrap_or_default(),
            occlusion_attenuation_scaler: inst.get_f32("occlusionAttenuationScaler").unwrap_or_default(),
            tags: match inst.get("tags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tag_triggers: inst.get_array("tagTriggers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagTrigger>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagTrigger>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityAudioControllerTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAudioControllerTypeParams {
    /// `audioControllerParams` (StrongPointer)
    #[serde(default)]
    pub audio_controller_params: Option<Handle<SEntityAudioControllerParams>>,
}

impl Pooled for EntityAudioControllerTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.entity_audio_controller_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.entity_audio_controller_type_params }
}

impl<'a> Extract<'a> for EntityAudioControllerTypeParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerTypeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_controller_params: match inst.get("audioControllerParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEntityAudioControllerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityAudioControllerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityAudioControllerTypeManagementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAudioControllerTypeManagementParams {
    /// `audioControllerEntityType` (EnumChoice)
    #[serde(default)]
    pub audio_controller_entity_type: String,
    /// `maxFullLODs` (Int32)
    #[serde(default)]
    pub max_full_lods: i32,
    /// `maxLowLODs` (Int32)
    #[serde(default)]
    pub max_low_lods: i32,
}

impl Pooled for EntityAudioControllerTypeManagementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.entity_audio_controller_type_management_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.entity_audio_controller_type_management_params }
}

impl<'a> Extract<'a> for EntityAudioControllerTypeManagementParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerTypeManagementParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            audio_controller_entity_type: inst.get_str("audioControllerEntityType").map(String::from).unwrap_or_default(),
            max_full_lods: inst.get_i32("maxFullLODs").unwrap_or_default(),
            max_low_lods: inst.get_i32("maxLowLODs").unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityAudioControllerManagerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAudioControllerManagerParams {
    /// `params` (Class (array))
    #[serde(default)]
    pub params: Vec<Handle<EntityAudioControllerTypeManagementParams>>,
}

impl Pooled for EntityAudioControllerManagerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.entity_audio_controller_manager_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.entity_audio_controller_manager_params }
}

impl<'a> Extract<'a> for EntityAudioControllerManagerParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerManagerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params: inst.get_array("params")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntityAudioControllerTypeManagementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<EntityAudioControllerTypeManagementParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioWhitelist`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioWhitelist {
    /// `triggerTypes` (EnumChoice (array))
    #[serde(default)]
    pub trigger_types: Vec<String>,
}

impl Pooled for AudioWhitelist {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_whitelist }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_whitelist }
}

impl<'a> Extract<'a> for AudioWhitelist {
    const TYPE_NAME: &'static str = "AudioWhitelist";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trigger_types: inst.get_array("triggerTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioEnvironment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEnvironment {
    /// `wwiseEnvironmentName` (String)
    #[serde(default)]
    pub wwise_environment_name: String,
    /// `interiorityMinimum` (Single)
    #[serde(default)]
    pub interiority_minimum: f32,
    /// `interiorityMaximum` (Single)
    #[serde(default)]
    pub interiority_maximum: f32,
    /// `sizeMinimum` (Single)
    #[serde(default)]
    pub size_minimum: f32,
    /// `sizeMaximum` (Single)
    #[serde(default)]
    pub size_maximum: f32,
    /// `primarySurfaceType` (String)
    #[serde(default)]
    pub primary_surface_type: String,
    /// `secondarySurfaceType` (String)
    #[serde(default)]
    pub secondary_surface_type: String,
}

impl Pooled for AudioEnvironment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_environment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_environment }
}

impl<'a> Extract<'a> for AudioEnvironment {
    const TYPE_NAME: &'static str = "AudioEnvironment";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            wwise_environment_name: inst.get_str("wwiseEnvironmentName").map(String::from).unwrap_or_default(),
            interiority_minimum: inst.get_f32("interiorityMinimum").unwrap_or_default(),
            interiority_maximum: inst.get_f32("interiorityMaximum").unwrap_or_default(),
            size_minimum: inst.get_f32("sizeMinimum").unwrap_or_default(),
            size_maximum: inst.get_f32("sizeMaximum").unwrap_or_default(),
            primary_surface_type: inst.get_str("primarySurfaceType").map(String::from).unwrap_or_default(),
            secondary_surface_type: inst.get_str("secondarySurfaceType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioOneShotManagerBudgetEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioOneShotManagerBudgetEntry {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `maxAudioObjects` (Int32)
    #[serde(default)]
    pub max_audio_objects: i32,
    /// `priorityFalloffPerSecond` (Single)
    #[serde(default)]
    pub priority_falloff_per_second: f32,
}

impl Pooled for AudioOneShotManagerBudgetEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_one_shot_manager_budget_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_one_shot_manager_budget_entry }
}

impl<'a> Extract<'a> for AudioOneShotManagerBudgetEntry {
    const TYPE_NAME: &'static str = "AudioOneShotManagerBudgetEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            max_audio_objects: inst.get_i32("maxAudioObjects").unwrap_or_default(),
            priority_falloff_per_second: inst.get_f32("priorityFalloffPerSecond").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioBudgetDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBudgetDefinition {
    /// `oneshotBudget` (Class (array))
    #[serde(default)]
    pub oneshot_budget: Vec<Handle<AudioOneShotManagerBudgetEntry>>,
    /// `shipAudioLimit` (Int32)
    #[serde(default)]
    pub ship_audio_limit: i32,
    /// `shipThrusterLimit` (Int32)
    #[serde(default)]
    pub ship_thruster_limit: i32,
    /// `actorFoleyLimit` (Int32)
    #[serde(default)]
    pub actor_foley_limit: i32,
}

impl Pooled for AudioBudgetDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_budget_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_budget_definition }
}

impl<'a> Extract<'a> for AudioBudgetDefinition {
    const TYPE_NAME: &'static str = "AudioBudgetDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            oneshot_budget: inst.get_array("oneshotBudget")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioOneShotManagerBudgetEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioOneShotManagerBudgetEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            ship_audio_limit: inst.get_i32("shipAudioLimit").unwrap_or_default(),
            ship_thruster_limit: inst.get_i32("shipThrusterLimit").unwrap_or_default(),
            actor_foley_limit: inst.get_i32("actorFoleyLimit").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioGameContextGlobals`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioGameContextGlobals {
    /// `globalStates` (Class (array))
    #[serde(default)]
    pub global_states: Vec<Handle<AudioSwitch>>,
    /// `globalRTPCs` (Class (array))
    #[serde(default)]
    pub global_rtpcs: Vec<Handle<AudioRtpcWithDefault>>,
}

impl Pooled for AudioGameContextGlobals {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_game_context_globals }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_game_context_globals }
}

impl<'a> Extract<'a> for AudioGameContextGlobals {
    const TYPE_NAME: &'static str = "AudioGameContextGlobals";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            global_states: inst.get_array("globalStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioSwitch>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_rtpcs: inst.get_array("globalRTPCs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpcWithDefault>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioRtpcWithDefault>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioGameContext`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioGameContext {
    /// `budgetDefinition` (Reference)
    #[serde(default)]
    pub budget_definition: Option<CigGuid>,
    /// `globalRtpcsAndStates` (Reference)
    #[serde(default)]
    pub global_rtpcs_and_states: Option<CigGuid>,
}

impl Pooled for AudioGameContext {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_game_context }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_game_context }
}

impl<'a> Extract<'a> for AudioGameContext {
    const TYPE_NAME: &'static str = "AudioGameContext";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            budget_definition: inst.get("budgetDefinition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            global_rtpcs_and_states: inst.get("globalRtpcsAndStates").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AudioGameContextSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioGameContextSetup {
    /// `gameContexts` (Class)
    #[serde(default)]
    pub game_contexts: Option<Handle<AudioGameContext>>,
}

impl Pooled for AudioGameContextSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_game_context_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_game_context_setup }
}

impl<'a> Extract<'a> for AudioGameContextSetup {
    const TYPE_NAME: &'static str = "AudioGameContextSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            game_contexts: match inst.get("gameContexts") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioGameContext>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioGameContext>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SurfaceAudioPropertiesDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceAudioPropertiesDefinition {
    /// `surfaceAudioProps` (Class (array))
    #[serde(default)]
    pub surface_audio_props: Vec<Handle<SurfaceAudioProperties>>,
}

impl Pooled for SurfaceAudioPropertiesDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.surface_audio_properties_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.surface_audio_properties_definition }
}

impl<'a> Extract<'a> for SurfaceAudioPropertiesDefinition {
    const TYPE_NAME: &'static str = "SurfaceAudioPropertiesDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            surface_audio_props: inst.get_array("surfaceAudioProps")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SurfaceAudioProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SurfaceAudioProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SurfaceAudioProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceAudioProperties {
    /// `surfaceType` (String)
    #[serde(default)]
    pub surface_type: String,
    /// `damping` (Single)
    #[serde(default)]
    pub damping: f32,
    /// `collisionTrigger` (Class)
    #[serde(default)]
    pub collision_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `collisionTriggerMinPlayTime` (Single)
    #[serde(default)]
    pub collision_trigger_min_play_time: f32,
    /// `slideStartTrigger` (Class)
    #[serde(default)]
    pub slide_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `slideStopTrigger` (Class)
    #[serde(default)]
    pub slide_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rollStartTrigger` (Class)
    #[serde(default)]
    pub roll_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rollStopTrigger` (Class)
    #[serde(default)]
    pub roll_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rtpcSlideVelocity` (Class)
    #[serde(default)]
    pub rtpc_slide_velocity: Option<Handle<AudioRtpc>>,
    /// `rtpcRollVelocity` (Class)
    #[serde(default)]
    pub rtpc_roll_velocity: Option<Handle<AudioRtpc>>,
    /// `rtpcMassOther` (Class)
    #[serde(default)]
    pub rtpc_mass_other: Option<Handle<AudioRtpc>>,
    /// `rtpcTimeSinceLastOneshot` (Class)
    #[serde(default)]
    pub rtpc_time_since_last_oneshot: Option<Handle<AudioRtpc>>,
    /// `rtpcMomentum` (Class)
    #[serde(default)]
    pub rtpc_momentum: Option<Handle<AudioRtpc>>,
    /// `surfaceSwitchAndState` (Class)
    #[serde(default)]
    pub surface_switch_and_state: Option<Handle<AudioSwitch>>,
}

impl Pooled for SurfaceAudioProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.surface_audio_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.surface_audio_properties }
}

impl<'a> Extract<'a> for SurfaceAudioProperties {
    const TYPE_NAME: &'static str = "SurfaceAudioProperties";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            surface_type: inst.get_str("surfaceType").map(String::from).unwrap_or_default(),
            damping: inst.get_f32("damping").unwrap_or_default(),
            collision_trigger: match inst.get("collisionTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            collision_trigger_min_play_time: inst.get_f32("collisionTriggerMinPlayTime").unwrap_or_default(),
            slide_start_trigger: match inst.get("slideStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            slide_stop_trigger: match inst.get("slideStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll_start_trigger: match inst.get("rollStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll_stop_trigger: match inst.get("rollStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_slide_velocity: match inst.get("rtpcSlideVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_roll_velocity: match inst.get("rtpcRollVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_mass_other: match inst.get("rtpcMassOther") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_time_since_last_oneshot: match inst.get("rtpcTimeSinceLastOneshot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_momentum: match inst.get("rtpcMomentum") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            surface_switch_and_state: match inst.get("surfaceSwitchAndState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioSwitch>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalAudioSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalAudioSettings {
    /// `enablePropagationPathing` (Boolean)
    #[serde(default)]
    pub enable_propagation_pathing: bool,
    /// `enablePropagationPathingActorOnly` (Boolean)
    #[serde(default)]
    pub enable_propagation_pathing_actor_only: bool,
    /// `gamePausedTrigger` (Class)
    #[serde(default)]
    pub game_paused_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `gameUnpausedTrigger` (Class)
    #[serde(default)]
    pub game_unpaused_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for GlobalAudioSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.global_audio_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.global_audio_settings }
}

impl<'a> Extract<'a> for GlobalAudioSettings {
    const TYPE_NAME: &'static str = "GlobalAudioSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_propagation_pathing: inst.get_bool("enablePropagationPathing").unwrap_or_default(),
            enable_propagation_pathing_actor_only: inst.get_bool("enablePropagationPathingActorOnly").unwrap_or_default(),
            game_paused_trigger: match inst.get("gamePausedTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            game_unpaused_trigger: match inst.get("gameUnpausedTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AudioTagAction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTagAction {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `audioTrigger` (Class)
    #[serde(default)]
    pub audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `switch` (Class)
    #[serde(default)]
    pub switch: Option<Handle<AudioSwitch>>,
}

impl Pooled for AudioTagAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_tag_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_tag_action }
}

impl<'a> Extract<'a> for AudioTagAction {
    const TYPE_NAME: &'static str = "AudioTagAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            audio_trigger: match inst.get("audioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            switch: match inst.get("switch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioSwitch>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AudioTagActionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTagActionList {
    /// `tagActionList` (Class (array))
    #[serde(default)]
    pub tag_action_list: Vec<Handle<AudioTagAction>>,
}

impl Pooled for AudioTagActionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_tag_action_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_tag_action_list }
}

impl<'a> Extract<'a> for AudioTagActionList {
    const TYPE_NAME: &'static str = "AudioTagActionList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag_action_list: inst.get_array("tagActionList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioTagAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioTagAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioValueOutputBehaviour`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioValueOutputBehaviour {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
}

impl Pooled for AudioValueOutputBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_value_output_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_value_output_behaviour }
}

impl<'a> Extract<'a> for AudioValueOutputBehaviour {
    const TYPE_NAME: &'static str = "AudioValueOutputBehaviour";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioValueOutput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioValueOutput {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `pluginInstanceID` (Int32)
    #[serde(default)]
    pub plugin_instance_id: i32,
    /// `behaviours` (StrongPointer (array))
    #[serde(default)]
    pub behaviours: Vec<Handle<AudioValueOutputBehaviour>>,
}

impl Pooled for AudioValueOutput {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_value_output }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_value_output }
}

impl<'a> Extract<'a> for AudioValueOutput {
    const TYPE_NAME: &'static str = "AudioValueOutput";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            plugin_instance_id: inst.get_i32("pluginInstanceID").unwrap_or_default(),
            behaviours: inst.get_array("behaviours")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioValueOutputBehaviour>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioValueOutputBehaviour>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioValueOutputSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioValueOutputSetup {
    /// `outputs` (Class (array))
    #[serde(default)]
    pub outputs: Vec<Handle<AudioValueOutput>>,
}

impl Pooled for AudioValueOutputSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_value_output_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_value_output_setup }
}

impl<'a> Extract<'a> for AudioValueOutputSetup {
    const TYPE_NAME: &'static str = "AudioValueOutputSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            outputs: inst.get_array("outputs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioValueOutput>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioValueOutput>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `QueueingBehaviour`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueingBehaviour {
    /// `canInterrupt` (Boolean)
    #[serde(default)]
    pub can_interrupt: bool,
    /// `canBeInterrupted` (Boolean)
    #[serde(default)]
    pub can_be_interrupted: bool,
    /// `canBeQueued` (Boolean)
    #[serde(default)]
    pub can_be_queued: bool,
}

impl Pooled for QueueingBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.queueing_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.queueing_behaviour }
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

/// DCB type: `CockpitResponseVariation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CockpitResponseVariation {
    /// `communicationName` (Reference)
    #[serde(default)]
    pub communication_name: Option<CigGuid>,
    /// `initialDelay` (Single)
    #[serde(default)]
    pub initial_delay: f32,
    /// `hasSfx` (Boolean)
    #[serde(default)]
    pub has_sfx: bool,
    /// `refireDelaySfx` (Single)
    #[serde(default)]
    pub refire_delay_sfx: f32,
    /// `hasDialogue` (Boolean)
    #[serde(default)]
    pub has_dialogue: bool,
    /// `refireDelayDialogue` (Single)
    #[serde(default)]
    pub refire_delay_dialogue: f32,
    /// `timeout` (Single)
    #[serde(default)]
    pub timeout: f32,
    /// `queuingBehaviour` (Class)
    #[serde(default)]
    pub queuing_behaviour: Option<Handle<QueueingBehaviour>>,
    /// `rules` (StrongPointer (array))
    #[serde(default)]
    pub rules: Vec<Handle<CockpitRuleBase>>,
}

impl Pooled for CockpitResponseVariation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_response_variation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_response_variation }
}

impl<'a> Extract<'a> for CockpitResponseVariation {
    const TYPE_NAME: &'static str = "CockpitResponseVariation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            communication_name: inst.get("communicationName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            initial_delay: inst.get_f32("initialDelay").unwrap_or_default(),
            has_sfx: inst.get_bool("hasSfx").unwrap_or_default(),
            refire_delay_sfx: inst.get_f32("refireDelaySfx").unwrap_or_default(),
            has_dialogue: inst.get_bool("hasDialogue").unwrap_or_default(),
            refire_delay_dialogue: inst.get_f32("refireDelayDialogue").unwrap_or_default(),
            timeout: inst.get_f32("timeout").unwrap_or_default(),
            queuing_behaviour: match inst.get("queuingBehaviour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QueueingBehaviour>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QueueingBehaviour>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CockpitRuleBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CockpitRuleBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitResponse`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CockpitResponse {
    /// `concept` (String)
    #[serde(default)]
    pub concept: String,
    /// `canPlayWhenLanded` (Boolean)
    #[serde(default)]
    pub can_play_when_landed: bool,
    /// `canPlayWhenRacing` (Boolean)
    #[serde(default)]
    pub can_play_when_racing: bool,
    /// `variations` (Class (array))
    #[serde(default)]
    pub variations: Vec<Handle<CockpitResponseVariation>>,
}

impl Pooled for CockpitResponse {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_response }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_response }
}

impl<'a> Extract<'a> for CockpitResponse {
    const TYPE_NAME: &'static str = "CockpitResponse";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            concept: inst.get_str("concept").map(String::from).unwrap_or_default(),
            can_play_when_landed: inst.get_bool("canPlayWhenLanded").unwrap_or_default(),
            can_play_when_racing: inst.get_bool("canPlayWhenRacing").unwrap_or_default(),
            variations: inst.get_array("variations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CockpitResponseVariation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CockpitResponseVariation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitResponses`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CockpitResponses {
    /// `responses` (Reference (array))
    #[serde(default)]
    pub responses: Vec<CigGuid>,
}

impl Pooled for CockpitResponses {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_responses }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_responses }
}

impl<'a> Extract<'a> for CockpitResponses {
    const TYPE_NAME: &'static str = "CockpitResponses";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            responses: inst.get_array("responses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitRuleBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CockpitRuleBase {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `priority` (Single)
    #[serde(default)]
    pub priority: f32,
}

impl Pooled for CockpitRuleBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_rule_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_rule_base }
}

impl<'a> Extract<'a> for CockpitRuleBase {
    const TYPE_NAME: &'static str = "CockpitRuleBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            priority: inst.get_f32("priority").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioAllegianceSwitches`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAllegianceSwitches {
    /// `allegianceRTPC` (Class)
    #[serde(default)]
    pub allegiance_rtpc: Option<Handle<AudioRtpc>>,
    /// `neutralRtpcValue` (Single)
    #[serde(default)]
    pub neutral_rtpc_value: f32,
    /// `friendlyRtpcValue` (Single)
    #[serde(default)]
    pub friendly_rtpc_value: f32,
    /// `hostileRtpcValue` (Single)
    #[serde(default)]
    pub hostile_rtpc_value: f32,
}

impl Pooled for AudioAllegianceSwitches {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_allegiance_switches }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_allegiance_switches }
}

impl<'a> Extract<'a> for AudioAllegianceSwitches {
    const TYPE_NAME: &'static str = "AudioAllegianceSwitches";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            allegiance_rtpc: match inst.get("allegianceRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            neutral_rtpc_value: inst.get_f32("neutralRtpcValue").unwrap_or_default(),
            friendly_rtpc_value: inst.get_f32("friendlyRtpcValue").unwrap_or_default(),
            hostile_rtpc_value: inst.get_f32("hostileRtpcValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioEnvironmentFeedbackZoneProcess`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEnvironmentFeedbackZoneProcess {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `radius` (Class)
    #[serde(default)]
    pub radius: Option<Handle<BezierCurve>>,
    /// `maxRadius` (Single)
    #[serde(default)]
    pub max_radius: f32,
    /// `environmentValue` (Class)
    #[serde(default)]
    pub environment_value: Option<Handle<BezierCurve>>,
    /// `maxEnvironmentValue` (Single)
    #[serde(default)]
    pub max_environment_value: f32,
    /// `lifeTime` (Single)
    #[serde(default)]
    pub life_time: f32,
    /// `effectRtpc` (Class)
    #[serde(default)]
    pub effect_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for AudioEnvironmentFeedbackZoneProcess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_environment_feedback_zone_process }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_environment_feedback_zone_process }
}

impl<'a> Extract<'a> for AudioEnvironmentFeedbackZoneProcess {
    const TYPE_NAME: &'static str = "AudioEnvironmentFeedbackZoneProcess";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            radius: match inst.get("radius") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_radius: inst.get_f32("maxRadius").unwrap_or_default(),
            environment_value: match inst.get("environmentValue") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_environment_value: inst.get_f32("maxEnvironmentValue").unwrap_or_default(),
            life_time: inst.get_f32("lifeTime").unwrap_or_default(),
            effect_rtpc: match inst.get("effectRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AudioEnvironmentFeedbackZoneSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEnvironmentFeedbackZoneSetup {
    /// `processes` (Class (array))
    #[serde(default)]
    pub processes: Vec<Handle<AudioEnvironmentFeedbackZoneProcess>>,
}

impl Pooled for AudioEnvironmentFeedbackZoneSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_environment_feedback_zone_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_environment_feedback_zone_setup }
}

impl<'a> Extract<'a> for AudioEnvironmentFeedbackZoneSetup {
    const TYPE_NAME: &'static str = "AudioEnvironmentFeedbackZoneSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            processes: inst.get_array("processes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioEnvironmentFeedbackZoneProcess>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioEnvironmentFeedbackZoneProcess>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioEnvironmentMovementRtpcBehavior`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEnvironmentMovementRtpcBehavior {
    /// `rtpcs` (Class (array))
    #[serde(default)]
    pub rtpcs: Vec<Handle<AudioRtpcWithBehaviour>>,
}

impl Pooled for AudioEnvironmentMovementRtpcBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_environment_movement_rtpc_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_environment_movement_rtpc_behavior }
}

impl<'a> Extract<'a> for AudioEnvironmentMovementRtpcBehavior {
    const TYPE_NAME: &'static str = "AudioEnvironmentMovementRtpcBehavior";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rtpcs: inst.get_array("rtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpcWithBehaviour>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioRtpcWithBehaviour>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioEnvironmentFeedbackTagAndEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEnvironmentFeedbackTagAndEvent {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `loopStart` (Class)
    #[serde(default)]
    pub loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `loopStop` (Class)
    #[serde(default)]
    pub loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `movementRtpcs` (Class)
    #[serde(default)]
    pub movement_rtpcs: Option<Handle<AudioEnvironmentMovementRtpcBehavior>>,
}

impl Pooled for AudioEnvironmentFeedbackTagAndEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_environment_feedback_tag_and_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_environment_feedback_tag_and_event }
}

impl<'a> Extract<'a> for AudioEnvironmentFeedbackTagAndEvent {
    const TYPE_NAME: &'static str = "AudioEnvironmentFeedbackTagAndEvent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
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
            movement_rtpcs: match inst.get("movementRtpcs") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioEnvironmentMovementRtpcBehavior>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioEnvironmentMovementRtpcBehavior>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AudioEnvironmentFeedbackPointDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioEnvironmentFeedbackPointDef {
    /// `tagAndEvents` (Class (array))
    #[serde(default)]
    pub tag_and_events: Vec<Handle<AudioEnvironmentFeedbackTagAndEvent>>,
}

impl Pooled for AudioEnvironmentFeedbackPointDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_environment_feedback_point_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_environment_feedback_point_def }
}

impl<'a> Extract<'a> for AudioEnvironmentFeedbackPointDef {
    const TYPE_NAME: &'static str = "AudioEnvironmentFeedbackPointDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag_and_events: inst.get_array("tagAndEvents")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioEnvironmentFeedbackTagAndEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioEnvironmentFeedbackTagAndEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioHitListenerTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioHitListenerTrigger {
    /// `trigger` (Class)
    #[serde(default)]
    pub trigger: Option<Handle<GlobalResourceAudio>>,
    /// `hitByPlayerTrigger` (Class)
    #[serde(default)]
    pub hit_by_player_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rtpcTimeSinceLastTrigger` (Class)
    #[serde(default)]
    pub rtpc_time_since_last_trigger: Option<Handle<AudioRtpc>>,
    /// `rtpcDamage` (Class)
    #[serde(default)]
    pub rtpc_damage: Option<Handle<AudioRtpc>>,
    /// `rtpcRatioAfterHit` (Class)
    #[serde(default)]
    pub rtpc_ratio_after_hit: Option<Handle<AudioRtpc>>,
    /// `oneShotMinPlayTime` (Single)
    #[serde(default)]
    pub one_shot_min_play_time: f32,
    /// `cooldown` (Single)
    #[serde(default)]
    pub cooldown: f32,
    /// `oneshotTag` (Reference)
    #[serde(default)]
    pub oneshot_tag: Option<CigGuid>,
    /// `oneshotTagPlayer` (Reference)
    #[serde(default)]
    pub oneshot_tag_player: Option<CigGuid>,
}

impl Pooled for AudioHitListenerTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_hit_listener_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_hit_listener_trigger }
}

impl<'a> Extract<'a> for AudioHitListenerTrigger {
    const TYPE_NAME: &'static str = "AudioHitListenerTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger: match inst.get("trigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hit_by_player_trigger: match inst.get("hitByPlayerTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_time_since_last_trigger: match inst.get("rtpcTimeSinceLastTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_damage: match inst.get("rtpcDamage") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rtpc_ratio_after_hit: match inst.get("rtpcRatioAfterHit") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            one_shot_min_play_time: inst.get_f32("oneShotMinPlayTime").unwrap_or_default(),
            cooldown: inst.get_f32("cooldown").unwrap_or_default(),
            oneshot_tag: inst.get("oneshotTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            oneshot_tag_player: inst.get("oneshotTagPlayer").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AudioHitTypeDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioHitTypeDefinition {
    /// `triggerImpactHealth` (Class)
    #[serde(default)]
    pub trigger_impact_health: Option<Handle<AudioHitListenerTrigger>>,
    /// `triggerImpactShield` (Class)
    #[serde(default)]
    pub trigger_impact_shield: Option<Handle<AudioHitListenerTrigger>>,
    /// `triggerShieldFail` (Class)
    #[serde(default)]
    pub trigger_shield_fail: Option<Handle<AudioHitListenerTrigger>>,
}

impl Pooled for AudioHitTypeDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_hit_type_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_hit_type_definition }
}

impl<'a> Extract<'a> for AudioHitTypeDefinition {
    const TYPE_NAME: &'static str = "AudioHitTypeDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_impact_health: match inst.get("triggerImpactHealth") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitListenerTrigger>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitListenerTrigger>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            trigger_impact_shield: match inst.get("triggerImpactShield") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitListenerTrigger>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitListenerTrigger>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            trigger_shield_fail: match inst.get("triggerShieldFail") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitListenerTrigger>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitListenerTrigger>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AudioHitListenerDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioHitListenerDefinition {
    /// `melee` (StrongPointer)
    #[serde(default)]
    pub melee: Option<Handle<AudioHitTypeDefinition>>,
    /// `collision` (StrongPointer)
    #[serde(default)]
    pub collision: Option<Handle<AudioHitTypeDefinition>>,
    /// `crash` (StrongPointer)
    #[serde(default)]
    pub crash: Option<Handle<AudioHitTypeDefinition>>,
    /// `frag` (StrongPointer)
    #[serde(default)]
    pub frag: Option<Handle<AudioHitTypeDefinition>>,
    /// `explosion` (StrongPointer)
    #[serde(default)]
    pub explosion: Option<Handle<AudioHitTypeDefinition>>,
    /// `takedown` (StrongPointer)
    #[serde(default)]
    pub takedown: Option<Handle<AudioHitTypeDefinition>>,
    /// `punish` (StrongPointer)
    #[serde(default)]
    pub punish: Option<Handle<AudioHitTypeDefinition>>,
    /// `normal` (StrongPointer)
    #[serde(default)]
    pub normal: Option<Handle<AudioHitTypeDefinition>>,
    /// `fire` (StrongPointer)
    #[serde(default)]
    pub fire: Option<Handle<AudioHitTypeDefinition>>,
    /// `bullet` (StrongPointer)
    #[serde(default)]
    pub bullet: Option<Handle<AudioHitTypeDefinition>>,
    /// `vehicleDestruction` (StrongPointer)
    #[serde(default)]
    pub vehicle_destruction: Option<Handle<AudioHitTypeDefinition>>,
    /// `eventDamage` (StrongPointer)
    #[serde(default)]
    pub event_damage: Option<Handle<AudioHitTypeDefinition>>,
    /// `bleedOut` (StrongPointer)
    #[serde(default)]
    pub bleed_out: Option<Handle<AudioHitTypeDefinition>>,
    /// `electricArc` (StrongPointer)
    #[serde(default)]
    pub electric_arc: Option<Handle<AudioHitTypeDefinition>>,
    /// `repair` (StrongPointer)
    #[serde(default)]
    pub repair: Option<Handle<AudioHitTypeDefinition>>,
    /// `suffocate` (StrongPointer)
    #[serde(default)]
    pub suffocate: Option<Handle<AudioHitTypeDefinition>>,
    /// `suicide` (StrongPointer)
    #[serde(default)]
    pub suicide: Option<Handle<AudioHitTypeDefinition>>,
    /// `selfDestruct` (StrongPointer)
    #[serde(default)]
    pub self_destruct: Option<Handle<AudioHitTypeDefinition>>,
    /// `boundaryViolation` (StrongPointer)
    #[serde(default)]
    pub boundary_violation: Option<Handle<AudioHitTypeDefinition>>,
    /// `drown` (StrongPointer)
    #[serde(default)]
    pub drown: Option<Handle<AudioHitTypeDefinition>>,
    /// `damageOverTime` (StrongPointer)
    #[serde(default)]
    pub damage_over_time: Option<Handle<AudioHitTypeDefinition>>,
    /// `hazard` (StrongPointer)
    #[serde(default)]
    pub hazard: Option<Handle<AudioHitTypeDefinition>>,
}

impl Pooled for AudioHitListenerDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_hit_listener_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_hit_listener_definition }
}

impl<'a> Extract<'a> for AudioHitListenerDefinition {
    const TYPE_NAME: &'static str = "AudioHitListenerDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            melee: match inst.get("melee") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            collision: match inst.get("collision") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            crash: match inst.get("crash") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            frag: match inst.get("frag") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            explosion: match inst.get("explosion") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            takedown: match inst.get("takedown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            punish: match inst.get("punish") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            normal: match inst.get("normal") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fire: match inst.get("fire") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bullet: match inst.get("bullet") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vehicle_destruction: match inst.get("vehicleDestruction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            event_damage: match inst.get("eventDamage") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bleed_out: match inst.get("bleedOut") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            electric_arc: match inst.get("electricArc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            repair: match inst.get("repair") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            suffocate: match inst.get("suffocate") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            suicide: match inst.get("suicide") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            self_destruct: match inst.get("selfDestruct") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            boundary_violation: match inst.get("boundaryViolation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drown: match inst.get("drown") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            damage_over_time: match inst.get("damageOverTime") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hazard: match inst.get("hazard") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioHitTypeDefinition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioHitTypeDefinition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WheelAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelAudioParams {
    /// `loopStart` (Class)
    #[serde(default)]
    pub loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `loopStop` (Class)
    #[serde(default)]
    pub loop_stop: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for WheelAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.wheel_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.wheel_audio_params }
}

impl<'a> Extract<'a> for WheelAudioParams {
    const TYPE_NAME: &'static str = "WheelAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
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

/// DCB type: `WheelAudioSurfaceMapping`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelAudioSurfaceMapping {
    /// `surfaceName` (String)
    #[serde(default)]
    pub surface_name: String,
    /// `audio` (Class)
    #[serde(default)]
    pub audio: Option<Handle<WheelAudioParams>>,
}

impl Pooled for WheelAudioSurfaceMapping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.wheel_audio_surface_mapping }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.wheel_audio_surface_mapping }
}

impl<'a> Extract<'a> for WheelAudioSurfaceMapping {
    const TYPE_NAME: &'static str = "WheelAudioSurfaceMapping";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            surface_name: inst.get_str("surfaceName").map(String::from).unwrap_or_default(),
            audio: match inst.get("audio") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WheelAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WheelAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WheelAudioSurfaceMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelAudioSurfaceMap {
    /// `generic` (Class)
    #[serde(default)]
    pub generic: Option<Handle<WheelAudioParams>>,
    /// `default` (Class)
    #[serde(default)]
    pub default: Option<Handle<WheelAudioParams>>,
    /// `invalidSurface` (Class)
    #[serde(default)]
    pub invalid_surface: Option<Handle<WheelAudioParams>>,
    /// `surfaceMappings` (Class (array))
    #[serde(default)]
    pub surface_mappings: Vec<Handle<WheelAudioSurfaceMapping>>,
}

impl Pooled for WheelAudioSurfaceMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.wheel_audio_surface_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.wheel_audio_surface_map }
}

impl<'a> Extract<'a> for WheelAudioSurfaceMap {
    const TYPE_NAME: &'static str = "WheelAudioSurfaceMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            generic: match inst.get("generic") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WheelAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WheelAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default: match inst.get("default") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WheelAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WheelAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invalid_surface: match inst.get("invalidSurface") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WheelAudioParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<WheelAudioParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            surface_mappings: inst.get_array("surfaceMappings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<WheelAudioSurfaceMapping>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<WheelAudioSurfaceMapping>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralPlanetAudioTagAndEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralPlanetAudioTagAndEvent {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `movementRtpc` (Class)
    #[serde(default)]
    pub movement_rtpc: Option<Handle<AudioRtpc>>,
    /// `loopStart` (Class)
    #[serde(default)]
    pub loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `loopStop` (Class)
    #[serde(default)]
    pub loop_stop: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for ProceduralPlanetAudioTagAndEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.procedural_planet_audio_tag_and_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.procedural_planet_audio_tag_and_event }
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
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `rtpc` (Class)
    #[serde(default)]
    pub rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for ProceduralPlanetAudioTagAndRtpc {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.procedural_planet_audio_tag_and_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.procedural_planet_audio_tag_and_rtpc }
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
    /// `tagAndEvents` (Class (array))
    #[serde(default)]
    pub tag_and_events: Vec<Handle<ProceduralPlanetAudioTagAndEvent>>,
}

impl Pooled for ProceduralPlanetAudioTagAndEventsDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.procedural_planet_audio_tag_and_events_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.procedural_planet_audio_tag_and_events_def }
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
    /// `countRtpcs` (Class (array))
    #[serde(default)]
    pub count_rtpcs: Vec<Handle<AudioRtpc>>,
}

impl Pooled for ProceduralPlanetAudioAlgorithm {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.procedural_planet_audio_algorithm }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.procedural_planet_audio_algorithm }
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
    /// `byActor` (Boolean)
    #[serde(default)]
    pub by_actor: bool,
    /// `byWheeledVehicle` (Boolean)
    #[serde(default)]
    pub by_wheeled_vehicle: bool,
    /// `bySpaceship` (Boolean)
    #[serde(default)]
    pub by_spaceship: bool,
    /// `byOther` (Boolean)
    #[serde(default)]
    pub by_other: bool,
    /// `idleTimeThreshold` (Single)
    #[serde(default)]
    pub idle_time_threshold: f32,
    /// `enterSound` (Class)
    #[serde(default)]
    pub enter_sound: Option<Handle<GlobalResourceAudio>>,
    /// `disturbedLoopStart` (Class)
    #[serde(default)]
    pub disturbed_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `disturbedLoopStop` (Class)
    #[serde(default)]
    pub disturbed_loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `redisturbSound` (Class)
    #[serde(default)]
    pub redisturb_sound: Option<Handle<GlobalResourceAudio>>,
    /// `exitSound` (Class)
    #[serde(default)]
    pub exit_sound: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for ProceduralPlanetAudioDisturbanceDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.procedural_planet_audio_disturbance_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.procedural_planet_audio_disturbance_def }
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
    /// `disturbances` (Class (array))
    #[serde(default)]
    pub disturbances: Vec<Handle<ProceduralPlanetAudioDisturbanceDef>>,
}

impl Pooled for ProceduralPlanetAudioDisturbanceList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.procedural_planet_audio_disturbance_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.procedural_planet_audio_disturbance_list }
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
    /// `audioName` (String)
    #[serde(default)]
    pub audio_name: String,
    /// `listenerMovementThreshold` (Single)
    #[serde(default)]
    pub listener_movement_threshold: f32,
    /// `algorithm` (StrongPointer)
    #[serde(default)]
    pub algorithm: Option<Handle<ProceduralPlanetAudioAlgorithm>>,
    /// `disturbances` (WeakPointer (array))
    #[serde(default)]
    pub disturbances: Vec<Handle<ProceduralPlanetAudioDisturbanceList>>,
}

impl Pooled for ProceduralPlanetAudioEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.procedural_planet_audio_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.procedural_planet_audio_entry }
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
    /// `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<ProceduralPlanetAudioEntry>>,
    /// `pressureRtpc` (Class)
    #[serde(default)]
    pub pressure_rtpc: Option<Handle<AudioRtpc>>,
    /// `temperatureRtpc` (Class)
    #[serde(default)]
    pub temperature_rtpc: Option<Handle<AudioRtpc>>,
    /// `humidityRtpc` (Class)
    #[serde(default)]
    pub humidity_rtpc: Option<Handle<AudioRtpc>>,
    /// `breathabilityRtpc` (Class)
    #[serde(default)]
    pub breathability_rtpc: Option<Handle<AudioRtpc>>,
    /// `firstRoomIsPlanetRoomRtpc` (Class)
    #[serde(default)]
    pub first_room_is_planet_room_rtpc: Option<Handle<AudioRtpc>>,
    /// `effectTagMovementRtpcs` (Class (array))
    #[serde(default)]
    pub effect_tag_movement_rtpcs: Vec<Handle<ProceduralPlanetAudioTagAndRtpc>>,
    /// `disturbanceLists` (Class (array))
    #[serde(default)]
    pub disturbance_lists: Vec<Handle<ProceduralPlanetAudioDisturbanceList>>,
    /// `disturbanceVelocityRtpc` (Class)
    #[serde(default)]
    pub disturbance_velocity_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for ProceduralPlanetAudioData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.procedural_planet_audio_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.procedural_planet_audio_data }
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
    /// `riverLoopStart` (Class)
    #[serde(default)]
    pub river_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `riverFlowSpeedRtpc` (Class)
    #[serde(default)]
    pub river_flow_speed_rtpc: Option<Handle<AudioRtpc>>,
    /// `riverWidthRtpc` (Class)
    #[serde(default)]
    pub river_width_rtpc: Option<Handle<AudioRtpc>>,
    /// `distanceFromRiverEdgeRtpc` (Class)
    #[serde(default)]
    pub distance_from_river_edge_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for ProceduralPlanetAudioRiverData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.procedural_planet_audio_river_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.procedural_planet_audio_river_data }
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

/// DCB type: `PlanetOceanAudioCheckpoint`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetOceanAudioCheckpoint {
    /// `beamCount` (Int32)
    #[serde(default)]
    pub beam_count: i32,
    /// `range` (Single)
    #[serde(default)]
    pub range: f32,
    /// `useDepthAssignment` (Boolean)
    #[serde(default)]
    pub use_depth_assignment: bool,
}

impl Pooled for PlanetOceanAudioCheckpoint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.planet_ocean_audio_checkpoint }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.planet_ocean_audio_checkpoint }
}

impl<'a> Extract<'a> for PlanetOceanAudioCheckpoint {
    const TYPE_NAME: &'static str = "PlanetOceanAudioCheckpoint";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            beam_count: inst.get_i32("beamCount").unwrap_or_default(),
            range: inst.get_f32("range").unwrap_or_default(),
            use_depth_assignment: inst.get_bool("useDepthAssignment").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetOceanDepthAssignment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetOceanDepthAssignment {
    /// `waterDepth` (Single)
    #[serde(default)]
    pub water_depth: f32,
    /// `assignmentStartTrigger` (Class)
    #[serde(default)]
    pub assignment_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `assignmentStopTrigger` (Class)
    #[serde(default)]
    pub assignment_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `recalculationDistance` (Single)
    #[serde(default)]
    pub recalculation_distance: f32,
}

impl Pooled for PlanetOceanDepthAssignment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.planet_ocean_depth_assignment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.planet_ocean_depth_assignment }
}

impl<'a> Extract<'a> for PlanetOceanDepthAssignment {
    const TYPE_NAME: &'static str = "PlanetOceanDepthAssignment";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            water_depth: inst.get_f32("waterDepth").unwrap_or_default(),
            assignment_start_trigger: match inst.get("assignmentStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            assignment_stop_trigger: match inst.get("assignmentStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            recalculation_distance: inst.get_f32("recalculationDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlanetOceanAudioData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetOceanAudioData {
    /// `checkpoints` (Class (array))
    #[serde(default)]
    pub checkpoints: Vec<Handle<PlanetOceanAudioCheckpoint>>,
    /// `assignments` (Class (array))
    #[serde(default)]
    pub assignments: Vec<Handle<PlanetOceanDepthAssignment>>,
    /// `waterStartTrigger` (Class)
    #[serde(default)]
    pub water_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `waterStopTrigger` (Class)
    #[serde(default)]
    pub water_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `oceanOffsetRtpc` (Class)
    #[serde(default)]
    pub ocean_offset_rtpc: Option<Handle<AudioRtpc>>,
    /// `windRTPC` (Class)
    #[serde(default)]
    pub wind_rtpc: Option<Handle<AudioRtpc>>,
    /// `waveHeightRTPC` (Class)
    #[serde(default)]
    pub wave_height_rtpc: Option<Handle<AudioRtpc>>,
    /// `atmosphereTag` (Reference)
    #[serde(default)]
    pub atmosphere_tag: Option<CigGuid>,
    /// `terrainChecksPerFrame` (Int32)
    #[serde(default)]
    pub terrain_checks_per_frame: i32,
    /// `checkOnListenerPosition` (Boolean)
    #[serde(default)]
    pub check_on_listener_position: bool,
    /// `listenerPositionUsesAssignment` (Boolean)
    #[serde(default)]
    pub listener_position_uses_assignment: bool,
}

impl Pooled for PlanetOceanAudioData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.planet_ocean_audio_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.planet_ocean_audio_data }
}

impl<'a> Extract<'a> for PlanetOceanAudioData {
    const TYPE_NAME: &'static str = "PlanetOceanAudioData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            checkpoints: inst.get_array("checkpoints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlanetOceanAudioCheckpoint>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlanetOceanAudioCheckpoint>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            assignments: inst.get_array("assignments")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlanetOceanDepthAssignment>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PlanetOceanDepthAssignment>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            water_start_trigger: match inst.get("waterStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            water_stop_trigger: match inst.get("waterStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ocean_offset_rtpc: match inst.get("oceanOffsetRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wind_rtpc: match inst.get("windRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wave_height_rtpc: match inst.get("waveHeightRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            atmosphere_tag: inst.get("atmosphereTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            terrain_checks_per_frame: inst.get_i32("terrainChecksPerFrame").unwrap_or_default(),
            check_on_listener_position: inst.get_bool("checkOnListenerPosition").unwrap_or_default(),
            listener_position_uses_assignment: inst.get_bool("listenerPositionUsesAssignment").unwrap_or_default(),
        }
    }
}

/// DCB type: `VibrationAudioEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationAudioEntry {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `vibrationRtpcs` (Class (array))
    #[serde(default)]
    pub vibration_rtpcs: Vec<Handle<AudioRtpcWithBehaviour>>,
    /// `loopStart` (Class)
    #[serde(default)]
    pub loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `loopStop` (Class)
    #[serde(default)]
    pub loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `jostleEvent` (Class)
    #[serde(default)]
    pub jostle_event: Option<Handle<GlobalResourceAudio>>,
    /// `jostleCooldown` (Single)
    #[serde(default)]
    pub jostle_cooldown: f32,
    /// `jostleThreshold` (Single)
    #[serde(default)]
    pub jostle_threshold: f32,
    /// `jostleRtpc` (Class)
    #[serde(default)]
    pub jostle_rtpc: Option<Handle<AudioRtpc>>,
    /// `usedVibrationTypes` (EnumChoice (array))
    #[serde(default)]
    pub used_vibration_types: Vec<String>,
    /// `vibrationInputModifiers` (Single)
    #[serde(default)]
    pub vibration_input_modifiers: f32,
    /// `calculationType` (EnumChoice)
    #[serde(default)]
    pub calculation_type: String,
}

impl Pooled for VibrationAudioEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.vibration_audio_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.vibration_audio_entry }
}

impl<'a> Extract<'a> for VibrationAudioEntry {
    const TYPE_NAME: &'static str = "VibrationAudioEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            vibration_rtpcs: inst.get_array("vibrationRtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpcWithBehaviour>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioRtpcWithBehaviour>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
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
            jostle_event: match inst.get("jostleEvent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            jostle_cooldown: inst.get_f32("jostleCooldown").unwrap_or_default(),
            jostle_threshold: inst.get_f32("jostleThreshold").unwrap_or_default(),
            jostle_rtpc: match inst.get("jostleRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            used_vibration_types: inst.get_array("usedVibrationTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            vibration_input_modifiers: inst.get_f32("vibrationInputModifiers").unwrap_or_default(),
            calculation_type: inst.get_str("calculationType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VibrationAudioPointDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationAudioPointDef {
    /// `vibrationAudioEntries` (Class (array))
    #[serde(default)]
    pub vibration_audio_entries: Vec<Handle<VibrationAudioEntry>>,
    /// `customFalloff` (Single)
    #[serde(default)]
    pub custom_falloff: f32,
}

impl Pooled for VibrationAudioPointDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.vibration_audio_point_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.vibration_audio_point_def }
}

impl<'a> Extract<'a> for VibrationAudioPointDef {
    const TYPE_NAME: &'static str = "VibrationAudioPointDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vibration_audio_entries: inst.get_array("vibrationAudioEntries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<VibrationAudioEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<VibrationAudioEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            custom_falloff: inst.get_f32("customFalloff").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioSignalList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSignalList {
    /// `Signals` (Class (array))
    #[serde(default)]
    pub signals: Vec<Handle<AudioSignal>>,
}

impl Pooled for AudioSignalList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_signal_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_signal_list }
}

impl<'a> Extract<'a> for AudioSignalList {
    const TYPE_NAME: &'static str = "AudioSignalList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            signals: inst.get_array("Signals")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioSignal>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioSignal>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TagToAudioRtpcList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagToAudioRtpcList {
    /// `TagsToRtpcs` (Class (array))
    #[serde(default)]
    pub tags_to_rtpcs: Vec<Handle<TagToAudioRtpc>>,
}

impl Pooled for TagToAudioRtpcList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.tag_to_audio_rtpc_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.tag_to_audio_rtpc_list }
}

impl<'a> Extract<'a> for TagToAudioRtpcList {
    const TYPE_NAME: &'static str = "TagToAudioRtpcList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags_to_rtpcs: inst.get_array("TagsToRtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagToAudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TagToAudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioSignalRtpc`
/// Inherits from: `AudioRtpcWithDefault`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSignalRtpc {
    /// `rtpc` (String)
    #[serde(default)]
    pub rtpc: String,
    /// `defaultValue` (Single)
    #[serde(default)]
    pub default_value: f32,
    /// `global` (Boolean)
    #[serde(default)]
    pub global: bool,
}

impl Pooled for AudioSignalRtpc {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_signal_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_signal_rtpc }
}

impl<'a> Extract<'a> for AudioSignalRtpc {
    const TYPE_NAME: &'static str = "AudioSignalRtpc";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rtpc: inst.get_str("rtpc").map(String::from).unwrap_or_default(),
            default_value: inst.get_f32("defaultValue").unwrap_or_default(),
            global: inst.get_bool("global").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioSignal`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSignal {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `audioTriggers` (Class (array))
    #[serde(default)]
    pub audio_triggers: Vec<Handle<GlobalResourceAudio>>,
    /// `switches` (Class (array))
    #[serde(default)]
    pub switches: Vec<Handle<AudioSwitch>>,
    /// `rtpcs` (Class (array))
    #[serde(default)]
    pub rtpcs: Vec<Handle<AudioSignalRtpc>>,
}

impl Pooled for AudioSignal {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_signal }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_signal }
}

impl<'a> Extract<'a> for AudioSignal {
    const TYPE_NAME: &'static str = "AudioSignal";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            audio_triggers: inst.get_array("audioTriggers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            switches: inst.get_array("switches")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioSwitch>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            rtpcs: inst.get_array("rtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioSignalRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioSignalRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioRtpcWithDefault`
/// Inherits from: `AudioRtpc`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRtpcWithDefault {
    /// `rtpc` (String)
    #[serde(default)]
    pub rtpc: String,
    /// `defaultValue` (Single)
    #[serde(default)]
    pub default_value: f32,
}

impl Pooled for AudioRtpcWithDefault {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_rtpc_with_default }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_rtpc_with_default }
}

impl<'a> Extract<'a> for AudioRtpcWithDefault {
    const TYPE_NAME: &'static str = "AudioRtpcWithDefault";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rtpc: inst.get_str("rtpc").map(String::from).unwrap_or_default(),
            default_value: inst.get_f32("defaultValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioSwitch`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSwitch {
    /// `switch` (String)
    #[serde(default)]
    pub switch: String,
}

impl Pooled for AudioSwitch {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_switch }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_switch }
}

impl<'a> Extract<'a> for AudioSwitch {
    const TYPE_NAME: &'static str = "AudioSwitch";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            switch: inst.get_str("switch").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SRtpcBehaviour`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRtpcBehaviour {
}

impl Pooled for SRtpcBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.srtpc_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.srtpc_behaviour }
}

impl<'a> Extract<'a> for SRtpcBehaviour {
    const TYPE_NAME: &'static str = "SRtpcBehaviour";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AudioRtpcWithBehaviour`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRtpcWithBehaviour {
    /// `rtpc` (Class)
    #[serde(default)]
    pub rtpc: Option<Handle<AudioRtpc>>,
    /// `behavior` (StrongPointer)
    #[serde(default)]
    pub behavior: Option<Handle<SRtpcBehaviour>>,
}

impl Pooled for AudioRtpcWithBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_rtpc_with_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_rtpc_with_behaviour }
}

impl<'a> Extract<'a> for AudioRtpcWithBehaviour {
    const TYPE_NAME: &'static str = "AudioRtpcWithBehaviour";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rtpc: match inst.get("rtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            behavior: match inst.get("behavior") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRtpcBehaviour>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRtpcBehaviour>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TagToAudioRtpc`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagToAudioRtpc {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `rtpcName` (String)
    #[serde(default)]
    pub rtpc_name: String,
    /// `rtpcValue` (Single)
    #[serde(default)]
    pub rtpc_value: f32,
}

impl Pooled for TagToAudioRtpc {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.tag_to_audio_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.tag_to_audio_rtpc }
}

impl<'a> Extract<'a> for TagToAudioRtpc {
    const TYPE_NAME: &'static str = "TagToAudioRtpc";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rtpc_name: inst.get_str("rtpcName").map(String::from).unwrap_or_default(),
            rtpc_value: inst.get_f32("rtpcValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `GPUParticleAudioList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUParticleAudioList {
    /// `particleAudioList` (Reference (array))
    #[serde(default)]
    pub particle_audio_list: Vec<CigGuid>,
}

impl Pooled for GPUParticleAudioList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.gpuparticle_audio_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.gpuparticle_audio_list }
}

impl<'a> Extract<'a> for GPUParticleAudioList {
    const TYPE_NAME: &'static str = "GPUParticleAudioList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            particle_audio_list: inst.get_array("particleAudioList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommsAudioEffect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsAudioEffect {
    /// `busName` (String)
    #[serde(default)]
    pub bus_name: String,
    /// `locationId` (EnumChoice)
    #[serde(default)]
    pub location_id: String,
}

impl Pooled for CommsAudioEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.comms_audio_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.comms_audio_effect }
}

impl<'a> Extract<'a> for CommsAudioEffect {
    const TYPE_NAME: &'static str = "CommsAudioEffect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            bus_name: inst.get_str("busName").map(String::from).unwrap_or_default(),
            location_id: inst.get_str("locationId").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ShipComputerDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipComputerDefinition {
    /// `communicationConfig` (Reference)
    #[serde(default)]
    pub communication_config: Option<CigGuid>,
    /// `preBootUpTime` (Single)
    #[serde(default)]
    pub pre_boot_up_time: f32,
    /// `bootUpTime` (Single)
    #[serde(default)]
    pub boot_up_time: f32,
    /// `timeSinceLastHitMarkerSfx` (Class)
    #[serde(default)]
    pub time_since_last_hit_marker_sfx: Option<Handle<AudioRtpc>>,
}

impl Pooled for ShipComputerDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.ship_computer_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.ship_computer_definition }
}

impl<'a> Extract<'a> for ShipComputerDefinition {
    const TYPE_NAME: &'static str = "ShipComputerDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            communication_config: inst.get("communicationConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pre_boot_up_time: inst.get_f32("preBootUpTime").unwrap_or_default(),
            boot_up_time: inst.get_f32("bootUpTime").unwrap_or_default(),
            time_since_last_hit_marker_sfx: match inst.get("timeSinceLastHitMarkerSfx") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioModifier {
}

impl Pooled for SSCSignatureSystemAudioModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.sscsignature_system_audio_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.sscsignature_system_audio_modifier }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioModifier {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioModifier";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioSubRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioSubRule {
}

impl Pooled for SSCSignatureSystemAudioSubRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.sscsignature_system_audio_sub_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.sscsignature_system_audio_sub_rule }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioSubRule {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioSubRule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioRule {
    /// `subRules` (StrongPointer (array))
    #[serde(default)]
    pub sub_rules: Vec<Handle<SSCSignatureSystemAudioSubRule>>,
    /// `modifier` (StrongPointer)
    #[serde(default)]
    pub modifier: Option<Handle<SSCSignatureSystemAudioModifier>>,
}

impl Pooled for SSCSignatureSystemAudioRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.sscsignature_system_audio_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.sscsignature_system_audio_rule }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioRule {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioRule";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sub_rules: inst.get_array("subRules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SSCSignatureSystemAudioSubRule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SSCSignatureSystemAudioSubRule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            modifier: match inst.get("modifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCSignatureSystemAudioModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCSignatureSystemAudioModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SSCSignatureSystemAudioRuleset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSCSignatureSystemAudioRuleset {
    /// `rules` (StrongPointer (array))
    #[serde(default)]
    pub rules: Vec<Handle<SSCSignatureSystemAudioRule>>,
}

impl Pooled for SSCSignatureSystemAudioRuleset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.sscsignature_system_audio_ruleset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.sscsignature_system_audio_ruleset }
}

impl<'a> Extract<'a> for SSCSignatureSystemAudioRuleset {
    const TYPE_NAME: &'static str = "SSCSignatureSystemAudioRuleset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SSCSignatureSystemAudioRule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SSCSignatureSystemAudioRule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ShipComputerPresetList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipComputerPresetList {
    /// `presets` (Reference (array))
    #[serde(default)]
    pub presets: Vec<CigGuid>,
}

impl Pooled for ShipComputerPresetList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.ship_computer_preset_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.ship_computer_preset_list }
}

impl<'a> Extract<'a> for ShipComputerPresetList {
    const TYPE_NAME: &'static str = "ShipComputerPresetList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            presets: inst.get_array("presets")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ShipComputerPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipComputerPreset {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `base` (Reference)
    #[serde(default)]
    pub base: Option<CigGuid>,
    /// `displayText` (Locale)
    #[serde(default)]
    pub display_text: String,
    /// `responses` (Reference (array))
    #[serde(default)]
    pub responses: Vec<CigGuid>,
}

impl Pooled for ShipComputerPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.ship_computer_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.ship_computer_preset }
}

impl<'a> Extract<'a> for ShipComputerPreset {
    const TYPE_NAME: &'static str = "ShipComputerPreset";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            base: inst.get("base").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_text: inst.get_str("displayText").map(String::from).unwrap_or_default(),
            responses: inst.get_array("responses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TransitStationAnnouncements`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitStationAnnouncements {
    /// `announcements` (Class (array))
    #[serde(default)]
    pub announcements: Vec<Handle<TransitStationAnnouncement>>,
}

impl Pooled for TransitStationAnnouncements {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.transit_station_announcements }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.transit_station_announcements }
}

impl<'a> Extract<'a> for TransitStationAnnouncements {
    const TYPE_NAME: &'static str = "TransitStationAnnouncements";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            announcements: inst.get_array("announcements")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TransitStationAnnouncement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TransitStationAnnouncement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TransitStationAnnouncement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitStationAnnouncement {
    /// `stationName` (Locale)
    #[serde(default)]
    pub station_name: String,
    /// `preArrival` (Class)
    #[serde(default)]
    pub pre_arrival: Option<Handle<GlobalResourceAudio>>,
    /// `postDeparture` (Class)
    #[serde(default)]
    pub post_departure: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for TransitStationAnnouncement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.transit_station_announcement }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.transit_station_announcement }
}

impl<'a> Extract<'a> for TransitStationAnnouncement {
    const TYPE_NAME: &'static str = "TransitStationAnnouncement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            station_name: inst.get_str("stationName").map(String::from).unwrap_or_default(),
            pre_arrival: match inst.get("preArrival") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            post_departure: match inst.get("postDeparture") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UIAudioEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAudioEvent {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `audioTrigger` (Class)
    #[serde(default)]
    pub audio_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for UIAudioEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.uiaudio_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.uiaudio_event }
}

impl<'a> Extract<'a> for UIAudioEvent {
    const TYPE_NAME: &'static str = "UIAudioEvent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            audio_trigger: match inst.get("audioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UIAudioParameter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAudioParameter {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `rtpc` (Class)
    #[serde(default)]
    pub rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for UIAudioParameter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.uiaudio_parameter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.uiaudio_parameter }
}

impl<'a> Extract<'a> for UIAudioParameter {
    const TYPE_NAME: &'static str = "UIAudioParameter";
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

/// DCB type: `UIDialogueEvent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIDialogueEvent {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `dialogueContext` (Reference)
    #[serde(default)]
    pub dialogue_context: Option<CigGuid>,
}

impl Pooled for UIDialogueEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.uidialogue_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.uidialogue_event }
}

impl<'a> Extract<'a> for UIDialogueEvent {
    const TYPE_NAME: &'static str = "UIDialogueEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            dialogue_context: inst.get("dialogueContext").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `UIAudioDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIAudioDefinition {
    /// `events` (Class (array))
    #[serde(default)]
    pub events: Vec<Handle<UIAudioEvent>>,
    /// `parameters` (Class (array))
    #[serde(default)]
    pub parameters: Vec<Handle<UIAudioParameter>>,
    /// `dialogueEvents` (Class (array))
    #[serde(default)]
    pub dialogue_events: Vec<Handle<UIDialogueEvent>>,
}

impl Pooled for UIAudioDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.uiaudio_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.uiaudio_definition }
}

impl<'a> Extract<'a> for UIAudioDefinition {
    const TYPE_NAME: &'static str = "UIAudioDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            events: inst.get_array("events")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIAudioEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIAudioEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            parameters: inst.get_array("parameters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIAudioParameter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIAudioParameter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            dialogue_events: inst.get_array("dialogueEvents")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UIDialogueEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UIDialogueEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VideoCommsAudioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCommsAudioParams {
    /// `lowTechInterferenceAudioRTPC` (Class)
    #[serde(default)]
    pub low_tech_interference_audio_rtpc: Option<Handle<AudioRtpc>>,
    /// `highTechInterferenceAudioRTPC` (Class)
    #[serde(default)]
    pub high_tech_interference_audio_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for VideoCommsAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.video_comms_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.video_comms_audio_params }
}

impl<'a> Extract<'a> for VideoCommsAudioParams {
    const TYPE_NAME: &'static str = "VideoCommsAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            low_tech_interference_audio_rtpc: match inst.get("lowTechInterferenceAudioRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            high_tech_interference_audio_rtpc: match inst.get("highTechInterferenceAudioRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

