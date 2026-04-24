// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `leangraph`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `LeanConnection`
pub struct LeanConnection {
    /// `waitUntilFinished` (Boolean)
    pub wait_until_finished: bool,
    /// `delaySeconds` (Single)
    pub delay_seconds: f32,
    /// `waitForEvent` (String)
    pub wait_for_event: String,
    /// `nextState` (WeakPointer)
    pub next_state: Option<Handle<LeanState>>,
}

impl Pooled for LeanConnection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.leangraph.lean_connection
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.leangraph.lean_connection
    }
}

impl<'a> Extract<'a> for LeanConnection {
    const TYPE_NAME: &'static str = "LeanConnection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wait_until_finished: inst.get_bool("waitUntilFinished").unwrap_or_default(),
            delay_seconds: inst.get_f32("delaySeconds").unwrap_or_default(),
            wait_for_event: inst
                .get_str("waitForEvent")
                .map(String::from)
                .unwrap_or_default(),
            next_state: match inst.get("nextState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<LeanState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `LeanState`
pub struct LeanState {
    /// `type` (EnumChoice)
    pub r#type: LeanStateType,
    /// `mannequinTags` (String)
    pub mannequin_tags: String,
    /// `mannequinFragment` (String)
    pub mannequin_fragment: String,
    /// `connections` (Class (array))
    pub connections: Vec<Handle<LeanConnection>>,
}

impl Pooled for LeanState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.leangraph.lean_state
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.leangraph.lean_state
    }
}

impl<'a> Extract<'a> for LeanState {
    const TYPE_NAME: &'static str = "LeanState";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: LeanStateType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            mannequin_tags: inst
                .get_str("mannequinTags")
                .map(String::from)
                .unwrap_or_default(),
            mannequin_fragment: inst
                .get_str("mannequinFragment")
                .map(String::from)
                .unwrap_or_default(),
            connections: inst
                .get_array("connections")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<LeanConnection>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<LeanConnection>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LeanGraph`
pub struct LeanGraph {
    /// `leanStates` (Class (array))
    pub lean_states: Vec<Handle<LeanState>>,
}

impl Pooled for LeanGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.leangraph.lean_graph
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.leangraph.lean_graph
    }
}

impl<'a> Extract<'a> for LeanGraph {
    const TYPE_NAME: &'static str = "LeanGraph";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            lean_states: inst
                .get_array("leanStates")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LeanState>(
                            Instance::from_inline_data(b.db, struct_index, data),
                            false,
                        )),
                        Value::ClassRef(r) => Some(b.alloc_nested::<LeanState>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
