// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Per-record-type `HashMap<CigGuid, Handle<T>>` lookups.
//!
//! Only top-level DCB record types get an entry here. Non-record
//! pool types live in `DataPools` only — they have no GUID.
//!
//! Composed of bucket sub-structs (`RecordIndexAb`, `RecordIndexSc1`,
//! …) mirroring `DataPools` and the `generated/types_*.rs` bucket
//! files. Same rationale as `DataPools`: keep each serde-derive
//! `visit_map` small enough that LLVM doesn't die on inlining.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;

use super::*;

/// Bucket record-index sub-struct for types in the `ac` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexAc {
    #[serde(default)]
    pub activity_data: HashMap<CigGuid, Handle<ActivityData>>,
    #[serde(default)]
    pub actor_ability_component: HashMap<CigGuid, Handle<ActorAbilityComponent>>,
    #[serde(default)]
    pub actor_ducking_params: HashMap<CigGuid, Handle<ActorDuckingParams>>,
    #[serde(default)]
    pub actor_environment_component: HashMap<CigGuid, Handle<ActorEnvironmentComponent>>,
    #[serde(default)]
    pub actor_gforce_component: HashMap<CigGuid, Handle<ActorGForceComponent>>,
    #[serde(default)]
    pub actor_gforce_head_bob: HashMap<CigGuid, Handle<ActorGForceHeadBob>>,
    #[serde(default)]
    pub actor_gforce_camera_effects: HashMap<CigGuid, Handle<ActorGForceCameraEffects>>,
    #[serde(default)]
    pub actor_locomotion_personality: HashMap<CigGuid, Handle<ActorLocomotionPersonality>>,
    #[serde(default)]
    pub actor_movement_modifiers: HashMap<CigGuid, Handle<ActorMovementModifiers>>,
    #[serde(default)]
    pub actor_movement_sets_config: HashMap<CigGuid, Handle<ActorMovementSetsConfig>>,
    #[serde(default)]
    pub actor_procedural_recoil_config: HashMap<CigGuid, Handle<ActorProceduralRecoilConfig>>,
    #[serde(default)]
    pub actor_procedural_recoil_modifiers: HashMap<CigGuid, Handle<ActorProceduralRecoilModifiers>>,
    #[serde(default)]
    pub actor_skeleton_config: HashMap<CigGuid, Handle<ActorSkeletonConfig>>,
    #[serde(default)]
    pub actor_sliding_params: HashMap<CigGuid, Handle<ActorSlidingParams>>,
    #[serde(default)]
    pub actor_speed_camera_effects: HashMap<CigGuid, Handle<ActorSpeedCameraEffects>>,
    #[serde(default)]
    pub actor_stamina_component: HashMap<CigGuid, Handle<ActorStaminaComponent>>,
    #[serde(default)]
    pub actor_stance_speeds_info: HashMap<CigGuid, Handle<ActorStanceSpeedsInfo>>,
    #[serde(default)]
    pub actor_stance_dimensions_info: HashMap<CigGuid, Handle<ActorStanceDimensionsInfo>>,
    #[serde(default)]
    pub actor_state_validation: HashMap<CigGuid, Handle<ActorStateValidation>>,
    #[serde(default)]
    pub actor_status_global_params: HashMap<CigGuid, Handle<ActorStatusGlobalParams>>,
    #[serde(default)]
    pub actor_status_component: HashMap<CigGuid, Handle<ActorStatusComponent>>,
    #[serde(default)]
    pub actor_zero_gtraversal_params: HashMap<CigGuid, Handle<ActorZeroGTraversalParams>>,
    #[serde(default)]
    pub actor_fovview_params: HashMap<CigGuid, Handle<ActorFOVViewParams>>,
    #[serde(default)]
    pub actor_look_ahead_vehicle: HashMap<CigGuid, Handle<ActorLookAheadVehicle>>,
    #[serde(default)]
    pub actor_default_actions_config: HashMap<CigGuid, Handle<ActorDefaultActionsConfig>>,
    #[serde(default)]
    pub actor_targeted_params: HashMap<CigGuid, Handle<ActorTargetedParams>>,
    #[serde(default)]
    pub actor_stance_config: HashMap<CigGuid, Handle<ActorStanceConfig>>,
    #[serde(default)]
    pub actor_restrain_config: HashMap<CigGuid, Handle<ActorRestrainConfig>>,
    #[serde(default)]
    pub actor_view_limit_preset_database: HashMap<CigGuid, Handle<ActorViewLimitPresetDatabase>>,
    #[serde(default)]
    pub actor_look_limits: HashMap<CigGuid, Handle<ActorLookLimits>>,
    #[serde(default)]
    pub actor_aim_limits: HashMap<CigGuid, Handle<ActorAimLimits>>,
}

