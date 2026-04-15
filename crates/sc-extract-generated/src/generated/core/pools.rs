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

/// Pool storage for the `core` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorePools {
    #[serde(default)]
    pub activity_behavior_request_condition: Vec<Option<ActivityBehaviorRequestCondition>>,
    #[serde(default)]
    pub iobservable_extender: Vec<Option<IObservableExtender>>,
    #[serde(default)]
    pub ivision_component_adapter: Vec<Option<IVisionComponentAdapter>>,
    #[serde(default)]
    pub movement_system_additional_params: Vec<Option<MovementSystemAdditionalParams>>,
    #[serde(default)]
    pub nav_link_location: Vec<Option<NavLinkLocation>>,
    #[serde(default)]
    pub navigation_link_controller: Vec<Option<NavigationLinkController>>,
    #[serde(default)]
    pub traversal_cost_shape_config: Vec<Option<TraversalCostShapeConfig>>,
    #[serde(default)]
    pub navigation_trigger_adapter: Vec<Option<NavigationTriggerAdapter>>,
    #[serde(default)]
    pub inavigation_cost_volume_extender: Vec<Option<INavigationCostVolumeExtender>>,
    #[serde(default)]
    pub action_area_extension_type: Vec<Option<ActionAreaExtensionType>>,
    #[serde(default)]
    pub sactor_force_reaction_procedural_lean_pose_list: Vec<Option<SActorForceReactionProceduralLeanPoseList>>,
    #[serde(default)]
    pub imannequin_action_def: Vec<Option<IMannequinActionDef>>,
    #[serde(default)]
    pub sactor_stance_dimensions_extra_def: Vec<Option<SActorStanceDimensionsExtraDef>>,
    #[serde(default)]
    pub status_effect_value: Vec<Option<StatusEffectValue>>,
    #[serde(default)]
    pub status_effect_setup_base: Vec<Option<StatusEffectSetupBase>>,
    #[serde(default)]
    pub status_masked_retrigger_setup_base: Vec<Option<StatusMaskedRetriggerSetupBase>>,
    #[serde(default)]
    pub linked_stat_pass_value_base: Vec<Option<LinkedStatPassValueBase>>,
    #[serde(default)]
    pub linked_stat_rule_base: Vec<Option<LinkedStatRuleBase>>,
    #[serde(default)]
    pub status_buff_type_base: Vec<Option<StatusBuffTypeBase>>,
    #[serde(default)]
    pub buff_duration_base: Vec<Option<BuffDurationBase>>,
    #[serde(default)]
    pub counter_measure_base_params: Vec<Option<CounterMeasureBaseParams>>,
    #[serde(default)]
    pub ssequencer_animation_task_params_base: Vec<Option<SSequencerAnimationTaskParamsBase>>,
    #[serde(default)]
    pub audio_breath_style_base_node: Vec<Option<AudioBreathStyleBaseNode>>,
    #[serde(default)]
    pub building_blocks_node: Vec<Option<BuildingBlocks_Node>>,
    #[serde(default)]
    pub building_blocks_bindings_path_base: Vec<Option<BuildingBlocks_BindingsPathBase>>,
    #[serde(default)]
    pub building_blocks_bindings_boolean_base: Vec<Option<BuildingBlocks_BindingsBooleanBase>>,
    #[serde(default)]
    pub building_blocks_bindings_integer_base: Vec<Option<BuildingBlocks_BindingsIntegerBase>>,
    #[serde(default)]
    pub building_blocks_bindings_number_base: Vec<Option<BuildingBlocks_BindingsNumberBase>>,
    #[serde(default)]
    pub bindings_operations_waveform_shape_base: Vec<Option<BindingsOperations_WaveformShapeBase>>,
    #[serde(default)]
    pub building_blocks_bindings_string_base: Vec<Option<BuildingBlocks_BindingsStringBase>>,
    #[serde(default)]
    pub building_blocks_bindings_localized_base: Vec<Option<BuildingBlocks_BindingsLocalizedBase>>,
    #[serde(default)]
    pub building_blocks_bindings_color_base: Vec<Option<BuildingBlocks_BindingsColorBase>>,
    #[serde(default)]
    pub building_blocks_bindings_vector_base: Vec<Option<BuildingBlocks_BindingsVectorBase>>,
    #[serde(default)]
    pub building_blocks_bindings_rotation_base: Vec<Option<BuildingBlocks_BindingsRotationBase>>,
    #[serde(default)]
    pub building_blocks_bindings_transform_base: Vec<Option<BuildingBlocks_BindingsTransformBase>>,
    #[serde(default)]
    pub building_blocks_layout_policy_base: Vec<Option<BuildingBlocks_LayoutPolicyBase>>,
    #[serde(default)]
    pub building_blocks_layout_policy_item_base: Vec<Option<BuildingBlocks_LayoutPolicyItemBase>>,
    #[serde(default)]
    pub building_blocks_scroll_policy_base: Vec<Option<BuildingBlocks_ScrollPolicyBase>>,
    #[serde(default)]
    pub building_blocks_field_modifier_base: Vec<Option<BuildingBlocks_FieldModifierBase>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_base: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeBase>>,
    #[serde(default)]
    pub building_blocks_renderer_policy_base: Vec<Option<BuildingBlocks_RendererPolicyBase>>,
    #[serde(default)]
    pub building_blocks_field_modifier_record_ref_type_base: Vec<Option<BuildingBlocks_FieldModifierRecordRefTypeBase>>,
    #[serde(default)]
    pub building_blocks_timing_function_base: Vec<Option<BuildingBlocks_TimingFunctionBase>>,
    #[serde(default)]
    pub building_blocks_timeline_type_base: Vec<Option<BuildingBlocks_TimelineTypeBase>>,
    #[serde(default)]
    pub building_blocks_transformer_base: Vec<Option<BuildingBlocks_TransformerBase>>,
    #[serde(default)]
    pub building_blocks_slicer_base: Vec<Option<BuildingBlocks_SlicerBase>>,
    #[serde(default)]
    pub building_blocks_trigger_base: Vec<Option<BuildingBlocks_TriggerBase>>,
    #[serde(default)]
    pub building_blocks_text_format_modifier_base: Vec<Option<BuildingBlocks_TextFormatModifierBase>>,
    #[serde(default)]
    pub building_blocks_style_selector_condition_base: Vec<Option<BuildingBlocks_StyleSelectorConditionBase>>,
    #[serde(default)]
    pub building_blocks_color_base: Vec<Option<BuildingBlocks_ColorBase>>,
    #[serde(default)]
    pub building_blocks_container_mode_base: Vec<Option<BuildingBlocks_ContainerModeBase>>,
    #[serde(default)]
    pub base_cargo_fill_capacity_value: Vec<Option<BaseCargoFillCapacityValue>>,
    #[serde(default)]
    pub scustomzier_color_def_base: Vec<Option<SCustomzierColorDefBase>>,
    #[serde(default)]
    pub scharacter_customizer_randomization_params: Vec<Option<SCharacterCustomizerRandomizationParams>>,
    #[serde(default)]
    pub scharacter_validation_params: Vec<Option<SCharacterValidationParams>>,
    #[serde(default)]
    pub temperature_damage_control: Vec<Option<TemperatureDamageControl>>,
    #[serde(default)]
    pub legacy_crafting_cost_base: Vec<Option<LegacyCraftingCost_Base>>,
    #[serde(default)]
    pub legacy_crafting_output_base: Vec<Option<LegacyCraftingOutput_Base>>,
    #[serde(default)]
    pub legacy_crafting_recipe_base: Vec<Option<LegacyCraftingRecipe_Base>>,
    #[serde(default)]
    pub legacy_crafting_recipe_def_base: Vec<Option<LegacyCraftingRecipeDef_Base>>,
    #[serde(default)]
    pub legacy_crafting_recipe_list_base: Vec<Option<LegacyCraftingRecipeList_Base>>,
    #[serde(default)]
    pub ssensor_mine_trigger_type: Vec<Option<SSensorMineTriggerType>>,
    #[serde(default)]
    pub handhold_attach_point_choice_params: Vec<Option<HandholdAttachPointChoiceParams>>,
    #[serde(default)]
    pub harvestable_tag_list_base: Vec<Option<HarvestableTagListBase>>,
    #[serde(default)]
    pub sub_harvestable_config_base: Vec<Option<SubHarvestableConfigBase>>,
    #[serde(default)]
    pub sub_harvestable_config_single_base: Vec<Option<SubHarvestableConfigSingleBase>>,
    #[serde(default)]
    pub harvest_condition_base: Vec<Option<HarvestConditionBase>>,
    #[serde(default)]
    pub harvestable_area_type_base: Vec<Option<HarvestableAreaTypeBase>>,
    #[serde(default)]
    pub item_throttle_params_base: Vec<Option<ItemThrottleParamsBase>>,
    #[serde(default)]
    pub item_modifier_lifetime: Vec<Option<ItemModifierLifetime>>,
    #[serde(default)]
    pub base_item_modifier_params: Vec<Option<BaseItemModifierParams>>,
    #[serde(default)]
    pub light_flicker_wave_params: Vec<Option<LightFlickerWaveParams>>,
    #[serde(default)]
    pub smisfire_condition: Vec<Option<SMisfireCondition>>,
    #[serde(default)]
    pub smisfire_damage: Vec<Option<SMisfireDamage>>,
    #[serde(default)]
    pub slegacy_item_misfire_params: Vec<Option<SLegacyItemMisfireParams>>,
    #[serde(default)]
    pub particle_effect_none_tinting_params: Vec<Option<ParticleEffectNoneTintingParams>>,
    #[serde(default)]
    pub sqed_visual_graph_transition_type_params: Vec<Option<SQedVisualGraphTransitionTypeParams>>,
    #[serde(default)]
    pub stargeting_method_base: Vec<Option<STargetingMethodBase>>,
    #[serde(default)]
    pub ctx_graph_node: Vec<Option<CtxGraph_Node>>,
    #[serde(default)]
    pub ctx_graph_component: Vec<Option<CtxGraph_Component>>,
    #[serde(default)]
    pub scitem_controllable_params: Vec<Option<SCItemControllableParams>>,
    #[serde(default)]
    pub scitem_control_condition_base: Vec<Option<SCItemControlCondition_Base>>,
    #[serde(default)]
    pub scitem_control_base_params: Vec<Option<SCItemControlBaseParams>>,
    #[serde(default)]
    pub scitem_control_priority_value: Vec<Option<SCItemControlPriorityValue>>,
    #[serde(default)]
    pub conversation_node_base: Vec<Option<ConversationNode_Base>>,
    #[serde(default)]
    pub crafting_cost_context_base: Vec<Option<CraftingCostContext_Base>>,
    #[serde(default)]
    pub crafting_optional_effect_base: Vec<Option<CraftingOptionalEffect_Base>>,
    #[serde(default)]
    pub crafting_gameplay_property_modifier_value_range_base: Vec<Option<CraftingGameplayPropertyModifierValueRange_Base>>,
    #[serde(default)]
    pub crafting_gameplay_property_modifier_base: Vec<Option<CraftingGameplayPropertyModifier_Base>>,
    #[serde(default)]
    pub crafting_gameplay_property_modifiers_base: Vec<Option<CraftingGameplayPropertyModifiers_Base>>,
    #[serde(default)]
    pub crafting_result_base: Vec<Option<CraftingResult_Base>>,
    #[serde(default)]
    pub crafting_recipe_costs_base: Vec<Option<CraftingRecipeCosts_Base>>,
    #[serde(default)]
    pub crafting_recipe_costs_base_non_ref: Vec<Option<CraftingRecipeCosts_Base_NonRef>>,
    #[serde(default)]
    pub crafting_recipe_results_base: Vec<Option<CraftingRecipeResults_Base>>,
    #[serde(default)]
    pub crafting_process_specific_recipe_data_base: Vec<Option<CraftingProcessSpecificRecipeData_Base>>,
    #[serde(default)]
    pub crafting_recipe_base: Vec<Option<CraftingRecipe_Base>>,
    #[serde(default)]
    pub crafting_recipe_base_non_ref: Vec<Option<CraftingRecipe_Base_NonRef>>,
    #[serde(default)]
    pub crafting_research_unlock_base: Vec<Option<CraftingResearchUnlock_Base>>,
    #[serde(default)]
    pub crafting_research_base: Vec<Option<CraftingResearch_Base>>,
    #[serde(default)]
    pub crafting_blueprint_tier_base: Vec<Option<CraftingBlueprintTier_Base>>,
    #[serde(default)]
    pub blueprint_category_availability_base: Vec<Option<BlueprintCategoryAvailability_Base>>,
    #[serde(default)]
    pub crafting_process_base: Vec<Option<CraftingProcess_Base>>,
    #[serde(default)]
    pub generic_crafting_process_base: Vec<Option<GenericCraftingProcess_Base>>,
    #[serde(default)]
    pub crafting_blueprint_base_non_ref: Vec<Option<CraftingBlueprint_Base_NonRef>>,
    #[serde(default)]
    pub default_blueprint_selection_base: Vec<Option<DefaultBlueprintSelection_Base>>,
    #[serde(default)]
    pub crafting_quality_distribution_base: Vec<Option<CraftingQualityDistribution_Base>>,
    #[serde(default)]
    pub crafting_quality_distribution_base_non_ref: Vec<Option<CraftingQualityDistribution_Base_NonRef>>,
    #[serde(default)]
    pub crafting_quality_location_override_base: Vec<Option<CraftingQualityLocationOverride_Base>>,
    #[serde(default)]
    pub crafting_quality_location_override_base_non_ref: Vec<Option<CraftingQualityLocationOverride_Base_NonRef>>,
    #[serde(default)]
    pub custom_float: Vec<Option<CustomFloat>>,
    #[serde(default)]
    pub damage_base: Vec<Option<DamageBase>>,
    #[serde(default)]
    pub damage_resistance_base: Vec<Option<DamageResistanceBase>>,
    #[serde(default)]
    pub explosion_base_params: Vec<Option<ExplosionBaseParams>>,
    #[serde(default)]
    pub default_action_def: Vec<Option<DefaultActionDef>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition: Vec<Option<DefaultActionsEntityEntryCondition>>,
    #[serde(default)]
    pub default_actions_entity_state: Vec<Option<DefaultActionsEntityState>>,
    #[serde(default)]
    pub dialogue_context_entry: Vec<Option<DialogueContextEntry>>,
    #[serde(default)]
    pub sirounds_module: Vec<Option<SIRoundsModule>>,
    #[serde(default)]
    pub seffect_params_node_base: Vec<Option<SEffectParamsNodeBase>>,
    #[serde(default)]
    pub data_forge_component_params: Vec<Option<DataForgeComponentParams>>,
    #[serde(default)]
    pub entity_class_static_data_params: Vec<Option<EntityClassStaticDataParams>>,
    #[serde(default)]
    pub shazard_area_shape_params: Vec<Option<SHazardAreaShapeParams>>,
    #[serde(default)]
    pub float_user_variable_task: Vec<Option<FloatUserVariableTask>>,
    #[serde(default)]
    pub int_user_variable_task: Vec<Option<IntUserVariableTask>>,
    #[serde(default)]
    pub bool_user_variable_task: Vec<Option<BoolUserVariableTask>>,
    #[serde(default)]
    pub string_user_variable_task: Vec<Option<StringUserVariableTask>>,
    #[serde(default)]
    pub record_ref_user_variable_type_base: Vec<Option<RecordRefUserVariableTypeBase>>,
    #[serde(default)]
    pub clone_location_medical_tier: Vec<Option<CloneLocationMedicalTier>>,
    #[serde(default)]
    pub ssequencer_carryable_task_params: Vec<Option<SSequencerCarryableTaskParams>>,
    #[serde(default)]
    pub chat_provider_settings_base: Vec<Option<ChatProviderSettingsBase>>,
    #[serde(default)]
    pub sloadout_requirement_base: Vec<Option<SLoadoutRequirementBase>>,
    #[serde(default)]
    pub ssequencer_entity_drag_task_params: Vec<Option<SSequencerEntityDragTaskParams>>,
    #[serde(default)]
    pub eatransport_base_transition_params: Vec<Option<EATransportBaseTransitionParams>>,
    #[serde(default)]
    pub sitem_misfire_params: Vec<Option<SItemMisfireParams>>,
    #[serde(default)]
    pub sobject_metadata_params: Vec<Option<SObjectMetadataParams>>,
    #[serde(default)]
    pub sanimated_outfit_swap_data: Vec<Option<SAnimatedOutfitSwapData>>,
    #[serde(default)]
    pub ssequencer_def_task_params: Vec<Option<SSequencerDefTaskParams>>,
    #[serde(default)]
    pub sspawn_rules: Vec<Option<SSpawnRules>>,
    #[serde(default)]
    pub base_port_refill_data: Vec<Option<BasePortRefillData>>,
    #[serde(default)]
    pub base_spawner_prerequisite: Vec<Option<BaseSpawnerPrerequisite>>,
    #[serde(default)]
    pub ssequencer_spawner_task_params: Vec<Option<SSequencerSpawnerTaskParams>>,
    #[serde(default)]
    pub ssequencer_despawner_task_params: Vec<Option<SSequencerDespawnerTaskParams>>,
    #[serde(default)]
    pub sentity_effect_system_property_modifier: Vec<Option<SEntityEffectSystem_PropertyModifier>>,
    #[serde(default)]
    pub entity_effect_system_base_sequencer_task: Vec<Option<EntityEffectSystem_BaseSequencerTask>>,
    #[serde(default)]
    pub event_dispatcher: Vec<Option<EventDispatcher>>,
    #[serde(default)]
    pub sentity_density_class_overrides_base: Vec<Option<SEntityDensityClassOverridesBase>>,
    #[serde(default)]
    pub sentity_traversing_node_type_params: Vec<Option<SEntityTraversingNodeTypeParams>>,
    #[serde(default)]
    pub sentity_traversing_execute_node_base: Vec<Option<SEntityTraversingExecuteNodeBase>>,
    #[serde(default)]
    pub fire_voxel_selection_shape: Vec<Option<FireVoxelSelectionShape>>,
    #[serde(default)]
    pub extinguish_type_base: Vec<Option<ExtinguishType_Base>>,
    #[serde(default)]
    pub fire_repairer_type_base: Vec<Option<FireRepairerType_Base>>,
    #[serde(default)]
    pub srtpc_behaviour: Vec<Option<SRtpcBehaviour>>,
    #[serde(default)]
    pub sipickup_module: Vec<Option<SIPickupModule>>,
    #[serde(default)]
    pub sidamage_handling_module: Vec<Option<SIDamageHandlingModule>>,
    #[serde(default)]
    pub sispectator_module: Vec<Option<SISpectatorModule>>,
    #[serde(default)]
    pub siplayer_setup_module: Vec<Option<SIPlayerSetupModule>>,
    #[serde(default)]
    pub sistats_recording_module: Vec<Option<SIStatsRecordingModule>>,
    #[serde(default)]
    pub sinotifications_module: Vec<Option<SINotificationsModule>>,
    #[serde(default)]
    pub siobjectives: Vec<Option<SIObjectives>>,
    #[serde(default)]
    pub sicameras_module: Vec<Option<SICamerasModule>>,
    #[serde(default)]
    pub siplayer_stats: Vec<Option<SIPlayerStats>>,
    #[serde(default)]
    pub sispawning: Vec<Option<SISpawning>>,
    #[serde(default)]
    pub seaspawn_respawn_scheduler_params: Vec<Option<SEASpawnRespawnSchedulerParams>>,
    #[serde(default)]
    pub sivictory_conditions_module: Vec<Option<SIVictoryConditionsModule>>,
    #[serde(default)]
    pub siparams_module: Vec<Option<SIParamsModule>>,
    #[serde(default)]
    pub sisubsumption_mission_module: Vec<Option<SISubsumptionMissionModule>>,
    #[serde(default)]
    pub schat_channel_type_base: Vec<Option<SChatChannelTypeBase>>,
    #[serde(default)]
    pub chat_system_options_module: Vec<Option<ChatSystemOptionsModule>>,
    #[serde(default)]
    pub sibetting_module: Vec<Option<SIBettingModule>>,
    #[serde(default)]
    pub sidifficulty_module: Vec<Option<SIDifficultyModule>>,
    #[serde(default)]
    pub sireputation_module: Vec<Option<SIReputationModule>>,
    #[serde(default)]
    pub sistate_module: Vec<Option<SIStateModule>>,
    #[serde(default)]
    pub siteams_module: Vec<Option<SITeamsModule>>,
    #[serde(default)]
    pub sivoting_module: Vec<Option<SIVotingModule>>,
    #[serde(default)]
    pub sgeometry_model_tag_base: Vec<Option<SGeometryModelTagBase>>,
    #[serde(default)]
    pub sinitial_damage_specifier_base: Vec<Option<SInitialDamageSpecifierBase>>,
    #[serde(default)]
    pub entity_component_health_sbase_sequencer_task: Vec<Option<EntityComponentHealth_SBaseSequencerTask>>,
    #[serde(default)]
    pub hit_behavior_base: Vec<Option<HitBehaviorBase>>,
    #[serde(default)]
    pub inner_thought_layout_base: Vec<Option<InnerThought_LayoutBase>>,
    #[serde(default)]
    pub shighlight_behavior_node_params: Vec<Option<SHighlightBehaviorNodeParams>>,
    #[serde(default)]
    pub sinteraction_point_primitive_params: Vec<Option<SInteractionPointPrimitiveParams>>,
    #[serde(default)]
    pub sstate_modifier: Vec<Option<SStateModifier>>,
    #[serde(default)]
    pub sentity_context_base: Vec<Option<SEntityContextBase>>,
    #[serde(default)]
    pub attachable_state_modifier_context_base: Vec<Option<AttachableStateModifierContextBase>>,
    #[serde(default)]
    pub sbase_cargo_unit: Vec<Option<SBaseCargoUnit>>,
    #[serde(default)]
    pub inventory_container_grid_cell_size_base: Vec<Option<InventoryContainerGridCellSizeBase>>,
    #[serde(default)]
    pub inventory_container_type_base: Vec<Option<InventoryContainerTypeBase>>,
    #[serde(default)]
    pub base_item: Vec<Option<BaseItem>>,
    #[serde(default)]
    pub base_expiration_type_params: Vec<Option<BaseExpirationTypeParams>>,
    #[serde(default)]
    pub sitem_port_rule_def: Vec<Option<SItemPortRuleDef>>,
    #[serde(default)]
    pub sitem_port_def_extension_base: Vec<Option<SItemPortDefExtensionBase>>,
    #[serde(default)]
    pub sitem_port_def_attachment_implementation_base: Vec<Option<SItemPortDefAttachmentImplementationBase>>,
    #[serde(default)]
    pub placement_validator: Vec<Option<PlacementValidator>>,
    #[serde(default)]
    pub item_resource_dynamic_amount_base: Vec<Option<ItemResourceDynamicAmountBase>>,
    #[serde(default)]
    pub item_resource_conversion_modifier_base: Vec<Option<ItemResourceConversionModifierBase>>,
    #[serde(default)]
    pub item_resource_dynamic_composition_base: Vec<Option<ItemResourceDynamicCompositionBase>>,
    #[serde(default)]
    pub item_resource_control_output_base: Vec<Option<ItemResourceControlOutputBase>>,
    #[serde(default)]
    pub item_resource_control_parameter_base: Vec<Option<ItemResourceControlParameterBase>>,
    #[serde(default)]
    pub item_resource_control_condition_base: Vec<Option<ItemResourceControlConditionBase>>,
    #[serde(default)]
    pub item_resource_dynamic_resource_base: Vec<Option<ItemResourceDynamicResourceBase>>,
    #[serde(default)]
    pub sbase_resource_unit: Vec<Option<SBaseResourceUnit>>,
    #[serde(default)]
    pub item_resource_delta_base: Vec<Option<ItemResourceDeltaBase>>,
    #[serde(default)]
    pub functionality_modifier_base: Vec<Option<FunctionalityModifierBase>>,
    #[serde(default)]
    pub base_journal_entry: Vec<Option<BaseJournalEntry>>,
    #[serde(default)]
    pub ilight_aiextender: Vec<Option<ILightAIExtender>>,
    #[serde(default)]
    pub control_hint_condition: Vec<Option<ControlHintCondition>>,
    #[serde(default)]
    pub control_hint_always_display_condition: Vec<Option<ControlHintAlwaysDisplayCondition>>,
    #[serde(default)]
    pub long_term_persistence_sub_type_list_option: Vec<Option<LongTermPersistenceSubTypeListOption>>,
    #[serde(default)]
    pub entry_optional_data_base: Vec<Option<EntryOptionalData_Base>>,
    #[serde(default)]
    pub entity_class_list_base: Vec<Option<EntityClassList_Base>>,
    #[serde(default)]
    pub pool_filter_base: Vec<Option<PoolFilter_Base>>,
    #[serde(default)]
    pub pool_filter_no_ref: Vec<Option<PoolFilter_NoRef>>,
    #[serde(default)]
    pub loot_table_v3_no_ref: Vec<Option<LootTableV3_NoRef>>,
    #[serde(default)]
    pub loot_archetype_v3_base: Vec<Option<LootArchetypeV3_Base>>,
    #[serde(default)]
    pub loot_archetype_v3_no_ref: Vec<Option<LootArchetypeV3_NoRef>>,
    #[serde(default)]
    pub loot_archetype_v3_selector_base: Vec<Option<LootArchetypeV3Selector_Base>>,
    #[serde(default)]
    pub archetype_optional_data_v3_base: Vec<Option<ArchetypeOptionalDataV3_Base>>,
    #[serde(default)]
    pub spawn_with_v3_selector_base: Vec<Option<SpawnWithV3Selector_Base>>,
    #[serde(default)]
    pub loot_table_optional_data_v3_base: Vec<Option<LootTableOptionalDataV3_Base>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_record_ref_base: Vec<Option<LootV3SecondaryChoicesRecordRef_Base>>,
    #[serde(default)]
    pub loot_v3_secondary_choice_entry_selector_base: Vec<Option<LootV3SecondaryChoiceEntrySelector_Base>>,
    #[serde(default)]
    pub quantity_range_base: Vec<Option<QuantityRange_Base>>,
    #[serde(default)]
    pub map_display_start_mode_base_params: Vec<Option<MapDisplayStartModeBaseParams>>,
    #[serde(default)]
    pub marker_show_rule: Vec<Option<Marker_ShowRule>>,
    #[serde(default)]
    pub marker_ability_base: Vec<Option<Marker_AbilityBase>>,
    #[serde(default)]
    pub visibility_condition_def: Vec<Option<VisibilityConditionDef>>,
    #[serde(default)]
    pub mission_location_validation: Vec<Option<MissionLocationValidation>>,
    #[serde(default)]
    pub contract_generation_params_base: Vec<Option<ContractGenerationParamsBase>>,
    #[serde(default)]
    pub scontract_plugin_base: Vec<Option<SContractPlugin_Base>>,
    #[serde(default)]
    pub contract_prerequisite_base: Vec<Option<ContractPrerequisiteBase>>,
    #[serde(default)]
    pub item_award_weightings_base: Vec<Option<ItemAwardWeightingsBase>>,
    #[serde(default)]
    pub contract_class_base: Vec<Option<ContractClassBase>>,
    #[serde(default)]
    pub hauling_order_content_base: Vec<Option<HaulingOrderContentBase>>,
    #[serde(default)]
    pub hauling_entity_class_list_base: Vec<Option<Hauling_EntityClassListBase>>,
    #[serde(default)]
    pub hauling_order_or_option_base: Vec<Option<HaulingOrder_OrOption_Base>>,
    #[serde(default)]
    pub mission_flow_condition_base: Vec<Option<MissionFlowConditionBase>>,
    #[serde(default)]
    pub mission_flow_action_base: Vec<Option<MissionFlowActionBase>>,
    #[serde(default)]
    pub base_mission_property_value: Vec<Option<BaseMissionPropertyValue>>,
    #[serde(default)]
    pub base_data_set_match_condition: Vec<Option<BaseDataSetMatchCondition>>,
    #[serde(default)]
    pub location_entity_type_base: Vec<Option<LocationEntityType_Base>>,
    #[serde(default)]
    pub module_declaration_type_base: Vec<Option<ModuleDeclarationType_Base>>,
    #[serde(default)]
    pub objective_property_base: Vec<Option<ObjectivePropertyBase>>,
    #[serde(default)]
    pub objective_handler_base: Vec<Option<ObjectiveHandlerBase>>,
    #[serde(default)]
    pub objective_reward_contribution_base: Vec<Option<ObjectiveRewardContributionBase>>,
    #[serde(default)]
    pub music_logic_node: Vec<Option<MusicLogicNode>>,
    #[serde(default)]
    pub sobject_data_bank_entry_tracker_params: Vec<Option<SObjectDataBankEntryTrackerParams>>,
    #[serde(default)]
    pub sandbox_infraction_base_def: Vec<Option<SandboxInfractionBaseDef>>,
    #[serde(default)]
    pub sandbox_trigger_base_def: Vec<Option<SandboxTriggerBaseDef>>,
    #[serde(default)]
    pub mission_complete_perk_base_def: Vec<Option<MissionCompletePerkBaseDef>>,
    #[serde(default)]
    pub cooling_equalization_params_base: Vec<Option<CoolingEqualizationParamsBase>>,
    #[serde(default)]
    pub player_animated_interaction_base: Vec<Option<PlayerAnimatedInteractionBase>>,
    #[serde(default)]
    pub player_choice_menu_option: Vec<Option<PlayerChoiceMenuOption>>,
    #[serde(default)]
    pub planet_day_night_temperature_base_params: Vec<Option<PlanetDayNightTemperatureBaseParams>>,
    #[serde(default)]
    pub procedural_layout_node_base: Vec<Option<ProceduralLayoutNode_Base>>,
    #[serde(default)]
    pub squantum_drive_effect_base_params: Vec<Option<SQuantumDriveEffectBaseParams>>,
    #[serde(default)]
    pub scitem_radar_sensitivity_modifier_type: Vec<Option<SCItemRadarSensitivityModifierType>>,
    #[serde(default)]
    pub scan_custom_value: Vec<Option<ScanCustomValue>>,
    #[serde(default)]
    pub scan_display_condition_base_params: Vec<Option<ScanDisplayConditionBaseParams>>,
    #[serde(default)]
    pub sbbdynamic_property_base: Vec<Option<SBBDynamicPropertyBase>>,
    #[serde(default)]
    pub sreputation_state_modifier_base: Vec<Option<SReputationStateModifierBase>>,
    #[serde(default)]
    pub sreputation_mission_requirement_expression_element: Vec<Option<SReputationMissionRequirementExpressionElement>>,
    #[serde(default)]
    pub base_density_unit: Vec<Option<BaseDensityUnit>>,
    #[serde(default)]
    pub resource_type_density_type: Vec<Option<ResourceTypeDensityType>>,
    #[serde(default)]
    pub room_extension: Vec<Option<RoomExtension>>,
    #[serde(default)]
    pub volume_shape: Vec<Option<VolumeShape>>,
    #[serde(default)]
    pub satmospheric_composition_base_params: Vec<Option<SAtmosphericCompositionBaseParams>>,
    #[serde(default)]
    pub asteroid_state_base: Vec<Option<AsteroidStateBase>>,
    #[serde(default)]
    pub atmosphere_state_base: Vec<Option<AtmosphereStateBase>>,
    #[serde(default)]
    pub aerodynamic_trail_calculation: Vec<Option<AerodynamicTrailCalculation>>,
    #[serde(default)]
    pub electrical_state_base: Vec<Option<ElectricalStateBase>>,
    #[serde(default)]
    pub radiation_state_base: Vec<Option<RadiationStateBase>>,
    #[serde(default)]
    pub sbase_interaction_gameplay_trigger: Vec<Option<SBaseInteractionGameplayTrigger>>,
    #[serde(default)]
    pub sdisruption_gameplay_trigger_type: Vec<Option<SDisruptionGameplayTriggerType>>,
    #[serde(default)]
    pub degradation_action: Vec<Option<DegradationAction>>,
    #[serde(default)]
    pub loadout_check_type: Vec<Option<LoadoutCheckType>>,
    #[serde(default)]
    pub scheck_type: Vec<Option<SCheckType>>,
    #[serde(default)]
    pub gameplay_trigger_target_type_base: Vec<Option<GameplayTrigger_TargetType_Base>>,
    #[serde(default)]
    pub gameplay_trigger_filter_type_base: Vec<Option<GameplayTrigger_FilterType_Base>>,
    #[serde(default)]
    pub gameplay_trigger_executor_activate_interaction_base: Vec<Option<GameplayTrigger_Executor_ActivateInteraction_Base>>,
    #[serde(default)]
    pub gameplay_trigger_executor_set_interaction_state_base: Vec<Option<GameplayTrigger_Executor_SetInteractionState_Base>>,
    #[serde(default)]
    pub gt_communication_message: Vec<Option<GT_CommunicationMessage>>,
    #[serde(default)]
    pub gt_communication_target: Vec<Option<GT_CommunicationTarget>>,
    #[serde(default)]
    pub entity_filter: Vec<Option<EntityFilter>>,
    #[serde(default)]
    pub area_communication_message: Vec<Option<AreaCommunicationMessage>>,
    #[serde(default)]
    pub self_communication_message: Vec<Option<SelfCommunicationMessage>>,
    #[serde(default)]
    pub gameplay_trigger_condition: Vec<Option<GameplayTriggerCondition>>,
    #[serde(default)]
    pub conditional_result: Vec<Option<ConditionalResult>>,
    #[serde(default)]
    pub triggered_health: Vec<Option<TriggeredHealth>>,
    #[serde(default)]
    pub smod_health: Vec<Option<SModHealth>>,
    #[serde(default)]
    pub vulnerability_state: Vec<Option<VulnerabilityState>>,
    #[serde(default)]
    pub gameplay_trigger_physics_set_parameter_base: Vec<Option<GameplayTrigger_Physics_SetParameter_Base>>,
    #[serde(default)]
    pub suser_variable_operation_type: Vec<Option<SUserVariableOperationType>>,
    #[serde(default)]
    pub gameplay_trigger_interpolation_type: Vec<Option<GameplayTrigger_InterpolationType>>,
    #[serde(default)]
    pub clothing_type: Vec<Option<ClothingType>>,
    #[serde(default)]
    pub purchasable_display_base: Vec<Option<PurchasableDisplayBase>>,
    #[serde(default)]
    pub scitem_proximity_sensor_shape_params: Vec<Option<SCItemProximitySensorShapeParams>>,
    #[serde(default)]
    pub sdoor_collision_reaction_base_params: Vec<Option<SDoorCollisionReactionBaseParams>>,
    #[serde(default)]
    pub scitem_door_portal_mode_params: Vec<Option<SCItemDoorPortalModeParams>>,
    #[serde(default)]
    pub ssequencer_item_door_task_params: Vec<Option<SSequencerItemDoorTaskParams>>,
    #[serde(default)]
    pub accessibility_base_params: Vec<Option<AccessibilityBaseParams>>,
    #[serde(default)]
    pub selevator_base_collision_params: Vec<Option<SElevatorBaseCollisionParams>>,
    #[serde(default)]
    pub base_holo_display_provider: Vec<Option<BaseHoloDisplayProvider>>,
    #[serde(default)]
    pub world_display_environment_color: Vec<Option<WorldDisplayEnvironmentColor>>,
    #[serde(default)]
    pub stool_arm_deploy_condition_base: Vec<Option<SToolArmDeployCondition_Base>>,
    #[serde(default)]
    pub scitem_turret_angle_limit_params: Vec<Option<SCItemTurretAngleLimitParams>>,
    #[serde(default)]
    pub weapon_aiaiming_method: Vec<Option<WeaponAIAimingMethod>>,
    #[serde(default)]
    pub slauncher_base: Vec<Option<SLauncherBase>>,
    #[serde(default)]
    pub sweapon_condition_base: Vec<Option<SWeaponConditionBase>>,
    #[serde(default)]
    pub sauxiliary_weapon_action_params: Vec<Option<SAuxiliaryWeaponActionParams>>,
    #[serde(default)]
    pub item_recovery_set_condition_def: Vec<Option<ItemRecoverySetConditionDef>>,
    #[serde(default)]
    pub sscsignature_emission_base_modifier: Vec<Option<SSCSignatureEmissionBaseModifier>>,
    #[serde(default)]
    pub sscsignature_params_base: Vec<Option<SSCSignatureParamsBase>>,
    #[serde(default)]
    pub sscsignature_system_cross_section_params: Vec<Option<SSCSignatureSystemCrossSectionParams>>,
    #[serde(default)]
    pub sscsignature_system_audio_modifier: Vec<Option<SSCSignatureSystemAudioModifier>>,
    #[serde(default)]
    pub sscsignature_system_audio_sub_rule: Vec<Option<SSCSignatureSystemAudioSubRule>>,
    #[serde(default)]
    pub completion_type_base: Vec<Option<CompletionTypeBase>>,
    #[serde(default)]
    pub screen_effects_param_value: Vec<Option<ScreenEffects_ParamValue>>,
    #[serde(default)]
    pub security_clearance_outfit_requirement_def: Vec<Option<SecurityClearance_OutfitRequirementDef>>,
    #[serde(default)]
    pub security_manual_input: Vec<Option<SecurityManualInput>>,
    #[serde(default)]
    pub security_notifications: Vec<Option<SecurityNotifications>>,
    #[serde(default)]
    pub security_network_variable_value_base: Vec<Option<SecurityNetworkVariableValue_Base>>,
    #[serde(default)]
    pub security_network_variable_effect_base: Vec<Option<SecurityNetworkVariableEffect_Base>>,
    #[serde(default)]
    pub sservice_beacon_creator_params_base: Vec<Option<SServiceBeaconCreatorParamsBase>>,
    #[serde(default)]
    pub time_value_base: Vec<Option<TimeValue_Base>>,
    #[serde(default)]
    pub transport_permissions_interface: Vec<Option<TransportPermissionsInterface>>,
    #[serde(default)]
    pub item_port_view_information: Vec<Option<ItemPortViewInformation>>,
    #[serde(default)]
    pub loadout_editor_additional_params: Vec<Option<LoadoutEditorAdditionalParams>>,
    #[serde(default)]
    pub area_alignment_slot_type_params: Vec<Option<AreaAlignmentSlotTypeParams>>,
    #[serde(default)]
    pub sspecialized_data_entry: Vec<Option<SSpecializedDataEntry>>,
    #[serde(default)]
    pub usable_slotting_reference_element_base: Vec<Option<UsableSlottingReferenceElementBase>>,
    #[serde(default)]
    pub ssequencer_usable_task: Vec<Option<SSequencerUsableTask>>,
    #[serde(default)]
    pub weather_effects_asteroid: Vec<Option<WeatherEffects_Asteroid>>,
    #[serde(default)]
    pub weather_effects_atmosphere: Vec<Option<WeatherEffects_Atmosphere>>,
    #[serde(default)]
    pub mobi_glas_app_data_base: Vec<Option<MobiGlasAppDataBase>>,
    #[serde(default)]
    pub smobi_glas_app_data_packet_base: Vec<Option<SMobiGlasAppDataPacketBase>>,
    #[serde(default)]
    pub smobi_glas_app_params_base: Vec<Option<SMobiGlasAppParamsBase>>,
}
