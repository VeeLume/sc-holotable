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

/// DCB type: `SRGBA8`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRGBA8 {
    /// DCB field: `r` (Byte)
    #[serde(default)]
    pub r: u32,
    /// DCB field: `g` (Byte)
    #[serde(default)]
    pub g: u32,
    /// DCB field: `b` (Byte)
    #[serde(default)]
    pub b: u32,
    /// DCB field: `a` (Byte)
    #[serde(default)]
    pub a: u32,
}

impl Pooled for SRGBA8 {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.srgba8 }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.srgba8 }
}

impl<'a> Extract<'a> for SRGBA8 {
    const TYPE_NAME: &'static str = "SRGBA8";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r: inst.get_u32("r").unwrap_or_default(),
            g: inst.get_u32("g").unwrap_or_default(),
            b: inst.get_u32("b").unwrap_or_default(),
            a: inst.get_u32("a").unwrap_or_default(),
        }
    }
}

/// DCB type: `SRGB8`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRGB8 {
    /// DCB field: `r` (Byte)
    #[serde(default)]
    pub r: u32,
    /// DCB field: `g` (Byte)
    #[serde(default)]
    pub g: u32,
    /// DCB field: `b` (Byte)
    #[serde(default)]
    pub b: u32,
}

impl Pooled for SRGB8 {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.srgb8 }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.srgb8 }
}

impl<'a> Extract<'a> for SRGB8 {
    const TYPE_NAME: &'static str = "SRGB8";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r: inst.get_u32("r").unwrap_or_default(),
            g: inst.get_u32("g").unwrap_or_default(),
            b: inst.get_u32("b").unwrap_or_default(),
        }
    }
}

/// DCB type: `SRtpcBehaviour`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRtpcBehaviour {
}

impl Pooled for SRtpcBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.srtpc_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.srtpc_behaviour }
}

impl<'a> Extract<'a> for SRtpcBehaviour {
    const TYPE_NAME: &'static str = "SRtpcBehaviour";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SReputationStandingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStandingParams {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `description` (String)
    #[serde(default)]
    pub description: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `perkDescription` (Locale)
    #[serde(default)]
    pub perk_description: String,
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `minReputation` (Int64)
    #[serde(default)]
    pub min_reputation: i64,
    /// DCB field: `driftReputation` (Int64)
    #[serde(default)]
    pub drift_reputation: i64,
    /// DCB field: `driftTimeHours` (Single)
    #[serde(default)]
    pub drift_time_hours: f32,
    /// DCB field: `gated` (Boolean)
    #[serde(default)]
    pub gated: bool,
}

impl Pooled for SReputationStandingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_standing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_standing_params }
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
    /// DCB field: `reputationCeiling` (Int64)
    #[serde(default)]
    pub reputation_ceiling: i64,
    /// DCB field: `initialReputation` (Int64)
    #[serde(default)]
    pub initial_reputation: i64,
    /// DCB field: `standings` (Reference (array))
    #[serde(default)]
    pub standings: Vec<CigGuid>,
}

impl Pooled for SReputationStandingMapParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_standing_map_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_standing_map_params }
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

/// DCB type: `SReputationContextBBPropertyParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationContextBBPropertyParams {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `dynamicProperty` (StrongPointer)
    #[serde(default)]
    pub dynamic_property: Option<Handle<SBBDynamicPropertyBase>>,
}

impl Pooled for SReputationContextBBPropertyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_context_bbproperty_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_context_bbproperty_params }
}

impl<'a> Extract<'a> for SReputationContextBBPropertyParams {
    const TYPE_NAME: &'static str = "SReputationContextBBPropertyParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            dynamic_property: match inst.get("dynamicProperty") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBBDynamicPropertyBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBBDynamicPropertyBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SReputationScopeContextUI`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationScopeContextUI {
    /// DCB field: `scope` (Reference)
    #[serde(default)]
    pub scope: Option<CigGuid>,
    /// DCB field: `propertiesBB` (Class (array))
    #[serde(default)]
    pub properties_bb: Vec<Handle<SReputationContextBBPropertyParams>>,
}

