// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Flat-pool storage for every emitted DCB struct type.
//!
//! Each field is a `Vec<Option<T>>` — nested struct fields store a
//! `Handle<T>` (a `u32` slot index tagged with the target type in
//! `PhantomData`) rather than a `Box<T>`, so deep DCB nesting cannot
//! overflow the stack during materialisation. See
//! `crate::builder::Builder` for the drive loop.
//!
//! `DataPools` is composed of bucket sub-structs (`DataPoolsAb`,
//! `DataPoolsSc1`, …) that mirror the letter-prefix groups used by
//! `generated/types_*.rs`. The split keeps each `serde::Deserialize`
//! derive small enough that LLVM's inliner doesn't blow up the
//! compile time on one monster `visit_map` function.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

use super::*;

/// Bucket pool sub-struct for types in the `ab` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAb {
    #[serde(default)]
    pub ability_breath_action: Vec<Option<AbilityBreathAction>>,
    #[serde(default)]
    pub ability_breathing_params: Vec<Option<AbilityBreathingParams>>,
    #[serde(default)]
    pub ability_definition: Vec<Option<AbilityDefinition>>,
    #[serde(default)]
    pub ability_stamina_states: Vec<Option<AbilityStaminaStates>>,
    #[serde(default)]
    pub ability_status_costs: Vec<Option<AbilityStatusCosts>>,
    #[serde(default)]
    pub abstract_mission_init_param: Vec<Option<AbstractMissionInitParam>>,
}

/// Bucket pool sub-struct for types in the `ac` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAc {
    #[serde(default)]
    pub activity_behavior_request_condition: Vec<Option<ActivityBehaviorRequestCondition>>,
    #[serde(default)]
    pub activity_behavior_request: Vec<Option<ActivityBehaviorRequest>>,
    #[serde(default)]
    pub activity_data_record: Vec<Option<ActivityDataRecord>>,
    #[serde(default)]
    pub activity_data: Vec<Option<ActivityData>>,
    #[serde(default)]
    pub actor_ability_component: Vec<Option<ActorAbilityComponent>>,
    #[serde(default)]
    pub actor_ducking_params: Vec<Option<ActorDuckingParams>>,
    #[serde(default)]
    pub actor_frosted_visor_params: Vec<Option<ActorFrostedVisorParams>>,
    #[serde(default)]
    pub actor_environment_component: Vec<Option<ActorEnvironmentComponent>>,
    #[serde(default)]
    pub actor_gforce_component: Vec<Option<ActorGForceComponent>>,
    #[serde(default)]
    pub actor_gforce_head_bob_fake_velocity_gdata: Vec<Option<ActorGForceHeadBobFakeVelocityGData>>,
    #[serde(default)]
    pub actor_gforce_head_bob_data: Vec<Option<ActorGForceHeadBobData>>,
    #[serde(default)]
    pub actor_gforce_head_bob: Vec<Option<ActorGForceHeadBob>>,
    #[serde(default)]
    pub actor_gforce_camera_effects_data: Vec<Option<ActorGForceCameraEffectsData>>,
    #[serde(default)]
    pub actor_gforce_camera_effects: Vec<Option<ActorGForceCameraEffects>>,
    #[serde(default)]
    pub actor_locomotion_rotate_params: Vec<Option<ActorLocomotionRotateParams>>,
    #[serde(default)]
    pub actor_locomotion_turn_on_spot_params: Vec<Option<ActorLocomotionTurnOnSpotParams>>,
    #[serde(default)]
    pub actor_locomotion_sharp_turn_params: Vec<Option<ActorLocomotionSharpTurnParams>>,
    #[serde(default)]
    pub actor_locomotion_avoidance_params: Vec<Option<ActorLocomotionAvoidanceParams>>,
    #[serde(default)]
    pub actor_locomotion_personality: Vec<Option<ActorLocomotionPersonality>>,
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
    pub actor_procedural_recoil_modifiers: Vec<Option<ActorProceduralRecoilModifiers>>,
    #[serde(default)]
    pub actor_foot_joint_pair_def: Vec<Option<ActorFootJointPairDef>>,
    #[serde(default)]
    pub actor_melee_def: Vec<Option<ActorMeleeDef>>,
    #[serde(default)]
    pub actor_skeleton_config: Vec<Option<ActorSkeletonConfig>>,
    #[serde(default)]
    pub actor_sliding_params: Vec<Option<ActorSlidingParams>>,
    #[serde(default)]
    pub actor_speed_camera_effects: Vec<Option<ActorSpeedCameraEffects>>,
    #[serde(default)]
    pub action_stamina_costs: Vec<Option<ActionStaminaCosts>>,
    #[serde(default)]
    pub actor_stamina_component: Vec<Option<ActorStaminaComponent>>,
    #[serde(default)]
    pub actor_stance_speeds: Vec<Option<ActorStanceSpeeds>>,
    #[serde(default)]
    pub actor_stance_dimensions: Vec<Option<ActorStanceDimensions>>,
    #[serde(default)]
    pub actor_stance_speeds_info: Vec<Option<ActorStanceSpeedsInfo>>,
    #[serde(default)]
    pub actor_stance_dimensions_info: Vec<Option<ActorStanceDimensionsInfo>>,
    #[serde(default)]
    pub actor_state_filter: Vec<Option<ActorStateFilter>>,
    #[serde(default)]
    pub actor_motion_state_filter: Vec<Option<ActorMotionStateFilter>>,
    #[serde(default)]
    pub actor_state_skeleton_filter: Vec<Option<ActorStateSkeletonFilter>>,
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
    pub actor_status_data: Vec<Option<ActorStatusData>>,
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
    pub action_status_costs: Vec<Option<ActionStatusCosts>>,
    #[serde(default)]
    pub actor_status_localisation: Vec<Option<ActorStatusLocalisation>>,
    #[serde(default)]
    pub actor_status_buff: Vec<Option<ActorStatusBuff>>,
    #[serde(default)]
    pub actor_status_add_buff: Vec<Option<ActorStatusAddBuff>>,
    #[serde(default)]
    pub actor_status_stat_modifier: Vec<Option<ActorStatusStatModifier>>,
    #[serde(default)]
    pub actor_status_effect: Vec<Option<ActorStatusEffect>>,
    #[serde(default)]
    pub actor_somatic_shake_config: Vec<Option<ActorSomaticShakeConfig>>,
    #[serde(default)]
    pub actor_shudder_config: Vec<Option<ActorShudderConfig>>,
    #[serde(default)]
    pub actor_somatic_shake_params: Vec<Option<ActorSomaticShakeParams>>,
    #[serde(default)]
    pub actor_somatic_shaking_params: Vec<Option<ActorSomaticShakingParams>>,
    #[serde(default)]
    pub actor_toxic_gas_params: Vec<Option<ActorToxicGasParams>>,
    #[serde(default)]
    pub actor_status_global_params: Vec<Option<ActorStatusGlobalParams>>,
    #[serde(default)]
    pub actor_status_component: Vec<Option<ActorStatusComponent>>,
    #[serde(default)]
    pub actor_zero_gtraversal_params: Vec<Option<ActorZeroGTraversalParams>>,
    #[serde(default)]
    pub actor_breathing_style_startup: Vec<Option<ActorBreathingStyleStartup>>,
    #[serde(default)]
    pub actor_fovview_params: Vec<Option<ActorFOVViewParams>>,
    #[serde(default)]
    pub actor_look_ahead_point: Vec<Option<ActorLookAheadPoint>>,
    #[serde(default)]
    pub actor_look_ahead_roll: Vec<Option<ActorLookAheadRoll>>,
    #[serde(default)]
    pub actor_look_ahead_target_tracking: Vec<Option<ActorLookAheadTargetTracking>>,
    #[serde(default)]
    pub actor_look_ahead_vehicle: Vec<Option<ActorLookAheadVehicle>>,
    #[serde(default)]
    pub actor_default_actions_config: Vec<Option<ActorDefaultActionsConfig>>,
    #[serde(default)]
    pub actor_landing_node: Vec<Option<ActorLandingNode>>,
    #[serde(default)]
    pub actor_fall_overlay_node: Vec<Option<ActorFallOverlayNode>>,
    #[serde(default)]
    pub actor_fall_node: Vec<Option<ActorFallNode>>,
    #[serde(default)]
    pub actor_jump_node: Vec<Option<ActorJumpNode>>,
    #[serde(default)]
    pub actor_jump_fall_land_variant_config_node: Vec<Option<ActorJumpFallLandVariantConfigNode>>,
    #[serde(default)]
    pub actor_targeted_params: Vec<Option<ActorTargetedParams>>,
    #[serde(default)]
    pub actor_stance_config: Vec<Option<ActorStanceConfig>>,
    #[serde(default)]
    pub actor_signature_multiplier_global_params: Vec<Option<ActorSignatureMultiplierGlobalParams>>,
    #[serde(default)]
    pub actor_restrain_per_attacker_config: Vec<Option<ActorRestrainPerAttackerConfig>>,
    #[serde(default)]
    pub actor_restrain_config: Vec<Option<ActorRestrainConfig>>,
    #[serde(default)]
    pub actor_view_limits: Vec<Option<ActorViewLimits>>,
    #[serde(default)]
    pub actor_aim_limits_state_filter: Vec<Option<ActorAimLimitsStateFilter>>,
    #[serde(default)]
    pub actor_look_limits_state_filter: Vec<Option<ActorLookLimitsStateFilter>>,
    #[serde(default)]
    pub actor_view_limit_preset: Vec<Option<ActorViewLimitPreset>>,
    #[serde(default)]
    pub actor_view_limit_preset_database: Vec<Option<ActorViewLimitPresetDatabase>>,
    #[serde(default)]
    pub actor_look_limits: Vec<Option<ActorLookLimits>>,
    #[serde(default)]
    pub actor_aim_limits: Vec<Option<ActorAimLimits>>,
    #[serde(default)]
    pub action_rule_display_params: Vec<Option<ActionRuleDisplayParams>>,
    #[serde(default)]
    pub action_rule_params: Vec<Option<ActionRuleParams>>,
    #[serde(default)]
    pub action_rule_list: Vec<Option<ActionRuleList>>,
}

/// Bucket pool sub-struct for types in the `ad` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAd {
    #[serde(default)]
    pub advanced_loot_constraints: Vec<Option<AdvancedLootConstraints>>,
}

/// Bucket pool sub-struct for types in the `ae` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAe {
    #[serde(default)]
    pub aerodynamic_trail_calculation: Vec<Option<AerodynamicTrailCalculation>>,
}

/// Bucket pool sub-struct for types in the `ai` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAi {
    #[serde(default)]
    pub aiperception_profile: Vec<Option<AIPerceptionProfile>>,
    #[serde(default)]
    pub aimercy_timer_settings: Vec<Option<AIMercyTimerSettings>>,
    #[serde(default)]
    pub aivisual_field_params: Vec<Option<AIVisualFieldParams>>,
    #[serde(default)]
    pub aicontextual_visual_field_profile: Vec<Option<AIContextualVisualFieldProfile>>,
    #[serde(default)]
    pub aivisual_field_profile: Vec<Option<AIVisualFieldProfile>>,
    #[serde(default)]
    pub aiobservable_filter_flags: Vec<Option<AIObservableFilterFlags>>,
    #[serde(default)]
    pub aiobservable_filters: Vec<Option<AIObservableFilters>>,
    #[serde(default)]
    pub aiobservable_filters_profile: Vec<Option<AIObservableFiltersProfile>>,
    #[serde(default)]
    pub aitargetable_settings: Vec<Option<AITargetableSettings>>,
    #[serde(default)]
    pub aispecial_ranged_attack_config: Vec<Option<AISpecialRangedAttackConfig>>,
    #[serde(default)]
    pub aiavailable_special_ranged_attacks_config: Vec<Option<AIAvailableSpecialRangedAttacksConfig>>,
    #[serde(default)]
    pub aifire_discipline_settings: Vec<Option<AIFireDisciplineSettings>>,
    #[serde(default)]
    pub aimotive_action_details: Vec<Option<AIMotiveActionDetails>>,
    #[serde(default)]
    pub aimotive_action: Vec<Option<AIMotiveAction>>,
    #[serde(default)]
    pub aimotive_condition: Vec<Option<AIMotiveCondition>>,
    #[serde(default)]
    pub aimotive: Vec<Option<AIMotive>>,
    #[serde(default)]
    pub aimotive_list: Vec<Option<AIMotiveList>>,
    #[serde(default)]
    pub aiprofile: Vec<Option<AIProfile>>,
    #[serde(default)]
    pub aiming: Vec<Option<Aiming>>,
    #[serde(default)]
    pub aitime_since_target_seen: Vec<Option<AITimeSinceTargetSeen>>,
    #[serde(default)]
    pub aiformula_score_modifiers: Vec<Option<AIFormulaScoreModifiers>>,
    #[serde(default)]
    pub aitargeting_formula_settings: Vec<Option<AITargetingFormulaSettings>>,
    #[serde(default)]
    pub aiwave_collection: Vec<Option<AIWaveCollection>>,
    #[serde(default)]
    pub aiwave: Vec<Option<AIWave>>,
    #[serde(default)]
    pub aiwave_member: Vec<Option<AIWaveMember>>,
    #[serde(default)]
    pub aimelee_attack: Vec<Option<AIMeleeAttack>>,
    #[serde(default)]
    pub aimelee_combat_config: Vec<Option<AIMeleeCombatConfig>>,
}

/// Bucket pool sub-struct for types in the `am` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAm {
    #[serde(default)]
    pub ammo_params: Vec<Option<AmmoParams>>,
}

/// Bucket pool sub-struct for types in the `an` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAn {
    #[serde(default)]
    pub animated_marker_marker: Vec<Option<AnimatedMarker_Marker>>,
    #[serde(default)]
    pub animated_marker: Vec<Option<AnimatedMarker>>,
    #[serde(default)]
    pub animation_graph_key_frame: Vec<Option<AnimationGraph_KeyFrame>>,
    #[serde(default)]
    pub animation_graph_track: Vec<Option<AnimationGraph_Track>>,
    #[serde(default)]
    pub animation_graph_timer: Vec<Option<AnimationGraph_Timer>>,
    #[serde(default)]
    pub animation_graph_timeline: Vec<Option<AnimationGraph_Timeline>>,
    #[serde(default)]
    pub announcement_game_token: Vec<Option<AnnouncementGameToken>>,
    #[serde(default)]
    pub announcement: Vec<Option<Announcement>>,
    #[serde(default)]
    pub announcer: Vec<Option<Announcer>>,
    #[serde(default)]
    pub ang3: Vec<Option<Ang3>>,
    #[serde(default)]
    pub ang_ypr: Vec<Option<AngYPR>>,
    #[serde(default)]
    pub animated_action: Vec<Option<AnimatedAction>>,
    #[serde(default)]
    pub animated_helmet_params: Vec<Option<AnimatedHelmetParams>>,
}

/// Bucket pool sub-struct for types in the `ap` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAp {
    #[serde(default)]
    pub apparent_temperature_params: Vec<Option<ApparentTemperatureParams>>,
}

/// Bucket pool sub-struct for types in the `ar` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAr {
    #[serde(default)]
    pub armarker_player_offset_params: Vec<Option<ARMarkerPlayerOffsetParams>>,
    #[serde(default)]
    pub armarker_global_params: Vec<Option<ARMarkerGlobalParams>>,
    #[serde(default)]
    pub arms_lock_single_ability: Vec<Option<ArmsLockSingleAbility>>,
    #[serde(default)]
    pub arms_lock_config: Vec<Option<ArmsLockConfig>>,
    #[serde(default)]
    pub arena_commander_location_object_containers_params: Vec<Option<ArenaCommanderLocationObjectContainersParams>>,
    #[serde(default)]
    pub arena_commander_planet_override_params: Vec<Option<ArenaCommanderPlanetOverrideParams>>,
    #[serde(default)]
    pub arena_commander_scenario_params: Vec<Option<ArenaCommanderScenarioParams>>,
    #[serde(default)]
    pub area_services: Vec<Option<AreaServices>>,
    #[serde(default)]
    pub armode_settings: Vec<Option<ARModeSettings>>,
    #[serde(default)]
    pub armor_move_view_restrictions: Vec<Option<ArmorMoveViewRestrictions>>,
}

/// Bucket pool sub-struct for types in the `as` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAs {
    #[serde(default)]
    pub asteroid_procedural: Vec<Option<AsteroidProcedural>>,
    #[serde(default)]
    pub asteroid_field_composition: Vec<Option<AsteroidFieldComposition>>,
    #[serde(default)]
    pub asteroid_state_template_internal: Vec<Option<AsteroidStateTemplateInternal>>,
    #[serde(default)]
    pub asteroid_state_template: Vec<Option<AsteroidStateTemplate>>,
    #[serde(default)]
    pub asteroid_behavior: Vec<Option<AsteroidBehavior>>,
    #[serde(default)]
    pub asteroid_behavior_weather_params: Vec<Option<AsteroidBehavior_WeatherParams>>,
}

/// Bucket pool sub-struct for types in the `at` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAt {
    #[serde(default)]
    pub atmospheric_flight_effects: Vec<Option<AtmosphericFlightEffects>>,
    #[serde(default)]
    pub attack_category_params_base: Vec<Option<AttackCategoryParamsBase>>,
    #[serde(default)]
    pub attack_category_params: Vec<Option<AttackCategoryParams>>,
    #[serde(default)]
    pub attack_detection_config: Vec<Option<AttackDetectionConfig>>,
    #[serde(default)]
    pub atmospheric_composition_template: Vec<Option<AtmosphericCompositionTemplate>>,
    #[serde(default)]
    pub atmosphere_state_template_internal: Vec<Option<AtmosphereStateTemplateInternal>>,
    #[serde(default)]
    pub atmosphere_state_template: Vec<Option<AtmosphereStateTemplate>>,
    #[serde(default)]
    pub atmosphere_state_pressure_template: Vec<Option<AtmosphereStatePressureTemplate>>,
    #[serde(default)]
    pub atmosphere_state_temperature_template: Vec<Option<AtmosphereStateTemperatureTemplate>>,
    #[serde(default)]
    pub atmosphere_state_humidity_template: Vec<Option<AtmosphereStateHumidityTemplate>>,
    #[serde(default)]
    pub atmosphere_behavior: Vec<Option<AtmosphereBehavior>>,
    #[serde(default)]
    pub atmosphere_behavior_turbulence_params: Vec<Option<AtmosphereBehavior_TurbulenceParams>>,
    #[serde(default)]
    pub atmosphere_behavior_weather_params: Vec<Option<AtmosphereBehavior_WeatherParams>>,
}

/// Bucket pool sub-struct for types in the `au` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAu {
    #[serde(default)]
    pub audio_breath_style_condition: Vec<Option<AudioBreathStyleCondition>>,
    #[serde(default)]
    pub audio_breath_style_condition_list: Vec<Option<AudioBreathStyleConditionList>>,
    #[serde(default)]
    pub audio_breath_style_transition_node: Vec<Option<AudioBreathStyleTransitionNode>>,
    #[serde(default)]
    pub audio_breath_style: Vec<Option<AudioBreathStyle>>,
    #[serde(default)]
    pub audio_breath_style_base_node: Vec<Option<AudioBreathStyleBaseNode>>,
    #[serde(default)]
    pub audio_breath_style_node: Vec<Option<AudioBreathStyleNode>>,
    #[serde(default)]
    pub audio_breath_style_suite: Vec<Option<AudioBreathStyleSuite>>,
    #[serde(default)]
    pub audio_breath_definition: Vec<Option<AudioBreathDefinition>>,
    #[serde(default)]
    pub audio_breath_interrupt: Vec<Option<AudioBreathInterrupt>>,
    #[serde(default)]
    pub audio_whitelist: Vec<Option<AudioWhitelist>>,
    #[serde(default)]
    pub audio_environment: Vec<Option<AudioEnvironment>>,
    #[serde(default)]
    pub audio_one_shot_manager_budget_entry: Vec<Option<AudioOneShotManagerBudgetEntry>>,
    #[serde(default)]
    pub audio_budget_definition: Vec<Option<AudioBudgetDefinition>>,
    #[serde(default)]
    pub audio_game_context_globals: Vec<Option<AudioGameContextGlobals>>,
    #[serde(default)]
    pub audio_game_context: Vec<Option<AudioGameContext>>,
    #[serde(default)]
    pub audio_game_context_setup: Vec<Option<AudioGameContextSetup>>,
    #[serde(default)]
    pub audio_tag_action: Vec<Option<AudioTagAction>>,
    #[serde(default)]
    pub audio_tag_action_list: Vec<Option<AudioTagActionList>>,
    #[serde(default)]
    pub audio_value_output_behaviour: Vec<Option<AudioValueOutputBehaviour>>,
    #[serde(default)]
    pub audio_value_output: Vec<Option<AudioValueOutput>>,
    #[serde(default)]
    pub audio_value_output_setup: Vec<Option<AudioValueOutputSetup>>,
    #[serde(default)]
    pub audio_allegiance_switches: Vec<Option<AudioAllegianceSwitches>>,
    #[serde(default)]
    pub audio_environment_feedback_zone_process: Vec<Option<AudioEnvironmentFeedbackZoneProcess>>,
    #[serde(default)]
    pub audio_environment_feedback_zone_setup: Vec<Option<AudioEnvironmentFeedbackZoneSetup>>,
    #[serde(default)]
    pub audio_environment_movement_rtpc_behavior: Vec<Option<AudioEnvironmentMovementRtpcBehavior>>,
    #[serde(default)]
    pub audio_environment_feedback_tag_and_event: Vec<Option<AudioEnvironmentFeedbackTagAndEvent>>,
    #[serde(default)]
    pub audio_environment_feedback_point_def: Vec<Option<AudioEnvironmentFeedbackPointDef>>,
    #[serde(default)]
    pub audio_hit_listener_trigger: Vec<Option<AudioHitListenerTrigger>>,
    #[serde(default)]
    pub audio_hit_type_definition: Vec<Option<AudioHitTypeDefinition>>,
    #[serde(default)]
    pub audio_hit_listener_definition: Vec<Option<AudioHitListenerDefinition>>,
    #[serde(default)]
    pub auto_spawn_settings: Vec<Option<AutoSpawnSettings>>,
    #[serde(default)]
    pub audio_footstep_surfaces_definition: Vec<Option<AudioFootstepSurfacesDefinition>>,
    #[serde(default)]
    pub audio_footstep_surface_mapping: Vec<Option<AudioFootstepSurfaceMapping>>,
    #[serde(default)]
    pub audio_signal_list: Vec<Option<AudioSignalList>>,
    #[serde(default)]
    pub audio_signal_rtpc: Vec<Option<AudioSignalRtpc>>,
    #[serde(default)]
    pub audio_signal: Vec<Option<AudioSignal>>,
    #[serde(default)]
    pub audio_rtpc: Vec<Option<AudioRtpc>>,
    #[serde(default)]
    pub audio_rtpc_with_default: Vec<Option<AudioRtpcWithDefault>>,
    #[serde(default)]
    pub audio_switch: Vec<Option<AudioSwitch>>,
    #[serde(default)]
    pub audio_rtpc_with_behaviour: Vec<Option<AudioRtpcWithBehaviour>>,
}

/// Bucket pool sub-struct for types in the `aw` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsAw {
    #[serde(default)]
    pub award_service_award: Vec<Option<AwardService_Award>>,
    #[serde(default)]
    pub award_service_config: Vec<Option<AwardService_Config>>,
}

/// Bucket pool sub-struct for types in the `ba` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsBa {
    #[serde(default)]
    pub basic_status_effect_application_type: Vec<Option<BasicStatusEffectApplicationType>>,
    #[serde(default)]
    pub base_cargo_fill_capacity_value: Vec<Option<BaseCargoFillCapacityValue>>,
    #[serde(default)]
    pub base_item: Vec<Option<BaseItem>>,
    #[serde(default)]
    pub base_journal_entry: Vec<Option<BaseJournalEntry>>,
    #[serde(default)]
    pub base_service: Vec<Option<BaseService>>,
    #[serde(default)]
    pub base_mission_property_value: Vec<Option<BaseMissionPropertyValue>>,
    #[serde(default)]
    pub base_mission_modifier: Vec<Option<BaseMissionModifier>>,
}

/// Bucket pool sub-struct for types in the `be` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsBe {
    #[serde(default)]
    pub bezier_curve: Vec<Option<BezierCurve>>,
    #[serde(default)]
    pub beacons_contracts: Vec<Option<BeaconsContracts>>,
    #[serde(default)]
    pub behavior_vehicle_effect_params: Vec<Option<Behavior_VehicleEffectParams>>,
    #[serde(default)]
    pub behavior_custom_vehicle_effects_preset: Vec<Option<Behavior_CustomVehicleEffectsPreset>>,
    #[serde(default)]
    pub behavior_atmosphere_vehicle_effect_params: Vec<Option<Behavior_AtmosphereVehicleEffectParams>>,
    #[serde(default)]
    pub behavior_electrical_vehicle_effect_params: Vec<Option<Behavior_ElectricalVehicleEffectParams>>,
}

/// Bucket pool sub-struct for types in the `bl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsBl {
    #[serde(default)]
    pub blueprint_category_record: Vec<Option<BlueprintCategoryRecord>>,
    #[serde(default)]
    pub blueprint_category_database_record: Vec<Option<BlueprintCategoryDatabaseRecord>>,
    #[serde(default)]
    pub blocked_text_params: Vec<Option<BlockedTextParams>>,
    #[serde(default)]
    pub blocked_cursor_params: Vec<Option<BlockedCursorParams>>,
    #[serde(default)]
    pub blocked_color_params: Vec<Option<BlockedColorParams>>,
    #[serde(default)]
    pub blocked_hint_params: Vec<Option<BlockedHintParams>>,
    #[serde(default)]
    pub blueprint_reward: Vec<Option<BlueprintReward>>,
    #[serde(default)]
    pub blueprint_pool_record: Vec<Option<BlueprintPoolRecord>>,
    #[serde(default)]
    pub blob_vfxshared_params: Vec<Option<BlobVFXSharedParams>>,
    #[serde(default)]
    pub blob_vfxdistance_params: Vec<Option<BlobVFXDistanceParams>>,
}

/// Bucket pool sub-struct for types in the `bo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsBo {
    #[serde(default)]
    pub bone_counter_rotate_config: Vec<Option<BoneCounterRotateConfig>>,
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
    pub boxout_stat: Vec<Option<BoxoutStat>>,
    #[serde(default)]
    pub boxout_item_status: Vec<Option<BoxoutItemStatus>>,
    #[serde(default)]
    pub boxout_atmosphere_status: Vec<Option<BoxoutAtmosphereStatus>>,
}

