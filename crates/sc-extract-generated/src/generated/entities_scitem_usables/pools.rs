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

/// Pool storage for the `entities-scitem-usables` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemUsablesPools {
    #[serde(default)]
    pub procedural_connection_link_controller: Vec<Option<ProceduralConnectionLinkController>>,
    #[serde(default)]
    pub closest_orientation_handhold_attach_spot_choice_params: Vec<Option<ClosestOrientationHandholdAttachSpotChoiceParams>>,
    #[serde(default)]
    pub specific_handhold_attach_spot_choice_params: Vec<Option<SpecificHandholdAttachSpotChoiceParams>>,
    #[serde(default)]
    pub handhold_attachment_trigger_params: Vec<Option<HandholdAttachmentTriggerParams>>,
    #[serde(default)]
    pub handhold_shared_interaction_link: Vec<Option<HandholdSharedInteractionLink>>,
    #[serde(default)]
    pub handhold_interaction_point_link: Vec<Option<HandholdInteractionPointLink>>,
    #[serde(default)]
    pub handhold_link_component_params: Vec<Option<HandholdLinkComponentParams>>,
    #[serde(default)]
    pub splayer_usable_search_route_usable: Vec<Option<SPlayerUsableSearchRouteUsable>>,
    #[serde(default)]
    pub ssequencer_player_usable_task_params: Vec<Option<SSequencerPlayerUsableTaskParams>>,
    #[serde(default)]
    pub ssequencer_player_usable_switch_channel_task_params: Vec<Option<SSequencerPlayerUsableSwitchChannelTaskParams>>,
    #[serde(default)]
    pub ssequencer_player_usable_scooch_task_params: Vec<Option<SSequencerPlayerUsableScoochTaskParams>>,
    #[serde(default)]
    pub ssequencer_player_usable_use_channel_task_params: Vec<Option<SSequencerPlayerUsableUseChannelTaskParams>>,
    #[serde(default)]
    pub ssequencer_player_usable_reserve_slot_task_params: Vec<Option<SSequencerPlayerUsableReserveSlotTaskParams>>,
    #[serde(default)]
    pub player_usable_interaction_point: Vec<Option<PlayerUsableInteractionPoint>>,
    #[serde(default)]
    pub player_usable_item_port: Vec<Option<PlayerUsableItemPort>>,
    #[serde(default)]
    pub player_usable_slot: Vec<Option<PlayerUsableSlot>>,
    #[serde(default)]
    pub player_usable_use_channel_instance: Vec<Option<PlayerUsableUseChannelInstance>>,
    #[serde(default)]
    pub player_usable_view: Vec<Option<PlayerUsableView>>,
    #[serde(default)]
    pub splayer_usable_params: Vec<Option<SPlayerUsableParams>>,
    #[serde(default)]
    pub med_bed_provider_params: Vec<Option<MedBedProviderParams>>,
    #[serde(default)]
    pub med_bed_surgery_names_injury_severity: Vec<Option<MedBedSurgeryNamesInjurySeverity>>,
    #[serde(default)]
    pub med_bed_surgery_names: Vec<Option<MedBedSurgeryNames>>,
    #[serde(default)]
    pub med_bed_respawn_range_override: Vec<Option<MedBedRespawnRangeOverride>>,
    #[serde(default)]
    pub med_bed_component_params: Vec<Option<MedBedComponentParams>>,
    #[serde(default)]
    pub medical_skeleton_uiprovider_params: Vec<Option<MedicalSkeletonUIProviderParams>>,
    #[serde(default)]
    pub linked_clone_location_medical_tier: Vec<Option<LinkedCloneLocationMedicalTier>>,
    #[serde(default)]
    pub set_clone_location_medical_tier: Vec<Option<SetCloneLocationMedicalTier>>,
    #[serde(default)]
    pub clone_location_uiprovider_params: Vec<Option<CloneLocationUIProviderParams>>,
    #[serde(default)]
    pub sentity_component_persistent_entity_entitlement_spawner_params: Vec<Option<SEntityComponentPersistentEntityEntitlementSpawnerParams>>,
    #[serde(default)]
    pub ssequencer_immediate_despawn_despawner_task_params: Vec<Option<SSequencerImmediateDespawnDespawnerTaskParams>>,
    #[serde(default)]
    pub entity_effect_system_enable_sequencer_task: Vec<Option<EntityEffectSystem_EnableSequencerTask>>,
    #[serde(default)]
    pub entity_effect_system_disable_sequencer_task: Vec<Option<EntityEffectSystem_DisableSequencerTask>>,
    #[serde(default)]
    pub hospital_checkin_screen_component_params: Vec<Option<HospitalCheckinScreenComponentParams>>,
    #[serde(default)]
    pub interaction_condition_empty_usable_item_port: Vec<Option<InteractionConditionEmptyUsableItemPort>>,
    #[serde(default)]
    pub interaction_condition_usable_alignment_slot_empty: Vec<Option<InteractionConditionUsableAlignmentSlotEmpty>>,
    #[serde(default)]
    pub interaction_condition_available_space_in_linked_usable_item: Vec<Option<InteractionConditionAvailableSpaceInLinkedUsableItem>>,
    #[serde(default)]
    pub interaction_condition_linked_usable_has_tag: Vec<Option<InteractionConditionLinkedUsableHasTag>>,
    #[serde(default)]
    pub sresource_container_state_modifier: Vec<Option<SResourceContainerStateModifier>>,
    #[serde(default)]
    pub sitem_port_rule_trigger_sequence_def: Vec<Option<SItemPortRule_TriggerSequenceDef>>,
    #[serde(default)]
    pub usable_channel_input_action_group: Vec<Option<UsableChannelInputActionGroup>>,
    #[serde(default)]
    pub usable_channel_input_action: Vec<Option<UsableChannelInputAction>>,
    #[serde(default)]
    pub usable_channel_input_action_control_interactive: Vec<Option<UsableChannelInputAction_ControlInteractive>>,
    #[serde(default)]
    pub interactive_variable_back_to_default_int_params: Vec<Option<InteractiveVariable_BackToDefaultIntParams>>,
    #[serde(default)]
    pub interactive_variable_back_to_default_float_params: Vec<Option<InteractiveVariable_BackToDefaultFloatParams>>,
    #[serde(default)]
    pub int_interactive_variable: Vec<Option<IntInteractiveVariable>>,
    #[serde(default)]
    pub float_interactive_variable: Vec<Option<FloatInteractiveVariable>>,
    #[serde(default)]
    pub control_interactive_variable: Vec<Option<ControlInteractiveVariable>>,
    #[serde(default)]
    pub control_int_interactive_variable: Vec<Option<ControlIntInteractiveVariable>>,
    #[serde(default)]
    pub control_float_interactive_variable: Vec<Option<ControlFloatInteractiveVariable>>,
    #[serde(default)]
    pub sclinked_interactive_controller_params: Vec<Option<SCLinkedInteractiveControllerParams>>,
    #[serde(default)]
    pub usable_archetypes: Vec<Option<UsableArchetypes>>,
    #[serde(default)]
    pub sphere_area_alignment_slot_type_params: Vec<Option<SphereAreaAlignmentSlotTypeParams>>,
    #[serde(default)]
    pub susable_routing_settings: Vec<Option<SUsableRoutingSettings>>,
    #[serde(default)]
    pub ssequencer_usable_delink_task: Vec<Option<SSequencerUsableDelinkTask>>,
    #[serde(default)]
    pub ssequencer_usable_fill_consumable_task_params: Vec<Option<SSequencerUsableFillConsumableTaskParams>>,
    #[serde(default)]
    pub camera_transition_params: Vec<Option<CameraTransitionParams>>,
}