impl Pooled for SReputationScopeContextUI {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_scope_context_ui }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_scope_context_ui }
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
    /// DCB field: `sortOrderScope` (EnumChoice)
    #[serde(default)]
    pub sort_order_scope: String,
    /// DCB field: `primaryScopeContext` (Class)
    #[serde(default)]
    pub primary_scope_context: Option<Handle<SReputationScopeContextUI>>,
    /// DCB field: `scopeContextList` (Class (array))
    #[serde(default)]
    pub scope_context_list: Vec<Handle<SReputationScopeContextUI>>,
}

impl Pooled for SReputationContextUI {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_context_ui }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_context_ui }
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

/// DCB type: `SReputationContextBBEntityListParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationContextBBEntityListParams {
    /// DCB field: `entityTabName` (Locale)
    #[serde(default)]
    pub entity_tab_name: String,
    /// DCB field: `entityType` (EnumChoice)
    #[serde(default)]
    pub entity_type: String,
}

impl Pooled for SReputationContextBBEntityListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_context_bbentity_list_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_context_bbentity_list_params }
}

impl<'a> Extract<'a> for SReputationContextBBEntityListParams {
    const TYPE_NAME: &'static str = "SReputationContextBBEntityListParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_tab_name: inst.get_str("entityTabName").map(String::from).unwrap_or_default(),
            entity_type: inst.get_str("entityType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationGlobalContextBBParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationGlobalContextBBParams {
    /// DCB field: `infoTabNames` (Locale (array))
    #[serde(default)]
    pub info_tab_names: Vec<String>,
    /// DCB field: `entityTabs` (Class (array))
    #[serde(default)]
    pub entity_tabs: Vec<Handle<SReputationContextBBEntityListParams>>,
    /// DCB field: `entitySortOrder` (EnumChoice)
    #[serde(default)]
    pub entity_sort_order: String,
}

impl Pooled for SReputationGlobalContextBBParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_global_context_bbparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_global_context_bbparams }
}

impl<'a> Extract<'a> for SReputationGlobalContextBBParams {
    const TYPE_NAME: &'static str = "SReputationGlobalContextBBParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            info_tab_names: inst.get_array("infoTabNames")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            entity_tabs: inst.get_array("entityTabs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationContextBBEntityListParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationContextBBEntityListParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            entity_sort_order: inst.get_str("entitySortOrder").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStateParams {
    /// DCB field: `Name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for SReputationStateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_state_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_state_params }
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
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_state_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_state_modifier_base }
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
    /// DCB field: `state` (Reference)
    #[serde(default)]
    pub state: Option<CigGuid>,
    /// DCB field: `modifier` (StrongPointer)
    #[serde(default)]
    pub modifier: Option<Handle<SReputationStateModifierBase>>,
}

impl Pooled for SReputationStateModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_state_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_state_modifier_params }
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
    /// DCB field: `stateModifiers` (Class (array))
    #[serde(default)]
    pub state_modifiers: Vec<Handle<SReputationStateModifierParams>>,
}

impl Pooled for SReputationStateMissionResultModifierListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_state_mission_result_modifier_list_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_state_mission_result_modifier_list_params }
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
    /// DCB field: `missionResultStateModifiers` (Class)
    #[serde(default)]
    pub mission_result_state_modifiers: Option<Handle<SReputationStateMissionResultModifierListParams>>,
}

impl Pooled for SReputationStateMissionResultModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_state_mission_result_modifier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_state_mission_result_modifier_params }
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
    /// DCB field: `scopeName` (String)
    #[serde(default)]
    pub scope_name: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `standingMap` (Class)
    #[serde(default)]
    pub standing_map: Option<Handle<SReputationStandingMapParams>>,
}

impl Pooled for SReputationScopeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_scope_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_scope_params }
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
    /// DCB field: `editorName` (String)
    #[serde(default)]
    pub editor_name: String,
    /// DCB field: `reputationAmount` (Int32)
    #[serde(default)]
    pub reputation_amount: i32,
}

impl Pooled for SReputationRewardAmount {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_reward_amount }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_reward_amount }
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

/// DCB type: `SReputationAmountParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationAmountParams {
    /// DCB field: `factionReputation` (Reference)
    #[serde(default)]
    pub faction_reputation: Option<CigGuid>,
    /// DCB field: `reputationScope` (Reference)
    #[serde(default)]
    pub reputation_scope: Option<CigGuid>,
    /// DCB field: `reward` (Reference)
    #[serde(default)]
    pub reward: Option<CigGuid>,
}

