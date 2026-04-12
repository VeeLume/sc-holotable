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

/// DCB type: `BreathableGasParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathableGasParams {
    /// DCB field: `gasToInhale` (Reference)
    #[serde(default)]
    pub gas_to_inhale: Option<CigGuid>,
    /// DCB field: `gasToExhale` (Reference)
    #[serde(default)]
    pub gas_to_exhale: Option<CigGuid>,
    /// DCB field: `gasMasRatio` (Single)
    #[serde(default)]
    pub gas_mas_ratio: f32,
    /// DCB field: `gasConversionRate` (Single)
    #[serde(default)]
    pub gas_conversion_rate: f32,
    /// DCB field: `gasPressureMaxEffect` (Single)
    #[serde(default)]
    pub gas_pressure_max_effect: f32,
    /// DCB field: `actorStatusToAffect` (EnumChoice)
    #[serde(default)]
    pub actor_status_to_affect: String,
}

impl Pooled for BreathableGasParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_br.breathable_gas_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_br.breathable_gas_params }
}

impl<'a> Extract<'a> for BreathableGasParams {
    const TYPE_NAME: &'static str = "BreathableGasParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            gas_to_inhale: inst.get("gasToInhale").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gas_to_exhale: inst.get("gasToExhale").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gas_mas_ratio: inst.get_f32("gasMasRatio").unwrap_or_default(),
            gas_conversion_rate: inst.get_f32("gasConversionRate").unwrap_or_default(),
            gas_pressure_max_effect: inst.get_f32("gasPressureMaxEffect").unwrap_or_default(),
            actor_status_to_affect: inst.get_str("actorStatusToAffect").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BreathableOxygenParams`
///
/// Inherits from: `BreathableGasParams` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathableOxygenParams {
    /// DCB field: `gasToInhale` (Reference)
    #[serde(default)]
    pub gas_to_inhale: Option<CigGuid>,
    /// DCB field: `gasToExhale` (Reference)
    #[serde(default)]
    pub gas_to_exhale: Option<CigGuid>,
    /// DCB field: `gasMasRatio` (Single)
    #[serde(default)]
    pub gas_mas_ratio: f32,
    /// DCB field: `gasConversionRate` (Single)
    #[serde(default)]
    pub gas_conversion_rate: f32,
    /// DCB field: `gasPressureMaxEffect` (Single)
    #[serde(default)]
    pub gas_pressure_max_effect: f32,
    /// DCB field: `actorStatusToAffect` (EnumChoice)
    #[serde(default)]
    pub actor_status_to_affect: String,
    /// DCB field: `gasLowQualityPercentage` (Single)
    #[serde(default)]
    pub gas_low_quality_percentage: f32,
}

impl Pooled for BreathableOxygenParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_br.breathable_oxygen_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_br.breathable_oxygen_params }
}

impl<'a> Extract<'a> for BreathableOxygenParams {
    const TYPE_NAME: &'static str = "BreathableOxygenParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            gas_to_inhale: inst.get("gasToInhale").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gas_to_exhale: inst.get("gasToExhale").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gas_mas_ratio: inst.get_f32("gasMasRatio").unwrap_or_default(),
            gas_conversion_rate: inst.get_f32("gasConversionRate").unwrap_or_default(),
            gas_pressure_max_effect: inst.get_f32("gasPressureMaxEffect").unwrap_or_default(),
            actor_status_to_affect: inst.get_str("actorStatusToAffect").map(String::from).unwrap_or_default(),
            gas_low_quality_percentage: inst.get_f32("gasLowQualityPercentage").unwrap_or_default(),
        }
    }
}

/// DCB type: `BreathingHelperParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathingHelperParams {
    /// DCB field: `attachBone` (String)
    #[serde(default)]
    pub attach_bone: String,
    /// DCB field: `attachOffset` (Class)
    #[serde(default)]
    pub attach_offset: Option<Handle<Vec3>>,
}

impl Pooled for BreathingHelperParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_br.breathing_helper_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_br.breathing_helper_params }
}

impl<'a> Extract<'a> for BreathingHelperParams {
    const TYPE_NAME: &'static str = "BreathingHelperParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attach_bone: inst.get_str("attachBone").map(String::from).unwrap_or_default(),
            attach_offset: match inst.get("attachOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BreathVolumeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathVolumeParams {
    /// DCB field: `defaultVolume` (Single)
    #[serde(default)]
    pub default_volume: f32,
    /// DCB field: `volumeModifier` (Single)
    #[serde(default)]
    pub volume_modifier: f32,
}

impl Pooled for BreathVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_br.breath_volume_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_br.breath_volume_params }
}

impl<'a> Extract<'a> for BreathVolumeParams {
    const TYPE_NAME: &'static str = "BreathVolumeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_volume: inst.get_f32("defaultVolume").unwrap_or_default(),
            volume_modifier: inst.get_f32("volumeModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `BreathDurationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathDurationParams {
    /// DCB field: `defaultDuration` (Single)
    #[serde(default)]
    pub default_duration: f32,
    /// DCB field: `durationModifier` (Single)
    #[serde(default)]
    pub duration_modifier: f32,
    /// DCB field: `holdBreathInhaleTime` (Single)
    #[serde(default)]
    pub hold_breath_inhale_time: f32,
    /// DCB field: `holdBreathExhaleTimes` (Class (array))
    #[serde(default)]
    pub hold_breath_exhale_times: Vec<Handle<HoldExhaleDuration>>,
}

impl Pooled for BreathDurationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_br.breath_duration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_br.breath_duration_params }
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

/// DCB type: `BreathingTriggerDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathingTriggerDef {
    /// DCB field: `audioBreathInterrupt` (Reference)
    #[serde(default)]
    pub audio_breath_interrupt: Option<CigGuid>,
    /// DCB field: `interruptParam` (Single)
    #[serde(default)]
    pub interrupt_param: f32,
    /// DCB field: `resumeBreathingWhenAudioEnds` (Boolean)
    #[serde(default)]
    pub resume_breathing_when_audio_ends: bool,
    /// DCB field: `forceInhaleAfterResume` (Boolean)
    #[serde(default)]
    pub force_inhale_after_resume: bool,
    /// DCB field: `forceExhaleAfterResume` (Boolean)
    #[serde(default)]
    pub force_exhale_after_resume: bool,
}

impl Pooled for BreathingTriggerDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_br.breathing_trigger_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_br.breathing_trigger_def }
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

