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

/// DCB type: `AudioBreathStyleCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleCondition {
    /// DCB field: `whenBreathParameter` (EnumChoice)
    #[serde(default)]
    pub when_breath_parameter: String,
    /// DCB field: `isAbove` (Single)
    #[serde(default)]
    pub is_above: f32,
    /// DCB field: `andBelow` (Single)
    #[serde(default)]
    pub and_below: f32,
    /// DCB field: `andJumpsUpBy` (Single)
    #[serde(default)]
    pub and_jumps_up_by: f32,
    /// DCB field: `orDropsBy` (Single)
    #[serde(default)]
    pub or_drops_by: f32,
}

impl Pooled for AudioBreathStyleCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_breath_style_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_breath_style_condition }
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
    /// DCB field: `list` (Reference (array))
    #[serde(default)]
    pub list: Vec<CigGuid>,
}

impl Pooled for AudioBreathStyleConditionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_breath_style_condition_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_breath_style_condition_list }
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
///
/// Inherits from: `AudioBreathStyleBaseNode` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleTransitionNode {
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `whenConditionsAreTrueFor` (Single)
    #[serde(default)]
    pub when_conditions_are_true_for: f32,
    /// DCB field: `onNextInhale` (Boolean)
    #[serde(default)]
    pub on_next_inhale: bool,
    /// DCB field: `onNextExhale` (Boolean)
    #[serde(default)]
    pub on_next_exhale: bool,
    /// DCB field: `immediately` (Boolean)
    #[serde(default)]
    pub immediately: bool,
    /// DCB field: `conditions` (Reference (array))
    #[serde(default)]
    pub conditions: Vec<CigGuid>,
    /// DCB field: `style` (WeakPointer)
    #[serde(default)]
    pub style: Option<Handle<AudioBreathStyleNode>>,
}

impl Pooled for AudioBreathStyleTransitionNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_breath_style_transition_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_breath_style_transition_node }
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

/// DCB type: `AudioBreathStyle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyle {
    /// DCB field: `firstBreathCustomisation` (Class (array))
    #[serde(default)]
    pub first_breath_customisation: Vec<Handle<ActorBreathingStyleStartup>>,
    /// DCB field: `VO2InputParam` (EnumChoice)
    #[serde(default)]
    pub vo2_input_param: String,
    /// DCB field: `minInputValue` (Single)
    #[serde(default)]
    pub min_input_value: f32,
    /// DCB field: `maxInputValue` (Single)
    #[serde(default)]
    pub max_input_value: f32,
    /// DCB field: `InputFallingPredictionTime` (Single)
    #[serde(default)]
    pub input_falling_prediction_time: f32,
    /// DCB field: `InputRisingPredictionTime` (Single)
    #[serde(default)]
    pub input_rising_prediction_time: f32,
    /// DCB field: `allowBreathShortening` (Boolean)
    #[serde(default)]
    pub allow_breath_shortening: bool,
    /// DCB field: `VO2FromInput` (Class)
    #[serde(default)]
    pub vo2_from_input: Option<Handle<BezierCurve>>,
    /// DCB field: `maxVO2FallRate` (Single)
    #[serde(default)]
    pub max_vo2_fall_rate: f32,
    /// DCB field: `maxVO2RiseRate` (Single)
    #[serde(default)]
    pub max_vo2_rise_rate: f32,
    /// DCB field: `durationFromVO2` (Class)
    #[serde(default)]
    pub duration_from_vo2: Option<Handle<BezierCurve>>,
    /// DCB field: `volumeFromVO2` (Class)
    #[serde(default)]
    pub volume_from_vo2: Option<Handle<BezierCurve>>,
    /// DCB field: `durationVolumeScaleFromVO2Delta` (Class)
    #[serde(default)]
    pub duration_volume_scale_from_vo2_delta: Option<Handle<BezierCurve>>,
    /// DCB field: `inhaleExhaleRatioFromVO2Delta` (Class)
    #[serde(default)]
    pub inhale_exhale_ratio_from_vo2_delta: Option<Handle<BezierCurve>>,
    /// DCB field: `minVolume` (Single)
    #[serde(default)]
    pub min_volume: f32,
    /// DCB field: `maxVolume` (Single)
    #[serde(default)]
    pub max_volume: f32,
    /// DCB field: `maxVolumeDrop` (Single)
    #[serde(default)]
    pub max_volume_drop: f32,
    /// DCB field: `maxVolumeRise` (Single)
    #[serde(default)]
    pub max_volume_rise: f32,
    /// DCB field: `minDuration` (Single)
    #[serde(default)]
    pub min_duration: f32,
    /// DCB field: `maxDuration` (Single)
    #[serde(default)]
    pub max_duration: f32,
    /// DCB field: `maxDurationDrop` (Single)
    #[serde(default)]
    pub max_duration_drop: f32,
    /// DCB field: `maxDurationRise` (Single)
    #[serde(default)]
    pub max_duration_rise: f32,
    /// DCB field: `audioEventOnEnter` (Class)
    #[serde(default)]
    pub audio_event_on_enter: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `audioEventOnExit` (Class)
    #[serde(default)]
    pub audio_event_on_exit: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `audioEvents` (Class)
    #[serde(default)]
    pub audio_events: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `landingInterrupt` (Reference)
    #[serde(default)]
    pub landing_interrupt: Option<CigGuid>,
}

