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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `DefaultAction_InteractionDef`
/// Inherits from: `DefaultActionDef`
pub struct DefaultAction_InteractionDef {
    /// `interactionName` (String)
    pub interaction_name: String,
}

impl Pooled for DefaultAction_InteractionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_action_interaction_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_action_interaction_def
    }
}

impl<'a> Extract<'a> for DefaultAction_InteractionDef {
    const TYPE_NAME: &'static str = "DefaultAction_InteractionDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            interaction_name: inst
                .get_str("interactionName")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultAction_CarryableInteractionDef`
/// Inherits from: `DefaultActionDef`
pub struct DefaultAction_CarryableInteractionDef {
    /// `carryableInteraction` (EnumChoice)
    pub carryable_interaction: ECarryableDefaultInteractions,
}

impl Pooled for DefaultAction_CarryableInteractionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_action_carryable_interaction_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_action_carryable_interaction_def
    }
}

impl<'a> Extract<'a> for DefaultAction_CarryableInteractionDef {
    const TYPE_NAME: &'static str = "DefaultAction_CarryableInteractionDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            carryable_interaction: ECarryableDefaultInteractions::from_dcb_str(
                inst.get_str("carryableInteraction").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `DefaultAction_LootingInteractionDef`
/// Inherits from: `DefaultActionDef`
pub struct DefaultAction_LootingInteractionDef {
    /// `lootingInteraction` (EnumChoice)
    pub looting_interaction: ELootingDefaultInteractions,
}

impl Pooled for DefaultAction_LootingInteractionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_action_looting_interaction_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_action_looting_interaction_def
    }
}

impl<'a> Extract<'a> for DefaultAction_LootingInteractionDef {
    const TYPE_NAME: &'static str = "DefaultAction_LootingInteractionDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            looting_interaction: ELootingDefaultInteractions::from_dcb_str(
                inst.get_str("lootingInteraction").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `DefaultAction_ActionDef`
/// Inherits from: `DefaultActionDef`
pub struct DefaultAction_ActionDef {
    /// `name` (Class)
    pub name: Option<Handle<InputAction>>,
    /// `trigger` (Boolean)
    pub trigger: bool,
}

impl Pooled for DefaultAction_ActionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_action_action_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_action_action_def
    }
}

impl<'a> Extract<'a> for DefaultAction_ActionDef {
    const TYPE_NAME: &'static str = "DefaultAction_ActionDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: match inst.get("name") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            trigger: inst.get_bool("trigger").unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsParams`
pub struct DefaultActionsParams {
    /// `states` (StrongPointer (array))
    pub states: Vec<DefaultActionsEntityStatePtr>,
    /// `defaultActions` (StrongPointer)
    pub default_actions: Option<DefaultActionDefPtr>,
}

impl Pooled for DefaultActionsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_params
    }
}

