// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `core` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoreIndex {
    #[serde(default)]
    pub aitargetable_settings: HashMap<CigGuid, Handle<AITargetableSettings>>,
    #[serde(default)]
    pub aifire_discipline_settings: HashMap<CigGuid, Handle<AIFireDisciplineSettings>>,
    #[serde(default)]
    pub actor_ability_component: HashMap<CigGuid, Handle<ActorAbilityComponent>>,
    #[serde(default)]
    pub actor_ducking_params: HashMap<CigGuid, Handle<ActorDuckingParams>>,
    #[serde(default)]
    pub actor_environment_component: HashMap<CigGuid, Handle<ActorEnvironmentComponent>>,
    #[serde(default)]
    pub actor_gforce_component: HashMap<CigGuid, Handle<ActorGForceComponent>>,
    #[serde(default)]
    pub actor_movement_modifiers: HashMap<CigGuid, Handle<ActorMovementModifiers>>,
    #[serde(default)]
    pub actor_movement_sets_config: HashMap<CigGuid, Handle<ActorMovementSetsConfig>>,
    #[serde(default)]
    pub actor_procedural_recoil_config: HashMap<CigGuid, Handle<ActorProceduralRecoilConfig>>,
    #[serde(default)]
    pub actor_procedural_recoil_modifiers: HashMap<CigGuid, Handle<ActorProceduralRecoilModifiers>>,
    #[serde(default)]
    pub actor_sliding_params: HashMap<CigGuid, Handle<ActorSlidingParams>>,
    #[serde(default)]
    pub actor_speed_camera_effects: HashMap<CigGuid, Handle<ActorSpeedCameraEffects>>,
    #[serde(default)]
    pub actor_stamina_component: HashMap<CigGuid, Handle<ActorStaminaComponent>>,
    #[serde(default)]
    pub actor_state_validation: HashMap<CigGuid, Handle<ActorStateValidation>>,
    #[serde(default)]
    pub actor_status_global_params: HashMap<CigGuid, Handle<ActorStatusGlobalParams>>,
    #[serde(default)]
    pub actor_status_component: HashMap<CigGuid, Handle<ActorStatusComponent>>,
    #[serde(default)]
    pub building_blocks_canvas: HashMap<CigGuid, Handle<BuildingBlocks_Canvas>>,
    #[serde(default)]
    pub building_blocks_external_color_reference: HashMap<CigGuid, Handle<BuildingBlocks_ExternalColorReference>>,
    #[serde(default)]
    pub building_blocks_aspect_ratio_library: HashMap<CigGuid, Handle<BuildingBlocks_AspectRatioLibrary>>,
    #[serde(default)]
    pub actor_fovview_params: HashMap<CigGuid, Handle<ActorFOVViewParams>>,
    #[serde(default)]
    pub carry_config: HashMap<CigGuid, Handle<CarryConfig>>,
    #[serde(default)]
    pub character_serialization_settings_preset: HashMap<CigGuid, Handle<CharacterSerializationSettingsPreset>>,
    #[serde(default)]
    pub character_random_name_params: HashMap<CigGuid, Handle<CharacterRandomNameParams>>,
    #[serde(default)]
    pub communication_channel_name: HashMap<CigGuid, Handle<CommunicationChannelName>>,
    #[serde(default)]
    pub entity_audio_controller_rtpc_subscriber_list_def: HashMap<CigGuid, Handle<EntityAudioControllerRtpcSubscriberListDef>>,
    #[serde(default)]
    pub ladder_config: HashMap<CigGuid, Handle<LadderConfig>>,
    #[serde(default)]
    pub mining_audio_params: HashMap<CigGuid, Handle<MiningAudioParams>>,
    #[serde(default)]
    pub conversation: HashMap<CigGuid, Handle<Conversation>>,
    #[serde(default)]
    pub conversation_bank: HashMap<CigGuid, Handle<ConversationBank>>,
    #[serde(default)]
    pub sbezier_curve_record: HashMap<CigGuid, Handle<SBezierCurveRecord>>,
    #[serde(default)]
    pub dev_owner: HashMap<CigGuid, Handle<DevOwner>>,
    #[serde(default)]
    pub dialogue_context: HashMap<CigGuid, Handle<DialogueContext>>,
    #[serde(default)]
    pub dialogue_context_bank: HashMap<CigGuid, Handle<DialogueContextBank>>,
    #[serde(default)]
    pub entity_class_definition: HashMap<CigGuid, Handle<EntityClassDefinition>>,
    #[serde(default)]
    pub sloadout_assortment: HashMap<CigGuid, Handle<SLoadoutAssortment>>,
    #[serde(default)]
    pub sentity_traversing_node_id: HashMap<CigGuid, Handle<SEntityTraversingNodeId>>,
    #[serde(default)]
    pub faction_legacy: HashMap<CigGuid, Handle<Faction_LEGACY>>,
    #[serde(default)]
    pub body_part: HashMap<CigGuid, Handle<BodyPart>>,
    #[serde(default)]
    pub body_mapping: HashMap<CigGuid, Handle<BodyMapping>>,
    #[serde(default)]
    pub body_health_config: HashMap<CigGuid, Handle<BodyHealthConfig>>,
    #[serde(default)]
    pub health_template: HashMap<CigGuid, Handle<HealthTemplate>>,
    #[serde(default)]
    pub inventory_container: HashMap<CigGuid, Handle<InventoryContainer>>,
    #[serde(default)]
    pub item_port_tags_dictionary: HashMap<CigGuid, Handle<ItemPortTagsDictionary>>,
    #[serde(default)]
    pub item_resource_network_global: HashMap<CigGuid, Handle<ItemResourceNetworkGlobal>>,
    #[serde(default)]
    pub journal_entry_type: HashMap<CigGuid, Handle<JournalEntryType>>,
    #[serde(default)]
    pub jump_fall_land_config: HashMap<CigGuid, Handle<JumpFallLandConfig>>,
    #[serde(default)]
    pub local_player_speed_throttle_component: HashMap<CigGuid, Handle<LocalPlayerSpeedThrottleComponent>>,
    #[serde(default)]
    pub interior_map_section_definition: HashMap<CigGuid, Handle<InteriorMapSectionDefinition>>,
    #[serde(default)]
    pub medical_item_tier_config: HashMap<CigGuid, Handle<MedicalItemTierConfig>>,
    #[serde(default)]
    pub melee_combat_config: HashMap<CigGuid, Handle<MeleeCombatConfig>>,
    #[serde(default)]
    pub contract_generator: HashMap<CigGuid, Handle<ContractGenerator>>,
    #[serde(default)]
    pub item_award_weightings_record: HashMap<CigGuid, Handle<ItemAwardWeightingsRecord>>,
    #[serde(default)]
    pub contract_template: HashMap<CigGuid, Handle<ContractTemplate>>,
    #[serde(default)]
    pub notification_def: HashMap<CigGuid, Handle<NotificationDef>>,
    #[serde(default)]
    pub comms_notification_stage: HashMap<CigGuid, Handle<CommsNotificationStage>>,
    #[serde(default)]
    pub game_notification_dock_item_params: HashMap<CigGuid, Handle<GameNotificationDockItemParams>>,
    #[serde(default)]
    pub gpuparticle_audio: HashMap<CigGuid, Handle<GPUParticleAudio>>,
    #[serde(default)]
    pub player_animated_interaction_template: HashMap<CigGuid, Handle<PlayerAnimatedInteractionTemplate>>,
    #[serde(default)]
    pub player_choice_menu_item: HashMap<CigGuid, Handle<PlayerChoiceMenuItem>>,
    #[serde(default)]
    pub player_choice_menu_items: HashMap<CigGuid, Handle<PlayerChoiceMenuItems>>,
    #[serde(default)]
    pub player_choice_menu: HashMap<CigGuid, Handle<PlayerChoiceMenu>>,
    #[serde(default)]
    pub player_choice_menu_type: HashMap<CigGuid, Handle<PlayerChoiceMenuType>>,
    #[serde(default)]
    pub actor_stance_config: HashMap<CigGuid, Handle<ActorStanceConfig>>,
    #[serde(default)]
    pub jump_fall_land_params: HashMap<CigGuid, Handle<JumpFallLandParams>>,
    #[serde(default)]
    pub player_dock_context_component_global_params: HashMap<CigGuid, Handle<PlayerDockContextComponentGlobalParams>>,
    #[serde(default)]
    pub player_group_manager_global_params: HashMap<CigGuid, Handle<PlayerGroupManagerGlobalParams>>,
    #[serde(default)]
    pub player_limitations_profile: HashMap<CigGuid, Handle<PlayerLimitationsProfile>>,
    #[serde(default)]
    pub player_notification_banner_manager_global_params: HashMap<CigGuid, Handle<PlayerNotificationBannerManagerGlobalParams>>,
    #[serde(default)]
    pub player_trade_global_params: HashMap<CigGuid, Handle<PlayerTradeGlobalParams>>,
    #[serde(default)]
    pub scan_display_instance_params: HashMap<CigGuid, Handle<ScanDisplayInstanceParams>>,
    #[serde(default)]
    pub reputation_value_setting: HashMap<CigGuid, Handle<ReputationValueSetting>>,
    #[serde(default)]
    pub reputation_value_settings: HashMap<CigGuid, Handle<ReputationValueSettings>>,
    #[serde(default)]
    pub actor_view_limit_preset_database: HashMap<CigGuid, Handle<ActorViewLimitPresetDatabase>>,
    #[serde(default)]
    pub scitem_light_amplification: HashMap<CigGuid, Handle<SCItemLightAmplification>>,
    #[serde(default)]
    pub saimable_game_mode_params: HashMap<CigGuid, Handle<SAimableGameModeParams>>,
    #[serde(default)]
    pub scitem_visor_dashboard_config: HashMap<CigGuid, Handle<SCItemVisorDashboardConfig>>,
    #[serde(default)]
    pub vehicle_salvage_global_params: HashMap<CigGuid, Handle<VehicleSalvageGlobalParams>>,
    #[serde(default)]
    pub player_to_player_comms_call_global_params: HashMap<CigGuid, Handle<PlayerToPlayerCommsCallGlobalParams>>,
    #[serde(default)]
    pub shop_franchise: HashMap<CigGuid, Handle<ShopFranchise>>,
    #[serde(default)]
    pub star_map_amenity_type_entry: HashMap<CigGuid, Handle<StarMapAmenityTypeEntry>>,
    #[serde(default)]
    pub star_map_amenity_types: HashMap<CigGuid, Handle<StarMapAmenityTypes>>,
    #[serde(default)]
    pub star_map_object: HashMap<CigGuid, Handle<StarMapObject>>,
    #[serde(default)]
    pub simple_sprite_sheet: HashMap<CigGuid, Handle<SimpleSpriteSheet>>,
    #[serde(default)]
    pub loadout_editor_component_params: HashMap<CigGuid, Handle<LoadoutEditorComponentParams>>,
    #[serde(default)]
    pub unit_test: HashMap<CigGuid, Handle<UnitTest>>,
    #[serde(default)]
    pub svehicle_ai_damage_modifiers: HashMap<CigGuid, Handle<SVehicleAiDamageModifiers>>,
}