impl Pooled for AudioBreathStyle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_breath_style }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_breath_style }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_breath_style_base_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_breath_style_base_node }
}

impl<'a> Extract<'a> for AudioBreathStyleBaseNode {
    const TYPE_NAME: &'static str = "AudioBreathStyleBaseNode";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `AudioBreathStyleNode`
///
/// Inherits from: `AudioBreathStyleBaseNode` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBreathStyleNode {
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `style` (Reference)
    #[serde(default)]
    pub style: Option<CigGuid>,
    /// DCB field: `transitions` (WeakPointer (array))
    #[serde(default)]
    pub transitions: Vec<Handle<AudioBreathStyleTransitionNode>>,
}

impl Pooled for AudioBreathStyleNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_breath_style_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_breath_style_node }
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
    /// DCB field: `initialStyle` (WeakPointer)
    #[serde(default)]
    pub initial_style: Option<Handle<AudioBreathStyleNode>>,
    /// DCB field: `nodes` (StrongPointer (array))
    #[serde(default)]
    pub nodes: Vec<Handle<AudioBreathStyleBaseNode>>,
}

impl Pooled for AudioBreathStyleSuite {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_breath_style_suite }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_breath_style_suite }
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
    /// DCB field: `defaultBreathingStyles` (Reference)
    #[serde(default)]
    pub default_breathing_styles: Option<CigGuid>,
    /// DCB field: `pilotBreathingStyles` (Reference)
    #[serde(default)]
    pub pilot_breathing_styles: Option<CigGuid>,
    /// DCB field: `params` (Class)
    #[serde(default)]
    pub params: Option<Handle<SAudioBreathParameters>>,
    /// DCB field: `audioEvents` (Class)
    #[serde(default)]
    pub audio_events: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `audioRTPCs` (Class)
    #[serde(default)]
    pub audio_rtpcs: Option<Handle<AudioRtpcWithDefault>>,
    /// DCB field: `breathVolumeParams` (Class)
    #[serde(default)]
    pub breath_volume_params: Option<Handle<BreathVolumeParams>>,
    /// DCB field: `breathDurationParams` (Class)
    #[serde(default)]
    pub breath_duration_params: Option<Handle<BreathDurationParams>>,
    /// DCB field: `holdBreathStylesWhitelist` (Reference (array))
    #[serde(default)]
    pub hold_breath_styles_whitelist: Vec<CigGuid>,
    /// DCB field: `defaultProcBreathingSetup` (Reference)
    #[serde(default)]
    pub default_proc_breathing_setup: Option<CigGuid>,
    /// DCB field: `defaultLandingRecord` (Reference)
    #[serde(default)]
    pub default_landing_record: Option<CigGuid>,
    /// DCB field: `mistedBreathParams` (Class)
    #[serde(default)]
    pub misted_breath_params: Option<Handle<MistedBreathParams>>,
    /// DCB field: `stanceBreathModifiers` (Class (array))
    #[serde(default)]
    pub stance_breath_modifiers: Vec<Handle<StanceBreathModifier>>,
}

