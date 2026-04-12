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

/// DCB type: `VoiceChannelSettingsRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChannelSettingsRecord {
    /// DCB field: `autoConnectProximityChannelType` (StrongPointer)
    #[serde(default)]
    pub auto_connect_proximity_channel_type: Option<Handle<SChatChannelTypeBase>>,
    /// DCB field: `autoConnectPrimaryChannelType` (StrongPointer (array))
    #[serde(default)]
    pub auto_connect_primary_channel_type: Vec<Handle<SChatChannelTypeBase>>,
    /// DCB field: `cyclingChannelOption` (Class)
    #[serde(default)]
    pub cycling_channel_option: Option<Handle<CyclingChannelOption>>,
}

impl Pooled for VoiceChannelSettingsRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vo.voice_channel_settings_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vo.voice_channel_settings_record }
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

/// DCB type: `VoiceSingle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSingle {
    /// DCB field: `mannequinTag` (String)
    #[serde(default)]
    pub mannequin_tag: String,
}

impl Pooled for VoiceSingle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vo.voice_single }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vo.voice_single }
}

impl<'a> Extract<'a> for VoiceSingle {
    const TYPE_NAME: &'static str = "VoiceSingle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mannequin_tag: inst.get_str("mannequinTag").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `VoiceBundle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceBundle {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `voices` (Reference (array))
    #[serde(default)]
    pub voices: Vec<CigGuid>,
}

impl Pooled for VoiceBundle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_vo.voice_bundle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_vo.voice_bundle }
}

impl<'a> Extract<'a> for VoiceBundle {
    const TYPE_NAME: &'static str = "VoiceBundle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            voices: inst.get_array("voices")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

