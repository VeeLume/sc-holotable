// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `item`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `BaseItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseItem {
}

impl Pooled for BaseItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.item.base_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.item.base_item }
}

impl<'a> Extract<'a> for BaseItem {
    const TYPE_NAME: &'static str = "BaseItem";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `Item`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// `type` (StrongPointer)
    #[serde(default)]
    pub r#type: Option<Handle<BaseItem>>,
}

impl Pooled for Item {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.item.item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.item.item }
}

impl<'a> Extract<'a> for Item {
    const TYPE_NAME: &'static str = "Item";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: match inst.get("type") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BaseItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BaseItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SHelmetLinkedState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHelmetLinkedState {
    /// `stateMachine` (EnumChoice)
    #[serde(default)]
    pub state_machine: String,
    /// `stateToEnter` (EnumChoice)
    #[serde(default)]
    pub state_to_enter: String,
}

impl Pooled for SHelmetLinkedState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.item.shelmet_linked_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.item.shelmet_linked_state }
}

impl<'a> Extract<'a> for SHelmetLinkedState {
    const TYPE_NAME: &'static str = "SHelmetLinkedState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_machine: inst.get_str("stateMachine").map(String::from).unwrap_or_default(),
            state_to_enter: inst.get_str("stateToEnter").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SHelmetStateBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHelmetStateBaseParams {
    /// `state` (EnumChoice)
    #[serde(default)]
    pub state: String,
    /// `nextState` (EnumChoice)
    #[serde(default)]
    pub next_state: String,
    /// `linkedStates` (Class (array))
    #[serde(default)]
    pub linked_states: Vec<Handle<SHelmetLinkedState>>,
}

impl Pooled for SHelmetStateBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.item.shelmet_state_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.item.shelmet_state_base_params }
}

impl<'a> Extract<'a> for SHelmetStateBaseParams {
    const TYPE_NAME: &'static str = "SHelmetStateBaseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: inst.get_str("state").map(String::from).unwrap_or_default(),
            next_state: inst.get_str("nextState").map(String::from).unwrap_or_default(),
            linked_states: inst.get_array("linkedStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHelmetLinkedState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHelmetLinkedState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SHelmetStateMachineParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHelmetStateMachineParams {
    /// `stateMachine` (EnumChoice)
    #[serde(default)]
    pub state_machine: String,
    /// `states` (StrongPointer (array))
    #[serde(default)]
    pub states: Vec<Handle<SHelmetStateBaseParams>>,
    /// `startState` (EnumChoice)
    #[serde(default)]
    pub start_state: String,
}

impl Pooled for SHelmetStateMachineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.item.shelmet_state_machine_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.item.shelmet_state_machine_params }
}

impl<'a> Extract<'a> for SHelmetStateMachineParams {
    const TYPE_NAME: &'static str = "SHelmetStateMachineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_machine: inst.get_str("stateMachine").map(String::from).unwrap_or_default(),
            states: inst.get_array("states")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHelmetStateBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHelmetStateBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            start_state: inst.get_str("startState").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AnimatedHelmetParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedHelmetParams {
    /// `stateMachines` (Class (array))
    #[serde(default)]
    pub state_machines: Vec<Handle<SHelmetStateMachineParams>>,
}

impl Pooled for AnimatedHelmetParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.item.animated_helmet_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.item.animated_helmet_params }
}

impl<'a> Extract<'a> for AnimatedHelmetParams {
    const TYPE_NAME: &'static str = "AnimatedHelmetParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_machines: inst.get_array("stateMachines")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SHelmetStateMachineParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SHelmetStateMachineParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

