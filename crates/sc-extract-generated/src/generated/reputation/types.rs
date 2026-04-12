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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ReputationRewardBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationRewardBaseDef {
    /// `scope` (Reference)
    #[serde(default)]
    pub scope: Option<CigGuid>,
    /// `reward` (Reference)
    #[serde(default)]
    pub reward: Option<CigGuid>,
}

impl Pooled for ReputationRewardBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.reputation_reward_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.reputation_reward_base_def }
}

impl<'a> Extract<'a> for ReputationRewardBaseDef {
    const TYPE_NAME: &'static str = "ReputationRewardBaseDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            scope: inst.get("scope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reward: inst.get("reward").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SandboxInfractionBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxInfractionBaseDef {
}

impl Pooled for SandboxInfractionBaseDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sandbox_infraction_base_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sandbox_infraction_base_def }
}

impl<'a> Extract<'a> for SandboxInfractionBaseDef {
    const TYPE_NAME: &'static str = "SandboxInfractionBaseDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SandboxTriggerManualParams`
/// Inherits from: `SandboxTriggerBaseDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTriggerManualParams {
    /// `infractionTrigger` (StrongPointer)
    #[serde(default)]
    pub infraction_trigger: Option<Handle<SandboxInfractionBaseDef>>,
    /// `triggerOnInnocentsOnly` (Boolean)
    #[serde(default)]
    pub trigger_on_innocents_only: bool,
    /// `outcomes` (StrongPointer (array))
    #[serde(default)]
    pub outcomes: Vec<Handle<ReputationRewardBaseDef>>,
}

impl Pooled for SandboxTriggerManualParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sandbox_trigger_manual_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sandbox_trigger_manual_params }
}

impl<'a> Extract<'a> for SandboxTriggerManualParams {
    const TYPE_NAME: &'static str = "SandboxTriggerManualParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            infraction_trigger: match inst.get("infractionTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SandboxInfractionBaseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SandboxInfractionBaseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            trigger_on_innocents_only: inst.get_bool("triggerOnInnocentsOnly").unwrap_or_default(),
            outcomes: inst.get_array("outcomes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ReputationRewardBaseDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ReputationRewardBaseDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SandboxTriggerRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTriggerRecord {
    /// `triggerManualParams` (Class (array))
    #[serde(default)]
    pub trigger_manual_params: Vec<Handle<SandboxTriggerManualParams>>,
}

impl Pooled for SandboxTriggerRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sandbox_trigger_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sandbox_trigger_record }
}

impl<'a> Extract<'a> for SandboxTriggerRecord {
    const TYPE_NAME: &'static str = "SandboxTriggerRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_manual_params: inst.get_array("triggerManualParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SandboxTriggerManualParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SandboxTriggerManualParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SPerkParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPerkParamsBase {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `displayDescription` (Locale)
    #[serde(default)]
    pub display_description: String,
    /// `icon` (String)
    #[serde(default)]
    pub icon: String,
}

impl Pooled for SPerkParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sperk_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sperk_params_base }
}

impl<'a> Extract<'a> for SPerkParamsBase {
    const TYPE_NAME: &'static str = "SPerkParamsBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            display_description: inst.get_str("displayDescription").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SPerkReputationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPerkReputationParams {
    /// `scope` (Reference)
    #[serde(default)]
    pub scope: Option<CigGuid>,
    /// `standing` (Reference)
    #[serde(default)]
    pub standing: Option<CigGuid>,
    /// `perk` (StrongPointer)
    #[serde(default)]
    pub perk: Option<Handle<SPerkParamsBase>>,
}

impl Pooled for SPerkReputationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sperk_reputation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sperk_reputation_params }
}

impl<'a> Extract<'a> for SPerkReputationParams {
    const TYPE_NAME: &'static str = "SPerkReputationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope: inst.get("scope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            standing: inst.get("standing").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            perk: match inst.get("perk") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SPerkParamsBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SPerkParamsBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SPerkReputationListParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPerkReputationListParams {
    /// `perks` (Class (array))
    #[serde(default)]
    pub perks: Vec<Handle<SPerkReputationParams>>,
}

impl Pooled for SPerkReputationListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sperk_reputation_list_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sperk_reputation_list_params }
}

impl<'a> Extract<'a> for SPerkReputationListParams {
    const TYPE_NAME: &'static str = "SPerkReputationListParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            perks: inst.get_array("perks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SPerkReputationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SPerkReputationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStandingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStandingParams {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `perkDescription` (Locale)
    #[serde(default)]
    pub perk_description: String,
    /// `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// `minReputation` (Int64)
    #[serde(default)]
    pub min_reputation: i64,
    /// `driftReputation` (Int64)
    #[serde(default)]
    pub drift_reputation: i64,
    /// `driftTimeHours` (Single)
    #[serde(default)]
    pub drift_time_hours: f32,
    /// `gated` (Boolean)
    #[serde(default)]
    pub gated: bool,
}

