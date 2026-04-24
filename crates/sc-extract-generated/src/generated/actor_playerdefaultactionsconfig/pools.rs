// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `actor-playerdefaultactionsconfig` feature.
#[derive(Default)]
pub struct ActorPlayerdefaultactionsconfigPools {
    pub default_action_interaction_def: Vec<Option<DefaultAction_InteractionDef>>,
    pub default_action_carryable_interaction_def:
        Vec<Option<DefaultAction_CarryableInteractionDef>>,
    pub default_action_looting_interaction_def: Vec<Option<DefaultAction_LootingInteractionDef>>,
    pub default_action_action_def: Vec<Option<DefaultAction_ActionDef>>,
    pub default_actions_params: Vec<Option<DefaultActionsParams>>,
    pub default_actions: Vec<Option<DefaultActions>>,
    pub default_action_description_override: Vec<Option<DefaultActionDescriptionOverride>>,
    pub default_actions_entry: Vec<Option<DefaultActionsEntry>>,
    pub actor_default_actions_config: Vec<Option<ActorDefaultActionsConfig>>,
    pub default_actions_entity_entry_condition_and:
        Vec<Option<DefaultActionsEntityEntryCondition_AND>>,
    pub default_actions_entity_entry_condition_or:
        Vec<Option<DefaultActionsEntityEntryCondition_OR>>,
    pub default_actions_entity_entry_condition_attachable_items:
        Vec<Option<DefaultActionsEntityEntryCondition_AttachableItems>>,
    pub default_actions_entity_entry_condition_entity_types:
        Vec<Option<DefaultActionsEntityEntryCondition_EntityTypes>>,
    pub default_actions_entity_entry_condition_customisable:
        Vec<Option<DefaultActionsEntityEntryCondition_Customisable>>,
    pub default_actions_entity_entry_condition_primed:
        Vec<Option<DefaultActionsEntityEntryCondition_Primed>>,
    pub default_actions_entity_entry_condition_tags:
        Vec<Option<DefaultActionsEntityEntryCondition_Tags>>,
    pub default_actions_entity_entry_condition_def_inventory_container_capacity:
        Vec<Option<DefaultActionsEntityEntryConditionDef_InventoryContainerCapacity>>,
    pub default_actions_entity_state_and: Vec<Option<DefaultActionsEntityState_AND>>,
    pub default_actions_entity_state_or: Vec<Option<DefaultActionsEntityState_OR>>,
    pub default_actions_entity_state_not: Vec<Option<DefaultActionsEntityState_NOT>>,
    pub default_actions_entity_state_carryable_state:
        Vec<Option<DefaultActionsEntityState_CarryableState>>,
    pub default_actions_entity_state_interaction_state_machine_state_tag:
        Vec<Option<DefaultActionsEntityState_InteractionStateMachineStateTag>>,
    pub default_actions_entity_state_in_takedown_range:
        Vec<Option<DefaultActionsEntityState_InTakedownRange>>,
    pub default_actions_entity_state_can_attach_to_held_weapon:
        Vec<Option<DefaultActionsEntityState_CanAttachToHeldWeapon>>,
    pub default_actions_entity_state_has_available_comms_tap:
        Vec<Option<DefaultActionsEntityState_HasAvailableCommsTap>>,
}
