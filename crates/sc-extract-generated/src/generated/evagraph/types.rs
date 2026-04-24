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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `EVAConnection`
pub struct EVAConnection {
    /// `waitUntillFinished` (Boolean)
    pub wait_untill_finished: bool,
    /// `delaySeconds` (Single)
    pub delay_seconds: f32,
    /// `waitForEvent` (String)
    pub wait_for_event: String,
    /// `nextState` (WeakPointer)
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
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EVAState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EVAState`
pub struct EVAState {
    /// `type` (EnumChoice)
    pub r#type: EVAStateType,
    /// `mannequinTags` (String)
    pub mannequin_tags: String,
    /// `mannequinFragment` (String)
    pub mannequin_fragment: String,
    /// `connections` (Class (array))
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
            r#type: EVAStateType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            mannequin_tags: inst.get_str("mannequinTags").map(String::from).unwrap_or_default(),
            mannequin_fragment: inst.get_str("mannequinFragment").map(String::from).unwrap_or_default(),
            connections: inst.get_array("connections")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EVAConnection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EVAConnection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EVAGraph`
pub struct EVAGraph {
    /// `EVAStates` (Class (array))
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
                        Value::ClassRef(r) => Some(b.alloc_nested::<EVAState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