/// Bucket pool sub-struct for types in the `br` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsBr {
    #[serde(default)]
    pub breathable_gas_params: Vec<Option<BreathableGasParams>>,
    #[serde(default)]
    pub breathable_oxygen_params: Vec<Option<BreathableOxygenParams>>,
    #[serde(default)]
    pub breathing_helper_params: Vec<Option<BreathingHelperParams>>,
    #[serde(default)]
    pub breath_volume_params: Vec<Option<BreathVolumeParams>>,
    #[serde(default)]
    pub breath_duration_params: Vec<Option<BreathDurationParams>>,
    #[serde(default)]
    pub breathing_trigger_def: Vec<Option<BreathingTriggerDef>>,
}

/// Bucket pool sub-struct for types in the `bu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsBu {
    #[serde(default)]
    pub burst: Vec<Option<Burst>>,
    #[serde(default)]
    pub buff_duration_base: Vec<Option<BuffDurationBase>>,
    #[serde(default)]
    pub buff_value_override: Vec<Option<BuffValueOverride>>,
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
    pub building_blocks_timeline_type_embedded: Vec<Option<BuildingBlocks_TimelineTypeEmbedded>>,
    #[serde(default)]
    pub building_blocks_timeline: Vec<Option<BuildingBlocks_Timeline>>,
    #[serde(default)]
    pub building_blocks_keyframe: Vec<Option<BuildingBlocks_Keyframe>>,
    #[serde(default)]
    pub building_blocks_keyframe_modifier_data: Vec<Option<BuildingBlocks_KeyframeModifierData>>,
    #[serde(default)]
    pub building_blocks_canvas: Vec<Option<BuildingBlocks_Canvas>>,
    #[serde(default)]
    pub building_blocks_font_style: Vec<Option<BuildingBlocks_FontStyle>>,
    #[serde(default)]
    pub building_blocks_font_replacement_pair: Vec<Option<BuildingBlocks_FontReplacementPair>>,
    #[serde(default)]
    pub building_blocks_language_specific_font_replacement: Vec<Option<BuildingBlocks_LanguageSpecificFontReplacement>>,
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
    pub building_blocks_text_format_modifier_base: Vec<Option<BuildingBlocks_TextFormatModifierBase>>,
    #[serde(default)]
    pub building_blocks_text_emphasis_modifier_list: Vec<Option<BuildingBlocks_TextEmphasisModifierList>>,
    #[serde(default)]
    pub building_blocks_style: Vec<Option<BuildingBlocks_Style>>,
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
}

/// Bucket pool sub-struct for types in the `ca` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsCa {
    #[serde(default)]
    pub camera_lens_streak: Vec<Option<CameraLensStreak>>,
    #[serde(default)]
    pub camera_lens_distortion: Vec<Option<CameraLensDistortion>>,
    #[serde(default)]
    pub camera_lens_chromatic_aberration: Vec<Option<CameraLensChromaticAberration>>,
    #[serde(default)]
    pub camera_lens_ghost_instance: Vec<Option<CameraLensGhostInstance>>,
    #[serde(default)]
    pub camera_lens_ghost_set: Vec<Option<CameraLensGhostSet>>,
    #[serde(default)]
    pub camera_lens_params: Vec<Option<CameraLensParams>>,
    #[serde(default)]
    pub camera: Vec<Option<Camera>>,
    #[serde(default)]
    pub camera_base_settings_config: Vec<Option<CameraBaseSettingsConfig>>,
    #[serde(default)]
    pub camera_blend_config: Vec<Option<CameraBlendConfig>>,
    #[serde(default)]
    pub camera_actor_vibration_shake_config: Vec<Option<CameraActorVibrationShakeConfig>>,
    #[serde(default)]
    pub camera_fovconfig: Vec<Option<CameraFOVConfig>>,
    #[serde(default)]
    pub camera_base_config: Vec<Option<CameraBaseConfig>>,
    #[serde(default)]
    pub camera_shop_item_offset: Vec<Option<CameraShopItemOffset>>,
    #[serde(default)]
    pub camera_shop_config: Vec<Option<CameraShopConfig>>,
    #[serde(default)]
    pub camera_fovchange_data: Vec<Option<CameraFOVChangeData>>,
    #[serde(default)]
    pub cargo_manifest: Vec<Option<CargoManifest>>,
    #[serde(default)]
    pub carry_config: Vec<Option<CarryConfig>>,
    #[serde(default)]
    pub camera_effects_modifiers: Vec<Option<CameraEffectsModifiers>>,
    #[serde(default)]
    pub carryable_interactions_metadata_def: Vec<Option<CarryableInteractionsMetadataDef>>,
    #[serde(default)]
    pub carryable_interactions_metadata_config_def: Vec<Option<CarryableInteractionsMetadataConfigDef>>,
    #[serde(default)]
    pub cargo_loading_notification_params: Vec<Option<CargoLoadingNotificationParams>>,
    #[serde(default)]
    pub cargo_grid_occupant_face: Vec<Option<CargoGridOccupantFace>>,
    #[serde(default)]
    pub cargo_grid_occupant_properties: Vec<Option<CargoGridOccupantProperties>>,
    #[serde(default)]
    pub capacitor_assignment_input_output_def: Vec<Option<CapacitorAssignmentInputOutputDef>>,
    #[serde(default)]
    pub camera_transition_interpolation_curve_record: Vec<Option<CameraTransitionInterpolationCurveRecord>>,
}

/// Bucket pool sub-struct for types in the `ch` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsCh {
    #[serde(default)]
    pub character_accuracy_modifiers: Vec<Option<CharacterAccuracyModifiers>>,
    #[serde(default)]
    pub character_skills: Vec<Option<CharacterSkills>>,
    #[serde(default)]
    pub character_personal_data: Vec<Option<CharacterPersonalData>>,
    #[serde(default)]
    pub character: Vec<Option<Character>>,
    #[serde(default)]
    pub character_serialization_settings_preset: Vec<Option<CharacterSerializationSettingsPreset>>,
    #[serde(default)]
    pub character_random_name_params: Vec<Option<CharacterRandomNameParams>>,
    #[serde(default)]
    pub chat_emote_record: Vec<Option<ChatEmoteRecord>>,
    #[serde(default)]
    pub chat_emote_pack: Vec<Option<ChatEmotePack>>,
    #[serde(default)]
    pub chat_emote_data: Vec<Option<ChatEmoteData>>,
    #[serde(default)]
    pub chat_emote_anim_data: Vec<Option<ChatEmoteAnimData>>,
    #[serde(default)]
    pub chat_command_fast_access: Vec<Option<ChatCommandFastAccess>>,
    #[serde(default)]
    pub chat_command_name: Vec<Option<ChatCommandName>>,
    #[serde(default)]
    pub chat_filter_options: Vec<Option<ChatFilterOptions>>,
    #[serde(default)]
    pub chat_filter: Vec<Option<ChatFilter>>,
    #[serde(default)]
    pub chat_manager_default_channel_color: Vec<Option<ChatManagerDefaultChannelColor>>,
    #[serde(default)]
    pub chat_manager_color: Vec<Option<ChatManagerColor>>,
    #[serde(default)]
    pub chat_manager_global_params: Vec<Option<ChatManagerGlobalParams>>,
    #[serde(default)]
    pub chat_channel_filter_record: Vec<Option<ChatChannelFilterRecord>>,
    #[serde(default)]
    pub chat_system_options_module: Vec<Option<ChatSystemOptionsModule>>,
    #[serde(default)]
    pub child_mission_phase: Vec<Option<ChildMissionPhase>>,
}

/// Bucket pool sub-struct for types in the `ci` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsCi {
    #[serde(default)]
    pub cinematic_camera_controller_setup: Vec<Option<CinematicCameraControllerSetup>>,
    #[serde(default)]
    pub cinematic_conversation_settings: Vec<Option<CinematicConversationSettings>>,
    #[serde(default)]
    pub cinematic_config: Vec<Option<CinematicConfig>>,
    #[serde(default)]
    pub cinematic_flythrough_point: Vec<Option<CinematicFlythroughPoint>>,
    #[serde(default)]
    pub cinematic_flight_points_record: Vec<Option<CinematicFlightPointsRecord>>,
}

/// Bucket pool sub-struct for types in the `co` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsCo {
    #[serde(default)]
    pub cost_modifier_per_agent_type: Vec<Option<CostModifierPerAgentType>>,
    #[serde(default)]
    pub common_targeting_same_target_score: Vec<Option<CommonTargetingSameTargetScore>>,
    #[serde(default)]
    pub common_target_visibility_score: Vec<Option<CommonTargetVisibilityScore>>,
    #[serde(default)]
    pub common_current_target_distance_score: Vec<Option<CommonCurrentTargetDistanceScore>>,
    #[serde(default)]
    pub common_tactic_scores: Vec<Option<CommonTacticScores>>,
    #[serde(default)]
    pub consumable_params: Vec<Option<ConsumableParams>>,
    #[serde(default)]
    pub combat_marker: Vec<Option<CombatMarker>>,
    #[serde(default)]
    pub cockpit_response_variation: Vec<Option<CockpitResponseVariation>>,
    #[serde(default)]
    pub cockpit_response: Vec<Option<CockpitResponse>>,
    #[serde(default)]
    pub cockpit_responses: Vec<Option<CockpitResponses>>,
    #[serde(default)]
    pub cockpit_rule_base: Vec<Option<CockpitRuleBase>>,
    #[serde(default)]
    pub commodity_temperature_tolerance: Vec<Option<CommodityTemperatureTolerance>>,
    #[serde(default)]
    pub communication_config: Vec<Option<CommunicationConfig>>,
    #[serde(default)]
    pub communication_entry: Vec<Option<CommunicationEntry>>,
    #[serde(default)]
    pub communication_variation: Vec<Option<CommunicationVariation>>,
    #[serde(default)]
    pub communication_variation_rules: Vec<Option<CommunicationVariationRules>>,
    #[serde(default)]
    pub communication_variation_condition: Vec<Option<CommunicationVariationCondition>>,
    #[serde(default)]
    pub communication_channel_config: Vec<Option<CommunicationChannelConfig>>,
    #[serde(default)]
    pub communication_channel: Vec<Option<CommunicationChannel>>,
    #[serde(default)]
    pub communication_subtitle_settings: Vec<Option<CommunicationSubtitleSettings>>,
    #[serde(default)]
    pub communication_audio_rtpc: Vec<Option<CommunicationAudioRTPC>>,
    #[serde(default)]
    pub communication_variable_config: Vec<Option<CommunicationVariableConfig>>,
    #[serde(default)]
    pub communication_variable_base: Vec<Option<CommunicationVariableBase>>,
    #[serde(default)]
    pub communication_variable_bool: Vec<Option<CommunicationVariableBool>>,
    #[serde(default)]
    pub communication_atlconfig: Vec<Option<CommunicationATLConfig>>,
    #[serde(default)]
    pub communication_location_auto_tags: Vec<Option<CommunicationLocationAutoTags>>,
    #[serde(default)]
    pub communication_auto_mannequin_tags_config: Vec<Option<CommunicationAutoMannequinTagsConfig>>,
    #[serde(default)]
    pub contextual_communication_config: Vec<Option<ContextualCommunicationConfig>>,
    #[serde(default)]
    pub contextual_communication_response: Vec<Option<ContextualCommunicationResponse>>,
    #[serde(default)]
    pub communication_request: Vec<Option<CommunicationRequest>>,
    #[serde(default)]
    pub contextual_communication_condition: Vec<Option<ContextualCommunicationCondition>>,
    #[serde(default)]
    pub communication_name: Vec<Option<CommunicationName>>,
    #[serde(default)]
    pub communication_channel_name: Vec<Option<CommunicationChannelName>>,
    #[serde(default)]
    pub conversation_sticky_filter: Vec<Option<ConversationStickyFilter>>,
    #[serde(default)]
    pub conversation: Vec<Option<Conversation>>,
    #[serde(default)]
    pub conversation_bank: Vec<Option<ConversationBank>>,
    #[serde(default)]
    pub conversation_node_base: Vec<Option<ConversationNode_Base>>,
    #[serde(default)]
    pub constant_dofpos_weights: Vec<Option<ConstantDOFPosWeights>>,
    #[serde(default)]
    pub constant_dofweights: Vec<Option<ConstantDOFWeights>>,
    #[serde(default)]
    pub constant_dofgrid: Vec<Option<ConstantDOFGrid>>,
    #[serde(default)]
    pub constant_dofglobal_data: Vec<Option<ConstantDOFGlobalData>>,
    #[serde(default)]
    pub commodity_type: Vec<Option<CommodityType>>,
    #[serde(default)]
    pub commodity_subtype: Vec<Option<CommoditySubtype>>,
    #[serde(default)]
    pub commodity_type_database: Vec<Option<CommodityTypeDatabase>>,
    #[serde(default)]
    pub commodity_damage_configuration: Vec<Option<CommodityDamageConfiguration>>,
    #[serde(default)]
    pub condition_prohibited_items_display_params: Vec<Option<ConditionProhibitedItemsDisplayParams>>,
    #[serde(default)]
    pub condition_display_params: Vec<Option<ConditionDisplayParams>>,
    #[serde(default)]
    pub controlled_substance_class: Vec<Option<ControlledSubstanceClass>>,
    #[serde(default)]
    pub control_hint_game_mode_records: Vec<Option<ControlHintGameModeRecords>>,
    #[serde(default)]
    pub control_hints_input: Vec<Option<ControlHints_Input>>,
    #[serde(default)]
    pub control_hints_hint_display_info_action: Vec<Option<ControlHints_HintDisplayInfoAction>>,
    #[serde(default)]
    pub control_hint_display_info_set: Vec<Option<ControlHint_DisplayInfoSet>>,
    #[serde(default)]
    pub control_hint_condition: Vec<Option<ControlHintCondition>>,
    #[serde(default)]
    pub control_hint_def: Vec<Option<ControlHintDef>>,
    #[serde(default)]
    pub control_hint_always_display_condition: Vec<Option<ControlHintAlwaysDisplayCondition>>,
    #[serde(default)]
    pub control_hint_entry: Vec<Option<ControlHint_Entry>>,
    #[serde(default)]
    pub control_hints_preset: Vec<Option<ControlHints_Preset>>,
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
    pub contract_difficulty_profile: Vec<Option<ContractDifficultyProfile>>,
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
    pub comms_notification_selector: Vec<Option<CommsNotificationSelector>>,
    #[serde(default)]
    pub comms_notification_stage_camera: Vec<Option<CommsNotificationStageCamera>>,
    #[serde(default)]
    pub comms_notification_stage_actor_mark: Vec<Option<CommsNotificationStageActorMark>>,
    #[serde(default)]
    pub comms_notification_stage_list_item: Vec<Option<CommsNotificationStageListItem>>,
    #[serde(default)]
    pub comms_notification_stage: Vec<Option<CommsNotificationStage>>,
    #[serde(default)]
    pub comms_notification: Vec<Option<CommsNotification>>,
    #[serde(default)]
    pub comms_notifications_global_params: Vec<Option<CommsNotificationsGlobalParams>>,
    #[serde(default)]
    pub contact_state_global_params: Vec<Option<ContactStateGlobalParams>>,
    #[serde(default)]
    pub contact_highlight_layers_params: Vec<Option<ContactHighlightLayersParams>>,
    #[serde(default)]
    pub contact_highlight_shared_params: Vec<Option<ContactHighlightSharedParams>>,
    #[serde(default)]
    pub contact_highlight_visual_base_params: Vec<Option<ContactHighlightVisualBaseParams>>,
    #[serde(default)]
    pub contact_highlight_state_params: Vec<Option<ContactHighlightStateParams>>,
    #[serde(default)]
    pub contact_tagging_shared_params: Vec<Option<ContactTaggingSharedParams>>,
    #[serde(default)]
    pub consumable_type: Vec<Option<ConsumableType>>,
    #[serde(default)]
    pub consumable_subtype: Vec<Option<ConsumableSubtype>>,
    #[serde(default)]
    pub consumable_type_database: Vec<Option<ConsumableTypeDatabase>>,
    #[serde(default)]
    pub consumable_effect: Vec<Option<ConsumableEffect>>,
    #[serde(default)]
    pub comms_audio_effect: Vec<Option<CommsAudioEffect>>,
    #[serde(default)]
    pub corpse_interaction_params: Vec<Option<CorpseInteractionParams>>,
    #[serde(default)]
    pub completion_type_base: Vec<Option<CompletionTypeBase>>,
    #[serde(default)]
    pub comms_channel_def: Vec<Option<CommsChannelDef>>,
}

/// Bucket pool sub-struct for types in the `cr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsCr {
    #[serde(default)]
    pub crafting_gameplay_property_def: Vec<Option<CraftingGameplayPropertyDef>>,
    #[serde(default)]
    pub crafting_blueprint_base_non_ref: Vec<Option<CraftingBlueprint_Base_NonRef>>,
    #[serde(default)]
    pub crafting_blueprint_record: Vec<Option<CraftingBlueprintRecord>>,
    #[serde(default)]
    pub crafting_quality_distribution_base_non_ref: Vec<Option<CraftingQualityDistribution_Base_NonRef>>,
    #[serde(default)]
    pub crafting_quality_distribution_record: Vec<Option<CraftingQualityDistributionRecord>>,
    #[serde(default)]
    pub crafting_quality_location_override_base_non_ref: Vec<Option<CraftingQualityLocationOverride_Base_NonRef>>,
    #[serde(default)]
    pub crafting_quality_location_override_record: Vec<Option<CraftingQualityLocationOverrideRecord>>,
    #[serde(default)]
    pub crafting_global_params: Vec<Option<CraftingGlobalParams>>,
    #[serde(default)]
    pub crew_manifest: Vec<Option<CrewManifest>>,
    #[serde(default)]
    pub crew_data: Vec<Option<CrewData>>,
    #[serde(default)]
    pub cross_section_global_params: Vec<Option<CrossSectionGlobalParams>>,
}

/// Bucket pool sub-struct for types in the `ct` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsCt {
    #[serde(default)]
    pub ctx_graph_node: Vec<Option<CtxGraph_Node>>,
    #[serde(default)]
    pub ctx_graph_dependency: Vec<Option<CtxGraph_Dependency>>,
    #[serde(default)]
    pub ctx_graph_group: Vec<Option<CtxGraph_Group>>,
    #[serde(default)]
    pub ctx_graph_context: Vec<Option<CtxGraph_Context>>,
    #[serde(default)]
    pub ctx_graph_component: Vec<Option<CtxGraph_Component>>,
    #[serde(default)]
    pub ctx_graph: Vec<Option<CtxGraph>>,
}

/// Bucket pool sub-struct for types in the `cu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsCu {
    #[serde(default)]
    pub custom_scan_procedure_params: Vec<Option<CustomScanProcedureParams>>,
}

/// Bucket pool sub-struct for types in the `cy` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsCy {
    #[serde(default)]
    pub cycling_channel_option: Vec<Option<CyclingChannelOption>>,
}

/// Bucket pool sub-struct for types in the `da` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsDa {
    #[serde(default)]
    pub damage_base: Vec<Option<DamageBase>>,
    #[serde(default)]
    pub damage_info: Vec<Option<DamageInfo>>,
    #[serde(default)]
    pub damage_macro: Vec<Option<DamageMacro>>,
    #[serde(default)]
    pub damage_resistance_base: Vec<Option<DamageResistanceBase>>,
    #[serde(default)]
    pub damage_resistance_entry: Vec<Option<DamageResistanceEntry>>,
    #[serde(default)]
    pub damage_resistance: Vec<Option<DamageResistance>>,
    #[serde(default)]
    pub damage_resistance_macro: Vec<Option<DamageResistanceMacro>>,
    #[serde(default)]
    pub damage_map_channels: Vec<Option<DamageMapChannels>>,
    #[serde(default)]
    pub damage_map_damage_types: Vec<Option<DamageMapDamageTypes>>,
    #[serde(default)]
    pub damage_map_damage_form: Vec<Option<DamageMapDamageForm>>,
    #[serde(default)]
    pub damage_map_global_params: Vec<Option<DamageMapGlobalParams>>,
    #[serde(default)]
    pub date: Vec<Option<Date>>,
    #[serde(default)]
    pub date_time: Vec<Option<DateTime>>,
    #[serde(default)]
    pub date_time_schedule: Vec<Option<DateTimeSchedule>>,
    #[serde(default)]
    pub data_forge_component_params: Vec<Option<DataForgeComponentParams>>,
}

/// Bucket pool sub-struct for types in the `de` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsDe {
    #[serde(default)]
    pub deg3: Vec<Option<Deg3>>,
    #[serde(default)]
    pub default_blueprint_selection_base: Vec<Option<DefaultBlueprintSelection_Base>>,
    #[serde(default)]
    pub default_action_def: Vec<Option<DefaultActionDef>>,
    #[serde(default)]
    pub default_actions_params: Vec<Option<DefaultActionsParams>>,
    #[serde(default)]
    pub default_actions: Vec<Option<DefaultActions>>,
    #[serde(default)]
    pub default_action_description_override: Vec<Option<DefaultActionDescriptionOverride>>,
    #[serde(default)]
    pub default_actions_entry: Vec<Option<DefaultActionsEntry>>,
    #[serde(default)]
    pub default_actions_entity_entry_condition: Vec<Option<DefaultActionsEntityEntryCondition>>,
    #[serde(default)]
    pub default_actions_entity_state: Vec<Option<DefaultActionsEntityState>>,
    #[serde(default)]
    pub default_entitlement_record: Vec<Option<DefaultEntitlementRecord>>,
    #[serde(default)]
    pub default_player_loadout_entitlement_params: Vec<Option<DefaultPlayerLoadoutEntitlementParams>>,
    #[serde(default)]
    pub default_player_loadout_entitlement_record: Vec<Option<DefaultPlayerLoadoutEntitlementRecord>>,
    #[serde(default)]
    pub dematerialize_animation: Vec<Option<DematerializeAnimation>>,
    #[serde(default)]
    pub dev_owner_type_base: Vec<Option<DevOwnerType_Base>>,
    #[serde(default)]
    pub dev_owner: Vec<Option<DevOwner>>,
    #[serde(default)]
    pub delta_signature_sensitivity_params: Vec<Option<DeltaSignatureSensitivityParams>>,
    #[serde(default)]
    pub delta_signature_spike_params: Vec<Option<DeltaSignatureSpikeParams>>,
    #[serde(default)]
    pub debug_loadout_kit: Vec<Option<DebugLoadoutKit>>,
}

/// Bucket pool sub-struct for types in the `di` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsDi {
    #[serde(default)]
    pub dialogue_bundle: Vec<Option<DialogueBundle>>,
    #[serde(default)]
    pub dialogue_external_source: Vec<Option<DialogueExternalSource>>,
    #[serde(default)]
    pub dialogue_content: Vec<Option<DialogueContent>>,
    #[serde(default)]
    pub dialogue_content_bank: Vec<Option<DialogueContentBank>>,
    #[serde(default)]
    pub dialogue_context: Vec<Option<DialogueContext>>,
    #[serde(default)]
    pub dialogue_context_bank: Vec<Option<DialogueContextBank>>,
    #[serde(default)]
    pub dialogue_realm: Vec<Option<DialogueRealm>>,
    #[serde(default)]
    pub digital_signage_content: Vec<Option<DigitalSignageContent>>,
    #[serde(default)]
    pub digital_signage_content_set: Vec<Option<DigitalSignageContentSet>>,
    #[serde(default)]
    pub direct_rtt_chromatic_aberration_params: Vec<Option<DirectRTT_ChromaticAberrationParams>>,
    #[serde(default)]
    pub direct_rtt_drop_shadow_params: Vec<Option<DirectRTT_DropShadowParams>>,
    #[serde(default)]
    pub direct_rtt_bloom_params: Vec<Option<DirectRTT_BloomParams>>,
    #[serde(default)]
    pub direct_rtt_pixel_grid_params: Vec<Option<DirectRTT_PixelGridParams>>,
    #[serde(default)]
    pub direct_rtt_interference_params: Vec<Option<DirectRTT_InterferenceParams>>,
    #[serde(default)]
    pub direct_rtt_after_tonemapping_params: Vec<Option<DirectRTT_AfterTonemappingParams>>,
    #[serde(default)]
    pub difficulty_modifier_range: Vec<Option<DifficultyModifierRange>>,
    #[serde(default)]
    pub difficulty_level_params: Vec<Option<DifficultyLevelParams>>,
    #[serde(default)]
    pub display_state: Vec<Option<DisplayState>>,
}

/// Bucket pool sub-struct for types in the `do` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsDo {
    #[serde(default)]
    pub downed_config: Vec<Option<DownedConfig>>,
    #[serde(default)]
    pub docking_slot_visibility_tag_set: Vec<Option<DockingSlotVisibilityTagSet>>,
    #[serde(default)]
    pub docking_slot_visibility_rule: Vec<Option<DockingSlotVisibilityRule>>,
    #[serde(default)]
    pub docking_slot_visibility: Vec<Option<DockingSlotVisibility>>,
    #[serde(default)]
    pub docking_sensitivity: Vec<Option<DockingSensitivity>>,
}

/// Bucket pool sub-struct for types in the `dr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsDr {
    #[serde(default)]
    pub drifting_consciousness_config: Vec<Option<DriftingConsciousnessConfig>>,
    #[serde(default)]
    pub drifting_drunk_bdleffects: Vec<Option<DriftingDrunkBDLEffects>>,
    #[serde(default)]
    pub drifting_drunk_config: Vec<Option<DriftingDrunkConfig>>,
    #[serde(default)]
    pub drug_efficacy: Vec<Option<DrugEfficacy>>,
    #[serde(default)]
    pub drug_efficacy_for_consumable_type: Vec<Option<DrugEfficacyForConsumableType>>,
    #[serde(default)]
    pub drug_efficacy_config_for_item_sub_type_base: Vec<Option<DrugEfficacyConfigForItemSubTypeBase>>,
    #[serde(default)]
    pub drug_efficacy_for_item_type: Vec<Option<DrugEfficacyForItemType>>,
    #[serde(default)]
    pub drug_type_to_apply: Vec<Option<DrugTypeToApply>>,
}

/// Bucket pool sub-struct for types in the `du` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsDu {
    #[serde(default)]
    pub duck_pose: Vec<Option<DuckPose>>,
    #[serde(default)]
    pub duration_tags: Vec<Option<DurationTags>>,
}

/// Bucket pool sub-struct for types in the `dy` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsDy {
    #[serde(default)]
    pub dynamic_camera_effects: Vec<Option<DynamicCameraEffects>>,
    #[serde(default)]
    pub dynamic_camera_effects_renderer_params: Vec<Option<DynamicCameraEffectsRendererParams>>,
    #[serde(default)]
    pub dynamic_camera_effects_list: Vec<Option<DynamicCameraEffectsList>>,
}

/// Bucket pool sub-struct for types in the `ea` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsEa {
    #[serde(default)]
    pub eagame_completion_award_base_params: Vec<Option<EAGameCompletionAwardBaseParams>>,
}

