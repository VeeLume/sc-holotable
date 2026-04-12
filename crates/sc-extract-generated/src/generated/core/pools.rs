// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `core` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorePools {
    #[serde(default)]
    pub aitargetable_settings: Vec<Option<AITargetableSettings>>,
    #[serde(default)]
    pub aifire_discipline_settings: Vec<Option<AIFireDisciplineSettings>>,
    #[serde(default)]
    pub ability_breath_action: Vec<Option<AbilityBreathAction>>,
    #[serde(default)]
    pub ability_breathing_params: Vec<Option<AbilityBreathingParams>>,
    #[serde(default)]
    pub ability_definition: Vec<Option<AbilityDefinition>>,
    #[serde(default)]
    pub actor_ability_component: Vec<Option<ActorAbilityComponent>>,
    #[serde(default)]
    pub bone_counter_rotate_config: Vec<Option<BoneCounterRotateConfig>>,
    #[serde(default)]
    pub spine_bone: Vec<Option<SpineBone>>,
    #[serde(default)]
    pub duck_pose: Vec<Option<DuckPose>>,
    #[serde(default)]
    pub actor_ducking_params: Vec<Option<ActorDuckingParams>>,
    #[serde(default)]
    pub environment_temperature_params: Vec<Option<EnvironmentTemperatureParams>>,
    #[serde(default)]
    pub temperature_uiparams: Vec<Option<TemperatureUIParams>>,
    #[serde(default)]
    pub actor_frosted_visor_params: Vec<Option<ActorFrostedVisorParams>>,
    #[serde(default)]
    pub actor_environment_component: Vec<Option<ActorEnvironmentComponent>>,
    #[serde(default)]
    pub gforce_params: Vec<Option<GForceParams>>,
    #[serde(default)]
    pub actor_gforce_component: Vec<Option<ActorGForceComponent>>,
    #[serde(default)]
    pub actor_base_stance_movement_modifiers: Vec<Option<ActorBaseStanceMovementModifiers>>,
    #[serde(default)]
    pub actor_base_movement_modifiers: Vec<Option<ActorBaseMovementModifiers>>,
    #[serde(default)]
    pub actor_force_movement_modifier_config: Vec<Option<ActorForceMovementModifierConfig>>,
    #[serde(default)]
    pub actor_external_force_movement_modifiers: Vec<Option<ActorExternalForceMovementModifiers>>,
    #[serde(default)]
    pub actor_environmental_modifier_config: Vec<Option<ActorEnvironmentalModifierConfig>>,
    #[serde(default)]
    pub actor_environmental_movement_modifiers: Vec<Option<ActorEnvironmentalMovementModifiers>>,
    #[serde(default)]
    pub actor_movement_modifiers: Vec<Option<ActorMovementModifiers>>,
    #[serde(default)]
    pub actor_movement_set_transition: Vec<Option<ActorMovementSetTransition>>,
    #[serde(default)]
    pub actor_movement_sets_config: Vec<Option<ActorMovementSetsConfig>>,
    #[serde(default)]
    pub actor_procedural_recoil_setup: Vec<Option<ActorProceduralRecoilSetup>>,
    #[serde(default)]
    pub actor_procedural_recoil_config: Vec<Option<ActorProceduralRecoilConfig>>,
    #[serde(default)]
    pub shands_recoil_curve_noise_modifer: Vec<Option<SHandsRecoilCurveNoiseModifer>>,
    #[serde(default)]
    pub sxyzcurves_with_max_values_modifer: Vec<Option<SXYZCurvesWithMaxValuesModifer>>,
    #[serde(default)]
    pub sactor_procedural_hands_recoil_curve_decay_modifiers_def: Vec<Option<SActorProceduralHandsRecoilCurveDecayModifiersDef>>,
    #[serde(default)]
    pub sactor_procedural_hands_recoil_curve_modifiers_def: Vec<Option<SActorProceduralHandsRecoilCurveModifiersDef>>,
    #[serde(default)]
    pub sactor_procedural_hands_recoil_modifiers: Vec<Option<SActorProceduralHandsRecoilModifiers>>,
    #[serde(default)]
    pub sactor_procedural_aim_recoil_curve_noise_modifiers_def: Vec<Option<SActorProceduralAimRecoilCurveNoiseModifiersDef>>,
    #[serde(default)]
    pub sactor_procedural_aim_recoil_curve_modifiers_def: Vec<Option<SActorProceduralAimRecoilCurveModifiersDef>>,
    #[serde(default)]
    pub sactor_procedural_aim_recoil_modifiers: Vec<Option<SActorProceduralAimRecoilModifiers>>,
    #[serde(default)]
    pub sactor_procedural_body_recoil_modifiers: Vec<Option<SActorProceduralBodyRecoilModifiers>>,
    #[serde(default)]
    pub shead_recoil_noise_modifier: Vec<Option<SHeadRecoilNoiseModifier>>,
    #[serde(default)]
    pub sweapon_procedural_head_recoil_curve_modifier_def: Vec<Option<SWeaponProceduralHeadRecoilCurveModifierDef>>,
    #[serde(default)]
    pub sactor_procedural_head_recoil_modifiers: Vec<Option<SActorProceduralHeadRecoilModifiers>>,
    #[serde(default)]
    pub actor_procedural_recoil_modifiers: Vec<Option<ActorProceduralRecoilModifiers>>,
    #[serde(default)]
    pub actor_sliding_params: Vec<Option<ActorSlidingParams>>,
    #[serde(default)]
    pub actor_speed_camera_effects: Vec<Option<ActorSpeedCameraEffects>>,
    #[serde(default)]
    pub breathable_gas_params: Vec<Option<BreathableGasParams>>,
    #[serde(default)]
    pub breathable_oxygen_params: Vec<Option<BreathableOxygenParams>>,
    #[serde(default)]
    pub stamina_cost: Vec<Option<StaminaCost>>,
    #[serde(default)]
    pub action_stamina_costs: Vec<Option<ActionStaminaCosts>>,
    #[serde(default)]
    pub ability_stamina_states: Vec<Option<AbilityStaminaStates>>,
    #[serde(default)]
    pub stamina_cost_params: Vec<Option<StaminaCostParams>>,
    #[serde(default)]
    pub hud_feedback_params: Vec<Option<HudFeedbackParams>>,
    #[serde(default)]
    pub breathing_helper_params: Vec<Option<BreathingHelperParams>>,
    #[serde(default)]
    pub npc_breathing_params: Vec<Option<NpcBreathingParams>>,
    #[serde(default)]
    pub actor_stamina_component: Vec<Option<ActorStaminaComponent>>,
    #[serde(default)]
    pub actor_state_filter: Vec<Option<ActorStateFilter>>,
    #[serde(default)]
    pub actor_motion_state_filter: Vec<Option<ActorMotionStateFilter>>,
    #[serde(default)]
    pub actor_action_handler_ladder_validation_params: Vec<Option<ActorActionHandler_LadderValidationParams>>,
    #[serde(default)]
    pub actor_action_handler_validation_params: Vec<Option<ActorActionHandler_ValidationParams>>,
    #[serde(default)]
    pub actor_state_data_state_validation_params: Vec<Option<ActorStateData_StateValidationParams>>,
    #[serde(default)]
    pub actor_state_data_animation_validation_params: Vec<Option<ActorStateData_AnimationValidationParams>>,
    #[serde(default)]
    pub actor_state_data_jump_fall_validation_params: Vec<Option<ActorStateData_JumpFallValidationParams>>,
    #[serde(default)]
    pub actor_state_data_ladder_validation_params: Vec<Option<ActorStateData_LadderValidationParams>>,
    #[serde(default)]
    pub actor_state_data_usable_validation_params: Vec<Option<ActorStateData_UsableValidationParams>>,
    #[serde(default)]
    pub actor_state_data_validation_params: Vec<Option<ActorStateData_ValidationParams>>,
    #[serde(default)]
    pub actor_state_validation: Vec<Option<ActorStateValidation>>,
    #[serde(default)]
    pub status_effect_buff_macro: Vec<Option<StatusEffectBuffMacro>>,
    #[serde(default)]
    pub speed_threshold: Vec<Option<SpeedThreshold>>,
    #[serde(default)]
    pub status_effect_value: Vec<Option<StatusEffectValue>>,
    #[serde(default)]
    pub status_effect_trigger: Vec<Option<StatusEffectTrigger>>,
    #[serde(default)]
    pub sstatus_trigger_threshold_level_modifier: Vec<Option<SStatusTriggerThresholdLevelModifier>>,
    #[serde(default)]
    pub sstatus_fortitude_level_modifier: Vec<Option<SStatusFortitudeLevelModifier>>,
    #[serde(default)]
    pub status_trigger_base: Vec<Option<StatusTriggerBase>>,
    #[serde(default)]
    pub actor_status_data: Vec<Option<ActorStatusData>>,
    #[serde(default)]
    pub status_effect_setup_base: Vec<Option<StatusEffectSetupBase>>,
    #[serde(default)]
    pub status_effect_setup: Vec<Option<StatusEffectSetup>>,
    #[serde(default)]
    pub status_masked_retrigger_setup_base: Vec<Option<StatusMaskedRetriggerSetupBase>>,
    #[serde(default)]
    pub status_masked_retrigger_setup: Vec<Option<StatusMaskedRetriggerSetup>>,
    #[serde(default)]
    pub status_masked_retrigger_preset: Vec<Option<StatusMaskedRetriggerPreset>>,
    #[serde(default)]
    pub linked_stat_pass_value_base: Vec<Option<LinkedStatPassValueBase>>,
    #[serde(default)]
    pub linked_stat_rule_base: Vec<Option<LinkedStatRuleBase>>,
    #[serde(default)]
    pub linked_stat_setup: Vec<Option<LinkedStatSetup>>,
    #[serde(default)]
    pub linked_stat_setup_preset: Vec<Option<LinkedStatSetupPreset>>,
    #[serde(default)]
    pub linked_stat_base: Vec<Option<LinkedStatBase>>,
    #[serde(default)]
    pub actor_stat_data: Vec<Option<ActorStatData>>,
    #[serde(default)]
    pub actor_status_preset: Vec<Option<ActorStatusPreset>>,
    #[serde(default)]
    pub actor_status_uiwarning_entry: Vec<Option<ActorStatusUIWarningEntry>>,
    #[serde(default)]
    pub actor_status_uiwarning: Vec<Option<ActorStatusUIWarning>>,
    #[serde(default)]
    pub actor_status_incapacitated_uidata: Vec<Option<ActorStatusIncapacitatedUIData>>,
    #[serde(default)]
    pub actor_status_uidata: Vec<Option<ActorStatusUIData>>,
    #[serde(default)]
    pub health_icon_data: Vec<Option<HealthIconData>>,
    #[serde(default)]
    pub health_icon_status_effect: Vec<Option<HealthIconStatusEffect>>,
    #[serde(default)]
    pub status_cost: Vec<Option<StatusCost>>,
    #[serde(default)]
    pub action_status_costs: Vec<Option<ActionStatusCosts>>,
    #[serde(default)]
    pub ability_status_costs: Vec<Option<AbilityStatusCosts>>,
    #[serde(default)]
    pub actor_status_localisation: Vec<Option<ActorStatusLocalisation>>,
    #[serde(default)]
    pub consumable_params: Vec<Option<ConsumableParams>>,
    #[serde(default)]
    pub status_buff_type_base: Vec<Option<StatusBuffTypeBase>>,
    #[serde(default)]
    pub actor_status_buff: Vec<Option<ActorStatusBuff>>,
    #[serde(default)]
    pub buff_duration_base: Vec<Option<BuffDurationBase>>,
    #[serde(default)]
    pub buff_value_override: Vec<Option<BuffValueOverride>>,
    #[serde(default)]
    pub actor_status_add_buff: Vec<Option<ActorStatusAddBuff>>,
    #[serde(default)]
    pub status_effect_ability_lock: Vec<Option<StatusEffectAbilityLock>>,
    #[serde(default)]
    pub actor_status_stat_modifier: Vec<Option<ActorStatusStatModifier>>,
    #[serde(default)]
    pub basic_status_effect_application_type: Vec<Option<BasicStatusEffectApplicationType>>,
    #[serde(default)]
    pub actor_status_effect: Vec<Option<ActorStatusEffect>>,
    #[serde(default)]
    pub arms_lock_single_ability: Vec<Option<ArmsLockSingleAbility>>,
    #[serde(default)]
    pub arms_lock_config: Vec<Option<ArmsLockConfig>>,
    #[serde(default)]
    pub drifting_consciousness_config: Vec<Option<DriftingConsciousnessConfig>>,
    #[serde(default)]
    pub downed_config: Vec<Option<DownedConfig>>,
    #[serde(default)]
    pub drifting_drunk_bdleffects: Vec<Option<DriftingDrunkBDLEffects>>,
    #[serde(default)]
    pub drifting_drunk_config: Vec<Option<DriftingDrunkConfig>>,
    #[serde(default)]
    pub signature_params: Vec<Option<SignatureParams>>,
    #[serde(default)]
    pub resistance_weight_params: Vec<Option<ResistanceWeightParams>>,
    #[serde(default)]
    pub revival_fade_in_params: Vec<Option<RevivalFadeInParams>>,
    #[serde(default)]
    pub actor_somatic_shake_config: Vec<Option<ActorSomaticShakeConfig>>,
    #[serde(default)]
    pub actor_shudder_config: Vec<Option<ActorShudderConfig>>,
    #[serde(default)]
    pub actor_somatic_shake_params: Vec<Option<ActorSomaticShakeParams>>,
    #[serde(default)]
    pub actor_somatic_shaking_params: Vec<Option<ActorSomaticShakingParams>>,
    #[serde(default)]
    pub toxic_gas_def: Vec<Option<ToxicGasDef>>,
    #[serde(default)]
    pub actor_toxic_gas_params: Vec<Option<ActorToxicGasParams>>,
    #[serde(default)]
    pub status_sweating_params: Vec<Option<StatusSweatingParams>>,
    #[serde(default)]
    pub orifice_blood_params: Vec<Option<OrificeBloodParams>>,
    #[serde(default)]
    pub status_blood_params: Vec<Option<StatusBloodParams>>,
    #[serde(default)]
    pub hygiene_params: Vec<Option<HygieneParams>>,
    #[serde(default)]
    pub actor_status_global_params: Vec<Option<ActorStatusGlobalParams>>,
    #[serde(default)]
    pub actor_status_component: Vec<Option<ActorStatusComponent>>,
    #[serde(default)]
    pub breath_volume_params: Vec<Option<BreathVolumeParams>>,
    #[serde(default)]
    pub building_blocks_node: Vec<Option<BuildingBlocks_Node>>,
    #[serde(default)]
    pub building_blocks_layout_policy_base: Vec<Option<BuildingBlocks_LayoutPolicyBase>>,
    #[serde(default)]
    pub building_blocks_layout_policy_item_base: Vec<Option<BuildingBlocks_LayoutPolicyItemBase>>,
    #[serde(default)]
    pub building_blocks_scroll_policy_base: Vec<Option<BuildingBlocks_ScrollPolicyBase>>,
    #[serde(default)]
    pub building_blocks_drop_target_policy_base: Vec<Option<BuildingBlocks_DropTargetPolicyBase>>,
    #[serde(default)]
    pub building_blocks_draggable_policy_base: Vec<Option<BuildingBlocks_DraggablePolicyBase>>,
    #[serde(default)]
    pub building_blocks_bindings_variable_input: Vec<Option<BuildingBlocks_BindingsVariableInput>>,
    #[serde(default)]
    pub building_blocks_fixed_or_relative_value: Vec<Option<BuildingBlocks_FixedOrRelativeValue>>,
    #[serde(default)]
    pub building_blocks_trbl: Vec<Option<BuildingBlocks_TRBL>>,
    #[serde(default)]
    pub building_blocks_size: Vec<Option<BuildingBlocks_Size>>,
    #[serde(default)]
    pub building_blocks_border_side: Vec<Option<BuildingBlocks_BorderSide>>,
    #[serde(default)]
    pub building_blocks_border_radius_corner: Vec<Option<BuildingBlocks_BorderRadiusCorner>>,
    #[serde(default)]
    pub building_blocks_border: Vec<Option<BuildingBlocks_Border>>,
    #[serde(default)]
    pub building_blocks_background: Vec<Option<BuildingBlocks_Background>>,
    #[serde(default)]
    pub building_blocks_layout_item_common: Vec<Option<BuildingBlocks_LayoutItemCommon>>,
    #[serde(default)]
    pub building_blocks_segmented_fill: Vec<Option<BuildingBlocks_SegmentedFill>>,
    #[serde(default)]
    pub building_blocks_svg_fill: Vec<Option<BuildingBlocks_SvgFill>>,
    #[serde(default)]
    pub building_blocks_overflow: Vec<Option<BuildingBlocks_Overflow>>,
    #[serde(default)]
    pub building_blocks_radial_transform: Vec<Option<BuildingBlocks_RadialTransform>>,
    #[serde(default)]
    pub building_blocks_radial_transform_child: Vec<Option<BuildingBlocks_RadialTransformChild>>,
    #[serde(default)]
    pub building_blocks_animation: Vec<Option<BuildingBlocks_Animation>>,
    #[serde(default)]
    pub building_blocks_preview_screen_base: Vec<Option<BuildingBlocks_PreviewScreenBase>>,
    #[serde(default)]
    pub building_blocks_preview_scene_entity_root: Vec<Option<BuildingBlocks_PreviewSceneEntityRoot>>,
    #[serde(default)]
    pub building_blocks_preview_scene_rtt_root: Vec<Option<BuildingBlocks_PreviewSceneRttRoot>>,
    #[serde(default)]
    pub building_blocks_preview_scene_augmented_reality_rtt: Vec<Option<BuildingBlocks_PreviewSceneAugmentedRealityRtt>>,
    #[serde(default)]
    pub building_blocks_primitive_visual_state: Vec<Option<BuildingBlocks_PrimitiveVisualState>>,
    #[serde(default)]
    pub building_blocks_primitive_settings: Vec<Option<BuildingBlocks_PrimitiveSettings>>,
    #[serde(default)]
    pub building_blocks_callout_settings: Vec<Option<BuildingBlocks_CalloutSettings>>,
    #[serde(default)]
    pub building_blocks_virtual_cursor_policy: Vec<Option<BuildingBlocks_VirtualCursorPolicy>>,
    #[serde(default)]
    pub building_blocks_widget_base: Vec<Option<BuildingBlocks_WidgetBase>>,
    #[serde(default)]
    pub building_blocks_field_modifier_base: Vec<Option<BuildingBlocks_FieldModifierBase>>,
    #[serde(default)]
    pub building_blocks_renderer_policy_base: Vec<Option<BuildingBlocks_RendererPolicyBase>>,
    #[serde(default)]
    pub building_blocks_timing_function_base: Vec<Option<BuildingBlocks_TimingFunctionBase>>,
    #[serde(default)]
    pub building_blocks_field_transition_base: Vec<Option<BuildingBlocks_FieldTransitionBase>>,
    #[serde(default)]
    pub building_blocks_timeline_type_base: Vec<Option<BuildingBlocks_TimelineTypeBase>>,
    #[serde(default)]
    pub building_blocks_canvas: Vec<Option<BuildingBlocks_Canvas>>,
    #[serde(default)]
    pub building_blocks_trigger_base: Vec<Option<BuildingBlocks_TriggerBase>>,
    #[serde(default)]
    pub building_blocks_interactions: Vec<Option<BuildingBlocks_Interactions>>,
    #[serde(default)]
    pub building_blocks_shape_base: Vec<Option<BuildingBlocks_ShapeBase>>,
    #[serde(default)]
    pub building_blocks_two_variable_picker: Vec<Option<BuildingBlocks_TwoVariablePicker>>,
    #[serde(default)]
    pub building_blocks_tooltip_policy: Vec<Option<BuildingBlocks_TooltipPolicy>>,
    #[serde(default)]
    pub building_blocks_context_menu_item: Vec<Option<BuildingBlocks_ContextMenuItem>>,
    #[serde(default)]
    pub building_blocks_context_menu_policy: Vec<Option<BuildingBlocks_ContextMenuPolicy>>,
    #[serde(default)]
    pub building_blocks_range: Vec<Option<BuildingBlocks_Range>>,
    #[serde(default)]
    pub building_blocks_grab_controls_policy: Vec<Option<BuildingBlocks_GrabControlsPolicy>>,
    #[serde(default)]
    pub building_blocks_default_styles: Vec<Option<BuildingBlocks_DefaultStyles>>,
    #[serde(default)]
    pub building_blocks_brand_styles: Vec<Option<BuildingBlocks_BrandStyles>>,
    #[serde(default)]
    pub building_blocks_style_entry: Vec<Option<BuildingBlocks_StyleEntry>>,
    #[serde(default)]
    pub building_blocks_style_condition_list: Vec<Option<BuildingBlocks_StyleConditionList>>,
    #[serde(default)]
    pub building_blocks_style_selector_condition_base: Vec<Option<BuildingBlocks_StyleSelectorConditionBase>>,
    #[serde(default)]
    pub building_blocks_static_variable_base: Vec<Option<BuildingBlocks_StaticVariableBase>>,
    #[serde(default)]
    pub building_blocks_color_base: Vec<Option<BuildingBlocks_ColorBase>>,
    #[serde(default)]
    pub building_blocks_external_color_reference: Vec<Option<BuildingBlocks_ExternalColorReference>>,
    #[serde(default)]
    pub building_blocks_aspect_ratio_option: Vec<Option<BuildingBlocks_AspectRatioOption>>,
    #[serde(default)]
    pub building_blocks_aspect_ratio_library: Vec<Option<BuildingBlocks_AspectRatioLibrary>>,
    #[serde(default)]
    pub actor_fovview_params: Vec<Option<ActorFOVViewParams>>,
    #[serde(default)]
    pub throw_params: Vec<Option<ThrowParams>>,
    #[serde(default)]
    pub item_carry_params: Vec<Option<ItemCarryParams>>,
    #[serde(default)]
    pub sactor_carry_config_tag_switch: Vec<Option<SActorCarryConfigTagSwitch>>,
    #[serde(default)]
    pub carry_config: Vec<Option<CarryConfig>>,
    #[serde(default)]
    pub scharacter_serialization_texture: Vec<Option<SCharacterSerializationTexture>>,
    #[serde(default)]
    pub scharacter_serialization_materials_settings_preset: Vec<Option<SCharacterSerializationMaterialsSettingsPreset>>,
    #[serde(default)]
    pub character_serialization_settings_preset: Vec<Option<CharacterSerializationSettingsPreset>>,
    #[serde(default)]
    pub character_random_name_params: Vec<Option<CharacterRandomNameParams>>,
    #[serde(default)]
    pub communication_channel_name: Vec<Option<CommunicationChannelName>>,
    #[serde(default)]
    pub entity_audio_controller_rtpc_subscriber_list_def: Vec<Option<EntityAudioControllerRtpcSubscriberListDef>>,
    #[serde(default)]
    pub ladder_animation_offset: Vec<Option<LadderAnimationOffset>>,
    #[serde(default)]
    pub ladder_animation_params: Vec<Option<LadderAnimationParams>>,
    #[serde(default)]
    pub ladder_movement_params: Vec<Option<LadderMovementParams>>,
    #[serde(default)]
    pub ladder_jump_params: Vec<Option<LadderJumpParams>>,
    #[serde(default)]
    pub ladder_look_around_params: Vec<Option<LadderLookAroundParams>>,
    #[serde(default)]
    pub ladder_config: Vec<Option<LadderConfig>>,
    #[serde(default)]
    pub mining_audio_params: Vec<Option<MiningAudioParams>>,
    #[serde(default)]
    pub conversation: Vec<Option<Conversation>>,
    #[serde(default)]
    pub conversation_bank: Vec<Option<ConversationBank>>,
    #[serde(default)]
    pub conversation_node_base: Vec<Option<ConversationNode_Base>>,
    #[serde(default)]
    pub rgb8: Vec<Option<RGB8>>,
    #[serde(default)]
    pub srgba8: Vec<Option<SRGBA8>>,
    #[serde(default)]
    pub srgb8: Vec<Option<SRGB8>>,
    #[serde(default)]
    pub rgba: Vec<Option<RGBA>>,
    #[serde(default)]
    pub rgb: Vec<Option<RGB>>,
    #[serde(default)]
    pub vec2: Vec<Option<Vec2>>,
    #[serde(default)]
    pub vec3: Vec<Option<Vec3>>,
    #[serde(default)]
    pub ang3: Vec<Option<Ang3>>,
    #[serde(default)]
    pub deg3: Vec<Option<Deg3>>,
    #[serde(default)]
    pub bezier_curve: Vec<Option<BezierCurve>>,
    #[serde(default)]
    pub sbezier_curve_record: Vec<Option<SBezierCurveRecord>>,
    #[serde(default)]
    pub quat_t: Vec<Option<QuatT>>,
    #[serde(default)]
    pub range: Vec<Option<Range>>,
    #[serde(default)]
    pub damage_base: Vec<Option<DamageBase>>,
    #[serde(default)]
    pub damage_info: Vec<Option<DamageInfo>>,
    #[serde(default)]
    pub explosion_flashbang_params: Vec<Option<ExplosionFlashbangParams>>,
    #[serde(default)]
    pub material_effect_entry: Vec<Option<MaterialEffectEntry>>,
    #[serde(default)]
    pub shockwave_params: Vec<Option<ShockwaveParams>>,
    #[serde(default)]
    pub explosive_fragment_params: Vec<Option<ExplosiveFragmentParams>>,
    #[serde(default)]
    pub explosion_params: Vec<Option<ExplosionParams>>,
    #[serde(default)]
    pub damage_map_channels: Vec<Option<DamageMapChannels>>,
    #[serde(default)]
    pub input_action: Vec<Option<InputAction>>,
    #[serde(default)]
    pub dev_owner_type_base: Vec<Option<DevOwnerType_Base>>,
    #[serde(default)]
    pub dev_owner: Vec<Option<DevOwner>>,
    #[serde(default)]
    pub dialogue_bundle: Vec<Option<DialogueBundle>>,
    #[serde(default)]
    pub dialogue_context: Vec<Option<DialogueContext>>,
    #[serde(default)]
    pub dialogue_context_bank: Vec<Option<DialogueContextBank>>,
    #[serde(default)]
    pub camera_effects_modifiers: Vec<Option<CameraEffectsModifiers>>,
    #[serde(default)]
    pub entity_class_definition: Vec<Option<EntityClassDefinition>>,
    #[serde(default)]
    pub data_forge_component_params: Vec<Option<DataForgeComponentParams>>,
    #[serde(default)]
    pub entity_class_static_data_params: Vec<Option<EntityClassStaticDataParams>>,
    #[serde(default)]
    pub sloadout_inventory_item: Vec<Option<SLoadoutInventoryItem>>,
    #[serde(default)]
    pub sitem_port_loadout_base_params: Vec<Option<SItemPortLoadoutBaseParams>>,
    #[serde(default)]
    pub sloadout_requirement_base: Vec<Option<SLoadoutRequirementBase>>,
    #[serde(default)]
    pub sgrouped_loadouts: Vec<Option<SGroupedLoadouts>>,
    #[serde(default)]
    pub sloadout_assortment: Vec<Option<SLoadoutAssortment>>,
    #[serde(default)]
    pub sentity_traversing_node_id: Vec<Option<SEntityTraversingNodeId>>,
    #[serde(default)]
    pub faction_relationship: Vec<Option<FactionRelationship>>,
    #[serde(default)]
    pub friendly_fire_reaction_override: Vec<Option<FriendlyFireReactionOverride>>,
    #[serde(default)]
    pub faction_legacy: Vec<Option<Faction_LEGACY>>,
    #[serde(default)]
    pub audio_rtpc: Vec<Option<AudioRtpc>>,
    #[serde(default)]
    pub schat_channel_type_base: Vec<Option<SChatChannelTypeBase>>,
    #[serde(default)]
    pub sgeometry_model_tag_base: Vec<Option<SGeometryModelTagBase>>,
    #[serde(default)]
    pub hudsilhouette_params: Vec<Option<HUDSilhouetteParams>>,
    #[serde(default)]
    pub body_part: Vec<Option<BodyPart>>,
    #[serde(default)]
    pub body_joint: Vec<Option<BodyJoint>>,
    #[serde(default)]
    pub body_mapping: Vec<Option<BodyMapping>>,
    #[serde(default)]
    pub body_part_config: Vec<Option<BodyPartConfig>>,
    #[serde(default)]
    pub body_health_config: Vec<Option<BodyHealthConfig>>,
    #[serde(default)]
    pub health_template: Vec<Option<HealthTemplate>>,
    #[serde(default)]
    pub sshared_interaction_params: Vec<Option<SSharedInteractionParams>>,
    #[serde(default)]
    pub sinteraction_params: Vec<Option<SInteractionParams>>,
    #[serde(default)]
    pub shighlight_behavior_node_params: Vec<Option<SHighlightBehaviorNodeParams>>,
    #[serde(default)]
    pub shighlight_behavior_node: Vec<Option<SHighlightBehaviorNode>>,
    #[serde(default)]
    pub sangle_constraint: Vec<Option<SAngleConstraint>>,
    #[serde(default)]
    pub sinteraction_point_primitive_params: Vec<Option<SInteractionPointPrimitiveParams>>,
    #[serde(default)]
    pub sinteraction_point_params: Vec<Option<SInteractionPointParams>>,
    #[serde(default)]
    pub sinteraction_point_modifier: Vec<Option<SInteractionPointModifier>>,
    #[serde(default)]
    pub blocked_text_params: Vec<Option<BlockedTextParams>>,
    #[serde(default)]
    pub blocked_cursor_params: Vec<Option<BlockedCursorParams>>,
    #[serde(default)]
    pub blocked_color_params: Vec<Option<BlockedColorParams>>,
    #[serde(default)]
    pub blocked_hint_params: Vec<Option<BlockedHintParams>>,
    #[serde(default)]
    pub condition_prohibited_items_display_params: Vec<Option<ConditionProhibitedItemsDisplayParams>>,
    #[serde(default)]
    pub condition_display_params: Vec<Option<ConditionDisplayParams>>,
    #[serde(default)]
    pub interaction_condition_params: Vec<Option<InteractionConditionParams>>,
    #[serde(default)]
    pub interaction_condition_list: Vec<Option<InteractionConditionList>>,
    #[serde(default)]
    pub sbase_cargo_unit: Vec<Option<SBaseCargoUnit>>,
    #[serde(default)]
    pub inventory_container_type_base: Vec<Option<InventoryContainerTypeBase>>,
    #[serde(default)]
    pub inventory_container_item_type_filter: Vec<Option<InventoryContainerItemTypeFilter>>,
    #[serde(default)]
    pub inventory_container: Vec<Option<InventoryContainer>>,
    #[serde(default)]
    pub item_port_tags_element: Vec<Option<ItemPortTagsElement>>,
    #[serde(default)]
    pub item_port_tags_dictionary: Vec<Option<ItemPortTagsDictionary>>,
    #[serde(default)]
    pub sitem_port_def_types: Vec<Option<SItemPortDefTypes>>,
    #[serde(default)]
    pub item_resource_type_data: Vec<Option<ItemResourceTypeData>>,
    #[serde(default)]
    pub item_resource_composition_map: Vec<Option<ItemResourceCompositionMap>>,
    #[serde(default)]
    pub item_room_resource_pair: Vec<Option<ItemRoomResourcePair>>,
    #[serde(default)]
    pub item_resource_network_map_trigger_entry: Vec<Option<ItemResourceNetworkMapTriggerEntry>>,
    #[serde(default)]
    pub engineering_state_messages: Vec<Option<EngineeringStateMessages>>,
    #[serde(default)]
    pub item_resource_network_type_uidata: Vec<Option<ItemResourceNetworkTypeUIData>>,
    #[serde(default)]
    pub boxout_stat: Vec<Option<BoxoutStat>>,
    #[serde(default)]
    pub boxout_item_status: Vec<Option<BoxoutItemStatus>>,
    #[serde(default)]
    pub boxout_atmosphere_status: Vec<Option<BoxoutAtmosphereStatus>>,
    #[serde(default)]
    pub item_resource_boxout_uiparams: Vec<Option<ItemResourceBoxoutUIParams>>,
    #[serde(default)]
    pub item_resource_network_uiparams: Vec<Option<ItemResourceNetworkUIParams>>,
    #[serde(default)]
    pub item_resource_network_power_modifier: Vec<Option<ItemResourceNetworkPowerModifier>>,
    #[serde(default)]
    pub item_resource_network_power_params: Vec<Option<ItemResourceNetworkPowerParams>>,
    #[serde(default)]
    pub item_resource_network_default_power_distribution_params: Vec<Option<ItemResourceNetworkDefaultPowerDistributionParams>>,
    #[serde(default)]
    pub item_resource_network_global: Vec<Option<ItemResourceNetworkGlobal>>,
    #[serde(default)]
    pub journal_entry_type: Vec<Option<JournalEntryType>>,
    #[serde(default)]
    pub fragment_info: Vec<Option<FragmentInfo>>,
    #[serde(default)]
    pub duration_tags: Vec<Option<DurationTags>>,
    #[serde(default)]
    pub fragment_required_info: Vec<Option<FragmentRequiredInfo>>,
    #[serde(default)]
    pub landing_selection: Vec<Option<LandingSelection>>,
    #[serde(default)]
    pub landing_animation_setup: Vec<Option<LandingAnimationSetup>>,
    #[serde(default)]
    pub actor_landing_node: Vec<Option<ActorLandingNode>>,
    #[serde(default)]
    pub actor_fall_overlay_node: Vec<Option<ActorFallOverlayNode>>,
    #[serde(default)]
    pub stance_check_config: Vec<Option<StanceCheckConfig>>,
    #[serde(default)]
    pub actor_fall_node: Vec<Option<ActorFallNode>>,
    #[serde(default)]
    pub actor_jump_node: Vec<Option<ActorJumpNode>>,
    #[serde(default)]
    pub actor_jump_fall_land_variant_config_node: Vec<Option<ActorJumpFallLandVariantConfigNode>>,
    #[serde(default)]
    pub jump_fall_land_config: Vec<Option<JumpFallLandConfig>>,
    #[serde(default)]
    pub scextended_localization_level_params: Vec<Option<SCExtendedLocalizationLevelParams>>,
    #[serde(default)]
    pub sloading_screen_information_def: Vec<Option<SLoadingScreenInformationDef>>,
    #[serde(default)]
    pub speed_throttle_configuration: Vec<Option<SpeedThrottleConfiguration>>,
    #[serde(default)]
    pub local_player_speed_throttle_component: Vec<Option<LocalPlayerSpeedThrottleComponent>>,
    #[serde(default)]
    pub interior_map_section_definition: Vec<Option<InteriorMapSectionDefinition>>,
    #[serde(default)]
    pub drug_efficacy: Vec<Option<DrugEfficacy>>,
    #[serde(default)]
    pub drug_efficacy_for_consumable_type: Vec<Option<DrugEfficacyForConsumableType>>,
    #[serde(default)]
    pub med_bed_tier_params: Vec<Option<MedBedTierParams>>,
    #[serde(default)]
    pub drug_efficacy_config_for_item_sub_type_base: Vec<Option<DrugEfficacyConfigForItemSubTypeBase>>,
    #[serde(default)]
    pub drug_efficacy_for_item_type: Vec<Option<DrugEfficacyForItemType>>,
    #[serde(default)]
    pub drug_type_to_apply: Vec<Option<DrugTypeToApply>>,
    #[serde(default)]
    pub med_bed_resource_consumption_params: Vec<Option<MedBedResourceConsumptionParams>>,
    #[serde(default)]
    pub medical_item_tier_config: Vec<Option<MedicalItemTierConfig>>,
    #[serde(default)]
    pub melee_frag_info: Vec<Option<MeleeFragInfo>>,
    #[serde(default)]
    pub attack_category_params: Vec<Option<AttackCategoryParams>>,
    #[serde(default)]
    pub melee_attack_category_info: Vec<Option<MeleeAttackCategoryInfo>>,
    #[serde(default)]
    pub melee_attack_info: Vec<Option<MeleeAttackInfo>>,
    #[serde(default)]
    pub melee_combat_config: Vec<Option<MeleeCombatConfig>>,
    #[serde(default)]
    pub contract_string_param: Vec<Option<ContractStringParam>>,
    #[serde(default)]
    pub contract_bool_param: Vec<Option<ContractBoolParam>>,
    #[serde(default)]
    pub contract_int_param: Vec<Option<ContractIntParam>>,
    #[serde(default)]
    pub contract_availability: Vec<Option<ContractAvailability>>,
    #[serde(default)]
    pub contract_generator_handler_base: Vec<Option<ContractGeneratorHandlerBase>>,
    #[serde(default)]
    pub contract_property_tag_replacement: Vec<Option<ContractPropertyTagReplacement>>,
    #[serde(default)]
    pub contract_param_overrides: Vec<Option<ContractParamOverrides>>,
    #[serde(default)]
    pub contract_generator: Vec<Option<ContractGenerator>>,
    #[serde(default)]
    pub contract_prerequisite_base: Vec<Option<ContractPrerequisiteBase>>,
    #[serde(default)]
    pub partial_contract_reward_rep_adjustment: Vec<Option<PartialContractRewardRepAdjustment>>,
    #[serde(default)]
    pub partial_contract_reward_range: Vec<Option<PartialContractRewardRange>>,
    #[serde(default)]
    pub partial_contract_rewards: Vec<Option<PartialContractRewards>>,
    #[serde(default)]
    pub item_award_base: Vec<Option<ItemAwardBase>>,
    #[serde(default)]
    pub item_award_weightings: Vec<Option<ItemAwardWeightings>>,
    #[serde(default)]
    pub item_award_weightings_record: Vec<Option<ItemAwardWeightingsRecord>>,
    #[serde(default)]
    pub contract_display_info: Vec<Option<ContractDisplayInfo>>,
    #[serde(default)]
    pub contract_class_base: Vec<Option<ContractClassBase>>,
    #[serde(default)]
    pub contract_comms_notification: Vec<Option<ContractCommsNotification>>,
    #[serde(default)]
    pub contract_end_comms_notification: Vec<Option<ContractEndCommsNotification>>,
    #[serde(default)]
    pub contract_template: Vec<Option<ContractTemplate>>,
    #[serde(default)]
    pub mission_flow_condition_base: Vec<Option<MissionFlowConditionBase>>,
    #[serde(default)]
    pub mission_flow_action_base: Vec<Option<MissionFlowActionBase>>,
    #[serde(default)]
    pub mission_flow_trigger: Vec<Option<MissionFlowTrigger>>,
    #[serde(default)]
    pub mission_flow: Vec<Option<MissionFlow>>,
    #[serde(default)]
    pub base_mission_property_value: Vec<Option<BaseMissionPropertyValue>>,
    #[serde(default)]
    pub tag_search_term: Vec<Option<TagSearchTerm>>,
    #[serde(default)]
    pub mission_property: Vec<Option<MissionProperty>>,
    #[serde(default)]
    pub mission_variable_base: Vec<Option<MissionVariableBase>>,
    #[serde(default)]
    pub objective_property_base: Vec<Option<ObjectivePropertyBase>>,
    #[serde(default)]
    pub objective_display_info: Vec<Option<ObjectiveDisplayInfo>>,
    #[serde(default)]
    pub objective_handler_base: Vec<Option<ObjectiveHandlerBase>>,
    #[serde(default)]
    pub objective_reward_contribution_base: Vec<Option<ObjectiveRewardContributionBase>>,
    #[serde(default)]
    pub comms_notification_selector: Vec<Option<CommsNotificationSelector>>,
    #[serde(default)]
    pub objective_token: Vec<Option<ObjectiveToken>>,
    #[serde(default)]
    pub child_mission_phase: Vec<Option<ChildMissionPhase>>,
    #[serde(default)]
    pub base_mission_modifier: Vec<Option<BaseMissionModifier>>,
    #[serde(default)]
    pub abstract_mission_init_param: Vec<Option<AbstractMissionInitParam>>,
    #[serde(default)]
    pub notification_def: Vec<Option<NotificationDef>>,
    #[serde(default)]
    pub comms_notification_stage_camera: Vec<Option<CommsNotificationStageCamera>>,
    #[serde(default)]
    pub comms_notification_stage_actor_mark: Vec<Option<CommsNotificationStageActorMark>>,
    #[serde(default)]
    pub comms_notification_stage_list_item: Vec<Option<CommsNotificationStageListItem>>,
    #[serde(default)]
    pub comms_notification_stage: Vec<Option<CommsNotificationStage>>,
    #[serde(default)]
    pub game_notification_dock_item_params: Vec<Option<GameNotificationDockItemParams>>,
    #[serde(default)]
    pub gpuparticle_audio: Vec<Option<GPUParticleAudio>>,
    #[serde(default)]
    pub player_animated_interaction_hand_params: Vec<Option<PlayerAnimatedInteractionHandParams>>,
    #[serde(default)]
    pub player_animated_interaction: Vec<Option<PlayerAnimatedInteraction>>,
    #[serde(default)]
    pub player_animated_interaction_base: Vec<Option<PlayerAnimatedInteractionBase>>,
    #[serde(default)]
    pub player_animated_interaction_template: Vec<Option<PlayerAnimatedInteractionTemplate>>,
    #[serde(default)]
    pub player_choice_menu_item: Vec<Option<PlayerChoiceMenuItem>>,
    #[serde(default)]
    pub player_choice_menu_option: Vec<Option<PlayerChoiceMenuOption>>,
    #[serde(default)]
    pub player_choice_menu_items: Vec<Option<PlayerChoiceMenuItems>>,
    #[serde(default)]
    pub player_choice_menu: Vec<Option<PlayerChoiceMenu>>,
    #[serde(default)]
    pub player_choice_menu_type: Vec<Option<PlayerChoiceMenuType>>,
    #[serde(default)]
    pub actor_stance_config: Vec<Option<ActorStanceConfig>>,
    #[serde(default)]
    pub jump_fall_land_params: Vec<Option<JumpFallLandParams>>,
    #[serde(default)]
    pub player_dock_context_component_global_params: Vec<Option<PlayerDockContextComponentGlobalParams>>,
    #[serde(default)]
    pub player_group_manager_objects_loc_string_params: Vec<Option<PlayerGroupManagerObjectsLocStringParams>>,
    #[serde(default)]
    pub player_group_manager_loc_string_params: Vec<Option<PlayerGroupManagerLocStringParams>>,
    #[serde(default)]
    pub player_group_manager_notification_params: Vec<Option<PlayerGroupManagerNotificationParams>>,
    #[serde(default)]
    pub player_group_manager_notifications_params: Vec<Option<PlayerGroupManagerNotificationsParams>>,
    #[serde(default)]
    pub player_group_manager_global_params: Vec<Option<PlayerGroupManagerGlobalParams>>,
    #[serde(default)]
    pub player_limitations_profile: Vec<Option<PlayerLimitationsProfile>>,
    #[serde(default)]
    pub player_notification_banner_manager_global_params: Vec<Option<PlayerNotificationBannerManagerGlobalParams>>,
    #[serde(default)]
    pub player_notification_banner_options_params: Vec<Option<PlayerNotificationBannerOptionsParams>>,
    #[serde(default)]
    pub player_notification_banner_params: Vec<Option<PlayerNotificationBannerParams>>,
    #[serde(default)]
    pub player_trade_notification: Vec<Option<PlayerTradeNotification>>,
    #[serde(default)]
    pub player_trade_global_params: Vec<Option<PlayerTradeGlobalParams>>,
    #[serde(default)]
    pub scan_custom_data: Vec<Option<ScanCustomData>>,
    #[serde(default)]
    pub scan_custom_data_info: Vec<Option<ScanCustomDataInfo>>,
    #[serde(default)]
    pub scan_display_setup_params: Vec<Option<ScanDisplaySetupParams>>,
    #[serde(default)]
    pub scan_display_instance_params: Vec<Option<ScanDisplayInstanceParams>>,
    #[serde(default)]
    pub custom_scan_procedure_params: Vec<Option<CustomScanProcedureParams>>,
    #[serde(default)]
    pub sbbdynamic_property_base: Vec<Option<SBBDynamicPropertyBase>>,
    #[serde(default)]
    pub sreputation_context_bbproperty_params: Vec<Option<SReputationContextBBPropertyParams>>,
    #[serde(default)]
    pub reputation_value_setting: Vec<Option<ReputationValueSetting>>,
    #[serde(default)]
    pub reputation_comparison_range: Vec<Option<ReputationComparisonRange>>,
    #[serde(default)]
    pub reputation_value_settings: Vec<Option<ReputationValueSettings>>,
    #[serde(default)]
    pub global_resource_geometry: Vec<Option<GlobalResourceGeometry>>,
    #[serde(default)]
    pub global_resource_particle: Vec<Option<GlobalResourceParticle>>,
    #[serde(default)]
    pub global_resource_material: Vec<Option<GlobalResourceMaterial>>,
    #[serde(default)]
    pub global_resource_audio: Vec<Option<GlobalResourceAudio>>,
    #[serde(default)]
    pub sresource_type_default_cargo_containers: Vec<Option<SResourceTypeDefaultCargoContainers>>,
    #[serde(default)]
    pub actor_view_limits: Vec<Option<ActorViewLimits>>,
    #[serde(default)]
    pub actor_view_limit_preset: Vec<Option<ActorViewLimitPreset>>,
    #[serde(default)]
    pub actor_view_limit_preset_database: Vec<Option<ActorViewLimitPresetDatabase>>,
    #[serde(default)]
    pub scitem_light_amplification: Vec<Option<SCItemLightAmplification>>,
    #[serde(default)]
    pub saimable_sub_targeting_stickiness: Vec<Option<SAimableSubTargetingStickiness>>,
    #[serde(default)]
    pub saimable_pip_auto: Vec<Option<SAimablePipAuto>>,
    #[serde(default)]
    pub saimable_pip_aiming: Vec<Option<SAimablePipAiming>>,
    #[serde(default)]
    pub saimable_target_painting: Vec<Option<SAimableTargetPainting>>,
    #[serde(default)]
    pub saimable_target_auto: Vec<Option<SAimableTargetAuto>>,
    #[serde(default)]
    pub saimable_game_mode_role_params: Vec<Option<SAimableGameModeRoleParams>>,
    #[serde(default)]
    pub saimable_game_mode_params: Vec<Option<SAimableGameModeParams>>,
    #[serde(default)]
    pub scitem_visor_dashboard_config: Vec<Option<SCItemVisorDashboardConfig>>,
    #[serde(default)]
    pub ssalvage_global_structural_vfxparams: Vec<Option<SSalvageGlobalStructuralVFXParams>>,
    #[serde(default)]
    pub ssalvage_global_structural_highlight_params: Vec<Option<SSalvageGlobalStructuralHighlightParams>>,
    #[serde(default)]
    pub vehicle_salvage_global_params: Vec<Option<VehicleSalvageGlobalParams>>,
    #[serde(default)]
    pub scitem_seat_dashboard_screen_pos: Vec<Option<SCItemSeatDashboardScreenPos>>,
    #[serde(default)]
    pub scitem_seat_dashboard_screen_style: Vec<Option<SCItemSeatDashboardScreenStyle>>,
    #[serde(default)]
    pub scitem_seat_dashboard_screen: Vec<Option<SCItemSeatDashboardScreen>>,
    #[serde(default)]
    pub saim_recoil_modifier: Vec<Option<SAimRecoilModifier>>,
    #[serde(default)]
    pub srecoil_modifier: Vec<Option<SRecoilModifier>>,
    #[serde(default)]
    pub sspread_modifier: Vec<Option<SSpreadModifier>>,
    #[serde(default)]
    pub saim_modifier: Vec<Option<SAimModifier>>,
    #[serde(default)]
    pub sregen_consumer_modifier: Vec<Option<SRegenConsumerModifier>>,
    #[serde(default)]
    pub ssalvage_modifier: Vec<Option<SSalvageModifier>>,
    #[serde(default)]
    pub sweapon_stats: Vec<Option<SWeaponStats>>,
    #[serde(default)]
    pub sweapon_modifier_params: Vec<Option<SWeaponModifierParams>>,
    #[serde(default)]
    pub web_rtccomms_call_projector_light_params: Vec<Option<WebRTCCommsCallProjectorLightParams>>,
    #[serde(default)]
    pub player_to_player_comms_call_global_params: Vec<Option<PlayerToPlayerCommsCallGlobalParams>>,
    #[serde(default)]
    pub action_rule_display_params: Vec<Option<ActionRuleDisplayParams>>,
    #[serde(default)]
    pub action_rule_params: Vec<Option<ActionRuleParams>>,
    #[serde(default)]
    pub shop_franchise: Vec<Option<ShopFranchise>>,
    #[serde(default)]
    pub sscradar_contact_properites: Vec<Option<SSCRadarContactProperites>>,
    #[serde(default)]
    pub sscsignature_system_scan_bounds: Vec<Option<SSCSignatureSystemScanBounds>>,
    #[serde(default)]
    pub scsignature_system_room_params: Vec<Option<SCSignatureSystemRoomParams>>,
    #[serde(default)]
    pub scsignature_death_params: Vec<Option<SCSignatureDeathParams>>,
    #[serde(default)]
    pub sscsignature_emission_base_modifier: Vec<Option<SSCSignatureEmissionBaseModifier>>,
    #[serde(default)]
    pub sscsignature_params_base: Vec<Option<SSCSignatureParamsBase>>,
    #[serde(default)]
    pub sscsignature_system_cross_section_params: Vec<Option<SSCSignatureSystemCrossSectionParams>>,
    #[serde(default)]
    pub star_map_amenity_type_entry: Vec<Option<StarMapAmenityTypeEntry>>,
    #[serde(default)]
    pub star_map_amenity_types: Vec<Option<StarMapAmenityTypes>>,
    #[serde(default)]
    pub star_map_object_location_params: Vec<Option<StarMapObjectLocationParams>>,
    #[serde(default)]
    pub star_map_asset_manager_location_params: Vec<Option<StarMapAssetManagerLocationParams>>,
    #[serde(default)]
    pub star_map_object: Vec<Option<StarMapObject>>,
    #[serde(default)]
    pub star_map_asteroid_ring: Vec<Option<StarMapAsteroidRing>>,
    #[serde(default)]
    pub star_map_quantum_travel_data_params: Vec<Option<StarMapQuantumTravelDataParams>>,
    #[serde(default)]
    pub tags_dnfterm: Vec<Option<TagsDNFTerm>>,
    #[serde(default)]
    pub tags_dnf: Vec<Option<TagsDNF>>,
    #[serde(default)]
    pub tag_list: Vec<Option<TagList>>,
    #[serde(default)]
    pub simple_sprite_sheet: Vec<Option<SimpleSpriteSheet>>,
    #[serde(default)]
    pub simple_sprite_slot: Vec<Option<SimpleSpriteSlot>>,
    #[serde(default)]
    pub loadout_dummy_transform_params: Vec<Option<LoadoutDummyTransformParams>>,
    #[serde(default)]
    pub loadout_item_preview_transform_params: Vec<Option<LoadoutItemPreviewTransformParams>>,
    #[serde(default)]
    pub item_port_view_information: Vec<Option<ItemPortViewInformation>>,
    #[serde(default)]
    pub loadout_item_port_view_params: Vec<Option<LoadoutItemPortViewParams>>,
    #[serde(default)]
    pub loadout_item_highlight_params: Vec<Option<LoadoutItemHighlightParams>>,
    #[serde(default)]
    pub loadout_required_attachments_params: Vec<Option<LoadoutRequiredAttachmentsParams>>,
    #[serde(default)]
    pub loadout_candidate_root_params: Vec<Option<LoadoutCandidateRootParams>>,
    #[serde(default)]
    pub loadout_editor_additional_params: Vec<Option<LoadoutEditorAdditionalParams>>,
    #[serde(default)]
    pub loadout_editor_params: Vec<Option<LoadoutEditorParams>>,
    #[serde(default)]
    pub rttsunlight_params: Vec<Option<RTTSunlightParams>>,
    #[serde(default)]
    pub loadout_editor_component_params: Vec<Option<LoadoutEditorComponentParams>>,
    #[serde(default)]
    pub uiworld_display3_dparams: Vec<Option<UIWorldDisplay3DParams>>,
    #[serde(default)]
    pub ui3_ddisplay_input_params: Vec<Option<UI3DDisplayInputParams>>,
    #[serde(default)]
    pub ui3_ddisplay_input: Vec<Option<UI3DDisplayInput>>,
    #[serde(default)]
    pub uiworld_display_holographic_settings: Vec<Option<UIWorldDisplayHolographicSettings>>,
    #[serde(default)]
    pub uiworld_display_rotation_mode_params: Vec<Option<UIWorldDisplayRotationModeParams>>,
    #[serde(default)]
    pub uiworld_display_auto_rotation_params: Vec<Option<UIWorldDisplayAutoRotationParams>>,
    #[serde(default)]
    pub uiworld_display_sound_params: Vec<Option<UIWorldDisplaySoundParams>>,
    #[serde(default)]
    pub uiworld_display_input_sound_params: Vec<Option<UIWorldDisplayInputSoundParams>>,
    #[serde(default)]
    pub uiworld_display_input_sound_rtpc_param: Vec<Option<UIWorldDisplayInputSoundRtpcParam>>,
    #[serde(default)]
    pub test_atomics: Vec<Option<TestAtomics>>,
    #[serde(default)]
    pub test_arrays: Vec<Option<TestArrays>>,
    #[serde(default)]
    pub unit_test_base_test: Vec<Option<UnitTest_BaseTest>>,
    #[serde(default)]
    pub unit_test_inheritance: Vec<Option<UnitTest_Inheritance>>,
    #[serde(default)]
    pub unit_test_override_defaults_test: Vec<Option<UnitTest_OverrideDefaultsTest>>,
    #[serde(default)]
    pub unit_test: Vec<Option<UnitTest>>,
    #[serde(default)]
    pub svehicle_ai_damage_modifiers: Vec<Option<SVehicleAiDamageModifiers>>,
    #[serde(default)]
    pub smobi_glas_app_link: Vec<Option<SMobiGlasAppLink>>,
}