impl Pooled for AudioBreathDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_breath_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_breath_definition }
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
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
    /// DCB field: `breathEvent` (EnumChoice)
    #[serde(default)]
    pub breath_event: String,
    /// DCB field: `customEvent` (Class)
    #[serde(default)]
    pub custom_event: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `triggerCustomEventOnly` (Boolean)
    #[serde(default)]
    pub trigger_custom_event_only: bool,
}

impl Pooled for AudioBreathInterrupt {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_breath_interrupt }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_breath_interrupt }
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

/// DCB type: `AudioWhitelist`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioWhitelist {
    /// DCB field: `triggerTypes` (EnumChoice (array))
    #[serde(default)]
    pub trigger_types: Vec<String>,
}

impl Pooled for AudioWhitelist {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_whitelist }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_whitelist }
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
    /// DCB field: `wwiseEnvironmentName` (String)
    #[serde(default)]
    pub wwise_environment_name: String,
    /// DCB field: `interiorityMinimum` (Single)
    #[serde(default)]
    pub interiority_minimum: f32,
    /// DCB field: `interiorityMaximum` (Single)
    #[serde(default)]
    pub interiority_maximum: f32,
    /// DCB field: `sizeMinimum` (Single)
    #[serde(default)]
    pub size_minimum: f32,
    /// DCB field: `sizeMaximum` (Single)
    #[serde(default)]
    pub size_maximum: f32,
    /// DCB field: `primarySurfaceType` (String)
    #[serde(default)]
    pub primary_surface_type: String,
    /// DCB field: `secondarySurfaceType` (String)
    #[serde(default)]
    pub secondary_surface_type: String,
}

impl Pooled for AudioEnvironment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_environment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_environment }
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
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `maxAudioObjects` (Int32)
    #[serde(default)]
    pub max_audio_objects: i32,
    /// DCB field: `priorityFalloffPerSecond` (Single)
    #[serde(default)]
    pub priority_falloff_per_second: f32,
}

impl Pooled for AudioOneShotManagerBudgetEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_one_shot_manager_budget_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_one_shot_manager_budget_entry }
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
    /// DCB field: `oneshotBudget` (Class (array))
    #[serde(default)]
    pub oneshot_budget: Vec<Handle<AudioOneShotManagerBudgetEntry>>,
    /// DCB field: `shipAudioLimit` (Int32)
    #[serde(default)]
    pub ship_audio_limit: i32,
    /// DCB field: `shipThrusterLimit` (Int32)
    #[serde(default)]
    pub ship_thruster_limit: i32,
    /// DCB field: `actorFoleyLimit` (Int32)
    #[serde(default)]
    pub actor_foley_limit: i32,
}

impl Pooled for AudioBudgetDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_budget_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_budget_definition }
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
    /// DCB field: `globalStates` (Class (array))
    #[serde(default)]
    pub global_states: Vec<Handle<AudioSwitch>>,
    /// DCB field: `globalRTPCs` (Class (array))
    #[serde(default)]
    pub global_rtpcs: Vec<Handle<AudioRtpcWithDefault>>,
}

impl Pooled for AudioGameContextGlobals {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_game_context_globals }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_game_context_globals }
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
    /// DCB field: `budgetDefinition` (Reference)
    #[serde(default)]
    pub budget_definition: Option<CigGuid>,
    /// DCB field: `globalRtpcsAndStates` (Reference)
    #[serde(default)]
    pub global_rtpcs_and_states: Option<CigGuid>,
}

