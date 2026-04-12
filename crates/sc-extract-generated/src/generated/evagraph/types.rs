// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `evagraph`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `EVAConnection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVAConnection {
    /// `waitUntillFinished` (Boolean)
    #[serde(default)]
    pub wait_untill_finished: bool,
    /// `delaySeconds` (Single)
    #[serde(default)]
    pub delay_seconds: f32,
    /// `waitForEvent` (String)
    #[serde(default)]
    pub wait_for_event: String,
    /// `nextState` (WeakPointer)
    #[serde(default)]
    pub next_state: Option<Handle<EVAState>>,
}

impl Pooled for EVAConnection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.evagraph.evaconnection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.evagraph.evaconnection }
}

impl<'a> Extract<'a> for EVAConnection {
    const TYPE_NAME: &'static str = "EVAConnection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wait_untill_finished: inst.get_bool("waitUntillFinished").unwrap_or_default(),
            delay_seconds: inst.get_f32("delaySeconds").unwrap_or_default(),
            wait_for_event: inst.get_str("waitForEvent").map(String::from).unwrap_or_default(),
            next_state: match inst.get("nextState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EVAState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EVAState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EVAState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVAState {
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// `mannequinTags` (String)
    #[serde(default)]
    pub mannequin_tags: String,
    /// `mannequinFragment` (String)
    #[serde(default)]
    pub mannequin_fragment: String,
    /// `connections` (Class (array))
    #[serde(default)]
    pub connections: Vec<Handle<EVAConnection>>,
}

impl Pooled for EVAState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.evagraph.evastate }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.evagraph.evastate }
}

impl<'a> Extract<'a> for EVAState {
    const TYPE_NAME: &'static str = "EVAState";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            mannequin_tags: inst.get_str("mannequinTags").map(String::from).unwrap_or_default(),
            mannequin_fragment: inst.get_str("mannequinFragment").map(String::from).unwrap_or_default(),
            connections: inst.get_array("connections")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EVAConnection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<EVAConnection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EVAGraph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVAGraph {
    /// `EVAStates` (Class (array))
    #[serde(default)]
    pub evastates: Vec<Handle<EVAState>>,
}

impl Pooled for EVAGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.evagraph.evagraph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.evagraph.evagraph }
}

impl<'a> Extract<'a> for EVAGraph {
    const TYPE_NAME: &'static str = "EVAGraph";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            evastates: inst.get_array("EVAStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EVAState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<EVAState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

