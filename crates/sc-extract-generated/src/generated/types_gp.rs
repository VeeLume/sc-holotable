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

/// DCB type: `GPUParticleAudio`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUParticleAudio {
    /// DCB field: `emissionStartTriggerOneShot` (Class)
    #[serde(default)]
    pub emission_start_trigger_one_shot: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `emissionStopTriggerOneShot` (Class)
    #[serde(default)]
    pub emission_stop_trigger_one_shot: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `emissionStartTriggerLoop` (Class)
    #[serde(default)]
    pub emission_start_trigger_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `emissionStopTriggerLoop` (Class)
    #[serde(default)]
    pub emission_stop_trigger_loop: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `pulseDurationRtpc` (Class)
    #[serde(default)]
    pub pulse_duration_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `normPulseTimeRtpc` (Class)
    #[serde(default)]
    pub norm_pulse_time_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `strengthValRtpc` (Class)
    #[serde(default)]
    pub strength_val_rtpc: Option<Handle<AudioRtpc>>,
    /// DCB field: `scalingFactorRtpc` (Class)
    #[serde(default)]
    pub scaling_factor_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for GPUParticleAudio {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gp.gpuparticle_audio }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gp.gpuparticle_audio }
}

impl<'a> Extract<'a> for GPUParticleAudio {
    const TYPE_NAME: &'static str = "GPUParticleAudio";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            emission_start_trigger_one_shot: match inst.get("emissionStartTriggerOneShot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            emission_stop_trigger_one_shot: match inst.get("emissionStopTriggerOneShot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            emission_start_trigger_loop: match inst.get("emissionStartTriggerLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            emission_stop_trigger_loop: match inst.get("emissionStopTriggerLoop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pulse_duration_rtpc: match inst.get("pulseDurationRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            norm_pulse_time_rtpc: match inst.get("normPulseTimeRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            strength_val_rtpc: match inst.get("strengthValRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scaling_factor_rtpc: match inst.get("scalingFactorRtpc") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GPUParticleAudioList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUParticleAudioList {
    /// DCB field: `particleAudioList` (Reference (array))
    #[serde(default)]
    pub particle_audio_list: Vec<CigGuid>,
}

impl Pooled for GPUParticleAudioList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_gp.gpuparticle_audio_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_gp.gpuparticle_audio_list }
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