impl Pooled for SReputationStandingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_standing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_standing_params }
}

impl<'a> Extract<'a> for SReputationStandingParams {
    const TYPE_NAME: &'static str = "SReputationStandingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            perk_description: inst.get_str("perkDescription").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            min_reputation: inst.get_i64("minReputation").unwrap_or_default(),
            drift_reputation: inst.get_i64("driftReputation").unwrap_or_default(),
            drift_time_hours: inst.get_f32("driftTimeHours").unwrap_or_default(),
            gated: inst.get_bool("gated").unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStandingMapParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStandingMapParams {
    /// `reputationCeiling` (Int64)
    #[serde(default)]
    pub reputation_ceiling: i64,
    /// `initialReputation` (Int64)
    #[serde(default)]
    pub initial_reputation: i64,
    /// `standings` (Reference (array))
    #[serde(default)]
    pub standings: Vec<CigGuid>,
}

impl Pooled for SReputationStandingMapParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_standing_map_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_standing_map_params }
}

impl<'a> Extract<'a> for SReputationStandingMapParams {
    const TYPE_NAME: &'static str = "SReputationStandingMapParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reputation_ceiling: inst.get_i64("reputationCeiling").unwrap_or_default(),
            initial_reputation: inst.get_i64("initialReputation").unwrap_or_default(),
            standings: inst.get_array("standings")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationScopeContextUI`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationScopeContextUI {
    /// `scope` (Reference)
    #[serde(default)]
    pub scope: Option<CigGuid>,
    /// `propertiesBB` (Class (array))
    #[serde(default)]
    pub properties_bb: Vec<Handle<SReputationContextBBPropertyParams>>,
}

impl Pooled for SReputationScopeContextUI {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_scope_context_ui }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_scope_context_ui }
}

impl<'a> Extract<'a> for SReputationScopeContextUI {
    const TYPE_NAME: &'static str = "SReputationScopeContextUI";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope: inst.get("scope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            properties_bb: inst.get_array("propertiesBB")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationContextBBPropertyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationContextBBPropertyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationContextUI`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationContextUI {
    /// `sortOrderScope` (EnumChoice)
    #[serde(default)]
    pub sort_order_scope: String,
    /// `primaryScopeContext` (Class)
    #[serde(default)]
    pub primary_scope_context: Option<Handle<SReputationScopeContextUI>>,
    /// `scopeContextList` (Class (array))
    #[serde(default)]
    pub scope_context_list: Vec<Handle<SReputationScopeContextUI>>,
}

impl Pooled for SReputationContextUI {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_context_ui }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_context_ui }
}

impl<'a> Extract<'a> for SReputationContextUI {
    const TYPE_NAME: &'static str = "SReputationContextUI";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sort_order_scope: inst.get_str("sortOrderScope").map(String::from).unwrap_or_default(),
            primary_scope_context: match inst.get("primaryScopeContext") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SReputationScopeContextUI>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SReputationScopeContextUI>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scope_context_list: inst.get_array("scopeContextList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationScopeContextUI>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationScopeContextUI>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStateParams {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for SReputationStateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_state_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_state_params }
}

impl<'a> Extract<'a> for SReputationStateParams {
    const TYPE_NAME: &'static str = "SReputationStateParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStateModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStateModifierBase {
}

impl Pooled for SReputationStateModifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_state_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_state_modifier_base }
}

impl<'a> Extract<'a> for SReputationStateModifierBase {
    const TYPE_NAME: &'static str = "SReputationStateModifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SReputationStateModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStateModifierParams {
    /// `state` (Reference)
    #[serde(default)]
    pub state: Option<CigGuid>,
    /// `modifier` (StrongPointer)
    #[serde(default)]
    pub modifier: Option<Handle<SReputationStateModifierBase>>,
}

impl Pooled for SReputationStateModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_state_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_state_modifier_params }
}

impl<'a> Extract<'a> for SReputationStateModifierParams {
    const TYPE_NAME: &'static str = "SReputationStateModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: inst.get("state").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            modifier: match inst.get("modifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SReputationStateModifierBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SReputationStateModifierBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SReputationStateMissionResultModifierListParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStateMissionResultModifierListParams {
    /// `stateModifiers` (Class (array))
    #[serde(default)]
    pub state_modifiers: Vec<Handle<SReputationStateModifierParams>>,
}

impl Pooled for SReputationStateMissionResultModifierListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_state_mission_result_modifier_list_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_state_mission_result_modifier_list_params }
}