impl Pooled for AudioGameContext {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_game_context }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_game_context }
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
    /// DCB field: `gameContexts` (Class)
    #[serde(default)]
    pub game_contexts: Option<Handle<AudioGameContext>>,
}

impl Pooled for AudioGameContextSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_game_context_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_game_context_setup }
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

/// DCB type: `AudioTagAction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTagAction {
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `audioTrigger` (Class)
    #[serde(default)]
    pub audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `switch` (Class)
    #[serde(default)]
    pub switch: Option<Handle<AudioSwitch>>,
}

impl Pooled for AudioTagAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_tag_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_tag_action }
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
    /// DCB field: `tagActionList` (Class (array))
    #[serde(default)]
    pub tag_action_list: Vec<Handle<AudioTagAction>>,
}

impl Pooled for AudioTagActionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_tag_action_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_tag_action_list }
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
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
}

impl Pooled for AudioValueOutputBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_value_output_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_value_output_behaviour }
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
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `pluginInstanceID` (Int32)
    #[serde(default)]
    pub plugin_instance_id: i32,
    /// DCB field: `behaviours` (StrongPointer (array))
    #[serde(default)]
    pub behaviours: Vec<Handle<AudioValueOutputBehaviour>>,
}

impl Pooled for AudioValueOutput {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_value_output }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_value_output }
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
    /// DCB field: `outputs` (Class (array))
    #[serde(default)]
    pub outputs: Vec<Handle<AudioValueOutput>>,
}

impl Pooled for AudioValueOutputSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_value_output_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_value_output_setup }
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

/// DCB type: `AudioAllegianceSwitches`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAllegianceSwitches {
    /// DCB field: `allegianceRTPC` (Class)
    #[serde(default)]
    pub allegiance_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `neutralRtpcValue` (Single)
    #[serde(default)]
    pub neutral_rtpc_value: f32,
    /// DCB field: `friendlyRtpcValue` (Single)
    #[serde(default)]
    pub friendly_rtpc_value: f32,
    /// DCB field: `hostileRtpcValue` (Single)
    #[serde(default)]
    pub hostile_rtpc_value: f32,
}

impl Pooled for AudioAllegianceSwitches {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_allegiance_switches }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_allegiance_switches }
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
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `radius` (Class)
    #[serde(default)]
    pub radius: Option<Handle<BezierCurve>>,
    /// DCB field: `maxRadius` (Single)
    #[serde(default)]
    pub max_radius: f32,
    /// DCB field: `environmentValue` (Class)
    #[serde(default)]
    pub environment_value: Option<Handle<BezierCurve>>,
    /// DCB field: `maxEnvironmentValue` (Single)
    #[serde(default)]
    pub max_environment_value: f32,
    /// DCB field: `lifeTime` (Single)
    #[serde(default)]
    pub life_time: f32,
    /// DCB field: `effectRtpc` (Class)
    #[serde(default)]
    pub effect_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for AudioEnvironmentFeedbackZoneProcess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_environment_feedback_zone_process }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_environment_feedback_zone_process }
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
    /// DCB field: `processes` (Class (array))
    #[serde(default)]
    pub processes: Vec<Handle<AudioEnvironmentFeedbackZoneProcess>>,
}

impl Pooled for AudioEnvironmentFeedbackZoneSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_environment_feedback_zone_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_environment_feedback_zone_setup }
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
    /// DCB field: `rtpcs` (Class (array))
    #[serde(default)]
    pub rtpcs: Vec<Handle<AudioRtpcWithBehaviour>>,
}

impl Pooled for AudioEnvironmentMovementRtpcBehavior {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_environment_movement_rtpc_behavior }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_environment_movement_rtpc_behavior }
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
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// DCB field: `loopStart` (Class)
    #[serde(default)]
    pub loop_start: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `loopStop` (Class)
    #[serde(default)]
    pub loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `movementRtpcs` (Class)
    #[serde(default)]
    pub movement_rtpcs: Option<Handle<AudioEnvironmentMovementRtpcBehavior>>,
}

