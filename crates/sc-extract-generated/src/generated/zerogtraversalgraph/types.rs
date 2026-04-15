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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ZeroGTraversalConnection`
pub struct ZeroGTraversalConnection {
    /// `waitUntillFinished` (Boolean)
    pub wait_untill_finished: bool,
    /// `delaySeconds` (Single)
    pub delay_seconds: f32,
    /// `waitForEvent` (String)
    pub wait_for_event: String,
    /// `resetViewOnTransition` (Boolean)
    pub reset_view_on_transition: bool,
    /// `playExitAnimation` (Boolean)
    pub play_exit_animation: bool,
    /// `allowExitYield` (Boolean)
    pub allow_exit_yield: bool,
    /// `nextState` (WeakPointer)
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
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ZeroGTraversalState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ZeroGTraversalState`
pub struct ZeroGTraversalState {
    /// `type` (EnumChoice)
    pub r#type: ZeroGTraversalAction,
    /// `mannequinTags` (String)
    pub mannequin_tags: String,
    /// `mannequinFragment` (String)
    pub mannequin_fragment: String,
    /// `playEnterAnimation` (Boolean)
    pub play_enter_animation: bool,
    /// `canTurnInState` (Boolean)
    pub can_turn_in_state: bool,
    /// `useAnimMotionControl` (Boolean)
    pub use_anim_motion_control: bool,
    /// `connections` (Class (array))
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
            r#type: ZeroGTraversalAction::from_dcb_str(inst.get_str("type").unwrap_or("")),
            mannequin_tags: inst.get_str("mannequinTags").map(String::from).unwrap_or_default(),
            mannequin_fragment: inst.get_str("mannequinFragment").map(String::from).unwrap_or_default(),
            play_enter_animation: inst.get_bool("playEnterAnimation").unwrap_or_default(),
            can_turn_in_state: inst.get_bool("canTurnInState").unwrap_or_default(),
            use_anim_motion_control: inst.get_bool("useAnimMotionControl").unwrap_or_default(),
            connections: inst.get_array("connections")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ZeroGTraversalConnection>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ZeroGTraversalConnection>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ZeroGTraversalGraph`
pub struct ZeroGTraversalGraph {
    /// `traversalStates` (Class (array))
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
                        Value::ClassRef(r) => Some(b.alloc_nested::<ZeroGTraversalState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

