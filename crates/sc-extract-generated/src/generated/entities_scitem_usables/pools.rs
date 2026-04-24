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

/// Pool storage for the `entities-scitem-usables` feature.
#[derive(Default)]
pub struct EntitiesScitemUsablesPools {
    pub procedural_connection_link_controller: Vec<Option<ProceduralConnectionLinkController>>,
    pub closest_orientation_handhold_attach_spot_choice_params: Vec<Option<ClosestOrientationHandholdAttachSpotChoiceParams>>,
    pub specific_handhold_attach_spot_choice_params: Vec<Option<SpecificHandholdAttachSpotChoiceParams>>,
    pub handhold_attachment_trigger_params: Vec<Option<HandholdAttachmentTriggerParams>>,
    pub handhold_shared_interaction_link: Vec<Option<HandholdSharedInteractionLink>>,
    pub handhold_interaction_point_link: Vec<Option<HandholdInteractionPointLink>>,
    pub handhold_link_component_params: Vec<Option<HandholdLinkComponentParams>>,
    pub splayer_usable_search_route_usable: Vec<Option<SPlayerUsableSearchRouteUsable>>,
    pub ssequencer_player_usable_task_params: Vec<Option<SSequencerPlayerUsableTaskParams>>,
    pub ssequencer_player_usable_switch_channel_task_params: Vec<Option<SSequencerPlayerUsableSwitchChannelTaskParams>>,
    pub ssequencer_player_usable_scooch_task_params: Vec<Option<SSequencerPlayerUsableScoochTaskParams>>,
    pub ssequencer_player_usable_use_channel_task_params: Vec<Option<SSequencerPlayerUsableUseChannelTaskParams>>,
    pub ssequencer_player_usable_reserve_slot_task_params: Vec<Option<SSequencerPlayerUsableReserveSlotTaskParams>>,
    pub player_usable_interaction_point: Vec<Option<PlayerUsableInteractionPoint>>,
    pub player_usable_item_port: Vec<Option<PlayerUsableItemPort>>,
    pub player_usable_slot: Vec<Option<PlayerUsableSlot>>,
    pub player_usable_use_channel_instance: Vec<Option<PlayerUsableUseChannelInstance>>,
    pub player_usable_view: Vec<Option<PlayerUsableView>>,
    pub splayer_usable_params: Vec<Option<SPlayerUsableParams>>,
    pub med_bed_provider_params: Vec<Option<MedBedProviderParams>>,
    pub med_bed_surgery_names_injury_severity: Vec<Option<MedBedSurgeryNamesInjurySeverity>>,
    pub med_bed_surgery_names: Vec<Option<MedBedSurgeryNames>>,
    pub med_bed_respawn_range_override: Vec<Option<MedBedRespawnRangeOverride>>,
    pub med_bed_component_params: Vec<Option<MedBedComponentParams>>,
    pub medical_skeleton_uiprovider_params: Vec<Option<MedicalSkeletonUIProviderParams>>,
    pub linked_clone_location_medical_tier: Vec<Option<LinkedCloneLocationMedicalTier>>,
    pub set_clone_location_medical_tier: Vec<Option<SetCloneLocationMedicalTier>>,
    pub clone_location_uiprovider_params: Vec<Option<CloneLocationUIProviderParams>>,
    pub sentity_component_persistent_entity_entitlement_spawner_params: Vec<Option<SEntityComponentPersistentEntityEntitlementSpawnerParams>>,
    pub ssequencer_immediate_despawn_despawner_task_params: Vec<Option<SSequencerImmediateDespawnDespawnerTaskParams>>,
    pub entity_effect_system_enable_sequencer_task: Vec<Option<EntityEffectSystem_EnableSequencerTask>>,
    pub entity_effect_system_disable_sequencer_task: Vec<Option<EntityEffectSystem_DisableSequencerTask>>,
    pub hospital_checkin_screen_component_params: Vec<Option<HospitalCheckinScreenComponentParams>>,
    pub interaction_condition_empty_usable_item_port: Vec<Option<InteractionConditionEmptyUsableItemPort>>,
    pub interaction_condition_usable_alignment_slot_empty: Vec<Option<InteractionConditionUsableAlignmentSlotEmpty>>,
    pub interaction_condition_available_space_in_linked_usable_item: Vec<Option<InteractionConditionAvailableSpaceInLinkedUsableItem>>,
    pub interaction_condition_linked_usable_has_tag: Vec<Option<InteractionConditionLinkedUsableHasTag>>,
    pub sresource_container_state_modifier: Vec<Option<SResourceContainerStateModifier>>,
    pub sitem_port_rule_trigger_sequence_def: Vec<Option<SItemPortRule_TriggerSequenceDef>>,
    pub usable_channel_input_action_group: Vec<Option<UsableChannelInputActionGroup>>,
    pub usable_channel_input_action: Vec<Option<UsableChannelInputAction>>,
    pub usable_channel_input_action_control_interactive: Vec<Option<UsableChannelInputAction_ControlInteractive>>,
    pub interactive_variable_back_to_default_int_params: Vec<Option<InteractiveVariable_BackToDefaultIntParams>>,
    pub interactive_variable_back_to_default_float_params: Vec<Option<InteractiveVariable_BackToDefaultFloatParams>>,
    pub int_interactive_variable: Vec<Option<IntInteractiveVariable>>,
    pub float_interactive_variable: Vec<Option<FloatInteractiveVariable>>,
    pub control_interactive_variable: Vec<Option<ControlInteractiveVariable>>,
    pub control_int_interactive_variable: Vec<Option<ControlIntInteractiveVariable>>,
    pub control_float_interactive_variable: Vec<Option<ControlFloatInteractiveVariable>>,
    pub sclinked_interactive_controller_params: Vec<Option<SCLinkedInteractiveControllerParams>>,
    pub usable_archetypes: Vec<Option<UsableArchetypes>>,
    pub sphere_area_alignment_slot_type_params: Vec<Option<SphereAreaAlignmentSlotTypeParams>>,
    pub susable_routing_settings: Vec<Option<SUsableRoutingSettings>>,
    pub ssequencer_usable_delink_task: Vec<Option<SSequencerUsableDelinkTask>>,
    pub ssequencer_usable_fill_consumable_task_params: Vec<Option<SSequencerUsableFillConsumableTaskParams>>,
    pub camera_transition_params: Vec<Option<CameraTransitionParams>>,
}
