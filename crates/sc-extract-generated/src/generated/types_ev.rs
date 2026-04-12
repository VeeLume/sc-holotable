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

/// DCB type: `EVAConnection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVAConnection {
    /// DCB field: `waitUntillFinished` (Boolean)
    #[serde(default)]
    pub wait_untill_finished: bool,
    /// DCB field: `delaySeconds` (Single)
    #[serde(default)]
    pub delay_seconds: f32,
    /// DCB field: `waitForEvent` (String)
    #[serde(default)]
    pub wait_for_event: String,
    /// DCB field: `nextState` (WeakPointer)
    #[serde(default)]
    pub next_state: Option<Handle<EVAState>>,
}

impl Pooled for EVAConnection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ev.evaconnection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ev.evaconnection }
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
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `mannequinTags` (String)
    #[serde(default)]
    pub mannequin_tags: String,
    /// DCB field: `mannequinFragment` (String)
    #[serde(default)]
    pub mannequin_fragment: String,
    /// DCB field: `connections` (Class (array))
    #[serde(default)]
    pub connections: Vec<Handle<EVAConnection>>,
}

impl Pooled for EVAState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ev.evastate }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ev.evastate }
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
    /// DCB field: `EVAStates` (Class (array))
    #[serde(default)]
    pub evastates: Vec<Handle<EVAState>>,
}

impl Pooled for EVAGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ev.evagraph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ev.evagraph }
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

/// DCB type: `EVAReticle_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVAReticle_Config {
    /// DCB field: `EnableFixedReticle` (Boolean)
    #[serde(default)]
    pub enable_fixed_reticle: bool,
    /// DCB field: `EnableLookReticle` (Boolean)
    #[serde(default)]
    pub enable_look_reticle: bool,
    /// DCB field: `EnableVelocityVector` (Boolean)
    #[serde(default)]
    pub enable_velocity_vector: bool,
    /// DCB field: `EnableControlFrameReticle` (Boolean)
    #[serde(default)]
    pub enable_control_frame_reticle: bool,
}

impl Pooled for EVAReticle_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ev.evareticle_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ev.evareticle_config }
}

impl<'a> Extract<'a> for EVAReticle_Config {
    const TYPE_NAME: &'static str = "EVAReticle_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable_fixed_reticle: inst.get_bool("EnableFixedReticle").unwrap_or_default(),
            enable_look_reticle: inst.get_bool("EnableLookReticle").unwrap_or_default(),
            enable_velocity_vector: inst.get_bool("EnableVelocityVector").unwrap_or_default(),
            enable_control_frame_reticle: inst.get_bool("EnableControlFrameReticle").unwrap_or_default(),
        }
    }
}