impl Pooled for AudioEnvironmentFeedbackTagAndEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_environment_feedback_tag_and_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_environment_feedback_tag_and_event }
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
    /// DCB field: `tagAndEvents` (Class (array))
    #[serde(default)]
    pub tag_and_events: Vec<Handle<AudioEnvironmentFeedbackTagAndEvent>>,
}

impl Pooled for AudioEnvironmentFeedbackPointDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_environment_feedback_point_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_environment_feedback_point_def }
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
    /// DCB field: `trigger` (Class)
    #[serde(default)]
    pub trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `hitByPlayerTrigger` (Class)
    #[serde(default)]
    pub hit_by_player_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `rtpcTimeSinceLastTrigger` (Class)
    #[serde(default)]
    pub rtpc_time_since_last_trigger: Option<Handle<AudioRtpc>>,
    /// DCB field: `rtpcDamage` (Class)
    #[serde(default)]
    pub rtpc_damage: Option<Handle<AudioRtpc>>,
    /// DCB field: `rtpcRatioAfterHit` (Class)
    #[serde(default)]
    pub rtpc_ratio_after_hit: Option<Handle<AudioRtpc>>,
    /// DCB field: `oneShotMinPlayTime` (Single)
    #[serde(default)]
    pub one_shot_min_play_time: f32,
    /// DCB field: `cooldown` (Single)
    #[serde(default)]
    pub cooldown: f32,
    /// DCB field: `oneshotTag` (Reference)
    #[serde(default)]
    pub oneshot_tag: Option<CigGuid>,
    /// DCB field: `oneshotTagPlayer` (Reference)
    #[serde(default)]
    pub oneshot_tag_player: Option<CigGuid>,
}

impl Pooled for AudioHitListenerTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_hit_listener_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_hit_listener_trigger }
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
    /// DCB field: `triggerImpactHealth` (Class)
    #[serde(default)]
    pub trigger_impact_health: Option<Handle<AudioHitListenerTrigger>>,
    /// DCB field: `triggerImpactShield` (Class)
    #[serde(default)]
    pub trigger_impact_shield: Option<Handle<AudioHitListenerTrigger>>,
    /// DCB field: `triggerShieldFail` (Class)
    #[serde(default)]
    pub trigger_shield_fail: Option<Handle<AudioHitListenerTrigger>>,
}

impl Pooled for AudioHitTypeDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_hit_type_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_hit_type_definition }
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
    /// DCB field: `melee` (StrongPointer)
    #[serde(default)]
    pub melee: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `collision` (StrongPointer)
    #[serde(default)]
    pub collision: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `crash` (StrongPointer)
    #[serde(default)]
    pub crash: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `frag` (StrongPointer)
    #[serde(default)]
    pub frag: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `explosion` (StrongPointer)
    #[serde(default)]
    pub explosion: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `takedown` (StrongPointer)
    #[serde(default)]
    pub takedown: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `punish` (StrongPointer)
    #[serde(default)]
    pub punish: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `normal` (StrongPointer)
    #[serde(default)]
    pub normal: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `fire` (StrongPointer)
    #[serde(default)]
    pub fire: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `bullet` (StrongPointer)
    #[serde(default)]
    pub bullet: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `vehicleDestruction` (StrongPointer)
    #[serde(default)]
    pub vehicle_destruction: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `eventDamage` (StrongPointer)
    #[serde(default)]
    pub event_damage: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `bleedOut` (StrongPointer)
    #[serde(default)]
    pub bleed_out: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `electricArc` (StrongPointer)
    #[serde(default)]
    pub electric_arc: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `repair` (StrongPointer)
    #[serde(default)]
    pub repair: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `suffocate` (StrongPointer)
    #[serde(default)]
    pub suffocate: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `suicide` (StrongPointer)
    #[serde(default)]
    pub suicide: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `selfDestruct` (StrongPointer)
    #[serde(default)]
    pub self_destruct: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `boundaryViolation` (StrongPointer)
    #[serde(default)]
    pub boundary_violation: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `drown` (StrongPointer)
    #[serde(default)]
    pub drown: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `damageOverTime` (StrongPointer)
    #[serde(default)]
    pub damage_over_time: Option<Handle<AudioHitTypeDefinition>>,
    /// DCB field: `hazard` (StrongPointer)
    #[serde(default)]
    pub hazard: Option<Handle<AudioHitTypeDefinition>>,
}