/// Bucket pool sub-struct for types in the `el` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsEl {
    #[serde(default)]
    pub electrical_state_template_internal: Vec<Option<ElectricalStateTemplateInternal>>,
    #[serde(default)]
    pub electrical_state_template: Vec<Option<ElectricalStateTemplate>>,
    #[serde(default)]
    pub electrical_calculation_property_range: Vec<Option<ElectricalCalculationPropertyRange>>,
    #[serde(default)]
    pub electrical_behavior: Vec<Option<ElectricalBehavior>>,
}

/// Bucket pool sub-struct for types in the `em` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsEm {
    #[serde(default)]
    pub emotion_description: Vec<Option<EmotionDescription>>,
    #[serde(default)]
    pub emotion_list: Vec<Option<EmotionList>>,
}

/// Bucket pool sub-struct for types in the `en` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsEn {
    #[serde(default)]
    pub environment_temperature_params: Vec<Option<EnvironmentTemperatureParams>>,
    #[serde(default)]
    pub entity_audio_controller_type_params: Vec<Option<EntityAudioControllerTypeParams>>,
    #[serde(default)]
    pub entity_audio_controller_type_management_params: Vec<Option<EntityAudioControllerTypeManagementParams>>,
    #[serde(default)]
    pub entity_audio_controller_manager_params: Vec<Option<EntityAudioControllerManagerParams>>,
    #[serde(default)]
    pub entity_audio_controller_rtpc_subscriber_list_def: Vec<Option<EntityAudioControllerRtpcSubscriberListDef>>,
    #[serde(default)]
    pub entitlement_item_type: Vec<Option<EntitlementItemType>>,
    #[serde(default)]
    pub entitlement_account_item_global_params: Vec<Option<EntitlementAccountItemGlobalParams>>,
    #[serde(default)]
    pub entitlement_non_inventory_storable_item_global_params: Vec<Option<EntitlementNonInventoryStorableItemGlobalParams>>,
    #[serde(default)]
    pub entity_class_definition: Vec<Option<EntityClassDefinition>>,
    #[serde(default)]
    pub entity_class_static_data_params: Vec<Option<EntityClassStaticDataParams>>,
    #[serde(default)]
    pub entity_default_loadout_params: Vec<Option<EntityDefaultLoadoutParams>>,
    #[serde(default)]
    pub engineering_state_messages: Vec<Option<EngineeringStateMessages>>,
    #[serde(default)]
    pub entry_optional_data_base: Vec<Option<EntryOptionalData_Base>>,
    #[serde(default)]
    pub entity_cluster_id: Vec<Option<EntityClusterId>>,
    #[serde(default)]
    pub entity_cluster_member: Vec<Option<EntityClusterMember>>,
    #[serde(default)]
    pub enemy_awareness_config: Vec<Option<EnemyAwarenessConfig>>,
}

/// Bucket pool sub-struct for types in the `es` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsEs {
    #[serde(default)]
    pub espparams: Vec<Option<ESPParams>>,
}

/// Bucket pool sub-struct for types in the `ev` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsEv {
    #[serde(default)]
    pub evaconnection: Vec<Option<EVAConnection>>,
    #[serde(default)]
    pub evastate: Vec<Option<EVAState>>,
    #[serde(default)]
    pub evagraph: Vec<Option<EVAGraph>>,
    #[serde(default)]
    pub evareticle_config: Vec<Option<EVAReticle_Config>>,
}

/// Bucket pool sub-struct for types in the `ex` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsEx {
    #[serde(default)]
    pub explosion_flashbang_params: Vec<Option<ExplosionFlashbangParams>>,
    #[serde(default)]
    pub explosive_fragment_params: Vec<Option<ExplosiveFragmentParams>>,
    #[serde(default)]
    pub explosion_params: Vec<Option<ExplosionParams>>,
    #[serde(default)]
    pub explosive_ordnance_ping_vfx: Vec<Option<ExplosiveOrdnancePingVFX>>,
    #[serde(default)]
    pub explosive_ordnance_ping_global_params: Vec<Option<ExplosiveOrdnancePingGlobalParams>>,
}

/// Bucket pool sub-struct for types in the `fa` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsFa {
    #[serde(default)]
    pub faction: Vec<Option<Faction>>,
    #[serde(default)]
    pub faction_palettes: Vec<Option<FactionPalettes>>,
    #[serde(default)]
    pub faction_palette: Vec<Option<FactionPalette>>,
    #[serde(default)]
    pub faction_reputation: Vec<Option<FactionReputation>>,
    #[serde(default)]
    pub faction_relationship: Vec<Option<FactionRelationship>>,
    #[serde(default)]
    pub faction_legacy: Vec<Option<Faction_LEGACY>>,
}

/// Bucket pool sub-struct for types in the `fi` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsFi {
    #[serde(default)]
    pub fidget: Vec<Option<Fidget>>,
    #[serde(default)]
    pub fidget_config: Vec<Option<FidgetConfig>>,
    #[serde(default)]
    pub fire_hazard_surface_properties: Vec<Option<FireHazardSurfaceProperties>>,
    #[serde(default)]
    pub fire_hazard_fire_properties: Vec<Option<FireHazardFireProperties>>,
    #[serde(default)]
    pub fire_hazard_afterglow_properties: Vec<Option<FireHazardAfterglowProperties>>,
    #[serde(default)]
    pub fire_hazard_permanent_effects: Vec<Option<FireHazardPermanentEffects>>,
    #[serde(default)]
    pub fire_hazard_spawn_params: Vec<Option<FireHazardSpawnParams>>,
    #[serde(default)]
    pub fire_hazard_fog_noise_params: Vec<Option<FireHazardFogNoiseParams>>,
    #[serde(default)]
    pub fire_hazard_fog_params: Vec<Option<FireHazardFogParams>>,
    #[serde(default)]
    pub fire_hazard_global_params: Vec<Option<FireHazardGlobalParams>>,
    #[serde(default)]
    pub fire_hazard_global_update: Vec<Option<FireHazardGlobalUpdate>>,
    #[serde(default)]
    pub fire_hazard_global_ignition: Vec<Option<FireHazardGlobalIgnition>>,
    #[serde(default)]
    pub fire_hazard_global_propagation: Vec<Option<FireHazardGlobalPropagation>>,
    #[serde(default)]
    pub fire_hazard_global_smoke_params: Vec<Option<FireHazardGlobalSmokeParams>>,
    #[serde(default)]
    pub fire_hazard_global_damage_to_health_params: Vec<Option<FireHazardGlobalDamageToHealthParams>>,
    #[serde(default)]
    pub fire_hazard_global_extinguishing: Vec<Option<FireHazardGlobalExtinguishing>>,
    #[serde(default)]
    pub fire_hazard_global_default_effects: Vec<Option<FireHazardGlobalDefaultEffects>>,
    #[serde(default)]
    pub fire_hazard_global_light_params: Vec<Option<FireHazardGlobalLightParams>>,
    #[serde(default)]
    pub fire_hazard_global_room_connector_params: Vec<Option<FireHazardGlobalRoomConnectorParams>>,
}

/// Bucket pool sub-struct for types in the `fl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsFl {
    #[serde(default)]
    pub flash_object_binding_group: Vec<Option<FlashObjectBindingGroup>>,
    #[serde(default)]
    pub flash_variable_object: Vec<Option<FlashVariableObject>>,
    #[serde(default)]
    pub float_factor_range: Vec<Option<FloatFactorRange>>,
    #[serde(default)]
    pub flash_palette: Vec<Option<Flash_Palette>>,
    #[serde(default)]
    pub flash_palette_entry: Vec<Option<Flash_PaletteEntry>>,
    #[serde(default)]
    pub flight_huduimessage: Vec<Option<FlightHUDUIMessage>>,
    #[serde(default)]
    pub flight_huduiview_config: Vec<Option<FlightHUDUIView_Config>>,
}

/// Bucket pool sub-struct for types in the `fo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsFo {
    #[serde(default)]
    pub foley_definition: Vec<Option<FoleyDefinition>>,
    #[serde(default)]
    pub foley_bone: Vec<Option<FoleyBone>>,
    #[serde(default)]
    pub foley_axis: Vec<Option<FoleyAxis>>,
    #[serde(default)]
    pub foley_one_shot: Vec<Option<FoleyOneShot>>,
    #[serde(default)]
    pub foley_collision: Vec<Option<FoleyCollision>>,
    #[serde(default)]
    pub foley_loop: Vec<Option<FoleyLoop>>,
    #[serde(default)]
    pub foley_item: Vec<Option<FoleyItem>>,
    #[serde(default)]
    pub foley_footstep_definition: Vec<Option<FoleyFootstepDefinition>>,
    #[serde(default)]
    pub formation: Vec<Option<Formation>>,
    #[serde(default)]
    pub formation_offset: Vec<Option<FormationOffset>>,
    #[serde(default)]
    pub force_feedback: Vec<Option<ForceFeedback>>,
    #[serde(default)]
    pub force_feedback_pattern: Vec<Option<ForceFeedbackPattern>>,
    #[serde(default)]
    pub force_feedback_envelope: Vec<Option<ForceFeedbackEnvelope>>,
    #[serde(default)]
    pub force_feedback_effect: Vec<Option<ForceFeedbackEffect>>,
    #[serde(default)]
    pub force_feedback_motor: Vec<Option<ForceFeedbackMotor>>,
}

/// Bucket pool sub-struct for types in the `fp` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsFp {
    #[serde(default)]
    pub fpsreticle_config: Vec<Option<FPSReticle_Config>>,
}

/// Bucket pool sub-struct for types in the `fr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsFr {
    #[serde(default)]
    pub friendly_fire_reaction_override: Vec<Option<FriendlyFireReactionOverride>>,
    #[serde(default)]
    pub friend_manager_notifications_params: Vec<Option<FriendManagerNotificationsParams>>,
    #[serde(default)]
    pub friend_manager_global_params: Vec<Option<FriendManagerGlobalParams>>,
    #[serde(default)]
    pub frontend_override_params: Vec<Option<FrontendOverrideParams>>,
    #[serde(default)]
    pub fragment_info: Vec<Option<FragmentInfo>>,
    #[serde(default)]
    pub fragment_required_info: Vec<Option<FragmentRequiredInfo>>,
}

/// Bucket pool sub-struct for types in the `ga` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsGa {
    #[serde(default)]
    pub game_module: Vec<Option<GameModule>>,
    #[serde(default)]
    pub game_mode_valid_map: Vec<Option<GameModeValidMap>>,
    #[serde(default)]
    pub game_mode_custom_setting: Vec<Option<GameModeCustomSetting>>,
    #[serde(default)]
    pub game_mode_filter: Vec<Option<GameModeFilter>>,
    #[serde(default)]
    pub game_mode: Vec<Option<GameMode>>,
    #[serde(default)]
    pub game_difficulty_modifiers: Vec<Option<GameDifficultyModifiers>>,
    #[serde(default)]
    pub game_notification_dock_item_params: Vec<Option<GameNotificationDockItemParams>>,
    #[serde(default)]
    pub gas_params: Vec<Option<GasParams>>,
}

/// Bucket pool sub-struct for types in the `ge` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsGe {
    #[serde(default)]
    pub geometry_transform_params: Vec<Option<GeometryTransformParams>>,
    #[serde(default)]
    pub geom_font_letter_node: Vec<Option<GeomFont_LetterNode>>,
    #[serde(default)]
    pub geom_font_config: Vec<Option<GeomFont_Config>>,
}

/// Bucket pool sub-struct for types in the `gf` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsGf {
    #[serde(default)]
    pub gforce_params: Vec<Option<GForceParams>>,
}

/// Bucket pool sub-struct for types in the `gl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsGl {
    #[serde(default)]
    pub global_engine_trails_setting: Vec<Option<GlobalEngineTrailsSetting>>,
    #[serde(default)]
    pub global_atmospheric_heating_settings: Vec<Option<GlobalAtmosphericHeatingSettings>>,
    #[serde(default)]
    pub global_aerodynamic_trail_settings: Vec<Option<GlobalAerodynamicTrailSettings>>,
    #[serde(default)]
    pub global_environment_effect_settings: Vec<Option<GlobalEnvironmentEffectSettings>>,
    #[serde(default)]
    pub global_audio_settings: Vec<Option<GlobalAudioSettings>>,
    #[serde(default)]
    pub global_gas_cloud_vdb_gameplay_params: Vec<Option<GlobalGasCloudVDB_GameplayParams>>,
    #[serde(default)]
    pub global_gas_cloud_vdbparams: Vec<Option<GlobalGasCloudVDBParams>>,
    #[serde(default)]
    pub global_shop_commodity_params: Vec<Option<GlobalShopCommodityParams>>,
    #[serde(default)]
    pub global_shop_terminal_params: Vec<Option<GlobalShopTerminalParams>>,
    #[serde(default)]
    pub global_shop_selling_params: Vec<Option<GlobalShopSellingParams>>,
    #[serde(default)]
    pub global_shop_buying_params: Vec<Option<GlobalShopBuyingParams>>,
    #[serde(default)]
    pub global_jump_point_tuning_params: Vec<Option<GlobalJumpPointTuningParams>>,
    #[serde(default)]
    pub global_jump_point_opening_params: Vec<Option<GlobalJumpPointOpeningParams>>,
    #[serde(default)]
    pub global_jump_point_closing_params: Vec<Option<GlobalJumpPointClosingParams>>,
    #[serde(default)]
    pub global_jump_point_effect_params: Vec<Option<GlobalJumpPointEffectParams>>,
    #[serde(default)]
    pub global_jump_point_params: Vec<Option<GlobalJumpPointParams>>,
    #[serde(default)]
    pub global_jump_tunnel_light_params: Vec<Option<GlobalJumpTunnelLightParams>>,
    #[serde(default)]
    pub global_jump_tunnel_fog_params: Vec<Option<GlobalJumpTunnelFogParams>>,
    #[serde(default)]
    pub global_jump_tunnel_pass_by_light_params: Vec<Option<GlobalJumpTunnelPassByLightParams>>,
    #[serde(default)]
    pub global_jump_tunnel_probe_params: Vec<Option<GlobalJumpTunnelProbeParams>>,
    #[serde(default)]
    pub global_jump_tunnel_effect_params: Vec<Option<GlobalJumpTunnelEffectParams>>,
    #[serde(default)]
    pub global_jump_tunnel_camera_effect_params: Vec<Option<GlobalJumpTunnelCameraEffectParams>>,
    #[serde(default)]
    pub global_jump_tunnel_host_params: Vec<Option<GlobalJumpTunnelHostParams>>,
    #[serde(default)]
    pub global_jump_drive_tuning_effect_params: Vec<Option<GlobalJumpDriveTuningEffectParams>>,
    #[serde(default)]
    pub global_jump_drive_entry_effect_params: Vec<Option<GlobalJumpDriveEntryEffectParams>>,
    #[serde(default)]
    pub global_jump_drive_exit_effect_params: Vec<Option<GlobalJumpDriveExitEffectParams>>,
    #[serde(default)]
    pub global_jump_drive_effect_params: Vec<Option<GlobalJumpDriveEffectParams>>,
    #[serde(default)]
    pub global_jump_drive_params: Vec<Option<GlobalJumpDriveParams>>,
    #[serde(default)]
    pub global_cargo_loading_params: Vec<Option<GlobalCargoLoadingParams>>,
    #[serde(default)]
    pub global_marker_configs: Vec<Option<GlobalMarkerConfigs>>,
    #[serde(default)]
    pub global_mission_settings: Vec<Option<GlobalMissionSettings>>,
    #[serde(default)]
    pub global_fog_volume: Vec<Option<GlobalFogVolume>>,
    #[serde(default)]
    pub global_resource_cgf: Vec<Option<GlobalResourceCGF>>,
    #[serde(default)]
    pub global_resource_geometry: Vec<Option<GlobalResourceGeometry>>,
    #[serde(default)]
    pub global_resource_particle: Vec<Option<GlobalResourceParticle>>,
    #[serde(default)]
    pub global_resource_material: Vec<Option<GlobalResourceMaterial>>,
    #[serde(default)]
    pub global_resource_audio: Vec<Option<GlobalResourceAudio>>,
    #[serde(default)]
    pub global_gas_params: Vec<Option<GlobalGasParams>>,
    #[serde(default)]
    pub global_room_state_params: Vec<Option<GlobalRoomStateParams>>,
    #[serde(default)]
    pub global_tutorial_params: Vec<Option<GlobalTutorialParams>>,
}

/// Bucket pool sub-struct for types in the `gp` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsGp {
    #[serde(default)]
    pub gpuparticle_audio: Vec<Option<GPUParticleAudio>>,
    #[serde(default)]
    pub gpuparticle_audio_list: Vec<Option<GPUParticleAudioList>>,
}

/// Bucket pool sub-struct for types in the `gr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsGr {
    #[serde(default)]
    pub grab_camera_control_params: Vec<Option<GrabCameraControlParams>>,
    #[serde(default)]
    pub grip: Vec<Option<Grip>>,
}

/// Bucket pool sub-struct for types in the `ha` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsHa {
    #[serde(default)]
    pub handhold_grip_type: Vec<Option<HandholdGripType>>,
    #[serde(default)]
    pub handhold_grip_database: Vec<Option<HandholdGripDatabase>>,
    #[serde(default)]
    pub harvestable_tag_list_base: Vec<Option<HarvestableTagListBase>>,
    #[serde(default)]
    pub harvestable_transform_params: Vec<Option<HarvestableTransformParams>>,
    #[serde(default)]
    pub harvest_condition_base: Vec<Option<HarvestConditionBase>>,
    #[serde(default)]
    pub harvest_despawn_timer_params: Vec<Option<HarvestDespawnTimerParams>>,
    #[serde(default)]
    pub harvest_behaviour_params: Vec<Option<HarvestBehaviourParams>>,
    #[serde(default)]
    pub harvestable_preset: Vec<Option<HarvestablePreset>>,
    #[serde(default)]
    pub harvestable_setup: Vec<Option<HarvestableSetup>>,
    #[serde(default)]
    pub harvestable_cluster_params: Vec<Option<HarvestableClusterParams>>,
    #[serde(default)]
    pub harvestable_cluster_preset: Vec<Option<HarvestableClusterPreset>>,
    #[serde(default)]
    pub harvestable_geometry: Vec<Option<HarvestableGeometry>>,
    #[serde(default)]
    pub harvestable_element: Vec<Option<HarvestableElement>>,
    #[serde(default)]
    pub harvestable_element_group: Vec<Option<HarvestableElementGroup>>,
    #[serde(default)]
    pub harvestable_element_modifier: Vec<Option<HarvestableElementModifier>>,
    #[serde(default)]
    pub harvestable_area_type_base: Vec<Option<HarvestableAreaTypeBase>>,
    #[serde(default)]
    pub harvestable_area_preset: Vec<Option<HarvestableAreaPreset>>,
    #[serde(default)]
    pub harvestable_provider_preset: Vec<Option<HarvestableProviderPreset>>,
    #[serde(default)]
    pub hardware_mouse_params: Vec<Option<HardwareMouseParams>>,
    #[serde(default)]
    pub hauling_entity_class_list_base: Vec<Option<Hauling_EntityClassListBase>>,
    #[serde(default)]
    pub hauling_entity_classes: Vec<Option<Hauling_EntityClasses>>,
    #[serde(default)]
    pub hazard_awareness_params: Vec<Option<HazardAwarenessParams>>,
}

/// Bucket pool sub-struct for types in the `he` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsHe {
    #[serde(default)]
    pub health_icon_data: Vec<Option<HealthIconData>>,
    #[serde(default)]
    pub health_icon_status_effect: Vec<Option<HealthIconStatusEffect>>,
    #[serde(default)]
    pub health_template: Vec<Option<HealthTemplate>>,
}

/// Bucket pool sub-struct for types in the `hi` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsHi {
    #[serde(default)]
    pub hit_consistency_params: Vec<Option<HitConsistencyParams>>,
    #[serde(default)]
    pub hint_uidata: Vec<Option<HintUIData>>,
    #[serde(default)]
    pub hint_trigger_data: Vec<Option<HintTriggerData>>,
}

/// Bucket pool sub-struct for types in the `ho` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsHo {
    #[serde(default)]
    pub hold_exhale_duration: Vec<Option<HoldExhaleDuration>>,
    #[serde(default)]
    pub hologram_params: Vec<Option<HologramParams>>,
}

/// Bucket pool sub-struct for types in the `hu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsHu {
    #[serde(default)]
    pub hud_feedback_params: Vec<Option<HudFeedbackParams>>,
    #[serde(default)]
    pub hudsilhouette_params: Vec<Option<HUDSilhouetteParams>>,
    #[serde(default)]
    pub hud_colors: Vec<Option<HudColors>>,
    #[serde(default)]
    pub hud_color_palette: Vec<Option<HudColor_Palette>>,
    #[serde(default)]
    pub hud_color_entry: Vec<Option<HudColor_Entry>>,
    #[serde(default)]
    pub hud_color_custom_entry: Vec<Option<HudColor_CustomEntry>>,
    #[serde(default)]
    pub hud_color_holo_param: Vec<Option<HudColor_HoloParam>>,
    #[serde(default)]
    pub hud_color_holo_mat_colors: Vec<Option<HudColor_HoloMatColors>>,
    #[serde(default)]
    pub hud_color_holo_mat_textures: Vec<Option<HudColor_HoloMatTextures>>,
}

/// Bucket pool sub-struct for types in the `hy` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsHy {
    #[serde(default)]
    pub hygiene_params: Vec<Option<HygieneParams>>,
}

/// Bucket pool sub-struct for types in the `if` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsIf {
    #[serde(default)]
    pub ifcs_input_deflection_time_params: Vec<Option<IfcsInputDeflectionTimeParams>>,
}

/// Bucket pool sub-struct for types in the `im` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsIm {
    #[serde(default)]
    pub imannequin_action_def: Vec<Option<IMannequinActionDef>>,
    #[serde(default)]
    pub impact_force_resistance: Vec<Option<ImpactForceResistance>>,
    #[serde(default)]
    pub impounding_definition: Vec<Option<ImpoundingDefinition>>,
}

/// Bucket pool sub-struct for types in the `in` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsIn {
    #[serde(default)]
    pub intoxication_ifcsmodifier_params: Vec<Option<IntoxicationIFCSModifierParams>>,
    #[serde(default)]
    pub intoxication_turret_modifier_params: Vec<Option<IntoxicationTurretModifierParams>>,
    #[serde(default)]
    pub intoxication_wheeled_modifier_params: Vec<Option<IntoxicationWheeledModifierParams>>,
    #[serde(default)]
    pub input_action: Vec<Option<InputAction>>,
    #[serde(default)]
    pub initial_damage_override: Vec<Option<InitialDamageOverride>>,
    #[serde(default)]
    pub inner_thought_anim_base: Vec<Option<InnerThought_AnimBase>>,
    #[serde(default)]
    pub inner_thought_anim: Vec<Option<InnerThought_Anim>>,
    #[serde(default)]
    pub inner_thought_layout_base: Vec<Option<InnerThought_LayoutBase>>,
    #[serde(default)]
    pub inner_thought_color_params: Vec<Option<InnerThought_ColorParams>>,
    #[serde(default)]
    pub inner_thought_config: Vec<Option<InnerThought_Config>>,
    #[serde(default)]
    pub inner_thought_layout_states: Vec<Option<InnerThought_LayoutStates>>,
    #[serde(default)]
    pub inner_thought_params: Vec<Option<InnerThought_Params>>,
    #[serde(default)]
    pub inner_thought_conversation_system_config: Vec<Option<InnerThought_ConversationSystemConfig>>,
    #[serde(default)]
    pub inner_thought_interaction_system_config: Vec<Option<InnerThought_InteractionSystemConfig>>,
    #[serde(default)]
    pub inner_thought_legacy_use_system_config: Vec<Option<InnerThought_LegacyUseSystemConfig>>,
    #[serde(default)]
    pub input_prompt_config: Vec<Option<InputPromptConfig>>,
    #[serde(default)]
    pub instanced_interior_location_params: Vec<Option<InstancedInteriorLocationParams>>,
    #[serde(default)]
    pub instanced_interior_location_map: Vec<Option<InstancedInteriorLocationMap>>,
    #[serde(default)]
    pub interaction_condition_params: Vec<Option<InteractionConditionParams>>,
    #[serde(default)]
    pub interaction_condition_preset: Vec<Option<InteractionConditionPreset>>,
    #[serde(default)]
    pub interaction_condition_list: Vec<Option<InteractionConditionList>>,
    #[serde(default)]
    pub interaction_point_template: Vec<Option<InteractionPointTemplate>>,
    #[serde(default)]
    pub inventory_container_type_base: Vec<Option<InventoryContainerTypeBase>>,
    #[serde(default)]
    pub inventory_container_item_type_filter: Vec<Option<InventoryContainerItemTypeFilter>>,
    #[serde(default)]
    pub inventory_container_manager: Vec<Option<InventoryContainerManager>>,
    #[serde(default)]
    pub inventory_container: Vec<Option<InventoryContainer>>,
    #[serde(default)]
    pub inventory_location: Vec<Option<InventoryLocation>>,
    #[serde(default)]
    pub infraction_parameters: Vec<Option<InfractionParameters>>,
    #[serde(default)]
    pub infraction_definition: Vec<Option<InfractionDefinition>>,
    #[serde(default)]
    pub infraction: Vec<Option<Infraction>>,
    #[serde(default)]
    pub interior_map_section_definition: Vec<Option<InteriorMapSectionDefinition>>,
    #[serde(default)]
    pub inventory_sort_mode: Vec<Option<InventorySortMode>>,
    #[serde(default)]
    pub inventory_container_params: Vec<Option<InventoryContainerParams>>,
    #[serde(default)]
    pub inventory_drop_detach_rules: Vec<Option<InventoryDropDetachRules>>,
}

