// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `fidgetconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `Fidget`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fidget {
    /// `FragmentTags` (String (array))
    #[serde(default)]
    pub fragment_tags: Vec<String>,
    /// `IntervalMin` (Single)
    #[serde(default)]
    pub interval_min: f32,
    /// `IntervalMax` (Single)
    #[serde(default)]
    pub interval_max: f32,
}

impl Pooled for Fidget {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.fidgetconfig.fidget }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.fidgetconfig.fidget }
}

impl<'a> Extract<'a> for Fidget {
    const TYPE_NAME: &'static str = "Fidget";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fragment_tags: inst.get_array("FragmentTags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            interval_min: inst.get_f32("IntervalMin").unwrap_or_default(),
            interval_max: inst.get_f32("IntervalMax").unwrap_or_default(),
        }
    }
}

/// DCB type: `FidgetConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FidgetConfig {
    /// `FragmentId` (String)
    #[serde(default)]
    pub fragment_id: String,
    /// `Fidgets` (Class (array))
    #[serde(default)]
    pub fidgets: Vec<Handle<Fidget>>,
    /// `RepeatTime` (Single)
    #[serde(default)]
    pub repeat_time: f32,
    /// `BreakTime` (Single)
    #[serde(default)]
    pub break_time: f32,
}

impl Pooled for FidgetConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.fidgetconfig.fidget_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.fidgetconfig.fidget_config }
}

impl<'a> Extract<'a> for FidgetConfig {
    const TYPE_NAME: &'static str = "FidgetConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fragment_id: inst.get_str("FragmentId").map(String::from).unwrap_or_default(),
            fidgets: inst.get_array("Fidgets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Fidget>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Fidget>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            repeat_time: inst.get_f32("RepeatTime").unwrap_or_default(),
            break_time: inst.get_f32("BreakTime").unwrap_or_default(),
        }
    }
}

