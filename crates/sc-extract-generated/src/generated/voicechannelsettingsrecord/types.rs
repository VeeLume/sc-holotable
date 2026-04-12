// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `voicechannelsettingsrecord`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CyclingChannelOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclingChannelOption {
    /// `cyclingChannelCount` (UInt32)
    #[serde(default)]
    pub cycling_channel_count: u32,
    /// `cycleChannelConnectionWaitSecond` (Single)
    #[serde(default)]
    pub cycle_channel_connection_wait_second: f32,
    /// `autoRegisterCyclingChannelTypes` (StrongPointer (array))
    #[serde(default)]
    pub auto_register_cycling_channel_types: Vec<Handle<SChatChannelTypeBase>>,
}

impl Pooled for CyclingChannelOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.voicechannelsettingsrecord.cycling_channel_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.voicechannelsettingsrecord.cycling_channel_option }
}

impl<'a> Extract<'a> for CyclingChannelOption {
    const TYPE_NAME: &'static str = "CyclingChannelOption";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            cycling_channel_count: inst.get_u32("cyclingChannelCount").unwrap_or_default(),
            cycle_channel_connection_wait_second: inst.get_f32("cycleChannelConnectionWaitSecond").unwrap_or_default(),
            auto_register_cycling_channel_types: inst.get_array("autoRegisterCyclingChannelTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SChatChannelTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SChatChannelTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VoiceChannelSettingsRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChannelSettingsRecord {
    /// `autoConnectProximityChannelType` (StrongPointer)
    #[serde(default)]
    pub auto_connect_proximity_channel_type: Option<Handle<SChatChannelTypeBase>>,
    /// `autoConnectPrimaryChannelType` (StrongPointer (array))
    #[serde(default)]
    pub auto_connect_primary_channel_type: Vec<Handle<SChatChannelTypeBase>>,
    /// `cyclingChannelOption` (Class)
    #[serde(default)]
    pub cycling_channel_option: Option<Handle<CyclingChannelOption>>,
}

impl Pooled for VoiceChannelSettingsRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.voicechannelsettingsrecord.voice_channel_settings_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.voicechannelsettingsrecord.voice_channel_settings_record }
}

impl<'a> Extract<'a> for VoiceChannelSettingsRecord {
    const TYPE_NAME: &'static str = "VoiceChannelSettingsRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            auto_connect_proximity_channel_type: match inst.get("autoConnectProximityChannelType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChatChannelTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChatChannelTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            auto_connect_primary_channel_type: inst.get_array("autoConnectPrimaryChannelType")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SChatChannelTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SChatChannelTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            cycling_channel_option: match inst.get("cyclingChannelOption") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CyclingChannelOption>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CyclingChannelOption>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