/// Bucket pool sub-struct for types in the `it` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsIt {
    #[serde(default)]
    pub item_carry_params: Vec<Option<ItemCarryParams>>,
    #[serde(default)]
    pub item_type_modifier: Vec<Option<ItemTypeModifier>>,
    #[serde(default)]
    pub item: Vec<Option<Item>>,
    #[serde(default)]
    pub item_kiosk_brand: Vec<Option<ItemKioskBrand>>,
    #[serde(default)]
    pub item_port_tags_element: Vec<Option<ItemPortTagsElement>>,
    #[serde(default)]
    pub item_port_tags_dictionary: Vec<Option<ItemPortTagsDictionary>>,
    #[serde(default)]
    pub item_resource_type_data: Vec<Option<ItemResourceTypeData>>,
    #[serde(default)]
    pub item_resource_composition_map: Vec<Option<ItemResourceCompositionMap>>,
    #[serde(default)]
    pub item_room_resource_pair: Vec<Option<ItemRoomResourcePair>>,
    #[serde(default)]
    pub item_resource_network_map_trigger_entry: Vec<Option<ItemResourceNetworkMapTriggerEntry>>,
    #[serde(default)]
    pub item_resource_network_type_uidata: Vec<Option<ItemResourceNetworkTypeUIData>>,
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
    pub item_award_base: Vec<Option<ItemAwardBase>>,
    #[serde(default)]
    pub item_award_weightings: Vec<Option<ItemAwardWeightings>>,
    #[serde(default)]
    pub item_award_weightings_record: Vec<Option<ItemAwardWeightingsRecord>>,
    #[serde(default)]
    pub item_recovery_configuration_params: Vec<Option<ItemRecoveryConfigurationParams>>,
    #[serde(default)]
    pub item_recovery_set_condition_def: Vec<Option<ItemRecoverySetConditionDef>>,
    #[serde(default)]
    pub item_recovery_notification_params: Vec<Option<ItemRecoveryNotificationParams>>,
    #[serde(default)]
    pub item_recovery_economy_params: Vec<Option<ItemRecoveryEconomyParams>>,
    #[serde(default)]
    pub item_category: Vec<Option<ItemCategory>>,
    #[serde(default)]
    pub item_preview_light_intensities: Vec<Option<ItemPreview_LightIntensities>>,
    #[serde(default)]
    pub item_preview_lighting_settings: Vec<Option<ItemPreview_LightingSettings>>,
    #[serde(default)]
    pub item_preview_skinned_loadout_override: Vec<Option<ItemPreview_SkinnedLoadoutOverride>>,
    #[serde(default)]
    pub item_preview_camera_settings: Vec<Option<ItemPreview_CameraSettings>>,
    #[serde(default)]
    pub item_preview_camera_settings_override: Vec<Option<ItemPreview_CameraSettingsOverride>>,
    #[serde(default)]
    pub item_preview_turntable_settings: Vec<Option<ItemPreview_TurntableSettings>>,
    #[serde(default)]
    pub item_preview_turntable_override: Vec<Option<ItemPreview_TurntableOverride>>,
    #[serde(default)]
    pub item_preview_config: Vec<Option<ItemPreview_Config>>,
    #[serde(default)]
    pub item_type_category: Vec<Option<ItemTypeCategory>>,
    #[serde(default)]
    pub item_type_category_exception: Vec<Option<ItemTypeCategoryException>>,
    #[serde(default)]
    pub item_type_category_map: Vec<Option<ItemTypeCategoryMap>>,
    #[serde(default)]
    pub item_type_info: Vec<Option<ItemTypeInfo>>,
    #[serde(default)]
    pub item_type_definition: Vec<Option<ItemTypeDefinition>>,
    #[serde(default)]
    pub item_port_view_information: Vec<Option<ItemPortViewInformation>>,
}

/// Bucket pool sub-struct for types in the `jo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsJo {
    #[serde(default)]
    pub journal_entry_type: Vec<Option<JournalEntryType>>,
    #[serde(default)]
    pub journal_entry: Vec<Option<JournalEntry>>,
}

/// Bucket pool sub-struct for types in the `ju` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsJu {
    #[serde(default)]
    pub jump_point_vibration_params: Vec<Option<JumpPointVibrationParams>>,
    #[serde(default)]
    pub jump_drive_vibration_params: Vec<Option<JumpDriveVibrationParams>>,
    #[serde(default)]
    pub jump_tunnel_vibration_params: Vec<Option<JumpTunnelVibrationParams>>,
    #[serde(default)]
    pub jump_system_vibration_params: Vec<Option<JumpSystemVibrationParams>>,
    #[serde(default)]
    pub jump_fall_land_config: Vec<Option<JumpFallLandConfig>>,
    #[serde(default)]
    pub jump_drive_uicone_params: Vec<Option<JumpDriveUIConeParams>>,
    #[serde(default)]
    pub jump_tunnel_camera_effect_param: Vec<Option<JumpTunnelCameraEffectParam>>,
    #[serde(default)]
    pub jump_tunnel_camera_effects: Vec<Option<JumpTunnelCameraEffects>>,
    #[serde(default)]
    pub jump_drive_velocity_strength_params: Vec<Option<JumpDriveVelocityStrengthParams>>,
    #[serde(default)]
    pub jump_drive_flight_params: Vec<Option<JumpDriveFlightParams>>,
    #[serde(default)]
    pub jump_tunnel_forces_params: Vec<Option<JumpTunnelForcesParams>>,
    #[serde(default)]
    pub jump_drive_state_audio_map: Vec<Option<JumpDriveStateAudioMap>>,
    #[serde(default)]
    pub jump_drive_audio_movement_params: Vec<Option<JumpDriveAudioMovementParams>>,
    #[serde(default)]
    pub jump_drive_audio_params: Vec<Option<JumpDriveAudioParams>>,
    #[serde(default)]
    pub jump_drive_music_event: Vec<Option<JumpDriveMusicEvent>>,
    #[serde(default)]
    pub jump_drive_state_music_map: Vec<Option<JumpDriveStateMusicMap>>,
    #[serde(default)]
    pub jump_drive_music_params: Vec<Option<JumpDriveMusicParams>>,
    #[serde(default)]
    pub jump_drive_approach_rings_params: Vec<Option<JumpDriveApproachRingsParams>>,
    #[serde(default)]
    pub jump_travel_camera_params: Vec<Option<JumpTravelCameraParams>>,
    #[serde(default)]
    pub jurisdiction: Vec<Option<Jurisdiction>>,
    #[serde(default)]
    pub jump_fall_land_params: Vec<Option<JumpFallLandParams>>,
    #[serde(default)]
    pub jump_thruster_pack_config: Vec<Option<JumpThrusterPackConfig>>,
}

/// Bucket pool sub-struct for types in the `la` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsLa {
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
    pub landing_zone_inventory_redirect: Vec<Option<LandingZoneInventoryRedirect>>,
    #[serde(default)]
    pub landing_zone_inventory: Vec<Option<LandingZoneInventory>>,
    #[serde(default)]
    pub landing_selection: Vec<Option<LandingSelection>>,
    #[serde(default)]
    pub landing_animation_setup: Vec<Option<LandingAnimationSetup>>,
    #[serde(default)]
    pub landing_pad_size: Vec<Option<LandingPadSize>>,
}

/// Bucket pool sub-struct for types in the `le` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsLe {
    #[serde(default)]
    pub lean_connection: Vec<Option<LeanConnection>>,
    #[serde(default)]
    pub lean_state: Vec<Option<LeanState>>,
    #[serde(default)]
    pub lean_graph: Vec<Option<LeanGraph>>,
    #[serde(default)]
    pub legacy_crafting_recipe_base: Vec<Option<LegacyCraftingRecipe_Base>>,
    #[serde(default)]
    pub legacy_crafting_recipe_def_base: Vec<Option<LegacyCraftingRecipeDef_Base>>,
    #[serde(default)]
    pub legacy_crafting_recipe_def_record: Vec<Option<LegacyCraftingRecipeDefRecord>>,
    #[serde(default)]
    pub legacy_crafting_recipe_list_record: Vec<Option<LegacyCraftingRecipeListRecord>>,
    #[serde(default)]
    pub level: Vec<Option<Level>>,
    #[serde(default)]
    pub ledge_nearby_params: Vec<Option<LedgeNearbyParams>>,
    #[serde(default)]
    pub ledge_transition_params: Vec<Option<LedgeTransitionParams>>,
    #[serde(default)]
    pub ledge_grabbing_params: Vec<Option<LedgeGrabbingParams>>,
}

/// Bucket pool sub-struct for types in the `li` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsLi {
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
    pub licensed_item_modifier: Vec<Option<LicensedItemModifier>>,
    #[serde(default)]
    pub lightning_behavior: Vec<Option<LightningBehavior>>,
    #[serde(default)]
    pub lightning_behavior_effect: Vec<Option<LightningBehavior_Effect>>,
    #[serde(default)]
    pub lightning_target_mode: Vec<Option<LightningTargetMode>>,
    #[serde(default)]
    pub lightning_strike_audio: Vec<Option<LightningStrikeAudio>>,
}

/// Bucket pool sub-struct for types in the `lo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsLo {
    #[serde(default)]
    pub locomotion_anim_sync_config: Vec<Option<LocomotionAnimSyncConfig>>,
    #[serde(default)]
    pub local_player_speed_throttle_component: Vec<Option<LocalPlayerSpeedThrottleComponent>>,
    #[serde(default)]
    pub long_term_persistence_sub_type_list_option: Vec<Option<LongTermPersistenceSubTypeListOption>>,
    #[serde(default)]
    pub long_term_persistence_white_list_entry: Vec<Option<LongTermPersistenceWhiteListEntry>>,
    #[serde(default)]
    pub long_term_persistence_global_params: Vec<Option<LongTermPersistenceGlobalParams>>,
    #[serde(default)]
    pub loot_archetype_entry_primary: Vec<Option<LootArchetypeEntry_Primary>>,
    #[serde(default)]
    pub loot_archetype_entry_secondary: Vec<Option<LootArchetypeEntry_Secondary>>,
    #[serde(default)]
    pub loot_archetype_or_group_primary: Vec<Option<LootArchetypeOrGroup_Primary>>,
    #[serde(default)]
    pub loot_archetype_or_group_secondary: Vec<Option<LootArchetypeOrGroup_Secondary>>,
    #[serde(default)]
    pub loot_archetype: Vec<Option<LootArchetype>>,
    #[serde(default)]
    pub loot_table: Vec<Option<LootTable>>,
    #[serde(default)]
    pub loot_constraints: Vec<Option<LootConstraints>>,
    #[serde(default)]
    pub loot_config: Vec<Option<LootConfig>>,
    #[serde(default)]
    pub loot_generation_special_event_archetype: Vec<Option<LootGenerationSpecialEventArchetype>>,
    #[serde(default)]
    pub loot_table_v3_no_ref: Vec<Option<LootTableV3_NoRef>>,
    #[serde(default)]
    pub loot_table_v3_record: Vec<Option<LootTableV3Record>>,
    #[serde(default)]
    pub loot_archetype_v3_base: Vec<Option<LootArchetypeV3_Base>>,
    #[serde(default)]
    pub loot_archetype_v3_no_ref: Vec<Option<LootArchetypeV3_NoRef>>,
    #[serde(default)]
    pub loot_archetype_v3_record: Vec<Option<LootArchetypeV3Record>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_record_ref_base: Vec<Option<LootV3SecondaryChoicesRecordRef_Base>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_single_layer_record: Vec<Option<LootV3SecondaryChoicesSingleLayerRecord>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_multi_layer_record: Vec<Option<LootV3SecondaryChoicesMultiLayerRecord>>,
    #[serde(default)]
    pub loot_v3_secondary_choice_entry_selector_base: Vec<Option<LootV3SecondaryChoiceEntrySelector_Base>>,
    #[serde(default)]
    pub loot_v3_secondary_choice_entry: Vec<Option<LootV3SecondaryChoiceEntry>>,
    #[serde(default)]
    pub loot_generation_global_params: Vec<Option<LootGenerationGlobalParams>>,
    #[serde(default)]
    pub location_mission_limit: Vec<Option<LocationMissionLimit>>,
    #[serde(default)]
    pub location_resource_slot: Vec<Option<LocationResourceSlot>>,
    #[serde(default)]
    pub location_entity_type_base: Vec<Option<LocationEntityType_Base>>,
    #[serde(default)]
    pub location_entity_declaration: Vec<Option<LocationEntityDeclaration>>,
    #[serde(default)]
    pub location_music_config: Vec<Option<LocationMusicConfig>>,
    #[serde(default)]
    pub looting_tab_params: Vec<Option<LootingTabParams>>,
    #[serde(default)]
    pub looting_inventory_params: Vec<Option<LootingInventoryParams>>,
    #[serde(default)]
    pub looting_item_port_size_class: Vec<Option<LootingItemPortSizeClass>>,
    #[serde(default)]
    pub loadout_dummy_component_params: Vec<Option<LoadoutDummyComponentParams>>,
    #[serde(default)]
    pub loadout_dummy_transform_params: Vec<Option<LoadoutDummyTransformParams>>,
    #[serde(default)]
    pub loadout_item_preview_transform_params: Vec<Option<LoadoutItemPreviewTransformParams>>,
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
    pub loadout_editor_component_params: Vec<Option<LoadoutEditorComponentParams>>,
    #[serde(default)]
    pub loadout_kit: Vec<Option<LoadoutKit>>,
}

/// Bucket pool sub-struct for types in the `ma` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsMa {
    #[serde(default)]
    pub master_mode_exclusion: Vec<Option<MasterModeExclusion>>,
    #[serde(default)]
    pub master_mode_exclusion_global_params: Vec<Option<MasterModeExclusionGlobalParams>>,
    #[serde(default)]
    pub material_effect_entry: Vec<Option<MaterialEffectEntry>>,
    #[serde(default)]
    pub marker_tracking_view_mode_parameters: Vec<Option<MarkerTrackingViewModeParameters>>,
    #[serde(default)]
    pub marker_tracking_common_map_parameters: Vec<Option<MarkerTrackingCommonMapParameters>>,
    #[serde(default)]
    pub marker_tracking_action_parameters: Vec<Option<MarkerTrackingActionParameters>>,
    #[serde(default)]
    pub marker_tracking_display_parameters: Vec<Option<MarkerTrackingDisplayParameters>>,
    #[serde(default)]
    pub marker_tracking_label_parameters: Vec<Option<MarkerTrackingLabelParameters>>,
    #[serde(default)]
    pub marker_decluttering_culling_order: Vec<Option<MarkerDeclutteringCullingOrder>>,
    #[serde(default)]
    pub marker_configuration: Vec<Option<Marker_Configuration>>,
    #[serde(default)]
    pub marker_show_rule: Vec<Option<Marker_ShowRule>>,
    #[serde(default)]
    pub marker_show_rule_map_display_mode: Vec<Option<Marker_ShowRuleMapDisplayMode>>,
    #[serde(default)]
    pub marker_ability_base: Vec<Option<Marker_AbilityBase>>,
    #[serde(default)]
    pub master_mode_switch_delta_signature_types: Vec<Option<MasterModeSwitchDeltaSignatureTypes>>,
    #[serde(default)]
    pub marker_ar_config_def: Vec<Option<MarkerAR_ConfigDef>>,
}

/// Bucket pool sub-struct for types in the `me` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsMe {
    #[serde(default)]
    pub med_bed_tier_params: Vec<Option<MedBedTierParams>>,
    #[serde(default)]
    pub med_bed_resource_consumption_params: Vec<Option<MedBedResourceConsumptionParams>>,
    #[serde(default)]
    pub medical_item_tier_config: Vec<Option<MedicalItemTierConfig>>,
    #[serde(default)]
    pub mega_map: Vec<Option<MegaMap>>,
    #[serde(default)]
    pub melee_frag_info: Vec<Option<MeleeFragInfo>>,
    #[serde(default)]
    pub melee_attack_category_info: Vec<Option<MeleeAttackCategoryInfo>>,
    #[serde(default)]
    pub melee_attack_info: Vec<Option<MeleeAttackInfo>>,
    #[serde(default)]
    pub melee_combat_config: Vec<Option<MeleeCombatConfig>>,
    #[serde(default)]
    pub melee_combo_chain_link: Vec<Option<MeleeComboChainLink>>,
    #[serde(default)]
    pub melee_attack_combo: Vec<Option<MeleeAttackCombo>>,
}

/// Bucket pool sub-struct for types in the `mi` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsMi {
    #[serde(default)]
    pub misted_breath_params: Vec<Option<MistedBreathParams>>,
    #[serde(default)]
    pub mineable_explosion_params: Vec<Option<MineableExplosionParams>>,
    #[serde(default)]
    pub mineable_instability_params: Vec<Option<MineableInstabilityParams>>,
    #[serde(default)]
    pub mining_global_params: Vec<Option<MiningGlobalParams>>,
    #[serde(default)]
    pub mining_audio_params: Vec<Option<MiningAudioParams>>,
    #[serde(default)]
    pub mineable_element: Vec<Option<MineableElement>>,
    #[serde(default)]
    pub mineable_composition_part: Vec<Option<MineableCompositionPart>>,
    #[serde(default)]
    pub mineable_composition: Vec<Option<MineableComposition>>,
    #[serde(default)]
    pub mining_laser_global_params: Vec<Option<MiningLaserGlobalParams>>,
    #[serde(default)]
    pub mission_location_tags: Vec<Option<MissionLocationTags>>,
    #[serde(default)]
    pub mission_string_variant: Vec<Option<MissionStringVariant>>,
    #[serde(default)]
    pub mission_string_variants: Vec<Option<MissionStringVariants>>,
    #[serde(default)]
    pub mission_location_data: Vec<Option<MissionLocationData>>,
    #[serde(default)]
    pub mission_location_template: Vec<Option<MissionLocationTemplate>>,
    #[serde(default)]
    pub mission_location_validation: Vec<Option<MissionLocationValidation>>,
    #[serde(default)]
    pub mission_item: Vec<Option<MissionItem>>,
    #[serde(default)]
    pub mission_organization: Vec<Option<MissionOrganization>>,
    #[serde(default)]
    pub mission_fail_condition_params: Vec<Option<MissionFailConditionParams>>,
    #[serde(default)]
    pub mission_fail_conditions_list: Vec<Option<MissionFailConditionsList>>,
    #[serde(default)]
    pub mission_flow_condition_base: Vec<Option<MissionFlowConditionBase>>,
    #[serde(default)]
    pub mission_flow_action_base: Vec<Option<MissionFlowActionBase>>,
    #[serde(default)]
    pub mission_flow_trigger: Vec<Option<MissionFlowTrigger>>,
    #[serde(default)]
    pub mission_flow: Vec<Option<MissionFlow>>,
    #[serde(default)]
    pub mission_property_value_ainame: Vec<Option<MissionPropertyValue_AIName>>,
    #[serde(default)]
    pub mission_property: Vec<Option<MissionProperty>>,
    #[serde(default)]
    pub mission_variable_base: Vec<Option<MissionVariableBase>>,
    #[serde(default)]
    pub mission_module_hierarchy_sub_mission: Vec<Option<MissionModuleHierarchySubMission>>,
    #[serde(default)]
    pub mission_module_hierarchy: Vec<Option<MissionModuleHierarchy>>,
    #[serde(default)]
    pub mission_scenario_cycle_phase: Vec<Option<MissionScenarioCyclePhase>>,
    #[serde(default)]
    pub mission_scenario_cycle: Vec<Option<MissionScenarioCycle>>,
    #[serde(default)]
    pub mission_scenario_schedule_constraint: Vec<Option<MissionScenarioScheduleConstraint>>,
    #[serde(default)]
    pub mission_scenario_schedule_recurrence: Vec<Option<MissionScenarioScheduleRecurrence>>,
    #[serde(default)]
    pub mission_scenario_schedule: Vec<Option<MissionScenarioSchedule>>,
    #[serde(default)]
    pub mission_scenario: Vec<Option<MissionScenario>>,
    #[serde(default)]
    pub mission_type: Vec<Option<MissionType>>,
    #[serde(default)]
    pub mission_deadline: Vec<Option<MissionDeadline>>,
    #[serde(default)]
    pub mission_reward: Vec<Option<MissionReward>>,
    #[serde(default)]
    pub mission_locality: Vec<Option<MissionLocality>>,
    #[serde(default)]
    pub min_required_missions: Vec<Option<MinRequiredMissions>>,
    #[serde(default)]
    pub mission_broker_entry: Vec<Option<MissionBrokerEntry>>,
    #[serde(default)]
    pub mission_giver: Vec<Option<MissionGiver>>,
    #[serde(default)]
    pub mission_complete_perk_base_def: Vec<Option<MissionCompletePerkBaseDef>>,
    #[serde(default)]
    pub mining_camera_shake_config: Vec<Option<MiningCameraShakeConfig>>,
    #[serde(default)]
    pub mining_controller_global_params: Vec<Option<MiningControllerGlobalParams>>,
    #[serde(default)]
    pub missile_lock_reticle_segment_def: Vec<Option<MissileLockReticleSegmentDef>>,
    #[serde(default)]
    pub missile_lock_reticle_config: Vec<Option<MissileLockReticle_Config>>,
}

/// Bucket pool sub-struct for types in the `mo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsMo {
    #[serde(default)]
    pub movement_system_additional_params: Vec<Option<MovementSystemAdditionalParams>>,
    #[serde(default)]
    pub movement_system_additional_params_record: Vec<Option<MovementSystemAdditionalParamsRecord>>,
    #[serde(default)]
    pub motion_connection: Vec<Option<MotionConnection>>,
    #[serde(default)]
    pub motion_state: Vec<Option<MotionState>>,
    #[serde(default)]
    pub motion_smoothing_params: Vec<Option<MotionSmoothingParams>>,
    #[serde(default)]
    pub motion_juke_params: Vec<Option<MotionJukeParams>>,
    #[serde(default)]
    pub motion_turn_params: Vec<Option<MotionTurnParams>>,
    #[serde(default)]
    pub motion_turn_setup_filtered: Vec<Option<MotionTurnSetupFiltered>>,
    #[serde(default)]
    pub motion_turn_setup_list: Vec<Option<MotionTurnSetupList>>,
    #[serde(default)]
    pub motion_foot_pinning_params: Vec<Option<MotionFootPinningParams>>,
    #[serde(default)]
    pub motion_graph: Vec<Option<MotionGraph>>,
    #[serde(default)]
    pub movement_speed_override: Vec<Option<MovementSpeedOverride>>,
    #[serde(default)]
    pub module_declaration_type_base: Vec<Option<ModuleDeclarationType_Base>>,
    #[serde(default)]
    pub module_declaration: Vec<Option<ModuleDeclaration>>,
    #[serde(default)]
    pub mobi_glas_app: Vec<Option<mobiGlasApp>>,
    #[serde(default)]
    pub move_view_restriction_penalty: Vec<Option<MoveViewRestrictionPenalty>>,
    #[serde(default)]
    pub move_view_restriction_weighting: Vec<Option<MoveViewRestrictionWeighting>>,
    #[serde(default)]
    pub movie_clip_transformation_interpolator_params: Vec<Option<MovieClipTransformationInterpolatorParams>>,
    #[serde(default)]
    pub movie_clip_transformation_interpolator: Vec<Option<MovieClipTransformationInterpolator>>,
    #[serde(default)]
    pub mobi_glas_app_data: Vec<Option<MobiGlasAppData>>,
    #[serde(default)]
    pub mobi_glas_app_data_base: Vec<Option<MobiGlasAppDataBase>>,
    #[serde(default)]
    pub mobi_glas_mission_note: Vec<Option<MobiGlasMissionNote>>,
}

/// Bucket pool sub-struct for types in the `mu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsMu {
    #[serde(default)]
    pub music_logic_config: Vec<Option<MusicLogicConfig>>,
    #[serde(default)]
    pub music_logic_parameter: Vec<Option<MusicLogicParameter>>,
    #[serde(default)]
    pub music_logic_event: Vec<Option<MusicLogicEvent>>,
    #[serde(default)]
    pub music_logic_event_list: Vec<Option<MusicLogicEventList>>,
    #[serde(default)]
    pub music_logic_switch_value: Vec<Option<MusicLogicSwitchValue>>,
    #[serde(default)]
    pub music_logic_node: Vec<Option<MusicLogicNode>>,
    #[serde(default)]
    pub music_logic_suite: Vec<Option<MusicLogicSuite>>,
}

/// Bucket pool sub-struct for types in the `no` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsNo {
    #[serde(default)]
    pub notification_def: Vec<Option<NotificationDef>>,
}

/// Bucket pool sub-struct for types in the `np` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsNp {
    #[serde(default)]
    pub npc_breathing_params: Vec<Option<NpcBreathingParams>>,
}

/// Bucket pool sub-struct for types in the `nu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsNu {
    #[serde(default)]
    pub num_results_constraints: Vec<Option<NumResultsConstraints>>,
}

/// Bucket pool sub-struct for types in the `ob` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsOb {
    #[serde(default)]
    pub objective_property_base: Vec<Option<ObjectivePropertyBase>>,
    #[serde(default)]
    pub objective_display_info: Vec<Option<ObjectiveDisplayInfo>>,
    #[serde(default)]
    pub objective_handler_base: Vec<Option<ObjectiveHandlerBase>>,
    #[serde(default)]
    pub objective_reward_contribution_base: Vec<Option<ObjectiveRewardContributionBase>>,
    #[serde(default)]
    pub objective_token: Vec<Option<ObjectiveToken>>,
}

/// Bucket pool sub-struct for types in the `oc` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsOc {
    #[serde(default)]
    pub occlusion_check_shared_params: Vec<Option<OcclusionCheckSharedParams>>,
}

/// Bucket pool sub-struct for types in the `op` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsOp {
    #[serde(default)]
    pub optional_probability: Vec<Option<OptionalProbability>>,
    #[serde(default)]
    pub operator_mode_availability: Vec<Option<OperatorModeAvailability>>,
    #[serde(default)]
    pub operator_mode_availability_params: Vec<Option<OperatorModeAvailabilityParams>>,
    #[serde(default)]
    pub operator_mode_definitions: Vec<Option<OperatorModeDefinitions>>,
    #[serde(default)]
    pub operator_mode_definition_params: Vec<Option<OperatorModeDefinitionParams>>,
    #[serde(default)]
    pub open_inventory_occupant_item_type_properties: Vec<Option<OpenInventoryOccupantItemTypeProperties>>,
}

/// Bucket pool sub-struct for types in the `or` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsOr {
    #[serde(default)]
    pub orifice_blood_params: Vec<Option<OrificeBloodParams>>,
}

/// Bucket pool sub-struct for types in the `pa` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsPa {
    #[serde(default)]
    pub partial_contract_reward_rep_adjustment: Vec<Option<PartialContractRewardRepAdjustment>>,
    #[serde(default)]
    pub partial_contract_reward_range: Vec<Option<PartialContractRewardRange>>,
    #[serde(default)]
    pub partial_contract_rewards: Vec<Option<PartialContractRewards>>,
}