impl Pooled for SReputationAmountParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_amount_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_amount_params }
}

impl<'a> Extract<'a> for SReputationAmountParams {
    const TYPE_NAME: &'static str = "SReputationAmountParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            faction_reputation: inst.get("factionReputation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reputation_scope: inst.get("reputationScope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reward: inst.get("reward").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SReputationAmountListParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationAmountListParams {
    /// DCB field: `reputationAmounts` (Class (array))
    #[serde(default)]
    pub reputation_amounts: Vec<Handle<SReputationAmountParams>>,
}

impl Pooled for SReputationAmountListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_amount_list_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_amount_list_params }
}

impl<'a> Extract<'a> for SReputationAmountListParams {
    const TYPE_NAME: &'static str = "SReputationAmountListParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reputation_amounts: inst.get_array("reputationAmounts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationAmountParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationAmountParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStandingRewardBonusParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStandingRewardBonusParams {
    /// DCB field: `standing` (Reference)
    #[serde(default)]
    pub standing: Option<CigGuid>,
    /// DCB field: `bonusFraction` (Single)
    #[serde(default)]
    pub bonus_fraction: f32,
}

impl Pooled for SReputationStandingRewardBonusParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_standing_reward_bonus_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_standing_reward_bonus_params }
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
    /// DCB field: `factionReputation` (Reference)
    #[serde(default)]
    pub faction_reputation: Option<CigGuid>,
    /// DCB field: `reputationScope` (Reference)
    #[serde(default)]
    pub reputation_scope: Option<CigGuid>,
    /// DCB field: `rewardBonuses` (Class (array))
    #[serde(default)]
    pub reward_bonuses: Vec<Handle<SReputationStandingRewardBonusParams>>,
}

impl Pooled for SReputationMissionGiverRewardBonusParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_mission_giver_reward_bonus_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_mission_giver_reward_bonus_params }
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
    /// DCB field: `missionGiverBonuses` (Class (array))
    #[serde(default)]
    pub mission_giver_bonuses: Vec<Handle<SReputationMissionGiverRewardBonusParams>>,
}

impl Pooled for SReputationMissionRewardBonusParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_mission_reward_bonus_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_mission_reward_bonus_params }
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

/// DCB type: `SReputationMissionRequirementExpressionElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationMissionRequirementExpressionElement {
}

impl Pooled for SReputationMissionRequirementExpressionElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_mission_requirement_expression_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_mission_requirement_expression_element }
}

impl<'a> Extract<'a> for SReputationMissionRequirementExpressionElement {
    const TYPE_NAME: &'static str = "SReputationMissionRequirementExpressionElement";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SReputationMissionRequirementsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationMissionRequirementsParams {
    /// DCB field: `expression` (StrongPointer (array))
    #[serde(default)]
    pub expression: Vec<Handle<SReputationMissionRequirementExpressionElement>>,
}

impl Pooled for SReputationMissionRequirementsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_mission_requirements_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_mission_requirements_params }
}

impl<'a> Extract<'a> for SReputationMissionRequirementsParams {
    const TYPE_NAME: &'static str = "SReputationMissionRequirementsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            expression: inst.get_array("expression")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationMissionRequirementExpressionElement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationMissionRequirementExpressionElement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStandingJournalEntryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStandingJournalEntryParams {
    /// DCB field: `changeDirection` (EnumChoice)
    #[serde(default)]
    pub change_direction: String,
    /// DCB field: `standing` (Reference)
    #[serde(default)]
    pub standing: Option<CigGuid>,
    /// DCB field: `journalEntry` (Reference)
    #[serde(default)]
    pub journal_entry: Option<CigGuid>,
}

impl Pooled for SReputationStandingJournalEntryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_standing_journal_entry_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_standing_journal_entry_params }
}

impl<'a> Extract<'a> for SReputationStandingJournalEntryParams {
    const TYPE_NAME: &'static str = "SReputationStandingJournalEntryParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            change_direction: inst.get_str("changeDirection").map(String::from).unwrap_or_default(),
            standing: inst.get("standing").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            journal_entry: inst.get("journalEntry").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SReputationJournalGroupParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationJournalGroupParams {
    /// DCB field: `standingEntries` (Class (array))
    #[serde(default)]
    pub standing_entries: Vec<Handle<SReputationStandingJournalEntryParams>>,
}

impl Pooled for SReputationJournalGroupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_journal_group_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_journal_group_params }
}

