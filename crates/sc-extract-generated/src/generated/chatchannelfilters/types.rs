// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `chatchannelfilters`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SChatChannelFilterBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SChatChannelFilterBase {
    /// `chatChannelType` (StrongPointer (array))
    #[serde(default)]
    pub chat_channel_type: Vec<Handle<SChatChannelTypeBase>>,
}

impl Pooled for SChatChannelFilterBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatchannelfilters.schat_channel_filter_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatchannelfilters.schat_channel_filter_base }
}

impl<'a> Extract<'a> for SChatChannelFilterBase {
    const TYPE_NAME: &'static str = "SChatChannelFilterBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            chat_channel_type: inst.get_array("chatChannelType")
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

/// DCB type: `ChatChannelFilterRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChannelFilterRecord {
    /// `chatChannelFilter` (StrongPointer)
    #[serde(default)]
    pub chat_channel_filter: Option<Handle<SChatChannelFilterBase>>,
}

impl Pooled for ChatChannelFilterRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatchannelfilters.chat_channel_filter_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatchannelfilters.chat_channel_filter_record }
}

impl<'a> Extract<'a> for ChatChannelFilterRecord {
    const TYPE_NAME: &'static str = "ChatChannelFilterRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            chat_channel_filter: match inst.get("chatChannelFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SChatChannelFilterBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SChatChannelFilterBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