/// Bucket pool sub-struct for types in the `pe` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsPe {
    #[serde(default)]
    pub personal_thought_camera_effects_params: Vec<Option<PersonalThoughtCameraEffectsParams>>,
    #[serde(default)]
    pub personal_inner_thought_action_rule_preset: Vec<Option<PersonalInnerThoughtActionRulePreset>>,
    #[serde(default)]
    pub personal_thought_actions_rules_params: Vec<Option<PersonalThoughtActionsRulesParams>>,
    #[serde(default)]
    pub personal_thought_action_description: Vec<Option<PersonalThoughtActionDescription>>,
    #[serde(default)]
    pub personal_thought_action_descriptions_list: Vec<Option<PersonalThoughtActionDescriptionsList>>,
    #[serde(default)]
    pub personal_thought_hologram_actions_list: Vec<Option<PersonalThoughtHologramActionsList>>,
    #[serde(default)]
    pub personal_thought_option: Vec<Option<PersonalThoughtOption>>,
    #[serde(default)]
    pub personal_thought_category_action: Vec<Option<PersonalThoughtCategoryAction>>,
    #[serde(default)]
    pub personal_thought_inventory_filter: Vec<Option<PersonalThoughtInventoryFilter>>,
    #[serde(default)]
    pub personal_thought_inventory_params: Vec<Option<PersonalThoughtInventoryParams>>,
    #[serde(default)]
    pub personal_thought_inventory_sort_menu: Vec<Option<PersonalThoughtInventorySortMenu>>,
    #[serde(default)]
    pub personal_thought_looting_screen_params: Vec<Option<PersonalThoughtLootingScreenParams>>,
    #[serde(default)]
    pub personal_thought_pop_window_params: Vec<Option<PersonalThoughtPopWindowParams>>,
    #[serde(default)]
    pub personal_thought_inventory_actions_params: Vec<Option<PersonalThoughtInventoryActionsParams>>,
    #[serde(default)]
    pub personal_thought_inventory_grid_params: Vec<Option<PersonalThoughtInventoryGridParams>>,
    #[serde(default)]
    pub personal_thought_inventory_item_orientation_offset: Vec<Option<PersonalThoughtInventoryItemOrientationOffset>>,
    #[serde(default)]
    pub personal_thought_inventory_item_light_group: Vec<Option<PersonalThoughtInventoryItemLightGroup>>,
    #[serde(default)]
    pub personal_thought_inventory_item_uiicon: Vec<Option<PersonalThoughtInventoryItemUIIcon>>,
    #[serde(default)]
    pub personal_thought_contextual_actions_menu: Vec<Option<PersonalThoughtContextualActionsMenu>>,
    #[serde(default)]
    pub personal_thought_game_mode_def: Vec<Option<PersonalThoughtGameModeDef>>,
    #[serde(default)]
    pub personal_thought_contextual_actions_menus_params: Vec<Option<PersonalThoughtContextualActionsMenusParams>>,
    #[serde(default)]
    pub personal_thought_quick_access_wheel: Vec<Option<PersonalThoughtQuickAccessWheel>>,
    #[serde(default)]
    pub personal_thought_quick_access_wheels: Vec<Option<PersonalThoughtQuickAccessWheels>>,
    #[serde(default)]
    pub personal_thought_hologram_animation_params: Vec<Option<PersonalThoughtHologramAnimationParams>>,
    #[serde(default)]
    pub personal_thought_hologram_params: Vec<Option<PersonalThoughtHologramParams>>,
    #[serde(default)]
    pub personal_thought_force_close_action_list: Vec<Option<PersonalThoughtForceCloseActionList>>,
    #[serde(default)]
    pub personal_thought_force_close_actions_params: Vec<Option<PersonalThoughtForceCloseActionsParams>>,
}

/// Bucket pool sub-struct for types in the `pi` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsPi {
    #[serde(default)]
    pub ping_shared_params: Vec<Option<PingSharedParams>>,
    #[serde(default)]
    pub ping_blob_life_time: Vec<Option<PingBlobLifeTime>>,
    #[serde(default)]
    pub ping_extended_life_time_params: Vec<Option<PingExtendedLifeTimeParams>>,
    #[serde(default)]
    pub ping_contact_life_time: Vec<Option<PingContactLifeTime>>,
    #[serde(default)]
    pub ping_sfxshared_params: Vec<Option<PingSFXSharedParams>>,
    #[serde(default)]
    pub ping_wave_vfxparams: Vec<Option<PingWaveVFXParams>>,
    #[serde(default)]
    pub ping_vfxshared_params: Vec<Option<PingVFXSharedParams>>,
    #[serde(default)]
    pub ping_type_params: Vec<Option<PingTypeParams>>,
}

/// Bucket pool sub-struct for types in the `pl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsPl {
    #[serde(default)]
    pub placed_surface_effects_emitter: Vec<Option<PlacedSurfaceEffects_Emitter>>,
    #[serde(default)]
    pub planet_ocean_audio_checkpoint: Vec<Option<PlanetOceanAudioCheckpoint>>,
    #[serde(default)]
    pub planet_ocean_depth_assignment: Vec<Option<PlanetOceanDepthAssignment>>,
    #[serde(default)]
    pub planet_ocean_audio_data: Vec<Option<PlanetOceanAudioData>>,
    #[serde(default)]
    pub player_formation_params: Vec<Option<PlayerFormationParams>>,
    #[serde(default)]
    pub playlist_rngconfig: Vec<Option<PlaylistRNGConfig>>,
    #[serde(default)]
    pub planet_effect_loddistance: Vec<Option<PlanetEffectLODDistance>>,
    #[serde(default)]
    pub planet_effect_lod: Vec<Option<PlanetEffectLOD>>,
    #[serde(default)]
    pub player_animated_interaction_hand_params: Vec<Option<PlayerAnimatedInteractionHandParams>>,
    #[serde(default)]
    pub player_animated_interaction: Vec<Option<PlayerAnimatedInteraction>>,
    #[serde(default)]
    pub player_animated_interaction_base: Vec<Option<PlayerAnimatedInteractionBase>>,
    #[serde(default)]
    pub player_animated_interaction_template: Vec<Option<PlayerAnimatedInteractionTemplate>>,
    #[serde(default)]
    pub player_animated_interaction_walking_request_params: Vec<Option<PlayerAnimatedInteractionWalkingRequestParams>>,
    #[serde(default)]
    pub player_animated_interaction_filtered: Vec<Option<PlayerAnimatedInteractionFiltered>>,
    #[serde(default)]
    pub player_animated_interaction_config: Vec<Option<PlayerAnimatedInteractionConfig>>,
    #[serde(default)]
    pub player_choice_interaction_modifier: Vec<Option<PlayerChoice_InteractionModifier>>,
    #[serde(default)]
    pub player_choice_signal_config: Vec<Option<PlayerChoice_SignalConfig>>,
    #[serde(default)]
    pub player_choice_option: Vec<Option<PlayerChoice_Option>>,
    #[serde(default)]
    pub player_choice_option_list: Vec<Option<PlayerChoice_OptionList>>,
    #[serde(default)]
    pub player_choice_library: Vec<Option<PlayerChoice_Library>>,
    #[serde(default)]
    pub player_choice_remote_comms_config: Vec<Option<PlayerChoice_RemoteCommsConfig>>,
    #[serde(default)]
    pub player_choice_head_look_params: Vec<Option<PlayerChoice_HeadLookParams>>,
    #[serde(default)]
    pub player_choice_imconfig: Vec<Option<PlayerChoice_IMConfig>>,
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
    pub player_ship_respawn_ship_info: Vec<Option<PlayerShipRespawnShipInfo>>,
    #[serde(default)]
    pub player_ship_respawn: Vec<Option<PlayerShipRespawn>>,
    #[serde(default)]
    pub player_trade_notification: Vec<Option<PlayerTradeNotification>>,
    #[serde(default)]
    pub player_trade_global_params: Vec<Option<PlayerTradeGlobalParams>>,
    #[serde(default)]
    pub planet_day_night_temperature_params: Vec<Option<PlanetDayNightTemperatureParams>>,
    #[serde(default)]
    pub planet_day_night_temperature_template: Vec<Option<PlanetDayNightTemperatureTemplate>>,
    #[serde(default)]
    pub player_to_player_comms_call_global_params: Vec<Option<PlayerToPlayerCommsCallGlobalParams>>,
    #[serde(default)]
    pub player_choice_pitconfig: Vec<Option<PlayerChoice_PITConfig>>,
    #[serde(default)]
    pub player_ecggraph_config: Vec<Option<PlayerECGGraph_Config>>,
}

/// Bucket pool sub-struct for types in the `po` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsPo {
    #[serde(default)]
    pub pooled_light_data: Vec<Option<PooledLightData>>,
    #[serde(default)]
    pub pool_filter_no_ref: Vec<Option<PoolFilter_NoRef>>,
    #[serde(default)]
    pub pool_filter_record: Vec<Option<PoolFilterRecord>>,
    #[serde(default)]
    pub point_of_interest_data: Vec<Option<PointOfInterestData>>,
    #[serde(default)]
    pub point_of_interest_list: Vec<Option<PointOfInterestList>>,
    #[serde(default)]
    pub posture_database: Vec<Option<PostureDatabase>>,
    #[serde(default)]
    pub posture_group: Vec<Option<PostureGroup>>,
    #[serde(default)]
    pub posture_data: Vec<Option<PostureData>>,
    #[serde(default)]
    pub popup_def: Vec<Option<PopupDef>>,
}

/// Bucket pool sub-struct for types in the `pr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsPr {
    #[serde(default)]
    pub procedural_idle_to_move_params: Vec<Option<ProceduralIdleToMoveParams>>,
    #[serde(default)]
    pub projectile_params: Vec<Option<ProjectileParams>>,
    #[serde(default)]
    pub projectile_spawned_entity_params: Vec<Option<ProjectileSpawnedEntityParams>>,
    #[serde(default)]
    pub projectile_detonation_params: Vec<Option<ProjectileDetonationParams>>,
    #[serde(default)]
    pub projectile_proximity_trigger_params: Vec<Option<ProjectileProximityTriggerParams>>,
    #[serde(default)]
    pub procedural_planet_audio_tag_and_event: Vec<Option<ProceduralPlanetAudioTagAndEvent>>,
    #[serde(default)]
    pub procedural_planet_audio_tag_and_rtpc: Vec<Option<ProceduralPlanetAudioTagAndRtpc>>,
    #[serde(default)]
    pub procedural_planet_audio_tag_and_events_def: Vec<Option<ProceduralPlanetAudioTagAndEventsDef>>,
    #[serde(default)]
    pub procedural_planet_audio_algorithm: Vec<Option<ProceduralPlanetAudioAlgorithm>>,
    #[serde(default)]
    pub procedural_planet_audio_disturbance_def: Vec<Option<ProceduralPlanetAudioDisturbanceDef>>,
    #[serde(default)]
    pub procedural_planet_audio_disturbance_list: Vec<Option<ProceduralPlanetAudioDisturbanceList>>,
    #[serde(default)]
    pub procedural_planet_audio_entry: Vec<Option<ProceduralPlanetAudioEntry>>,
    #[serde(default)]
    pub procedural_planet_audio_data: Vec<Option<ProceduralPlanetAudioData>>,
    #[serde(default)]
    pub procedural_planet_audio_river_data: Vec<Option<ProceduralPlanetAudioRiverData>>,
    #[serde(default)]
    pub proc_breathing_curve: Vec<Option<ProcBreathingCurve>>,
    #[serde(default)]
    pub proc_breathing_curve_database: Vec<Option<ProcBreathingCurveDatabase>>,
    #[serde(default)]
    pub proc_breathing_graph: Vec<Option<ProcBreathingGraph>>,
    #[serde(default)]
    pub proc_breathing_graph_entry: Vec<Option<ProcBreathingGraphEntry>>,
    #[serde(default)]
    pub proc_breathing_exertion: Vec<Option<ProcBreathingExertion>>,
    #[serde(default)]
    pub proc_breathing_hold_breath_noise: Vec<Option<ProcBreathingHoldBreathNoise>>,
    #[serde(default)]
    pub proc_breathing_setup: Vec<Option<ProcBreathingSetup>>,
    #[serde(default)]
    pub proc_aim_base_joint_type_config: Vec<Option<ProcAimBaseJointTypeConfig>>,
    #[serde(default)]
    pub proc_aim_rig_config: Vec<Option<ProcAimRigConfig>>,
    #[serde(default)]
    pub procedural_aim_rig_record: Vec<Option<ProceduralAimRigRecord>>,
    #[serde(default)]
    pub procedural_animation_bone: Vec<Option<ProceduralAnimationBone>>,
    #[serde(default)]
    pub procedural_animation_sequence: Vec<Option<ProceduralAnimationSequence>>,
    #[serde(default)]
    pub procedural_animation: Vec<Option<ProceduralAnimation>>,
    #[serde(default)]
    pub procedural_landing_filter: Vec<Option<ProceduralLandingFilter>>,
    #[serde(default)]
    pub procedural_landing_setup: Vec<Option<ProceduralLandingSetup>>,
    #[serde(default)]
    pub procedural_layout_node_base: Vec<Option<ProceduralLayoutNode_Base>>,
    #[serde(default)]
    pub procedural_layout_supplementary_element_tags_options: Vec<Option<ProceduralLayout_SupplementaryElementTagsOptions>>,
    #[serde(default)]
    pub procedural_layout_tag_filter: Vec<Option<ProceduralLayout_TagFilter>>,
    #[serde(default)]
    pub procedural_layout_graph: Vec<Option<ProceduralLayoutGraph>>,
    #[serde(default)]
    pub proximity_inventory_detection_params: Vec<Option<ProximityInventoryDetectionParams>>,
}

/// Bucket pool sub-struct for types in the `qt` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsQt {
    #[serde(default)]
    pub qterequest_config: Vec<Option<QTERequestConfig>>,
}

/// Bucket pool sub-struct for types in the `qu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsQu {
    #[serde(default)]
    pub queueing_behaviour: Vec<Option<QueueingBehaviour>>,
    #[serde(default)]
    pub quat: Vec<Option<Quat>>,
    #[serde(default)]
    pub quat_t: Vec<Option<QuatT>>,
    #[serde(default)]
    pub quantum_drive_spline_rollback_params: Vec<Option<QuantumDriveSplineRollbackParams>>,
    #[serde(default)]
    pub quantum_drive_spline_fxnode: Vec<Option<QuantumDriveSplineFXNode>>,
    #[serde(default)]
    pub quantum_drive_spline_fxparams: Vec<Option<QuantumDriveSplineFXParams>>,
    #[serde(default)]
    pub quantum_drive_spline_traversal_params: Vec<Option<QuantumDriveSplineTraversalParams>>,
    #[serde(default)]
    pub quantum_drive_notification: Vec<Option<QuantumDriveNotification>>,
    #[serde(default)]
    pub quantum_drive_notifications: Vec<Option<QuantumDriveNotifications>>,
    #[serde(default)]
    pub quantum_drive_audio_params: Vec<Option<QuantumDriveAudioParams>>,
    #[serde(default)]
    pub quantum_drive_global_params: Vec<Option<QuantumDriveGlobalParams>>,
    #[serde(default)]
    pub quantum_drive_effect_settings: Vec<Option<QuantumDriveEffectSettings>>,
    #[serde(default)]
    pub quantum_music_event_base: Vec<Option<QuantumMusicEventBase>>,
    #[serde(default)]
    pub quantum_drive_state_music_map: Vec<Option<QuantumDriveStateMusicMap>>,
    #[serde(default)]
    pub quantum_state_music_map: Vec<Option<QuantumStateMusicMap>>,
    #[serde(default)]
    pub quantum_drive_loc_type_music_map: Vec<Option<QuantumDriveLocTypeMusicMap>>,
    #[serde(default)]
    pub quantum_effect_music: Vec<Option<QuantumEffectMusic>>,
    #[serde(default)]
    pub quantum_music_trip_category: Vec<Option<QuantumMusicTripCategory>>,
    #[serde(default)]
    pub quantum_music_params: Vec<Option<QuantumMusicParams>>,
    #[serde(default)]
    pub quick_access_wheel_element: Vec<Option<QuickAccessWheelElement>>,
}

/// Bucket pool sub-struct for types in the `ra` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsRa {
    #[serde(default)]
    pub ragdoll_recovery_config: Vec<Option<RagdollRecoveryConfig>>,
    #[serde(default)]
    pub range: Vec<Option<Range>>,
    #[serde(default)]
    pub ra_sta_rlibrary_element: Vec<Option<RaSTaRLibraryElement>>,
    #[serde(default)]
    pub ra_sta_rlibrary_category: Vec<Option<RaSTaRLibraryCategory>>,
    #[serde(default)]
    pub ra_sta_rlibrary: Vec<Option<RaSTaRLibrary>>,
    #[serde(default)]
    pub radar_plate_config: Vec<Option<RadarPlate_Config>>,
    #[serde(default)]
    pub radar_display_entry_effects_config: Vec<Option<RadarDisplayEntryEffects_Config>>,
    #[serde(default)]
    pub radar_display_config: Vec<Option<RadarDisplay_Config>>,
    #[serde(default)]
    pub radar_system_global_params: Vec<Option<RadarSystemGlobalParams>>,
    #[serde(default)]
    pub radar_delta_signature_detection_params: Vec<Option<RadarDeltaSignatureDetectionParams>>,
    #[serde(default)]
    pub radar_system_shared_params: Vec<Option<RadarSystemSharedParams>>,
    #[serde(default)]
    pub radar_shared_params: Vec<Option<RadarSharedParams>>,
    #[serde(default)]
    pub radar_jammer_shared_params: Vec<Option<RadarJammerSharedParams>>,
    #[serde(default)]
    pub radar_contact_game_play_properties: Vec<Option<RadarContactGamePlayProperties>>,
    #[serde(default)]
    pub radar_signature_category_definition: Vec<Option<RadarSignatureCategoryDefinition>>,
    #[serde(default)]
    pub radar_signature_category_entry: Vec<Option<RadarSignatureCategoryEntry>>,
    #[serde(default)]
    pub radar_contact_type_definition: Vec<Option<RadarContactTypeDefinition>>,
    #[serde(default)]
    pub radar_contact_type_entry: Vec<Option<RadarContactTypeEntry>>,
    #[serde(default)]
    pub radar_contact_group_definition: Vec<Option<RadarContactGroupDefinition>>,
    #[serde(default)]
    pub radar_contact_group_entry: Vec<Option<RadarContactGroupEntry>>,
    #[serde(default)]
    pub radar_contact_sub_group_entry: Vec<Option<RadarContactSubGroupEntry>>,
    #[serde(default)]
    pub radar_delta_signature_notification_params: Vec<Option<RadarDeltaSignatureNotificationParams>>,
    #[serde(default)]
    pub radar_delta_signature_definition: Vec<Option<RadarDeltaSignatureDefinition>>,
    #[serde(default)]
    pub radar_delta_signature_entry: Vec<Option<RadarDeltaSignatureEntry>>,
    #[serde(default)]
    pub radiation_state_property_params: Vec<Option<RadiationStatePropertyParams>>,
    #[serde(default)]
    pub radiation_state_template_internal: Vec<Option<RadiationStateTemplateInternal>>,
    #[serde(default)]
    pub radiation_state_template: Vec<Option<RadiationStateTemplate>>,
    #[serde(default)]
    pub radiation_behavior: Vec<Option<RadiationBehavior>>,
    #[serde(default)]
    pub radiation_behavior_surface_radiation_params: Vec<Option<RadiationBehavior_SurfaceRadiationParams>>,
    #[serde(default)]
    pub radar_display3_dpreset: Vec<Option<RadarDisplay3DPreset>>,
}

/// Bucket pool sub-struct for types in the `re` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsRe {
    #[serde(default)]
    pub resistance_weight_params: Vec<Option<ResistanceWeightParams>>,
    #[serde(default)]
    pub revival_fade_in_params: Vec<Option<RevivalFadeInParams>>,
    #[serde(default)]
    pub reverse_trails_setting: Vec<Option<ReverseTrailsSetting>>,
    #[serde(default)]
    pub reputation_prerequisite_range: Vec<Option<ReputationPrerequisiteRange>>,
    #[serde(default)]
    pub reputation_prerequisites: Vec<Option<ReputationPrerequisites>>,
    #[serde(default)]
    pub relation_marker_params: Vec<Option<RelationMarkerParams>>,
    #[serde(default)]
    pub relation_standing_params: Vec<Option<RelationStandingParams>>,
    #[serde(default)]
    pub reputation_reward_base_def: Vec<Option<ReputationRewardBaseDef>>,
    #[serde(default)]
    pub refining_process: Vec<Option<RefiningProcess>>,
    #[serde(default)]
    pub refinery_notification_configuration: Vec<Option<RefineryNotificationConfiguration>>,
    #[serde(default)]
    pub rental_notification_params: Vec<Option<RentalNotificationParams>>,
    #[serde(default)]
    pub reputation_value_setting: Vec<Option<ReputationValueSetting>>,
    #[serde(default)]
    pub reputation_comparison_range: Vec<Option<ReputationComparisonRange>>,
    #[serde(default)]
    pub reputation_value_settings: Vec<Option<ReputationValueSettings>>,
    #[serde(default)]
    pub resource_type_properties: Vec<Option<ResourceTypeProperties>>,
    #[serde(default)]
    pub resource_type_density_type: Vec<Option<ResourceTypeDensityType>>,
    #[serde(default)]
    pub resource_type: Vec<Option<ResourceType>>,
    #[serde(default)]
    pub resource_type_group: Vec<Option<ResourceTypeGroup>>,
    #[serde(default)]
    pub resource_type_database: Vec<Option<ResourceTypeDatabase>>,
}

/// Bucket pool sub-struct for types in the `rg` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsRg {
    #[serde(default)]
    pub rgb8: Vec<Option<RGB8>>,
    #[serde(default)]
    pub rgba: Vec<Option<RGBA>>,
    #[serde(default)]
    pub rgb: Vec<Option<RGB>>,
}

/// Bucket pool sub-struct for types in the `ro` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsRo {
    #[serde(default)]
    pub room_traversal_operations_params: Vec<Option<RoomTraversalOperationsParams>>,
    #[serde(default)]
    pub room_traversal_contact_type_entry: Vec<Option<RoomTraversalContactTypeEntry>>,
    #[serde(default)]
    pub room_traversal_params: Vec<Option<RoomTraversalParams>>,
    #[serde(default)]
    pub room_shared_params: Vec<Option<RoomSharedParams>>,
}

/// Bucket pool sub-struct for types in the `rt` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsRt {
    #[serde(default)]
    pub rttsunlight_params: Vec<Option<RTTSunlightParams>>,
}