impl<'a> Extract<'a> for DefaultActionsParams {
    const TYPE_NAME: &'static str = "DefaultActionsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            states: inst
                .get_array("states")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(DefaultActionsEntityStatePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_actions: match inst.get("defaultActions") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(DefaultActionDefPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultActions`
pub struct DefaultActions {
    /// `defaultActionsPerState` (Class (array))
    pub default_actions_per_state: Vec<Handle<DefaultActionsParams>>,
}

impl Pooled for DefaultActions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.actor_playerdefaultactionsconfig.default_actions
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.actor_playerdefaultactionsconfig.default_actions
    }
}

impl<'a> Extract<'a> for DefaultActions {
    const TYPE_NAME: &'static str = "DefaultActions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_actions_per_state: inst
                .get_array("defaultActionsPerState")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<DefaultActionsParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<DefaultActionsParams>(
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

/// DCB type: `DefaultActionDescriptionOverride`
pub struct DefaultActionDescriptionOverride {
    /// `action` (String)
    pub action: String,
    /// `actionsDescription` (Locale)
    pub actions_description: LocaleKey,
}

impl Pooled for DefaultActionDescriptionOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_action_description_override
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_action_description_override
    }
}

impl<'a> Extract<'a> for DefaultActionDescriptionOverride {
    const TYPE_NAME: &'static str = "DefaultActionDescriptionOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            action: inst.get_str("action").map(String::from).unwrap_or_default(),
            actions_description: inst
                .get_str("actionsDescription")
                .map(LocaleKey::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntry`
pub struct DefaultActionsEntry {
    /// `description` (String)
    pub description: String,
    /// `conditions` (StrongPointer (array))
    pub conditions: Vec<DefaultActionsEntityEntryConditionPtr>,
    /// `defaultActions` (Class)
    pub default_actions: Option<Handle<DefaultActions>>,
}

impl Pooled for DefaultActionsEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.actor_playerdefaultactionsconfig.default_actions_entry
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.actor_playerdefaultactionsconfig.default_actions_entry
    }
}

impl<'a> Extract<'a> for DefaultActionsEntry {
    const TYPE_NAME: &'static str = "DefaultActionsEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst
                .get_str("description")
                .map(String::from)
                .unwrap_or_default(),
            conditions: inst
                .get_array("conditions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(DefaultActionsEntityEntryConditionPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_actions: match inst.get("defaultActions") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<DefaultActions>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `ActorDefaultActionsConfig`
pub struct ActorDefaultActionsConfig {
    /// `versionID` (Int32)
    pub version_id: i32,
    /// `defaultActionsList` (Class (array))
    pub default_actions_list: Vec<Handle<DefaultActionsEntry>>,
    /// `defaultActionsDescriptions` (Class (array))
    pub default_actions_descriptions: Vec<Handle<DefaultActionDescriptionOverride>>,
}

impl Pooled for ActorDefaultActionsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .actor_default_actions_config
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .actor_default_actions_config
    }
}

impl<'a> Extract<'a> for ActorDefaultActionsConfig {
    const TYPE_NAME: &'static str = "ActorDefaultActionsConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            version_id: inst.get_i32("versionID").unwrap_or_default(),
            default_actions_list: inst
                .get_array("defaultActionsList")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<DefaultActionsEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<DefaultActionsEntry>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_actions_descriptions: inst
                .get_array("defaultActionsDescriptions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<DefaultActionDescriptionOverride>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<DefaultActionDescriptionOverride>(
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

/// DCB type: `DefaultActionsEntityEntryCondition_AND`
/// Inherits from: `DefaultActionsEntityEntryCondition`
pub struct DefaultActionsEntityEntryCondition_AND {
    /// `conditions` (StrongPointer (array))
    pub conditions: Vec<DefaultActionsEntityEntryConditionPtr>,
}

impl Pooled for DefaultActionsEntityEntryCondition_AND {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_and
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_and
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition_AND {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition_AND";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            conditions: inst
                .get_array("conditions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(DefaultActionsEntityEntryConditionPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryCondition_OR`
/// Inherits from: `DefaultActionsEntityEntryCondition`
pub struct DefaultActionsEntityEntryCondition_OR {
    /// `conditions` (StrongPointer (array))
    pub conditions: Vec<DefaultActionsEntityEntryConditionPtr>,
}

impl Pooled for DefaultActionsEntityEntryCondition_OR {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_or
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_or
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition_OR {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition_OR";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            conditions: inst
                .get_array("conditions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(DefaultActionsEntityEntryConditionPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryCondition_AttachableItems`
/// Inherits from: `DefaultActionsEntityEntryCondition`
pub struct DefaultActionsEntityEntryCondition_AttachableItems {
    /// `type` (EnumChoice)
    pub r#type: EItemType,
    /// `subType` (EnumChoice)
    pub sub_type: EItemSubType,
}

impl Pooled for DefaultActionsEntityEntryCondition_AttachableItems {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_attachable_items
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_attachable_items
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition_AttachableItems {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition_AttachableItems";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: EItemType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            sub_type: EItemSubType::from_dcb_str(inst.get_str("subType").unwrap_or("")),
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryCondition_EntityTypes`
/// Inherits from: `DefaultActionsEntityEntryCondition`
pub struct DefaultActionsEntityEntryCondition_EntityTypes {
    /// `type` (EnumChoice)
    pub r#type: EDefaultActionsEntityType,
}

impl Pooled for DefaultActionsEntityEntryCondition_EntityTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_entity_types
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_entity_types
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition_EntityTypes {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition_EntityTypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: EDefaultActionsEntityType::from_dcb_str(inst.get_str("type").unwrap_or("")),
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryCondition_Customisable`
/// Inherits from: `DefaultActionsEntityEntryCondition`
pub struct DefaultActionsEntityEntryCondition_Customisable {
    /// `isCustomisable` (Boolean)
    pub is_customisable: bool,
}

impl Pooled for DefaultActionsEntityEntryCondition_Customisable {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_customisable
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_customisable
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition_Customisable {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition_Customisable";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            is_customisable: inst.get_bool("isCustomisable").unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryCondition_Primed`
/// Inherits from: `DefaultActionsEntityEntryCondition`
pub struct DefaultActionsEntityEntryCondition_Primed {
    /// `isPrimed` (Boolean)
    pub is_primed: bool,
}

impl Pooled for DefaultActionsEntityEntryCondition_Primed {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_primed
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_primed
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition_Primed {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition_Primed";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            is_primed: inst.get_bool("isPrimed").unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryCondition_Tags`
/// Inherits from: `DefaultActionsEntityEntryCondition`
pub struct DefaultActionsEntityEntryCondition_Tags {
    /// `hasTags` (Boolean)
    pub has_tags: bool,
    /// `tags` (Class)
    pub tags: Option<Handle<TagList>>,
}

impl Pooled for DefaultActionsEntityEntryCondition_Tags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_tags
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_tags
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryCondition_Tags {
    const TYPE_NAME: &'static str = "DefaultActionsEntityEntryCondition_Tags";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            has_tags: inst.get_bool("hasTags").unwrap_or_default(),
            tags: match inst.get("tags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultActionsEntityEntryConditionDef_InventoryContainerCapacity`
/// Inherits from: `DefaultActionsEntityEntryCondition`
pub struct DefaultActionsEntityEntryConditionDef_InventoryContainerCapacity {
    /// `minCapacity` (StrongPointer)
    pub min_capacity: Option<SBaseCargoUnitPtr>,
    /// `maxCapacity` (StrongPointer)
    pub max_capacity: Option<SBaseCargoUnitPtr>,
}

impl Pooled for DefaultActionsEntityEntryConditionDef_InventoryContainerCapacity {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_def_inventory_container_capacity
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_entry_condition_def_inventory_container_capacity
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityEntryConditionDef_InventoryContainerCapacity {
    const TYPE_NAME: &'static str =
        "DefaultActionsEntityEntryConditionDef_InventoryContainerCapacity";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_capacity: match inst.get("minCapacity") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SBaseCargoUnitPtr::from_ref(b, r))
                }
                _ => None,
            },
            max_capacity: match inst.get("maxCapacity") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SBaseCargoUnitPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultActionsEntityState_AND`
/// Inherits from: `DefaultActionsEntityState`
pub struct DefaultActionsEntityState_AND {
    /// `states` (StrongPointer (array))
    pub states: Vec<DefaultActionsEntityStatePtr>,
}

impl Pooled for DefaultActionsEntityState_AND {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_and
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_and
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityState_AND {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState_AND";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            states: inst
                .get_array("states")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(DefaultActionsEntityStatePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntityState_OR`
/// Inherits from: `DefaultActionsEntityState`
pub struct DefaultActionsEntityState_OR {
    /// `states` (StrongPointer (array))
    pub states: Vec<DefaultActionsEntityStatePtr>,
}

impl Pooled for DefaultActionsEntityState_OR {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_or
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_or
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityState_OR {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState_OR";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            states: inst
                .get_array("states")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(DefaultActionsEntityStatePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DefaultActionsEntityState_NOT`
/// Inherits from: `DefaultActionsEntityState`
pub struct DefaultActionsEntityState_NOT {
    /// `state` (StrongPointer)
    pub state: Option<DefaultActionsEntityStatePtr>,
}

impl Pooled for DefaultActionsEntityState_NOT {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_not
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_not
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityState_NOT {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState_NOT";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state: match inst.get("state") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(DefaultActionsEntityStatePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `DefaultActionsEntityState_CarryableState`
/// Inherits from: `DefaultActionsEntityState`
pub struct DefaultActionsEntityState_CarryableState {
    /// `state` (EnumChoice)
    pub state: EActorActionEntityCarryableState,
}

impl Pooled for DefaultActionsEntityState_CarryableState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_carryable_state
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_carryable_state
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityState_CarryableState {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState_CarryableState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            state: EActorActionEntityCarryableState::from_dcb_str(
                inst.get_str("state").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `DefaultActionsEntityState_InteractionStateMachineStateTag`
/// Inherits from: `DefaultActionsEntityState`
pub struct DefaultActionsEntityState_InteractionStateMachineStateTag {
    /// `interactionStateMachineStateTag` (Reference)
    pub interaction_state_machine_state_tag: Option<CigGuid>,
}

impl Pooled for DefaultActionsEntityState_InteractionStateMachineStateTag {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_interaction_state_machine_state_tag
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_interaction_state_machine_state_tag
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityState_InteractionStateMachineStateTag {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState_InteractionStateMachineStateTag";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            interaction_state_machine_state_tag: inst
                .get("interactionStateMachineStateTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `DefaultActionsEntityState_InTakedownRange`
/// Inherits from: `DefaultActionsEntityState`
pub struct DefaultActionsEntityState_InTakedownRange {}

impl Pooled for DefaultActionsEntityState_InTakedownRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_in_takedown_range
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_in_takedown_range
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityState_InTakedownRange {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState_InTakedownRange";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `DefaultActionsEntityState_CanAttachToHeldWeapon`
/// Inherits from: `DefaultActionsEntityState`
pub struct DefaultActionsEntityState_CanAttachToHeldWeapon {}

impl Pooled for DefaultActionsEntityState_CanAttachToHeldWeapon {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_can_attach_to_held_weapon
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_can_attach_to_held_weapon
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityState_CanAttachToHeldWeapon {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState_CanAttachToHeldWeapon";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `DefaultActionsEntityState_HasAvailableCommsTap`
/// Inherits from: `DefaultActionsEntityState`
pub struct DefaultActionsEntityState_HasAvailableCommsTap {}

impl Pooled for DefaultActionsEntityState_HasAvailableCommsTap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_has_available_comms_tap
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .actor_playerdefaultactionsconfig
            .default_actions_entity_state_has_available_comms_tap
    }
}

impl<'a> Extract<'a> for DefaultActionsEntityState_HasAvailableCommsTap {
    const TYPE_NAME: &'static str = "DefaultActionsEntityState_HasAvailableCommsTap";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
