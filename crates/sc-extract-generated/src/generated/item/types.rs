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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `GameTokens`
pub struct GameTokens {
    /// `GameTokenLibraries` (String (array))
    pub game_token_libraries: Vec<String>,
    /// `FlowGraphs` (String (array))
    pub flow_graphs: Vec<String>,
}

impl Pooled for GameTokens {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.item.game_tokens
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.item.game_tokens
    }
}

impl<'a> Extract<'a> for GameTokens {
    const TYPE_NAME: &'static str = "GameTokens";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            game_token_libraries: inst
                .get_array("GameTokenLibraries")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            flow_graphs: inst
                .get_array("FlowGraphs")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Item`
pub struct Item {
    /// `type` (StrongPointer)
    pub r#type: Option<BaseItemPtr>,
}

impl Pooled for Item {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.item.item
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.item.item
    }
}

impl<'a> Extract<'a> for Item {
    const TYPE_NAME: &'static str = "Item";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: match inst.get("type") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(BaseItemPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `VehicleItemInteriorController`
/// Inherits from: `VehicleItem`
pub struct VehicleItemInteriorController {
    /// `GameTokenList` (Class)
    pub game_token_list: Option<Handle<GameTokens>>,
}

impl Pooled for VehicleItemInteriorController {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.item.vehicle_item_interior_controller
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.item.vehicle_item_interior_controller
    }
}

impl<'a> Extract<'a> for VehicleItemInteriorController {
    const TYPE_NAME: &'static str = "VehicleItemInteriorController";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            game_token_list: match inst.get("GameTokenList") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GameTokens>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `SHelmetLinkedState`
pub struct SHelmetLinkedState {
    /// `stateMachine` (EnumChoice)
    pub state_machine: EHelmetStateMachine,
    /// `stateToEnter` (EnumChoice)
    pub state_to_enter: EHelmetState,
}

impl Pooled for SHelmetLinkedState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.item.shelmet_linked_state
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.item.shelmet_linked_state
    }
}

impl<'a> Extract<'a> for SHelmetLinkedState {
    const TYPE_NAME: &'static str = "SHelmetLinkedState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state_machine: EHelmetStateMachine::from_dcb_str(
                inst.get_str("stateMachine").unwrap_or(""),
            ),
            state_to_enter: EHelmetState::from_dcb_str(inst.get_str("stateToEnter").unwrap_or("")),
        }
    }
}

/// DCB type: `SHelmetStateBaseParams`
pub struct SHelmetStateBaseParams {
    /// `state` (EnumChoice)
    pub state: EHelmetState,
    /// `nextState` (EnumChoice)
    pub next_state: EHelmetState,
    /// `linkedStates` (Class (array))
    pub linked_states: Vec<Handle<SHelmetLinkedState>>,
}

impl Pooled for SHelmetStateBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.item.shelmet_state_base_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.item.shelmet_state_base_params
    }
}

impl<'a> Extract<'a> for SHelmetStateBaseParams {
    const TYPE_NAME: &'static str = "SHelmetStateBaseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: EHelmetState::from_dcb_str(inst.get_str("state").unwrap_or("")),
            next_state: EHelmetState::from_dcb_str(inst.get_str("nextState").unwrap_or("")),
            linked_states: inst
                .get_array("linkedStates")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SHelmetLinkedState>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SHelmetLinkedState>(
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

/// DCB type: `SHelmetStateMachineParams`
pub struct SHelmetStateMachineParams {
    /// `stateMachine` (EnumChoice)
    pub state_machine: EHelmetStateMachine,
    /// `states` (StrongPointer (array))
    pub states: Vec<SHelmetStateBaseParamsPtr>,
    /// `startState` (EnumChoice)
    pub start_state: EHelmetState,
}

impl Pooled for SHelmetStateMachineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.item.shelmet_state_machine_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.item.shelmet_state_machine_params
    }
}

impl<'a> Extract<'a> for SHelmetStateMachineParams {
    const TYPE_NAME: &'static str = "SHelmetStateMachineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_machine: EHelmetStateMachine::from_dcb_str(
                inst.get_str("stateMachine").unwrap_or(""),
            ),
            states: inst
                .get_array("states")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(SHelmetStateBaseParamsPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            start_state: EHelmetState::from_dcb_str(inst.get_str("startState").unwrap_or("")),
        }
    }
}

/// DCB type: `AnimatedHelmetParams`
pub struct AnimatedHelmetParams {
    /// `stateMachines` (Class (array))
    pub state_machines: Vec<Handle<SHelmetStateMachineParams>>,
}

impl Pooled for AnimatedHelmetParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.item.animated_helmet_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.item.animated_helmet_params
    }
}

impl<'a> Extract<'a> for AnimatedHelmetParams {
    const TYPE_NAME: &'static str = "AnimatedHelmetParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_machines: inst
                .get_array("stateMachines")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SHelmetStateMachineParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SHelmetStateMachineParams>(
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