impl<'a> Extract<'a> for SReputationStateMissionResultModifierListParams {
    const TYPE_NAME: &'static str = "SReputationStateMissionResultModifierListParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_modifiers: inst.get_array("stateModifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationStateModifierParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationStateModifierParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStateMissionResultModifierParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStateMissionResultModifierParams {
    /// `missionResultStateModifiers` (Class)
    #[serde(default)]
    pub mission_result_state_modifiers: Option<Handle<SReputationStateMissionResultModifierListParams>>,
}

impl Pooled for SReputationStateMissionResultModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_state_mission_result_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_state_mission_result_modifier_params }
}

impl<'a> Extract<'a> for SReputationStateMissionResultModifierParams {
    const TYPE_NAME: &'static str = "SReputationStateMissionResultModifierParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_result_state_modifiers: match inst.get("missionResultStateModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SReputationStateMissionResultModifierListParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SReputationStateMissionResultModifierListParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SReputationScopeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationScopeParams {
    /// `scopeName` (String)
    #[serde(default)]
    pub scope_name: String,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// `standingMap` (Class)
    #[serde(default)]
    pub standing_map: Option<Handle<SReputationStandingMapParams>>,
}

impl Pooled for SReputationScopeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_scope_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_scope_params }
}

impl<'a> Extract<'a> for SReputationScopeParams {
    const TYPE_NAME: &'static str = "SReputationScopeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope_name: inst.get_str("scopeName").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            standing_map: match inst.get("standingMap") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SReputationStandingMapParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SReputationStandingMapParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SReputationRewardAmount`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationRewardAmount {
    /// `editorName` (String)
    #[serde(default)]
    pub editor_name: String,
    /// `reputationAmount` (Int32)
    #[serde(default)]
    pub reputation_amount: i32,
}

impl Pooled for SReputationRewardAmount {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_reward_amount }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_reward_amount }
}

impl<'a> Extract<'a> for SReputationRewardAmount {
    const TYPE_NAME: &'static str = "SReputationRewardAmount";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            editor_name: inst.get_str("editorName").map(String::from).unwrap_or_default(),
            reputation_amount: inst.get_i32("reputationAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStandingRewardBonusParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStandingRewardBonusParams {
    /// `standing` (Reference)
    #[serde(default)]
    pub standing: Option<CigGuid>,
    /// `bonusFraction` (Single)
    #[serde(default)]
    pub bonus_fraction: f32,
}

impl Pooled for SReputationStandingRewardBonusParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_standing_reward_bonus_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_standing_reward_bonus_params }
}

impl<'a> Extract<'a> for SReputationStandingRewardBonusParams {
    const TYPE_NAME: &'static str = "SReputationStandingRewardBonusParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            standing: inst.get("standing").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            bonus_fraction: inst.get_f32("bonusFraction").unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationMissionGiverRewardBonusParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationMissionGiverRewardBonusParams {
    /// `factionReputation` (Reference)
    #[serde(default)]
    pub faction_reputation: Option<CigGuid>,
    /// `reputationScope` (Reference)
    #[serde(default)]
    pub reputation_scope: Option<CigGuid>,
    /// `rewardBonuses` (Class (array))
    #[serde(default)]
    pub reward_bonuses: Vec<Handle<SReputationStandingRewardBonusParams>>,
}

impl Pooled for SReputationMissionGiverRewardBonusParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_mission_giver_reward_bonus_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_mission_giver_reward_bonus_params }
}

impl<'a> Extract<'a> for SReputationMissionGiverRewardBonusParams {
    const TYPE_NAME: &'static str = "SReputationMissionGiverRewardBonusParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            faction_reputation: inst.get("factionReputation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reputation_scope: inst.get("reputationScope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reward_bonuses: inst.get_array("rewardBonuses")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationStandingRewardBonusParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationStandingRewardBonusParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationMissionRewardBonusParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationMissionRewardBonusParams {
    /// `missionGiverBonuses` (Class (array))
    #[serde(default)]
    pub mission_giver_bonuses: Vec<Handle<SReputationMissionGiverRewardBonusParams>>,
}

impl Pooled for SReputationMissionRewardBonusParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.reputation.sreputation_mission_reward_bonus_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.reputation.sreputation_mission_reward_bonus_params }
}

impl<'a> Extract<'a> for SReputationMissionRewardBonusParams {
    const TYPE_NAME: &'static str = "SReputationMissionRewardBonusParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_giver_bonuses: inst.get_array("missionGiverBonuses")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationMissionGiverRewardBonusParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationMissionGiverRewardBonusParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

