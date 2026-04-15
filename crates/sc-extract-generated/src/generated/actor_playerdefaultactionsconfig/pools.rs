// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `actor-playerdefaultactionsconfig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorPlayerdefaultactionsconfigPools {
    #[serde(default)]
    pub default_action_interaction_def: Vec<Option<DefaultAction_InteractionDef>>,
    #[serde(default)]
    pub default_action_carryable_interaction_def: Vec<Option<DefaultAction_CarryableInteractionDef>>,
    #[serde(default)]
    pub default_action_looting_interaction_def: Vec<Option<DefaultAction_LootingInteractionDef>>,
    #[serde(default)]
    pub default_action_action_def: Vec<Option<DefaultAction_ActionDef>>,
    #[serde(default)]
    pub default_actions_params: Vec<Option<DefaultActionsParams>>,
    #[serde(default)]
    pub default_actions: Vec<Option<DefaultActions>>,
    #[serde(default)]
    pub default_action_description_override: Vec<Option<DefaultActionDescriptionOverride>>,
    #[serde(default)]
    pub default_actions_entry: Vec<Option<DefaultActionsEntry>>,
    #[serde(default)]
    pub actor_default_actions_config: Vec<Option<ActorDefaultActionsConfig>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_and: Vec<Option<DefaultActionsEntityEntryCondition_AND>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_or: Vec<Option<DefaultActionsEntityEntryCondition_OR>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_attachable_items: Vec<Option<DefaultActionsEntityEntryCondition_AttachableItems>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_entity_types: Vec<Option<DefaultActionsEntityEntryCondition_EntityTypes>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_customisable: Vec<Option<DefaultActionsEntityEntryCondition_Customisable>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_primed: Vec<Option<DefaultActionsEntityEntryCondition_Primed>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_tags: Vec<Option<DefaultActionsEntityEntryCondition_Tags>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_def_inventory_container_capacity: Vec<Option<DefaultActionsEntityEntryConditionDef_InventoryContainerCapacity>>,
    #[serde(default)]
    pub default_actions_entity_state_and: Vec<Option<DefaultActionsEntityState_AND>>,
    #[serde(default)]
    pub default_actions_entity_state_or: Vec<Option<DefaultActionsEntityState_OR>>,
    #[serde(default)]
    pub default_actions_entity_state_not: Vec<Option<DefaultActionsEntityState_NOT>>,
    #[serde(default)]
    pub default_actions_entity_state_carryable_state: Vec<Option<DefaultActionsEntityState_CarryableState>>,
    #[serde(default)]
    pub default_actions_entity_state_interaction_state_machine_state_tag: Vec<Option<DefaultActionsEntityState_InteractionStateMachineStateTag>>,
    #[serde(default)]
    pub default_actions_entity_state_in_takedown_range: Vec<Option<DefaultActionsEntityState_InTakedownRange>>,
    #[serde(default)]
    pub default_actions_entity_state_can_attach_to_held_weapon: Vec<Option<DefaultActionsEntityState_CanAttachToHeldWeapon>>,
    #[serde(default)]
    pub default_actions_entity_state_has_available_comms_tap: Vec<Option<DefaultActionsEntityState_HasAvailableCommsTap>>,
}