impl<'a> Extract<'a> for SReputationJournalGroupParams {
    const TYPE_NAME: &'static str = "SReputationJournalGroupParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            standing_entries: inst.get_array("standingEntries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationStandingJournalEntryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationStandingJournalEntryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationJournalEntriesParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationJournalEntriesParams {
    /// DCB field: `factionReputation` (Reference)
    #[serde(default)]
    pub faction_reputation: Option<CigGuid>,
    /// DCB field: `reputationScope` (Reference)
    #[serde(default)]
    pub reputation_scope: Option<CigGuid>,
    /// DCB field: `journalGroups` (Class (array))
    #[serde(default)]
    pub journal_groups: Vec<Handle<SReputationJournalGroupParams>>,
}

impl Pooled for SReputationJournalEntriesParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_journal_entries_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_journal_entries_params }
}

impl<'a> Extract<'a> for SReputationJournalEntriesParams {
    const TYPE_NAME: &'static str = "SReputationJournalEntriesParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            faction_reputation: inst.get("factionReputation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reputation_scope: inst.get("reputationScope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            journal_groups: inst.get_array("journalGroups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationJournalGroupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationJournalGroupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationJournalEntryHandlerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationJournalEntryHandlerParams {
    /// DCB field: `reputationTypes` (Class (array))
    #[serde(default)]
    pub reputation_types: Vec<Handle<SReputationJournalEntriesParams>>,
}

impl Pooled for SReputationJournalEntryHandlerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreputation_journal_entry_handler_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreputation_journal_entry_handler_params }
}

impl<'a> Extract<'a> for SReputationJournalEntryHandlerParams {
    const TYPE_NAME: &'static str = "SReputationJournalEntryHandlerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reputation_types: inst.get_array("reputationTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationJournalEntriesParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationJournalEntriesParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SResourceTypeDefaultCargoContainers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SResourceTypeDefaultCargoContainers {
    /// DCB field: `one_eighthSCU` (Reference)
    #[serde(default)]
    pub one_eighth_scu: Option<CigGuid>,
    /// DCB field: `one_quarterSCU` (Reference)
    #[serde(default)]
    pub one_quarter_scu: Option<CigGuid>,
    /// DCB field: `one_halfSCU` (Reference)
    #[serde(default)]
    pub one_half_scu: Option<CigGuid>,
    /// DCB field: `oneSCU` (Reference)
    #[serde(default)]
    pub one_scu: Option<CigGuid>,
    /// DCB field: `twoSCU` (Reference)
    #[serde(default)]
    pub two_scu: Option<CigGuid>,
    /// DCB field: `fourSCU` (Reference)
    #[serde(default)]
    pub four_scu: Option<CigGuid>,
    /// DCB field: `eightSCU` (Reference)
    #[serde(default)]
    pub eight_scu: Option<CigGuid>,
    /// DCB field: `sixteenSCU` (Reference)
    #[serde(default)]
    pub sixteen_scu: Option<CigGuid>,
    /// DCB field: `twentyFourSCU` (Reference)
    #[serde(default)]
    pub twenty_four_scu: Option<CigGuid>,
    /// DCB field: `thirtyTwoSCU` (Reference)
    #[serde(default)]
    pub thirty_two_scu: Option<CigGuid>,
}

impl Pooled for SResourceTypeDefaultCargoContainers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sresource_type_default_cargo_containers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sresource_type_default_cargo_containers }
}