impl RecordIndexAc {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.activity_data.len()
            + self.actor_ability_component.len()
            + self.actor_ducking_params.len()
            + self.actor_environment_component.len()
            + self.actor_gforce_component.len()
            + self.actor_gforce_head_bob.len()
            + self.actor_gforce_camera_effects.len()
            + self.actor_locomotion_personality.len()
            + self.actor_movement_modifiers.len()
            + self.actor_movement_sets_config.len()
            + self.actor_procedural_recoil_config.len()
            + self.actor_procedural_recoil_modifiers.len()
            + self.actor_skeleton_config.len()
            + self.actor_sliding_params.len()
            + self.actor_speed_camera_effects.len()
            + self.actor_stamina_component.len()
            + self.actor_stance_speeds_info.len()
            + self.actor_stance_dimensions_info.len()
            + self.actor_state_validation.len()
            + self.actor_status_global_params.len()
            + self.actor_status_component.len()
            + self.actor_zero_gtraversal_params.len()
            + self.actor_fovview_params.len()
            + self.actor_look_ahead_vehicle.len()
            + self.actor_default_actions_config.len()
            + self.actor_targeted_params.len()
            + self.actor_stance_config.len()
            + self.actor_restrain_config.len()
            + self.actor_view_limit_preset_database.len()
            + self.actor_look_limits.len()
            + self.actor_aim_limits.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ai` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexAi {
    #[serde(default)]
    pub aiperception_profile: HashMap<CigGuid, Handle<AIPerceptionProfile>>,
    #[serde(default)]
    pub aimercy_timer_settings: HashMap<CigGuid, Handle<AIMercyTimerSettings>>,
    #[serde(default)]
    pub aivisual_field_params: HashMap<CigGuid, Handle<AIVisualFieldParams>>,
    #[serde(default)]
    pub aivisual_field_profile: HashMap<CigGuid, Handle<AIVisualFieldProfile>>,
    #[serde(default)]
    pub aiobservable_filter_flags: HashMap<CigGuid, Handle<AIObservableFilterFlags>>,
    #[serde(default)]
    pub aiobservable_filters_profile: HashMap<CigGuid, Handle<AIObservableFiltersProfile>>,
    #[serde(default)]
    pub aitargetable_settings: HashMap<CigGuid, Handle<AITargetableSettings>>,
    #[serde(default)]
    pub aispecial_ranged_attack_config: HashMap<CigGuid, Handle<AISpecialRangedAttackConfig>>,
    #[serde(default)]
    pub aiavailable_special_ranged_attacks_config: HashMap<CigGuid, Handle<AIAvailableSpecialRangedAttacksConfig>>,
    #[serde(default)]
    pub aifire_discipline_settings: HashMap<CigGuid, Handle<AIFireDisciplineSettings>>,
    #[serde(default)]
    pub aimotive_list: HashMap<CigGuid, Handle<AIMotiveList>>,
    #[serde(default)]
    pub aiprofile: HashMap<CigGuid, Handle<AIProfile>>,
    #[serde(default)]
    pub aitargeting_formula_settings: HashMap<CigGuid, Handle<AITargetingFormulaSettings>>,
    #[serde(default)]
    pub aiwave_collection: HashMap<CigGuid, Handle<AIWaveCollection>>,
    #[serde(default)]
    pub aimelee_combat_config: HashMap<CigGuid, Handle<AIMeleeCombatConfig>>,
}

impl RecordIndexAi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.aiperception_profile.len()
            + self.aimercy_timer_settings.len()
            + self.aivisual_field_params.len()
            + self.aivisual_field_profile.len()
            + self.aiobservable_filter_flags.len()
            + self.aiobservable_filters_profile.len()
            + self.aitargetable_settings.len()
            + self.aispecial_ranged_attack_config.len()
            + self.aiavailable_special_ranged_attacks_config.len()
            + self.aifire_discipline_settings.len()
            + self.aimotive_list.len()
            + self.aiprofile.len()
            + self.aitargeting_formula_settings.len()
            + self.aiwave_collection.len()
            + self.aimelee_combat_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `am` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexAm {
    #[serde(default)]
    pub ammo_params: HashMap<CigGuid, Handle<AmmoParams>>,
}

impl RecordIndexAm {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.ammo_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `an` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexAn {
    #[serde(default)]
    pub animated_marker: HashMap<CigGuid, Handle<AnimatedMarker>>,
    #[serde(default)]
    pub announcer: HashMap<CigGuid, Handle<Announcer>>,
    #[serde(default)]
    pub animated_helmet_params: HashMap<CigGuid, Handle<AnimatedHelmetParams>>,
}

impl RecordIndexAn {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.animated_marker.len()
            + self.announcer.len()
            + self.animated_helmet_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ar` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexAr {
    #[serde(default)]
    pub armarker_global_params: HashMap<CigGuid, Handle<ARMarkerGlobalParams>>,
    #[serde(default)]
    pub area_services: HashMap<CigGuid, Handle<AreaServices>>,
    #[serde(default)]
    pub armode_settings: HashMap<CigGuid, Handle<ARModeSettings>>,
    #[serde(default)]
    pub armor_move_view_restrictions: HashMap<CigGuid, Handle<ArmorMoveViewRestrictions>>,
}

impl RecordIndexAr {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.armarker_global_params.len()
            + self.area_services.len()
            + self.armode_settings.len()
            + self.armor_move_view_restrictions.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `as` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexAs {
    #[serde(default)]
    pub asteroid_field_composition: HashMap<CigGuid, Handle<AsteroidFieldComposition>>,
    #[serde(default)]
    pub asteroid_state_template: HashMap<CigGuid, Handle<AsteroidStateTemplate>>,
    #[serde(default)]
    pub asteroid_behavior: HashMap<CigGuid, Handle<AsteroidBehavior>>,
}

impl RecordIndexAs {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.asteroid_field_composition.len()
            + self.asteroid_state_template.len()
            + self.asteroid_behavior.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `at` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexAt {
    #[serde(default)]
    pub atmospheric_flight_effects: HashMap<CigGuid, Handle<AtmosphericFlightEffects>>,
    #[serde(default)]
    pub atmospheric_composition_template: HashMap<CigGuid, Handle<AtmosphericCompositionTemplate>>,
    #[serde(default)]
    pub atmosphere_state_template: HashMap<CigGuid, Handle<AtmosphereStateTemplate>>,
    #[serde(default)]
    pub atmosphere_state_pressure_template: HashMap<CigGuid, Handle<AtmosphereStatePressureTemplate>>,
    #[serde(default)]
    pub atmosphere_state_temperature_template: HashMap<CigGuid, Handle<AtmosphereStateTemperatureTemplate>>,
    #[serde(default)]
    pub atmosphere_state_humidity_template: HashMap<CigGuid, Handle<AtmosphereStateHumidityTemplate>>,
    #[serde(default)]
    pub atmosphere_behavior: HashMap<CigGuid, Handle<AtmosphereBehavior>>,
}

impl RecordIndexAt {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.atmospheric_flight_effects.len()
            + self.atmospheric_composition_template.len()
            + self.atmosphere_state_template.len()
            + self.atmosphere_state_pressure_template.len()
            + self.atmosphere_state_temperature_template.len()
            + self.atmosphere_state_humidity_template.len()
            + self.atmosphere_behavior.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `au` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexAu {
    #[serde(default)]
    pub audio_breath_style_condition: HashMap<CigGuid, Handle<AudioBreathStyleCondition>>,
    #[serde(default)]
    pub audio_breath_style_condition_list: HashMap<CigGuid, Handle<AudioBreathStyleConditionList>>,
    #[serde(default)]
    pub audio_breath_style: HashMap<CigGuid, Handle<AudioBreathStyle>>,
    #[serde(default)]
    pub audio_breath_style_suite: HashMap<CigGuid, Handle<AudioBreathStyleSuite>>,
    #[serde(default)]
    pub audio_breath_definition: HashMap<CigGuid, Handle<AudioBreathDefinition>>,
    #[serde(default)]
    pub audio_breath_interrupt: HashMap<CigGuid, Handle<AudioBreathInterrupt>>,
    #[serde(default)]
    pub audio_whitelist: HashMap<CigGuid, Handle<AudioWhitelist>>,
    #[serde(default)]
    pub audio_environment: HashMap<CigGuid, Handle<AudioEnvironment>>,
    #[serde(default)]
    pub audio_budget_definition: HashMap<CigGuid, Handle<AudioBudgetDefinition>>,
    #[serde(default)]
    pub audio_game_context_globals: HashMap<CigGuid, Handle<AudioGameContextGlobals>>,
    #[serde(default)]
    pub audio_game_context_setup: HashMap<CigGuid, Handle<AudioGameContextSetup>>,
    #[serde(default)]
    pub audio_tag_action_list: HashMap<CigGuid, Handle<AudioTagActionList>>,
    #[serde(default)]
    pub audio_value_output_setup: HashMap<CigGuid, Handle<AudioValueOutputSetup>>,
    #[serde(default)]
    pub audio_allegiance_switches: HashMap<CigGuid, Handle<AudioAllegianceSwitches>>,
    #[serde(default)]
    pub audio_environment_feedback_zone_setup: HashMap<CigGuid, Handle<AudioEnvironmentFeedbackZoneSetup>>,
    #[serde(default)]
    pub audio_environment_feedback_point_def: HashMap<CigGuid, Handle<AudioEnvironmentFeedbackPointDef>>,
    #[serde(default)]
    pub audio_hit_listener_definition: HashMap<CigGuid, Handle<AudioHitListenerDefinition>>,
    #[serde(default)]
    pub audio_signal_list: HashMap<CigGuid, Handle<AudioSignalList>>,
}

impl RecordIndexAu {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.audio_breath_style_condition.len()
            + self.audio_breath_style_condition_list.len()
            + self.audio_breath_style.len()
            + self.audio_breath_style_suite.len()
            + self.audio_breath_definition.len()
            + self.audio_breath_interrupt.len()
            + self.audio_whitelist.len()
            + self.audio_environment.len()
            + self.audio_budget_definition.len()
            + self.audio_game_context_globals.len()
            + self.audio_game_context_setup.len()
            + self.audio_tag_action_list.len()
            + self.audio_value_output_setup.len()
            + self.audio_allegiance_switches.len()
            + self.audio_environment_feedback_zone_setup.len()
            + self.audio_environment_feedback_point_def.len()
            + self.audio_hit_listener_definition.len()
            + self.audio_signal_list.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `aw` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexAw {
    #[serde(default)]
    pub award_service_config: HashMap<CigGuid, Handle<AwardService_Config>>,
}

impl RecordIndexAw {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.award_service_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `be` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexBe {
    #[serde(default)]
    pub beacons_contracts: HashMap<CigGuid, Handle<BeaconsContracts>>,
}

impl RecordIndexBe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.beacons_contracts.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `bl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexBl {
    #[serde(default)]
    pub blueprint_category_record: HashMap<CigGuid, Handle<BlueprintCategoryRecord>>,
    #[serde(default)]
    pub blueprint_category_database_record: HashMap<CigGuid, Handle<BlueprintCategoryDatabaseRecord>>,
    #[serde(default)]
    pub blueprint_pool_record: HashMap<CigGuid, Handle<BlueprintPoolRecord>>,
}

impl RecordIndexBl {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.blueprint_category_record.len()
            + self.blueprint_category_database_record.len()
            + self.blueprint_pool_record.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `bo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexBo {
    #[serde(default)]
    pub body_part: HashMap<CigGuid, Handle<BodyPart>>,
    #[serde(default)]
    pub body_mapping: HashMap<CigGuid, Handle<BodyMapping>>,
    #[serde(default)]
    pub body_health_config: HashMap<CigGuid, Handle<BodyHealthConfig>>,
}

impl RecordIndexBo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.body_part.len()
            + self.body_mapping.len()
            + self.body_health_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `br` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexBr {
    #[serde(default)]
    pub breathing_trigger_def: HashMap<CigGuid, Handle<BreathingTriggerDef>>,
}

impl RecordIndexBr {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.breathing_trigger_def.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `bu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexBu {
    #[serde(default)]
    pub building_blocks_timeline: HashMap<CigGuid, Handle<BuildingBlocks_Timeline>>,
    #[serde(default)]
    pub building_blocks_canvas: HashMap<CigGuid, Handle<BuildingBlocks_Canvas>>,
    #[serde(default)]
    pub building_blocks_font_style: HashMap<CigGuid, Handle<BuildingBlocks_FontStyle>>,
    #[serde(default)]
    pub building_blocks_language_specific_font_replacement: HashMap<CigGuid, Handle<BuildingBlocks_LanguageSpecificFontReplacement>>,
    #[serde(default)]
    pub building_blocks_style: HashMap<CigGuid, Handle<BuildingBlocks_Style>>,
    #[serde(default)]
    pub building_blocks_external_color_reference: HashMap<CigGuid, Handle<BuildingBlocks_ExternalColorReference>>,
    #[serde(default)]
    pub building_blocks_aspect_ratio_library: HashMap<CigGuid, Handle<BuildingBlocks_AspectRatioLibrary>>,
}

impl RecordIndexBu {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.building_blocks_timeline.len()
            + self.building_blocks_canvas.len()
            + self.building_blocks_font_style.len()
            + self.building_blocks_language_specific_font_replacement.len()
            + self.building_blocks_style.len()
            + self.building_blocks_external_color_reference.len()
            + self.building_blocks_aspect_ratio_library.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ca` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexCa {
    #[serde(default)]
    pub camera_lens_params: HashMap<CigGuid, Handle<CameraLensParams>>,
    #[serde(default)]
    pub camera: HashMap<CigGuid, Handle<Camera>>,
    #[serde(default)]
    pub camera_shop_config: HashMap<CigGuid, Handle<CameraShopConfig>>,
    #[serde(default)]
    pub camera_fovchange_data: HashMap<CigGuid, Handle<CameraFOVChangeData>>,
    #[serde(default)]
    pub cargo_manifest: HashMap<CigGuid, Handle<CargoManifest>>,
    #[serde(default)]
    pub carry_config: HashMap<CigGuid, Handle<CarryConfig>>,
    #[serde(default)]
    pub carryable_interactions_metadata_config_def: HashMap<CigGuid, Handle<CarryableInteractionsMetadataConfigDef>>,
    #[serde(default)]
    pub capacitor_assignment_input_output_def: HashMap<CigGuid, Handle<CapacitorAssignmentInputOutputDef>>,
    #[serde(default)]
    pub camera_transition_interpolation_curve_record: HashMap<CigGuid, Handle<CameraTransitionInterpolationCurveRecord>>,
}

impl RecordIndexCa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.camera_lens_params.len()
            + self.camera.len()
            + self.camera_shop_config.len()
            + self.camera_fovchange_data.len()
            + self.cargo_manifest.len()
            + self.carry_config.len()
            + self.carryable_interactions_metadata_config_def.len()
            + self.capacitor_assignment_input_output_def.len()
            + self.camera_transition_interpolation_curve_record.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ch` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexCh {
    #[serde(default)]
    pub character: HashMap<CigGuid, Handle<Character>>,
    #[serde(default)]
    pub character_serialization_settings_preset: HashMap<CigGuid, Handle<CharacterSerializationSettingsPreset>>,
    #[serde(default)]
    pub character_random_name_params: HashMap<CigGuid, Handle<CharacterRandomNameParams>>,
    #[serde(default)]
    pub chat_emote_record: HashMap<CigGuid, Handle<ChatEmoteRecord>>,
    #[serde(default)]
    pub chat_command_fast_access: HashMap<CigGuid, Handle<ChatCommandFastAccess>>,
    #[serde(default)]
    pub chat_filter_options: HashMap<CigGuid, Handle<ChatFilterOptions>>,
    #[serde(default)]
    pub chat_manager_global_params: HashMap<CigGuid, Handle<ChatManagerGlobalParams>>,
    #[serde(default)]
    pub chat_channel_filter_record: HashMap<CigGuid, Handle<ChatChannelFilterRecord>>,
}

impl RecordIndexCh {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.character.len()
            + self.character_serialization_settings_preset.len()
            + self.character_random_name_params.len()
            + self.chat_emote_record.len()
            + self.chat_command_fast_access.len()
            + self.chat_filter_options.len()
            + self.chat_manager_global_params.len()
            + self.chat_channel_filter_record.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ci` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexCi {
    #[serde(default)]
    pub cinematic_camera_controller_setup: HashMap<CigGuid, Handle<CinematicCameraControllerSetup>>,
    #[serde(default)]
    pub cinematic_conversation_settings: HashMap<CigGuid, Handle<CinematicConversationSettings>>,
    #[serde(default)]
    pub cinematic_flight_points_record: HashMap<CigGuid, Handle<CinematicFlightPointsRecord>>,
}

impl RecordIndexCi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.cinematic_camera_controller_setup.len()
            + self.cinematic_conversation_settings.len()
            + self.cinematic_flight_points_record.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `co` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexCo {
    #[serde(default)]
    pub combat_marker: HashMap<CigGuid, Handle<CombatMarker>>,
    #[serde(default)]
    pub cockpit_response: HashMap<CigGuid, Handle<CockpitResponse>>,
    #[serde(default)]
    pub cockpit_responses: HashMap<CigGuid, Handle<CockpitResponses>>,
    #[serde(default)]
    pub communication_config: HashMap<CigGuid, Handle<CommunicationConfig>>,
    #[serde(default)]
    pub communication_channel_config: HashMap<CigGuid, Handle<CommunicationChannelConfig>>,
    #[serde(default)]
    pub communication_variable_config: HashMap<CigGuid, Handle<CommunicationVariableConfig>>,
    #[serde(default)]
    pub communication_atlconfig: HashMap<CigGuid, Handle<CommunicationATLConfig>>,
    #[serde(default)]
    pub communication_auto_mannequin_tags_config: HashMap<CigGuid, Handle<CommunicationAutoMannequinTagsConfig>>,
    #[serde(default)]
    pub contextual_communication_config: HashMap<CigGuid, Handle<ContextualCommunicationConfig>>,
    #[serde(default)]
    pub communication_name: HashMap<CigGuid, Handle<CommunicationName>>,
    #[serde(default)]
    pub communication_channel_name: HashMap<CigGuid, Handle<CommunicationChannelName>>,
    #[serde(default)]
    pub conversation_sticky_filter: HashMap<CigGuid, Handle<ConversationStickyFilter>>,
    #[serde(default)]
    pub conversation: HashMap<CigGuid, Handle<Conversation>>,
    #[serde(default)]
    pub conversation_bank: HashMap<CigGuid, Handle<ConversationBank>>,
    #[serde(default)]
    pub constant_dofglobal_data: HashMap<CigGuid, Handle<ConstantDOFGlobalData>>,
    #[serde(default)]
    pub commodity_type: HashMap<CigGuid, Handle<CommodityType>>,
    #[serde(default)]
    pub commodity_subtype: HashMap<CigGuid, Handle<CommoditySubtype>>,
    #[serde(default)]
    pub commodity_type_database: HashMap<CigGuid, Handle<CommodityTypeDatabase>>,
    #[serde(default)]
    pub commodity_damage_configuration: HashMap<CigGuid, Handle<CommodityDamageConfiguration>>,
    #[serde(default)]
    pub control_hints_preset: HashMap<CigGuid, Handle<ControlHints_Preset>>,
    #[serde(default)]
    pub contract_generator: HashMap<CigGuid, Handle<ContractGenerator>>,
    #[serde(default)]
    pub contract_difficulty_profile: HashMap<CigGuid, Handle<ContractDifficultyProfile>>,
    #[serde(default)]
    pub contract_template: HashMap<CigGuid, Handle<ContractTemplate>>,
    #[serde(default)]
    pub comms_notification_stage: HashMap<CigGuid, Handle<CommsNotificationStage>>,
    #[serde(default)]
    pub comms_notification: HashMap<CigGuid, Handle<CommsNotification>>,
    #[serde(default)]
    pub comms_notifications_global_params: HashMap<CigGuid, Handle<CommsNotificationsGlobalParams>>,
    #[serde(default)]
    pub consumable_type: HashMap<CigGuid, Handle<ConsumableType>>,
    #[serde(default)]
    pub consumable_subtype: HashMap<CigGuid, Handle<ConsumableSubtype>>,
    #[serde(default)]
    pub consumable_type_database: HashMap<CigGuid, Handle<ConsumableTypeDatabase>>,
    #[serde(default)]
    pub comms_audio_effect: HashMap<CigGuid, Handle<CommsAudioEffect>>,
    #[serde(default)]
    pub corpse_interaction_params: HashMap<CigGuid, Handle<CorpseInteractionParams>>,
    #[serde(default)]
    pub comms_channel_def: HashMap<CigGuid, Handle<CommsChannelDef>>,
}

impl RecordIndexCo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.combat_marker.len()
            + self.cockpit_response.len()
            + self.cockpit_responses.len()
            + self.communication_config.len()
            + self.communication_channel_config.len()
            + self.communication_variable_config.len()
            + self.communication_atlconfig.len()
            + self.communication_auto_mannequin_tags_config.len()
            + self.contextual_communication_config.len()
            + self.communication_name.len()
            + self.communication_channel_name.len()
            + self.conversation_sticky_filter.len()
            + self.conversation.len()
            + self.conversation_bank.len()
            + self.constant_dofglobal_data.len()
            + self.commodity_type.len()
            + self.commodity_subtype.len()
            + self.commodity_type_database.len()
            + self.commodity_damage_configuration.len()
            + self.control_hints_preset.len()
            + self.contract_generator.len()
            + self.contract_difficulty_profile.len()
            + self.contract_template.len()
            + self.comms_notification_stage.len()
            + self.comms_notification.len()
            + self.comms_notifications_global_params.len()
            + self.consumable_type.len()
            + self.consumable_subtype.len()
            + self.consumable_type_database.len()
            + self.comms_audio_effect.len()
            + self.corpse_interaction_params.len()
            + self.comms_channel_def.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `cr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexCr {
    #[serde(default)]
    pub crafting_gameplay_property_def: HashMap<CigGuid, Handle<CraftingGameplayPropertyDef>>,
    #[serde(default)]
    pub crafting_blueprint_record: HashMap<CigGuid, Handle<CraftingBlueprintRecord>>,
    #[serde(default)]
    pub crafting_quality_distribution_record: HashMap<CigGuid, Handle<CraftingQualityDistributionRecord>>,
    #[serde(default)]
    pub crafting_quality_location_override_record: HashMap<CigGuid, Handle<CraftingQualityLocationOverrideRecord>>,
    #[serde(default)]
    pub crafting_global_params: HashMap<CigGuid, Handle<CraftingGlobalParams>>,
    #[serde(default)]
    pub crew_manifest: HashMap<CigGuid, Handle<CrewManifest>>,
}

impl RecordIndexCr {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.crafting_gameplay_property_def.len()
            + self.crafting_blueprint_record.len()
            + self.crafting_quality_distribution_record.len()
            + self.crafting_quality_location_override_record.len()
            + self.crafting_global_params.len()
            + self.crew_manifest.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ct` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexCt {
    #[serde(default)]
    pub ctx_graph: HashMap<CigGuid, Handle<CtxGraph>>,
}

impl RecordIndexCt {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.ctx_graph.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `da` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexDa {
    #[serde(default)]
    pub damage_macro: HashMap<CigGuid, Handle<DamageMacro>>,
    #[serde(default)]
    pub damage_resistance_macro: HashMap<CigGuid, Handle<DamageResistanceMacro>>,
    #[serde(default)]
    pub damage_map_global_params: HashMap<CigGuid, Handle<DamageMapGlobalParams>>,
}

impl RecordIndexDa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.damage_macro.len()
            + self.damage_resistance_macro.len()
            + self.damage_map_global_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `de` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexDe {
    #[serde(default)]
    pub default_entitlement_record: HashMap<CigGuid, Handle<DefaultEntitlementRecord>>,
    #[serde(default)]
    pub default_player_loadout_entitlement_record: HashMap<CigGuid, Handle<DefaultPlayerLoadoutEntitlementRecord>>,
    #[serde(default)]
    pub dematerialize_animation: HashMap<CigGuid, Handle<DematerializeAnimation>>,
    #[serde(default)]
    pub dev_owner: HashMap<CigGuid, Handle<DevOwner>>,
}

impl RecordIndexDe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.default_entitlement_record.len()
            + self.default_player_loadout_entitlement_record.len()
            + self.dematerialize_animation.len()
            + self.dev_owner.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `di` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexDi {
    #[serde(default)]
    pub dialogue_external_source: HashMap<CigGuid, Handle<DialogueExternalSource>>,
    #[serde(default)]
    pub dialogue_content: HashMap<CigGuid, Handle<DialogueContent>>,
    #[serde(default)]
    pub dialogue_content_bank: HashMap<CigGuid, Handle<DialogueContentBank>>,
    #[serde(default)]
    pub dialogue_context: HashMap<CigGuid, Handle<DialogueContext>>,
    #[serde(default)]
    pub dialogue_context_bank: HashMap<CigGuid, Handle<DialogueContextBank>>,
    #[serde(default)]
    pub dialogue_realm: HashMap<CigGuid, Handle<DialogueRealm>>,
    #[serde(default)]
    pub digital_signage_content_set: HashMap<CigGuid, Handle<DigitalSignageContentSet>>,
    #[serde(default)]
    pub direct_rtt_after_tonemapping_params: HashMap<CigGuid, Handle<DirectRTT_AfterTonemappingParams>>,
}

impl RecordIndexDi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.dialogue_external_source.len()
            + self.dialogue_content.len()
            + self.dialogue_content_bank.len()
            + self.dialogue_context.len()
            + self.dialogue_context_bank.len()
            + self.dialogue_realm.len()
            + self.digital_signage_content_set.len()
            + self.direct_rtt_after_tonemapping_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `do` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexDo {
    #[serde(default)]
    pub docking_slot_visibility: HashMap<CigGuid, Handle<DockingSlotVisibility>>,
    #[serde(default)]
    pub docking_sensitivity: HashMap<CigGuid, Handle<DockingSensitivity>>,
}

impl RecordIndexDo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.docking_slot_visibility.len()
            + self.docking_sensitivity.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `dy` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexDy {
    #[serde(default)]
    pub dynamic_camera_effects: HashMap<CigGuid, Handle<DynamicCameraEffects>>,
    #[serde(default)]
    pub dynamic_camera_effects_list: HashMap<CigGuid, Handle<DynamicCameraEffectsList>>,
}

impl RecordIndexDy {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.dynamic_camera_effects.len()
            + self.dynamic_camera_effects_list.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `el` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexEl {
    #[serde(default)]
    pub electrical_state_template: HashMap<CigGuid, Handle<ElectricalStateTemplate>>,
    #[serde(default)]
    pub electrical_behavior: HashMap<CigGuid, Handle<ElectricalBehavior>>,
}

impl RecordIndexEl {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.electrical_state_template.len()
            + self.electrical_behavior.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `em` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexEm {
    #[serde(default)]
    pub emotion_list: HashMap<CigGuid, Handle<EmotionList>>,
}

impl RecordIndexEm {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.emotion_list.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `en` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexEn {
    #[serde(default)]
    pub entity_audio_controller_type_params: HashMap<CigGuid, Handle<EntityAudioControllerTypeParams>>,
    #[serde(default)]
    pub entity_audio_controller_manager_params: HashMap<CigGuid, Handle<EntityAudioControllerManagerParams>>,
    #[serde(default)]
    pub entity_audio_controller_rtpc_subscriber_list_def: HashMap<CigGuid, Handle<EntityAudioControllerRtpcSubscriberListDef>>,
    #[serde(default)]
    pub entitlement_account_item_global_params: HashMap<CigGuid, Handle<EntitlementAccountItemGlobalParams>>,
    #[serde(default)]
    pub entitlement_non_inventory_storable_item_global_params: HashMap<CigGuid, Handle<EntitlementNonInventoryStorableItemGlobalParams>>,
    #[serde(default)]
    pub entity_class_definition: HashMap<CigGuid, Handle<EntityClassDefinition>>,
    #[serde(default)]
    pub entity_default_loadout_params: HashMap<CigGuid, Handle<EntityDefaultLoadoutParams>>,
    #[serde(default)]
    pub entity_cluster_id: HashMap<CigGuid, Handle<EntityClusterId>>,
    #[serde(default)]
    pub entity_cluster_member: HashMap<CigGuid, Handle<EntityClusterMember>>,
}

impl RecordIndexEn {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.entity_audio_controller_type_params.len()
            + self.entity_audio_controller_manager_params.len()
            + self.entity_audio_controller_rtpc_subscriber_list_def.len()
            + self.entitlement_account_item_global_params.len()
            + self.entitlement_non_inventory_storable_item_global_params.len()
            + self.entity_class_definition.len()
            + self.entity_default_loadout_params.len()
            + self.entity_cluster_id.len()
            + self.entity_cluster_member.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `es` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexEs {
    #[serde(default)]
    pub espparams: HashMap<CigGuid, Handle<ESPParams>>,
}

impl RecordIndexEs {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.espparams.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ev` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexEv {
    #[serde(default)]
    pub evagraph: HashMap<CigGuid, Handle<EVAGraph>>,
}

impl RecordIndexEv {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.evagraph.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ex` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexEx {
    #[serde(default)]
    pub explosive_ordnance_ping_global_params: HashMap<CigGuid, Handle<ExplosiveOrdnancePingGlobalParams>>,
}

impl RecordIndexEx {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.explosive_ordnance_ping_global_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `fa` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexFa {
    #[serde(default)]
    pub faction: HashMap<CigGuid, Handle<Faction>>,
    #[serde(default)]
    pub faction_palettes: HashMap<CigGuid, Handle<FactionPalettes>>,
    #[serde(default)]
    pub faction_palette: HashMap<CigGuid, Handle<FactionPalette>>,
    #[serde(default)]
    pub faction_reputation: HashMap<CigGuid, Handle<FactionReputation>>,
    #[serde(default)]
    pub faction_legacy: HashMap<CigGuid, Handle<Faction_LEGACY>>,
}

impl RecordIndexFa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.faction.len()
            + self.faction_palettes.len()
            + self.faction_palette.len()
            + self.faction_reputation.len()
            + self.faction_legacy.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `fi` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexFi {
    #[serde(default)]
    pub fidget_config: HashMap<CigGuid, Handle<FidgetConfig>>,
    #[serde(default)]
    pub fire_hazard_global_params: HashMap<CigGuid, Handle<FireHazardGlobalParams>>,
}

impl RecordIndexFi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.fidget_config.len()
            + self.fire_hazard_global_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `fl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexFl {
    #[serde(default)]
    pub flash_object_binding_group: HashMap<CigGuid, Handle<FlashObjectBindingGroup>>,
    #[serde(default)]
    pub flight_huduiview_config: HashMap<CigGuid, Handle<FlightHUDUIView_Config>>,
}

impl RecordIndexFl {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.flash_object_binding_group.len()
            + self.flight_huduiview_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `fo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexFo {
    #[serde(default)]
    pub foley_definition: HashMap<CigGuid, Handle<FoleyDefinition>>,
    #[serde(default)]
    pub foley_bone: HashMap<CigGuid, Handle<FoleyBone>>,
    #[serde(default)]
    pub foley_axis: HashMap<CigGuid, Handle<FoleyAxis>>,
    #[serde(default)]
    pub foley_footstep_definition: HashMap<CigGuid, Handle<FoleyFootstepDefinition>>,
    #[serde(default)]
    pub formation: HashMap<CigGuid, Handle<Formation>>,
    #[serde(default)]
    pub force_feedback: HashMap<CigGuid, Handle<ForceFeedback>>,
}

impl RecordIndexFo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.foley_definition.len()
            + self.foley_bone.len()
            + self.foley_axis.len()
            + self.foley_footstep_definition.len()
            + self.formation.len()
            + self.force_feedback.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `fr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexFr {
    #[serde(default)]
    pub friend_manager_global_params: HashMap<CigGuid, Handle<FriendManagerGlobalParams>>,
    #[serde(default)]
    pub frontend_override_params: HashMap<CigGuid, Handle<FrontendOverrideParams>>,
}

impl RecordIndexFr {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.friend_manager_global_params.len()
            + self.frontend_override_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ga` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexGa {
    #[serde(default)]
    pub game_module: HashMap<CigGuid, Handle<GameModule>>,
    #[serde(default)]
    pub game_mode: HashMap<CigGuid, Handle<GameMode>>,
    #[serde(default)]
    pub game_difficulty_modifiers: HashMap<CigGuid, Handle<GameDifficultyModifiers>>,
    #[serde(default)]
    pub game_notification_dock_item_params: HashMap<CigGuid, Handle<GameNotificationDockItemParams>>,
    #[serde(default)]
    pub gas_params: HashMap<CigGuid, Handle<GasParams>>,
}

impl RecordIndexGa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.game_module.len()
            + self.game_mode.len()
            + self.game_difficulty_modifiers.len()
            + self.game_notification_dock_item_params.len()
            + self.gas_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ge` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexGe {
    #[serde(default)]
    pub geom_font_config: HashMap<CigGuid, Handle<GeomFont_Config>>,
}

impl RecordIndexGe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.geom_font_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `gl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexGl {
    #[serde(default)]
    pub global_audio_settings: HashMap<CigGuid, Handle<GlobalAudioSettings>>,
    #[serde(default)]
    pub global_gas_cloud_vdbparams: HashMap<CigGuid, Handle<GlobalGasCloudVDBParams>>,
    #[serde(default)]
    pub global_shop_commodity_params: HashMap<CigGuid, Handle<GlobalShopCommodityParams>>,
    #[serde(default)]
    pub global_shop_terminal_params: HashMap<CigGuid, Handle<GlobalShopTerminalParams>>,
    #[serde(default)]
    pub global_shop_selling_params: HashMap<CigGuid, Handle<GlobalShopSellingParams>>,
    #[serde(default)]
    pub global_shop_buying_params: HashMap<CigGuid, Handle<GlobalShopBuyingParams>>,
    #[serde(default)]
    pub global_jump_point_params: HashMap<CigGuid, Handle<GlobalJumpPointParams>>,
    #[serde(default)]
    pub global_jump_tunnel_host_params: HashMap<CigGuid, Handle<GlobalJumpTunnelHostParams>>,
    #[serde(default)]
    pub global_jump_drive_params: HashMap<CigGuid, Handle<GlobalJumpDriveParams>>,
    #[serde(default)]
    pub global_cargo_loading_params: HashMap<CigGuid, Handle<GlobalCargoLoadingParams>>,
    #[serde(default)]
    pub global_marker_configs: HashMap<CigGuid, Handle<GlobalMarkerConfigs>>,
    #[serde(default)]
    pub global_mission_settings: HashMap<CigGuid, Handle<GlobalMissionSettings>>,
    #[serde(default)]
    pub global_gas_params: HashMap<CigGuid, Handle<GlobalGasParams>>,
    #[serde(default)]
    pub global_room_state_params: HashMap<CigGuid, Handle<GlobalRoomStateParams>>,
    #[serde(default)]
    pub global_tutorial_params: HashMap<CigGuid, Handle<GlobalTutorialParams>>,
}

impl RecordIndexGl {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.global_audio_settings.len()
            + self.global_gas_cloud_vdbparams.len()
            + self.global_shop_commodity_params.len()
            + self.global_shop_terminal_params.len()
            + self.global_shop_selling_params.len()
            + self.global_shop_buying_params.len()
            + self.global_jump_point_params.len()
            + self.global_jump_tunnel_host_params.len()
            + self.global_jump_drive_params.len()
            + self.global_cargo_loading_params.len()
            + self.global_marker_configs.len()
            + self.global_mission_settings.len()
            + self.global_gas_params.len()
            + self.global_room_state_params.len()
            + self.global_tutorial_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `gp` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexGp {
    #[serde(default)]
    pub gpuparticle_audio: HashMap<CigGuid, Handle<GPUParticleAudio>>,
    #[serde(default)]
    pub gpuparticle_audio_list: HashMap<CigGuid, Handle<GPUParticleAudioList>>,
}

impl RecordIndexGp {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.gpuparticle_audio.len()
            + self.gpuparticle_audio_list.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `gr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexGr {
    #[serde(default)]
    pub grab_camera_control_params: HashMap<CigGuid, Handle<GrabCameraControlParams>>,
    #[serde(default)]
    pub grip: HashMap<CigGuid, Handle<Grip>>,
}

impl RecordIndexGr {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.grab_camera_control_params.len()
            + self.grip.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ha` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexHa {
    #[serde(default)]
    pub handhold_grip_type: HashMap<CigGuid, Handle<HandholdGripType>>,
    #[serde(default)]
    pub handhold_grip_database: HashMap<CigGuid, Handle<HandholdGripDatabase>>,
    #[serde(default)]
    pub harvestable_preset: HashMap<CigGuid, Handle<HarvestablePreset>>,
    #[serde(default)]
    pub harvestable_setup: HashMap<CigGuid, Handle<HarvestableSetup>>,
    #[serde(default)]
    pub harvestable_cluster_preset: HashMap<CigGuid, Handle<HarvestableClusterPreset>>,
    #[serde(default)]
    pub harvestable_provider_preset: HashMap<CigGuid, Handle<HarvestableProviderPreset>>,
    #[serde(default)]
    pub hardware_mouse_params: HashMap<CigGuid, Handle<HardwareMouseParams>>,
    #[serde(default)]
    pub hauling_entity_classes: HashMap<CigGuid, Handle<Hauling_EntityClasses>>,
    #[serde(default)]
    pub hazard_awareness_params: HashMap<CigGuid, Handle<HazardAwarenessParams>>,
}

impl RecordIndexHa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.handhold_grip_type.len()
            + self.handhold_grip_database.len()
            + self.harvestable_preset.len()
            + self.harvestable_setup.len()
            + self.harvestable_cluster_preset.len()
            + self.harvestable_provider_preset.len()
            + self.hardware_mouse_params.len()
            + self.hauling_entity_classes.len()
            + self.hazard_awareness_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `he` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexHe {
    #[serde(default)]
    pub health_template: HashMap<CigGuid, Handle<HealthTemplate>>,
}

impl RecordIndexHe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.health_template.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `hi` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexHi {
    #[serde(default)]
    pub hint_uidata: HashMap<CigGuid, Handle<HintUIData>>,
    #[serde(default)]
    pub hint_trigger_data: HashMap<CigGuid, Handle<HintTriggerData>>,
}

impl RecordIndexHi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.hint_uidata.len()
            + self.hint_trigger_data.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ho` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexHo {
    #[serde(default)]
    pub hologram_params: HashMap<CigGuid, Handle<HologramParams>>,
}

impl RecordIndexHo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.hologram_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `hu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexHu {
    #[serde(default)]
    pub hud_colors: HashMap<CigGuid, Handle<HudColors>>,
}

impl RecordIndexHu {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.hud_colors.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `if` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexIf {
    #[serde(default)]
    pub ifcs_input_deflection_time_params: HashMap<CigGuid, Handle<IfcsInputDeflectionTimeParams>>,
}

impl RecordIndexIf {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.ifcs_input_deflection_time_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `in` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexIn {
    #[serde(default)]
    pub intoxication_ifcsmodifier_params: HashMap<CigGuid, Handle<IntoxicationIFCSModifierParams>>,
    #[serde(default)]
    pub intoxication_turret_modifier_params: HashMap<CigGuid, Handle<IntoxicationTurretModifierParams>>,
    #[serde(default)]
    pub intoxication_wheeled_modifier_params: HashMap<CigGuid, Handle<IntoxicationWheeledModifierParams>>,
    #[serde(default)]
    pub initial_damage_override: HashMap<CigGuid, Handle<InitialDamageOverride>>,
    #[serde(default)]
    pub inner_thought_anim: HashMap<CigGuid, Handle<InnerThought_Anim>>,
    #[serde(default)]
    pub inner_thought_color_params: HashMap<CigGuid, Handle<InnerThought_ColorParams>>,
    #[serde(default)]
    pub inner_thought_params: HashMap<CigGuid, Handle<InnerThought_Params>>,
    #[serde(default)]
    pub inner_thought_conversation_system_config: HashMap<CigGuid, Handle<InnerThought_ConversationSystemConfig>>,
    #[serde(default)]
    pub inner_thought_interaction_system_config: HashMap<CigGuid, Handle<InnerThought_InteractionSystemConfig>>,
    #[serde(default)]
    pub inner_thought_legacy_use_system_config: HashMap<CigGuid, Handle<InnerThought_LegacyUseSystemConfig>>,
    #[serde(default)]
    pub input_prompt_config: HashMap<CigGuid, Handle<InputPromptConfig>>,
    #[serde(default)]
    pub instanced_interior_location_params: HashMap<CigGuid, Handle<InstancedInteriorLocationParams>>,
    #[serde(default)]
    pub instanced_interior_location_map: HashMap<CigGuid, Handle<InstancedInteriorLocationMap>>,
    #[serde(default)]
    pub interaction_condition_preset: HashMap<CigGuid, Handle<InteractionConditionPreset>>,
    #[serde(default)]
    pub interaction_point_template: HashMap<CigGuid, Handle<InteractionPointTemplate>>,
    #[serde(default)]
    pub inventory_container_manager: HashMap<CigGuid, Handle<InventoryContainerManager>>,
    #[serde(default)]
    pub inventory_container: HashMap<CigGuid, Handle<InventoryContainer>>,
    #[serde(default)]
    pub infraction_definition: HashMap<CigGuid, Handle<InfractionDefinition>>,
    #[serde(default)]
    pub interior_map_section_definition: HashMap<CigGuid, Handle<InteriorMapSectionDefinition>>,
}

impl RecordIndexIn {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.intoxication_ifcsmodifier_params.len()
            + self.intoxication_turret_modifier_params.len()
            + self.intoxication_wheeled_modifier_params.len()
            + self.initial_damage_override.len()
            + self.inner_thought_anim.len()
            + self.inner_thought_color_params.len()
            + self.inner_thought_params.len()
            + self.inner_thought_conversation_system_config.len()
            + self.inner_thought_interaction_system_config.len()
            + self.inner_thought_legacy_use_system_config.len()
            + self.input_prompt_config.len()
            + self.instanced_interior_location_params.len()
            + self.instanced_interior_location_map.len()
            + self.interaction_condition_preset.len()
            + self.interaction_point_template.len()
            + self.inventory_container_manager.len()
            + self.inventory_container.len()
            + self.infraction_definition.len()
            + self.interior_map_section_definition.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `it` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexIt {
    #[serde(default)]
    pub item: HashMap<CigGuid, Handle<Item>>,
    #[serde(default)]
    pub item_kiosk_brand: HashMap<CigGuid, Handle<ItemKioskBrand>>,
    #[serde(default)]
    pub item_port_tags_dictionary: HashMap<CigGuid, Handle<ItemPortTagsDictionary>>,
    #[serde(default)]
    pub item_resource_network_global: HashMap<CigGuid, Handle<ItemResourceNetworkGlobal>>,
    #[serde(default)]
    pub item_award_weightings_record: HashMap<CigGuid, Handle<ItemAwardWeightingsRecord>>,
    #[serde(default)]
    pub item_recovery_configuration_params: HashMap<CigGuid, Handle<ItemRecoveryConfigurationParams>>,
    #[serde(default)]
    pub item_preview_config: HashMap<CigGuid, Handle<ItemPreview_Config>>,
    #[serde(default)]
    pub item_type_definition: HashMap<CigGuid, Handle<ItemTypeDefinition>>,
}

impl RecordIndexIt {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.item.len()
            + self.item_kiosk_brand.len()
            + self.item_port_tags_dictionary.len()
            + self.item_resource_network_global.len()
            + self.item_award_weightings_record.len()
            + self.item_recovery_configuration_params.len()
            + self.item_preview_config.len()
            + self.item_type_definition.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `jo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexJo {
    #[serde(default)]
    pub journal_entry_type: HashMap<CigGuid, Handle<JournalEntryType>>,
    #[serde(default)]
    pub journal_entry: HashMap<CigGuid, Handle<JournalEntry>>,
}

impl RecordIndexJo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.journal_entry_type.len()
            + self.journal_entry.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ju` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexJu {
    #[serde(default)]
    pub jump_fall_land_config: HashMap<CigGuid, Handle<JumpFallLandConfig>>,
    #[serde(default)]
    pub jump_drive_flight_params: HashMap<CigGuid, Handle<JumpDriveFlightParams>>,
    #[serde(default)]
    pub jump_tunnel_forces_params: HashMap<CigGuid, Handle<JumpTunnelForcesParams>>,
    #[serde(default)]
    pub jump_travel_camera_params: HashMap<CigGuid, Handle<JumpTravelCameraParams>>,
    #[serde(default)]
    pub jurisdiction: HashMap<CigGuid, Handle<Jurisdiction>>,
    #[serde(default)]
    pub jump_fall_land_params: HashMap<CigGuid, Handle<JumpFallLandParams>>,
    #[serde(default)]
    pub jump_thruster_pack_config: HashMap<CigGuid, Handle<JumpThrusterPackConfig>>,
}

impl RecordIndexJu {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.jump_fall_land_config.len()
            + self.jump_drive_flight_params.len()
            + self.jump_tunnel_forces_params.len()
            + self.jump_travel_camera_params.len()
            + self.jurisdiction.len()
            + self.jump_fall_land_params.len()
            + self.jump_thruster_pack_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `la` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexLa {
    #[serde(default)]
    pub ladder_config: HashMap<CigGuid, Handle<LadderConfig>>,
    #[serde(default)]
    pub landing_zone_inventory: HashMap<CigGuid, Handle<LandingZoneInventory>>,
    #[serde(default)]
    pub landing_pad_size: HashMap<CigGuid, Handle<LandingPadSize>>,
}

impl RecordIndexLa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.ladder_config.len()
            + self.landing_zone_inventory.len()
            + self.landing_pad_size.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `le` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexLe {
    #[serde(default)]
    pub lean_graph: HashMap<CigGuid, Handle<LeanGraph>>,
    #[serde(default)]
    pub legacy_crafting_recipe_def_record: HashMap<CigGuid, Handle<LegacyCraftingRecipeDefRecord>>,
    #[serde(default)]
    pub legacy_crafting_recipe_list_record: HashMap<CigGuid, Handle<LegacyCraftingRecipeListRecord>>,
    #[serde(default)]
    pub level: HashMap<CigGuid, Handle<Level>>,
    #[serde(default)]
    pub ledge_grabbing_params: HashMap<CigGuid, Handle<LedgeGrabbingParams>>,
}

impl RecordIndexLe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.lean_graph.len()
            + self.legacy_crafting_recipe_def_record.len()
            + self.legacy_crafting_recipe_list_record.len()
            + self.level.len()
            + self.ledge_grabbing_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `lo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexLo {
    #[serde(default)]
    pub local_player_speed_throttle_component: HashMap<CigGuid, Handle<LocalPlayerSpeedThrottleComponent>>,
    #[serde(default)]
    pub long_term_persistence_global_params: HashMap<CigGuid, Handle<LongTermPersistenceGlobalParams>>,
    #[serde(default)]
    pub loot_archetype: HashMap<CigGuid, Handle<LootArchetype>>,
    #[serde(default)]
    pub loot_table: HashMap<CigGuid, Handle<LootTable>>,
    #[serde(default)]
    pub loot_table_v3_record: HashMap<CigGuid, Handle<LootTableV3Record>>,
    #[serde(default)]
    pub loot_archetype_v3_record: HashMap<CigGuid, Handle<LootArchetypeV3Record>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_single_layer_record: HashMap<CigGuid, Handle<LootV3SecondaryChoicesSingleLayerRecord>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_multi_layer_record: HashMap<CigGuid, Handle<LootV3SecondaryChoicesMultiLayerRecord>>,
    #[serde(default)]
    pub loot_generation_global_params: HashMap<CigGuid, Handle<LootGenerationGlobalParams>>,
    #[serde(default)]
    pub location_resource_slot: HashMap<CigGuid, Handle<LocationResourceSlot>>,
    #[serde(default)]
    pub location_entity_declaration: HashMap<CigGuid, Handle<LocationEntityDeclaration>>,
    #[serde(default)]
    pub loadout_dummy_component_params: HashMap<CigGuid, Handle<LoadoutDummyComponentParams>>,
    #[serde(default)]
    pub loadout_editor_component_params: HashMap<CigGuid, Handle<LoadoutEditorComponentParams>>,
    #[serde(default)]
    pub loadout_kit: HashMap<CigGuid, Handle<LoadoutKit>>,
}

impl RecordIndexLo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.local_player_speed_throttle_component.len()
            + self.long_term_persistence_global_params.len()
            + self.loot_archetype.len()
            + self.loot_table.len()
            + self.loot_table_v3_record.len()
            + self.loot_archetype_v3_record.len()
            + self.loot_v3_secondary_choices_single_layer_record.len()
            + self.loot_v3_secondary_choices_multi_layer_record.len()
            + self.loot_generation_global_params.len()
            + self.location_resource_slot.len()
            + self.location_entity_declaration.len()
            + self.loadout_dummy_component_params.len()
            + self.loadout_editor_component_params.len()
            + self.loadout_kit.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ma` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexMa {
    #[serde(default)]
    pub master_mode_exclusion_global_params: HashMap<CigGuid, Handle<MasterModeExclusionGlobalParams>>,
    #[serde(default)]
    pub marker_tracking_view_mode_parameters: HashMap<CigGuid, Handle<MarkerTrackingViewModeParameters>>,
    #[serde(default)]
    pub marker_tracking_common_map_parameters: HashMap<CigGuid, Handle<MarkerTrackingCommonMapParameters>>,
    #[serde(default)]
    pub marker_decluttering_culling_order: HashMap<CigGuid, Handle<MarkerDeclutteringCullingOrder>>,
    #[serde(default)]
    pub marker_configuration: HashMap<CigGuid, Handle<Marker_Configuration>>,
    #[serde(default)]
    pub marker_ar_config_def: HashMap<CigGuid, Handle<MarkerAR_ConfigDef>>,
}

impl RecordIndexMa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.master_mode_exclusion_global_params.len()
            + self.marker_tracking_view_mode_parameters.len()
            + self.marker_tracking_common_map_parameters.len()
            + self.marker_decluttering_culling_order.len()
            + self.marker_configuration.len()
            + self.marker_ar_config_def.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `me` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexMe {
    #[serde(default)]
    pub medical_item_tier_config: HashMap<CigGuid, Handle<MedicalItemTierConfig>>,
    #[serde(default)]
    pub mega_map: HashMap<CigGuid, Handle<MegaMap>>,
    #[serde(default)]
    pub melee_combat_config: HashMap<CigGuid, Handle<MeleeCombatConfig>>,
}

impl RecordIndexMe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.medical_item_tier_config.len()
            + self.mega_map.len()
            + self.melee_combat_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `mi` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexMi {
    #[serde(default)]
    pub mining_global_params: HashMap<CigGuid, Handle<MiningGlobalParams>>,
    #[serde(default)]
    pub mining_audio_params: HashMap<CigGuid, Handle<MiningAudioParams>>,
    #[serde(default)]
    pub mineable_element: HashMap<CigGuid, Handle<MineableElement>>,
    #[serde(default)]
    pub mineable_composition: HashMap<CigGuid, Handle<MineableComposition>>,
    #[serde(default)]
    pub mining_laser_global_params: HashMap<CigGuid, Handle<MiningLaserGlobalParams>>,
    #[serde(default)]
    pub mission_location_template: HashMap<CigGuid, Handle<MissionLocationTemplate>>,
    #[serde(default)]
    pub mission_item: HashMap<CigGuid, Handle<MissionItem>>,
    #[serde(default)]
    pub mission_organization: HashMap<CigGuid, Handle<MissionOrganization>>,
    #[serde(default)]
    pub mission_fail_conditions_list: HashMap<CigGuid, Handle<MissionFailConditionsList>>,
    #[serde(default)]
    pub mission_module_hierarchy: HashMap<CigGuid, Handle<MissionModuleHierarchy>>,
    #[serde(default)]
    pub mission_scenario: HashMap<CigGuid, Handle<MissionScenario>>,
    #[serde(default)]
    pub mission_type: HashMap<CigGuid, Handle<MissionType>>,
    #[serde(default)]
    pub mission_locality: HashMap<CigGuid, Handle<MissionLocality>>,
    #[serde(default)]
    pub mission_broker_entry: HashMap<CigGuid, Handle<MissionBrokerEntry>>,
    #[serde(default)]
    pub mission_giver: HashMap<CigGuid, Handle<MissionGiver>>,
    #[serde(default)]
    pub mining_controller_global_params: HashMap<CigGuid, Handle<MiningControllerGlobalParams>>,
    #[serde(default)]
    pub missile_lock_reticle_config: HashMap<CigGuid, Handle<MissileLockReticle_Config>>,
}

impl RecordIndexMi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.mining_global_params.len()
            + self.mining_audio_params.len()
            + self.mineable_element.len()
            + self.mineable_composition.len()
            + self.mining_laser_global_params.len()
            + self.mission_location_template.len()
            + self.mission_item.len()
            + self.mission_organization.len()
            + self.mission_fail_conditions_list.len()
            + self.mission_module_hierarchy.len()
            + self.mission_scenario.len()
            + self.mission_type.len()
            + self.mission_locality.len()
            + self.mission_broker_entry.len()
            + self.mission_giver.len()
            + self.mining_controller_global_params.len()
            + self.missile_lock_reticle_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `mo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexMo {
    #[serde(default)]
    pub movement_system_additional_params_record: HashMap<CigGuid, Handle<MovementSystemAdditionalParamsRecord>>,
    #[serde(default)]
    pub motion_graph: HashMap<CigGuid, Handle<MotionGraph>>,
    #[serde(default)]
    pub module_declaration: HashMap<CigGuid, Handle<ModuleDeclaration>>,
    #[serde(default)]
    pub mobi_glas_app: HashMap<CigGuid, Handle<mobiGlasApp>>,
    #[serde(default)]
    pub move_view_restriction_penalty: HashMap<CigGuid, Handle<MoveViewRestrictionPenalty>>,
    #[serde(default)]
    pub movie_clip_transformation_interpolator: HashMap<CigGuid, Handle<MovieClipTransformationInterpolator>>,
    #[serde(default)]
    pub mobi_glas_app_data: HashMap<CigGuid, Handle<MobiGlasAppData>>,
}

impl RecordIndexMo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.movement_system_additional_params_record.len()
            + self.motion_graph.len()
            + self.module_declaration.len()
            + self.mobi_glas_app.len()
            + self.move_view_restriction_penalty.len()
            + self.movie_clip_transformation_interpolator.len()
            + self.mobi_glas_app_data.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `mu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexMu {
    #[serde(default)]
    pub music_logic_config: HashMap<CigGuid, Handle<MusicLogicConfig>>,
    #[serde(default)]
    pub music_logic_parameter: HashMap<CigGuid, Handle<MusicLogicParameter>>,
    #[serde(default)]
    pub music_logic_event: HashMap<CigGuid, Handle<MusicLogicEvent>>,
    #[serde(default)]
    pub music_logic_event_list: HashMap<CigGuid, Handle<MusicLogicEventList>>,
    #[serde(default)]
    pub music_logic_switch_value: HashMap<CigGuid, Handle<MusicLogicSwitchValue>>,
    #[serde(default)]
    pub music_logic_suite: HashMap<CigGuid, Handle<MusicLogicSuite>>,
}

impl RecordIndexMu {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.music_logic_config.len()
            + self.music_logic_parameter.len()
            + self.music_logic_event.len()
            + self.music_logic_event_list.len()
            + self.music_logic_switch_value.len()
            + self.music_logic_suite.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `no` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexNo {
    #[serde(default)]
    pub notification_def: HashMap<CigGuid, Handle<NotificationDef>>,
}

impl RecordIndexNo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.notification_def.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `op` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexOp {
    #[serde(default)]
    pub operator_mode_availability_params: HashMap<CigGuid, Handle<OperatorModeAvailabilityParams>>,
    #[serde(default)]
    pub operator_mode_definition_params: HashMap<CigGuid, Handle<OperatorModeDefinitionParams>>,
}

impl RecordIndexOp {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.operator_mode_availability_params.len()
            + self.operator_mode_definition_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `pe` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexPe {
    #[serde(default)]
    pub personal_inner_thought_action_rule_preset: HashMap<CigGuid, Handle<PersonalInnerThoughtActionRulePreset>>,
}

impl RecordIndexPe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.personal_inner_thought_action_rule_preset.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `pl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexPl {
    #[serde(default)]
    pub placed_surface_effects_emitter: HashMap<CigGuid, Handle<PlacedSurfaceEffects_Emitter>>,
    #[serde(default)]
    pub planet_ocean_audio_data: HashMap<CigGuid, Handle<PlanetOceanAudioData>>,
    #[serde(default)]
    pub planet_effect_lod: HashMap<CigGuid, Handle<PlanetEffectLOD>>,
    #[serde(default)]
    pub player_animated_interaction_template: HashMap<CigGuid, Handle<PlayerAnimatedInteractionTemplate>>,
    #[serde(default)]
    pub player_animated_interaction_config: HashMap<CigGuid, Handle<PlayerAnimatedInteractionConfig>>,
    #[serde(default)]
    pub player_choice_signal_config: HashMap<CigGuid, Handle<PlayerChoice_SignalConfig>>,
    #[serde(default)]
    pub player_choice_library: HashMap<CigGuid, Handle<PlayerChoice_Library>>,
    #[serde(default)]
    pub player_choice_imconfig: HashMap<CigGuid, Handle<PlayerChoice_IMConfig>>,
    #[serde(default)]
    pub player_choice_menu_item: HashMap<CigGuid, Handle<PlayerChoiceMenuItem>>,
    #[serde(default)]
    pub player_choice_menu_items: HashMap<CigGuid, Handle<PlayerChoiceMenuItems>>,
    #[serde(default)]
    pub player_choice_menu: HashMap<CigGuid, Handle<PlayerChoiceMenu>>,
    #[serde(default)]
    pub player_choice_menu_type: HashMap<CigGuid, Handle<PlayerChoiceMenuType>>,
    #[serde(default)]
    pub player_dock_context_component_global_params: HashMap<CigGuid, Handle<PlayerDockContextComponentGlobalParams>>,
    #[serde(default)]
    pub player_group_manager_global_params: HashMap<CigGuid, Handle<PlayerGroupManagerGlobalParams>>,
    #[serde(default)]
    pub player_limitations_profile: HashMap<CigGuid, Handle<PlayerLimitationsProfile>>,
    #[serde(default)]
    pub player_notification_banner_manager_global_params: HashMap<CigGuid, Handle<PlayerNotificationBannerManagerGlobalParams>>,
    #[serde(default)]
    pub player_ship_respawn: HashMap<CigGuid, Handle<PlayerShipRespawn>>,
    #[serde(default)]
    pub player_trade_global_params: HashMap<CigGuid, Handle<PlayerTradeGlobalParams>>,
    #[serde(default)]
    pub planet_day_night_temperature_template: HashMap<CigGuid, Handle<PlanetDayNightTemperatureTemplate>>,
    #[serde(default)]
    pub player_to_player_comms_call_global_params: HashMap<CigGuid, Handle<PlayerToPlayerCommsCallGlobalParams>>,
    #[serde(default)]
    pub player_choice_pitconfig: HashMap<CigGuid, Handle<PlayerChoice_PITConfig>>,
    #[serde(default)]
    pub player_ecggraph_config: HashMap<CigGuid, Handle<PlayerECGGraph_Config>>,
}

impl RecordIndexPl {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.placed_surface_effects_emitter.len()
            + self.planet_ocean_audio_data.len()
            + self.planet_effect_lod.len()
            + self.player_animated_interaction_template.len()
            + self.player_animated_interaction_config.len()
            + self.player_choice_signal_config.len()
            + self.player_choice_library.len()
            + self.player_choice_imconfig.len()
            + self.player_choice_menu_item.len()
            + self.player_choice_menu_items.len()
            + self.player_choice_menu.len()
            + self.player_choice_menu_type.len()
            + self.player_dock_context_component_global_params.len()
            + self.player_group_manager_global_params.len()
            + self.player_limitations_profile.len()
            + self.player_notification_banner_manager_global_params.len()
            + self.player_ship_respawn.len()
            + self.player_trade_global_params.len()
            + self.planet_day_night_temperature_template.len()
            + self.player_to_player_comms_call_global_params.len()
            + self.player_choice_pitconfig.len()
            + self.player_ecggraph_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `po` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexPo {
    #[serde(default)]
    pub pool_filter_record: HashMap<CigGuid, Handle<PoolFilterRecord>>,
    #[serde(default)]
    pub point_of_interest_list: HashMap<CigGuid, Handle<PointOfInterestList>>,
    #[serde(default)]
    pub posture_database: HashMap<CigGuid, Handle<PostureDatabase>>,
    #[serde(default)]
    pub popup_def: HashMap<CigGuid, Handle<PopupDef>>,
}

impl RecordIndexPo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.pool_filter_record.len()
            + self.point_of_interest_list.len()
            + self.posture_database.len()
            + self.popup_def.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `pr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexPr {
    #[serde(default)]
    pub procedural_planet_audio_tag_and_events_def: HashMap<CigGuid, Handle<ProceduralPlanetAudioTagAndEventsDef>>,
    #[serde(default)]
    pub procedural_planet_audio_data: HashMap<CigGuid, Handle<ProceduralPlanetAudioData>>,
    #[serde(default)]
    pub procedural_planet_audio_river_data: HashMap<CigGuid, Handle<ProceduralPlanetAudioRiverData>>,
    #[serde(default)]
    pub proc_breathing_curve: HashMap<CigGuid, Handle<ProcBreathingCurve>>,
    #[serde(default)]
    pub proc_breathing_curve_database: HashMap<CigGuid, Handle<ProcBreathingCurveDatabase>>,
    #[serde(default)]
    pub proc_breathing_setup: HashMap<CigGuid, Handle<ProcBreathingSetup>>,
    #[serde(default)]
    pub procedural_aim_rig_record: HashMap<CigGuid, Handle<ProceduralAimRigRecord>>,
    #[serde(default)]
    pub procedural_animation: HashMap<CigGuid, Handle<ProceduralAnimation>>,
    #[serde(default)]
    pub procedural_landing_setup: HashMap<CigGuid, Handle<ProceduralLandingSetup>>,
    #[serde(default)]
    pub procedural_layout_graph: HashMap<CigGuid, Handle<ProceduralLayoutGraph>>,
}

impl RecordIndexPr {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.procedural_planet_audio_tag_and_events_def.len()
            + self.procedural_planet_audio_data.len()
            + self.procedural_planet_audio_river_data.len()
            + self.proc_breathing_curve.len()
            + self.proc_breathing_curve_database.len()
            + self.proc_breathing_setup.len()
            + self.procedural_aim_rig_record.len()
            + self.procedural_animation.len()
            + self.procedural_landing_setup.len()
            + self.procedural_layout_graph.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `qt` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexQt {
    #[serde(default)]
    pub qterequest_config: HashMap<CigGuid, Handle<QTERequestConfig>>,
}

impl RecordIndexQt {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.qterequest_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `qu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexQu {
    #[serde(default)]
    pub quantum_drive_global_params: HashMap<CigGuid, Handle<QuantumDriveGlobalParams>>,
    #[serde(default)]
    pub quantum_drive_effect_settings: HashMap<CigGuid, Handle<QuantumDriveEffectSettings>>,
}

impl RecordIndexQu {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.quantum_drive_global_params.len()
            + self.quantum_drive_effect_settings.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ra` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexRa {
    #[serde(default)]
    pub ra_sta_rlibrary_element: HashMap<CigGuid, Handle<RaSTaRLibraryElement>>,
    #[serde(default)]
    pub ra_sta_rlibrary: HashMap<CigGuid, Handle<RaSTaRLibrary>>,
    #[serde(default)]
    pub radar_display_config: HashMap<CigGuid, Handle<RadarDisplay_Config>>,
    #[serde(default)]
    pub radar_system_global_params: HashMap<CigGuid, Handle<RadarSystemGlobalParams>>,
    #[serde(default)]
    pub radar_system_shared_params: HashMap<CigGuid, Handle<RadarSystemSharedParams>>,
    #[serde(default)]
    pub radar_signature_category_definition: HashMap<CigGuid, Handle<RadarSignatureCategoryDefinition>>,
    #[serde(default)]
    pub radar_signature_category_entry: HashMap<CigGuid, Handle<RadarSignatureCategoryEntry>>,
    #[serde(default)]
    pub radar_contact_type_definition: HashMap<CigGuid, Handle<RadarContactTypeDefinition>>,
    #[serde(default)]
    pub radar_contact_type_entry: HashMap<CigGuid, Handle<RadarContactTypeEntry>>,
    #[serde(default)]
    pub radar_contact_group_definition: HashMap<CigGuid, Handle<RadarContactGroupDefinition>>,
    #[serde(default)]
    pub radar_contact_group_entry: HashMap<CigGuid, Handle<RadarContactGroupEntry>>,
    #[serde(default)]
    pub radar_delta_signature_definition: HashMap<CigGuid, Handle<RadarDeltaSignatureDefinition>>,
    #[serde(default)]
    pub radar_delta_signature_entry: HashMap<CigGuid, Handle<RadarDeltaSignatureEntry>>,
    #[serde(default)]
    pub radiation_state_template: HashMap<CigGuid, Handle<RadiationStateTemplate>>,
    #[serde(default)]
    pub radiation_behavior: HashMap<CigGuid, Handle<RadiationBehavior>>,
    #[serde(default)]
    pub radar_display3_dpreset: HashMap<CigGuid, Handle<RadarDisplay3DPreset>>,
}

impl RecordIndexRa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.ra_sta_rlibrary_element.len()
            + self.ra_sta_rlibrary.len()
            + self.radar_display_config.len()
            + self.radar_system_global_params.len()
            + self.radar_system_shared_params.len()
            + self.radar_signature_category_definition.len()
            + self.radar_signature_category_entry.len()
            + self.radar_contact_type_definition.len()
            + self.radar_contact_type_entry.len()
            + self.radar_contact_group_definition.len()
            + self.radar_contact_group_entry.len()
            + self.radar_delta_signature_definition.len()
            + self.radar_delta_signature_entry.len()
            + self.radiation_state_template.len()
            + self.radiation_behavior.len()
            + self.radar_display3_dpreset.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `re` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexRe {
    #[serde(default)]
    pub refining_process: HashMap<CigGuid, Handle<RefiningProcess>>,
    #[serde(default)]
    pub refinery_notification_configuration: HashMap<CigGuid, Handle<RefineryNotificationConfiguration>>,
    #[serde(default)]
    pub rental_notification_params: HashMap<CigGuid, Handle<RentalNotificationParams>>,
    #[serde(default)]
    pub reputation_value_setting: HashMap<CigGuid, Handle<ReputationValueSetting>>,
    #[serde(default)]
    pub reputation_value_settings: HashMap<CigGuid, Handle<ReputationValueSettings>>,
    #[serde(default)]
    pub resource_type: HashMap<CigGuid, Handle<ResourceType>>,
    #[serde(default)]
    pub resource_type_group: HashMap<CigGuid, Handle<ResourceTypeGroup>>,
    #[serde(default)]
    pub resource_type_database: HashMap<CigGuid, Handle<ResourceTypeDatabase>>,
}

impl RecordIndexRe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.refining_process.len()
            + self.refinery_notification_configuration.len()
            + self.rental_notification_params.len()
            + self.reputation_value_setting.len()
            + self.reputation_value_settings.len()
            + self.resource_type.len()
            + self.resource_type_group.len()
            + self.resource_type_database.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sa` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSa {
    #[serde(default)]
    pub sactor_force_reactions_def: HashMap<CigGuid, Handle<SActorForceReactionsDef>>,
    #[serde(default)]
    pub sactor_hit_reactions_def: HashMap<CigGuid, Handle<SActorHitReactionsDef>>,
    #[serde(default)]
    pub sactor_external_force_response_camera_shake_def: HashMap<CigGuid, Handle<SActorExternalForceResponseCameraShakeDef>>,
    #[serde(default)]
    pub sactor_force_reactions_preset_record: HashMap<CigGuid, Handle<SActorForceReactionsPresetRecord>>,
    #[serde(default)]
    pub sactor_locomotion_fidget_state_filtered_def: HashMap<CigGuid, Handle<SActorLocomotionFidgetStateFilteredDef>>,
    #[serde(default)]
    pub sactor_locomotion_fidget_def: HashMap<CigGuid, Handle<SActorLocomotionFidgetDef>>,
    #[serde(default)]
    pub sandbox_trigger_record: HashMap<CigGuid, Handle<SandboxTriggerRecord>>,
    #[serde(default)]
    pub sanalytics_event: HashMap<CigGuid, Handle<SAnalyticsEvent>>,
    #[serde(default)]
    pub sanalytics_event_database: HashMap<CigGuid, Handle<SAnalyticsEventDatabase>>,
    #[serde(default)]
    pub saimable_gimbal_mode_labels: HashMap<CigGuid, Handle<SAimableGimbalModeLabels>>,
    #[serde(default)]
    pub saimable_game_mode_params: HashMap<CigGuid, Handle<SAimableGameModeParams>>,
    #[serde(default)]
    pub saimable_controller_hud_params: HashMap<CigGuid, Handle<SAimableControllerHudParams>>,
}

impl RecordIndexSa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.sactor_force_reactions_def.len()
            + self.sactor_hit_reactions_def.len()
            + self.sactor_external_force_response_camera_shake_def.len()
            + self.sactor_force_reactions_preset_record.len()
            + self.sactor_locomotion_fidget_state_filtered_def.len()
            + self.sactor_locomotion_fidget_def.len()
            + self.sandbox_trigger_record.len()
            + self.sanalytics_event.len()
            + self.sanalytics_event_database.len()
            + self.saimable_gimbal_mode_labels.len()
            + self.saimable_game_mode_params.len()
            + self.saimable_controller_hud_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sb` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSb {
    #[serde(default)]
    pub sbezier_curve_record: HashMap<CigGuid, Handle<SBezierCurveRecord>>,
}

impl RecordIndexSb {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.sbezier_curve_record.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sc` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSc {
    #[serde(default)]
    pub scprone_motion_graph_def: HashMap<CigGuid, Handle<SCProneMotionGraphDef>>,
    #[serde(default)]
    pub scharacter_generation_params: HashMap<CigGuid, Handle<SCharacterGenerationParams>>,
    #[serde(default)]
    pub scdynamic_lighting_rig_global_params: HashMap<CigGuid, Handle<SCDynamicLightingRigGlobalParams>>,
    #[serde(default)]
    pub scarryable_ikinteraction_list: HashMap<CigGuid, Handle<SCarryableIKInteractionList>>,
    #[serde(default)]
    pub scuttable_shape_definition: HashMap<CigGuid, Handle<SCuttableShapeDefinition>>,
    #[serde(default)]
    pub scan_information_def: HashMap<CigGuid, Handle<ScanInformationDef>>,
    #[serde(default)]
    pub scan_information_table: HashMap<CigGuid, Handle<ScanInformationTable>>,
    #[serde(default)]
    pub scan_custom_data_def: HashMap<CigGuid, Handle<ScanCustomDataDef>>,
    #[serde(default)]
    pub scan_display_instance_params: HashMap<CigGuid, Handle<ScanDisplayInstanceParams>>,
    #[serde(default)]
    pub scan_display_layout_params: HashMap<CigGuid, Handle<ScanDisplayLayoutParams>>,
    #[serde(default)]
    pub scitem_light_amplification: HashMap<CigGuid, Handle<SCItemLightAmplification>>,
    #[serde(default)]
    pub scitem_manufacturer: HashMap<CigGuid, Handle<SCItemManufacturer>>,
    #[serde(default)]
    pub scitem_visor_dashboard_config: HashMap<CigGuid, Handle<SCItemVisorDashboardConfig>>,
    #[serde(default)]
    pub scitem_suit_fuel_params: HashMap<CigGuid, Handle<SCItemSuitFuelParams>>,
    #[serde(default)]
    pub scitem_comms_component_setup: HashMap<CigGuid, Handle<SCItemCommsComponentSetup>>,
    #[serde(default)]
    pub scitem_display_screen_preset: HashMap<CigGuid, Handle<SCItemDisplayScreenPreset>>,
    #[serde(default)]
    pub scseat_head_pos_adjust_setup: HashMap<CigGuid, Handle<SCSeatHeadPosAdjustSetup>>,
    #[serde(default)]
    pub scitem_seat_head_tracking_position_limit_params: HashMap<CigGuid, Handle<SCItemSeatHeadTrackingPositionLimitParams>>,
    #[serde(default)]
    pub scenario_progress: HashMap<CigGuid, Handle<ScenarioProgress>>,
    #[serde(default)]
    pub screen_effects_library: HashMap<CigGuid, Handle<ScreenEffects_Library>>,
    #[serde(default)]
    pub screen_effects_effect: HashMap<CigGuid, Handle<ScreenEffects_Effect>>,
    #[serde(default)]
    pub screen_effects_debug: HashMap<CigGuid, Handle<ScreenEffects_Debug>>,
    #[serde(default)]
    pub scobject_data_bank_entry_marker_config: HashMap<CigGuid, Handle<SCObjectDataBankEntryMarkerConfig>>,
    #[serde(default)]
    pub scitem_uiview_dashboard_canvas_def: HashMap<CigGuid, Handle<SCItemUIView_DashboardCanvasDef>>,
}

impl RecordIndexSc {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.scprone_motion_graph_def.len()
            + self.scharacter_generation_params.len()
            + self.scdynamic_lighting_rig_global_params.len()
            + self.scarryable_ikinteraction_list.len()
            + self.scuttable_shape_definition.len()
            + self.scan_information_def.len()
            + self.scan_information_table.len()
            + self.scan_custom_data_def.len()
            + self.scan_display_instance_params.len()
            + self.scan_display_layout_params.len()
            + self.scitem_light_amplification.len()
            + self.scitem_manufacturer.len()
            + self.scitem_visor_dashboard_config.len()
            + self.scitem_suit_fuel_params.len()
            + self.scitem_comms_component_setup.len()
            + self.scitem_display_screen_preset.len()
            + self.scseat_head_pos_adjust_setup.len()
            + self.scitem_seat_head_tracking_position_limit_params.len()
            + self.scenario_progress.len()
            + self.screen_effects_library.len()
            + self.screen_effects_effect.len()
            + self.screen_effects_debug.len()
            + self.scobject_data_bank_entry_marker_config.len()
            + self.scitem_uiview_dashboard_canvas_def.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `se` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSe {
    #[serde(default)]
    pub seaplayer_loadout_snapshots: HashMap<CigGuid, Handle<SEAPlayerLoadoutSnapshots>>,
    #[serde(default)]
    pub seaglobal_special_loadout: HashMap<CigGuid, Handle<SEAGlobalSpecialLoadout>>,
    #[serde(default)]
    pub seaglobal_event_loadouts: HashMap<CigGuid, Handle<SEAGlobalEventLoadouts>>,
    #[serde(default)]
    pub sentrances_def: HashMap<CigGuid, Handle<SEntrancesDef>>,
    #[serde(default)]
    pub sentity_density_class: HashMap<CigGuid, Handle<SEntityDensityClass>>,
    #[serde(default)]
    pub sentity_density_class_overrides_record: HashMap<CigGuid, Handle<SEntityDensityClassOverridesRecord>>,
    #[serde(default)]
    pub sentity_traversing_node_id: HashMap<CigGuid, Handle<SEntityTraversingNodeId>>,
    #[serde(default)]
    pub seat_reticle_archetype: HashMap<CigGuid, Handle<SeatReticleArchetype>>,
    #[serde(default)]
    pub seat_ads_def: HashMap<CigGuid, Handle<SeatAdsDef>>,
    #[serde(default)]
    pub seat_user_actor_cdikrecord: HashMap<CigGuid, Handle<SeatUserActorCDIKRecord>>,
    #[serde(default)]
    pub security_clearance_token: HashMap<CigGuid, Handle<SecurityClearanceToken>>,
    #[serde(default)]
    pub security_network_room_settings: HashMap<CigGuid, Handle<SecurityNetworkRoomSettings>>,
    #[serde(default)]
    pub security_network_manifest: HashMap<CigGuid, Handle<SecurityNetworkManifest>>,
    #[serde(default)]
    pub service_beacon_params: HashMap<CigGuid, Handle<ServiceBeaconParams>>,
    #[serde(default)]
    pub service_beacon_global_params: HashMap<CigGuid, Handle<ServiceBeaconGlobalParams>>,
}

impl RecordIndexSe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.seaplayer_loadout_snapshots.len()
            + self.seaglobal_special_loadout.len()
            + self.seaglobal_event_loadouts.len()
            + self.sentrances_def.len()
            + self.sentity_density_class.len()
            + self.sentity_density_class_overrides_record.len()
            + self.sentity_traversing_node_id.len()
            + self.seat_reticle_archetype.len()
            + self.seat_ads_def.len()
            + self.seat_user_actor_cdikrecord.len()
            + self.security_clearance_token.len()
            + self.security_network_room_settings.len()
            + self.security_network_manifest.len()
            + self.service_beacon_params.len()
            + self.service_beacon_global_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sg` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSg {
    #[serde(default)]
    pub sgeometry_view_distance_ratio_categories: HashMap<CigGuid, Handle<SGeometryViewDistanceRatioCategories>>,
    #[serde(default)]
    pub sglobal_charge_drain_beam_params: HashMap<CigGuid, Handle<SGlobalChargeDrainBeamParams>>,
    #[serde(default)]
    pub sglobal_crosshair_params: HashMap<CigGuid, Handle<SGlobalCrosshairParams>>,
    #[serde(default)]
    pub sglobal_cuttable_shape_params: HashMap<CigGuid, Handle<SGlobalCuttableShapeParams>>,
    #[serde(default)]
    pub sglobal_electron_params: HashMap<CigGuid, Handle<SGlobalElectronParams>>,
    #[serde(default)]
    pub sglobal_healing_beam_params: HashMap<CigGuid, Handle<SGlobalHealingBeamParams>>,
    #[serde(default)]
    pub sglobal_salvage_repair_beam_params: HashMap<CigGuid, Handle<SGlobalSalvageRepairBeamParams>>,
    #[serde(default)]
    pub sglobal_tractor_beam_params: HashMap<CigGuid, Handle<SGlobalTractorBeamParams>>,
    #[serde(default)]
    pub sglobal_hit_behavior_params: HashMap<CigGuid, Handle<SGlobalHitBehaviorParams>>,
}

impl RecordIndexSg {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.sgeometry_view_distance_ratio_categories.len()
            + self.sglobal_charge_drain_beam_params.len()
            + self.sglobal_crosshair_params.len()
            + self.sglobal_cuttable_shape_params.len()
            + self.sglobal_electron_params.len()
            + self.sglobal_healing_beam_params.len()
            + self.sglobal_salvage_repair_beam_params.len()
            + self.sglobal_tractor_beam_params.len()
            + self.sglobal_hit_behavior_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sh` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSh {
    #[serde(default)]
    pub ship_insurance_policy_record: HashMap<CigGuid, Handle<ShipInsurancePolicyRecord>>,
    #[serde(default)]
    pub ship_computer_definition: HashMap<CigGuid, Handle<ShipComputerDefinition>>,
    #[serde(default)]
    pub shop_interaction_data: HashMap<CigGuid, Handle<ShopInteractionData>>,
    #[serde(default)]
    pub shop_franchise: HashMap<CigGuid, Handle<ShopFranchise>>,
    #[serde(default)]
    pub shield_type_params: HashMap<CigGuid, Handle<ShieldTypeParams>>,
    #[serde(default)]
    pub ship_computer_preset_list: HashMap<CigGuid, Handle<ShipComputerPresetList>>,
    #[serde(default)]
    pub ship_computer_preset: HashMap<CigGuid, Handle<ShipComputerPreset>>,
}

impl RecordIndexSh {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.ship_insurance_policy_record.len()
            + self.ship_computer_definition.len()
            + self.shop_interaction_data.len()
            + self.shop_franchise.len()
            + self.shield_type_params.len()
            + self.ship_computer_preset_list.len()
            + self.ship_computer_preset.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `si` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSi {
    #[serde(default)]
    pub sifcsmodifiers_legacy: HashMap<CigGuid, Handle<SIFCSModifiersLegacy>>,
    #[serde(default)]
    pub sifcsesp: HashMap<CigGuid, Handle<SIFCSEsp>>,
    #[serde(default)]
    pub sifcsgame_mode_params: HashMap<CigGuid, Handle<SIFCSGameModeParams>>,
    #[serde(default)]
    pub simple_sprite_sheet: HashMap<CigGuid, Handle<SimpleSpriteSheet>>,
}

impl RecordIndexSi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.sifcsmodifiers_legacy.len()
            + self.sifcsesp.len()
            + self.sifcsgame_mode_params.len()
            + self.simple_sprite_sheet.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sk` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSk {
    #[serde(default)]
    pub skill_definitions: HashMap<CigGuid, Handle<SkillDefinitions>>,
    #[serde(default)]
    pub skin_interactable_templates: HashMap<CigGuid, Handle<SkinInteractableTemplates>>,
}

impl RecordIndexSk {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.skill_definitions.len()
            + self.skin_interactable_templates.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sl` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSl {
    #[serde(default)]
    pub sloadout_assortment: HashMap<CigGuid, Handle<SLoadoutAssortment>>,
    #[serde(default)]
    pub slocal_player_shopping_data: HashMap<CigGuid, Handle<SLocalPlayerShoppingData>>,
}

impl RecordIndexSl {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.sloadout_assortment.len()
            + self.slocal_player_shopping_data.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sm` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSm {
    #[serde(default)]
    pub smannequin_action_def_record: HashMap<CigGuid, Handle<SMannequinActionDefRecord>>,
    #[serde(default)]
    pub smaster_mode_labels: HashMap<CigGuid, Handle<SMasterModeLabels>>,
    #[serde(default)]
    pub smovable_limits: HashMap<CigGuid, Handle<SMovableLimits>>,
    #[serde(default)]
    pub smfdmode_config: HashMap<CigGuid, Handle<SMFDModeConfig>>,
    #[serde(default)]
    pub smfdview: HashMap<CigGuid, Handle<SMFDView>>,
    #[serde(default)]
    pub smfdview_list: HashMap<CigGuid, Handle<SMFDViewList>>,
    #[serde(default)]
    pub smfdparams_diagnostics: HashMap<CigGuid, Handle<SMFDParamsDiagnostics>>,
}

impl RecordIndexSm {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.smannequin_action_def_record.len()
            + self.smaster_mode_labels.len()
            + self.smovable_limits.len()
            + self.smfdmode_config.len()
            + self.smfdview.len()
            + self.smfdview_list.len()
            + self.smfdparams_diagnostics.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `so` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSo {
    #[serde(default)]
    pub soperator_mode_labels: HashMap<CigGuid, Handle<SOperatorModeLabels>>,
}

impl RecordIndexSo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.soperator_mode_labels.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sp` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSp {
    #[serde(default)]
    pub sperk_reputation_list_params: HashMap<CigGuid, Handle<SPerkReputationListParams>>,
    #[serde(default)]
    pub sprojected_hud_params: HashMap<CigGuid, Handle<SProjectedHudParams>>,
    #[serde(default)]
    pub spawn_descriptions: HashMap<CigGuid, Handle<SpawnDescriptions>>,
    #[serde(default)]
    pub special_event_manufacturer: HashMap<CigGuid, Handle<SpecialEventManufacturer>>,
    #[serde(default)]
    pub special_event_day: HashMap<CigGuid, Handle<SpecialEventDay>>,
    #[serde(default)]
    pub special_event_database: HashMap<CigGuid, Handle<SpecialEventDatabase>>,
}

impl RecordIndexSp {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.sperk_reputation_list_params.len()
            + self.sprojected_hud_params.len()
            + self.spawn_descriptions.len()
            + self.special_event_manufacturer.len()
            + self.special_event_day.len()
            + self.special_event_database.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sq` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSq {
    #[serde(default)]
    pub squantum_drive_effect_template: HashMap<CigGuid, Handle<SQuantumDriveEffectTemplate>>,
    #[serde(default)]
    pub squantum_camera_effects_def: HashMap<CigGuid, Handle<SQuantumCameraEffectsDef>>,
}

impl RecordIndexSq {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.squantum_drive_effect_template.len()
            + self.squantum_camera_effects_def.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSr {
    #[serde(default)]
    pub sreputation_standing_params: HashMap<CigGuid, Handle<SReputationStandingParams>>,
    #[serde(default)]
    pub sreputation_context_ui: HashMap<CigGuid, Handle<SReputationContextUI>>,
    #[serde(default)]
    pub sreputation_global_context_bbparams: HashMap<CigGuid, Handle<SReputationGlobalContextBBParams>>,
    #[serde(default)]
    pub sreputation_state_params: HashMap<CigGuid, Handle<SReputationStateParams>>,
    #[serde(default)]
    pub sreputation_state_mission_result_modifier_params: HashMap<CigGuid, Handle<SReputationStateMissionResultModifierParams>>,
    #[serde(default)]
    pub sreputation_scope_params: HashMap<CigGuid, Handle<SReputationScopeParams>>,
    #[serde(default)]
    pub sreputation_reward_amount: HashMap<CigGuid, Handle<SReputationRewardAmount>>,
    #[serde(default)]
    pub sreputation_mission_reward_bonus_params: HashMap<CigGuid, Handle<SReputationMissionRewardBonusParams>>,
    #[serde(default)]
    pub sreputation_journal_entry_handler_params: HashMap<CigGuid, Handle<SReputationJournalEntryHandlerParams>>,
}

impl RecordIndexSr {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.sreputation_standing_params.len()
            + self.sreputation_context_ui.len()
            + self.sreputation_global_context_bbparams.len()
            + self.sreputation_state_params.len()
            + self.sreputation_state_mission_result_modifier_params.len()
            + self.sreputation_scope_params.len()
            + self.sreputation_reward_amount.len()
            + self.sreputation_mission_reward_bonus_params.len()
            + self.sreputation_journal_entry_handler_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ss` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSs {
    #[serde(default)]
    pub sscsignature_system_audio_ruleset: HashMap<CigGuid, Handle<SSCSignatureSystemAudioRuleset>>,
    #[serde(default)]
    pub ssolar_system: HashMap<CigGuid, Handle<SSolarSystem>>,
    #[serde(default)]
    pub sscene_player_choice_settings: HashMap<CigGuid, Handle<SScenePlayerChoiceSettings>>,
}

impl RecordIndexSs {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.sscsignature_system_audio_ruleset.len()
            + self.ssolar_system.len()
            + self.sscene_player_choice_settings.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `st` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSt {
    #[serde(default)]
    pub stat_definitions: HashMap<CigGuid, Handle<StatDefinitions>>,
    #[serde(default)]
    pub stargeting_method_record: HashMap<CigGuid, Handle<STargetingMethodRecord>>,
    #[serde(default)]
    pub stargetable_item_types_record: HashMap<CigGuid, Handle<STargetableItemTypesRecord>>,
    #[serde(default)]
    pub starget_selector_hud_params: HashMap<CigGuid, Handle<STargetSelectorHudParams>>,
    #[serde(default)]
    pub starget_selector_global_targeting_params: HashMap<CigGuid, Handle<STargetSelectorGlobalTargetingParams>>,
    #[serde(default)]
    pub sturret_health_modifier_def: HashMap<CigGuid, Handle<STurretHealthModifierDef>>,
    #[serde(default)]
    pub sturret_esp: HashMap<CigGuid, Handle<STurretESP>>,
    #[serde(default)]
    pub sturret_global_params: HashMap<CigGuid, Handle<STurretGlobalParams>>,
    #[serde(default)]
    pub star_map_object_type: HashMap<CigGuid, Handle<StarMapObjectType>>,
    #[serde(default)]
    pub star_map_object_types: HashMap<CigGuid, Handle<StarMapObjectTypes>>,
    #[serde(default)]
    pub star_map_amenity_type_entry: HashMap<CigGuid, Handle<StarMapAmenityTypeEntry>>,
    #[serde(default)]
    pub star_map_amenity_types: HashMap<CigGuid, Handle<StarMapAmenityTypes>>,
    #[serde(default)]
    pub star_map_object: HashMap<CigGuid, Handle<StarMapObject>>,
    #[serde(default)]
    pub star_map_mission_object: HashMap<CigGuid, Handle<StarMapMissionObject>>,
    #[serde(default)]
    pub star_map_party_member_object: HashMap<CigGuid, Handle<StarMapPartyMemberObject>>,
    #[serde(default)]
    pub status_widget_display_preset: HashMap<CigGuid, Handle<StatusWidgetDisplayPreset>>,
}

impl RecordIndexSt {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.stat_definitions.len()
            + self.stargeting_method_record.len()
            + self.stargetable_item_types_record.len()
            + self.starget_selector_hud_params.len()
            + self.starget_selector_global_targeting_params.len()
            + self.sturret_health_modifier_def.len()
            + self.sturret_esp.len()
            + self.sturret_global_params.len()
            + self.star_map_object_type.len()
            + self.star_map_object_types.len()
            + self.star_map_amenity_type_entry.len()
            + self.star_map_amenity_types.len()
            + self.star_map_object.len()
            + self.star_map_mission_object.len()
            + self.star_map_party_member_object.len()
            + self.status_widget_display_preset.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `su` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSu {
    #[serde(default)]
    pub surface_audio_properties_definition: HashMap<CigGuid, Handle<SurfaceAudioPropertiesDefinition>>,
    #[serde(default)]
    pub suggested_fovsetup: HashMap<CigGuid, Handle<SuggestedFOVSetup>>,
    #[serde(default)]
    pub sub_harvestable_config_record: HashMap<CigGuid, Handle<SubHarvestableConfigRecord>>,
    #[serde(default)]
    pub sub_harvestable_multi_config_record: HashMap<CigGuid, Handle<SubHarvestableMultiConfigRecord>>,
}

impl RecordIndexSu {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.surface_audio_properties_definition.len()
            + self.suggested_fovsetup.len()
            + self.sub_harvestable_config_record.len()
            + self.sub_harvestable_multi_config_record.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `sv` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexSv {
    #[serde(default)]
    pub svibration_def: HashMap<CigGuid, Handle<SVibrationDef>>,
    #[serde(default)]
    pub svibration_vehicle_def: HashMap<CigGuid, Handle<SVibrationVehicleDef>>,
    #[serde(default)]
    pub sview_distance_ratio_params: HashMap<CigGuid, Handle<SViewDistanceRatioParams>>,
    #[serde(default)]
    pub svehicle_hud_params: HashMap<CigGuid, Handle<SVehicleHudParams>>,
    #[serde(default)]
    pub svehicle_ai_damage_modifiers: HashMap<CigGuid, Handle<SVehicleAiDamageModifiers>>,
}

impl RecordIndexSv {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.svibration_def.len()
            + self.svibration_vehicle_def.len()
            + self.sview_distance_ratio_params.len()
            + self.svehicle_hud_params.len()
            + self.svehicle_ai_damage_modifiers.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ta` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexTa {
    #[serde(default)]
    pub target_tracking_auto_zoom_def: HashMap<CigGuid, Handle<TargetTrackingAutoZoomDef>>,
    #[serde(default)]
    pub tag_to_audio_rtpc_list: HashMap<CigGuid, Handle<TagToAudioRtpcList>>,
    #[serde(default)]
    pub tactical_query: HashMap<CigGuid, Handle<TacticalQuery>>,
    #[serde(default)]
    pub tag: HashMap<CigGuid, Handle<Tag>>,
    #[serde(default)]
    pub tag_database: HashMap<CigGuid, Handle<TagDatabase>>,
    #[serde(default)]
    pub take_down_config: HashMap<CigGuid, Handle<TakeDownConfig>>,
}

impl RecordIndexTa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.target_tracking_auto_zoom_def.len()
            + self.tag_to_audio_rtpc_list.len()
            + self.tactical_query.len()
            + self.tag.len()
            + self.tag_database.len()
            + self.take_down_config.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ti` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexTi {
    #[serde(default)]
    pub tint_palette_tree: HashMap<CigGuid, Handle<TintPaletteTree>>,
}

impl RecordIndexTi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.tint_palette_tree.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `tq` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexTq {
    #[serde(default)]
    pub tqsoption_content_record: HashMap<CigGuid, Handle<TQSOptionContentRecord>>,
}

impl RecordIndexTq {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.tqsoption_content_record.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `tr` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexTr {
    #[serde(default)]
    pub traversal_cost_config: HashMap<CigGuid, Handle<TraversalCostConfig>>,
    #[serde(default)]
    pub transformation_interpolator: HashMap<CigGuid, Handle<TransformationInterpolator>>,
    #[serde(default)]
    pub transit_station_announcements: HashMap<CigGuid, Handle<TransitStationAnnouncements>>,
}

impl RecordIndexTr {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.traversal_cost_config.len()
            + self.transformation_interpolator.len()
            + self.transit_station_announcements.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `tu` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexTu {
    #[serde(default)]
    pub turret_input_deflection_time_params: HashMap<CigGuid, Handle<TurretInputDeflectionTimeParams>>,
}

impl RecordIndexTu {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.turret_input_deflection_time_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ui` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexUi {
    #[serde(default)]
    pub uiholo_vehicle_config: HashMap<CigGuid, Handle<UIHoloVehicle_Config>>,
    #[serde(default)]
    pub uiconfig: HashMap<CigGuid, Handle<UIConfig>>,
    #[serde(default)]
    pub uimode_visibility_settings: HashMap<CigGuid, Handle<UIModeVisibilitySettings>>,
    #[serde(default)]
    pub uistate_display: HashMap<CigGuid, Handle<UIStateDisplay>>,
    #[serde(default)]
    pub uielement: HashMap<CigGuid, Handle<UIElement>>,
    #[serde(default)]
    pub uielement_sounds_record: HashMap<CigGuid, Handle<UIElementSoundsRecord>>,
    #[serde(default)]
    pub uiaudio_definition: HashMap<CigGuid, Handle<UIAudioDefinition>>,
}

impl RecordIndexUi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.uiholo_vehicle_config.len()
            + self.uiconfig.len()
            + self.uimode_visibility_settings.len()
            + self.uistate_display.len()
            + self.uielement.len()
            + self.uielement_sounds_record.len()
            + self.uiaudio_definition.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `un` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexUn {
    #[serde(default)]
    pub unit_test_sub_record: HashMap<CigGuid, Handle<UnitTestSubRecord>>,
    #[serde(default)]
    pub unit_test: HashMap<CigGuid, Handle<UnitTest>>,
}

impl RecordIndexUn {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.unit_test_sub_record.len()
            + self.unit_test.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `us` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexUs {
    #[serde(default)]
    pub usable_archetype: HashMap<CigGuid, Handle<UsableArchetype>>,
    #[serde(default)]
    pub use_channel_archetype: HashMap<CigGuid, Handle<UseChannelArchetype>>,
    #[serde(default)]
    pub usable_archetypes: HashMap<CigGuid, Handle<UsableArchetypes>>,
}

impl RecordIndexUs {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.usable_archetype.len()
            + self.use_channel_archetype.len()
            + self.usable_archetypes.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ve` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexVe {
    #[serde(default)]
    pub vehicle_serial_number_character_type: HashMap<CigGuid, Handle<VehicleSerialNumberCharacterType>>,
    #[serde(default)]
    pub vehicle_serial_number_format: HashMap<CigGuid, Handle<VehicleSerialNumberFormat>>,
    #[serde(default)]
    pub vehicle_salvage_global_params: HashMap<CigGuid, Handle<VehicleSalvageGlobalParams>>,
    #[serde(default)]
    pub vehicle_landing_gear_system: HashMap<CigGuid, Handle<VehicleLandingGearSystem>>,
    #[serde(default)]
    pub vehicle_role: HashMap<CigGuid, Handle<VehicleRole>>,
    #[serde(default)]
    pub vehicle_career: HashMap<CigGuid, Handle<VehicleCareer>>,
    #[serde(default)]
    pub vehicle_career_list: HashMap<CigGuid, Handle<VehicleCareerList>>,
}

impl RecordIndexVe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.vehicle_serial_number_character_type.len()
            + self.vehicle_serial_number_format.len()
            + self.vehicle_salvage_global_params.len()
            + self.vehicle_landing_gear_system.len()
            + self.vehicle_role.len()
            + self.vehicle_career.len()
            + self.vehicle_career_list.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `vi` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexVi {
    #[serde(default)]
    pub vibration_audio_point_def: HashMap<CigGuid, Handle<VibrationAudioPointDef>>,
    #[serde(default)]
    pub visor_hud_config: HashMap<CigGuid, Handle<VisorHUD_Config>>,
    #[serde(default)]
    pub visor_control_hints_config: HashMap<CigGuid, Handle<Visor_ControlHintsConfig>>,
    #[serde(default)]
    pub video_comms: HashMap<CigGuid, Handle<VideoComms>>,
    #[serde(default)]
    pub video_comms_shader_params: HashMap<CigGuid, Handle<VideoCommsShaderParams>>,
    #[serde(default)]
    pub video_comms_audio_params: HashMap<CigGuid, Handle<VideoCommsAudioParams>>,
    #[serde(default)]
    pub visor_lens_layout: HashMap<CigGuid, Handle<VisorLens_Layout>>,
    #[serde(default)]
    pub visor_lens_region: HashMap<CigGuid, Handle<VisorLens_Region>>,
}

impl RecordIndexVi {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.vibration_audio_point_def.len()
            + self.visor_hud_config.len()
            + self.visor_control_hints_config.len()
            + self.video_comms.len()
            + self.video_comms_shader_params.len()
            + self.video_comms_audio_params.len()
            + self.visor_lens_layout.len()
            + self.visor_lens_region.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `vo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexVo {
    #[serde(default)]
    pub voice_channel_settings_record: HashMap<CigGuid, Handle<VoiceChannelSettingsRecord>>,
    #[serde(default)]
    pub voice_single: HashMap<CigGuid, Handle<VoiceSingle>>,
    #[serde(default)]
    pub voice_bundle: HashMap<CigGuid, Handle<VoiceBundle>>,
}

impl RecordIndexVo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.voice_channel_settings_record.len()
            + self.voice_single.len()
            + self.voice_bundle.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `wa` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexWa {
    #[serde(default)]
    pub water_effects_global_params: HashMap<CigGuid, Handle<WaterEffectsGlobalParams>>,
}

impl RecordIndexWa {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.water_effects_global_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `we` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexWe {
    #[serde(default)]
    pub weapon_aimable_angles_def: HashMap<CigGuid, Handle<WeaponAimableAnglesDef>>,
    #[serde(default)]
    pub weapon_gimbal_mode_modifier_def: HashMap<CigGuid, Handle<WeaponGimbalModeModifierDef>>,
    #[serde(default)]
    pub weapon_misfire_def: HashMap<CigGuid, Handle<WeaponMisfireDef>>,
    #[serde(default)]
    pub weapon_armodifier: HashMap<CigGuid, Handle<WeaponARModifier>>,
    #[serde(default)]
    pub weapon_procedural_animation: HashMap<CigGuid, Handle<WeaponProceduralAnimation>>,
    #[serde(default)]
    pub weapon_procedural_clip: HashMap<CigGuid, Handle<WeaponProceduralClip>>,
    #[serde(default)]
    pub weapon_procedural_recoil_config_def: HashMap<CigGuid, Handle<WeaponProceduralRecoilConfigDef>>,
    #[serde(default)]
    pub web_customization_debug: HashMap<CigGuid, Handle<WebCustomizationDebug>>,
    #[serde(default)]
    pub web_customization_global_params: HashMap<CigGuid, Handle<WebCustomizationGlobalParams>>,
}

impl RecordIndexWe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.weapon_aimable_angles_def.len()
            + self.weapon_gimbal_mode_modifier_def.len()
            + self.weapon_misfire_def.len()
            + self.weapon_armodifier.len()
            + self.weapon_procedural_animation.len()
            + self.weapon_procedural_clip.len()
            + self.weapon_procedural_recoil_config_def.len()
            + self.web_customization_debug.len()
            + self.web_customization_global_params.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `wh` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexWh {
    #[serde(default)]
    pub wheel_audio_surface_map: HashMap<CigGuid, Handle<WheelAudioSurfaceMap>>,
}

impl RecordIndexWh {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.wheel_audio_surface_map.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `wo` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexWo {
    #[serde(default)]
    pub world_display_radar: HashMap<CigGuid, Handle<WorldDisplayRadar>>,
    #[serde(default)]
    pub world_display_environment: HashMap<CigGuid, Handle<WorldDisplayEnvironment>>,
}

impl RecordIndexWo {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.world_display_radar.len()
            + self.world_display_environment.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Bucket record-index sub-struct for types in the `ze` group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndexZe {
    #[serde(default)]
    pub zero_gtraversal_graph: HashMap<CigGuid, Handle<ZeroGTraversalGraph>>,
}

impl RecordIndexZe {
    /// Total number of records indexed in this bucket.
    pub fn len(&self) -> usize {
        self.zero_gtraversal_graph.len()
    }