impl Pooled for AudioHitListenerDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_hit_listener_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_hit_listener_definition }
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

/// DCB type: `AutoSpawnSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSpawnSettings {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `initialActivity` (String)
    #[serde(default)]
    pub initial_activity: String,
    /// DCB field: `positiveCharacterTags` (Class)
    #[serde(default)]
    pub positive_character_tags: Option<Handle<TagList>>,
    /// DCB field: `negativeCharacterTags` (Class)
    #[serde(default)]
    pub negative_character_tags: Option<Handle<TagList>>,
    /// DCB field: `excludeShipCrew` (Boolean)
    #[serde(default)]
    pub exclude_ship_crew: bool,
    /// DCB field: `excludeSpawnGender` (EnumChoice)
    #[serde(default)]
    pub exclude_spawn_gender: String,
    /// DCB field: `minGroupSize` (Int32)
    #[serde(default)]
    pub min_group_size: i32,
    /// DCB field: `maxGroupSize` (Int32)
    #[serde(default)]
    pub max_group_size: i32,
    /// DCB field: `maxConcurrentSpawns` (Int32)
    #[serde(default)]
    pub max_concurrent_spawns: i32,
    /// DCB field: `maxSpawns` (Int32)
    #[serde(default)]
    pub max_spawns: i32,
    /// DCB field: `minSpawnDelay` (Single)
    #[serde(default)]
    pub min_spawn_delay: f32,
    /// DCB field: `maxSpawnDelay` (Single)
    #[serde(default)]
    pub max_spawn_delay: f32,
    /// DCB field: `inventoryItems` (Class (array))
    #[serde(default)]
    pub inventory_items: Vec<Handle<SpawnSettingsInventoryItem>>,
    /// DCB field: `closetPositiveTags` (Class)
    #[serde(default)]
    pub closet_positive_tags: Option<Handle<TagList>>,
    /// DCB field: `closetNegativeTags` (Class)
    #[serde(default)]
    pub closet_negative_tags: Option<Handle<TagList>>,
    /// DCB field: `roomPositiveTags` (Class)
    #[serde(default)]
    pub room_positive_tags: Option<Handle<TagList>>,
    /// DCB field: `roomNegativeTags` (Class)
    #[serde(default)]
    pub room_negative_tags: Option<Handle<TagList>>,
    /// DCB field: `defendAreaPositiveTags` (Class)
    #[serde(default)]
    pub defend_area_positive_tags: Option<Handle<TagList>>,
    /// DCB field: `defendAreaNegativeTags` (Class)
    #[serde(default)]
    pub defend_area_negative_tags: Option<Handle<TagList>>,
    /// DCB field: `scheduleAreaPositiveTags` (Class)
    #[serde(default)]
    pub schedule_area_positive_tags: Option<Handle<TagList>>,
    /// DCB field: `scheduleAreaNegativeTags` (Class)
    #[serde(default)]
    pub schedule_area_negative_tags: Option<Handle<TagList>>,
    /// DCB field: `entityTags` (Class)
    #[serde(default)]
    pub entity_tags: Option<Handle<TagList>>,
    /// DCB field: `factionOverride` (Reference)
    #[serde(default)]
    pub faction_override: Option<CigGuid>,
    /// DCB field: `missionAlliedMarker` (Boolean)
    #[serde(default)]
    pub mission_allied_marker: bool,
    /// DCB field: `isCritical` (Boolean)
    #[serde(default)]
    pub is_critical: bool,
}