/// Bucket pool sub-struct for types in the `sa` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSa {
    #[serde(default)]
    pub saiperception_disturbance_type_settings: Vec<Option<SAIPerceptionDisturbanceTypeSettings>>,
    #[serde(default)]
    pub saiperception_meter_settings: Vec<Option<SAIPerceptionMeterSettings>>,
    #[serde(default)]
    pub saiperception_audio_type_settings: Vec<Option<SAIPerceptionAudioTypeSettings>>,
    #[serde(default)]
    pub saiperception_audio_settings: Vec<Option<SAIPerceptionAudioSettings>>,
    #[serde(default)]
    pub saiperception_visual_settings: Vec<Option<SAIPerceptionVisualSettings>>,
    #[serde(default)]
    pub sactor_force_reaction_filter_range_override_def: Vec<Option<SActorForceReactionFilterRangeOverrideDef>>,
    #[serde(default)]
    pub sactor_force_reaction_filter_item_def: Vec<Option<SActorForceReactionFilterItemDef>>,
    #[serde(default)]
    pub sactor_force_reaction_lean_filter_item_def: Vec<Option<SActorForceReactionLeanFilterItemDef>>,
    #[serde(default)]
    pub sactor_force_reaction_limit_def: Vec<Option<SActorForceReactionLimitDef>>,
    #[serde(default)]
    pub sactor_force_reaction_state_config: Vec<Option<SActorForceReactionStateConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_filter_def: Vec<Option<SActorForceReactionFilterDef>>,
    #[serde(default)]
    pub sactor_force_reaction_envelope: Vec<Option<SActorForceReactionEnvelope>>,
    #[serde(default)]
    pub sactor_force_reaction_curve: Vec<Option<SActorForceReactionCurve>>,
    #[serde(default)]
    pub sactor_force_reaction_curve_config: Vec<Option<SActorForceReactionCurveConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_effect_defaults: Vec<Option<SActorForceReactionEffectDefaults>>,
    #[serde(default)]
    pub sactor_force_reaction_global_effect_config: Vec<Option<SActorForceReactionGlobalEffectConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_aim_punch_config: Vec<Option<SActorForceReactionAimPunchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_weapon_twitch_config: Vec<Option<SActorForceReactionWeaponTwitchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_head_recoil_config: Vec<Option<SActorForceReactionHeadRecoilConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_fovscale_config: Vec<Option<SActorForceReactionFOVScaleConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_animation_twitch_config: Vec<Option<SActorForceReactionAnimationTwitchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_animation_flinch_config: Vec<Option<SActorForceReactionAnimationFlinchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_animation_stagger_config: Vec<Option<SActorForceReactionAnimationStaggerConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_stagger_tag_config: Vec<Option<SActorForceReactionStaggerTagConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_global_stagger_config: Vec<Option<SActorForceReactionGlobalStaggerConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_impulse_accumulation_config: Vec<Option<SActorForceReactionImpulseAccumulationConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_flight_duration_config: Vec<Option<SActorForceReactionFlightDurationConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_movement_launch_config: Vec<Option<SActorForceReactionMovementLaunchConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_block_adsconfig: Vec<Option<SActorForceReactionBlockADSConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_block_config: Vec<Option<SActorForceReactionBlockConfig>>,
    #[serde(default)]
    pub sactor_force_reaction_twitch_range_def: Vec<Option<SActorForceReactionTwitchRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_twitch_config_def: Vec<Option<SActorForceReactionTwitchConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_flinch_range_def: Vec<Option<SActorForceReactionFlinchRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_flinch_config_def: Vec<Option<SActorForceReactionFlinchConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_stagger_range_def: Vec<Option<SActorForceReactionStaggerRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_filtered_stagger_range_def: Vec<Option<SActorForceReactionFilteredStaggerRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_filtered_stagger_config_def: Vec<Option<SActorForceReactionFilteredStaggerConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_unfiltered_stagger_config_def: Vec<Option<SActorForceReactionUnfilteredStaggerConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_knockdown_range_def: Vec<Option<SActorForceReactionKnockdownRangeDef>>,
    #[serde(default)]
    pub sactor_force_reaction_knockdown_config_def: Vec<Option<SActorForceReactionKnockdownConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_lean_angle_limits_def: Vec<Option<SActorForceReactionLeanAngleLimitsDef>>,
    #[serde(default)]
    pub sactor_force_reaction_lean_filter_def: Vec<Option<SActorForceReactionLeanFilterDef>>,
    #[serde(default)]
    pub sactor_force_reaction_procedural_lean_pose_list: Vec<Option<SActorForceReactionProceduralLeanPoseList>>,
    #[serde(default)]
    pub sactor_force_reaction_lean_config_def: Vec<Option<SActorForceReactionLeanConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_stumble_config_def: Vec<Option<SActorForceReactionStumbleConfigDef>>,
    #[serde(default)]
    pub sactor_force_reaction_sustained_impulse_def: Vec<Option<SActorForceReactionSustainedImpulseDef>>,
    #[serde(default)]
    pub sactor_force_reaction_external_impulse_def: Vec<Option<SActorForceReactionExternalImpulseDef>>,
    #[serde(default)]
    pub sactor_force_reaction_actor_bump_def: Vec<Option<SActorForceReactionActorBumpDef>>,
    #[serde(default)]
    pub sactor_force_reaction_sustained_force_def: Vec<Option<SActorForceReactionSustainedForceDef>>,
    #[serde(default)]
    pub sactor_force_reactions_stun_def: Vec<Option<SActorForceReactionsStunDef>>,
    #[serde(default)]
    pub sactor_force_reactions_vehicle_force_dampening_def: Vec<Option<SActorForceReactionsVehicleForceDampeningDef>>,
    #[serde(default)]
    pub sactor_force_reactions_def: Vec<Option<SActorForceReactionsDef>>,
    #[serde(default)]
    pub sactor_hit_reactions_def: Vec<Option<SActorHitReactionsDef>>,
    #[serde(default)]
    pub sactor_external_force_response_vibration_entry: Vec<Option<SActorExternalForceResponseVibrationEntry>>,
    #[serde(default)]
    pub sactor_external_force_response_camera_shake_def: Vec<Option<SActorExternalForceResponseCameraShakeDef>>,
    #[serde(default)]
    pub sactor_force_reactions_procedural_lean_override: Vec<Option<SActorForceReactionsProceduralLeanOverride>>,
    #[serde(default)]
    pub sactor_force_reactions_preset_record: Vec<Option<SActorForceReactionsPresetRecord>>,
    #[serde(default)]
    pub sactor_locomotion_fidget_severity_params: Vec<Option<SActorLocomotionFidgetSeverityParams>>,
    #[serde(default)]
    pub sactor_locomotion_fidget_state_filtered_def: Vec<Option<SActorLocomotionFidgetStateFilteredDef>>,
    #[serde(default)]
    pub sactor_locomotion_fidget_def: Vec<Option<SActorLocomotionFidgetDef>>,
    #[serde(default)]
    pub sactor_locomotion_feature_def_slope: Vec<Option<SActorLocomotionFeatureDef_Slope>>,
    #[serde(default)]
    pub sactor_locomotion_submerged_creature_params: Vec<Option<SActorLocomotionSubmergedCreatureParams>>,
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
    pub sactor_procedural_head_recoil_modifiers: Vec<Option<SActorProceduralHeadRecoilModifiers>>,
    #[serde(default)]
    pub sactor_stance_dimensions_extra_def: Vec<Option<SActorStanceDimensionsExtraDef>>,
    #[serde(default)]
    pub saudio_breath_parameters: Vec<Option<SAudioBreathParameters>>,
    #[serde(default)]
    pub sactor_carry_config_tag_switch: Vec<Option<SActorCarryConfigTagSwitch>>,
    #[serde(default)]
    pub sarchetype_asset_def_base: Vec<Option<SArchetypeAssetDefBase>>,
    #[serde(default)]
    pub sarchetype_asset_list: Vec<Option<SArchetypeAssetList>>,
    #[serde(default)]
    pub sasset_list_condition: Vec<Option<SAssetListCondition>>,
    #[serde(default)]
    pub sauto_loading_box_size_prices: Vec<Option<SAutoLoadingBoxSizePrices>>,
    #[serde(default)]
    pub sangle_constraint: Vec<Option<SAngleConstraint>>,
    #[serde(default)]
    pub sauto_loading_box_size_loading_time: Vec<Option<SAutoLoadingBoxSizeLoadingTime>>,
    #[serde(default)]
    pub sandbox_infraction_base_def: Vec<Option<SandboxInfractionBaseDef>>,
    #[serde(default)]
    pub sandbox_trigger_base_def: Vec<Option<SandboxTriggerBaseDef>>,
    #[serde(default)]
    pub sandbox_trigger_manual_params: Vec<Option<SandboxTriggerManualParams>>,
    #[serde(default)]
    pub sandbox_trigger_record: Vec<Option<SandboxTriggerRecord>>,
    #[serde(default)]
    pub satmospheric_composition_params: Vec<Option<SAtmosphericCompositionParams>>,
    #[serde(default)]
    pub sanalytics_event: Vec<Option<SAnalyticsEvent>>,
    #[serde(default)]
    pub sanalytics_event_database: Vec<Option<SAnalyticsEventDatabase>>,
    #[serde(default)]
    pub saimable_gimbal_mode_labels: Vec<Option<SAimableGimbalModeLabels>>,
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
    pub saimable_controller_hud_params: Vec<Option<SAimableControllerHudParams>>,
    #[serde(default)]
    pub saim_recoil_modifier: Vec<Option<SAimRecoilModifier>>,
    #[serde(default)]
    pub saim_modifier: Vec<Option<SAimModifier>>,
    #[serde(default)]
    pub saim_recoil_noise_curves: Vec<Option<SAimRecoilNoiseCurves>>,
    #[serde(default)]
    pub samplitude_freqency_decay_curves: Vec<Option<SAmplitudeFreqencyDecayCurves>>,
}

/// Bucket pool sub-struct for types in the `sb` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSb {
    #[serde(default)]
    pub sbezier_curve_record: Vec<Option<SBezierCurveRecord>>,
    #[serde(default)]
    pub sbase_cargo_unit: Vec<Option<SBaseCargoUnit>>,
    #[serde(default)]
    pub sbreakable_physics_params: Vec<Option<SBreakablePhysicsParams>>,
    #[serde(default)]
    pub sbbdynamic_property_base: Vec<Option<SBBDynamicPropertyBase>>,
}

/// Bucket pool sub-struct for types in the `sc` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSc {
    #[serde(default)]
    pub scprone_motion_graph_def: Vec<Option<SCProneMotionGraphDef>>,
    #[serde(default)]
    pub sczero_glaunch_params: Vec<Option<SCZeroGLaunchParams>>,
    #[serde(default)]
    pub scdefault_zero_gtraversal_params: Vec<Option<SCDefaultZeroGTraversalParams>>,
    #[serde(default)]
    pub scoptional_zero_gtraversal_params: Vec<Option<SCOptionalZeroGTraversalParams>>,
    #[serde(default)]
    pub scustomizable_material_entry: Vec<Option<SCustomizableMaterialEntry>>,
    #[serde(default)]
    pub scustomizable_material_lookup_table: Vec<Option<SCustomizableMaterialLookupTable>>,
    #[serde(default)]
    pub scharacter_archetype_params: Vec<Option<SCharacterArchetypeParams>>,
    #[serde(default)]
    pub scharacter_archetype: Vec<Option<SCharacterArchetype>>,
    #[serde(default)]
    pub scharacter_archetype_list: Vec<Option<SCharacterArchetypeList>>,
    #[serde(default)]
    pub scharacter_generation_params: Vec<Option<SCharacterGenerationParams>>,
    #[serde(default)]
    pub scharacter_serialization_texture: Vec<Option<SCharacterSerializationTexture>>,
    #[serde(default)]
    pub scharacter_serialization_materials_settings_preset: Vec<Option<SCharacterSerializationMaterialsSettingsPreset>>,
    #[serde(default)]
    pub scharacter_archetype_additive_asset_list: Vec<Option<SCharacterArchetypeAdditiveAssetList>>,
    #[serde(default)]
    pub scdynamic_rig_intensity_params: Vec<Option<SCDynamicRigIntensityParams>>,
    #[serde(default)]
    pub scdynamic_rig_light_params: Vec<Option<SCDynamicRigLightParams>>,
    #[serde(default)]
    pub scdynamic_lighting_rig_global_params: Vec<Option<SCDynamicLightingRigGlobalParams>>,
    #[serde(default)]
    pub scombat_targeting: Vec<Option<SCombatTargeting>>,
    #[serde(default)]
    pub scentrance_item: Vec<Option<SCEntranceItem>>,
    #[serde(default)]
    pub scarryable_ikinteraction: Vec<Option<SCarryableIKInteraction>>,
    #[serde(default)]
    pub scarryable_ikinteraction_list: Vec<Option<SCarryableIKInteractionList>>,
    #[serde(default)]
    pub schat_channel_type_base: Vec<Option<SChatChannelTypeBase>>,
    #[serde(default)]
    pub schat_channel_filter_base: Vec<Option<SChatChannelFilterBase>>,
    #[serde(default)]
    pub scharge_drain_highlight_outline_values: Vec<Option<SChargeDrainHighlightOutlineValues>>,
    #[serde(default)]
    pub scharge_drain_target_state_outline_params: Vec<Option<SChargeDrainTargetStateOutlineParams>>,
    #[serde(default)]
    pub scharge_drain_card_params: Vec<Option<SChargeDrainCardParams>>,
    #[serde(default)]
    pub scuttable_shape_definition: Vec<Option<SCuttableShapeDefinition>>,
    #[serde(default)]
    pub scextended_localization_level_params: Vec<Option<SCExtendedLocalizationLevelParams>>,
    #[serde(default)]
    pub scan_wave_detection_params: Vec<Option<ScanWaveDetectionParams>>,
    #[serde(default)]
    pub scan_shared_params: Vec<Option<ScanSharedParams>>,
    #[serde(default)]
    pub scan_sfxshared_params: Vec<Option<ScanSFXSharedParams>>,
    #[serde(default)]
    pub scan_information_def: Vec<Option<ScanInformationDef>>,
    #[serde(default)]
    pub scan_information_table: Vec<Option<ScanInformationTable>>,
    #[serde(default)]
    pub scan_boxout_icon_def: Vec<Option<ScanBoxoutIconDef>>,
    #[serde(default)]
    pub scan_custom_data: Vec<Option<ScanCustomData>>,
    #[serde(default)]
    pub scan_custom_data_info: Vec<Option<ScanCustomDataInfo>>,
    #[serde(default)]
    pub scan_custom_data_def: Vec<Option<ScanCustomDataDef>>,
    #[serde(default)]
    pub scan_display_setup_params: Vec<Option<ScanDisplaySetupParams>>,
    #[serde(default)]
    pub scan_display_section_params: Vec<Option<ScanDisplaySectionParams>>,
    #[serde(default)]
    pub scan_display_instance_params: Vec<Option<ScanDisplayInstanceParams>>,
    #[serde(default)]
    pub scan_display_layout_params: Vec<Option<ScanDisplayLayoutParams>>,
    #[serde(default)]
    pub scan_procedure_params: Vec<Option<ScanProcedureParams>>,
    #[serde(default)]
    pub scitem_localization: Vec<Option<SCItemLocalization>>,
    #[serde(default)]
    pub scitem_light_amplification: Vec<Option<SCItemLightAmplification>>,
    #[serde(default)]
    pub scitem_manufacturer: Vec<Option<SCItemManufacturer>>,
    #[serde(default)]
    pub scitem_visor_dashboard_config: Vec<Option<SCItemVisorDashboardConfig>>,
    #[serde(default)]
    pub scitem_suit_atmosphere_fuel_conversion_params: Vec<Option<SCItemSuitAtmosphereFuelConversionParams>>,
    #[serde(default)]
    pub scitem_suit_fuel_params: Vec<Option<SCItemSuitFuelParams>>,
    #[serde(default)]
    pub scitem_comms_component_setup: Vec<Option<SCItemCommsComponentSetup>>,
    #[serde(default)]
    pub scitem_display_screen_preset: Vec<Option<SCItemDisplayScreenPreset>>,
    #[serde(default)]
    pub scseat_head_pos_adjust_setup: Vec<Option<SCSeatHeadPosAdjustSetup>>,
    #[serde(default)]
    pub scitem_seat_head_tracking_position_limit_params: Vec<Option<SCItemSeatHeadTrackingPositionLimitParams>>,
    #[serde(default)]
    pub scitem_seat_dashboard_screen_pos: Vec<Option<SCItemSeatDashboardScreenPos>>,
    #[serde(default)]
    pub scitem_seat_dashboard_screen_style: Vec<Option<SCItemSeatDashboardScreenStyle>>,
    #[serde(default)]
    pub scitem_seat_dashboard_screen: Vec<Option<SCItemSeatDashboardScreen>>,
    #[serde(default)]
    pub scsignature_system_room_params: Vec<Option<SCSignatureSystemRoomParams>>,
    #[serde(default)]
    pub scsignature_death_params: Vec<Option<SCSignatureDeathParams>>,
    #[serde(default)]
    pub scenario_progress: Vec<Option<ScenarioProgress>>,
    #[serde(default)]
    pub screen_effects_library: Vec<Option<ScreenEffects_Library>>,
    #[serde(default)]
    pub screen_effects_effect: Vec<Option<ScreenEffects_Effect>>,
    #[serde(default)]
    pub screen_effects_pattern: Vec<Option<ScreenEffects_Pattern>>,
    #[serde(default)]
    pub screen_effects_param: Vec<Option<ScreenEffects_Param>>,
    #[serde(default)]
    pub screen_effects_param_value: Vec<Option<ScreenEffects_ParamValue>>,
    #[serde(default)]
    pub screen_effects_param_strength_behavior: Vec<Option<ScreenEffects_ParamStrengthBehavior>>,
    #[serde(default)]
    pub screen_effects_debug: Vec<Option<ScreenEffects_Debug>>,
    #[serde(default)]
    pub screen_effects_debug_effect: Vec<Option<ScreenEffects_DebugEffect>>,
    #[serde(default)]
    pub screen_effects_debug_param: Vec<Option<ScreenEffects_DebugParam>>,
    #[serde(default)]
    pub sconversation_icon_params: Vec<Option<SConversationIconParams>>,
    #[serde(default)]
    pub scobject_data_bank_entry_marker_config: Vec<Option<SCObjectDataBankEntryMarkerConfig>>,
    #[serde(default)]
    pub scitem_uiview_dashboard_canvas_view_def: Vec<Option<SCItemUIView_DashboardCanvasViewDef>>,
    #[serde(default)]
    pub scitem_uiview_dashboard_canvas_def: Vec<Option<SCItemUIView_DashboardCanvasDef>>,
    #[serde(default)]
    pub scurve: Vec<Option<SCurve>>,
}

/// Bucket pool sub-struct for types in the `sd` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSd {
    #[serde(default)]
    pub sdensity_class_lifetime_override_entry: Vec<Option<SDensityClassLifetimeOverrideEntry>>,
    #[serde(default)]
    pub sdefault_density_class_lifetime_overrides: Vec<Option<SDefaultDensityClassLifetimeOverrides>>,
    #[serde(default)]
    pub sdecay_curve_max_value_params: Vec<Option<SDecayCurveMaxValueParams>>,
    #[serde(default)]
    pub sdecay_curve_max_values: Vec<Option<SDecayCurveMaxValues>>,
    #[serde(default)]
    pub sdecay_times_and_curves: Vec<Option<SDecayTimesAndCurves>>,
}

/// Bucket pool sub-struct for types in the `se` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSe {
    #[serde(default)]
    pub seat_operator_skills: Vec<Option<SeatOperatorSkills>>,
    #[serde(default)]
    pub sentity_audio_controller_params: Vec<Option<SEntityAudioControllerParams>>,
    #[serde(default)]
    pub seaplayer_loadout_snapshot_entry: Vec<Option<SEAPlayerLoadoutSnapshotEntry>>,
    #[serde(default)]
    pub seaplayer_loadout_snapshots: Vec<Option<SEAPlayerLoadoutSnapshots>>,
    #[serde(default)]
    pub sealoadout_attachment: Vec<Option<SEALoadoutAttachment>>,
    #[serde(default)]
    pub sealoadout_explicit: Vec<Option<SEALoadoutExplicit>>,
    #[serde(default)]
    pub sealoadout_item: Vec<Option<SEALoadoutItem>>,
    #[serde(default)]
    pub sealoadout_set: Vec<Option<SEALoadoutSet>>,
    #[serde(default)]
    pub sealoadout_collection: Vec<Option<SEALoadoutCollection>>,
    #[serde(default)]
    pub seaglobal_special_loadout: Vec<Option<SEAGlobalSpecialLoadout>>,
    #[serde(default)]
    pub seaglobal_event_loadouts: Vec<Option<SEAGlobalEventLoadouts>>,
    #[serde(default)]
    pub sentrances_def: Vec<Option<SEntrancesDef>>,
    #[serde(default)]
    pub sentity_effect_system_attachment: Vec<Option<SEntityEffectSystem_Attachment>>,
    #[serde(default)]
    pub sentity_effect_system_property_modifier: Vec<Option<SEntityEffectSystem_PropertyModifier>>,
    #[serde(default)]
    pub sentity_effect_system_particle_category: Vec<Option<SEntityEffectSystem_ParticleCategory>>,
    #[serde(default)]
    pub sentity_effect_system_particle_tag_effect: Vec<Option<SEntityEffectSystem_ParticleTagEffect>>,
    #[serde(default)]
    pub sentity_effect_system_particle_trigger_effect: Vec<Option<SEntityEffectSystem_ParticleTriggerEffect>>,
    #[serde(default)]
    pub sentity_effect_system_particle_property_link: Vec<Option<SEntityEffectSystem_ParticlePropertyLink>>,
    #[serde(default)]
    pub sentity_density_class: Vec<Option<SEntityDensityClass>>,
    #[serde(default)]
    pub sentity_density_class_overrides_record: Vec<Option<SEntityDensityClassOverridesRecord>>,
    #[serde(default)]
    pub sentity_density_class_overrides_manual: Vec<Option<SEntityDensityClassOverridesManual>>,
    #[serde(default)]
    pub sentity_traversing_node_id: Vec<Option<SEntityTraversingNodeId>>,
    #[serde(default)]
    pub sentity_physics_controller_params: Vec<Option<SEntityPhysicsControllerParams>>,
    #[serde(default)]
    pub sentity_base_physics_controller_params: Vec<Option<SEntityBasePhysicsControllerParams>>,
    #[serde(default)]
    pub sentity_soft_ex_physics_controller_params: Vec<Option<SEntitySoftExPhysicsControllerParams>>,
    #[serde(default)]
    pub seat_reticle_archetype: Vec<Option<SeatReticleArchetype>>,
    #[serde(default)]
    pub seat_ads_def: Vec<Option<SeatAdsDef>>,
    #[serde(default)]
    pub seat_user_actor_cdikconfig: Vec<Option<SeatUserActorCDIKConfig>>,
    #[serde(default)]
    pub seat_user_actor_cdikmapping: Vec<Option<SeatUserActorCDIKMapping>>,
    #[serde(default)]
    pub seat_user_actor_cdikrecord: Vec<Option<SeatUserActorCDIKRecord>>,
    #[serde(default)]
    pub security_network_permissions: Vec<Option<SecurityNetworkPermissions>>,
    #[serde(default)]
    pub security_clearance_token_data: Vec<Option<SecurityClearanceTokenData>>,
    #[serde(default)]
    pub security_manual_input: Vec<Option<SecurityManualInput>>,
    #[serde(default)]
    pub security_notifications: Vec<Option<SecurityNotifications>>,
    #[serde(default)]
    pub security_clearance_token: Vec<Option<SecurityClearanceToken>>,
    #[serde(default)]
    pub security_clearance_conditions: Vec<Option<SecurityClearanceConditions>>,
    #[serde(default)]
    pub security_network_protocol: Vec<Option<SecurityNetworkProtocol>>,
    #[serde(default)]
    pub security_network_room_settings: Vec<Option<SecurityNetworkRoomSettings>>,
    #[serde(default)]
    pub security_network_protocol_override: Vec<Option<SecurityNetworkProtocolOverride>>,
    #[serde(default)]
    pub security_network_manifest: Vec<Option<SecurityNetworkManifest>>,
    #[serde(default)]
    pub security_network_variable_value_base: Vec<Option<SecurityNetworkVariableValue_Base>>,
    #[serde(default)]
    pub security_network_variable: Vec<Option<SecurityNetworkVariable>>,
    #[serde(default)]
    pub security_network_variable_effect_base: Vec<Option<SecurityNetworkVariableEffect_Base>>,
    #[serde(default)]
    pub security_network_variable_effects: Vec<Option<SecurityNetworkVariableEffects>>,
    #[serde(default)]
    pub service_beacon_base_params: Vec<Option<ServiceBeaconBaseParams>>,
    #[serde(default)]
    pub service_beacon_params: Vec<Option<ServiceBeaconParams>>,
    #[serde(default)]
    pub service_beacon_notification_params: Vec<Option<ServiceBeaconNotificationParams>>,
    #[serde(default)]
    pub service_beacon_global_params: Vec<Option<ServiceBeaconGlobalParams>>,
}

/// Bucket pool sub-struct for types in the `sg` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSg {
    #[serde(default)]
    pub sgrouped_loadouts: Vec<Option<SGroupedLoadouts>>,
    #[serde(default)]
    pub sgeometry_view_distance_ratio_categories: Vec<Option<SGeometryViewDistanceRatioCategories>>,
    #[serde(default)]
    pub sgeometry_data_params: Vec<Option<SGeometryDataParams>>,
    #[serde(default)]
    pub sgeometry_node_params: Vec<Option<SGeometryNodeParams>>,
    #[serde(default)]
    pub sgeometry_model_tag_base: Vec<Option<SGeometryModelTagBase>>,
    #[serde(default)]
    pub sgeometry_meshsetup_params: Vec<Option<SGeometryMeshsetupParams>>,
    #[serde(default)]
    pub sgeometry_resource_params: Vec<Option<SGeometryResourceParams>>,
    #[serde(default)]
    pub sglobal_charge_drain_beam_params: Vec<Option<SGlobalChargeDrainBeamParams>>,
    #[serde(default)]
    pub sglobal_crosshair_params: Vec<Option<SGlobalCrosshairParams>>,
    #[serde(default)]
    pub sglobal_cuttable_shape_params: Vec<Option<SGlobalCuttableShapeParams>>,
    #[serde(default)]
    pub sglobal_electron_params: Vec<Option<SGlobalElectronParams>>,
    #[serde(default)]
    pub sglobal_healing_beam_params: Vec<Option<SGlobalHealingBeamParams>>,
    #[serde(default)]
    pub sglobal_salvage_repair_beam_params: Vec<Option<SGlobalSalvageRepairBeamParams>>,
    #[serde(default)]
    pub sglobal_shop_errors: Vec<Option<SGlobalShopErrors>>,
    #[serde(default)]
    pub sglobal_tractor_beam_params: Vec<Option<SGlobalTractorBeamParams>>,
    #[serde(default)]
    pub sgrip: Vec<Option<SGrip>>,
    #[serde(default)]
    pub sgrip_shape_params: Vec<Option<SGripShapeParams>>,
    #[serde(default)]
    pub sglobal_hit_behavior_params: Vec<Option<SGlobalHitBehaviorParams>>,
    #[serde(default)]
    pub sgame_collision_class: Vec<Option<SGameCollisionClass>>,
    #[serde(default)]
    pub sgas_atmosphere_entry_params: Vec<Option<SGasAtmosphereEntryParams>>,
}

/// Bucket pool sub-struct for types in the `sh` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSh {
    #[serde(default)]
    pub shared_tactic_params: Vec<Option<SharedTacticParams>>,
    #[serde(default)]
    pub shooting_params: Vec<Option<ShootingParams>>,
    #[serde(default)]
    pub shands_recoil_curve_noise_modifer: Vec<Option<SHandsRecoilCurveNoiseModifer>>,
    #[serde(default)]
    pub shead_recoil_noise_modifier: Vec<Option<SHeadRecoilNoiseModifier>>,
    #[serde(default)]
    pub shockwave_params: Vec<Option<ShockwaveParams>>,
    #[serde(default)]
    pub shealing_beam_bone_entry_params: Vec<Option<SHealingBeamBoneEntryParams>>,
    #[serde(default)]
    pub shealing_beam_body_part_highlighting_params: Vec<Option<SHealingBeamBodyPartHighlightingParams>>,
    #[serde(default)]
    pub shealing_beam_body_part_params: Vec<Option<SHealingBeamBodyPartParams>>,
    #[serde(default)]
    pub shostility_rules: Vec<Option<SHostilityRules>>,
    #[serde(default)]
    pub ship_insurance_policy_record: Vec<Option<ShipInsurancePolicyRecord>>,
    #[serde(default)]
    pub shighlight_behavior_node_params: Vec<Option<SHighlightBehaviorNodeParams>>,
    #[serde(default)]
    pub shighlight_behavior_node: Vec<Option<SHighlightBehaviorNode>>,
    #[serde(default)]
    pub shud_tape_params: Vec<Option<SHudTapeParams>>,
    #[serde(default)]
    pub ship_computer_definition: Vec<Option<ShipComputerDefinition>>,
    #[serde(default)]
    pub shelmet_linked_state: Vec<Option<SHelmetLinkedState>>,
    #[serde(default)]
    pub shelmet_state_base_params: Vec<Option<SHelmetStateBaseParams>>,
    #[serde(default)]
    pub shelmet_state_machine_params: Vec<Option<SHelmetStateMachineParams>>,
    #[serde(default)]
    pub shop_interaction_data: Vec<Option<ShopInteractionData>>,
    #[serde(default)]
    pub shop_franchise: Vec<Option<ShopFranchise>>,
    #[serde(default)]
    pub shield_type_params: Vec<Option<ShieldTypeParams>>,
    #[serde(default)]
    pub ship_computer_preset_list: Vec<Option<ShipComputerPresetList>>,
    #[serde(default)]
    pub ship_computer_preset: Vec<Option<ShipComputerPreset>>,
    #[serde(default)]
    pub shands_recoil_curve_noise_params: Vec<Option<SHandsRecoilCurveNoiseParams>>,
    #[serde(default)]
    pub shands_recoil_time_modifier: Vec<Option<SHandsRecoilTimeModifier>>,
    #[serde(default)]
    pub shead_recoil_noise_params: Vec<Option<SHeadRecoilNoiseParams>>,
}