    /// True if every per-type map in this bucket is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Per-record-type GUID → slot-index lookup.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndex {
    #[serde(default)]
    pub b_ac: RecordIndexAc,
    #[serde(default)]
    pub b_ai: RecordIndexAi,
    #[serde(default)]
    pub b_am: RecordIndexAm,
    #[serde(default)]
    pub b_an: RecordIndexAn,
    #[serde(default)]
    pub b_ar: RecordIndexAr,
    #[serde(default)]
    pub b_as: RecordIndexAs,
    #[serde(default)]
    pub b_at: RecordIndexAt,
    #[serde(default)]
    pub b_au: RecordIndexAu,
    #[serde(default)]
    pub b_aw: RecordIndexAw,
    #[serde(default)]
    pub b_be: RecordIndexBe,
    #[serde(default)]
    pub b_bl: RecordIndexBl,
    #[serde(default)]
    pub b_bo: RecordIndexBo,
    #[serde(default)]
    pub b_br: RecordIndexBr,
    #[serde(default)]
    pub b_bu: RecordIndexBu,
    #[serde(default)]
    pub b_ca: RecordIndexCa,
    #[serde(default)]
    pub b_ch: RecordIndexCh,
    #[serde(default)]
    pub b_ci: RecordIndexCi,
    #[serde(default)]
    pub b_co: RecordIndexCo,
    #[serde(default)]
    pub b_cr: RecordIndexCr,
    #[serde(default)]
    pub b_ct: RecordIndexCt,
    #[serde(default)]
    pub b_da: RecordIndexDa,
    #[serde(default)]
    pub b_de: RecordIndexDe,
    #[serde(default)]
    pub b_di: RecordIndexDi,
    #[serde(default)]
    pub b_do: RecordIndexDo,
    #[serde(default)]
    pub b_dy: RecordIndexDy,
    #[serde(default)]
    pub b_el: RecordIndexEl,
    #[serde(default)]
    pub b_em: RecordIndexEm,
    #[serde(default)]
    pub b_en: RecordIndexEn,
    #[serde(default)]
    pub b_es: RecordIndexEs,
    #[serde(default)]
    pub b_ev: RecordIndexEv,
    #[serde(default)]
    pub b_ex: RecordIndexEx,
    #[serde(default)]
    pub b_fa: RecordIndexFa,
    #[serde(default)]
    pub b_fi: RecordIndexFi,
    #[serde(default)]
    pub b_fl: RecordIndexFl,
    #[serde(default)]
    pub b_fo: RecordIndexFo,
    #[serde(default)]
    pub b_fr: RecordIndexFr,
    #[serde(default)]
    pub b_ga: RecordIndexGa,
    #[serde(default)]
    pub b_ge: RecordIndexGe,
    #[serde(default)]
    pub b_gl: RecordIndexGl,
    #[serde(default)]
    pub b_gp: RecordIndexGp,
    #[serde(default)]
    pub b_gr: RecordIndexGr,
    #[serde(default)]
    pub b_ha: RecordIndexHa,
    #[serde(default)]
    pub b_he: RecordIndexHe,
    #[serde(default)]
    pub b_hi: RecordIndexHi,
    #[serde(default)]
    pub b_ho: RecordIndexHo,
    #[serde(default)]
    pub b_hu: RecordIndexHu,
    #[serde(default)]
    pub b_if: RecordIndexIf,
    #[serde(default)]
    pub b_in: RecordIndexIn,
    #[serde(default)]
    pub b_it: RecordIndexIt,
    #[serde(default)]
    pub b_jo: RecordIndexJo,
    #[serde(default)]
    pub b_ju: RecordIndexJu,
    #[serde(default)]
    pub b_la: RecordIndexLa,
    #[serde(default)]
    pub b_le: RecordIndexLe,
    #[serde(default)]
    pub b_lo: RecordIndexLo,
    #[serde(default)]
    pub b_ma: RecordIndexMa,
    #[serde(default)]
    pub b_me: RecordIndexMe,
    #[serde(default)]
    pub b_mi: RecordIndexMi,
    #[serde(default)]
    pub b_mo: RecordIndexMo,
    #[serde(default)]
    pub b_mu: RecordIndexMu,
    #[serde(default)]
    pub b_no: RecordIndexNo,
    #[serde(default)]
    pub b_op: RecordIndexOp,
    #[serde(default)]
    pub b_pe: RecordIndexPe,
    #[serde(default)]
    pub b_pl: RecordIndexPl,
    #[serde(default)]
    pub b_po: RecordIndexPo,
    #[serde(default)]
    pub b_pr: RecordIndexPr,
    #[serde(default)]
    pub b_qt: RecordIndexQt,
    #[serde(default)]
    pub b_qu: RecordIndexQu,
    #[serde(default)]
    pub b_ra: RecordIndexRa,
    #[serde(default)]
    pub b_re: RecordIndexRe,
    #[serde(default)]
    pub b_sa: RecordIndexSa,
    #[serde(default)]
    pub b_sb: RecordIndexSb,
    #[serde(default)]
    pub b_sc: RecordIndexSc,
    #[serde(default)]
    pub b_se: RecordIndexSe,
    #[serde(default)]
    pub b_sg: RecordIndexSg,
    #[serde(default)]
    pub b_sh: RecordIndexSh,
    #[serde(default)]
    pub b_si: RecordIndexSi,
    #[serde(default)]
    pub b_sk: RecordIndexSk,
    #[serde(default)]
    pub b_sl: RecordIndexSl,
    #[serde(default)]
    pub b_sm: RecordIndexSm,
    #[serde(default)]
    pub b_so: RecordIndexSo,
    #[serde(default)]
    pub b_sp: RecordIndexSp,
    #[serde(default)]
    pub b_sq: RecordIndexSq,
    #[serde(default)]
    pub b_sr: RecordIndexSr,
    #[serde(default)]
    pub b_ss: RecordIndexSs,
    #[serde(default)]
    pub b_st: RecordIndexSt,
    #[serde(default)]
    pub b_su: RecordIndexSu,
    #[serde(default)]
    pub b_sv: RecordIndexSv,
    #[serde(default)]
    pub b_ta: RecordIndexTa,
    #[serde(default)]
    pub b_ti: RecordIndexTi,
    #[serde(default)]
    pub b_tq: RecordIndexTq,
    #[serde(default)]
    pub b_tr: RecordIndexTr,
    #[serde(default)]
    pub b_tu: RecordIndexTu,
    #[serde(default)]
    pub b_ui: RecordIndexUi,
    #[serde(default)]
    pub b_un: RecordIndexUn,
    #[serde(default)]
    pub b_us: RecordIndexUs,
    #[serde(default)]
    pub b_ve: RecordIndexVe,
    #[serde(default)]
    pub b_vi: RecordIndexVi,
    #[serde(default)]
    pub b_vo: RecordIndexVo,
    #[serde(default)]
    pub b_wa: RecordIndexWa,
    #[serde(default)]
    pub b_we: RecordIndexWe,
    #[serde(default)]
    pub b_wh: RecordIndexWh,
    #[serde(default)]
    pub b_wo: RecordIndexWo,
    #[serde(default)]
    pub b_ze: RecordIndexZe,
}

impl RecordIndex {
    /// Total number of records indexed across every type.
    pub fn len(&self) -> usize {
        self.b_ac.len()
            + self.b_ai.len()
            + self.b_am.len()
            + self.b_an.len()
            + self.b_ar.len()
            + self.b_as.len()
            + self.b_at.len()
            + self.b_au.len()
            + self.b_aw.len()
            + self.b_be.len()
            + self.b_bl.len()
            + self.b_bo.len()
            + self.b_br.len()
            + self.b_bu.len()
            + self.b_ca.len()
            + self.b_ch.len()
            + self.b_ci.len()
            + self.b_co.len()
            + self.b_cr.len()
            + self.b_ct.len()
            + self.b_da.len()
            + self.b_de.len()
            + self.b_di.len()
            + self.b_do.len()
            + self.b_dy.len()
            + self.b_el.len()
            + self.b_em.len()
            + self.b_en.len()
            + self.b_es.len()
            + self.b_ev.len()
            + self.b_ex.len()
            + self.b_fa.len()
            + self.b_fi.len()
            + self.b_fl.len()
            + self.b_fo.len()
            + self.b_fr.len()
            + self.b_ga.len()
            + self.b_ge.len()
            + self.b_gl.len()
            + self.b_gp.len()
            + self.b_gr.len()
            + self.b_ha.len()
            + self.b_he.len()
            + self.b_hi.len()
            + self.b_ho.len()
            + self.b_hu.len()
            + self.b_if.len()
            + self.b_in.len()
            + self.b_it.len()
            + self.b_jo.len()
            + self.b_ju.len()
            + self.b_la.len()
            + self.b_le.len()
            + self.b_lo.len()
            + self.b_ma.len()
            + self.b_me.len()
            + self.b_mi.len()
            + self.b_mo.len()
            + self.b_mu.len()
            + self.b_no.len()
            + self.b_op.len()
            + self.b_pe.len()
            + self.b_pl.len()
            + self.b_po.len()
            + self.b_pr.len()
            + self.b_qt.len()
            + self.b_qu.len()
            + self.b_ra.len()
            + self.b_re.len()
            + self.b_sa.len()
            + self.b_sb.len()
            + self.b_sc.len()
            + self.b_se.len()
            + self.b_sg.len()
            + self.b_sh.len()
            + self.b_si.len()
            + self.b_sk.len()
            + self.b_sl.len()
            + self.b_sm.len()
            + self.b_so.len()
            + self.b_sp.len()
            + self.b_sq.len()
            + self.b_sr.len()
            + self.b_ss.len()
            + self.b_st.len()
            + self.b_su.len()
            + self.b_sv.len()
            + self.b_ta.len()
            + self.b_ti.len()
            + self.b_tq.len()
            + self.b_tr.len()
            + self.b_tu.len()
            + self.b_ui.len()
            + self.b_un.len()
            + self.b_us.len()
            + self.b_ve.len()
            + self.b_vi.len()
            + self.b_vo.len()
            + self.b_wa.len()
            + self.b_we.len()
            + self.b_wh.len()
            + self.b_wo.len()
            + self.b_ze.len()
    }

    /// True if every per-type map is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