impl<'a> Extract<'a> for SResourceTypeDefaultCargoContainers {
    const TYPE_NAME: &'static str = "SResourceTypeDefaultCargoContainers";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            one_eighth_scu: inst.get("one_eighthSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            one_quarter_scu: inst.get("one_quarterSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            one_half_scu: inst.get("one_halfSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            one_scu: inst.get("oneSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            two_scu: inst.get("twoSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            four_scu: inst.get("fourSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            eight_scu: inst.get("eightSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            sixteen_scu: inst.get("sixteenSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            twenty_four_scu: inst.get("twentyFourSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            thirty_two_scu: inst.get("thirtyTwoSCU").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SRecoilModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRecoilModifier {
    /// DCB field: `decayMultiplier` (Single)
    #[serde(default)]
    pub decay_multiplier: f32,
    /// DCB field: `endDecayMultiplier` (Single)
    #[serde(default)]
    pub end_decay_multiplier: f32,
    /// DCB field: `fireRecoilTimeMultiplier` (Single)
    #[serde(default)]
    pub fire_recoil_time_multiplier: f32,
    /// DCB field: `fireRecoilStrengthFirstMultiplier` (Single)
    #[serde(default)]
    pub fire_recoil_strength_first_multiplier: f32,
    /// DCB field: `fireRecoilStrengthMultiplier` (Single)
    #[serde(default)]
    pub fire_recoil_strength_multiplier: f32,
    /// DCB field: `angleRecoilStrengthMultiplier` (Single)
    #[serde(default)]
    pub angle_recoil_strength_multiplier: f32,
    /// DCB field: `randomnessMultiplier` (Single)
    #[serde(default)]
    pub randomness_multiplier: f32,
    /// DCB field: `randomnessBackPushMultiplier` (Single)
    #[serde(default)]
    pub randomness_back_push_multiplier: f32,
    /// DCB field: `frontalOscillationRotationMultiplier` (Single)
    #[serde(default)]
    pub frontal_oscillation_rotation_multiplier: f32,
    /// DCB field: `frontalOscillationStrengthMultiplier` (Single)
    #[serde(default)]
    pub frontal_oscillation_strength_multiplier: f32,
    /// DCB field: `frontalOscillationDecayMultiplier` (Single)
    #[serde(default)]
    pub frontal_oscillation_decay_multiplier: f32,
    /// DCB field: `frontalOscillationRandomnessMultiplier` (Single)
    #[serde(default)]
    pub frontal_oscillation_randomness_multiplier: f32,
    /// DCB field: `animatedRecoilMultiplier` (Single)
    #[serde(default)]
    pub animated_recoil_multiplier: f32,
    /// DCB field: `disableRecoil` (Boolean)
    #[serde(default)]
    pub disable_recoil: bool,
    /// DCB field: `headRotationMultiplier` (Class)
    #[serde(default)]
    pub head_rotation_multiplier: Option<Handle<Vec3>>,
    /// DCB field: `aimRecoilModifier` (Class)
    #[serde(default)]
    pub aim_recoil_modifier: Option<Handle<SAimRecoilModifier>>,
    /// DCB field: `curveRecoil` (Class)
    #[serde(default)]
    pub curve_recoil: Option<Handle<SActorProceduralHandsRecoilCurveModifiersDef>>,
    /// DCB field: `curveRecoilHead` (Class)
    #[serde(default)]
    pub curve_recoil_head: Option<Handle<SWeaponProceduralHeadRecoilCurveModifierDef>>,
}

impl Pooled for SRecoilModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.srecoil_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.srecoil_modifier }
}

