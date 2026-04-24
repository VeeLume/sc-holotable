// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `reputation`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SandboxInfractionDefinitionTrigger`
/// Inherits from: `SandboxInfractionBaseDef`
pub struct SandboxInfractionDefinitionTrigger {
    /// `sandboxInfractionDefinitionTrigger` (Reference)
    pub sandbox_infraction_definition_trigger: Option<CigGuid>,
}

impl Pooled for SandboxInfractionDefinitionTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.reputation.sandbox_infraction_definition_trigger
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.reputation.sandbox_infraction_definition_trigger
    }
}

impl<'a> Extract<'a> for SandboxInfractionDefinitionTrigger {
    const TYPE_NAME: &'static str = "SandboxInfractionDefinitionTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sandbox_infraction_definition_trigger: inst
                .get("sandboxInfractionDefinitionTrigger")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `SReputationStateParams`
pub struct SReputationStateParams {
    /// `Name` (String)
    pub name: String,
}

impl Pooled for SReputationStateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.reputation.sreputation_state_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.reputation.sreputation_state_params
    }
}

impl<'a> Extract<'a> for SReputationStateParams {
    const TYPE_NAME: &'static str = "SReputationStateParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStateModifierIncrement`
/// Inherits from: `SReputationStateModifierBase`
pub struct SReputationStateModifierIncrement {
    /// `value` (Int32)
    pub value: i32,
}

impl Pooled for SReputationStateModifierIncrement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.reputation.sreputation_state_modifier_increment
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.reputation.sreputation_state_modifier_increment
    }
}

impl<'a> Extract<'a> for SReputationStateModifierIncrement {
    const TYPE_NAME: &'static str = "SReputationStateModifierIncrement";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: inst.get_i32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStateModifierSet`
/// Inherits from: `SReputationStateModifierBase`
pub struct SReputationStateModifierSet {
    /// `value` (Int32)
    pub value: i32,
}

impl Pooled for SReputationStateModifierSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.reputation.sreputation_state_modifier_set
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.reputation.sreputation_state_modifier_set
    }
}

impl<'a> Extract<'a> for SReputationStateModifierSet {
    const TYPE_NAME: &'static str = "SReputationStateModifierSet";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: inst.get_i32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStateModifierSetBool`
/// Inherits from: `SReputationStateModifierBase`
pub struct SReputationStateModifierSetBool {
    /// `value` (Boolean)
    pub value: bool,
}

impl Pooled for SReputationStateModifierSetBool {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.reputation.sreputation_state_modifier_set_bool
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.reputation.sreputation_state_modifier_set_bool
    }
}

impl<'a> Extract<'a> for SReputationStateModifierSetBool {
    const TYPE_NAME: &'static str = "SReputationStateModifierSetBool";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: inst.get_bool("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStateModifierSetToState`
/// Inherits from: `SReputationStateModifierBase`
pub struct SReputationStateModifierSetToState {
    /// `modifierState` (Reference)
    pub modifier_state: Option<CigGuid>,
}

impl Pooled for SReputationStateModifierSetToState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.reputation.sreputation_state_modifier_set_to_state
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.reputation.sreputation_state_modifier_set_to_state
    }
}

impl<'a> Extract<'a> for SReputationStateModifierSetToState {
    const TYPE_NAME: &'static str = "SReputationStateModifierSetToState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            modifier_state: inst
                .get("modifierState")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `SReputationStateModifierParams`
pub struct SReputationStateModifierParams {
    /// `state` (Reference)
    pub state: Option<CigGuid>,
    /// `modifier` (StrongPointer)
    pub modifier: Option<SReputationStateModifierBasePtr>,
}

impl Pooled for SReputationStateModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.reputation.sreputation_state_modifier_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.reputation.sreputation_state_modifier_params
    }
}

impl<'a> Extract<'a> for SReputationStateModifierParams {
    const TYPE_NAME: &'static str = "SReputationStateModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: inst
                .get("state")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            modifier: match inst.get("modifier") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SReputationStateModifierBasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SReputationStateMissionResultModifierListParams`
pub struct SReputationStateMissionResultModifierListParams {
    /// `stateModifiers` (Class (array))
    pub state_modifiers: Vec<Handle<SReputationStateModifierParams>>,
}

impl Pooled for SReputationStateMissionResultModifierListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .reputation
            .sreputation_state_mission_result_modifier_list_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .reputation
            .sreputation_state_mission_result_modifier_list_params
    }
}

impl<'a> Extract<'a> for SReputationStateMissionResultModifierListParams {
    const TYPE_NAME: &'static str = "SReputationStateMissionResultModifierListParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_modifiers: inst
                .get_array("stateModifiers")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SReputationStateModifierParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<SReputationStateModifierParams>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStateMissionResultModifierParams`
pub struct SReputationStateMissionResultModifierParams {
    /// `missionResultStateModifiers` (Class)
    pub mission_result_state_modifiers:
        Option<Handle<SReputationStateMissionResultModifierListParams>>,
}

impl Pooled for SReputationStateMissionResultModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .reputation
            .sreputation_state_mission_result_modifier_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .reputation
            .sreputation_state_mission_result_modifier_params
    }
}

impl<'a> Extract<'a> for SReputationStateMissionResultModifierParams {
    const TYPE_NAME: &'static str = "SReputationStateMissionResultModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_result_state_modifiers: match inst.get("missionResultStateModifiers") {
                Some(Value::Class { struct_index, data }) => Some(
                    b.alloc_nested::<SReputationStateMissionResultModifierListParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ),
                ),
                _ => None,
            },
        }
    }
}