/// Bucket pool sub-struct for types in the `si` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSi {
    #[serde(default)]
    pub signature_params: Vec<Option<SignatureParams>>,
    #[serde(default)]
    pub sinput_deflection_time_params: Vec<Option<SInputDeflectionTimeParams>>,
    #[serde(default)]
    pub sirounds_module: Vec<Option<SIRoundsModule>>,
    #[serde(default)]
    pub sitem_port_loadout_base_params: Vec<Option<SItemPortLoadoutBaseParams>>,
    #[serde(default)]
    pub sifcsmodifier_number: Vec<Option<SIFCSModifierNumber>>,
    #[serde(default)]
    pub sifcsmodifier_vector: Vec<Option<SIFCSModifierVector>>,
    #[serde(default)]
    pub sifcsmodifiers_legacy: Vec<Option<SIFCSModifiersLegacy>>,
    #[serde(default)]
    pub sifcsesp_params: Vec<Option<SIFCSEspParams>>,
    #[serde(default)]
    pub sifcsesp: Vec<Option<SIFCSEsp>>,
    #[serde(default)]
    pub sifcsgame_mode_physics_damping: Vec<Option<SIFCSGameModePhysicsDamping>>,
    #[serde(default)]
    pub sifcsgame_mode_params: Vec<Option<SIFCSGameModeParams>>,
    #[serde(default)]
    pub sipickup_module: Vec<Option<SIPickupModule>>,
    #[serde(default)]
    pub sidamage_handling_module: Vec<Option<SIDamageHandlingModule>>,
    #[serde(default)]
    pub sihostility_module: Vec<Option<SIHostilityModule>>,
    #[serde(default)]
    pub sispectator_module: Vec<Option<SISpectatorModule>>,
    #[serde(default)]
    pub siscoring_module: Vec<Option<SIScoringModule>>,
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
    pub sivictory_conditions_module: Vec<Option<SIVictoryConditionsModule>>,
    #[serde(default)]
    pub siparams_module: Vec<Option<SIParamsModule>>,
    #[serde(default)]
    pub sisubsumption_mission_module: Vec<Option<SISubsumptionMissionModule>>,
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
    pub sitem_type_filter: Vec<Option<SItemTypeFilter>>,
    #[serde(default)]
    pub sinitial_damage_specifier_base: Vec<Option<SInitialDamageSpecifierBase>>,
    #[serde(default)]
    pub sinitial_damage: Vec<Option<SInitialDamage>>,
    #[serde(default)]
    pub sinteraction_params: Vec<Option<SInteractionParams>>,
    #[serde(default)]
    pub sinteraction_point_primitive_params: Vec<Option<SInteractionPointPrimitiveParams>>,
    #[serde(default)]
    pub sinteraction_point_params: Vec<Option<SInteractionPointParams>>,
    #[serde(default)]
    pub sinteraction_point_modifier: Vec<Option<SInteractionPointModifier>>,
    #[serde(default)]
    pub sitem_port_def_types: Vec<Option<SItemPortDefTypes>>,
    #[serde(default)]
    pub signature_system_global_params: Vec<Option<SignatureSystemGlobalParams>>,
    #[serde(default)]
    pub signature_uiglobal_params: Vec<Option<SignatureUIGlobalParams>>,
    #[serde(default)]
    pub signature_type_global_params: Vec<Option<SignatureTypeGlobalParams>>,
    #[serde(default)]
    pub sitem_shop_reference: Vec<Option<SItemShopReference>>,
    #[serde(default)]
    pub sitem_shop_arparams: Vec<Option<SItemShopARParams>>,
    #[serde(default)]
    pub simple_sprite_sheet: Vec<Option<SimpleSpriteSheet>>,
    #[serde(default)]
    pub simple_sprite_slot: Vec<Option<SimpleSpriteSlot>>,
}

/// Bucket pool sub-struct for types in the `sj` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSj {
    #[serde(default)]
    pub sjump_tunnel_visual_params: Vec<Option<SJumpTunnelVisualParams>>,
    #[serde(default)]
    pub sjump_point_push_area_params: Vec<Option<SJumpPointPushAreaParams>>,
    #[serde(default)]
    pub sjump_tunnel_section_probability_params: Vec<Option<SJumpTunnelSectionProbabilityParams>>,
    #[serde(default)]
    pub sjump_tunnel_section_control_point_generation_params: Vec<Option<SJumpTunnelSectionControlPointGenerationParams>>,
    #[serde(default)]
    pub sjump_tunnel_obstacle_generation_params: Vec<Option<SJumpTunnelObstacleGenerationParams>>,
    #[serde(default)]
    pub sjump_tunnel_elliptical_params: Vec<Option<SJumpTunnelEllipticalParams>>,
    #[serde(default)]
    pub sjump_tunnel_section_generation_params: Vec<Option<SJumpTunnelSectionGenerationParams>>,
    #[serde(default)]
    pub sjump_tunnel_generation_params: Vec<Option<SJumpTunnelGenerationParams>>,
    #[serde(default)]
    pub sjump_tunnel_distortion_params: Vec<Option<SJumpTunnelDistortionParams>>,
    #[serde(default)]
    pub sjump_tunnel_failure_params: Vec<Option<SJumpTunnelFailureParams>>,
    #[serde(default)]
    pub sjump_tunnel_exit_params: Vec<Option<SJumpTunnelExitParams>>,
    #[serde(default)]
    pub sjump_drive_flight_rotation_params: Vec<Option<SJumpDriveFlightRotationParams>>,
    #[serde(default)]
    pub sjump_drive_flight_linear_params: Vec<Option<SJumpDriveFlightLinearParams>>,
    #[serde(default)]
    pub sjump_drive_flight_steering_params: Vec<Option<SJumpDriveFlightSteeringParams>>,
    #[serde(default)]
    pub sjump_drive_flight_turbulence_noise_params: Vec<Option<SJumpDriveFlightTurbulenceNoiseParams>>,
    #[serde(default)]
    pub sjump_drive_flight_turbulence_params: Vec<Option<SJumpDriveFlightTurbulenceParams>>,
}

/// Bucket pool sub-struct for types in the `sk` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSk {
    #[serde(default)]
    pub skill_definitions: Vec<Option<SkillDefinitions>>,
    #[serde(default)]
    pub skill: Vec<Option<Skill>>,
    #[serde(default)]
    pub skin_interactable_template: Vec<Option<SkinInteractableTemplate>>,
    #[serde(default)]
    pub skin_interactable_templates: Vec<Option<SkinInteractableTemplates>>,
}

/// Bucket pool sub-struct for types in the `sl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSl {
    #[serde(default)]
    pub slocomotion_personality_state_filter: Vec<Option<SLocomotionPersonalityStateFilter>>,
    #[serde(default)]
    pub sloadout_inventory_item: Vec<Option<SLoadoutInventoryItem>>,
    #[serde(default)]
    pub sloadout_requirement_base: Vec<Option<SLoadoutRequirementBase>>,
    #[serde(default)]
    pub sloadout_assortment: Vec<Option<SLoadoutAssortment>>,
    #[serde(default)]
    pub sloading_screen_information_def: Vec<Option<SLoadingScreenInformationDef>>,
    #[serde(default)]
    pub slocal_player_shopping_data: Vec<Option<SLocalPlayerShoppingData>>,
    #[serde(default)]
    pub slocal_player_shopping_predefined_arparams: Vec<Option<SLocalPlayerShoppingPredefinedARParams>>,
    #[serde(default)]
    pub slocal_player_shopping_notification_configuration: Vec<Option<SLocalPlayerShoppingNotificationConfiguration>>,
}

/// Bucket pool sub-struct for types in the `sm` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSm {
    #[serde(default)]
    pub smannequin_action_def_record: Vec<Option<SMannequinActionDefRecord>>,
    #[serde(default)]
    pub smisfire_effect: Vec<Option<SMisfireEffect>>,
    #[serde(default)]
    pub smining_targeting: Vec<Option<SMiningTargeting>>,
    #[serde(default)]
    pub smaster_mode_labels: Vec<Option<SMasterModeLabels>>,
    #[serde(default)]
    pub smaterial_node_params: Vec<Option<SMaterialNodeParams>>,
    #[serde(default)]
    pub smega_map_solar_system: Vec<Option<SMegaMapSolarSystem>>,
    #[serde(default)]
    pub smission_location_module: Vec<Option<SMissionLocationModule>>,
    #[serde(default)]
    pub smovable_limits: Vec<Option<SMovableLimits>>,
    #[serde(default)]
    pub smfdoperator_mode_config: Vec<Option<SMFDOperatorModeConfig>>,
    #[serde(default)]
    pub smfdview_exception: Vec<Option<SMFDViewException>>,
    #[serde(default)]
    pub smfdmaster_mode_view_exceptions: Vec<Option<SMFDMasterModeViewExceptions>>,
    #[serde(default)]
    pub smfdmode_config: Vec<Option<SMFDModeConfig>>,
    #[serde(default)]
    pub smanufacturer_mfdview: Vec<Option<SManufacturerMFDView>>,
    #[serde(default)]
    pub smfdview: Vec<Option<SMFDView>>,
    #[serde(default)]
    pub smaster_mode_mfdview_list: Vec<Option<SMasterModeMFDViewList>>,
    #[serde(default)]
    pub smfdview_list: Vec<Option<SMFDViewList>>,
    #[serde(default)]
    pub smfdparams_diagnostics: Vec<Option<SMFDParamsDiagnostics>>,
    #[serde(default)]
    pub smobi_glas_app_link: Vec<Option<SMobiGlasAppLink>>,
    #[serde(default)]
    pub smobi_glas_app_params_base: Vec<Option<SMobiGlasAppParamsBase>>,
}

/// Bucket pool sub-struct for types in the `so` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSo {
    #[serde(default)]
    pub soperator_mode_labels: Vec<Option<SOperatorModeLabels>>,
}

/// Bucket pool sub-struct for types in the `sp` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSp {
    #[serde(default)]
    pub spine_bone: Vec<Option<SpineBone>>,
    #[serde(default)]
    pub splayer_role_shake_multipliers: Vec<Option<SPlayerRoleShakeMultipliers>>,
    #[serde(default)]
    pub speed_threshold: Vec<Option<SpeedThreshold>>,
    #[serde(default)]
    pub spawn_settings_inventory_item: Vec<Option<SpawnSettingsInventoryItem>>,
    #[serde(default)]
    pub sprecision_targeting_item_name: Vec<Option<SPrecisionTargetingItemName>>,
    #[serde(default)]
    pub splayer_scoring: Vec<Option<SPlayerScoring>>,
    #[serde(default)]
    pub speed_throttle_configuration: Vec<Option<SpeedThrottleConfiguration>>,
    #[serde(default)]
    pub spvpbounty_contract_generators: Vec<Option<SPVPBountyContractGenerators>>,
    #[serde(default)]
    pub sperk_params_base: Vec<Option<SPerkParamsBase>>,
    #[serde(default)]
    pub sperk_reputation_params: Vec<Option<SPerkReputationParams>>,
    #[serde(default)]
    pub sperk_reputation_list_params: Vec<Option<SPerkReputationListParams>>,
    #[serde(default)]
    pub sprojected_pitch_ladder_params: Vec<Option<SProjectedPitchLadderParams>>,
    #[serde(default)]
    pub sprojected_yaw_line_params: Vec<Option<SProjectedYawLineParams>>,
    #[serde(default)]
    pub sprojected_display_params: Vec<Option<SProjectedDisplayParams>>,
    #[serde(default)]
    pub sprojected_hud_params: Vec<Option<SProjectedHudParams>>,
    #[serde(default)]
    pub spawn_description_entry: Vec<Option<SpawnDescriptionEntry>>,
    #[serde(default)]
    pub spawn_descriptions: Vec<Option<SpawnDescriptions>>,
    #[serde(default)]
    pub special_event_manufacturer: Vec<Option<SpecialEventManufacturer>>,
    #[serde(default)]
    pub special_event_day: Vec<Option<SpecialEventDay>>,
    #[serde(default)]
    pub special_event_database: Vec<Option<SpecialEventDatabase>>,
}

/// Bucket pool sub-struct for types in the `sq` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSq {
    #[serde(default)]
    pub squantum_drive_effect_params_legacy: Vec<Option<SQuantumDriveEffectParams_LEGACY>>,
    #[serde(default)]
    pub squantum_drive_effect_template: Vec<Option<SQuantumDriveEffectTemplate>>,
    #[serde(default)]
    pub squantum_camera_state_mapping_def: Vec<Option<SQuantumCameraStateMappingDef>>,
    #[serde(default)]
    pub squantum_camera_state_effects_def: Vec<Option<SQuantumCameraStateEffectsDef>>,
    #[serde(default)]
    pub squantum_camera_effects_def: Vec<Option<SQuantumCameraEffectsDef>>,
}

/// Bucket pool sub-struct for types in the `sr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSr {
    #[serde(default)]
    pub srgba8: Vec<Option<SRGBA8>>,
    #[serde(default)]
    pub srgb8: Vec<Option<SRGB8>>,
    #[serde(default)]
    pub srtpc_behaviour: Vec<Option<SRtpcBehaviour>>,
    #[serde(default)]
    pub sreputation_standing_params: Vec<Option<SReputationStandingParams>>,
    #[serde(default)]
    pub sreputation_standing_map_params: Vec<Option<SReputationStandingMapParams>>,
    #[serde(default)]
    pub sreputation_context_bbproperty_params: Vec<Option<SReputationContextBBPropertyParams>>,
    #[serde(default)]
    pub sreputation_scope_context_ui: Vec<Option<SReputationScopeContextUI>>,
    #[serde(default)]
    pub sreputation_context_ui: Vec<Option<SReputationContextUI>>,
    #[serde(default)]
    pub sreputation_context_bbentity_list_params: Vec<Option<SReputationContextBBEntityListParams>>,
    #[serde(default)]
    pub sreputation_global_context_bbparams: Vec<Option<SReputationGlobalContextBBParams>>,
    #[serde(default)]
    pub sreputation_state_params: Vec<Option<SReputationStateParams>>,
    #[serde(default)]
    pub sreputation_state_modifier_base: Vec<Option<SReputationStateModifierBase>>,
    #[serde(default)]
    pub sreputation_state_modifier_params: Vec<Option<SReputationStateModifierParams>>,
    #[serde(default)]
    pub sreputation_state_mission_result_modifier_list_params: Vec<Option<SReputationStateMissionResultModifierListParams>>,
    #[serde(default)]
    pub sreputation_state_mission_result_modifier_params: Vec<Option<SReputationStateMissionResultModifierParams>>,
    #[serde(default)]
    pub sreputation_scope_params: Vec<Option<SReputationScopeParams>>,
    #[serde(default)]
    pub sreputation_reward_amount: Vec<Option<SReputationRewardAmount>>,
    #[serde(default)]
    pub sreputation_amount_params: Vec<Option<SReputationAmountParams>>,
    #[serde(default)]
    pub sreputation_amount_list_params: Vec<Option<SReputationAmountListParams>>,
    #[serde(default)]
    pub sreputation_standing_reward_bonus_params: Vec<Option<SReputationStandingRewardBonusParams>>,
    #[serde(default)]
    pub sreputation_mission_giver_reward_bonus_params: Vec<Option<SReputationMissionGiverRewardBonusParams>>,
    #[serde(default)]
    pub sreputation_mission_reward_bonus_params: Vec<Option<SReputationMissionRewardBonusParams>>,
    #[serde(default)]
    pub sreputation_mission_requirement_expression_element: Vec<Option<SReputationMissionRequirementExpressionElement>>,
    #[serde(default)]
    pub sreputation_mission_requirements_params: Vec<Option<SReputationMissionRequirementsParams>>,
    #[serde(default)]
    pub sreputation_standing_journal_entry_params: Vec<Option<SReputationStandingJournalEntryParams>>,
    #[serde(default)]
    pub sreputation_journal_group_params: Vec<Option<SReputationJournalGroupParams>>,
    #[serde(default)]
    pub sreputation_journal_entries_params: Vec<Option<SReputationJournalEntriesParams>>,
    #[serde(default)]
    pub sreputation_journal_entry_handler_params: Vec<Option<SReputationJournalEntryHandlerParams>>,
    #[serde(default)]
    pub sresource_type_default_cargo_containers: Vec<Option<SResourceTypeDefaultCargoContainers>>,
    #[serde(default)]
    pub srecoil_modifier: Vec<Option<SRecoilModifier>>,
    #[serde(default)]
    pub sregen_consumer_modifier: Vec<Option<SRegenConsumerModifier>>,
    #[serde(default)]
    pub sresource_reward_multiplier: Vec<Option<SResourceRewardMultiplier>>,
    #[serde(default)]
    pub sreward_periodical: Vec<Option<SRewardPeriodical>>,
}

/// Bucket pool sub-struct for types in the `ss` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSs {
    #[serde(default)]
    pub sstatus_trigger_threshold_level_modifier: Vec<Option<SStatusTriggerThresholdLevelModifier>>,
    #[serde(default)]
    pub sstatus_fortitude_level_modifier: Vec<Option<SStatusFortitudeLevelModifier>>,
    #[serde(default)]
    pub sscan_targeting: Vec<Option<SScanTargeting>>,
    #[serde(default)]
    pub sscore_event: Vec<Option<SScoreEvent>>,
    #[serde(default)]
    pub ssoftbody_geometry_params: Vec<Option<SSoftbodyGeometryParams>>,
    #[serde(default)]
    pub ssalvage_repair_highlight_color_params: Vec<Option<SSalvageRepairHighlightColorParams>>,
    #[serde(default)]
    pub ssalvage_repair_highlight_outline_values: Vec<Option<SSalvageRepairHighlightOutlineValues>>,
    #[serde(default)]
    pub ssalvage_repair_highlight_params: Vec<Option<SSalvageRepairHighlightParams>>,
    #[serde(default)]
    pub ssalvage_repair_card_params: Vec<Option<SSalvageRepairCardParams>>,
    #[serde(default)]
    pub ssalvage_repair_item_type_localization_pair: Vec<Option<SSalvageRepairItemTypeLocalizationPair>>,
    #[serde(default)]
    pub ssalvage_repair_localization_params: Vec<Option<SSalvageRepairLocalizationParams>>,
    #[serde(default)]
    pub ssalvage_repair_material_params: Vec<Option<SSalvageRepairMaterialParams>>,
    #[serde(default)]
    pub ssalvage_repair_audio_params: Vec<Option<SSalvageRepairAudioParams>>,
    #[serde(default)]
    pub sshared_interaction_params: Vec<Option<SSharedInteractionParams>>,
    #[serde(default)]
    pub ssilhouette_params_def: Vec<Option<SSilhouetteParamsDef>>,
    #[serde(default)]
    pub ssalvage_global_structural_vfxparams: Vec<Option<SSalvageGlobalStructuralVFXParams>>,
    #[serde(default)]
    pub ssalvage_global_structural_highlight_params: Vec<Option<SSalvageGlobalStructuralHighlightParams>>,
    #[serde(default)]
    pub sspread_modifier: Vec<Option<SSpreadModifier>>,
    #[serde(default)]
    pub ssalvage_modifier: Vec<Option<SSalvageModifier>>,
    #[serde(default)]
    pub sscradar_contact_properites: Vec<Option<SSCRadarContactProperites>>,
    #[serde(default)]
    pub sscsignature_system_scan_bounds: Vec<Option<SSCSignatureSystemScanBounds>>,
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
    pub sscsignature_system_audio_rule: Vec<Option<SSCSignatureSystemAudioRule>>,
    #[serde(default)]
    pub sscsignature_system_audio_ruleset: Vec<Option<SSCSignatureSystemAudioRuleset>>,
    #[serde(default)]
    pub sscsignature_system_audio_params: Vec<Option<SSCSignatureSystemAudioParams>>,
    #[serde(default)]
    pub sscsignature_system_params: Vec<Option<SSCSignatureSystemParams>>,
    #[serde(default)]
    pub sscenario_progress_rewards_tiers: Vec<Option<SScenarioProgressRewardsTiers>>,
    #[serde(default)]
    pub sservice_beacon_creator_params_base: Vec<Option<SServiceBeaconCreatorParamsBase>>,
    #[serde(default)]
    pub sservice_beacon_creator_params: Vec<Option<SServiceBeaconCreatorParams>>,
    #[serde(default)]
    pub ssolar_system: Vec<Option<SSolarSystem>>,
    #[serde(default)]
    pub sscene_player_choice_settings: Vec<Option<SScenePlayerChoiceSettings>>,
}

/// Bucket pool sub-struct for types in the `st` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSt {
    #[serde(default)]
    pub stat_definitions: Vec<Option<StatDefinitions>>,
    #[serde(default)]
    pub stat: Vec<Option<Stat>>,
    #[serde(default)]
    pub stat_influence: Vec<Option<StatInfluence>>,
    #[serde(default)]
    pub stamina_cost: Vec<Option<StaminaCost>>,
    #[serde(default)]
    pub stamina_cost_params: Vec<Option<StaminaCostParams>>,
    #[serde(default)]
    pub status_effect_buff_macro: Vec<Option<StatusEffectBuffMacro>>,
    #[serde(default)]
    pub status_effect_value: Vec<Option<StatusEffectValue>>,
    #[serde(default)]
    pub status_effect_trigger: Vec<Option<StatusEffectTrigger>>,
    #[serde(default)]
    pub status_trigger_base: Vec<Option<StatusTriggerBase>>,
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
    pub status_cost: Vec<Option<StatusCost>>,
    #[serde(default)]
    pub status_buff_type_base: Vec<Option<StatusBuffTypeBase>>,
    #[serde(default)]
    pub status_effect_ability_lock: Vec<Option<StatusEffectAbilityLock>>,
    #[serde(default)]
    pub status_sweating_params: Vec<Option<StatusSweatingParams>>,
    #[serde(default)]
    pub status_blood_params: Vec<Option<StatusBloodParams>>,
    #[serde(default)]
    pub stance_breath_modifier: Vec<Option<StanceBreathModifier>>,
    #[serde(default)]
    pub stargeting_method_base: Vec<Option<STargetingMethodBase>>,
    #[serde(default)]
    pub stargeting_method_record: Vec<Option<STargetingMethodRecord>>,
    #[serde(default)]
    pub stargetable_item_type: Vec<Option<STargetableItemType>>,
    #[serde(default)]
    pub stargetable_item_types_record: Vec<Option<STargetableItemTypesRecord>>,
    #[serde(default)]
    pub starget_selector_color_highlighting: Vec<Option<STargetSelectorColorHighlighting>>,
    #[serde(default)]
    pub starget_selector_hud_params: Vec<Option<STargetSelectorHudParams>>,
    #[serde(default)]
    pub starget_selector_global_targeting_params: Vec<Option<STargetSelectorGlobalTargetingParams>>,
    #[serde(default)]
    pub sticky_filter_movement_params: Vec<Option<StickyFilterMovementParams>>,
    #[serde(default)]
    pub sticky_filter_rotation_params: Vec<Option<StickyFilterRotationParams>>,
    #[serde(default)]
    pub sticky_filter_autocenter_params: Vec<Option<StickyFilterAutocenterParams>>,
    #[serde(default)]
    pub steam_scoring: Vec<Option<STeamScoring>>,
    #[serde(default)]
    pub stractor_beam_holo_visual_params: Vec<Option<STractorBeamHoloVisualParams>>,
    #[serde(default)]
    pub stractor_beam_outline_visual_params: Vec<Option<STractorBeamOutlineVisualParams>>,
    #[serde(default)]
    pub stance_check_config: Vec<Option<StanceCheckConfig>>,
    #[serde(default)]
    pub sturret_health_modifier_def: Vec<Option<STurretHealthModifierDef>>,
    #[serde(default)]
    pub sturret_esp: Vec<Option<STurretESP>>,
    #[serde(default)]
    pub sturret_global_params: Vec<Option<STurretGlobalParams>>,
    #[serde(default)]
    pub stier_progressions: Vec<Option<STierProgressions>>,
    #[serde(default)]
    pub stier_reward: Vec<Option<STierReward>>,
    #[serde(default)]
    pub star_map_object_type: Vec<Option<StarMapObjectType>>,
    #[serde(default)]
    pub star_map_object_types: Vec<Option<StarMapObjectTypes>>,
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
    pub star_map_mission_object: Vec<Option<StarMapMissionObject>>,
    #[serde(default)]
    pub star_map_party_member_object: Vec<Option<StarMapPartyMemberObject>>,
    #[serde(default)]
    pub status_widget_display_preset: Vec<Option<StatusWidgetDisplayPreset>>,
}

/// Bucket pool sub-struct for types in the `su` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSu {
    #[serde(default)]
    pub surface_audio_properties_definition: Vec<Option<SurfaceAudioPropertiesDefinition>>,
    #[serde(default)]
    pub surface_audio_properties: Vec<Option<SurfaceAudioProperties>>,
    #[serde(default)]
    pub suggested_fovsetup: Vec<Option<SuggestedFOVSetup>>,
    #[serde(default)]
    pub sub_harvestable_config: Vec<Option<SubHarvestableConfig>>,
    #[serde(default)]
    pub sub_harvestable_config_record: Vec<Option<SubHarvestableConfigRecord>>,
    #[serde(default)]
    pub sub_harvestable_multi_config: Vec<Option<SubHarvestableMultiConfig>>,
    #[serde(default)]
    pub sub_harvestable_multi_config_record: Vec<Option<SubHarvestableMultiConfigRecord>>,
    #[serde(default)]
    pub sub_harvestable_config_base: Vec<Option<SubHarvestableConfigBase>>,
    #[serde(default)]
    pub sub_harvestable_config_single_base: Vec<Option<SubHarvestableConfigSingleBase>>,
    #[serde(default)]
    pub sub_harvestable_slot: Vec<Option<SubHarvestableSlot>>,
    #[serde(default)]
    pub suninsured_item: Vec<Option<SUninsuredItem>>,
}

/// Bucket pool sub-struct for types in the `sv` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSv {
    #[serde(default)]
    pub svibration_def: Vec<Option<SVibrationDef>>,
    #[serde(default)]
    pub svibration_suppression: Vec<Option<SVibrationSuppression>>,
    #[serde(default)]
    pub svibration_reference_data: Vec<Option<SVibrationReferenceData>>,
    #[serde(default)]
    pub svibration_vehicle_def: Vec<Option<SVibrationVehicleDef>>,
    #[serde(default)]
    pub sview_distance_ratio_params: Vec<Option<SViewDistanceRatioParams>>,
    #[serde(default)]
    pub svehicle_hud_params: Vec<Option<SVehicleHudParams>>,
    #[serde(default)]
    pub svehicle_ai_damage_modifiers: Vec<Option<SVehicleAiDamageModifiers>>,
    #[serde(default)]
    pub svehicle_afterburner_params: Vec<Option<SVehicleAfterburnerParams>>,
    #[serde(default)]
    pub svec_with_noise_params: Vec<Option<SVecWithNoiseParams>>,
}

/// Bucket pool sub-struct for types in the `sw` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSw {
    #[serde(default)]
    pub sweapon_procedural_head_recoil_curve_modifier_def: Vec<Option<SWeaponProceduralHeadRecoilCurveModifierDef>>,
    #[serde(default)]
    pub sweapon_stats: Vec<Option<SWeaponStats>>,
    #[serde(default)]
    pub sweapon_modifier_params: Vec<Option<SWeaponModifierParams>>,
    #[serde(default)]
    pub sweapon_misfire_entry: Vec<Option<SWeaponMisfireEntry>>,
    #[serde(default)]
    pub sweapon_procedural_hands_recoil_curve_config_def: Vec<Option<SWeaponProceduralHandsRecoilCurveConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_hands_recoil_config_def: Vec<Option<SWeaponProceduralHandsRecoilConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_aim_recoil_curve_config_def: Vec<Option<SWeaponProceduralAimRecoilCurveConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_aim_recoil_config_def: Vec<Option<SWeaponProceduralAimRecoilConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_body_recoil_config_def: Vec<Option<SWeaponProceduralBodyRecoilConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_head_recoil_curve_config_def: Vec<Option<SWeaponProceduralHeadRecoilCurveConfigDef>>,
    #[serde(default)]
    pub sweapon_procedural_head_recoil_config_def: Vec<Option<SWeaponProceduralHeadRecoilConfigDef>>,
}

