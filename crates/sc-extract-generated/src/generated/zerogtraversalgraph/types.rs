// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `zerogtraversalgraph`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ZeroGTraversalConnection`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroGTraversalConnection {
    /// `waitUntillFinished` (Boolean)
    #[serde(default)]
    pub wait_untill_finished: bool,
    /// `delaySeconds` (Single)
    #[serde(default)]
    pub delay_seconds: f32,
    /// `waitForEvent` (String)
    #[serde(default)]
    pub wait_for_event: String,
    /// `resetViewOnTransition` (Boolean)
    #[serde(default)]
    pub reset_view_on_transition: bool,
    /// `playExitAnimation` (Boolean)
    #[serde(default)]
    pub play_exit_animation: bool,
    /// `allowExitYield` (Boolean)
    #[serde(default)]
    pub allow_exit_yield: bool,
    /// `nextState` (WeakPointer)
    #[serde(default)]
    pub next_state: Option<Handle<ZeroGTraversalState>>,
}

impl Pooled for ZeroGTraversalConnection {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.zerogtraversalgraph.zero_gtraversal_connection }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.zerogtraversalgraph.zero_gtraversal_connection }
}

impl<'a> Extract<'a> for ZeroGTraversalConnection {
    const TYPE_NAME: &'static str = "ZeroGTraversalConnection";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            wait_untill_finished: inst.get_bool("waitUntillFinished").unwrap_or_default(),
            delay_seconds: inst.get_f32("delaySeconds").unwrap_or_default(),
            wait_for_event: inst.get_str("waitForEvent").map(String::from).unwrap_or_default(),
            reset_view_on_transition: inst.get_bool("resetViewOnTransition").unwrap_or_default(),
            play_exit_animation: inst.get_bool("playExitAnimation").unwrap_or_default(),
            allow_exit_yield: inst.get_bool("allowExitYield").unwrap_or_default(),
            next_state: match inst.get("nextState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ZeroGTraversalState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ZeroGTraversalState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ZeroGTraversalState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroGTraversalState {
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// `mannequinTags` (String)
    #[serde(default)]
    pub mannequin_tags: String,
    /// `mannequinFragment` (String)
    #[serde(default)]
    pub mannequin_fragment: String,
    /// `playEnterAnimation` (Boolean)
    #[serde(default)]
    pub play_enter_animation: bool,
    /// `canTurnInState` (Boolean)
    #[serde(default)]
    pub can_turn_in_state: bool,
    /// `useAnimMotionControl` (Boolean)
    #[serde(default)]
    pub use_anim_motion_control: bool,
    /// `connections` (Class (array))
    #[serde(default)]
    pub connections: Vec<Handle<ZeroGTraversalConnection>>,
}

impl Pooled for ZeroGTraversalState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.zerogtraversalgraph.zero_gtraversal_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.zerogtraversalgraph.zero_gtraversal_state }
}

impl<'a> Extract<'a> for ZeroGTraversalState {
    const TYPE_NAME: &'static str = "ZeroGTraversalState";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            mannequin_tags: inst.get_str("mannequinTags").map(String::from).unwrap_or_default(),
            mannequin_fragment: inst.get_str("mannequinFragment").map(String::from).unwrap_or_default(),
            play_enter_animation: inst.get_bool("playEnterAnimation").unwrap_or_default(),
            can_turn_in_state: inst.get_bool("canTurnInState").unwrap_or_default(),
            use_anim_motion_control: inst.get_bool("useAnimMotionControl").unwrap_or_default(),
            connections: inst.get_array("connections")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ZeroGTraversalConnection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ZeroGTraversalConnection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ZeroGTraversalGraph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroGTraversalGraph {
    /// `traversalStates` (Class (array))
    #[serde(default)]
    pub traversal_states: Vec<Handle<ZeroGTraversalState>>,
}

impl Pooled for ZeroGTraversalGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.zerogtraversalgraph.zero_gtraversal_graph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.zerogtraversalgraph.zero_gtraversal_graph }
}

impl<'a> Extract<'a> for ZeroGTraversalGraph {
    const TYPE_NAME: &'static str = "ZeroGTraversalGraph";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            traversal_states: inst.get_array("traversalStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ZeroGTraversalState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ZeroGTraversalState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

