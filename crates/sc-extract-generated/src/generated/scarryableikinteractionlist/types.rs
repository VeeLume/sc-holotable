// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `scarryableikinteractionlist`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCarryableIKInteraction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCarryableIKInteraction {
    /// `helperName` (String)
    #[serde(default)]
    pub helper_name: String,
    /// `targetOffset` (Class)
    #[serde(default)]
    pub target_offset: Option<Handle<QuatT>>,
    /// `cdikTargetName` (String)
    #[serde(default)]
    pub cdik_target_name: String,
}

impl Pooled for SCarryableIKInteraction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.scarryableikinteractionlist.scarryable_ikinteraction }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.scarryableikinteractionlist.scarryable_ikinteraction }
}

impl<'a> Extract<'a> for SCarryableIKInteraction {
    const TYPE_NAME: &'static str = "SCarryableIKInteraction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            helper_name: inst.get_str("helperName").map(String::from).unwrap_or_default(),
            target_offset: match inst.get("targetOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cdik_target_name: inst.get_str("cdikTargetName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCarryableIKInteractionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCarryableIKInteractionList {
    /// `ikInteractions` (Class (array))
    #[serde(default)]
    pub ik_interactions: Vec<Handle<SCarryableIKInteraction>>,
}

impl Pooled for SCarryableIKInteractionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.scarryableikinteractionlist.scarryable_ikinteraction_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.scarryableikinteractionlist.scarryable_ikinteraction_list }
}

impl<'a> Extract<'a> for SCarryableIKInteractionList {
    const TYPE_NAME: &'static str = "SCarryableIKInteractionList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ik_interactions: inst.get_array("ikInteractions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCarryableIKInteraction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCarryableIKInteraction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