impl<'a> Extract<'a> for SRecoilModifier {
    const TYPE_NAME: &'static str = "SRecoilModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            decay_multiplier: inst.get_f32("decayMultiplier").unwrap_or_default(),
            end_decay_multiplier: inst.get_f32("endDecayMultiplier").unwrap_or_default(),
            fire_recoil_time_multiplier: inst.get_f32("fireRecoilTimeMultiplier").unwrap_or_default(),
            fire_recoil_strength_first_multiplier: inst.get_f32("fireRecoilStrengthFirstMultiplier").unwrap_or_default(),
            fire_recoil_strength_multiplier: inst.get_f32("fireRecoilStrengthMultiplier").unwrap_or_default(),
            angle_recoil_strength_multiplier: inst.get_f32("angleRecoilStrengthMultiplier").unwrap_or_default(),
            randomness_multiplier: inst.get_f32("randomnessMultiplier").unwrap_or_default(),
            randomness_back_push_multiplier: inst.get_f32("randomnessBackPushMultiplier").unwrap_or_default(),
            frontal_oscillation_rotation_multiplier: inst.get_f32("frontalOscillationRotationMultiplier").unwrap_or_default(),
            frontal_oscillation_strength_multiplier: inst.get_f32("frontalOscillationStrengthMultiplier").unwrap_or_default(),
            frontal_oscillation_decay_multiplier: inst.get_f32("frontalOscillationDecayMultiplier").unwrap_or_default(),
            frontal_oscillation_randomness_multiplier: inst.get_f32("frontalOscillationRandomnessMultiplier").unwrap_or_default(),
            animated_recoil_multiplier: inst.get_f32("animatedRecoilMultiplier").unwrap_or_default(),
            disable_recoil: inst.get_bool("disableRecoil").unwrap_or_default(),
            head_rotation_multiplier: match inst.get("headRotationMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            aim_recoil_modifier: match inst.get("aimRecoilModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimRecoilModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimRecoilModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            curve_recoil: match inst.get("curveRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorProceduralHandsRecoilCurveModifiersDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorProceduralHandsRecoilCurveModifiersDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            curve_recoil_head: match inst.get("curveRecoilHead") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilCurveModifierDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilCurveModifierDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SRegenConsumerModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRegenConsumerModifier {
    /// DCB field: `powerRatioMultiplier` (Single)
    #[serde(default)]
    pub power_ratio_multiplier: f32,
    /// DCB field: `maxAmmoLoadMultiplier` (Single)
    #[serde(default)]
    pub max_ammo_load_multiplier: f32,
    /// DCB field: `maxRegenPerSecMultiplier` (Single)
    #[serde(default)]
    pub max_regen_per_sec_multiplier: f32,
}

impl Pooled for SRegenConsumerModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sregen_consumer_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sregen_consumer_modifier }
}

impl<'a> Extract<'a> for SRegenConsumerModifier {
    const TYPE_NAME: &'static str = "SRegenConsumerModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            power_ratio_multiplier: inst.get_f32("powerRatioMultiplier").unwrap_or_default(),
            max_ammo_load_multiplier: inst.get_f32("maxAmmoLoadMultiplier").unwrap_or_default(),
            max_regen_per_sec_multiplier: inst.get_f32("maxRegenPerSecMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `SResourceRewardMultiplier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SResourceRewardMultiplier {
    /// DCB field: `resource` (Reference)
    #[serde(default)]
    pub resource: Option<CigGuid>,
    /// DCB field: `UECPerSCU` (Int32)
    #[serde(default)]
    pub uecper_scu: i32,
}

impl Pooled for SResourceRewardMultiplier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sresource_reward_multiplier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sresource_reward_multiplier }
}

impl<'a> Extract<'a> for SResourceRewardMultiplier {
    const TYPE_NAME: &'static str = "SResourceRewardMultiplier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            resource: inst.get("resource").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            uecper_scu: inst.get_i32("UECPerSCU").unwrap_or_default(),
        }
    }
}

/// DCB type: `SRewardPeriodical`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRewardPeriodical {
    /// DCB field: `timePeriodText` (Locale)
    #[serde(default)]
    pub time_period_text: String,
    /// DCB field: `minPoints` (Int32)
    #[serde(default)]
    pub min_points: i32,
    /// DCB field: `globalPointsMultiplier` (Single)
    #[serde(default)]
    pub global_points_multiplier: f32,
    /// DCB field: `minMultiplierThreshold` (Single)
    #[serde(default)]
    pub min_multiplier_threshold: f32,
    /// DCB field: `completionTags` (Reference)
    #[serde(default)]
    pub completion_tags: Option<CigGuid>,
    /// DCB field: `periodMissionScenario` (Reference)
    #[serde(default)]
    pub period_mission_scenario: Option<CigGuid>,
}

impl Pooled for SRewardPeriodical {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sr.sreward_periodical }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sr.sreward_periodical }
}

impl<'a> Extract<'a> for SRewardPeriodical {
    const TYPE_NAME: &'static str = "SRewardPeriodical";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            time_period_text: inst.get_str("timePeriodText").map(String::from).unwrap_or_default(),
            min_points: inst.get_i32("minPoints").unwrap_or_default(),
            global_points_multiplier: inst.get_f32("globalPointsMultiplier").unwrap_or_default(),
            min_multiplier_threshold: inst.get_f32("minMultiplierThreshold").unwrap_or_default(),
            completion_tags: inst.get("completionTags").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            period_mission_scenario: inst.get("periodMissionScenario").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