impl CoreIndex {
    pub fn len(&self) -> usize {
        self.aitargetable_settings.len()
            + self.aifire_discipline_settings.len()
            + self.actor_ability_component.len()
            + self.actor_ducking_params.len()
            + self.actor_environment_component.len()
            + self.actor_gforce_component.len()
            + self.actor_movement_modifiers.len()
            + self.actor_movement_sets_config.len()
            + self.actor_procedural_recoil_config.len()
            + self.actor_procedural_recoil_modifiers.len()
            + self.actor_sliding_params.len()
            + self.actor_speed_camera_effects.len()
            + self.actor_stamina_component.len()
            + self.actor_state_validation.len()
            + self.actor_status_global_params.len()
            + self.actor_status_component.len()
            + self.building_blocks_canvas.len()
            + self.building_blocks_external_color_reference.len()
            + self.building_blocks_aspect_ratio_library.len()
            + self.actor_fovview_params.len()
            + self.carry_config.len()
            + self.character_serialization_settings_preset.len()
            + self.character_random_name_params.len()
            + self.communication_channel_name.len()
            + self.entity_audio_controller_rtpc_subscriber_list_def.len()
            + self.ladder_config.len()
            + self.mining_audio_params.len()
            + self.conversation.len()
            + self.conversation_bank.len()
            + self.sbezier_curve_record.len()
            + self.dev_owner.len()
            + self.dialogue_context.len()
            + self.dialogue_context_bank.len()
            + self.entity_class_definition.len()
            + self.sloadout_assortment.len()
            + self.sentity_traversing_node_id.len()
            + self.faction_legacy.len()
            + self.body_part.len()
            + self.body_mapping.len()
            + self.body_health_config.len()
            + self.health_template.len()
            + self.inventory_container.len()
            + self.item_port_tags_dictionary.len()
            + self.item_resource_network_global.len()
            + self.journal_entry_type.len()
            + self.jump_fall_land_config.len()
            + self.local_player_speed_throttle_component.len()
            + self.interior_map_section_definition.len()
            + self.medical_item_tier_config.len()
            + self.melee_combat_config.len()
            + self.contract_generator.len()
            + self.item_award_weightings_record.len()
            + self.contract_template.len()
            + self.notification_def.len()
            + self.comms_notification_stage.len()
            + self.game_notification_dock_item_params.len()
            + self.gpuparticle_audio.len()
            + self.player_animated_interaction_template.len()
            + self.player_choice_menu_item.len()
            + self.player_choice_menu_items.len()
            + self.player_choice_menu.len()
            + self.player_choice_menu_type.len()
            + self.actor_stance_config.len()
            + self.jump_fall_land_params.len()
            + self.player_dock_context_component_global_params.len()
            + self.player_group_manager_global_params.len()
            + self.player_limitations_profile.len()
            + self.player_notification_banner_manager_global_params.len()
            + self.player_trade_global_params.len()
            + self.scan_display_instance_params.len()
            + self.reputation_value_setting.len()
            + self.reputation_value_settings.len()
            + self.actor_view_limit_preset_database.len()
            + self.scitem_light_amplification.len()
            + self.saimable_game_mode_params.len()
            + self.scitem_visor_dashboard_config.len()
            + self.vehicle_salvage_global_params.len()
            + self.player_to_player_comms_call_global_params.len()
            + self.shop_franchise.len()
            + self.star_map_amenity_type_entry.len()
            + self.star_map_amenity_types.len()
            + self.star_map_object.len()
            + self.simple_sprite_sheet.len()
            + self.loadout_editor_component_params.len()
            + self.unit_test.len()
            + self.svehicle_ai_damage_modifiers.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