/// Bucket pool sub-struct for types in the `sx` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSx {
    #[serde(default)]
    pub sxyzcurves_with_max_values_modifer: Vec<Option<SXYZCurvesWithMaxValuesModifer>>,
    #[serde(default)]
    pub sxyzcurves: Vec<Option<SXYZCurves>>,
    #[serde(default)]
    pub sxyzcurves_arrays: Vec<Option<SXYZCurvesArrays>>,
    #[serde(default)]
    pub sxyzcurves_with_max_values: Vec<Option<SXYZCurvesWithMaxValues>>,
}

/// Bucket pool sub-struct for types in the `sy` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsSy {
    #[serde(default)]
    pub syaw_pitch_roll_curves: Vec<Option<SYawPitchRollCurves>>,
}

/// Bucket pool sub-struct for types in the `ta` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsTa {
    #[serde(default)]
    pub tactic_scoring_profile: Vec<Option<TacticScoringProfile>>,
    #[serde(default)]
    pub tactic_player_distance: Vec<Option<TacticPlayerDistance>>,
    #[serde(default)]
    pub tag_trigger: Vec<Option<TagTrigger>>,
    #[serde(default)]
    pub target_tracking_auto_zoom_def: Vec<Option<TargetTrackingAutoZoomDef>>,
    #[serde(default)]
    pub tagged_sub_harvestable_config: Vec<Option<TaggedSubHarvestableConfig>>,
    #[serde(default)]
    pub tag_to_audio_rtpc_list: Vec<Option<TagToAudioRtpcList>>,
    #[serde(default)]
    pub tag_to_audio_rtpc: Vec<Option<TagToAudioRtpc>>,
    #[serde(default)]
    pub tag_search_term: Vec<Option<TagSearchTerm>>,
    #[serde(default)]
    pub tactical_query: Vec<Option<TacticalQuery>>,
    #[serde(default)]
    pub tag: Vec<Option<Tag>>,
    #[serde(default)]
    pub tag_database: Vec<Option<TagDatabase>>,
    #[serde(default)]
    pub tags_dnfterm: Vec<Option<TagsDNFTerm>>,
    #[serde(default)]
    pub tags_dnf: Vec<Option<TagsDNF>>,
    #[serde(default)]
    pub tag_list: Vec<Option<TagList>>,
    #[serde(default)]
    pub take_down_max_distances: Vec<Option<TakeDownMaxDistances>>,
    #[serde(default)]
    pub take_down_params: Vec<Option<TakeDownParams>>,
    #[serde(default)]
    pub take_down_config: Vec<Option<TakeDownConfig>>,
}

/// Bucket pool sub-struct for types in the `te` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsTe {
    #[serde(default)]
    pub temperature_uiparams: Vec<Option<TemperatureUIParams>>,
    #[serde(default)]
    pub temperature_damage_control: Vec<Option<TemperatureDamageControl>>,
    #[serde(default)]
    pub test_atomics: Vec<Option<TestAtomics>>,
    #[serde(default)]
    pub test_arrays: Vec<Option<TestArrays>>,
}

/// Bucket pool sub-struct for types in the `th` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsTh {
    #[serde(default)]
    pub throw_params: Vec<Option<ThrowParams>>,
}

/// Bucket pool sub-struct for types in the `ti` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsTi {
    #[serde(default)]
    pub time: Vec<Option<Time>>,
    #[serde(default)]
    pub time_value_base: Vec<Option<TimeValue_Base>>,
    #[serde(default)]
    pub tint_entry: Vec<Option<TintEntry>>,
    #[serde(default)]
    pub tint_palette: Vec<Option<TintPalette>>,
    #[serde(default)]
    pub tint_palette_tree: Vec<Option<TintPaletteTree>>,
    #[serde(default)]
    pub tint_palette_swizzle: Vec<Option<TintPaletteSwizzle>>,
    #[serde(default)]
    pub tint_palette_ref: Vec<Option<TintPaletteRef>>,
}

/// Bucket pool sub-struct for types in the `to` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsTo {
    #[serde(default)]
    pub toxic_gas_def: Vec<Option<ToxicGasDef>>,
    #[serde(default)]
    pub toxi_input_modifier_axis: Vec<Option<ToxiInputModifierAxis>>,
    #[serde(default)]
    pub toxi_input_modifier_distortion: Vec<Option<ToxiInputModifierDistortion>>,
    #[serde(default)]
    pub toxi_input_modifier_delay: Vec<Option<ToxiInputModifierDelay>>,
}

/// Bucket pool sub-struct for types in the `tq` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsTq {
    #[serde(default)]
    pub tqsinput: Vec<Option<TQSInput>>,
    #[serde(default)]
    pub tqsweight_input: Vec<Option<TQSWeightInput>>,
    #[serde(default)]
    pub tqsoption: Vec<Option<TQSOption>>,
    #[serde(default)]
    pub tqsoption_content_record: Vec<Option<TQSOptionContentRecord>>,
    #[serde(default)]
    pub tqsoption_content: Vec<Option<TQSOptionContent>>,
}

/// Bucket pool sub-struct for types in the `tr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsTr {
    #[serde(default)]
    pub traversal_cost_condition_tags: Vec<Option<TraversalCostConditionTags>>,
    #[serde(default)]
    pub traversal_cost_config: Vec<Option<TraversalCostConfig>>,
    #[serde(default)]
    pub trail_fading_settings: Vec<Option<TrailFadingSettings>>,
    #[serde(default)]
    pub transformation_interpolator_params: Vec<Option<TransformationInterpolatorParams>>,
    #[serde(default)]
    pub transformation_interpolator: Vec<Option<TransformationInterpolator>>,
    #[serde(default)]
    pub transit_station_announcements: Vec<Option<TransitStationAnnouncements>>,
    #[serde(default)]
    pub transit_station_announcement: Vec<Option<TransitStationAnnouncement>>,
}

/// Bucket pool sub-struct for types in the `tu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsTu {
    #[serde(default)]
    pub turret_input_deflection_time_params: Vec<Option<TurretInputDeflectionTimeParams>>,
}

/// Bucket pool sub-struct for types in the `ty` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsTy {
    #[serde(default)]
    pub type_subtype_params: Vec<Option<TypeSubtypeParams>>,
}

/// Bucket pool sub-struct for types in the `ui` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsUi {
    #[serde(default)]
    pub uiholo_vehicle_config: Vec<Option<UIHoloVehicle_Config>>,
    #[serde(default)]
    pub uiconfig: Vec<Option<UIConfig>>,
    #[serde(default)]
    pub uistate_color: Vec<Option<UIStateColor>>,
    #[serde(default)]
    pub uistate_color_threshold: Vec<Option<UIStateColor_Threshold>>,
    #[serde(default)]
    pub uimode_visibility_settings: Vec<Option<UIModeVisibilitySettings>>,
    #[serde(default)]
    pub uistate_display_threshold: Vec<Option<UIStateDisplay_Threshold>>,
    #[serde(default)]
    pub uistate_display: Vec<Option<UIStateDisplay>>,
    #[serde(default)]
    pub uielement: Vec<Option<UIElement>>,
    #[serde(default)]
    pub uielement_sounds_record: Vec<Option<UIElementSoundsRecord>>,
    #[serde(default)]
    pub uielement_sound_entry: Vec<Option<UIElementSoundEntry>>,
    #[serde(default)]
    pub uiaudio_event: Vec<Option<UIAudioEvent>>,
    #[serde(default)]
    pub uiaudio_parameter: Vec<Option<UIAudioParameter>>,
    #[serde(default)]
    pub uidialogue_event: Vec<Option<UIDialogueEvent>>,
    #[serde(default)]
    pub uiaudio_definition: Vec<Option<UIAudioDefinition>>,
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
    pub uidata_bank_display3_dspace_dust_params: Vec<Option<UIDataBankDisplay3DSpaceDustParams>>,
    #[serde(default)]
    pub uidata_bank_display3_dinterpolate_params: Vec<Option<UIDataBankDisplay3DInterpolateParams>>,
    #[serde(default)]
    pub uidata_bank_display3_dparams: Vec<Option<UIDataBankDisplay3DParams>>,
    #[serde(default)]
    pub uiworld_display_path_state_params: Vec<Option<UIWorldDisplayPathStateParams>>,
    #[serde(default)]
    pub uiworld_display_path_line_params: Vec<Option<UIWorldDisplayPathLineParams>>,
    #[serde(default)]
    pub uiworld_display_path_params: Vec<Option<UIWorldDisplayPathParams>>,
}

/// Bucket pool sub-struct for types in the `un` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsUn {
    #[serde(default)]
    pub unit_test_sub_record: Vec<Option<UnitTestSubRecord>>,
    #[serde(default)]
    pub unit_test_base_test: Vec<Option<UnitTest_BaseTest>>,
    #[serde(default)]
    pub unit_test_inheritance: Vec<Option<UnitTest_Inheritance>>,
    #[serde(default)]
    pub unit_test_override_defaults_test: Vec<Option<UnitTest_OverrideDefaultsTest>>,
    #[serde(default)]
    pub unit_test: Vec<Option<UnitTest>>,
}

/// Bucket pool sub-struct for types in the `us` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsUs {
    #[serde(default)]
    pub user_rtpc: Vec<Option<UserRTPC>>,
    #[serde(default)]
    pub usable_archetype: Vec<Option<UsableArchetype>>,
    #[serde(default)]
    pub use_channel_archetype: Vec<Option<UseChannelArchetype>>,
    #[serde(default)]
    pub usable_archetypes: Vec<Option<UsableArchetypes>>,
}

/// Bucket pool sub-struct for types in the `ve` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsVe {
    #[serde(default)]
    pub vec2: Vec<Option<Vec2>>,
    #[serde(default)]
    pub vec3: Vec<Option<Vec3>>,
    #[serde(default)]
    pub vehicle_difficulty_params: Vec<Option<VehicleDifficultyParams>>,
    #[serde(default)]
    pub vehicle_serial_number_character_type: Vec<Option<VehicleSerialNumberCharacterType>>,
    #[serde(default)]
    pub vehicle_serial_number_format: Vec<Option<VehicleSerialNumberFormat>>,
    #[serde(default)]
    pub vehicle_landing_gear_spring: Vec<Option<VehicleLandingGearSpring>>,
    #[serde(default)]
    pub vehicle_salvage_global_params: Vec<Option<VehicleSalvageGlobalParams>>,
    #[serde(default)]
    pub vehicle_landing_gear: Vec<Option<VehicleLandingGear>>,
    #[serde(default)]
    pub vehicle_landing_gear_system: Vec<Option<VehicleLandingGearSystem>>,
    #[serde(default)]
    pub vehicle_role: Vec<Option<VehicleRole>>,
    #[serde(default)]
    pub vehicle_career: Vec<Option<VehicleCareer>>,
    #[serde(default)]
    pub vehicle_career_list: Vec<Option<VehicleCareerList>>,
}

/// Bucket pool sub-struct for types in the `vi` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsVi {
    #[serde(default)]
    pub vibration_type_data: Vec<Option<VibrationTypeData>>,
    #[serde(default)]
    pub vibration_audio_entry: Vec<Option<VibrationAudioEntry>>,
    #[serde(default)]
    pub vibration_audio_point_def: Vec<Option<VibrationAudioPointDef>>,
    #[serde(default)]
    pub virtual_cursor_params: Vec<Option<VirtualCursorParams>>,
    #[serde(default)]
    pub virtual_cursor_hover_friction_params: Vec<Option<VirtualCursorHoverFrictionParams>>,
    #[serde(default)]
    pub virtual_cursor_wheel_params: Vec<Option<VirtualCursorWheelParams>>,
    #[serde(default)]
    pub visor_hud_config: Vec<Option<VisorHUD_Config>>,
    #[serde(default)]
    pub visor_control_hints_config: Vec<Option<Visor_ControlHintsConfig>>,
    #[serde(default)]
    pub video_comms: Vec<Option<VideoComms>>,
    #[serde(default)]
    pub video_comms_shader_params: Vec<Option<VideoCommsShaderParams>>,
    #[serde(default)]
    pub video_comms_audio_params: Vec<Option<VideoCommsAudioParams>>,
    #[serde(default)]
    pub visor_lens_layout: Vec<Option<VisorLens_Layout>>,
    #[serde(default)]
    pub visor_lens_region: Vec<Option<VisorLens_Region>>,
    #[serde(default)]
    pub visor_lens_widget: Vec<Option<VisorLens_Widget>>,
}

/// Bucket pool sub-struct for types in the `vo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsVo {
    #[serde(default)]
    pub voice_channel_settings_record: Vec<Option<VoiceChannelSettingsRecord>>,
    #[serde(default)]
    pub voice_single: Vec<Option<VoiceSingle>>,
    #[serde(default)]
    pub voice_bundle: Vec<Option<VoiceBundle>>,
}

/// Bucket pool sub-struct for types in the `wa` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsWa {
    #[serde(default)]
    pub walk_to_align_params: Vec<Option<WalkToAlignParams>>,
    #[serde(default)]
    pub water_interaction_effect_params: Vec<Option<WaterInteractionEffectParams>>,
    #[serde(default)]
    pub water_effects_global_params: Vec<Option<WaterEffectsGlobalParams>>,
}

/// Bucket pool sub-struct for types in the `we` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsWe {
    #[serde(default)]
    pub weighted_loot_archetype: Vec<Option<WeightedLootArchetype>>,
    #[serde(default)]
    pub weapon_aimable_angles_def: Vec<Option<WeaponAimableAnglesDef>>,
    #[serde(default)]
    pub weapon_gimbal_mode_modifier_def: Vec<Option<WeaponGimbalModeModifierDef>>,
    #[serde(default)]
    pub weapon_misfire_def: Vec<Option<WeaponMisfireDef>>,
    #[serde(default)]
    pub weapon_armodifier: Vec<Option<WeaponARModifier>>,
    #[serde(default)]
    pub web_rtccomms_call_projector_light_params: Vec<Option<WebRTCCommsCallProjectorLightParams>>,
    #[serde(default)]
    pub weapon_procedural_clips_set_up: Vec<Option<WeaponProceduralClipsSetUp>>,
    #[serde(default)]
    pub weapon_procedural_animation: Vec<Option<WeaponProceduralAnimation>>,
    #[serde(default)]
    pub weapon_procedural_clip: Vec<Option<WeaponProceduralClip>>,
    #[serde(default)]
    pub weapon_procedural_clip_base: Vec<Option<WeaponProceduralClipBase>>,
    #[serde(default)]
    pub weapon_procedural_recoil_config_def: Vec<Option<WeaponProceduralRecoilConfigDef>>,
    #[serde(default)]
    pub weather_effects_space_loop_effect: Vec<Option<WeatherEffects_SpaceLoopEffect>>,
    #[serde(default)]
    pub weather_effects_asteroid: Vec<Option<WeatherEffects_Asteroid>>,
    #[serde(default)]
    pub weather_effects_atmosphere: Vec<Option<WeatherEffects_Atmosphere>>,
    #[serde(default)]
    pub web_customization_debug: Vec<Option<WebCustomizationDebug>>,
    #[serde(default)]
    pub web_customization_item_type_name: Vec<Option<WebCustomizationItemTypeName>>,
    #[serde(default)]
    pub web_customization_global_params: Vec<Option<WebCustomizationGlobalParams>>,
}

/// Bucket pool sub-struct for types in the `wh` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsWh {
    #[serde(default)]
    pub wheel_audio_params: Vec<Option<WheelAudioParams>>,
    #[serde(default)]
    pub wheel_audio_surface_mapping: Vec<Option<WheelAudioSurfaceMapping>>,
    #[serde(default)]
    pub wheel_audio_surface_map: Vec<Option<WheelAudioSurfaceMap>>,
}

/// Bucket pool sub-struct for types in the `wo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsWo {
    #[serde(default)]
    pub world_display_radar: Vec<Option<WorldDisplayRadar>>,
    #[serde(default)]
    pub world_display_radar_icon: Vec<Option<WorldDisplayRadar_Icon>>,
    #[serde(default)]
    pub world_display_radar_line: Vec<Option<WorldDisplayRadar_Line>>,
    #[serde(default)]
    pub world_display_environment: Vec<Option<WorldDisplayEnvironment>>,
    #[serde(default)]
    pub world_display_environment_base: Vec<Option<WorldDisplayEnvironmentBase>>,
    #[serde(default)]
    pub world_display_environment_color: Vec<Option<WorldDisplayEnvironmentColor>>,
}

/// Bucket pool sub-struct for types in the `ze` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPoolsZe {
    #[serde(default)]
    pub zero_gtraversal_connection: Vec<Option<ZeroGTraversalConnection>>,
    #[serde(default)]
    pub zero_gtraversal_state: Vec<Option<ZeroGTraversalState>>,
    #[serde(default)]
    pub zero_gtraversal_graph: Vec<Option<ZeroGTraversalGraph>>,
}

/// Per-type `Vec<Option<T>>` storage for every generated DCB struct.
///
/// A freshly-reserved slot starts as `None` and is filled by the
/// [`Builder`](crate::Builder) drain loop. After a clean
/// `Builder::consume_database().finish()` every slot is `Some`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPools {
    #[serde(default)]
    pub b_ab: DataPoolsAb,
    #[serde(default)]
    pub b_ac: DataPoolsAc,
    #[serde(default)]
    pub b_ad: DataPoolsAd,
    #[serde(default)]
    pub b_ae: DataPoolsAe,
    #[serde(default)]
    pub b_ai: DataPoolsAi,
    #[serde(default)]
    pub b_am: DataPoolsAm,
    #[serde(default)]
    pub b_an: DataPoolsAn,
    #[serde(default)]
    pub b_ap: DataPoolsAp,
    #[serde(default)]
    pub b_ar: DataPoolsAr,
    #[serde(default)]
    pub b_as: DataPoolsAs,
    #[serde(default)]
    pub b_at: DataPoolsAt,
    #[serde(default)]
    pub b_au: DataPoolsAu,
    #[serde(default)]
    pub b_aw: DataPoolsAw,
    #[serde(default)]
    pub b_ba: DataPoolsBa,
    #[serde(default)]
    pub b_be: DataPoolsBe,
    #[serde(default)]
    pub b_bl: DataPoolsBl,
    #[serde(default)]
    pub b_bo: DataPoolsBo,
    #[serde(default)]
    pub b_br: DataPoolsBr,
    #[serde(default)]
    pub b_bu: DataPoolsBu,
    #[serde(default)]
    pub b_ca: DataPoolsCa,
    #[serde(default)]
    pub b_ch: DataPoolsCh,
    #[serde(default)]
    pub b_ci: DataPoolsCi,
    #[serde(default)]
    pub b_co: DataPoolsCo,
    #[serde(default)]
    pub b_cr: DataPoolsCr,
    #[serde(default)]
    pub b_ct: DataPoolsCt,
    #[serde(default)]
    pub b_cu: DataPoolsCu,
    #[serde(default)]
    pub b_cy: DataPoolsCy,
    #[serde(default)]
    pub b_da: DataPoolsDa,
    #[serde(default)]
    pub b_de: DataPoolsDe,
    #[serde(default)]
    pub b_di: DataPoolsDi,
    #[serde(default)]
    pub b_do: DataPoolsDo,
    #[serde(default)]
    pub b_dr: DataPoolsDr,
    #[serde(default)]
    pub b_du: DataPoolsDu,
    #[serde(default)]
    pub b_dy: DataPoolsDy,
    #[serde(default)]
    pub b_ea: DataPoolsEa,
    #[serde(default)]
    pub b_el: DataPoolsEl,
    #[serde(default)]
    pub b_em: DataPoolsEm,
    #[serde(default)]
    pub b_en: DataPoolsEn,
    #[serde(default)]
    pub b_es: DataPoolsEs,
    #[serde(default)]
    pub b_ev: DataPoolsEv,
    #[serde(default)]
    pub b_ex: DataPoolsEx,
    #[serde(default)]
    pub b_fa: DataPoolsFa,
    #[serde(default)]
    pub b_fi: DataPoolsFi,
    #[serde(default)]
    pub b_fl: DataPoolsFl,
    #[serde(default)]
    pub b_fo: DataPoolsFo,
    #[serde(default)]
    pub b_fp: DataPoolsFp,
    #[serde(default)]
    pub b_fr: DataPoolsFr,
    #[serde(default)]
    pub b_ga: DataPoolsGa,
    #[serde(default)]
    pub b_ge: DataPoolsGe,
    #[serde(default)]
    pub b_gf: DataPoolsGf,
    #[serde(default)]
    pub b_gl: DataPoolsGl,
    #[serde(default)]
    pub b_gp: DataPoolsGp,
    #[serde(default)]
    pub b_gr: DataPoolsGr,
    #[serde(default)]
    pub b_ha: DataPoolsHa,
    #[serde(default)]
    pub b_he: DataPoolsHe,
    #[serde(default)]
    pub b_hi: DataPoolsHi,
    #[serde(default)]
    pub b_ho: DataPoolsHo,
    #[serde(default)]
    pub b_hu: DataPoolsHu,
    #[serde(default)]
    pub b_hy: DataPoolsHy,
    #[serde(default)]
    pub b_if: DataPoolsIf,
    #[serde(default)]
    pub b_im: DataPoolsIm,
    #[serde(default)]
    pub b_in: DataPoolsIn,
    #[serde(default)]
    pub b_it: DataPoolsIt,
    #[serde(default)]
    pub b_jo: DataPoolsJo,
    #[serde(default)]
    pub b_ju: DataPoolsJu,
    #[serde(default)]
    pub b_la: DataPoolsLa,
    #[serde(default)]
    pub b_le: DataPoolsLe,
    #[serde(default)]
    pub b_li: DataPoolsLi,
    #[serde(default)]
    pub b_lo: DataPoolsLo,
    #[serde(default)]
    pub b_ma: DataPoolsMa,
    #[serde(default)]
    pub b_me: DataPoolsMe,
    #[serde(default)]
    pub b_mi: DataPoolsMi,
    #[serde(default)]
    pub b_mo: DataPoolsMo,
    #[serde(default)]
    pub b_mu: DataPoolsMu,
    #[serde(default)]
    pub b_no: DataPoolsNo,
    #[serde(default)]
    pub b_np: DataPoolsNp,
    #[serde(default)]
    pub b_nu: DataPoolsNu,
    #[serde(default)]
    pub b_ob: DataPoolsOb,
    #[serde(default)]
    pub b_oc: DataPoolsOc,
    #[serde(default)]
    pub b_op: DataPoolsOp,
    #[serde(default)]
    pub b_or: DataPoolsOr,
    #[serde(default)]
    pub b_pa: DataPoolsPa,
    #[serde(default)]
    pub b_pe: DataPoolsPe,
    #[serde(default)]
    pub b_pi: DataPoolsPi,
    #[serde(default)]
    pub b_pl: DataPoolsPl,
    #[serde(default)]
    pub b_po: DataPoolsPo,
    #[serde(default)]
    pub b_pr: DataPoolsPr,
    #[serde(default)]
    pub b_qt: DataPoolsQt,
    #[serde(default)]
    pub b_qu: DataPoolsQu,
    #[serde(default)]
    pub b_ra: DataPoolsRa,
    #[serde(default)]
    pub b_re: DataPoolsRe,
    #[serde(default)]
    pub b_rg: DataPoolsRg,
    #[serde(default)]
    pub b_ro: DataPoolsRo,
    #[serde(default)]
    pub b_rt: DataPoolsRt,
    #[serde(default)]
    pub b_sa: DataPoolsSa,
    #[serde(default)]
    pub b_sb: DataPoolsSb,
    #[serde(default)]
    pub b_sc: DataPoolsSc,
    #[serde(default)]
    pub b_sd: DataPoolsSd,
    #[serde(default)]
    pub b_se: DataPoolsSe,
    #[serde(default)]
    pub b_sg: DataPoolsSg,
    #[serde(default)]
    pub b_sh: DataPoolsSh,
    #[serde(default)]
    pub b_si: DataPoolsSi,
    #[serde(default)]
    pub b_sj: DataPoolsSj,
    #[serde(default)]
    pub b_sk: DataPoolsSk,
    #[serde(default)]
    pub b_sl: DataPoolsSl,
    #[serde(default)]
    pub b_sm: DataPoolsSm,
    #[serde(default)]
    pub b_so: DataPoolsSo,
    #[serde(default)]
    pub b_sp: DataPoolsSp,
    #[serde(default)]
    pub b_sq: DataPoolsSq,
    #[serde(default)]
    pub b_sr: DataPoolsSr,
    #[serde(default)]
    pub b_ss: DataPoolsSs,
    #[serde(default)]
    pub b_st: DataPoolsSt,
    #[serde(default)]
    pub b_su: DataPoolsSu,
    #[serde(default)]
    pub b_sv: DataPoolsSv,
    #[serde(default)]
    pub b_sw: DataPoolsSw,
    #[serde(default)]
    pub b_sx: DataPoolsSx,
    #[serde(default)]
    pub b_sy: DataPoolsSy,
    #[serde(default)]
    pub b_ta: DataPoolsTa,
    #[serde(default)]
    pub b_te: DataPoolsTe,
    #[serde(default)]
    pub b_th: DataPoolsTh,
    #[serde(default)]
    pub b_ti: DataPoolsTi,
    #[serde(default)]
    pub b_to: DataPoolsTo,
    #[serde(default)]
    pub b_tq: DataPoolsTq,
    #[serde(default)]
    pub b_tr: DataPoolsTr,
    #[serde(default)]
    pub b_tu: DataPoolsTu,
    #[serde(default)]
    pub b_ty: DataPoolsTy,
    #[serde(default)]
    pub b_ui: DataPoolsUi,
    #[serde(default)]
    pub b_un: DataPoolsUn,
    #[serde(default)]
    pub b_us: DataPoolsUs,
    #[serde(default)]
    pub b_ve: DataPoolsVe,
    #[serde(default)]
    pub b_vi: DataPoolsVi,
    #[serde(default)]
    pub b_vo: DataPoolsVo,
    #[serde(default)]
    pub b_wa: DataPoolsWa,
    #[serde(default)]
    pub b_we: DataPoolsWe,
    #[serde(default)]
    pub b_wh: DataPoolsWh,
    #[serde(default)]
    pub b_wo: DataPoolsWo,
    #[serde(default)]
    pub b_ze: DataPoolsZe,
}