impl Pooled for AutoSpawnSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.auto_spawn_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.auto_spawn_settings }
}

impl<'a> Extract<'a> for AutoSpawnSettings {
    const TYPE_NAME: &'static str = "AutoSpawnSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            initial_activity: inst.get_str("initialActivity").map(String::from).unwrap_or_default(),
            positive_character_tags: match inst.get("positiveCharacterTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            negative_character_tags: match inst.get("negativeCharacterTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exclude_ship_crew: inst.get_bool("excludeShipCrew").unwrap_or_default(),
            exclude_spawn_gender: inst.get_str("excludeSpawnGender").map(String::from).unwrap_or_default(),
            min_group_size: inst.get_i32("minGroupSize").unwrap_or_default(),
            max_group_size: inst.get_i32("maxGroupSize").unwrap_or_default(),
            max_concurrent_spawns: inst.get_i32("maxConcurrentSpawns").unwrap_or_default(),
            max_spawns: inst.get_i32("maxSpawns").unwrap_or_default(),
            min_spawn_delay: inst.get_f32("minSpawnDelay").unwrap_or_default(),
            max_spawn_delay: inst.get_f32("maxSpawnDelay").unwrap_or_default(),
            inventory_items: inst.get_array("inventoryItems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SpawnSettingsInventoryItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SpawnSettingsInventoryItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            closet_positive_tags: match inst.get("closetPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            closet_negative_tags: match inst.get("closetNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_positive_tags: match inst.get("roomPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_negative_tags: match inst.get("roomNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            defend_area_positive_tags: match inst.get("defendAreaPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            defend_area_negative_tags: match inst.get("defendAreaNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            schedule_area_positive_tags: match inst.get("scheduleAreaPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            schedule_area_negative_tags: match inst.get("scheduleAreaNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entity_tags: match inst.get("entityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            faction_override: inst.get("factionOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_allied_marker: inst.get_bool("missionAlliedMarker").unwrap_or_default(),
            is_critical: inst.get_bool("isCritical").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioFootstepSurfacesDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFootstepSurfacesDefinition {
    /// DCB field: `audioSurfaces` (Class (array))
    #[serde(default)]
    pub audio_surfaces: Vec<Handle<AudioFootstepSurfaceMapping>>,
}

impl Pooled for AudioFootstepSurfacesDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_footstep_surfaces_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_footstep_surfaces_definition }
}

impl<'a> Extract<'a> for AudioFootstepSurfacesDefinition {
    const TYPE_NAME: &'static str = "AudioFootstepSurfacesDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_surfaces: inst.get_array("audioSurfaces")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioFootstepSurfaceMapping>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AudioFootstepSurfaceMapping>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioFootstepSurfaceMapping`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFootstepSurfaceMapping {
    /// DCB field: `surfaceType` (String)
    #[serde(default)]
    pub surface_type: String,
    /// DCB field: `heelLandAudioTrigger` (String)
    #[serde(default)]
    pub heel_land_audio_trigger: String,
    /// DCB field: `toeLandAudioTrigger` (String)
    #[serde(default)]
    pub toe_land_audio_trigger: String,
    /// DCB field: `footLiftAudioTrigger` (String)
    #[serde(default)]
    pub foot_lift_audio_trigger: String,
    /// DCB field: `turnPlayAudioTrigger` (String)
    #[serde(default)]
    pub turn_play_audio_trigger: String,
    /// DCB field: `turnStopAudioTrigger` (String)
    #[serde(default)]
    pub turn_stop_audio_trigger: String,
    /// DCB field: `fadeSteps` (Int32)
    #[serde(default)]
    pub fade_steps: i32,
}

impl Pooled for AudioFootstepSurfaceMapping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_footstep_surface_mapping }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_footstep_surface_mapping }
}

impl<'a> Extract<'a> for AudioFootstepSurfaceMapping {
    const TYPE_NAME: &'static str = "AudioFootstepSurfaceMapping";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            surface_type: inst.get_str("surfaceType").map(String::from).unwrap_or_default(),
            heel_land_audio_trigger: inst.get_str("heelLandAudioTrigger").map(String::from).unwrap_or_default(),
            toe_land_audio_trigger: inst.get_str("toeLandAudioTrigger").map(String::from).unwrap_or_default(),
            foot_lift_audio_trigger: inst.get_str("footLiftAudioTrigger").map(String::from).unwrap_or_default(),
            turn_play_audio_trigger: inst.get_str("turnPlayAudioTrigger").map(String::from).unwrap_or_default(),
            turn_stop_audio_trigger: inst.get_str("turnStopAudioTrigger").map(String::from).unwrap_or_default(),
            fade_steps: inst.get_i32("fadeSteps").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioSignalList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSignalList {
    /// DCB field: `Signals` (Class (array))
    #[serde(default)]
    pub signals: Vec<Handle<AudioSignal>>,
}

impl Pooled for AudioSignalList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_signal_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_signal_list }
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

/// DCB type: `AudioSignalRtpc`
///
/// Inherits from: `AudioRtpcWithDefault` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSignalRtpc {
    /// DCB field: `rtpc` (String)
    #[serde(default)]
    pub rtpc: String,
    /// DCB field: `defaultValue` (Single)
    #[serde(default)]
    pub default_value: f32,
    /// DCB field: `global` (Boolean)
    #[serde(default)]
    pub global: bool,
}

impl Pooled for AudioSignalRtpc {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_signal_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_signal_rtpc }
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
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `audioTriggers` (Class (array))
    #[serde(default)]
    pub audio_triggers: Vec<Handle<GlobalResourceAudio>>,
    /// DCB field: `switches` (Class (array))
    #[serde(default)]
    pub switches: Vec<Handle<AudioSwitch>>,
    /// DCB field: `rtpcs` (Class (array))
    #[serde(default)]
    pub rtpcs: Vec<Handle<AudioSignalRtpc>>,
}

impl Pooled for AudioSignal {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_signal }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_signal }
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

