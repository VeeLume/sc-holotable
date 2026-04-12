// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-playerdefaultactionsconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `DefaultActionDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionDef {
}

impl Pooled for DefaultActionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playerdefaultactionsconfig.default_action_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playerdefaultactionsconfig.default_action_def }
}

impl<'a> Extract<'a> for DefaultActionDef {
    const TYPE_NAME: &'static str = "DefaultActionDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultActionsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsParams {
    /// `states` (StrongPointer (array))
    #[serde(default)]
    pub states: Vec<Handle<DefaultActionsEntityState>>,
    /// `defaultActions` (StrongPointer)
    #[serde(default)]
    pub default_actions: Option<Handle<DefaultActionDef>>,
}

impl Pooled for DefaultActionsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playerdefaultactionsconfig.default_actions_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playerdefaultactionsconfig.default_actions_params }
}

impl<'a> Extract<'a> for DefaultActionsParams {
    const TYPE_NAME: &'static str = "DefaultActionsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            states: inst.get_array("states")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionsEntityState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionsEntityState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_actions: match inst.get("defaultActions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DefaultActionDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DefaultActionDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultActions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActions {
    /// `defaultActionsPerState` (Class (array))
    #[serde(default)]
    pub default_actions_per_state: Vec<Handle<DefaultActionsParams>>,
}

impl Pooled for DefaultActions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playerdefaultactionsconfig.default_actions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playerdefaultactionsconfig.default_actions }
}

impl<'a> Extract<'a> for DefaultActions {
    const TYPE_NAME: &'static str = "DefaultActions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_actions_per_state: inst.get_array("defaultActionsPerState")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionDescriptionOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionDescriptionOverride {
    /// `action` (String)
    #[serde(default)]
    pub action: String,
    /// `actionsDescription` (Locale)
    #[serde(default)]
    pub actions_description: String,
}

impl Pooled for DefaultActionDescriptionOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playerdefaultactionsconfig.default_action_description_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playerdefaultactionsconfig.default_action_description_override }
}

impl<'a> Extract<'a> for DefaultActionDescriptionOverride {
    const TYPE_NAME: &'static str = "DefaultActionDescriptionOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            action: inst.get_str("action").map(String::from).unwrap_or_default(),
            actions_description: inst.get_str("actionsDescription").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsEntry {
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `conditions` (StrongPointer (array))
    #[serde(default)]
    pub conditions: Vec<Handle<DefaultActionsEntityEntryCondition>>,
    /// `defaultActions` (Class)
    #[serde(default)]
    pub default_actions: Option<Handle<DefaultActions>>,
}

impl Pooled for DefaultActionsEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playerdefaultactionsconfig.default_actions_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playerdefaultactionsconfig.default_actions_entry }
}

impl<'a> Extract<'a> for DefaultActionsEntry {
    const TYPE_NAME: &'static str = "DefaultActionsEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionsEntityEntryCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionsEntityEntryCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_actions: match inst.get("defaultActions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DefaultActions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DefaultActions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorDefaultActionsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorDefaultActionsConfig {
    /// `versionID` (Int32)
    #[serde(default)]
    pub version_id: i32,
    /// `defaultActionsList` (Class (array))
    #[serde(default)]
    pub default_actions_list: Vec<Handle<DefaultActionsEntry>>,
    /// `defaultActionsDescriptions` (Class (array))
    #[serde(default)]
    pub default_actions_descriptions: Vec<Handle<DefaultActionDescriptionOverride>>,
}

impl Pooled for ActorDefaultActionsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playerdefaultactionsconfig.actor_default_actions_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playerdefaultactionsconfig.actor_default_actions_config }
}

impl<'a> Extract<'a> for ActorDefaultActionsConfig {
    const TYPE_NAME: &'static str = "ActorDefaultActionsConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            version_id: inst.get_i32("versionID").unwrap_or_default(),
            default_actions_list: inst.get_array("defaultActionsList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionsEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionsEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_actions_descriptions: inst.get_array("defaultActionsDescriptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DefaultActionDescriptionOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DefaultActionDescriptionOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsEntityEntryCondition {
}

impl Pooled for DefaultActionsEntityEntryCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playerdefaultactionsconfig.default_actions_entity_entry_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playerdefaultactionsconfig.default_actions_entity_entry_condition }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `DefaultActionsEntityState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultActionsEntityState {
}

impl Pooled for DefaultActionsEntityState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_playerdefaultactionsconfig.default_actions_entity_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_playerdefaultactionsconfig.default_actions_entity_state }
}

impl<'a> Extract<'a> for DefaultActionsEntityState {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

