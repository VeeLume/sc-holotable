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

/// Pool storage for the `dormant` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DormantPools {
    #[serde(default)]
    pub activity_behavior_request_condition_timed: Vec<Option<ActivityBehaviorRequestCondition_Timed>>,
    #[serde(default)]
    pub activity_behavior_request_animate: Vec<Option<ActivityBehaviorRequest_Animate>>,
    #[serde(default)]
    pub condition_boss: Vec<Option<Condition_Boss>>,
    #[serde(default)]
    pub condition_boss_hpamount_lost_this_boss_phase: Vec<Option<Condition_Boss_HPAmountLostThisBossPhase>>,
    #[serde(default)]
    pub condition_boss_hpproportion_lost_this_boss_phase: Vec<Option<Condition_Boss_HPProportionLostThisBossPhase>>,
    #[serde(default)]
    pub large_observable_extender: Vec<Option<LargeObservableExtender>>,
    #[serde(default)]
    pub fixed_sized_circle_traversal_cost_shape_config: Vec<Option<FixedSizedCircleTraversalCostShapeConfig>>,
    #[serde(default)]
    pub aidirect_control_component_params: Vec<Option<AIDirectControlComponentParams>>,
    #[serde(default)]
    pub subsumption_conversation_component_params: Vec<Option<SubsumptionConversationComponentParams>>,
    #[serde(default)]
    pub subsumption_conversation_link_component_params: Vec<Option<SubsumptionConversationLinkComponentParams>>,
    #[serde(default)]
    pub condition_distance_to_target: Vec<Option<Condition_DistanceToTarget>>,
    #[serde(default)]
    pub condition_or: Vec<Option<Condition_Or>>,
    #[serde(default)]
    pub condition_target_is_ship: Vec<Option<Condition_TargetIsShip>>,
    #[serde(default)]
    pub condition_target_reachable: Vec<Option<Condition_TargetReachable>>,
    #[serde(default)]
    pub condition_target_ship_size_category: Vec<Option<Condition_TargetShipSizeCategory>>,
    #[serde(default)]
    pub condition_yaw_to_target: Vec<Option<Condition_YawToTarget>>,
    #[serde(default)]
    pub seated_tactic_scores: Vec<Option<SeatedTacticScores>>,
    #[serde(default)]
    pub action_area_dynamic_cost_extension: Vec<Option<ActionAreaDynamicCostExtension>>,
    #[serde(default)]
    pub action_area_circling_formation_extension: Vec<Option<ActionAreaCirclingFormationExtension>>,
    #[serde(default)]
    pub scactor_shopping_assistance: Vec<Option<SCActorShoppingAssistance>>,
    #[serde(default)]
    pub status_effect_damage: Vec<Option<StatusEffectDamage>>,
    #[serde(default)]
    pub linked_status_effect_trigger: Vec<Option<LinkedStatusEffectTrigger>>,
    #[serde(default)]
    pub status_effect_multiple_trigger: Vec<Option<StatusEffectMultipleTrigger>>,
    #[serde(default)]
    pub probabilistic_status_trigger: Vec<Option<ProbabilisticStatusTrigger>>,
    #[serde(default)]
    pub status_masked_retrigger_setup_preset: Vec<Option<StatusMaskedRetriggerSetupPreset>>,
    #[serde(default)]
    pub linked_stat_pass_excess_above_threshold: Vec<Option<LinkedStatPassExcessAboveThreshold>>,
    #[serde(default)]
    pub linked_stat_rule_pass_always_positive: Vec<Option<LinkedStatRulePassAlwaysPositive>>,
    #[serde(default)]
    pub linked_stat_rule_pass_above_threshold: Vec<Option<LinkedStatRulePassAboveThreshold>>,
    #[serde(default)]
    pub linked_stat_rule_has_crossed_above_threshold: Vec<Option<LinkedStatRuleHasCrossedAboveThreshold>>,
    #[serde(default)]
    pub linked_stat_rule_pass_below_threshold: Vec<Option<LinkedStatRulePassBelowThreshold>>,
    #[serde(default)]
    pub linked_stat_rule_pass_crossing_below_threshold: Vec<Option<LinkedStatRulePassCrossingBelowThreshold>>,
    #[serde(default)]
    pub animation_scalar_variable: Vec<Option<AnimationScalarVariable>>,
    #[serde(default)]
    pub animation_variable_link: Vec<Option<AnimationVariableLink>>,
    #[serde(default)]
    pub animation_combine_variable: Vec<Option<AnimationCombineVariable>>,
    #[serde(default)]
    pub sasteroid_field_component_params: Vec<Option<SAsteroidFieldComponentParams>>,
    #[serde(default)]
    pub base_building_validation_point_component_params: Vec<Option<BaseBuildingValidationPointComponentParams>>,
    #[serde(default)]
    pub player_base_params: Vec<Option<PlayerBaseParams>>,
    #[serde(default)]
    pub base_building_structure_params: Vec<Option<BaseBuildingStructureParams>>,
    #[serde(default)]
    pub bindings_operations_integer_string_font_style_pair: Vec<Option<BindingsOperations_IntegerStringFontStylePair>>,
    #[serde(default)]
    pub building_blocks_string_localized_pair: Vec<Option<BuildingBlocks_StringLocalizedPair>>,
    #[serde(default)]
    pub building_blocks_bindings_field_base: Vec<Option<BuildingBlocks_BindingsFieldBase>>,
    #[serde(default)]
    pub building_blocks_bindings_operation_base: Vec<Option<BuildingBlocks_BindingsOperationBase>>,
    #[serde(default)]
    pub building_blocks_bindings_boolean_from_string_switch: Vec<Option<BuildingBlocks_BindingsBooleanFromStringSwitch>>,
    #[serde(default)]
    pub bindings_operations_dialogue_event: Vec<Option<BindingsOperations_DialogueEvent>>,
    #[serde(default)]
    pub building_blocks_boolean_integer_pair: Vec<Option<BuildingBlocks_BooleanIntegerPair>>,
    #[serde(default)]
    pub building_blocks_integer_from_boolean_condition_def: Vec<Option<BuildingBlocks_IntegerFromBooleanConditionDef>>,
    #[serde(default)]
    pub building_blocks_integer_from_boolean_condition_sum_def: Vec<Option<BuildingBlocks_IntegerFromBooleanConditionSumDef>>,
    #[serde(default)]
    pub bindings_operations_string_from_string_canvas: Vec<Option<BindingsOperations_StringFromStringCanvas>>,
    #[serde(default)]
    pub bindings_operations_string_from_integer_switch_font_style: Vec<Option<BindingsOperations_StringFromIntegerSwitchFontStyle>>,
    #[serde(default)]
    pub building_blocks_bindings_localization_from_string_switch: Vec<Option<BuildingBlocks_BindingsLocalizationFromStringSwitch>>,
    #[serde(default)]
    pub building_blocks_bindings_localized_random_from_integer: Vec<Option<BuildingBlocks_BindingsLocalizedRandomFromInteger>>,
    #[serde(default)]
    pub building_blocks_bindings_transform_invert: Vec<Option<BuildingBlocks_BindingsTransformInvert>>,
    #[serde(default)]
    pub boids_cylindrical_limiter_rule: Vec<Option<BoidsCylindricalLimiterRule>>,
    #[serde(default)]
    pub boids_area_limiter_rule: Vec<Option<BoidsAreaLimiterRule>>,
    #[serde(default)]
    pub building_blocks_preview_screen_world_origin: Vec<Option<BuildingBlocks_PreviewScreenWorldOrigin>>,
    #[serde(default)]
    pub building_blocks_preview_scene_card: Vec<Option<BuildingBlocks_PreviewSceneCard>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_min_height_behavior: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeMinHeightBehavior>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_drop_target_policy: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeDropTargetPolicy>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_draggable_policy: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeDraggablePolicy>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_grid_pack_direction: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeGridPackDirection>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_unidirectional_scroll_direction: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeUnidirectionalScrollDirection>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_unidirectional_auto_scroll_behavior: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeUnidirectionalAutoScrollBehavior>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_scroll_easing_type: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeScrollEasingType>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_stroke_joint_style: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeStrokeJointStyle>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_segment_easing: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeSegmentEasing>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_slider_mode: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeSliderMode>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_tick_box_mode: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeTickBoxMode>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_canvas_widget_sizing_method: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeCanvasWidgetSizingMethod>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_dust_particle_movement_restriction: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeDustParticleMovementRestriction>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_progress_meter_state: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeProgressMeterState>>,
    #[serde(default)]
    pub building_blocks_widget_line_list: Vec<Option<BuildingBlocks_WidgetLineList>>,
    #[serde(default)]
    pub building_blocks_shape_widget: Vec<Option<BuildingBlocks_ShapeWidget>>,
    #[serde(default)]
    pub building_blocks_entity_widget_base: Vec<Option<BuildingBlocks_EntityWidgetBase>>,
    #[serde(default)]
    pub building_blocks_widget_runtime_volume: Vec<Option<BuildingBlocks_WidgetRuntimeVolume>>,
    #[serde(default)]
    pub building_blocks_widget_card_texture: Vec<Option<BuildingBlocks_WidgetCardTexture>>,
    #[serde(default)]
    pub building_blocks_toggler_base: Vec<Option<BuildingBlocks_TogglerBase>>,
    #[serde(default)]
    pub building_blocks_texture_base: Vec<Option<BuildingBlocks_TextureBase>>,
    #[serde(default)]
    pub building_blocks_trigger_modify_number: Vec<Option<BuildingBlocks_TriggerModifyNumber>>,
    #[serde(default)]
    pub building_blocks_trigger_hyper_link: Vec<Option<BuildingBlocks_TriggerHyperLink>>,
    #[serde(default)]
    pub building_blocks_number_text_format_modifier: Vec<Option<BuildingBlocks_NumberTextFormatModifier>>,
    #[serde(default)]
    pub building_blocks_static_variable_integer_constant: Vec<Option<BuildingBlocks_StaticVariableIntegerConstant>>,
    #[serde(default)]
    pub camera_simple3_pconfig: Vec<Option<CameraSimple3PConfig>>,
    #[serde(default)]
    pub camera_third_person_base_config: Vec<Option<CameraThirdPersonBaseConfig>>,
    #[serde(default)]
    pub camera_orbit_config: Vec<Option<CameraOrbitConfig>>,
    #[serde(default)]
    pub build_mode_config: Vec<Option<BuildModeConfig>>,
    #[serde(default)]
    pub camera_build_mode_config: Vec<Option<CameraBuildModeConfig>>,
    #[serde(default)]
    pub cargo_fill_capacity_value_random_beta: Vec<Option<CargoFillCapacityValue_RandomBeta>>,
    #[serde(default)]
    pub cargo_resource_distribution: Vec<Option<CargoResourceDistribution>>,
    #[serde(default)]
    pub scustomzier_color_rgba8: Vec<Option<SCustomzierColorRGBA8>>,
    #[serde(default)]
    pub scharacter_eye_validation_params: Vec<Option<SCharacterEyeValidationParams>>,
    #[serde(default)]
    pub scharacter_customizer_item_select: Vec<Option<SCharacterCustomizerItemSelect>>,
    #[serde(default)]
    pub scharacter_customizer_preset_item_base: Vec<Option<SCharacterCustomizerPresetItemBase>>,
    #[serde(default)]
    pub scharacter_customizer_preset_item_chf: Vec<Option<SCharacterCustomizerPresetItemCHF>>,
    #[serde(default)]
    pub scharacter_customizer_preset_item_decal: Vec<Option<SCharacterCustomizerPresetItemDecal>>,
    #[serde(default)]
    pub scharacter_customizer_preset: Vec<Option<SCharacterCustomizerPreset>>,
    #[serde(default)]
    pub scharacter_customizer_feature_preset: Vec<Option<SCharacterCustomizerFeaturePreset>>,
    #[serde(default)]
    pub sarchetype_entity_asset_def_base: Vec<Option<SArchetypeEntityAssetDefBase>>,
    #[serde(default)]
    pub sarchetype_asset_entity_def: Vec<Option<SArchetypeAssetEntityDef>>,
    #[serde(default)]
    pub sarchetype_asset_chfdef: Vec<Option<SArchetypeAssetCHFDef>>,
    #[serde(default)]
    pub sarchetype_asset_loadout_def: Vec<Option<SArchetypeAssetLoadoutDef>>,
    #[serde(default)]
    pub sarchetype_asset_tag_def: Vec<Option<SArchetypeAssetTagDef>>,
    #[serde(default)]
    pub cockpit_rule_int: Vec<Option<CockpitRuleInt>>,
    #[serde(default)]
    pub scollectible_component_params: Vec<Option<SCollectibleComponentParams>>,
    #[serde(default)]
    pub bezier_damage: Vec<Option<BezierDamage>>,
    #[serde(default)]
    pub exponential_damage: Vec<Option<ExponentialDamage>>,
    #[serde(default)]
    pub communication_variable_float: Vec<Option<CommunicationVariableFloat>>,
    #[serde(default)]
    pub communication_variable_string: Vec<Option<CommunicationVariableString>>,
    #[serde(default)]
    pub armoury_terminal_params: Vec<Option<ArmouryTerminalParams>>,
    #[serde(default)]
    pub legacy_crafting_recipe_list_direct: Vec<Option<LegacyCraftingRecipeList_Direct>>,
    #[serde(default)]
    pub ground_vehicle_audio_component_params: Vec<Option<GroundVehicleAudioComponentParams>>,
    #[serde(default)]
    pub sentity_object_character_component_params: Vec<Option<SEntityObjectCharacterComponentParams>>,
    #[serde(default)]
    pub ssubstitution_proxy_params: Vec<Option<SSubstitutionProxyParams>>,
    #[serde(default)]
    pub sentity_node_proxy_params: Vec<Option<SEntityNodeProxyParams>>,
    #[serde(default)]
    pub sclip_volume_proxy_params: Vec<Option<SClipVolumeProxyParams>>,
    #[serde(default)]
    pub sscript_proxy_params: Vec<Option<SScriptProxyParams>>,
    #[serde(default)]
    pub dynamic_instance_properties_params: Vec<Option<DynamicInstancePropertiesParams>>,
    #[serde(default)]
    pub lifetime_debug_params: Vec<Option<LifetimeDebugParams>>,
    #[serde(default)]
    pub firing_range_params: Vec<Option<FiringRangeParams>>,
    #[serde(default)]
    pub firing_range_weapon_item: Vec<Option<FiringRangeWeaponItem>>,
    #[serde(default)]
    pub gravity_base_params: Vec<Option<GravityBaseParams>>,
    #[serde(default)]
    pub harvestable_tag_list_string: Vec<Option<HarvestableTagListString>>,
    #[serde(default)]
    pub sub_harvestable_multi_config_ref: Vec<Option<SubHarvestableMultiConfigRef>>,
    #[serde(default)]
    pub harvestable_area_type_manual_override: Vec<Option<HarvestableAreaTypeManualOverride>>,
    #[serde(default)]
    pub linear_modifier_interpolator: Vec<Option<LinearModifierInterpolator>>,
    #[serde(default)]
    pub bezier_modifier_interpolator: Vec<Option<BezierModifierInterpolator>>,
    #[serde(default)]
    pub lifetime_controlled_item_modifier_params: Vec<Option<LifetimeControlledItemModifierParams>>,
    #[serde(default)]
    pub delayed_modifier_trigger_params: Vec<Option<DelayedModifierTriggerParams>>,
    #[serde(default)]
    pub item_resource_room_modifier_params: Vec<Option<ItemResourceRoomModifierParams>>,
    #[serde(default)]
    pub item_signature_modifier_params: Vec<Option<ItemSignatureModifierParams>>,
    #[serde(default)]
    pub item_heat_modifier_params: Vec<Option<ItemHeatModifierParams>>,
    #[serde(default)]
    pub sjump_tunnel_mesh_params: Vec<Option<SJumpTunnelMeshParams>>,
    #[serde(default)]
    pub light_flicker_wave_sphere_params: Vec<Option<LightFlickerWaveSphereParams>>,
    #[serde(default)]
    pub light_flicker_wave_linear_params: Vec<Option<LightFlickerWaveLinearParams>>,
    #[serde(default)]
    pub light_flicker_wave_random_params: Vec<Option<LightFlickerWaveRandomParams>>,
    #[serde(default)]
    pub slocal_player_haptic_params: Vec<Option<SLocalPlayerHapticParams>>,
    #[serde(default)]
    pub smisfire_mean_time_condition: Vec<Option<SMisfireMeanTimeCondition>>,
    #[serde(default)]
    pub smisfire_damage_value: Vec<Option<SMisfireDamageValue>>,
    #[serde(default)]
    pub smisfire_damage_ratio: Vec<Option<SMisfireDamageRatio>>,
    #[serde(default)]
    pub smisfire_specific_damage: Vec<Option<SMisfireSpecificDamage>>,
    #[serde(default)]
    pub sdamage_misfire_effect: Vec<Option<SDamageMisfireEffect>>,
    #[serde(default)]
    pub sburst_misfire_effect: Vec<Option<SBurstMisfireEffect>>,
    #[serde(default)]
    pub sentity_fire_misfire_effect: Vec<Option<SEntityFireMisfireEffect>>,
    #[serde(default)]
    pub slegacy_thruster_misfire_params: Vec<Option<SLegacyThrusterMisfireParams>>,
    #[serde(default)]
    pub particle_effect_fixed_tinting_params: Vec<Option<ParticleEffectFixedTintingParams>>,
    #[serde(default)]
    pub particle_effect_random_tinting_params: Vec<Option<ParticleEffectRandomTintingParams>>,
    #[serde(default)]
    pub placed_surface_effects_component_params: Vec<Option<PlacedSurfaceEffectsComponentParams>>,
    #[serde(default)]
    pub placed_surface_effects_emitter_entry: Vec<Option<PlacedSurfaceEffects_EmitterEntry>>,
    #[serde(default)]
    pub procedural_planet_audio_algorithm_average_position: Vec<Option<ProceduralPlanetAudioAlgorithmAveragePosition>>,
    #[serde(default)]
    pub procedural_planet_audio_algorithm_count: Vec<Option<ProceduralPlanetAudioAlgorithmCount>>,
    #[serde(default)]
    pub projectile_test_params: Vec<Option<ProjectileTestParams>>,
    #[serde(default)]
    pub movement_recording_params: Vec<Option<MovementRecordingParams>>,
    #[serde(default)]
    pub scontrolled_entity_group_condition: Vec<Option<SControlledEntityGroupCondition>>,
    #[serde(default)]
    pub scontrolled_entity_rope_attachment_pair: Vec<Option<SControlledEntityRopeAttachmentPair>>,
    #[serde(default)]
    pub scontrolled_entity_rope_attachment_pairs: Vec<Option<SControlledEntityRopeAttachmentPairs>>,
    #[serde(default)]
    pub scontrolled_entity_static_transform_params: Vec<Option<SControlledEntityStaticTransformParams>>,
    #[serde(default)]
    pub scontrolled_entity_auto_crane_curve: Vec<Option<SControlledEntityAutoCraneCurve>>,
    #[serde(default)]
    pub scontrolled_entity_auto_crane_data: Vec<Option<SControlledEntityAutoCraneData>>,
    #[serde(default)]
    pub scontrolled_entity_dynamic_transform_params: Vec<Option<SControlledEntityDynamicTransformParams>>,
    #[serde(default)]
    pub scontrolled_entity_transform_params: Vec<Option<SControlledEntityTransformParams>>,
    #[serde(default)]
    pub scontrolled_entity_ifcsparams: Vec<Option<SControlledEntityIFCSParams>>,
    #[serde(default)]
    pub scontrolled_entity_params: Vec<Option<SControlledEntityParams>>,
    #[serde(default)]
    pub sauto_crane_params: Vec<Option<SAutoCraneParams>>,
    #[serde(default)]
    pub scontrolled_entity_camera_params: Vec<Option<SControlledEntityCameraParams>>,
    #[serde(default)]
    pub soverride_controlled_entity_camera_params: Vec<Option<SOverrideControlledEntityCameraParams>>,
    #[serde(default)]
    pub sremote_rigid_entity_controlled_entities_params: Vec<Option<SRemoteRigidEntityControlledEntitiesParams>>,
    #[serde(default)]
    pub sremote_rigid_entity_controller_controlled_group_params: Vec<Option<SRemoteRigidEntityControllerControlledGroupParams>>,
    #[serde(default)]
    pub sremote_rigid_entity_controller_one_handed_params: Vec<Option<SRemoteRigidEntityControllerOneHandedParams>>,
    #[serde(default)]
    pub sremote_rigid_entity_controller_params: Vec<Option<SRemoteRigidEntityControllerParams>>,
    #[serde(default)]
    pub ssimpod_component_params: Vec<Option<SSimpodComponentParams>>,
    #[serde(default)]
    pub entity_component_spawn_closet_params: Vec<Option<EntityComponentSpawnClosetParams>>,
    #[serde(default)]
    pub entity_component_spawn_closet_npcparams: Vec<Option<EntityComponentSpawnClosetNPCParams>>,
    #[serde(default)]
    pub entity_component_spawn_closet_transit_manager_params: Vec<Option<EntityComponentSpawnClosetTransitManagerParams>>,
    #[serde(default)]
    pub tile_manager_params: Vec<Option<TileManagerParams>>,
    #[serde(default)]
    pub vending_machine_item_params: Vec<Option<VendingMachineItemParams>>,
    #[serde(default)]
    pub scitem_control_condition_tag: Vec<Option<SCItemControlCondition_Tag>>,
    #[serde(default)]
    pub scitem_control_condition_allowed_by_remote_turret_view: Vec<Option<SCItemControlCondition_AllowedByRemoteTurretView>>,
    #[serde(default)]
    pub scitem_control_condition_and: Vec<Option<SCItemControlCondition_And>>,
    #[serde(default)]
    pub scitem_conditional_priority: Vec<Option<SCItemConditionalPriority>>,
    #[serde(default)]
    pub scitem_conditional_controllable_group_params: Vec<Option<SCItemConditionalControllableGroupParams>>,
    #[serde(default)]
    pub scsticky_filter_component_params: Vec<Option<SCStickyFilterComponentParams>>,
    #[serde(default)]
    pub conversation_node_base_next: Vec<Option<ConversationNode_BaseNext>>,
    #[serde(default)]
    pub conversation_node_variable_game_token: Vec<Option<ConversationNode_VariableGameToken>>,
    #[serde(default)]
    pub conversation_node_flow_graph_event: Vec<Option<ConversationNode_FlowGraphEvent>>,
    #[serde(default)]
    pub rgba8: Vec<Option<RGBA8>>,
    #[serde(default)]
    pub crafting_cost_context_base_non_ref: Vec<Option<CraftingCostContext_Base_NonRef>>,
    #[serde(default)]
    pub crafting_cost_base_non_ref: Vec<Option<CraftingCost_Base_NonRef>>,
    #[serde(default)]
    pub crafting_cost_base_material: Vec<Option<CraftingCost_Base_Material>>,
    #[serde(default)]
    pub crafting_cost_record_ref: Vec<Option<CraftingCost_RecordRef>>,
    #[serde(default)]
    pub crafting_cost_ref: Vec<Option<CraftingCost_Ref>>,
    #[serde(default)]
    pub crafting_option_effect_time: Vec<Option<CraftingOptionEffect_Time>>,
    #[serde(default)]
    pub crafting_gameplay_property_modifier_value_range_base_non_ref: Vec<Option<CraftingGameplayPropertyModifierValueRange_Base_NonRef>>,
    #[serde(default)]
    pub crafting_gameplay_property_modifier_base_non_ref: Vec<Option<CraftingGameplayPropertyModifier_Base_NonRef>>,
    #[serde(default)]
    pub crafting_gameplay_property_modifier_record_ref: Vec<Option<CraftingGameplayPropertyModifier_RecordRef>>,
    #[serde(default)]
    pub crafting_gameplay_property_modifiers_base_non_ref: Vec<Option<CraftingGameplayPropertyModifiers_Base_NonRef>>,
    #[serde(default)]
    pub crafting_gameplay_property_modifiers_record_ref: Vec<Option<CraftingGameplayPropertyModifiers_RecordRef>>,
    #[serde(default)]
    pub crafting_result_item: Vec<Option<CraftingResult_Item>>,
    #[serde(default)]
    pub crafting_result_resource: Vec<Option<CraftingResult_Resource>>,
    #[serde(default)]
    pub crafting_recipe_costs_record_ref: Vec<Option<CraftingRecipeCosts_RecordRef>>,
    #[serde(default)]
    pub crafting_recipe_costs_ref: Vec<Option<CraftingRecipeCosts_Ref>>,
    #[serde(default)]
    pub crafting_recipe_results_base_non_ref: Vec<Option<CraftingRecipeResults_Base_NonRef>>,
    #[serde(default)]
    pub crafting_recipe_results_record_ref: Vec<Option<CraftingRecipeResults_RecordRef>>,
    #[serde(default)]
    pub crafting_recipe_results_ref: Vec<Option<CraftingRecipeResults_Ref>>,
    #[serde(default)]
    pub crafting_process_specific_recipe_data_base_non_ref: Vec<Option<CraftingProcessSpecificRecipeData_Base_NonRef>>,
    #[serde(default)]
    pub crafting_process_specific_recipe_data_refining: Vec<Option<CraftingProcessSpecificRecipeData_Refining>>,
    #[serde(default)]
    pub crafting_recipe_record_ref: Vec<Option<CraftingRecipe_RecordRef>>,
    #[serde(default)]
    pub crafting_recipe_ref: Vec<Option<CraftingRecipe_Ref>>,
    #[serde(default)]
    pub crafting_research_unlock_base_non_ref: Vec<Option<CraftingResearchUnlock_Base_NonRef>>,
    #[serde(default)]
    pub crafting_research_unlock: Vec<Option<CraftingResearchUnlock>>,
    #[serde(default)]
    pub crafting_research_base_non_ref: Vec<Option<CraftingResearch_Base_NonRef>>,
    #[serde(default)]
    pub crafting_blueprint_tier_base_non_ref: Vec<Option<CraftingBlueprintTier_Base_NonRef>>,
    #[serde(default)]
    pub blueprint_category_availability_base_non_ref: Vec<Option<BlueprintCategoryAvailability_Base_NonRef>>,
    #[serde(default)]
    pub blueprint_category_availability_ref: Vec<Option<BlueprintCategoryAvailability_Ref>>,
    #[serde(default)]
    pub crafting_process_base_non_ref: Vec<Option<CraftingProcess_Base_NonRef>>,
    #[serde(default)]
    pub crafting_process_refining: Vec<Option<CraftingProcess_Refining>>,
    #[serde(default)]
    pub crafting_process_repair: Vec<Option<CraftingProcess_Repair>>,
    #[serde(default)]
    pub crafting_process_upgrade: Vec<Option<CraftingProcess_Upgrade>>,
    #[serde(default)]
    pub crafting_process_dismantle: Vec<Option<CraftingProcess_Dismantle>>,
    #[serde(default)]
    pub generic_crafting_process_base_non_ref: Vec<Option<GenericCraftingProcess_Base_NonRef>>,
    #[serde(default)]
    pub default_blueprint_selection_base_non_ref: Vec<Option<DefaultBlueprintSelection_Base_NonRef>>,
    #[serde(default)]
    pub crafted_item_component_params: Vec<Option<CraftedItemComponentParams>>,
    #[serde(default)]
    pub crafting_quality_distribution_uniform: Vec<Option<CraftingQualityDistributionUniform>>,
    #[serde(default)]
    pub camera_component_override_controller_spawn_notifier_component_params: Vec<Option<CameraComponentOverrideControllerSpawnNotifierComponentParams>>,
    #[serde(default)]
    pub game_token_container_params: Vec<Option<GameTokenContainerParams>>,
    #[serde(default)]
    pub audio_group_component_params: Vec<Option<AudioGroupComponentParams>>,
    #[serde(default)]
    pub entity_component_network_params: Vec<Option<EntityComponentNetworkParams>>,
    #[serde(default)]
    pub random_float: Vec<Option<RandomFloat>>,
    #[serde(default)]
    pub explosion_by_record_params: Vec<Option<ExplosionByRecordParams>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_not: Vec<Option<DefaultActionsEntityEntryCondition_NOT>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition_can_actor_be_looted: Vec<Option<DefaultActionsEntityEntryCondition_CanActorBeLooted>>,
    #[serde(default)]
    pub default_actions_entity_state_can_actor_be_looted: Vec<Option<DefaultActionsEntityState_CanActorBeLooted>>,
    #[serde(default)]
    pub default_actions_entity_state_entity_is_attached_to_other_actor: Vec<Option<DefaultActionsEntityState_EntityIsAttachedToOtherActor>>,
    #[serde(default)]
    pub default_actions_entity_state_entity_is_attached_to_lootable_actor: Vec<Option<DefaultActionsEntityState_EntityIsAttachedToLootableActor>>,
    #[serde(default)]
    pub default_actions_entity_state_entity_is_attached_to_draggable_body: Vec<Option<DefaultActionsEntityState_EntityIsAttachedToDraggableBody>>,
    #[serde(default)]
    pub default_actions_entity_state_in_restrain_range: Vec<Option<DefaultActionsEntityState_InRestrainRange>>,
    #[serde(default)]
    pub dialogue_context_embedded_entry: Vec<Option<DialogueContextEmbeddedEntry>>,
    #[serde(default)]
    pub dialogue_context_reference_entry: Vec<Option<DialogueContextReferenceEntry>>,
    #[serde(default)]
    pub dialogue_bundle_entry: Vec<Option<DialogueBundleEntry>>,
    #[serde(default)]
    pub weighted_dialogue_bundle_entry: Vec<Option<WeightedDialogueBundleEntry>>,
    #[serde(default)]
    pub dialogue_bundle_reference: Vec<Option<DialogueBundleReference>>,
    #[serde(default)]
    pub dialogue_bundle_embedded: Vec<Option<DialogueBundleEmbedded>>,
    #[serde(default)]
    pub dialogue_bundle_weighted: Vec<Option<DialogueBundleWeighted>>,
    #[serde(default)]
    pub dialogue_bundle_weighted_reference: Vec<Option<DialogueBundleWeightedReference>>,
    #[serde(default)]
    pub dialogue_bundle_weighted_embedded: Vec<Option<DialogueBundleWeightedEmbedded>>,
    #[serde(default)]
    pub seauimessage_component_params: Vec<Option<SEAUIMessageComponentParams>>,
    #[serde(default)]
    pub easerialized_carrier: Vec<Option<EASerializedCarrier>>,
    #[serde(default)]
    pub sentity_targeting_properties: Vec<Option<SEntityTargetingProperties>>,
    #[serde(default)]
    pub radar_quantum_override_params: Vec<Option<RadarQuantumOverrideParams>>,
    #[serde(default)]
    pub sattachment_implementation_bone: Vec<Option<SAttachmentImplementationBone>>,
    #[serde(default)]
    pub quantum_grid_component_params: Vec<Option<QuantumGridComponentParams>>,
    #[serde(default)]
    pub sbase_object_container_component_params: Vec<Option<SBaseObjectContainerComponentParams>>,
    #[serde(default)]
    pub sstreaming_object_container_component_params: Vec<Option<SStreamingObjectContainerComponentParams>>,
    #[serde(default)]
    pub shake_component_params: Vec<Option<ShakeComponentParams>>,
    #[serde(default)]
    pub skyline_component_params: Vec<Option<SkylineComponentParams>>,
    #[serde(default)]
    pub area_base_component_params: Vec<Option<AreaBaseComponentParams>>,
    #[serde(default)]
    pub mission_beacon_provider_entity_component_params: Vec<Option<MissionBeaconProviderEntityComponentParams>>,
    #[serde(default)]
    pub inventory_provider_entity_component_params: Vec<Option<InventoryProviderEntityComponentParams>>,
    #[serde(default)]
    pub bindings_subsumption_broadcast_event: Vec<Option<Bindings_SubsumptionBroadcastEvent>>,
    #[serde(default)]
    pub uibindings_subsumption_component_params: Vec<Option<UIBindingsSubsumptionComponentParams>>,
    #[serde(default)]
    pub string_user_variable_set_value_task: Vec<Option<StringUserVariableSetValueTask>>,
    #[serde(default)]
    pub record_ref_user_variable_type_canvas_record: Vec<Option<RecordRefUserVariableTypeCanvasRecord>>,
    #[serde(default)]
    pub dummy_test_arrays_component_params: Vec<Option<DummyTestArraysComponentParams>>,
    #[serde(default)]
    pub fo_ipconfiguration_provider_params: Vec<Option<FoIPConfigurationProviderParams>>,
    #[serde(default)]
    pub ss42_subsumption_mission_component_params: Vec<Option<SS42SubsumptionMissionComponentParams>>,
    #[serde(default)]
    pub srender_to_texture_view_base_params: Vec<Option<SRenderToTextureViewBaseParams>>,
    #[serde(default)]
    pub character_customizer_controller_params: Vec<Option<CharacterCustomizerControllerParams>>,
    #[serde(default)]
    pub sentity_component_vehicle_seater_params: Vec<Option<SEntityComponentVehicleSeaterParams>>,
    #[serde(default)]
    pub landing_spline_visual_arparams: Vec<Option<LandingSplineVisualARParams>>,
    #[serde(default)]
    pub quad_geometry_entity_component_params: Vec<Option<QuadGeometryEntityComponentParams>>,
    #[serde(default)]
    pub rearm_refuel_terminal_params: Vec<Option<RearmRefuelTerminalParams>>,
    #[serde(default)]
    pub scigtest_base: Vec<Option<SCIGTestBase>>,
    #[serde(default)]
    pub scigtest_a: Vec<Option<SCIGTestA>>,
    #[serde(default)]
    pub scigtest_aa: Vec<Option<SCIGTestAA>>,
    #[serde(default)]
    pub scigtest_aaa: Vec<Option<SCIGTestAAA>>,
    #[serde(default)]
    pub scigtest_b: Vec<Option<SCIGTestB>>,
    #[serde(default)]
    pub scigtest_bb: Vec<Option<SCIGTestBB>>,
    #[serde(default)]
    pub scigtest_c: Vec<Option<SCIGTestC>>,
    #[serde(default)]
    pub sentity_component_cigtest_aparams: Vec<Option<SEntityComponentCIGTestAParams>>,
    #[serde(default)]
    pub sentity_component_cigtest_bparams: Vec<Option<SEntityComponentCIGTestBParams>>,
    #[serde(default)]
    pub code_driven_chat_provider_settings: Vec<Option<CodeDrivenChatProviderSettings>>,
    #[serde(default)]
    pub sloadout_requirement_and: Vec<Option<SLoadoutRequirementAND>>,
    #[serde(default)]
    pub sloadout_requirement_not: Vec<Option<SLoadoutRequirementNOT>>,
    #[serde(default)]
    pub sdirt_accumulator_params: Vec<Option<SDirtAccumulatorParams>>,
    #[serde(default)]
    pub swetness_accumulator_params: Vec<Option<SWetnessAccumulatorParams>>,
    #[serde(default)]
    pub ssequencer_move_to_animate_entity_drag_task_params: Vec<Option<SSequencerMoveToAnimateEntityDragTaskParams>>,
    #[serde(default)]
    pub ssequencer_bespoke_entity_drag_task_params: Vec<Option<SSequencerBespokeEntityDragTaskParams>>,
    #[serde(default)]
    pub eatransport_qtravel_transition_params: Vec<Option<EATransportQTravelTransitionParams>>,
    #[serde(default)]
    pub eatransport_spline_transition_params: Vec<Option<EATransportSplineTransitionParams>>,
    #[serde(default)]
    pub sthruster_misfire_params: Vec<Option<SThrusterMisfireParams>>,
    #[serde(default)]
    pub spower_plant_misfire_params: Vec<Option<SPowerPlantMisfireParams>>,
    #[serde(default)]
    pub scooler_misfire_params: Vec<Option<SCoolerMisfireParams>>,
    #[serde(default)]
    pub sshield_generator_misfire_params: Vec<Option<SShieldGeneratorMisfireParams>>,
    #[serde(default)]
    pub shacking_threat_level_condition_ability_started: Vec<Option<SHackingThreatLevelConditionAbilityStarted>>,
    #[serde(default)]
    pub shacking_threat_level_condition_link_point_activated: Vec<Option<SHackingThreatLevelConditionLinkPointActivated>>,
    #[serde(default)]
    pub shacking_threat_level_condition_node_swapped: Vec<Option<SHackingThreatLevelConditionNodeSwapped>>,
    #[serde(default)]
    pub shacking_threat_level_condition_time: Vec<Option<SHackingThreatLevelConditionTime>>,
    #[serde(default)]
    pub sentities_group_component_params: Vec<Option<SEntitiesGroupComponentParams>>,
    #[serde(default)]
    pub sentity_component_nav_point_params: Vec<Option<SEntityComponentNavPointParams>>,
    #[serde(default)]
    pub smission_location_object_metadata_params: Vec<Option<SMissionLocationObjectMetadataParams>>,
    #[serde(default)]
    pub strack_view_outfit_external_swap_data: Vec<Option<STrackViewOutfitExternalSwapData>>,
    #[serde(default)]
    pub sentity_component_proximity_mirror_params: Vec<Option<SEntityComponentProximityMirrorParams>>,
    #[serde(default)]
    pub ssequencer_def_interaction_task_params: Vec<Option<SSequencerDefInteractionTaskParams>>,
    #[serde(default)]
    pub spawner_prerequisite_and: Vec<Option<SpawnerPrerequisite_AND>>,
    #[serde(default)]
    pub spawner_prerequisite_not: Vec<Option<SpawnerPrerequisite_NOT>>,
    #[serde(default)]
    pub sspawner_spawned_entity_params: Vec<Option<SSpawner_SpawnedEntityParams>>,
    #[serde(default)]
    pub sstreaming_dependency_component_params: Vec<Option<SStreamingDependencyComponentParams>>,
    #[serde(default)]
    pub striggerable_devices_behavior_actor_status_params: Vec<Option<STriggerableDevicesBehaviorActorStatusParams>>,
    #[serde(default)]
    pub striggerable_devices_behavior_detach_params: Vec<Option<STriggerableDevicesBehaviorDetachParams>>,
    #[serde(default)]
    pub striggerable_devices_behavior_unprime_params: Vec<Option<STriggerableDevicesBehaviorUnprimeParams>>,
    #[serde(default)]
    pub striggerable_devices_behavior_radar_jammer_params: Vec<Option<STriggerableDevicesBehaviorRadarJammerParams>>,
    #[serde(default)]
    pub striggerable_devices_trigger_all_any_params: Vec<Option<STriggerableDevicesTriggerAllAnyParams>>,
    #[serde(default)]
    pub striggerable_devices_trigger_gravity_params: Vec<Option<STriggerableDevicesTriggerGravityParams>>,
    #[serde(default)]
    pub sentity_component_effects_test: Vec<Option<SEntityComponentEffects_Test>>,
    #[serde(default)]
    pub entity_component_effects_test_tag: Vec<Option<EntityComponentEffects_Test_Tag>>,
    #[serde(default)]
    pub entity_component_effects_test_trigger: Vec<Option<EntityComponentEffects_Test_Trigger>>,
    #[serde(default)]
    pub entity_component_effects_test_link: Vec<Option<EntityComponentEffects_Test_Link>>,
    #[serde(default)]
    pub entity_component_effects_test_color_link: Vec<Option<EntityComponentEffects_Test_ColorLink>>,
    #[serde(default)]
    pub send_landing_gear_obstructed_event: Vec<Option<SendLandingGearObstructedEvent>>,
    #[serde(default)]
    pub send_attempt_wireless_link_event: Vec<Option<SendAttemptWirelessLinkEvent>>,
    #[serde(default)]
    pub set_power_source_state_event: Vec<Option<SetPowerSourceStateEvent>>,
    #[serde(default)]
    pub set_radar_jammer_enabled_event: Vec<Option<SetRadarJammerEnabledEvent>>,
    #[serde(default)]
    pub sentity_density_class_overrides_ref: Vec<Option<SEntityDensityClassOverridesRef>>,
    #[serde(default)]
    pub rttname_tag_params: Vec<Option<RTTNameTagParams>>,
    #[serde(default)]
    pub extinguish_type_sphere: Vec<Option<ExtinguishType_Sphere>>,
    #[serde(default)]
    pub fire_repairer_type_raycast: Vec<Option<FireRepairerType_Raycast>>,
    #[serde(default)]
    pub sapplication_form_music_params: Vec<Option<SApplicationFormMusicParams>>,
    #[serde(default)]
    pub flight_academy_application_form_component_params: Vec<Option<FlightAcademyApplicationFormComponentParams>>,
    #[serde(default)]
    pub sapplication_form_head_homeworld: Vec<Option<SApplicationFormHeadHomeworld>>,
    #[serde(default)]
    pub sapplication_form_relation_params: Vec<Option<SApplicationFormRelationParams>>,
    #[serde(default)]
    pub sapplication_form_body_type_option: Vec<Option<SApplicationFormBodyTypeOption>>,
    #[serde(default)]
    pub sapplication_form_head: Vec<Option<SApplicationFormHead>>,
    #[serde(default)]
    pub sapplication_form_voice_params: Vec<Option<SApplicationFormVoiceParams>>,
    #[serde(default)]
    pub frontend_s42_action_data: Vec<Option<FrontendS42ActionData>>,
    #[serde(default)]
    pub frontend_s42_actions: Vec<Option<FrontendS42Actions>>,
    #[serde(default)]
    pub frontend_s42_scenario: Vec<Option<FrontendS42Scenario>>,
    #[serde(default)]
    pub frontend_controller_provider_sq42_params: Vec<Option<FrontendControllerProviderSQ42Params>>,
    #[serde(default)]
    pub scommon_damage_handling: Vec<Option<SCommonDamageHandling>>,
    #[serde(default)]
    pub sispectator_base: Vec<Option<SISpectatorBase>>,
    #[serde(default)]
    pub snon_authoritative_client_stats_recording: Vec<Option<SNonAuthoritativeClientStatsRecording>>,
    #[serde(default)]
    pub eaphase_base_component_def: Vec<Option<EAPhaseBaseComponentDef>>,
    #[serde(default)]
    pub ss42_player_stats: Vec<Option<SS42PlayerStats>>,
    #[serde(default)]
    pub ssimpod_stats: Vec<Option<SSimpodStats>>,
    #[serde(default)]
    pub sscspawning_base: Vec<Option<SSCSpawningBase>>,
    #[serde(default)]
    pub splayer_loadout_override: Vec<Option<SPlayerLoadoutOverride>>,
    #[serde(default)]
    pub ss42_spawning: Vec<Option<SS42Spawning>>,
    #[serde(default)]
    pub sspawning_base: Vec<Option<SSpawningBase>>,
    #[serde(default)]
    pub seasimpod_spawning: Vec<Option<SEASimpodSpawning>>,
    #[serde(default)]
    pub easpawn_notifier_params: Vec<Option<EASpawnNotifierParams>>,
    #[serde(default)]
    pub easpawn_modifier_params: Vec<Option<EASpawnModifierParams>>,
    #[serde(default)]
    pub easpawn_rez_params: Vec<Option<EASpawnRezParams>>,
    #[serde(default)]
    pub sfpsvictory_conditions_team_lives: Vec<Option<SFPSVictoryConditionsTeamLives>>,
    #[serde(default)]
    pub ssimpod_subsumption_mission: Vec<Option<SSimpodSubsumptionMission>>,
    #[serde(default)]
    pub game_mode_params: Vec<Option<GameModeParams>>,
    #[serde(default)]
    pub eabetting_module_params: Vec<Option<EABettingModuleParams>>,
    #[serde(default)]
    pub s42_difficulty_params: Vec<Option<S42DifficultyParams>>,
    #[serde(default)]
    pub sgame_rules_s42_reputation_params_def: Vec<Option<SGameRulesS42ReputationParamsDef>>,
    #[serde(default)]
    pub game_rules_state_base_params: Vec<Option<GameRulesStateBaseParams>>,
    #[serde(default)]
    pub s42_state_params: Vec<Option<S42StateParams>>,
    #[serde(default)]
    pub easimpod_state_params: Vec<Option<EASimpodStateParams>>,
    #[serde(default)]
    pub geometry_prop_model_tag: Vec<Option<GeometryPropModelTag>>,
    #[serde(default)]
    pub suigeometry_resource_component_params: Vec<Option<SUIGeometryResourceComponentParams>>,
    #[serde(default)]
    pub sbase_health_component_params: Vec<Option<SBaseHealthComponentParams>>,
    #[serde(default)]
    pub sinitial_damage_specifier_fixed: Vec<Option<SInitialDamageSpecifierFixed>>,
    #[serde(default)]
    pub hint_marker_params: Vec<Option<HintMarkerParams>>,
    #[serde(default)]
    pub hit_behavior_damage_over_time: Vec<Option<HitBehaviorDamageOverTime>>,
    #[serde(default)]
    pub shospital_emergency_screen_state_modifier: Vec<Option<SHospitalEmergencyScreenStateModifier>>,
    #[serde(default)]
    pub inner_thought_layout_choice_base: Vec<Option<InnerThought_LayoutChoiceBase>>,
    #[serde(default)]
    pub sentity_link_highlight_node: Vec<Option<SEntityLinkHighlightNode>>,
    #[serde(default)]
    pub sinteraction_link_highlight_node: Vec<Option<SInteractionLinkHighlightNode>>,
    #[serde(default)]
    pub sinteractable_menu_set_title: Vec<Option<SInteractableMenuSetTitle>>,
    #[serde(default)]
    pub sinteraction_point_modifier_apply_actor_offset_by_gravity: Vec<Option<SInteractionPointModifierApplyActorOffsetByGravity>>,
    #[serde(default)]
    pub sinteraction_point_modifier_replace_by_bone_transform: Vec<Option<SInteractionPointModifierReplaceByBoneTransform>>,
    #[serde(default)]
    pub interaction_condition_item_power: Vec<Option<InteractionConditionItemPower>>,
    #[serde(default)]
    pub interaction_condition_actor_inventory_container_entity_has_tag: Vec<Option<InteractionConditionActorInventoryContainerEntityHasTag>>,
    #[serde(default)]
    pub interaction_condition_security_clearance_token: Vec<Option<InteractionConditionSecurityClearanceToken>>,
    #[serde(default)]
    pub interaction_condition_actor_sealed_helmet: Vec<Option<InteractionConditionActorSealedHelmet>>,
    #[serde(default)]
    pub interaction_condition_can_attach_item_on_hanger: Vec<Option<InteractionConditionCanAttachItemOnHanger>>,
    #[serde(default)]
    pub interaction_condition_has_item_type_attached: Vec<Option<InteractionConditionHasItemTypeAttached>>,
    #[serde(default)]
    pub interaction_condition_limit_nearby_carryables: Vec<Option<InteractionConditionLimitNearbyCarryables>>,
    #[serde(default)]
    pub interaction_condition_dragged_actor_has_prohibited_items: Vec<Option<InteractionConditionDraggedActorHasProhibitedItems>>,
    #[serde(default)]
    pub interaction_condition_has_supporting_itemport: Vec<Option<InteractionConditionHasSupportingItemport>>,
    #[serde(default)]
    pub interaction_condition_swap_only: Vec<Option<InteractionConditionSwapOnly>>,
    #[serde(default)]
    pub sinteraction_condition_player_vehicle_not_in_armistice_zone: Vec<Option<SInteractionConditionPlayerVehicleNotInArmisticeZone>>,
    #[serde(default)]
    pub sinteraction_condition_movable_mover: Vec<Option<SInteractionConditionMovableMover>>,
    #[serde(default)]
    pub interaction_condition_hacking_controller_enabled: Vec<Option<InteractionConditionHackingControllerEnabled>>,
    #[serde(default)]
    pub interaction_condition_is_interaction_shown_as_prompt: Vec<Option<InteractionConditionIsInteractionShownAsPrompt>>,
    #[serde(default)]
    pub interaction_condition_held_item_has_tag: Vec<Option<InteractionConditionHeldItemHasTag>>,
    #[serde(default)]
    pub interaction_condition_skill_level: Vec<Option<InteractionConditionSkillLevel>>,
    #[serde(default)]
    pub interaction_condition_stamina_requirement: Vec<Option<InteractionConditionStaminaRequirement>>,
    #[serde(default)]
    pub srange_state_modifier: Vec<Option<SRangeStateModifier>>,
    #[serde(default)]
    pub spower_source_state_modifier: Vec<Option<SPowerSourceStateModifier>>,
    #[serde(default)]
    pub sprison_sentence_state_modifier: Vec<Option<SPrisonSentenceStateModifier>>,
    #[serde(default)]
    pub sitem_expiry_state_modifier: Vec<Option<SItemExpiryStateModifier>>,
    #[serde(default)]
    pub attachable_state_modifier_base: Vec<Option<AttachableStateModifierBase>>,
    #[serde(default)]
    pub landing_system_state_modifier_base: Vec<Option<LandingSystemStateModifierBase>>,
    #[serde(default)]
    pub state_token_requirements: Vec<Option<StateTokenRequirements>>,
    #[serde(default)]
    pub state_type_state_token_requirements: Vec<Option<StateTypeStateTokenRequirements>>,
    #[serde(default)]
    pub security_access_token_state_modifier: Vec<Option<SecurityAccessTokenStateModifier>>,
    #[serde(default)]
    pub collectible_state_modifier: Vec<Option<CollectibleStateModifier>>,
    #[serde(default)]
    pub inventory_container_grid_cell_size_meters: Vec<Option<InventoryContainerGridCellSizeMeters>>,
    #[serde(default)]
    pub vehicle_item: Vec<Option<VehicleItem>>,
    #[serde(default)]
    pub timer_expiration_params: Vec<Option<TimerExpirationParams>>,
    #[serde(default)]
    pub date_expiration_params: Vec<Option<DateExpirationParams>>,
    #[serde(default)]
    pub scitem_expiry_component_params: Vec<Option<SCItemExpiryComponentParams>>,
    #[serde(default)]
    pub sitem_port_rule_hide_interactions_def: Vec<Option<SItemPortRule_HideInteractionsDef>>,
    #[serde(default)]
    pub ssimulation_params_pendulum: Vec<Option<SSimulationParamsPendulum>>,
    #[serde(default)]
    pub ssimulation_params_translational_projection: Vec<Option<SSimulationParamsTranslationalProjection>>,
    #[serde(default)]
    pub sitem_port_def_attachment_implementation_face: Vec<Option<SItemPortDefAttachmentImplementationFace>>,
    #[serde(default)]
    pub sitem_port_def_attachment_implementation_logical: Vec<Option<SItemPortDefAttachmentImplementationLogical>>,
    #[serde(default)]
    pub item_port_host_component_params: Vec<Option<ItemPortHostComponentParams>>,
    #[serde(default)]
    pub item_resource_conversion_pair: Vec<Option<ItemResourceConversionPair>>,
    #[serde(default)]
    pub item_resource_delta_inject: Vec<Option<ItemResourceDeltaInject>>,
    #[serde(default)]
    pub item_resource_modifier: Vec<Option<ItemResourceModifier>>,
    #[serde(default)]
    pub functionality_modifier_linear: Vec<Option<FunctionalityModifierLinear>>,
    #[serde(default)]
    pub functionality_modifier_curve: Vec<Option<FunctionalityModifierCurve>>,
    #[serde(default)]
    pub item_resource_dynamic_amount_example: Vec<Option<ItemResourceDynamicAmountExample>>,
    #[serde(default)]
    pub item_resource_dynamic_amount_functionality_ratio: Vec<Option<ItemResourceDynamicAmountFunctionalityRatio>>,
    #[serde(default)]
    pub item_resource_dynamic_amount_weapon_ammo: Vec<Option<ItemResourceDynamicAmountWeaponAmmo>>,
    #[serde(default)]
    pub item_resource_dynamic_amount_weapon_regen: Vec<Option<ItemResourceDynamicAmountWeaponRegen>>,
    #[serde(default)]
    pub item_resource_conversion_modifier_scrub: Vec<Option<ItemResourceConversionModifierScrub>>,
    #[serde(default)]
    pub item_resource_dynamic_composition_example: Vec<Option<ItemResourceDynamicCompositionExample>>,
    #[serde(default)]
    pub item_resource_control_parameter_rnparameter: Vec<Option<ItemResourceControlParameterRNParameter>>,
    #[serde(default)]
    pub item_resource_control_rnparameter_external_port: Vec<Option<ItemResourceControlRNParameterExternalPort>>,
    #[serde(default)]
    pub item_resource_control_output_state: Vec<Option<ItemResourceControlOutputState>>,
    #[serde(default)]
    pub item_resource_control_output_control_block: Vec<Option<ItemResourceControlOutputControlBlock>>,
    #[serde(default)]
    pub item_resource_control_output_change_interaction_state: Vec<Option<ItemResourceControlOutputChangeInteractionState>>,
    #[serde(default)]
    pub item_resource_control_condition_limit: Vec<Option<ItemResourceControlConditionLimit>>,
    #[serde(default)]
    pub item_resource_control_condition_limit_parameter: Vec<Option<ItemResourceControlConditionLimitParameter>>,
    #[serde(default)]
    pub item_resource_control_condition_gravity: Vec<Option<ItemResourceControlConditionGravity>>,
    #[serde(default)]
    pub item_resource_control_condition_throttle: Vec<Option<ItemResourceControlConditionThrottle>>,
    #[serde(default)]
    pub journal_entry_mission_progress_text: Vec<Option<JournalEntryMissionProgressText>>,
    #[serde(default)]
    pub sjump_point_access_params: Vec<Option<SJumpPointAccessParams>>,
    #[serde(default)]
    pub refuel_base_service: Vec<Option<RefuelBaseService>>,
    #[serde(default)]
    pub light_aivision_helper_extender: Vec<Option<LightAIVisionHelperExtender>>,
    #[serde(default)]
    pub interference_params_one_shot: Vec<Option<InterferenceParams_OneShot>>,
    #[serde(default)]
    pub interference_params_continuous: Vec<Option<InterferenceParams_Continuous>>,
    #[serde(default)]
    pub lightning_target_mode_entity: Vec<Option<LightningTargetMode_Entity>>,
    #[serde(default)]
    pub lightning_shape_ellipsoid: Vec<Option<LightningShape_Ellipsoid>>,
    #[serde(default)]
    pub usable_channel_input_action_trigger_sequence: Vec<Option<UsableChannelInputAction_TriggerSequence>>,
    #[serde(default)]
    pub usable_channel_input_action_consume_with_usable: Vec<Option<UsableChannelInputAction_ConsumeWithUsable>>,
    #[serde(default)]
    pub usable_channel_input_action_exercise: Vec<Option<UsableChannelInputAction_Exercise>>,
    #[serde(default)]
    pub skill_requirement: Vec<Option<SkillRequirement>>,
    #[serde(default)]
    pub skill_gain: Vec<Option<SkillGain>>,
    #[serde(default)]
    pub interactive_variable_animate_to_default_float_params: Vec<Option<InteractiveVariable_AnimateToDefaultFloatParams>>,
    #[serde(default)]
    pub interactive_variable_snap_to_default_float_params: Vec<Option<InteractiveVariable_SnapToDefaultFloatParams>>,
    #[serde(default)]
    pub loadout_response_params: Vec<Option<LoadoutResponseParams>>,
    #[serde(default)]
    pub entity_component_control_hints_params: Vec<Option<EntityComponentControlHintsParams>>,
    #[serde(default)]
    pub control_hints_usable_linked_interactive_action: Vec<Option<ControlHints_UsableLinkedInteractiveAction>>,
    #[serde(default)]
    pub control_hint_condition_actor_ability_available: Vec<Option<ControlHintCondition_ActorAbilityAvailable>>,
    #[serde(default)]
    pub control_hint_condition_actor_ladder_state: Vec<Option<ControlHintCondition_ActorLadderState>>,
    #[serde(default)]
    pub control_hint_condition_actor_zero_gevastate: Vec<Option<ControlHintCondition_ActorZeroGEVAState>>,
    #[serde(default)]
    pub control_hint_condition_actor_tags: Vec<Option<ControlHintCondition_ActorTags>>,
    #[serde(default)]
    pub control_hint_condition_vehicle_scan_wave_availability: Vec<Option<ControlHintCondition_VehicleScanWaveAvailability>>,
    #[serde(default)]
    pub control_hint_condition_vehicle_formation_mode: Vec<Option<ControlHintCondition_VehicleFormationMode>>,
    #[serde(default)]
    pub control_hint_condition_opt_in_event_state: Vec<Option<ControlHintCondition_OptInEventState>>,
    #[serde(default)]
    pub control_hint_condition_hint_trigger: Vec<Option<ControlHintCondition_HintTrigger>>,
    #[serde(default)]
    pub control_hint_condition_any_hint_trigger: Vec<Option<ControlHintCondition_AnyHintTrigger>>,
    #[serde(default)]
    pub entity_class_list_no_ref: Vec<Option<EntityClassList_NoRef>>,
    #[serde(default)]
    pub entity_class_list_record_ref: Vec<Option<EntityClassList_RecordRef>>,
    #[serde(default)]
    pub pool_filter_entity_classes: Vec<Option<PoolFilter_EntityClasses>>,
    #[serde(default)]
    pub loot_archetype_v3_selector_no_ref: Vec<Option<LootArchetypeV3Selector_NoRef>>,
    #[serde(default)]
    pub archetype_optional_data_v3_no_ref: Vec<Option<ArchetypeOptionalDataV3_NoRef>>,
    #[serde(default)]
    pub spawn_with_v3_selector_no_ref: Vec<Option<SpawnWithV3Selector_NoRef>>,
    #[serde(default)]
    pub spawn_with_v3_selector_entity_classes: Vec<Option<SpawnWithV3Selector_EntityClasses>>,
    #[serde(default)]
    pub loot_table_optional_data_v3_no_ref: Vec<Option<LootTableOptionalDataV3_NoRef>>,
    #[serde(default)]
    pub loot_v3_secondary_choice_entry_selector_no_ref: Vec<Option<LootV3SecondaryChoiceEntrySelector_NoRef>>,
    #[serde(default)]
    pub quantity_range_no_ref: Vec<Option<QuantityRange_NoRef>>,
    #[serde(default)]
    pub maelstrom_destroy_event: Vec<Option<MaelstromDestroyEvent>>,
    #[serde(default)]
    pub maelstrom_interaction_state_machine_event: Vec<Option<MaelstromInteractionStateMachineEvent>>,
    #[serde(default)]
    pub smag_recovery_motion_params: Vec<Option<SMagRecoveryMotionParams>>,
    #[serde(default)]
    pub smag_recovery_params: Vec<Option<SMagRecoveryParams>>,
    #[serde(default)]
    pub map_display_params_radar_map_single_mode: Vec<Option<MapDisplayParamsRadarMapSingleMode>>,
    #[serde(default)]
    pub map_display_params_radar_map_multi_mode: Vec<Option<MapDisplayParamsRadarMapMultiMode>>,
    #[serde(default)]
    pub map_display_radar_mode: Vec<Option<MapDisplayRadarMode>>,
    #[serde(default)]
    pub map_display_start_mode_fixed_override_params: Vec<Option<MapDisplayStartModeFixedOverrideParams>>,
    #[serde(default)]
    pub map_display_start_mode_landing_override_params: Vec<Option<MapDisplayStartModeLandingOverrideParams>>,
    #[serde(default)]
    pub marker_ability_scan: Vec<Option<Marker_AbilityScan>>,
    #[serde(default)]
    pub visibility_condition_def_and: Vec<Option<VisibilityConditionDef_AND>>,
    #[serde(default)]
    pub synced_attack_category_params: Vec<Option<SyncedAttackCategoryParams>>,
    #[serde(default)]
    pub aisynced_melee_attack: Vec<Option<AISyncedMeleeAttack>>,
    #[serde(default)]
    pub contract_generation_params_request_only: Vec<Option<ContractGenerationParams_RequestOnly>>,
    #[serde(default)]
    pub contract_prerequisite_journal_entries: Vec<Option<ContractPrerequisite_JournalEntries>>,
    #[serde(default)]
    pub contract_prerequisite_area_tags: Vec<Option<ContractPrerequisite_AreaTags>>,
    #[serde(default)]
    pub item_award_tag: Vec<Option<ItemAwardTag>>,
    #[serde(default)]
    pub contract_class_global_event: Vec<Option<ContractClass_GlobalEvent>>,
    #[serde(default)]
    pub hauling_order_content_resource_base: Vec<Option<HaulingOrderContent_ResourceBase>>,
    #[serde(default)]
    pub hauling_order_content_resource_unlimited_drop_off: Vec<Option<HaulingOrderContent_ResourceUnlimitedDropOff>>,
    #[serde(default)]
    pub hauling_order_property_base: Vec<Option<HaulingOrder_PropertyBase>>,
    #[serde(default)]
    pub hauling_order_property_drop_off: Vec<Option<HaulingOrder_PropertyDropOff>>,
    #[serde(default)]
    pub hauling_order_resource_base: Vec<Option<HaulingOrder_ResourceBase>>,
    #[serde(default)]
    pub hauling_order_mission_item_drop_off: Vec<Option<HaulingOrder_MissionItemDropOff>>,
    #[serde(default)]
    pub hauling_order_or: Vec<Option<HaulingOrder_Or>>,
    #[serde(default)]
    pub investigation_suspect: Vec<Option<Investigation_Suspect>>,
    #[serde(default)]
    pub investigation_clue: Vec<Option<Investigation_Clue>>,
    #[serde(default)]
    pub investigation_clue_journal_entry: Vec<Option<Investigation_Clue_JournalEntry>>,
    #[serde(default)]
    pub investigation_clue_corpse_scan_info: Vec<Option<Investigation_Clue_CorpseScanInfo>>,
    #[serde(default)]
    pub investigation_clue_item: Vec<Option<Investigation_ClueItem>>,
    #[serde(default)]
    pub investigation_clue_item_datapad: Vec<Option<Investigation_ClueItem_Datapad>>,
    #[serde(default)]
    pub investigation_clue_item_corpse: Vec<Option<Investigation_ClueItem_Corpse>>,
    #[serde(default)]
    pub investigation_location: Vec<Option<Investigation_Location>>,
    #[serde(default)]
    pub mission_property_value_investigation: Vec<Option<MissionPropertyValue_Investigation>>,
    #[serde(default)]
    pub mission_variable_float: Vec<Option<MissionVariableFloat>>,
    #[serde(default)]
    pub mission_variable_string: Vec<Option<MissionVariableString>>,
    #[serde(default)]
    pub module_declaration_type_beacon: Vec<Option<ModuleDeclarationType_Beacon>>,
    #[serde(default)]
    pub objective_handler_with_module: Vec<Option<ObjectiveHandler_WithModule>>,
    #[serde(default)]
    pub objective_handler_event_module: Vec<Option<ObjectiveHandler_EventModule>>,
    #[serde(default)]
    pub mobiglas_display_location: Vec<Option<MobiglasDisplayLocation>>,
    #[serde(default)]
    pub mobiglas_display_counter: Vec<Option<MobiglasDisplayCounter>>,
    #[serde(default)]
    pub mission_modifier_shop_item_perks_def: Vec<Option<MissionModifier_ShopItemPerksDef>>,
    #[serde(default)]
    pub smission_staging_area_component_params: Vec<Option<SMissionStagingAreaComponentParams>>,
    #[serde(default)]
    pub spawning_manager_notifier_component_params: Vec<Option<SpawningManagerNotifierComponentParams>>,
    #[serde(default)]
    pub environmental_mission_component_params: Vec<Option<EnvironmentalMissionComponentParams>>,
    #[serde(default)]
    pub mission_data_component_params: Vec<Option<MissionDataComponentParams>>,
    #[serde(default)]
    pub delivery_locker_item_component_params: Vec<Option<DeliveryLockerItemComponentParams>>,
    #[serde(default)]
    pub delivery_item_port_pickup_component_params: Vec<Option<DeliveryItemPortPickupComponentParams>>,
    #[serde(default)]
    pub hauling_mission_helper_component_params: Vec<Option<HaulingMissionHelperComponentParams>>,
    #[serde(default)]
    pub mission_init_param_activity: Vec<Option<MissionInitParamActivity>>,
    #[serde(default)]
    pub sardata_generic_component_params: Vec<Option<SARDataGenericComponentParams>>,
    #[serde(default)]
    pub ss42_galactapedia_source_data_params: Vec<Option<SS42GalactapediaSourceDataParams>>,
    #[serde(default)]
    pub smobi_glas_launcher_app: Vec<Option<SMobiGlasLauncherApp>>,
    #[serde(default)]
    pub entity_component_mobiglas_launcher_provider_params: Vec<Option<EntityComponentMobiglasLauncherProviderParams>>,
    #[serde(default)]
    pub parent_music_logic_node: Vec<Option<ParentMusicLogicNode>>,
    #[serde(default)]
    pub music_logic_condition: Vec<Option<MusicLogicCondition>>,
    #[serde(default)]
    pub comms_notification_stage_light: Vec<Option<CommsNotificationStageLight>>,
    #[serde(default)]
    pub comms_stage_uiprovider_params: Vec<Option<CommsStageUIProviderParams>>,
    #[serde(default)]
    pub comms_notification_user_params: Vec<Option<CommsNotificationUserParams>>,
    #[serde(default)]
    pub squantum_travel_entry_tracker_params: Vec<Option<SQuantumTravelEntryTrackerParams>>,
    #[serde(default)]
    pub sturret_entry_tracker_params: Vec<Option<STurretEntryTrackerParams>>,
    #[serde(default)]
    pub sdebris_entry_tracker_params: Vec<Option<SDebrisEntryTrackerParams>>,
    #[serde(default)]
    pub sunknown_entry_tracker_params: Vec<Option<SUnknownEntryTrackerParams>>,
    #[serde(default)]
    pub sblob_contact_entry_tracker_params: Vec<Option<SBlobContactEntryTrackerParams>>,
    #[serde(default)]
    pub sinteractable_entry_tracker_params: Vec<Option<SInteractableEntryTrackerParams>>,
    #[serde(default)]
    pub sinteraction_point_entry_tracker_params: Vec<Option<SInteractionPointEntryTrackerParams>>,
    #[serde(default)]
    pub shint_entry_tracker_params: Vec<Option<SHintEntryTrackerParams>>,
    #[serde(default)]
    pub sexplosive_entry_tracker_params: Vec<Option<SExplosiveEntryTrackerParams>>,
    #[serde(default)]
    pub reputation_reward_faction: Vec<Option<ReputationRewardFaction>>,
    #[serde(default)]
    pub sentity_overclock_component_params: Vec<Option<SEntityOverclockComponentParams>>,
    #[serde(default)]
    pub sperk_shop_item_category_base: Vec<Option<SPerkShopItemCategoryBase>>,
    #[serde(default)]
    pub sperk_discount_item_category: Vec<Option<SPerkDiscountItemCategory>>,
    #[serde(default)]
    pub sperk_enabled_item_category: Vec<Option<SPerkEnabledItemCategory>>,
    #[serde(default)]
    pub sitem_perk_list_params: Vec<Option<SItemPerkListParams>>,
    #[serde(default)]
    pub smulti_mission_complete_reward: Vec<Option<SMultiMissionCompleteReward>>,
    #[serde(default)]
    pub smulti_mission_complete_reward_params: Vec<Option<SMultiMissionCompleteRewardParams>>,
    #[serde(default)]
    pub sbadge_complete_reward: Vec<Option<SBadgeCompleteReward>>,
    #[serde(default)]
    pub sentity_cgf_grid_property: Vec<Option<SEntityCgfGridProperty>>,
    #[serde(default)]
    pub sentity_sphere_grid_property: Vec<Option<SEntitySphereGridProperty>>,
    #[serde(default)]
    pub sentity_box_grid_property: Vec<Option<SEntityBoxGridProperty>>,
    #[serde(default)]
    pub sentity_object_container_grid_property: Vec<Option<SEntityObjectContainerGridProperty>>,
    #[serde(default)]
    pub sentity_space_ship_physics_controller_params: Vec<Option<SEntitySpaceShipPhysicsControllerParams>>,
    #[serde(default)]
    pub sentity_soft_deprecated_physics_controller_params: Vec<Option<SEntitySoftDeprecatedPhysicsControllerParams>>,
    #[serde(default)]
    pub sentity_sphere_physics_grid_params: Vec<Option<SEntitySpherePhysicsGridParams>>,
    #[serde(default)]
    pub sentity_box_physics_grid_params: Vec<Option<SEntityBoxPhysicsGridParams>>,
    #[serde(default)]
    pub sentity_voxel_physics_grid_params: Vec<Option<SEntityVoxelPhysicsGridParams>>,
    #[serde(default)]
    pub sentity_cgfphysics_grid_params: Vec<Option<SEntityCGFPhysicsGridParams>>,
    #[serde(default)]
    pub sentity_vis_area_physics_grid_params: Vec<Option<SEntityVisAreaPhysicsGridParams>>,
    #[serde(default)]
    pub sactor_default_death_behaviour: Vec<Option<SActorDefaultDeathBehaviour>>,
    #[serde(default)]
    pub planet_day_night_temperature_template_ref: Vec<Option<PlanetDayNightTemperatureTemplateRef>>,
    #[serde(default)]
    pub procedural_layout_graph_node_base: Vec<Option<ProceduralLayoutGraphNode_Base>>,
    #[serde(default)]
    pub procedural_layout_node_alternate_element_properties: Vec<Option<ProceduralLayoutNode_AlternateElementProperties>>,
    #[serde(default)]
    pub procedural_layout_graph_node_alternate_element: Vec<Option<ProceduralLayoutGraphNode_AlternateElement>>,
    #[serde(default)]
    pub procedural_layout_graph_node_corridor_element: Vec<Option<ProceduralLayoutGraphNode_CorridorElement>>,
    #[serde(default)]
    pub procedural_layout_node_alternate_sub_graph_properties: Vec<Option<ProceduralLayoutNode_AlternateSubGraphProperties>>,
    #[serde(default)]
    pub procedural_layout_graph_node_alternate_sub_graph: Vec<Option<ProceduralLayoutGraphNode_AlternateSubGraph>>,
    #[serde(default)]
    pub procedural_layout_graph_node_vertical_element: Vec<Option<ProceduralLayoutGraphNode_VerticalElement>>,
    #[serde(default)]
    pub quantum_drive_effect_params: Vec<Option<QuantumDriveEffectParams>>,
    #[serde(default)]
    pub behavior_custom_quantum_drive_effects_preset: Vec<Option<Behavior_CustomQuantumDriveEffectsPreset>>,
    #[serde(default)]
    pub squantum_boost_audio_params: Vec<Option<SQuantumBoostAudioParams>>,
    #[serde(default)]
    pub squantum_rotation_params: Vec<Option<SQuantumRotationParams>>,
    #[serde(default)]
    pub squantum_rotation_kick_params: Vec<Option<SQuantumRotationKickParams>>,
    #[serde(default)]
    pub squantum_bubble_integrity_params: Vec<Option<SQuantumBubbleIntegrityParams>>,
    #[serde(default)]
    pub squantum_timer_data: Vec<Option<SQuantumTimerData>>,
    #[serde(default)]
    pub squantum_timers: Vec<Option<SQuantumTimers>>,
    #[serde(default)]
    pub squantum_drive_boost_params: Vec<Option<SQuantumDriveBoostParams>>,
    #[serde(default)]
    pub squantum_drive_travel_params: Vec<Option<SQuantumDriveTravelParams>>,
    #[serde(default)]
    pub squantum_drive_vibration_noise_params: Vec<Option<SQuantumDriveVibrationNoiseParams>>,
    #[serde(default)]
    pub squantum_drive_vibration_params: Vec<Option<SQuantumDriveVibrationParams>>,
    #[serde(default)]
    pub squantum_hud_messages: Vec<Option<SQuantumHudMessages>>,
    #[serde(default)]
    pub scitem_quantum_drive_params_new: Vec<Option<SCItemQuantumDriveParams_NEW>>,
    #[serde(default)]
    pub squantum_resource_network_params: Vec<Option<SQuantumResourceNetworkParams>>,
    #[serde(default)]
    pub sentity_component_orbital_nav_point_params: Vec<Option<SEntityComponentOrbitalNavPointParams>>,
    #[serde(default)]
    pub squantum_camera_state_effects_time_def: Vec<Option<SQuantumCameraStateEffectsTimeDef>>,
    #[serde(default)]
    pub contact_highlight_material_params: Vec<Option<ContactHighlightMaterialParams>>,
    #[serde(default)]
    pub scitem_radar_sensitivity_modifier_type_contacts: Vec<Option<SCItemRadarSensitivityModifierTypeContacts>>,
    #[serde(default)]
    pub scitem_radar_sensitivity_modifier_type_signature_category: Vec<Option<SCItemRadarSensitivityModifierTypeSignatureCategory>>,
    #[serde(default)]
    pub scan_custom_value_array: Vec<Option<ScanCustomValueArray>>,
    #[serde(default)]
    pub scan_data_entry_params: Vec<Option<ScanDataEntryParams>>,
    #[serde(default)]
    pub scan_display_armor_variable_params: Vec<Option<ScanDisplayArmorVariableParams>>,
    #[serde(default)]
    pub status_priority_float: Vec<Option<StatusPriorityFloat>>,
    #[serde(default)]
    pub status_priority_stamina: Vec<Option<StatusPriorityStamina>>,
    #[serde(default)]
    pub classification_scan_procedure_params: Vec<Option<ClassificationScanProcedureParams>>,
    #[serde(default)]
    pub cryopod_scan_procedure_params: Vec<Option<CryopodScanProcedureParams>>,
    #[serde(default)]
    pub jurisdiction_scan_procedure_params: Vec<Option<JurisdictionScanProcedureParams>>,
    #[serde(default)]
    pub vehicle_self_destruct_scan_procedure_params: Vec<Option<VehicleSelfDestructScanProcedureParams>>,
    #[serde(default)]
    pub sbbdynamic_property_int: Vec<Option<SBBDynamicPropertyInt>>,
    #[serde(default)]
    pub sbbdynamic_property_float: Vec<Option<SBBDynamicPropertyFloat>>,
    #[serde(default)]
    pub sreputation_state_modifier_decrement: Vec<Option<SReputationStateModifierDecrement>>,
    #[serde(default)]
    pub sreputation_mission_requirement_expression_or: Vec<Option<SReputationMissionRequirementExpression_Or>>,
    #[serde(default)]
    pub sreputation_mission_requirement_expression_not: Vec<Option<SReputationMissionRequirementExpression_Not>>,
    #[serde(default)]
    pub kilograms_per_cubic_meter: Vec<Option<KilogramsPerCubicMeter>>,
    #[serde(default)]
    pub resource_type_molar_mass: Vec<Option<ResourceTypeMolarMass>>,
    #[serde(default)]
    pub scitem_restraint_params: Vec<Option<SCItemRestraintParams>>,
    #[serde(default)]
    pub volume_shape_box: Vec<Option<VolumeShape_Box>>,
    #[serde(default)]
    pub volume_shape_ellipsoid: Vec<Option<VolumeShape_Ellipsoid>>,
    #[serde(default)]
    pub satmospheric_composition_inherit_planet: Vec<Option<SAtmosphericCompositionInheritPlanet>>,
    #[serde(default)]
    pub aerodynamic_trail_calculation_gas_cloud_optical_density_range: Vec<Option<AerodynamicTrailCalculationGasCloudOpticalDensityRange>>,
    #[serde(default)]
    pub electrical_room_extension: Vec<Option<ElectricalRoomExtension>>,
    #[serde(default)]
    pub electrical_state: Vec<Option<ElectricalState>>,
    #[serde(default)]
    pub electrical_state_ref: Vec<Option<ElectricalStateRef>>,
    #[serde(default)]
    pub area_disruption_gameplay_trigger: Vec<Option<AreaDisruptionGameplayTrigger>>,
    #[serde(default)]
    pub reset_area_disruption_gameplay_trigger: Vec<Option<ResetAreaDisruptionGameplayTrigger>>,
    #[serde(default)]
    pub hearing_disruption_gameplay_trigger: Vec<Option<HearingDisruptionGameplayTrigger>>,
    #[serde(default)]
    pub sdisruption_gameplay_trigger: Vec<Option<SDisruptionGameplayTrigger>>,
    #[serde(default)]
    pub sreset_entity_lifetime_gameplay_trigger: Vec<Option<SResetEntityLifetimeGameplayTrigger>>,
    #[serde(default)]
    pub sattachable_tag_check: Vec<Option<SAttachableTagCheck>>,
    #[serde(default)]
    pub gameplay_trigger_target_type_self: Vec<Option<GameplayTrigger_TargetType_Self>>,
    #[serde(default)]
    pub gameplay_trigger_target_type_root: Vec<Option<GameplayTrigger_TargetType_Root>>,
    #[serde(default)]
    pub gameplay_trigger_target_type_item_port_name: Vec<Option<GameplayTrigger_TargetType_ItemPortName>>,
    #[serde(default)]
    pub gameplay_trigger_target_type_filtered: Vec<Option<GameplayTrigger_TargetType_Filtered>>,
    #[serde(default)]
    pub gameplay_trigger_filter_type_entity_class: Vec<Option<GameplayTrigger_FilterType_EntityClass>>,
    #[serde(default)]
    pub gameplay_trigger_filter_type_entity_tag: Vec<Option<GameplayTrigger_FilterType_EntityTag>>,
    #[serde(default)]
    pub gameplay_trigger_filter_type_item_type: Vec<Option<GameplayTrigger_FilterType_ItemType>>,
    #[serde(default)]
    pub gameplay_trigger_executor_activate_interaction_by_tag: Vec<Option<GameplayTrigger_Executor_ActivateInteraction_ByTag>>,
    #[serde(default)]
    pub gameplay_trigger_executor_set_interaction_state_by_tag: Vec<Option<GameplayTrigger_Executor_SetInteractionState_ByTag>>,
    #[serde(default)]
    pub area_target_state_change: Vec<Option<AreaTargetStateChange>>,
    #[serde(default)]
    pub gameplay_trigger_condition_nand: Vec<Option<GameplayTriggerConditionNAND>>,
    #[serde(default)]
    pub gameplay_trigger_condition_nor: Vec<Option<GameplayTriggerConditionNOR>>,
    #[serde(default)]
    pub gameplay_trigger_condition_xor: Vec<Option<GameplayTriggerConditionXOR>>,
    #[serde(default)]
    pub gameplay_trigger_condition_xnor: Vec<Option<GameplayTriggerConditionXNOR>>,
    #[serde(default)]
    pub gameplay_trigger_condition_check_state_other: Vec<Option<GameplayTriggerConditionCheckStateOther>>,
    #[serde(default)]
    pub gameplay_trigger_condition_target_entity: Vec<Option<GameplayTriggerConditionTargetEntity>>,
    #[serde(default)]
    pub user_variable_check_bool_equal: Vec<Option<UserVariableCheckBoolEqual>>,
    #[serde(default)]
    pub user_variable_check_int_greater: Vec<Option<UserVariableCheckIntGreater>>,
    #[serde(default)]
    pub user_variable_check_float_equal: Vec<Option<UserVariableCheckFloatEqual>>,
    #[serde(default)]
    pub user_variable_check_float_greater: Vec<Option<UserVariableCheckFloatGreater>>,
    #[serde(default)]
    pub user_variable_check_float_less: Vec<Option<UserVariableCheckFloatLess>>,
    #[serde(default)]
    pub user_variable_check_string_equal: Vec<Option<UserVariableCheckStringEqual>>,
    #[serde(default)]
    pub fire_hazard_extinguish_gameplay_trigger: Vec<Option<FireHazardExtinguishGameplayTrigger>>,
    #[serde(default)]
    pub set_sub_geometry_tag_gameplay_trigger: Vec<Option<SetSubGeometryTagGameplayTrigger>>,
    #[serde(default)]
    pub sadd_health_value: Vec<Option<SAddHealthValue>>,
    #[serde(default)]
    pub ssubstract_health_value: Vec<Option<SSubstractHealthValue>>,
    #[serde(default)]
    pub sadd_health_ratio: Vec<Option<SAddHealthRatio>>,
    #[serde(default)]
    pub invulnerability_state: Vec<Option<InvulnerabilityState>>,
    #[serde(default)]
    pub sself_hint_activator: Vec<Option<SSelfHintActivator>>,
    #[serde(default)]
    pub sinteractor_hint_activator: Vec<Option<SInteractorHintActivator>>,
    #[serde(default)]
    pub sactivate_item_expiration_gameplay_trigger: Vec<Option<SActivateItemExpirationGameplayTrigger>>,
    #[serde(default)]
    pub gameplay_trigger_physics_set_parameter_kinematic_state: Vec<Option<GameplayTrigger_Physics_SetParameter_KinematicState>>,
    #[serde(default)]
    pub sset_screen_shake_area_enabled_state_gameplay_trigger: Vec<Option<SSetScreenShakeAreaEnabledStateGameplayTrigger>>,
    #[serde(default)]
    pub triggered_warning: Vec<Option<TriggeredWarning>>,
    #[serde(default)]
    pub triggered_warning_clear: Vec<Option<TriggeredWarningClear>>,
    #[serde(default)]
    pub suser_variable_set_string_value: Vec<Option<SUserVariableSetStringValue>>,
    #[serde(default)]
    pub user_variable_inverse_bool_value: Vec<Option<UserVariableInverseBoolValue>>,
    #[serde(default)]
    pub user_variable_multiply_int_value: Vec<Option<UserVariableMultiplyIntValue>>,
    #[serde(default)]
    pub user_variable_divide_int_value: Vec<Option<UserVariableDivideIntValue>>,
    #[serde(default)]
    pub user_variable_modulus_int_value: Vec<Option<UserVariableModulusIntValue>>,
    #[serde(default)]
    pub user_variable_subtract_float_value: Vec<Option<UserVariableSubtractFloatValue>>,
    #[serde(default)]
    pub user_variable_multiply_float_value: Vec<Option<UserVariableMultiplyFloatValue>>,
    #[serde(default)]
    pub user_variable_divide_float_value: Vec<Option<UserVariableDivideFloatValue>>,
    #[serde(default)]
    pub svending_machine_spawn_gameplay_trigger: Vec<Option<SVendingMachineSpawnGameplayTrigger>>,
    #[serde(default)]
    pub stransfer_credit_gameplay_trigger: Vec<Option<STransferCreditGameplayTrigger>>,
    #[serde(default)]
    pub gameplay_trigger_interpolation_type_user_variable_float: Vec<Option<GameplayTrigger_InterpolationType_UserVariableFloat>>,
    #[serde(default)]
    pub sclegacy_weapon_component_params: Vec<Option<SCLegacyWeaponComponentParams>>,
    #[serde(default)]
    pub scitem_boat_controller_params: Vec<Option<SCItemBoatControllerParams>>,
    #[serde(default)]
    pub scitem_foley_params: Vec<Option<SCItemFoleyParams>>,
    #[serde(default)]
    pub scitem_cargo_grid_params: Vec<Option<SCItemCargoGridParams>>,
    #[serde(default)]
    pub cargo_grid_spawned_component_params: Vec<Option<CargoGridSpawnedComponentParams>>,
    #[serde(default)]
    pub scitem_cooler_params: Vec<Option<SCItemCoolerParams>>,
    #[serde(default)]
    pub purchasable_display_clothing: Vec<Option<PurchasableDisplayClothing>>,
    #[serde(default)]
    pub scitem_power_plant_params: Vec<Option<SCItemPowerPlantParams>>,
    #[serde(default)]
    pub scitem_proximity_sensor_sphere_params: Vec<Option<SCItemProximitySensorSphereParams>>,
    #[serde(default)]
    pub spower_source_component_params: Vec<Option<SPowerSourceComponentParams>>,
    #[serde(default)]
    pub scitem_explosive_params: Vec<Option<SCItemExplosiveParams>>,
    #[serde(default)]
    pub blinking_light_params: Vec<Option<BlinkingLightParams>>,
    #[serde(default)]
    pub consumable_effect_clear_buff_effect: Vec<Option<ConsumableEffectClearBuffEffect>>,
    #[serde(default)]
    pub scitem_docking_controller_params: Vec<Option<SCItemDockingControllerParams>>,
    #[serde(default)]
    pub scmelee_weapon_params: Vec<Option<SCMeleeWeaponParams>>,
    #[serde(default)]
    pub animation_component_params: Vec<Option<AnimationComponentParams>>,
    #[serde(default)]
    pub radar_jammer_component_params: Vec<Option<RadarJammerComponentParams>>,
    #[serde(default)]
    pub scitem_door_five_way_procedural_params: Vec<Option<SCItemDoorFiveWayProceduralParams>>,
    #[serde(default)]
    pub sdoor_collision_reaction_params: Vec<Option<SDoorCollisionReactionParams>>,
    #[serde(default)]
    pub scitem_door_status_lights_params: Vec<Option<SCItemDoorStatusLightsParams>>,
    #[serde(default)]
    pub accessibility_exterior_zone_params: Vec<Option<AccessibilityExteriorZoneParams>>,
    #[serde(default)]
    pub selevator_bbox_collision_params: Vec<Option<SElevatorBBoxCollisionParams>>,
    #[serde(default)]
    pub selevator_custom_collision_params: Vec<Option<SElevatorCustomCollisionParams>>,
    #[serde(default)]
    pub scitem_landing_gear_params: Vec<Option<SCItemLandingGearParams>>,
    #[serde(default)]
    pub sexplosive_ordnance_params: Vec<Option<SExplosiveOrdnanceParams>>,
    #[serde(default)]
    pub ssend_seat_ready_pose_locked_event: Vec<Option<SSendSeatReadyPoseLockedEvent>>,
    #[serde(default)]
    pub shelmet_state_animation_params: Vec<Option<SHelmetStateAnimationParams>>,
    #[serde(default)]
    pub shelmet_state_fade_params: Vec<Option<SHelmetStateFadeParams>>,
    #[serde(default)]
    pub shelmet_state_lights_params: Vec<Option<SHelmetStateLightsParams>>,
    #[serde(default)]
    pub scitem_weapon_regen_pool_component_params: Vec<Option<SCItemWeaponRegenPoolComponentParams>>,
    #[serde(default)]
    pub weapon_aiaiming_method_high_low_arc: Vec<Option<WeaponAIAimingMethodHighLowArc>>,
    #[serde(default)]
    pub weapon_aiaiming_method_preferred_height_arc: Vec<Option<WeaponAIAimingMethodPreferredHeightArc>>,
    #[serde(default)]
    pub sthrowable_launcher: Vec<Option<SThrowableLauncher>>,
    #[serde(default)]
    pub sweapon_condition_any: Vec<Option<SWeaponConditionAny>>,
    #[serde(default)]
    pub sweapon_condition_not: Vec<Option<SWeaponConditionNot>>,
    #[serde(default)]
    pub sweapon_condition_has_attachment_with_tags: Vec<Option<SWeaponConditionHasAttachmentWithTags>>,
    #[serde(default)]
    pub sweapon_action_mark_grapple_params: Vec<Option<SWeaponActionMarkGrappleParams>>,
    #[serde(default)]
    pub sweapon_action_toggle_params: Vec<Option<SWeaponActionToggleParams>>,
    #[serde(default)]
    pub sweapon_action_highlight_cuttables_params: Vec<Option<SWeaponActionHighlightCuttablesParams>>,
    #[serde(default)]
    pub sweapon_action_melee_params: Vec<Option<SWeaponActionMeleeParams>>,
    #[serde(default)]
    pub sweapon_action_grapple_params: Vec<Option<SWeaponActionGrappleParams>>,
    #[serde(default)]
    pub iweapon_component_params: Vec<Option<IWeaponComponentParams>>,
    #[serde(default)]
    pub item_recovery_condition_entity_tag: Vec<Option<ItemRecoveryCondition_EntityTag>>,
    #[serde(default)]
    pub personal_thought_action: Vec<Option<PersonalThoughtAction>>,
    #[serde(default)]
    pub personal_thought_category: Vec<Option<PersonalThoughtCategory>>,
    #[serde(default)]
    pub personal_thought_player_item: Vec<Option<PersonalThoughtPlayerItem>>,
    #[serde(default)]
    pub quick_access_wheel_auto_operator_mode_def: Vec<Option<QuickAccessWheelAutoOperatorModeDef>>,
    #[serde(default)]
    pub quick_access_wheel_auto_wingman_commands_def: Vec<Option<QuickAccessWheelAutoWingmanCommandsDef>>,
    #[serde(default)]
    pub smodifier_signatures_entry: Vec<Option<SModifierSignaturesEntry>>,
    #[serde(default)]
    pub sscsignature_emission_modifier: Vec<Option<SSCSignatureEmissionModifier>>,
    #[serde(default)]
    pub sscsignature_system_audio_multiplier: Vec<Option<SSCSignatureSystemAudioMultiplier>>,
    #[serde(default)]
    pub sscsignature_system_audio_offset: Vec<Option<SSCSignatureSystemAudioOffset>>,
    #[serde(default)]
    pub sscsignature_system_audio_trigger_tag_rule: Vec<Option<SSCSignatureSystemAudioTriggerTagRule>>,
    #[serde(default)]
    pub sscsignature_system_audio_trigger_rule: Vec<Option<SSCSignatureSystemAudioTriggerRule>>,
    #[serde(default)]
    pub scannable_component_params: Vec<Option<ScannableComponentParams>>,
    #[serde(default)]
    pub completion_type_delivered_scu: Vec<Option<CompletionType_DeliveredSCU>>,
    #[serde(default)]
    pub screen_effects_param_value_bool: Vec<Option<ScreenEffects_ParamValue_Bool>>,
    #[serde(default)]
    pub screen_effects_param_value_int: Vec<Option<ScreenEffects_ParamValue_Int>>,
    #[serde(default)]
    pub screen_effects_param_value_vec4_base: Vec<Option<ScreenEffects_ParamValue_Vec4Base>>,
    #[serde(default)]
    pub screen_effects_param_value_vec4: Vec<Option<ScreenEffects_ParamValue_Vec4>>,
    #[serde(default)]
    pub screen_effects_param_value_color: Vec<Option<ScreenEffects_ParamValue_Color>>,
    #[serde(default)]
    pub screen_effects_param_value_texture: Vec<Option<ScreenEffects_ParamValue_Texture>>,
    #[serde(default)]
    pub security_clearance_token_data_and: Vec<Option<SecurityClearanceTokenData_AND>>,
    #[serde(default)]
    pub security_clearance_token_data_or: Vec<Option<SecurityClearanceTokenData_OR>>,
    #[serde(default)]
    pub security_clearance_token_data_not: Vec<Option<SecurityClearanceTokenData_NOT>>,
    #[serde(default)]
    pub security_clearance_token_data_other_token: Vec<Option<SecurityClearanceTokenData_OtherToken>>,
    #[serde(default)]
    pub security_clearance_token_data_unarmed_def: Vec<Option<SecurityClearanceTokenData_UnarmedDef>>,
    #[serde(default)]
    pub security_clearance_outfit_requirement_tag_requirement_def: Vec<Option<SecurityClearance_OutfitRequirement_TagRequirementDef>>,
    #[serde(default)]
    pub security_clearance_outfit_requirement_required_piece_def: Vec<Option<SecurityClearance_OutfitRequirement_RequiredPieceDef>>,
    #[serde(default)]
    pub security_clearance_outfit_requirement_or_def: Vec<Option<SecurityClearance_OutfitRequirement_OrDef>>,
    #[serde(default)]
    pub security_clearance_outfit_requirement_and_def: Vec<Option<SecurityClearance_OutfitRequirement_AndDef>>,
    #[serde(default)]
    pub security_clearance_outfit_requirement_not_def: Vec<Option<SecurityClearance_OutfitRequirement_NotDef>>,
    #[serde(default)]
    pub security_clearance_token_data_outfit_requirement_def: Vec<Option<SecurityClearanceTokenData_OutfitRequirementDef>>,
    #[serde(default)]
    pub security_clearance_token_data_weapon_drawn: Vec<Option<SecurityClearanceTokenData_WeaponDrawn>>,
    #[serde(default)]
    pub security_login_input: Vec<Option<SecurityLoginInput>>,
    #[serde(default)]
    pub security_token_notification_record_reference: Vec<Option<SecurityTokenNotificationRecordReference>>,
    #[serde(default)]
    pub security_network_variable_value_int: Vec<Option<SecurityNetworkVariableValue_Int>>,
    #[serde(default)]
    pub tqsinput_string_value: Vec<Option<TQSInputStringValue>>,
    #[serde(default)]
    pub tqsweight_input_int_value: Vec<Option<TQSWeightInputIntValue>>,
    #[serde(default)]
    pub tqsweight_dynamic_variable_value: Vec<Option<TQSWeightDynamicVariableValue>>,
    #[serde(default)]
    pub tile_params: Vec<Option<TileParams>>,
    #[serde(default)]
    pub time_value_long_seconds: Vec<Option<TimeValue_LongSeconds>>,
    #[serde(default)]
    pub transport_event_item_spawner_params: Vec<Option<TransportEventItemSpawnerParams>>,
    #[serde(default)]
    pub transport_test_permissions_interface: Vec<Option<TransportTestPermissionsInterface>>,
    #[serde(default)]
    pub transport_iimhangar_interface: Vec<Option<TransportIIMHangarInterface>>,
    #[serde(default)]
    pub transport_blacklist_interface: Vec<Option<TransportBlacklistInterface>>,
    #[serde(default)]
    pub transport_manager_params: Vec<Option<TransportManagerParams>>,
    #[serde(default)]
    pub transport_destination_params: Vec<Option<TransportDestinationParams>>,
    #[serde(default)]
    pub transport_gateway_params: Vec<Option<TransportGatewayParams>>,
    #[serde(default)]
    pub transport_pause_point_params: Vec<Option<TransportPausePointParams>>,
    #[serde(default)]
    pub transport_event_listener_params: Vec<Option<TransportEventListenerParams>>,
    #[serde(default)]
    pub transport_gateway_door_params: Vec<Option<TransportGatewayDoorParams>>,
    #[serde(default)]
    pub transport_gateway_timer_panel_params: Vec<Option<TransportGatewayTimerPanelParams>>,
    #[serde(default)]
    pub transport_gateway_control_panel_params: Vec<Option<TransportGatewayControlPanelParams>>,
    #[serde(default)]
    pub transport_carriage_door_params: Vec<Option<TransportCarriageDoorParams>>,
    #[serde(default)]
    pub transport_carriage_control_panel_params: Vec<Option<TransportCarriageControlPanelParams>>,
    #[serde(default)]
    pub transport_carriage_timer_panel_params: Vec<Option<TransportCarriageTimerPanelParams>>,
    #[serde(default)]
    pub transport_carriage_audio: Vec<Option<TransportCarriageAudio>>,
    #[serde(default)]
    pub transport_carriage_effects: Vec<Option<TransportCarriageEffects>>,
    #[serde(default)]
    pub transport_carriage_params: Vec<Option<TransportCarriageParams>>,
    #[serde(default)]
    pub uigraph_loadout_selector_component: Vec<Option<UIGraph_LoadoutSelectorComponent>>,
    #[serde(default)]
    pub uigraph_contact_widget_component: Vec<Option<UIGraph_ContactWidgetComponent>>,
    #[serde(default)]
    pub uigraph_fast_contact_list_component: Vec<Option<UIGraph_FastContactListComponent>>,
    #[serde(default)]
    pub uigraph_launcher_screen_component: Vec<Option<UIGraph_LauncherScreenComponent>>,
    #[serde(default)]
    pub uigraph_landing_area_services_component: Vec<Option<UIGraph_LandingAreaServicesComponent>>,
    #[serde(default)]
    pub uigraph_sky_line_nav_context_component: Vec<Option<UIGraph_SkyLineNavContextComponent>>,
    #[serde(default)]
    pub eastatic_loadout_dummy: Vec<Option<EAStaticLoadoutDummy>>,
    #[serde(default)]
    pub uigraph_inventory_component: Vec<Option<UIGraph_InventoryComponent>>,
    #[serde(default)]
    pub unit_test_override_defaults_class_a: Vec<Option<UnitTest_OverrideDefaultsClassA>>,
    #[serde(default)]
    pub unit_test_override_defaults_class_b: Vec<Option<UnitTest_OverrideDefaultsClassB>>,
    #[serde(default)]
    pub alignment_slot_area_helper_component_params: Vec<Option<AlignmentSlotAreaHelperComponentParams>>,
    #[serde(default)]
    pub sscooch_overrides: Vec<Option<SScoochOverrides>>,
    #[serde(default)]
    pub sscooch_override: Vec<Option<SScoochOverride>>,
    #[serde(default)]
    pub usable_slotting_reference_loadout_entry: Vec<Option<UsableSlottingReferenceLoadoutEntry>>,
    #[serde(default)]
    pub ssequencer_usable_enable_use_channel_task: Vec<Option<SSequencerUsableEnableUseChannelTask>>,
    #[serde(default)]
    pub ssequencer_usable_disable_use_channel_task: Vec<Option<SSequencerUsableDisableUseChannelTask>>,
    #[serde(default)]
    pub ssequencer_usable_send_event_to_user_task: Vec<Option<SSequencerUsableSendEventToUserTask>>,
    #[serde(default)]
    pub entity_component_rtt_aspect_focus_vehicle_params: Vec<Option<EntityComponentRttAspectFocusVehicleParams>>,
    #[serde(default)]
    pub entity_component_rtt_aspect_own_vehicle_params: Vec<Option<EntityComponentRttAspectOwnVehicleParams>>,
    #[serde(default)]
    pub vehicle_editor_terminal_params: Vec<Option<VehicleEditorTerminalParams>>,
    #[serde(default)]
    pub vehicle_editor_manager_params: Vec<Option<VehicleEditorManagerParams>>,
    #[serde(default)]
    pub distress_comms_signal_component_params: Vec<Option<DistressCommsSignalComponentParams>>,
    #[serde(default)]
    pub voice_chat_rx_params: Vec<Option<VoiceChatRxParams>>,
    #[serde(default)]
    pub weapon: Vec<Option<Weapon>>,
    #[serde(default)]
    pub weapon_procedural_clip_recoil: Vec<Option<WeaponProceduralClipRecoil>>,
    #[serde(default)]
    pub weather_effects_atmosphere_water_depth: Vec<Option<WeatherEffects_Atmosphere_WaterDepth>>,
    #[serde(default)]
    pub planet_spawned_entity_params: Vec<Option<PlanetSpawnedEntityParams>>,
    #[serde(default)]
    pub crater_modifier_component_params: Vec<Option<CraterModifierComponentParams>>,
    #[serde(default)]
    pub mobi_glas_personal_message_ship_chat: Vec<Option<MobiGlasPersonalMessage_ShipChat>>,
    #[serde(default)]
    pub mobi_glas_personal_message_ship_chat_schedule_entry: Vec<Option<MobiGlasPersonalMessage_ShipChatScheduleEntry>>,
    #[serde(default)]
    pub mobi_glas_personal_message_ship_chat_schedule: Vec<Option<MobiGlasPersonalMessage_ShipChatSchedule>>,
    #[serde(default)]
    pub mobi_glas_personal_message_message: Vec<Option<MobiGlasPersonalMessage_Message>>,
    #[serde(default)]
    pub mobi_glas_mission_briefing: Vec<Option<MobiGlasMissionBriefing>>,
    #[serde(default)]
    pub mobi_glas_after_action_report_generation_rules: Vec<Option<MobiGlasAfterActionReportGenerationRules>>,
    #[serde(default)]
    pub after_action_report_reputation_impact: Vec<Option<AfterActionReportReputationImpact>>,
    #[serde(default)]
    pub mobi_glas_after_action_report_rank_rules: Vec<Option<MobiGlasAfterActionReportRankRules>>,
    #[serde(default)]
    pub mobi_glas_mission_manager_summary_update: Vec<Option<MobiGlasMissionManagerSummaryUpdate>>,
    #[serde(default)]
    pub s42_field_manual_content_piece: Vec<Option<S42FieldManualContentPiece>>,
    #[serde(default)]
    pub s42_field_manual: Vec<Option<S42FieldManual>>,
    #[serde(default)]
    pub smobi_glas_app_data_packet: Vec<Option<SMobiGlasAppDataPacket>>,
    #[serde(default)]
    pub smobi_glas_app_data_packet_embedded: Vec<Option<SMobiGlasAppDataPacketEmbedded>>,
    #[serde(default)]
    pub smobi_glas_app_data_packet_referenced: Vec<Option<SMobiGlasAppDataPacketReferenced>>,
    #[serde(default)]
    pub smobi_glas_set_ship_recall_allowed: Vec<Option<SMobiGlasSetShipRecallAllowed>>,
    #[serde(default)]
    pub smobi_glas_set_unique_ship: Vec<Option<SMobiGlasSetUniqueShip>>,
    #[serde(default)]
    pub sship_status_app_params: Vec<Option<SShipStatusAppParams>>,
    #[serde(default)]
    pub s42_ship_status_app_params: Vec<Option<S42ShipStatusAppParams>>,
    #[serde(default)]
    pub s42_ship_status_allowed_ship_params: Vec<Option<S42ShipStatusAllowedShipParams>>,
    #[serde(default)]
    pub sminigame_app_params: Vec<Option<SMinigameAppParams>>,
    #[serde(default)]
    pub s42_player_data_skip_point_rank: Vec<Option<S42PlayerDataSkipPointRank>>,
    #[serde(default)]
    pub ss42_player_data_app_params: Vec<Option<SS42PlayerDataAppParams>>,
}