/// DCB type: `AudioRtpc`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRtpc {
    /// DCB field: `rtpc` (String)
    #[serde(default)]
    pub rtpc: String,
}

impl Pooled for AudioRtpc {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_rtpc }
}

impl<'a> Extract<'a> for AudioRtpc {
    const TYPE_NAME: &'static str = "AudioRtpc";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rtpc: inst.get_str("rtpc").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioRtpcWithDefault`
///
/// Inherits from: `AudioRtpc` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRtpcWithDefault {
    /// DCB field: `rtpc` (String)
    #[serde(default)]
    pub rtpc: String,
    /// DCB field: `defaultValue` (Single)
    #[serde(default)]
    pub default_value: f32,
}

impl Pooled for AudioRtpcWithDefault {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_rtpc_with_default }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_rtpc_with_default }
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
    /// DCB field: `switch` (String)
    #[serde(default)]
    pub switch: String,
}

impl Pooled for AudioSwitch {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_switch }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_switch }
}

impl<'a> Extract<'a> for AudioSwitch {
    const TYPE_NAME: &'static str = "AudioSwitch";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            switch: inst.get_str("switch").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioRtpcWithBehaviour`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRtpcWithBehaviour {
    /// DCB field: `rtpc` (Class)
    #[serde(default)]
    pub rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `behavior` (StrongPointer)
    #[serde(default)]
    pub behavior: Option<Handle<SRtpcBehaviour>>,
}

impl Pooled for AudioRtpcWithBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_au.audio_rtpc_with_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_au.audio_rtpc_with_behaviour }
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

