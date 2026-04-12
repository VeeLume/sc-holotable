// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! `RecordStore` — the flat-pool record store.
//!
//! Thin wrapper around `DataPools` (per-type `Vec<Option<T>>`) and
//! `RecordIndex` (per-record-type `HashMap<CigGuid, TId>`). Built via
//! `Builder::new(db).consume_database().finish()`.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::Instance;
use crate::{Builder, Extract, Handle};
use super::*;

/// Flat-pool record store produced by [`Builder::finish`](crate::Builder::finish).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordStore {
    /// Per-type `Vec<Option<T>>` storage.
    #[serde(default)]
    pub pools: DataPools,
    /// Per-record-type GUID → slot-index lookups.
    #[serde(default)]
    pub records: RecordIndex,
}

impl RecordStore {
    /// Create an empty record store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Total number of records indexed across every type.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// True if no records have been loaded.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

// ── Per-type record extractor adapters ─────────────────────────────────
//
// One free function per emitted record type. `seed_database` resolves
// the current DCB's struct_index for each type by name and stores a
// pointer to the matching function in a dispatch table. This keeps the
// extractor robust against game patches that reorder struct definitions.
type RecordExtractor<'a> = fn(&mut Builder<'a>, CigGuid, Instance<'a>);

fn extract_record_activity_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActivityData> = Handle::new(b.alloc_record::<ActivityData>(inst, guid));
    b.records.b_ac.activity_data.insert(guid, id);
}
fn extract_record_aiperception_profile<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIPerceptionProfile> = Handle::new(b.alloc_record::<AIPerceptionProfile>(inst, guid));
    b.records.b_ai.aiperception_profile.insert(guid, id);
}
fn extract_record_aimercy_timer_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIMercyTimerSettings> = Handle::new(b.alloc_record::<AIMercyTimerSettings>(inst, guid));
    b.records.b_ai.aimercy_timer_settings.insert(guid, id);
}
fn extract_record_aivisual_field_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIVisualFieldParams> = Handle::new(b.alloc_record::<AIVisualFieldParams>(inst, guid));
    b.records.b_ai.aivisual_field_params.insert(guid, id);
}
fn extract_record_aivisual_field_profile<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIVisualFieldProfile> = Handle::new(b.alloc_record::<AIVisualFieldProfile>(inst, guid));
    b.records.b_ai.aivisual_field_profile.insert(guid, id);
}
fn extract_record_aiobservable_filter_flags<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIObservableFilterFlags> = Handle::new(b.alloc_record::<AIObservableFilterFlags>(inst, guid));
    b.records.b_ai.aiobservable_filter_flags.insert(guid, id);
}
fn extract_record_aiobservable_filters_profile<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIObservableFiltersProfile> = Handle::new(b.alloc_record::<AIObservableFiltersProfile>(inst, guid));
    b.records.b_ai.aiobservable_filters_profile.insert(guid, id);
}
fn extract_record_movement_system_additional_params_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MovementSystemAdditionalParamsRecord> = Handle::new(b.alloc_record::<MovementSystemAdditionalParamsRecord>(inst, guid));
    b.records.b_mo.movement_system_additional_params_record.insert(guid, id);
}
fn extract_record_aitargetable_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AITargetableSettings> = Handle::new(b.alloc_record::<AITargetableSettings>(inst, guid));
    b.records.b_ai.aitargetable_settings.insert(guid, id);
}
fn extract_record_aispecial_ranged_attack_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AISpecialRangedAttackConfig> = Handle::new(b.alloc_record::<AISpecialRangedAttackConfig>(inst, guid));
    b.records.b_ai.aispecial_ranged_attack_config.insert(guid, id);
}
fn extract_record_aiavailable_special_ranged_attacks_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIAvailableSpecialRangedAttacksConfig> = Handle::new(b.alloc_record::<AIAvailableSpecialRangedAttacksConfig>(inst, guid));
    b.records.b_ai.aiavailable_special_ranged_attacks_config.insert(guid, id);
}
fn extract_record_traversal_cost_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TraversalCostConfig> = Handle::new(b.alloc_record::<TraversalCostConfig>(inst, guid));
    b.records.b_tr.traversal_cost_config.insert(guid, id);
}
fn extract_record_aifire_discipline_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIFireDisciplineSettings> = Handle::new(b.alloc_record::<AIFireDisciplineSettings>(inst, guid));
    b.records.b_ai.aifire_discipline_settings.insert(guid, id);
}
fn extract_record_aimotive_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIMotiveList> = Handle::new(b.alloc_record::<AIMotiveList>(inst, guid));
    b.records.b_ai.aimotive_list.insert(guid, id);
}
fn extract_record_aiprofile<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIProfile> = Handle::new(b.alloc_record::<AIProfile>(inst, guid));
    b.records.b_ai.aiprofile.insert(guid, id);
}
fn extract_record_skill_definitions<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SkillDefinitions> = Handle::new(b.alloc_record::<SkillDefinitions>(inst, guid));
    b.records.b_sk.skill_definitions.insert(guid, id);
}
fn extract_record_stat_definitions<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<StatDefinitions> = Handle::new(b.alloc_record::<StatDefinitions>(inst, guid));
    b.records.b_st.stat_definitions.insert(guid, id);
}
fn extract_record_aitargeting_formula_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AITargetingFormulaSettings> = Handle::new(b.alloc_record::<AITargetingFormulaSettings>(inst, guid));
    b.records.b_ai.aitargeting_formula_settings.insert(guid, id);
}
fn extract_record_aiwave_collection<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIWaveCollection> = Handle::new(b.alloc_record::<AIWaveCollection>(inst, guid));
    b.records.b_ai.aiwave_collection.insert(guid, id);
}
fn extract_record_armarker_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ARMarkerGlobalParams> = Handle::new(b.alloc_record::<ARMarkerGlobalParams>(inst, guid));
    b.records.b_ar.armarker_global_params.insert(guid, id);
}
fn extract_record_actor_ability_component<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorAbilityComponent> = Handle::new(b.alloc_record::<ActorAbilityComponent>(inst, guid));
    b.records.b_ac.actor_ability_component.insert(guid, id);
}
fn extract_record_actor_ducking_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorDuckingParams> = Handle::new(b.alloc_record::<ActorDuckingParams>(inst, guid));
    b.records.b_ac.actor_ducking_params.insert(guid, id);
}
fn extract_record_evagraph<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EVAGraph> = Handle::new(b.alloc_record::<EVAGraph>(inst, guid));
    b.records.b_ev.evagraph.insert(guid, id);
}
fn extract_record_actor_environment_component<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorEnvironmentComponent> = Handle::new(b.alloc_record::<ActorEnvironmentComponent>(inst, guid));
    b.records.b_ac.actor_environment_component.insert(guid, id);
}
fn extract_record_sactor_force_reactions_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SActorForceReactionsDef> = Handle::new(b.alloc_record::<SActorForceReactionsDef>(inst, guid));
    b.records.b_sa.sactor_force_reactions_def.insert(guid, id);
}
fn extract_record_sactor_hit_reactions_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SActorHitReactionsDef> = Handle::new(b.alloc_record::<SActorHitReactionsDef>(inst, guid));
    b.records.b_sa.sactor_hit_reactions_def.insert(guid, id);
}
fn extract_record_sactor_external_force_response_camera_shake_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SActorExternalForceResponseCameraShakeDef> = Handle::new(b.alloc_record::<SActorExternalForceResponseCameraShakeDef>(inst, guid));
    b.records.b_sa.sactor_external_force_response_camera_shake_def.insert(guid, id);
}
fn extract_record_sactor_force_reactions_preset_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SActorForceReactionsPresetRecord> = Handle::new(b.alloc_record::<SActorForceReactionsPresetRecord>(inst, guid));
    b.records.b_sa.sactor_force_reactions_preset_record.insert(guid, id);
}
fn extract_record_actor_gforce_component<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorGForceComponent> = Handle::new(b.alloc_record::<ActorGForceComponent>(inst, guid));
    b.records.b_ac.actor_gforce_component.insert(guid, id);
}
fn extract_record_actor_gforce_head_bob<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorGForceHeadBob> = Handle::new(b.alloc_record::<ActorGForceHeadBob>(inst, guid));
    b.records.b_ac.actor_gforce_head_bob.insert(guid, id);
}
fn extract_record_actor_gforce_camera_effects<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorGForceCameraEffects> = Handle::new(b.alloc_record::<ActorGForceCameraEffects>(inst, guid));
    b.records.b_ac.actor_gforce_camera_effects.insert(guid, id);
}
fn extract_record_lean_graph<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LeanGraph> = Handle::new(b.alloc_record::<LeanGraph>(inst, guid));
    b.records.b_le.lean_graph.insert(guid, id);
}
fn extract_record_sactor_locomotion_fidget_state_filtered_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SActorLocomotionFidgetStateFilteredDef> = Handle::new(b.alloc_record::<SActorLocomotionFidgetStateFilteredDef>(inst, guid));
    b.records.b_sa.sactor_locomotion_fidget_state_filtered_def.insert(guid, id);
}
fn extract_record_sactor_locomotion_fidget_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SActorLocomotionFidgetDef> = Handle::new(b.alloc_record::<SActorLocomotionFidgetDef>(inst, guid));
    b.records.b_sa.sactor_locomotion_fidget_def.insert(guid, id);
}
fn extract_record_actor_locomotion_personality<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorLocomotionPersonality> = Handle::new(b.alloc_record::<ActorLocomotionPersonality>(inst, guid));
    b.records.b_ac.actor_locomotion_personality.insert(guid, id);
}
fn extract_record_motion_graph<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MotionGraph> = Handle::new(b.alloc_record::<MotionGraph>(inst, guid));
    b.records.b_mo.motion_graph.insert(guid, id);
}
fn extract_record_scprone_motion_graph_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCProneMotionGraphDef> = Handle::new(b.alloc_record::<SCProneMotionGraphDef>(inst, guid));
    b.records.b_sc.scprone_motion_graph_def.insert(guid, id);
}
fn extract_record_smannequin_action_def_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SMannequinActionDefRecord> = Handle::new(b.alloc_record::<SMannequinActionDefRecord>(inst, guid));
    b.records.b_sm.smannequin_action_def_record.insert(guid, id);
}
fn extract_record_actor_movement_modifiers<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorMovementModifiers> = Handle::new(b.alloc_record::<ActorMovementModifiers>(inst, guid));
    b.records.b_ac.actor_movement_modifiers.insert(guid, id);
}
fn extract_record_actor_movement_sets_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorMovementSetsConfig> = Handle::new(b.alloc_record::<ActorMovementSetsConfig>(inst, guid));
    b.records.b_ac.actor_movement_sets_config.insert(guid, id);
}
fn extract_record_actor_procedural_recoil_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorProceduralRecoilConfig> = Handle::new(b.alloc_record::<ActorProceduralRecoilConfig>(inst, guid));
    b.records.b_ac.actor_procedural_recoil_config.insert(guid, id);
}
fn extract_record_actor_procedural_recoil_modifiers<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorProceduralRecoilModifiers> = Handle::new(b.alloc_record::<ActorProceduralRecoilModifiers>(inst, guid));
    b.records.b_ac.actor_procedural_recoil_modifiers.insert(guid, id);
}
fn extract_record_actor_skeleton_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorSkeletonConfig> = Handle::new(b.alloc_record::<ActorSkeletonConfig>(inst, guid));
    b.records.b_ac.actor_skeleton_config.insert(guid, id);
}
fn extract_record_actor_sliding_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorSlidingParams> = Handle::new(b.alloc_record::<ActorSlidingParams>(inst, guid));
    b.records.b_ac.actor_sliding_params.insert(guid, id);
}
fn extract_record_actor_speed_camera_effects<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorSpeedCameraEffects> = Handle::new(b.alloc_record::<ActorSpeedCameraEffects>(inst, guid));
    b.records.b_ac.actor_speed_camera_effects.insert(guid, id);
}
fn extract_record_actor_stamina_component<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorStaminaComponent> = Handle::new(b.alloc_record::<ActorStaminaComponent>(inst, guid));
    b.records.b_ac.actor_stamina_component.insert(guid, id);
}
fn extract_record_actor_stance_speeds_info<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorStanceSpeedsInfo> = Handle::new(b.alloc_record::<ActorStanceSpeedsInfo>(inst, guid));
    b.records.b_ac.actor_stance_speeds_info.insert(guid, id);
}
fn extract_record_actor_stance_dimensions_info<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorStanceDimensionsInfo> = Handle::new(b.alloc_record::<ActorStanceDimensionsInfo>(inst, guid));
    b.records.b_ac.actor_stance_dimensions_info.insert(guid, id);
}
fn extract_record_actor_state_validation<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorStateValidation> = Handle::new(b.alloc_record::<ActorStateValidation>(inst, guid));
    b.records.b_ac.actor_state_validation.insert(guid, id);
}
fn extract_record_actor_status_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorStatusGlobalParams> = Handle::new(b.alloc_record::<ActorStatusGlobalParams>(inst, guid));
    b.records.b_ac.actor_status_global_params.insert(guid, id);
}
fn extract_record_actor_status_component<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorStatusComponent> = Handle::new(b.alloc_record::<ActorStatusComponent>(inst, guid));
    b.records.b_ac.actor_status_component.insert(guid, id);
}
fn extract_record_actor_zero_gtraversal_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorZeroGTraversalParams> = Handle::new(b.alloc_record::<ActorZeroGTraversalParams>(inst, guid));
    b.records.b_ac.actor_zero_gtraversal_params.insert(guid, id);
}
fn extract_record_zero_gtraversal_graph<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ZeroGTraversalGraph> = Handle::new(b.alloc_record::<ZeroGTraversalGraph>(inst, guid));
    b.records.b_ze.zero_gtraversal_graph.insert(guid, id);
}
fn extract_record_ammo_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AmmoParams> = Handle::new(b.alloc_record::<AmmoParams>(inst, guid));
    b.records.b_am.ammo_params.insert(guid, id);
}
fn extract_record_animated_marker<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AnimatedMarker> = Handle::new(b.alloc_record::<AnimatedMarker>(inst, guid));
    b.records.b_an.animated_marker.insert(guid, id);
}
fn extract_record_combat_marker<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CombatMarker> = Handle::new(b.alloc_record::<CombatMarker>(inst, guid));
    b.records.b_co.combat_marker.insert(guid, id);
}
fn extract_record_announcer<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Announcer> = Handle::new(b.alloc_record::<Announcer>(inst, guid));
    b.records.b_an.announcer.insert(guid, id);
}
fn extract_record_asteroid_field_composition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AsteroidFieldComposition> = Handle::new(b.alloc_record::<AsteroidFieldComposition>(inst, guid));
    b.records.b_as.asteroid_field_composition.insert(guid, id);
}
fn extract_record_atmospheric_flight_effects<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AtmosphericFlightEffects> = Handle::new(b.alloc_record::<AtmosphericFlightEffects>(inst, guid));
    b.records.b_at.atmospheric_flight_effects.insert(guid, id);
}
fn extract_record_audio_breath_style_condition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioBreathStyleCondition> = Handle::new(b.alloc_record::<AudioBreathStyleCondition>(inst, guid));
    b.records.b_au.audio_breath_style_condition.insert(guid, id);
}
fn extract_record_audio_breath_style_condition_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioBreathStyleConditionList> = Handle::new(b.alloc_record::<AudioBreathStyleConditionList>(inst, guid));
    b.records.b_au.audio_breath_style_condition_list.insert(guid, id);
}
fn extract_record_audio_breath_style<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioBreathStyle> = Handle::new(b.alloc_record::<AudioBreathStyle>(inst, guid));
    b.records.b_au.audio_breath_style.insert(guid, id);
}
fn extract_record_audio_breath_style_suite<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioBreathStyleSuite> = Handle::new(b.alloc_record::<AudioBreathStyleSuite>(inst, guid));
    b.records.b_au.audio_breath_style_suite.insert(guid, id);
}
fn extract_record_audio_breath_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioBreathDefinition> = Handle::new(b.alloc_record::<AudioBreathDefinition>(inst, guid));
    b.records.b_au.audio_breath_definition.insert(guid, id);
}
fn extract_record_audio_breath_interrupt<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioBreathInterrupt> = Handle::new(b.alloc_record::<AudioBreathInterrupt>(inst, guid));
    b.records.b_au.audio_breath_interrupt.insert(guid, id);
}
fn extract_record_breathing_trigger_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BreathingTriggerDef> = Handle::new(b.alloc_record::<BreathingTriggerDef>(inst, guid));
    b.records.b_br.breathing_trigger_def.insert(guid, id);
}
fn extract_record_entity_audio_controller_type_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EntityAudioControllerTypeParams> = Handle::new(b.alloc_record::<EntityAudioControllerTypeParams>(inst, guid));
    b.records.b_en.entity_audio_controller_type_params.insert(guid, id);
}
fn extract_record_entity_audio_controller_manager_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EntityAudioControllerManagerParams> = Handle::new(b.alloc_record::<EntityAudioControllerManagerParams>(inst, guid));
    b.records.b_en.entity_audio_controller_manager_params.insert(guid, id);
}
fn extract_record_audio_whitelist<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioWhitelist> = Handle::new(b.alloc_record::<AudioWhitelist>(inst, guid));
    b.records.b_au.audio_whitelist.insert(guid, id);
}
fn extract_record_audio_environment<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioEnvironment> = Handle::new(b.alloc_record::<AudioEnvironment>(inst, guid));
    b.records.b_au.audio_environment.insert(guid, id);
}
fn extract_record_audio_budget_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioBudgetDefinition> = Handle::new(b.alloc_record::<AudioBudgetDefinition>(inst, guid));
    b.records.b_au.audio_budget_definition.insert(guid, id);
}
fn extract_record_audio_game_context_globals<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioGameContextGlobals> = Handle::new(b.alloc_record::<AudioGameContextGlobals>(inst, guid));
    b.records.b_au.audio_game_context_globals.insert(guid, id);
}
fn extract_record_audio_game_context_setup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioGameContextSetup> = Handle::new(b.alloc_record::<AudioGameContextSetup>(inst, guid));
    b.records.b_au.audio_game_context_setup.insert(guid, id);
}
fn extract_record_surface_audio_properties_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SurfaceAudioPropertiesDefinition> = Handle::new(b.alloc_record::<SurfaceAudioPropertiesDefinition>(inst, guid));
    b.records.b_su.surface_audio_properties_definition.insert(guid, id);
}
fn extract_record_global_audio_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalAudioSettings> = Handle::new(b.alloc_record::<GlobalAudioSettings>(inst, guid));
    b.records.b_gl.global_audio_settings.insert(guid, id);
}
fn extract_record_audio_tag_action_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioTagActionList> = Handle::new(b.alloc_record::<AudioTagActionList>(inst, guid));
    b.records.b_au.audio_tag_action_list.insert(guid, id);
}
fn extract_record_audio_value_output_setup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioValueOutputSetup> = Handle::new(b.alloc_record::<AudioValueOutputSetup>(inst, guid));
    b.records.b_au.audio_value_output_setup.insert(guid, id);
}
fn extract_record_award_service_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AwardService_Config> = Handle::new(b.alloc_record::<AwardService_Config>(inst, guid));
    b.records.b_aw.award_service_config.insert(guid, id);
}
fn extract_record_building_blocks_timeline<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BuildingBlocks_Timeline> = Handle::new(b.alloc_record::<BuildingBlocks_Timeline>(inst, guid));
    b.records.b_bu.building_blocks_timeline.insert(guid, id);
}
fn extract_record_grab_camera_control_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GrabCameraControlParams> = Handle::new(b.alloc_record::<GrabCameraControlParams>(inst, guid));
    b.records.b_gr.grab_camera_control_params.insert(guid, id);
}
fn extract_record_building_blocks_canvas<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BuildingBlocks_Canvas> = Handle::new(b.alloc_record::<BuildingBlocks_Canvas>(inst, guid));
    b.records.b_bu.building_blocks_canvas.insert(guid, id);
}
fn extract_record_building_blocks_font_style<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BuildingBlocks_FontStyle> = Handle::new(b.alloc_record::<BuildingBlocks_FontStyle>(inst, guid));
    b.records.b_bu.building_blocks_font_style.insert(guid, id);
}
fn extract_record_building_blocks_language_specific_font_replacement<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BuildingBlocks_LanguageSpecificFontReplacement> = Handle::new(b.alloc_record::<BuildingBlocks_LanguageSpecificFontReplacement>(inst, guid));
    b.records.b_bu.building_blocks_language_specific_font_replacement.insert(guid, id);
}
fn extract_record_building_blocks_style<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BuildingBlocks_Style> = Handle::new(b.alloc_record::<BuildingBlocks_Style>(inst, guid));
    b.records.b_bu.building_blocks_style.insert(guid, id);
}
fn extract_record_building_blocks_external_color_reference<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BuildingBlocks_ExternalColorReference> = Handle::new(b.alloc_record::<BuildingBlocks_ExternalColorReference>(inst, guid));
    b.records.b_bu.building_blocks_external_color_reference.insert(guid, id);
}
fn extract_record_building_blocks_aspect_ratio_library<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BuildingBlocks_AspectRatioLibrary> = Handle::new(b.alloc_record::<BuildingBlocks_AspectRatioLibrary>(inst, guid));
    b.records.b_bu.building_blocks_aspect_ratio_library.insert(guid, id);
}
fn extract_record_camera_lens_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CameraLensParams> = Handle::new(b.alloc_record::<CameraLensParams>(inst, guid));
    b.records.b_ca.camera_lens_params.insert(guid, id);
}
fn extract_record_camera<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Camera> = Handle::new(b.alloc_record::<Camera>(inst, guid));
    b.records.b_ca.camera.insert(guid, id);
}
fn extract_record_camera_shop_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CameraShopConfig> = Handle::new(b.alloc_record::<CameraShopConfig>(inst, guid));
    b.records.b_ca.camera_shop_config.insert(guid, id);
}
fn extract_record_actor_fovview_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorFOVViewParams> = Handle::new(b.alloc_record::<ActorFOVViewParams>(inst, guid));
    b.records.b_ac.actor_fovview_params.insert(guid, id);
}
fn extract_record_suggested_fovsetup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SuggestedFOVSetup> = Handle::new(b.alloc_record::<SuggestedFOVSetup>(inst, guid));
    b.records.b_su.suggested_fovsetup.insert(guid, id);
}
fn extract_record_cinematic_camera_controller_setup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CinematicCameraControllerSetup> = Handle::new(b.alloc_record::<CinematicCameraControllerSetup>(inst, guid));
    b.records.b_ci.cinematic_camera_controller_setup.insert(guid, id);
}
fn extract_record_camera_fovchange_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CameraFOVChangeData> = Handle::new(b.alloc_record::<CameraFOVChangeData>(inst, guid));
    b.records.b_ca.camera_fovchange_data.insert(guid, id);
}
fn extract_record_cargo_manifest<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CargoManifest> = Handle::new(b.alloc_record::<CargoManifest>(inst, guid));
    b.records.b_ca.cargo_manifest.insert(guid, id);
}
fn extract_record_carry_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CarryConfig> = Handle::new(b.alloc_record::<CarryConfig>(inst, guid));
    b.records.b_ca.carry_config.insert(guid, id);
}
fn extract_record_character<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Character> = Handle::new(b.alloc_record::<Character>(inst, guid));
    b.records.b_ch.character.insert(guid, id);
}
fn extract_record_scharacter_generation_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCharacterGenerationParams> = Handle::new(b.alloc_record::<SCharacterGenerationParams>(inst, guid));
    b.records.b_sc.scharacter_generation_params.insert(guid, id);
}
fn extract_record_character_serialization_settings_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CharacterSerializationSettingsPreset> = Handle::new(b.alloc_record::<CharacterSerializationSettingsPreset>(inst, guid));
    b.records.b_ch.character_serialization_settings_preset.insert(guid, id);
}
fn extract_record_character_random_name_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CharacterRandomNameParams> = Handle::new(b.alloc_record::<CharacterRandomNameParams>(inst, guid));
    b.records.b_ch.character_random_name_params.insert(guid, id);
}
fn extract_record_chat_emote_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ChatEmoteRecord> = Handle::new(b.alloc_record::<ChatEmoteRecord>(inst, guid));
    b.records.b_ch.chat_emote_record.insert(guid, id);
}
fn extract_record_chat_command_fast_access<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ChatCommandFastAccess> = Handle::new(b.alloc_record::<ChatCommandFastAccess>(inst, guid));
    b.records.b_ch.chat_command_fast_access.insert(guid, id);
}
fn extract_record_chat_filter_options<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ChatFilterOptions> = Handle::new(b.alloc_record::<ChatFilterOptions>(inst, guid));
    b.records.b_ch.chat_filter_options.insert(guid, id);
}
fn extract_record_chat_manager_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ChatManagerGlobalParams> = Handle::new(b.alloc_record::<ChatManagerGlobalParams>(inst, guid));
    b.records.b_ch.chat_manager_global_params.insert(guid, id);
}
fn extract_record_cockpit_response<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CockpitResponse> = Handle::new(b.alloc_record::<CockpitResponse>(inst, guid));
    b.records.b_co.cockpit_response.insert(guid, id);
}
fn extract_record_cockpit_responses<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CockpitResponses> = Handle::new(b.alloc_record::<CockpitResponses>(inst, guid));
    b.records.b_co.cockpit_responses.insert(guid, id);
}
fn extract_record_communication_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommunicationConfig> = Handle::new(b.alloc_record::<CommunicationConfig>(inst, guid));
    b.records.b_co.communication_config.insert(guid, id);
}
fn extract_record_communication_channel_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommunicationChannelConfig> = Handle::new(b.alloc_record::<CommunicationChannelConfig>(inst, guid));
    b.records.b_co.communication_channel_config.insert(guid, id);
}
fn extract_record_communication_variable_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommunicationVariableConfig> = Handle::new(b.alloc_record::<CommunicationVariableConfig>(inst, guid));
    b.records.b_co.communication_variable_config.insert(guid, id);
}
fn extract_record_communication_atlconfig<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommunicationATLConfig> = Handle::new(b.alloc_record::<CommunicationATLConfig>(inst, guid));
    b.records.b_co.communication_atlconfig.insert(guid, id);
}
fn extract_record_communication_auto_mannequin_tags_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommunicationAutoMannequinTagsConfig> = Handle::new(b.alloc_record::<CommunicationAutoMannequinTagsConfig>(inst, guid));
    b.records.b_co.communication_auto_mannequin_tags_config.insert(guid, id);
}
fn extract_record_contextual_communication_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ContextualCommunicationConfig> = Handle::new(b.alloc_record::<ContextualCommunicationConfig>(inst, guid));
    b.records.b_co.contextual_communication_config.insert(guid, id);
}
fn extract_record_communication_name<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommunicationName> = Handle::new(b.alloc_record::<CommunicationName>(inst, guid));
    b.records.b_co.communication_name.insert(guid, id);
}
fn extract_record_communication_channel_name<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommunicationChannelName> = Handle::new(b.alloc_record::<CommunicationChannelName>(inst, guid));
    b.records.b_co.communication_channel_name.insert(guid, id);
}
fn extract_record_actor_look_ahead_vehicle<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorLookAheadVehicle> = Handle::new(b.alloc_record::<ActorLookAheadVehicle>(inst, guid));
    b.records.b_ac.actor_look_ahead_vehicle.insert(guid, id);
}
fn extract_record_ifcs_input_deflection_time_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<IfcsInputDeflectionTimeParams> = Handle::new(b.alloc_record::<IfcsInputDeflectionTimeParams>(inst, guid));
    b.records.b_if.ifcs_input_deflection_time_params.insert(guid, id);
}
fn extract_record_turret_input_deflection_time_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TurretInputDeflectionTimeParams> = Handle::new(b.alloc_record::<TurretInputDeflectionTimeParams>(inst, guid));
    b.records.b_tu.turret_input_deflection_time_params.insert(guid, id);
}
fn extract_record_intoxication_ifcsmodifier_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<IntoxicationIFCSModifierParams> = Handle::new(b.alloc_record::<IntoxicationIFCSModifierParams>(inst, guid));
    b.records.b_in.intoxication_ifcsmodifier_params.insert(guid, id);
}
fn extract_record_intoxication_turret_modifier_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<IntoxicationTurretModifierParams> = Handle::new(b.alloc_record::<IntoxicationTurretModifierParams>(inst, guid));
    b.records.b_in.intoxication_turret_modifier_params.insert(guid, id);
}
fn extract_record_intoxication_wheeled_modifier_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<IntoxicationWheeledModifierParams> = Handle::new(b.alloc_record::<IntoxicationWheeledModifierParams>(inst, guid));
    b.records.b_in.intoxication_wheeled_modifier_params.insert(guid, id);
}
fn extract_record_target_tracking_auto_zoom_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TargetTrackingAutoZoomDef> = Handle::new(b.alloc_record::<TargetTrackingAutoZoomDef>(inst, guid));
    b.records.b_ta.target_tracking_auto_zoom_def.insert(guid, id);
}
fn extract_record_audio_allegiance_switches<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioAllegianceSwitches> = Handle::new(b.alloc_record::<AudioAllegianceSwitches>(inst, guid));
    b.records.b_au.audio_allegiance_switches.insert(guid, id);
}
fn extract_record_scdynamic_lighting_rig_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCDynamicLightingRigGlobalParams> = Handle::new(b.alloc_record::<SCDynamicLightingRigGlobalParams>(inst, guid));
    b.records.b_sc.scdynamic_lighting_rig_global_params.insert(guid, id);
}
fn extract_record_entity_audio_controller_rtpc_subscriber_list_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EntityAudioControllerRtpcSubscriberListDef> = Handle::new(b.alloc_record::<EntityAudioControllerRtpcSubscriberListDef>(inst, guid));
    b.records.b_en.entity_audio_controller_rtpc_subscriber_list_def.insert(guid, id);
}
fn extract_record_audio_environment_feedback_zone_setup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioEnvironmentFeedbackZoneSetup> = Handle::new(b.alloc_record::<AudioEnvironmentFeedbackZoneSetup>(inst, guid));
    b.records.b_au.audio_environment_feedback_zone_setup.insert(guid, id);
}
fn extract_record_audio_environment_feedback_point_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioEnvironmentFeedbackPointDef> = Handle::new(b.alloc_record::<AudioEnvironmentFeedbackPointDef>(inst, guid));
    b.records.b_au.audio_environment_feedback_point_def.insert(guid, id);
}
fn extract_record_audio_hit_listener_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioHitListenerDefinition> = Handle::new(b.alloc_record::<AudioHitListenerDefinition>(inst, guid));
    b.records.b_au.audio_hit_listener_definition.insert(guid, id);
}
fn extract_record_legacy_crafting_recipe_def_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LegacyCraftingRecipeDefRecord> = Handle::new(b.alloc_record::<LegacyCraftingRecipeDefRecord>(inst, guid));
    b.records.b_le.legacy_crafting_recipe_def_record.insert(guid, id);
}
fn extract_record_legacy_crafting_recipe_list_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LegacyCraftingRecipeListRecord> = Handle::new(b.alloc_record::<LegacyCraftingRecipeListRecord>(inst, guid));
    b.records.b_le.legacy_crafting_recipe_list_record.insert(guid, id);
}
fn extract_record_wheel_audio_surface_map<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WheelAudioSurfaceMap> = Handle::new(b.alloc_record::<WheelAudioSurfaceMap>(inst, guid));
    b.records.b_wh.wheel_audio_surface_map.insert(guid, id);
}
fn extract_record_handhold_grip_type<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HandholdGripType> = Handle::new(b.alloc_record::<HandholdGripType>(inst, guid));
    b.records.b_ha.handhold_grip_type.insert(guid, id);
}
fn extract_record_handhold_grip_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HandholdGripDatabase> = Handle::new(b.alloc_record::<HandholdGripDatabase>(inst, guid));
    b.records.b_ha.handhold_grip_database.insert(guid, id);
}
fn extract_record_sub_harvestable_config_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SubHarvestableConfigRecord> = Handle::new(b.alloc_record::<SubHarvestableConfigRecord>(inst, guid));
    b.records.b_su.sub_harvestable_config_record.insert(guid, id);
}
fn extract_record_sub_harvestable_multi_config_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SubHarvestableMultiConfigRecord> = Handle::new(b.alloc_record::<SubHarvestableMultiConfigRecord>(inst, guid));
    b.records.b_su.sub_harvestable_multi_config_record.insert(guid, id);
}
fn extract_record_harvestable_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HarvestablePreset> = Handle::new(b.alloc_record::<HarvestablePreset>(inst, guid));
    b.records.b_ha.harvestable_preset.insert(guid, id);
}
fn extract_record_harvestable_setup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HarvestableSetup> = Handle::new(b.alloc_record::<HarvestableSetup>(inst, guid));
    b.records.b_ha.harvestable_setup.insert(guid, id);
}
fn extract_record_harvestable_cluster_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HarvestableClusterPreset> = Handle::new(b.alloc_record::<HarvestableClusterPreset>(inst, guid));
    b.records.b_ha.harvestable_cluster_preset.insert(guid, id);
}
fn extract_record_harvestable_provider_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HarvestableProviderPreset> = Handle::new(b.alloc_record::<HarvestableProviderPreset>(inst, guid));
    b.records.b_ha.harvestable_provider_preset.insert(guid, id);
}
fn extract_record_operator_mode_availability_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<OperatorModeAvailabilityParams> = Handle::new(b.alloc_record::<OperatorModeAvailabilityParams>(inst, guid));
    b.records.b_op.operator_mode_availability_params.insert(guid, id);
}
fn extract_record_operator_mode_definition_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<OperatorModeDefinitionParams> = Handle::new(b.alloc_record::<OperatorModeDefinitionParams>(inst, guid));
    b.records.b_op.operator_mode_definition_params.insert(guid, id);
}
fn extract_record_master_mode_exclusion_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MasterModeExclusionGlobalParams> = Handle::new(b.alloc_record::<MasterModeExclusionGlobalParams>(inst, guid));
    b.records.b_ma.master_mode_exclusion_global_params.insert(guid, id);
}
fn extract_record_ladder_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LadderConfig> = Handle::new(b.alloc_record::<LadderConfig>(inst, guid));
    b.records.b_la.ladder_config.insert(guid, id);
}
fn extract_record_mining_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MiningGlobalParams> = Handle::new(b.alloc_record::<MiningGlobalParams>(inst, guid));
    b.records.b_mi.mining_global_params.insert(guid, id);
}
fn extract_record_mining_audio_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MiningAudioParams> = Handle::new(b.alloc_record::<MiningAudioParams>(inst, guid));
    b.records.b_mi.mining_audio_params.insert(guid, id);
}
fn extract_record_mineable_element<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MineableElement> = Handle::new(b.alloc_record::<MineableElement>(inst, guid));
    b.records.b_mi.mineable_element.insert(guid, id);
}
fn extract_record_mineable_composition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MineableComposition> = Handle::new(b.alloc_record::<MineableComposition>(inst, guid));
    b.records.b_mi.mineable_composition.insert(guid, id);
}
fn extract_record_mining_laser_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MiningLaserGlobalParams> = Handle::new(b.alloc_record::<MiningLaserGlobalParams>(inst, guid));
    b.records.b_mi.mining_laser_global_params.insert(guid, id);
}
fn extract_record_placed_surface_effects_emitter<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlacedSurfaceEffects_Emitter> = Handle::new(b.alloc_record::<PlacedSurfaceEffects_Emitter>(inst, guid));
    b.records.b_pl.placed_surface_effects_emitter.insert(guid, id);
}
fn extract_record_procedural_planet_audio_tag_and_events_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProceduralPlanetAudioTagAndEventsDef> = Handle::new(b.alloc_record::<ProceduralPlanetAudioTagAndEventsDef>(inst, guid));
    b.records.b_pr.procedural_planet_audio_tag_and_events_def.insert(guid, id);
}
fn extract_record_procedural_planet_audio_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProceduralPlanetAudioData> = Handle::new(b.alloc_record::<ProceduralPlanetAudioData>(inst, guid));
    b.records.b_pr.procedural_planet_audio_data.insert(guid, id);
}
fn extract_record_procedural_planet_audio_river_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProceduralPlanetAudioRiverData> = Handle::new(b.alloc_record::<ProceduralPlanetAudioRiverData>(inst, guid));
    b.records.b_pr.procedural_planet_audio_river_data.insert(guid, id);
}
fn extract_record_planet_ocean_audio_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlanetOceanAudioData> = Handle::new(b.alloc_record::<PlanetOceanAudioData>(inst, guid));
    b.records.b_pl.planet_ocean_audio_data.insert(guid, id);
}
fn extract_record_stargeting_method_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<STargetingMethodRecord> = Handle::new(b.alloc_record::<STargetingMethodRecord>(inst, guid));
    b.records.b_st.stargeting_method_record.insert(guid, id);
}
fn extract_record_stargetable_item_types_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<STargetableItemTypesRecord> = Handle::new(b.alloc_record::<STargetableItemTypesRecord>(inst, guid));
    b.records.b_st.stargetable_item_types_record.insert(guid, id);
}
fn extract_record_starget_selector_hud_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<STargetSelectorHudParams> = Handle::new(b.alloc_record::<STargetSelectorHudParams>(inst, guid));
    b.records.b_st.starget_selector_hud_params.insert(guid, id);
}
fn extract_record_starget_selector_global_targeting_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<STargetSelectorGlobalTargetingParams> = Handle::new(b.alloc_record::<STargetSelectorGlobalTargetingParams>(inst, guid));
    b.records.b_st.starget_selector_global_targeting_params.insert(guid, id);
}
fn extract_record_ctx_graph<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CtxGraph> = Handle::new(b.alloc_record::<CtxGraph>(inst, guid));
    b.records.b_ct.ctx_graph.insert(guid, id);
}
fn extract_record_soperator_mode_labels<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SOperatorModeLabels> = Handle::new(b.alloc_record::<SOperatorModeLabels>(inst, guid));
    b.records.b_so.soperator_mode_labels.insert(guid, id);
}
fn extract_record_smaster_mode_labels<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SMasterModeLabels> = Handle::new(b.alloc_record::<SMasterModeLabels>(inst, guid));
    b.records.b_sm.smaster_mode_labels.insert(guid, id);
}
fn extract_record_conversation_sticky_filter<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ConversationStickyFilter> = Handle::new(b.alloc_record::<ConversationStickyFilter>(inst, guid));
    b.records.b_co.conversation_sticky_filter.insert(guid, id);
}
fn extract_record_cinematic_conversation_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CinematicConversationSettings> = Handle::new(b.alloc_record::<CinematicConversationSettings>(inst, guid));
    b.records.b_ci.cinematic_conversation_settings.insert(guid, id);
}
fn extract_record_conversation<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Conversation> = Handle::new(b.alloc_record::<Conversation>(inst, guid));
    b.records.b_co.conversation.insert(guid, id);
}
fn extract_record_conversation_bank<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ConversationBank> = Handle::new(b.alloc_record::<ConversationBank>(inst, guid));
    b.records.b_co.conversation_bank.insert(guid, id);
}
fn extract_record_sbezier_curve_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SBezierCurveRecord> = Handle::new(b.alloc_record::<SBezierCurveRecord>(inst, guid));
    b.records.b_sb.sbezier_curve_record.insert(guid, id);
}
fn extract_record_crafting_gameplay_property_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CraftingGameplayPropertyDef> = Handle::new(b.alloc_record::<CraftingGameplayPropertyDef>(inst, guid));
    b.records.b_cr.crafting_gameplay_property_def.insert(guid, id);
}
fn extract_record_blueprint_category_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BlueprintCategoryRecord> = Handle::new(b.alloc_record::<BlueprintCategoryRecord>(inst, guid));
    b.records.b_bl.blueprint_category_record.insert(guid, id);
}
fn extract_record_blueprint_category_database_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BlueprintCategoryDatabaseRecord> = Handle::new(b.alloc_record::<BlueprintCategoryDatabaseRecord>(inst, guid));
    b.records.b_bl.blueprint_category_database_record.insert(guid, id);
}
fn extract_record_crafting_blueprint_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CraftingBlueprintRecord> = Handle::new(b.alloc_record::<CraftingBlueprintRecord>(inst, guid));
    b.records.b_cr.crafting_blueprint_record.insert(guid, id);
}
fn extract_record_crafting_quality_distribution_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CraftingQualityDistributionRecord> = Handle::new(b.alloc_record::<CraftingQualityDistributionRecord>(inst, guid));
    b.records.b_cr.crafting_quality_distribution_record.insert(guid, id);
}
fn extract_record_crafting_quality_location_override_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CraftingQualityLocationOverrideRecord> = Handle::new(b.alloc_record::<CraftingQualityLocationOverrideRecord>(inst, guid));
    b.records.b_cr.crafting_quality_location_override_record.insert(guid, id);
}
fn extract_record_crafting_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CraftingGlobalParams> = Handle::new(b.alloc_record::<CraftingGlobalParams>(inst, guid));
    b.records.b_cr.crafting_global_params.insert(guid, id);
}
fn extract_record_crew_manifest<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CrewManifest> = Handle::new(b.alloc_record::<CrewManifest>(inst, guid));
    b.records.b_cr.crew_manifest.insert(guid, id);
}
fn extract_record_damage_macro<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DamageMacro> = Handle::new(b.alloc_record::<DamageMacro>(inst, guid));
    b.records.b_da.damage_macro.insert(guid, id);
}
fn extract_record_damage_resistance_macro<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DamageResistanceMacro> = Handle::new(b.alloc_record::<DamageResistanceMacro>(inst, guid));
    b.records.b_da.damage_resistance_macro.insert(guid, id);
}
fn extract_record_damage_map_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DamageMapGlobalParams> = Handle::new(b.alloc_record::<DamageMapGlobalParams>(inst, guid));
    b.records.b_da.damage_map_global_params.insert(guid, id);
}
fn extract_record_actor_default_actions_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorDefaultActionsConfig> = Handle::new(b.alloc_record::<ActorDefaultActionsConfig>(inst, guid));
    b.records.b_ac.actor_default_actions_config.insert(guid, id);
}
fn extract_record_default_entitlement_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DefaultEntitlementRecord> = Handle::new(b.alloc_record::<DefaultEntitlementRecord>(inst, guid));
    b.records.b_de.default_entitlement_record.insert(guid, id);
}
fn extract_record_default_player_loadout_entitlement_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DefaultPlayerLoadoutEntitlementRecord> = Handle::new(b.alloc_record::<DefaultPlayerLoadoutEntitlementRecord>(inst, guid));
    b.records.b_de.default_player_loadout_entitlement_record.insert(guid, id);
}
fn extract_record_dematerialize_animation<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DematerializeAnimation> = Handle::new(b.alloc_record::<DematerializeAnimation>(inst, guid));
    b.records.b_de.dematerialize_animation.insert(guid, id);
}
fn extract_record_dev_owner<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DevOwner> = Handle::new(b.alloc_record::<DevOwner>(inst, guid));
    b.records.b_de.dev_owner.insert(guid, id);
}
fn extract_record_dialogue_external_source<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DialogueExternalSource> = Handle::new(b.alloc_record::<DialogueExternalSource>(inst, guid));
    b.records.b_di.dialogue_external_source.insert(guid, id);
}
fn extract_record_dialogue_content<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DialogueContent> = Handle::new(b.alloc_record::<DialogueContent>(inst, guid));
    b.records.b_di.dialogue_content.insert(guid, id);
}
fn extract_record_dialogue_content_bank<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DialogueContentBank> = Handle::new(b.alloc_record::<DialogueContentBank>(inst, guid));
    b.records.b_di.dialogue_content_bank.insert(guid, id);
}
fn extract_record_dialogue_context<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DialogueContext> = Handle::new(b.alloc_record::<DialogueContext>(inst, guid));
    b.records.b_di.dialogue_context.insert(guid, id);
}
fn extract_record_dialogue_context_bank<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DialogueContextBank> = Handle::new(b.alloc_record::<DialogueContextBank>(inst, guid));
    b.records.b_di.dialogue_context_bank.insert(guid, id);
}
fn extract_record_dialogue_realm<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DialogueRealm> = Handle::new(b.alloc_record::<DialogueRealm>(inst, guid));
    b.records.b_di.dialogue_realm.insert(guid, id);
}
fn extract_record_digital_signage_content_set<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DigitalSignageContentSet> = Handle::new(b.alloc_record::<DigitalSignageContentSet>(inst, guid));
    b.records.b_di.digital_signage_content_set.insert(guid, id);
}
fn extract_record_direct_rtt_after_tonemapping_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DirectRTT_AfterTonemappingParams> = Handle::new(b.alloc_record::<DirectRTT_AfterTonemappingParams>(inst, guid));
    b.records.b_di.direct_rtt_after_tonemapping_params.insert(guid, id);
}
fn extract_record_docking_slot_visibility<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DockingSlotVisibility> = Handle::new(b.alloc_record::<DockingSlotVisibility>(inst, guid));
    b.records.b_do.docking_slot_visibility.insert(guid, id);
}
fn extract_record_dynamic_camera_effects<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DynamicCameraEffects> = Handle::new(b.alloc_record::<DynamicCameraEffects>(inst, guid));
    b.records.b_dy.dynamic_camera_effects.insert(guid, id);
}
fn extract_record_dynamic_camera_effects_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DynamicCameraEffectsList> = Handle::new(b.alloc_record::<DynamicCameraEffectsList>(inst, guid));
    b.records.b_dy.dynamic_camera_effects_list.insert(guid, id);
}
fn extract_record_constant_dofglobal_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ConstantDOFGlobalData> = Handle::new(b.alloc_record::<ConstantDOFGlobalData>(inst, guid));
    b.records.b_co.constant_dofglobal_data.insert(guid, id);
}
fn extract_record_seaplayer_loadout_snapshots<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SEAPlayerLoadoutSnapshots> = Handle::new(b.alloc_record::<SEAPlayerLoadoutSnapshots>(inst, guid));
    b.records.b_se.seaplayer_loadout_snapshots.insert(guid, id);
}
fn extract_record_seaglobal_special_loadout<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SEAGlobalSpecialLoadout> = Handle::new(b.alloc_record::<SEAGlobalSpecialLoadout>(inst, guid));
    b.records.b_se.seaglobal_special_loadout.insert(guid, id);
}
fn extract_record_seaglobal_event_loadouts<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SEAGlobalEventLoadouts> = Handle::new(b.alloc_record::<SEAGlobalEventLoadouts>(inst, guid));
    b.records.b_se.seaglobal_event_loadouts.insert(guid, id);
}
fn extract_record_emotion_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EmotionList> = Handle::new(b.alloc_record::<EmotionList>(inst, guid));
    b.records.b_em.emotion_list.insert(guid, id);
}
fn extract_record_entitlement_account_item_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EntitlementAccountItemGlobalParams> = Handle::new(b.alloc_record::<EntitlementAccountItemGlobalParams>(inst, guid));
    b.records.b_en.entitlement_account_item_global_params.insert(guid, id);
}
fn extract_record_entitlement_non_inventory_storable_item_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EntitlementNonInventoryStorableItemGlobalParams> = Handle::new(b.alloc_record::<EntitlementNonInventoryStorableItemGlobalParams>(inst, guid));
    b.records.b_en.entitlement_non_inventory_storable_item_global_params.insert(guid, id);
}
fn extract_record_entity_class_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EntityClassDefinition> = Handle::new(b.alloc_record::<EntityClassDefinition>(inst, guid));
    b.records.b_en.entity_class_definition.insert(guid, id);
}
fn extract_record_commodity_type<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommodityType> = Handle::new(b.alloc_record::<CommodityType>(inst, guid));
    b.records.b_co.commodity_type.insert(guid, id);
}
fn extract_record_commodity_subtype<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommoditySubtype> = Handle::new(b.alloc_record::<CommoditySubtype>(inst, guid));
    b.records.b_co.commodity_subtype.insert(guid, id);
}
fn extract_record_commodity_type_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommodityTypeDatabase> = Handle::new(b.alloc_record::<CommodityTypeDatabase>(inst, guid));
    b.records.b_co.commodity_type_database.insert(guid, id);
}
fn extract_record_commodity_damage_configuration<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommodityDamageConfiguration> = Handle::new(b.alloc_record::<CommodityDamageConfiguration>(inst, guid));
    b.records.b_co.commodity_damage_configuration.insert(guid, id);
}
fn extract_record_sentrances_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SEntrancesDef> = Handle::new(b.alloc_record::<SEntrancesDef>(inst, guid));
    b.records.b_se.sentrances_def.insert(guid, id);
}
fn extract_record_scarryable_ikinteraction_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCarryableIKInteractionList> = Handle::new(b.alloc_record::<SCarryableIKInteractionList>(inst, guid));
    b.records.b_sc.scarryable_ikinteraction_list.insert(guid, id);
}
fn extract_record_carryable_interactions_metadata_config_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CarryableInteractionsMetadataConfigDef> = Handle::new(b.alloc_record::<CarryableInteractionsMetadataConfigDef>(inst, guid));
    b.records.b_ca.carryable_interactions_metadata_config_def.insert(guid, id);
}
fn extract_record_entity_default_loadout_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EntityDefaultLoadoutParams> = Handle::new(b.alloc_record::<EntityDefaultLoadoutParams>(inst, guid));
    b.records.b_en.entity_default_loadout_params.insert(guid, id);
}
fn extract_record_sloadout_assortment<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SLoadoutAssortment> = Handle::new(b.alloc_record::<SLoadoutAssortment>(inst, guid));
    b.records.b_sl.sloadout_assortment.insert(guid, id);
}
fn extract_record_sifcsmodifiers_legacy<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SIFCSModifiersLegacy> = Handle::new(b.alloc_record::<SIFCSModifiersLegacy>(inst, guid));
    b.records.b_si.sifcsmodifiers_legacy.insert(guid, id);
}
fn extract_record_espparams<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ESPParams> = Handle::new(b.alloc_record::<ESPParams>(inst, guid));
    b.records.b_es.espparams.insert(guid, id);
}
fn extract_record_sifcsesp<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SIFCSEsp> = Handle::new(b.alloc_record::<SIFCSEsp>(inst, guid));
    b.records.b_si.sifcsesp.insert(guid, id);
}
fn extract_record_sifcsgame_mode_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SIFCSGameModeParams> = Handle::new(b.alloc_record::<SIFCSGameModeParams>(inst, guid));
    b.records.b_si.sifcsgame_mode_params.insert(guid, id);
}
fn extract_record_svibration_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SVibrationDef> = Handle::new(b.alloc_record::<SVibrationDef>(inst, guid));
    b.records.b_sv.svibration_def.insert(guid, id);
}
fn extract_record_svibration_vehicle_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SVibrationVehicleDef> = Handle::new(b.alloc_record::<SVibrationVehicleDef>(inst, guid));
    b.records.b_sv.svibration_vehicle_def.insert(guid, id);
}
fn extract_record_vibration_audio_point_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VibrationAudioPointDef> = Handle::new(b.alloc_record::<VibrationAudioPointDef>(inst, guid));
    b.records.b_vi.vibration_audio_point_def.insert(guid, id);
}
fn extract_record_global_gas_cloud_vdbparams<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalGasCloudVDBParams> = Handle::new(b.alloc_record::<GlobalGasCloudVDBParams>(inst, guid));
    b.records.b_gl.global_gas_cloud_vdbparams.insert(guid, id);
}
fn extract_record_sentity_density_class<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SEntityDensityClass> = Handle::new(b.alloc_record::<SEntityDensityClass>(inst, guid));
    b.records.b_se.sentity_density_class.insert(guid, id);
}
fn extract_record_sentity_density_class_overrides_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SEntityDensityClassOverridesRecord> = Handle::new(b.alloc_record::<SEntityDensityClassOverridesRecord>(inst, guid));
    b.records.b_se.sentity_density_class_overrides_record.insert(guid, id);
}
fn extract_record_sentity_traversing_node_id<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SEntityTraversingNodeId> = Handle::new(b.alloc_record::<SEntityTraversingNodeId>(inst, guid));
    b.records.b_se.sentity_traversing_node_id.insert(guid, id);
}
fn extract_record_faction<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Faction> = Handle::new(b.alloc_record::<Faction>(inst, guid));
    b.records.b_fa.faction.insert(guid, id);
}
fn extract_record_faction_palettes<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FactionPalettes> = Handle::new(b.alloc_record::<FactionPalettes>(inst, guid));
    b.records.b_fa.faction_palettes.insert(guid, id);
}
fn extract_record_faction_palette<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FactionPalette> = Handle::new(b.alloc_record::<FactionPalette>(inst, guid));
    b.records.b_fa.faction_palette.insert(guid, id);
}
fn extract_record_faction_reputation<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FactionReputation> = Handle::new(b.alloc_record::<FactionReputation>(inst, guid));
    b.records.b_fa.faction_reputation.insert(guid, id);
}
fn extract_record_faction_legacy<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Faction_LEGACY> = Handle::new(b.alloc_record::<Faction_LEGACY>(inst, guid));
    b.records.b_fa.faction_legacy.insert(guid, id);
}
fn extract_record_fidget_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FidgetConfig> = Handle::new(b.alloc_record::<FidgetConfig>(inst, guid));
    b.records.b_fi.fidget_config.insert(guid, id);
}
fn extract_record_fire_hazard_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FireHazardGlobalParams> = Handle::new(b.alloc_record::<FireHazardGlobalParams>(inst, guid));
    b.records.b_fi.fire_hazard_global_params.insert(guid, id);
}
fn extract_record_flash_object_binding_group<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FlashObjectBindingGroup> = Handle::new(b.alloc_record::<FlashObjectBindingGroup>(inst, guid));
    b.records.b_fl.flash_object_binding_group.insert(guid, id);
}
fn extract_record_foley_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FoleyDefinition> = Handle::new(b.alloc_record::<FoleyDefinition>(inst, guid));
    b.records.b_fo.foley_definition.insert(guid, id);
}
fn extract_record_foley_bone<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FoleyBone> = Handle::new(b.alloc_record::<FoleyBone>(inst, guid));
    b.records.b_fo.foley_bone.insert(guid, id);
}
fn extract_record_foley_axis<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FoleyAxis> = Handle::new(b.alloc_record::<FoleyAxis>(inst, guid));
    b.records.b_fo.foley_axis.insert(guid, id);
}
fn extract_record_foley_footstep_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FoleyFootstepDefinition> = Handle::new(b.alloc_record::<FoleyFootstepDefinition>(inst, guid));
    b.records.b_fo.foley_footstep_definition.insert(guid, id);
}
fn extract_record_formation<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Formation> = Handle::new(b.alloc_record::<Formation>(inst, guid));
    b.records.b_fo.formation.insert(guid, id);
}
fn extract_record_friend_manager_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FriendManagerGlobalParams> = Handle::new(b.alloc_record::<FriendManagerGlobalParams>(inst, guid));
    b.records.b_fr.friend_manager_global_params.insert(guid, id);
}
fn extract_record_frontend_override_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FrontendOverrideParams> = Handle::new(b.alloc_record::<FrontendOverrideParams>(inst, guid));
    b.records.b_fr.frontend_override_params.insert(guid, id);
}
fn extract_record_audio_signal_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AudioSignalList> = Handle::new(b.alloc_record::<AudioSignalList>(inst, guid));
    b.records.b_au.audio_signal_list.insert(guid, id);
}
fn extract_record_tag_to_audio_rtpc_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TagToAudioRtpcList> = Handle::new(b.alloc_record::<TagToAudioRtpcList>(inst, guid));
    b.records.b_ta.tag_to_audio_rtpc_list.insert(guid, id);
}
fn extract_record_force_feedback<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ForceFeedback> = Handle::new(b.alloc_record::<ForceFeedback>(inst, guid));
    b.records.b_fo.force_feedback.insert(guid, id);
}
fn extract_record_game_module<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GameModule> = Handle::new(b.alloc_record::<GameModule>(inst, guid));
    b.records.b_ga.game_module.insert(guid, id);
}
fn extract_record_chat_channel_filter_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ChatChannelFilterRecord> = Handle::new(b.alloc_record::<ChatChannelFilterRecord>(inst, guid));
    b.records.b_ch.chat_channel_filter_record.insert(guid, id);
}
fn extract_record_voice_channel_settings_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VoiceChannelSettingsRecord> = Handle::new(b.alloc_record::<VoiceChannelSettingsRecord>(inst, guid));
    b.records.b_vo.voice_channel_settings_record.insert(guid, id);
}
fn extract_record_game_mode<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GameMode> = Handle::new(b.alloc_record::<GameMode>(inst, guid));
    b.records.b_ga.game_mode.insert(guid, id);
}
fn extract_record_game_difficulty_modifiers<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GameDifficultyModifiers> = Handle::new(b.alloc_record::<GameDifficultyModifiers>(inst, guid));
    b.records.b_ga.game_difficulty_modifiers.insert(guid, id);
}
fn extract_record_sview_distance_ratio_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SViewDistanceRatioParams> = Handle::new(b.alloc_record::<SViewDistanceRatioParams>(inst, guid));
    b.records.b_sv.sview_distance_ratio_params.insert(guid, id);
}
fn extract_record_sgeometry_view_distance_ratio_categories<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SGeometryViewDistanceRatioCategories> = Handle::new(b.alloc_record::<SGeometryViewDistanceRatioCategories>(inst, guid));
    b.records.b_sg.sgeometry_view_distance_ratio_categories.insert(guid, id);
}
fn extract_record_sglobal_charge_drain_beam_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SGlobalChargeDrainBeamParams> = Handle::new(b.alloc_record::<SGlobalChargeDrainBeamParams>(inst, guid));
    b.records.b_sg.sglobal_charge_drain_beam_params.insert(guid, id);
}
fn extract_record_sglobal_crosshair_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SGlobalCrosshairParams> = Handle::new(b.alloc_record::<SGlobalCrosshairParams>(inst, guid));
    b.records.b_sg.sglobal_crosshair_params.insert(guid, id);
}
fn extract_record_sglobal_cuttable_shape_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SGlobalCuttableShapeParams> = Handle::new(b.alloc_record::<SGlobalCuttableShapeParams>(inst, guid));
    b.records.b_sg.sglobal_cuttable_shape_params.insert(guid, id);
}
fn extract_record_scuttable_shape_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCuttableShapeDefinition> = Handle::new(b.alloc_record::<SCuttableShapeDefinition>(inst, guid));
    b.records.b_sc.scuttable_shape_definition.insert(guid, id);
}
fn extract_record_sglobal_electron_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SGlobalElectronParams> = Handle::new(b.alloc_record::<SGlobalElectronParams>(inst, guid));
    b.records.b_sg.sglobal_electron_params.insert(guid, id);
}
fn extract_record_sglobal_healing_beam_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SGlobalHealingBeamParams> = Handle::new(b.alloc_record::<SGlobalHealingBeamParams>(inst, guid));
    b.records.b_sg.sglobal_healing_beam_params.insert(guid, id);
}
fn extract_record_sglobal_salvage_repair_beam_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SGlobalSalvageRepairBeamParams> = Handle::new(b.alloc_record::<SGlobalSalvageRepairBeamParams>(inst, guid));
    b.records.b_sg.sglobal_salvage_repair_beam_params.insert(guid, id);
}
fn extract_record_global_shop_commodity_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalShopCommodityParams> = Handle::new(b.alloc_record::<GlobalShopCommodityParams>(inst, guid));
    b.records.b_gl.global_shop_commodity_params.insert(guid, id);
}
fn extract_record_global_shop_terminal_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalShopTerminalParams> = Handle::new(b.alloc_record::<GlobalShopTerminalParams>(inst, guid));
    b.records.b_gl.global_shop_terminal_params.insert(guid, id);
}
fn extract_record_global_shop_selling_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalShopSellingParams> = Handle::new(b.alloc_record::<GlobalShopSellingParams>(inst, guid));
    b.records.b_gl.global_shop_selling_params.insert(guid, id);
}
fn extract_record_global_shop_buying_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalShopBuyingParams> = Handle::new(b.alloc_record::<GlobalShopBuyingParams>(inst, guid));
    b.records.b_gl.global_shop_buying_params.insert(guid, id);
}
fn extract_record_sglobal_tractor_beam_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SGlobalTractorBeamParams> = Handle::new(b.alloc_record::<SGlobalTractorBeamParams>(inst, guid));
    b.records.b_sg.sglobal_tractor_beam_params.insert(guid, id);
}
fn extract_record_grip<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Grip> = Handle::new(b.alloc_record::<Grip>(inst, guid));
    b.records.b_gr.grip.insert(guid, id);
}
fn extract_record_hardware_mouse_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HardwareMouseParams> = Handle::new(b.alloc_record::<HardwareMouseParams>(inst, guid));
    b.records.b_ha.hardware_mouse_params.insert(guid, id);
}
fn extract_record_initial_damage_override<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InitialDamageOverride> = Handle::new(b.alloc_record::<InitialDamageOverride>(inst, guid));
    b.records.b_in.initial_damage_override.insert(guid, id);
}
fn extract_record_body_part<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BodyPart> = Handle::new(b.alloc_record::<BodyPart>(inst, guid));
    b.records.b_bo.body_part.insert(guid, id);
}
fn extract_record_body_mapping<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BodyMapping> = Handle::new(b.alloc_record::<BodyMapping>(inst, guid));
    b.records.b_bo.body_mapping.insert(guid, id);
}
fn extract_record_body_health_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BodyHealthConfig> = Handle::new(b.alloc_record::<BodyHealthConfig>(inst, guid));
    b.records.b_bo.body_health_config.insert(guid, id);
}
fn extract_record_health_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HealthTemplate> = Handle::new(b.alloc_record::<HealthTemplate>(inst, guid));
    b.records.b_he.health_template.insert(guid, id);
}
fn extract_record_hint_uidata<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HintUIData> = Handle::new(b.alloc_record::<HintUIData>(inst, guid));
    b.records.b_hi.hint_uidata.insert(guid, id);
}
fn extract_record_hint_trigger_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HintTriggerData> = Handle::new(b.alloc_record::<HintTriggerData>(inst, guid));
    b.records.b_hi.hint_trigger_data.insert(guid, id);
}
fn extract_record_sglobal_hit_behavior_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SGlobalHitBehaviorParams> = Handle::new(b.alloc_record::<SGlobalHitBehaviorParams>(inst, guid));
    b.records.b_sg.sglobal_hit_behavior_params.insert(guid, id);
}
fn extract_record_hologram_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HologramParams> = Handle::new(b.alloc_record::<HologramParams>(inst, guid));
    b.records.b_ho.hologram_params.insert(guid, id);
}
fn extract_record_inner_thought_anim<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InnerThought_Anim> = Handle::new(b.alloc_record::<InnerThought_Anim>(inst, guid));
    b.records.b_in.inner_thought_anim.insert(guid, id);
}
fn extract_record_inner_thought_color_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InnerThought_ColorParams> = Handle::new(b.alloc_record::<InnerThought_ColorParams>(inst, guid));
    b.records.b_in.inner_thought_color_params.insert(guid, id);
}
fn extract_record_geom_font_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GeomFont_Config> = Handle::new(b.alloc_record::<GeomFont_Config>(inst, guid));
    b.records.b_ge.geom_font_config.insert(guid, id);
}
fn extract_record_inner_thought_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InnerThought_Params> = Handle::new(b.alloc_record::<InnerThought_Params>(inst, guid));
    b.records.b_in.inner_thought_params.insert(guid, id);
}
fn extract_record_inner_thought_conversation_system_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InnerThought_ConversationSystemConfig> = Handle::new(b.alloc_record::<InnerThought_ConversationSystemConfig>(inst, guid));
    b.records.b_in.inner_thought_conversation_system_config.insert(guid, id);
}
fn extract_record_inner_thought_interaction_system_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InnerThought_InteractionSystemConfig> = Handle::new(b.alloc_record::<InnerThought_InteractionSystemConfig>(inst, guid));
    b.records.b_in.inner_thought_interaction_system_config.insert(guid, id);
}
fn extract_record_inner_thought_legacy_use_system_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InnerThought_LegacyUseSystemConfig> = Handle::new(b.alloc_record::<InnerThought_LegacyUseSystemConfig>(inst, guid));
    b.records.b_in.inner_thought_legacy_use_system_config.insert(guid, id);
}
fn extract_record_input_prompt_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InputPromptConfig> = Handle::new(b.alloc_record::<InputPromptConfig>(inst, guid));
    b.records.b_in.input_prompt_config.insert(guid, id);
}
fn extract_record_qterequest_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<QTERequestConfig> = Handle::new(b.alloc_record::<QTERequestConfig>(inst, guid));
    b.records.b_qt.qterequest_config.insert(guid, id);
}
fn extract_record_instanced_interior_location_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InstancedInteriorLocationParams> = Handle::new(b.alloc_record::<InstancedInteriorLocationParams>(inst, guid));
    b.records.b_in.instanced_interior_location_params.insert(guid, id);
}
fn extract_record_instanced_interior_location_map<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InstancedInteriorLocationMap> = Handle::new(b.alloc_record::<InstancedInteriorLocationMap>(inst, guid));
    b.records.b_in.instanced_interior_location_map.insert(guid, id);
}
fn extract_record_ship_insurance_policy_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ShipInsurancePolicyRecord> = Handle::new(b.alloc_record::<ShipInsurancePolicyRecord>(inst, guid));
    b.records.b_sh.ship_insurance_policy_record.insert(guid, id);
}
fn extract_record_skin_interactable_templates<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SkinInteractableTemplates> = Handle::new(b.alloc_record::<SkinInteractableTemplates>(inst, guid));
    b.records.b_sk.skin_interactable_templates.insert(guid, id);
}
fn extract_record_interaction_condition_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InteractionConditionPreset> = Handle::new(b.alloc_record::<InteractionConditionPreset>(inst, guid));
    b.records.b_in.interaction_condition_preset.insert(guid, id);
}
fn extract_record_interaction_point_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InteractionPointTemplate> = Handle::new(b.alloc_record::<InteractionPointTemplate>(inst, guid));
    b.records.b_in.interaction_point_template.insert(guid, id);
}
fn extract_record_inventory_container_manager<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InventoryContainerManager> = Handle::new(b.alloc_record::<InventoryContainerManager>(inst, guid));
    b.records.b_in.inventory_container_manager.insert(guid, id);
}
fn extract_record_inventory_container<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InventoryContainer> = Handle::new(b.alloc_record::<InventoryContainer>(inst, guid));
    b.records.b_in.inventory_container.insert(guid, id);
}
fn extract_record_landing_zone_inventory<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LandingZoneInventory> = Handle::new(b.alloc_record::<LandingZoneInventory>(inst, guid));
    b.records.b_la.landing_zone_inventory.insert(guid, id);
}
fn extract_record_item<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Item> = Handle::new(b.alloc_record::<Item>(inst, guid));
    b.records.b_it.item.insert(guid, id);
}
fn extract_record_item_kiosk_brand<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ItemKioskBrand> = Handle::new(b.alloc_record::<ItemKioskBrand>(inst, guid));
    b.records.b_it.item_kiosk_brand.insert(guid, id);
}
fn extract_record_item_port_tags_dictionary<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ItemPortTagsDictionary> = Handle::new(b.alloc_record::<ItemPortTagsDictionary>(inst, guid));
    b.records.b_it.item_port_tags_dictionary.insert(guid, id);
}
fn extract_record_item_resource_network_global<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ItemResourceNetworkGlobal> = Handle::new(b.alloc_record::<ItemResourceNetworkGlobal>(inst, guid));
    b.records.b_it.item_resource_network_global.insert(guid, id);
}
fn extract_record_journal_entry_type<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<JournalEntryType> = Handle::new(b.alloc_record::<JournalEntryType>(inst, guid));
    b.records.b_jo.journal_entry_type.insert(guid, id);
}
fn extract_record_journal_entry<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<JournalEntry> = Handle::new(b.alloc_record::<JournalEntry>(inst, guid));
    b.records.b_jo.journal_entry.insert(guid, id);
}
fn extract_record_jump_fall_land_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<JumpFallLandConfig> = Handle::new(b.alloc_record::<JumpFallLandConfig>(inst, guid));
    b.records.b_ju.jump_fall_land_config.insert(guid, id);
}
fn extract_record_global_jump_point_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalJumpPointParams> = Handle::new(b.alloc_record::<GlobalJumpPointParams>(inst, guid));
    b.records.b_gl.global_jump_point_params.insert(guid, id);
}
fn extract_record_global_jump_tunnel_host_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalJumpTunnelHostParams> = Handle::new(b.alloc_record::<GlobalJumpTunnelHostParams>(inst, guid));
    b.records.b_gl.global_jump_tunnel_host_params.insert(guid, id);
}
fn extract_record_jump_drive_flight_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<JumpDriveFlightParams> = Handle::new(b.alloc_record::<JumpDriveFlightParams>(inst, guid));
    b.records.b_ju.jump_drive_flight_params.insert(guid, id);
}
fn extract_record_jump_tunnel_forces_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<JumpTunnelForcesParams> = Handle::new(b.alloc_record::<JumpTunnelForcesParams>(inst, guid));
    b.records.b_ju.jump_tunnel_forces_params.insert(guid, id);
}
fn extract_record_global_jump_drive_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalJumpDriveParams> = Handle::new(b.alloc_record::<GlobalJumpDriveParams>(inst, guid));
    b.records.b_gl.global_jump_drive_params.insert(guid, id);
}
fn extract_record_jump_travel_camera_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<JumpTravelCameraParams> = Handle::new(b.alloc_record::<JumpTravelCameraParams>(inst, guid));
    b.records.b_ju.jump_travel_camera_params.insert(guid, id);
}
fn extract_record_area_services<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AreaServices> = Handle::new(b.alloc_record::<AreaServices>(inst, guid));
    b.records.b_ar.area_services.insert(guid, id);
}
fn extract_record_global_cargo_loading_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalCargoLoadingParams> = Handle::new(b.alloc_record::<GlobalCargoLoadingParams>(inst, guid));
    b.records.b_gl.global_cargo_loading_params.insert(guid, id);
}
fn extract_record_landing_pad_size<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LandingPadSize> = Handle::new(b.alloc_record::<LandingPadSize>(inst, guid));
    b.records.b_la.landing_pad_size.insert(guid, id);
}
fn extract_record_infraction_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InfractionDefinition> = Handle::new(b.alloc_record::<InfractionDefinition>(inst, guid));
    b.records.b_in.infraction_definition.insert(guid, id);
}
fn extract_record_jurisdiction<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Jurisdiction> = Handle::new(b.alloc_record::<Jurisdiction>(inst, guid));
    b.records.b_ju.jurisdiction.insert(guid, id);
}
fn extract_record_vehicle_serial_number_character_type<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VehicleSerialNumberCharacterType> = Handle::new(b.alloc_record::<VehicleSerialNumberCharacterType>(inst, guid));
    b.records.b_ve.vehicle_serial_number_character_type.insert(guid, id);
}
fn extract_record_vehicle_serial_number_format<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VehicleSerialNumberFormat> = Handle::new(b.alloc_record::<VehicleSerialNumberFormat>(inst, guid));
    b.records.b_ve.vehicle_serial_number_format.insert(guid, id);
}
fn extract_record_level<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Level> = Handle::new(b.alloc_record::<Level>(inst, guid));
    b.records.b_le.level.insert(guid, id);
}
fn extract_record_visor_hud_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VisorHUD_Config> = Handle::new(b.alloc_record::<VisorHUD_Config>(inst, guid));
    b.records.b_vi.visor_hud_config.insert(guid, id);
}
fn extract_record_visor_control_hints_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Visor_ControlHintsConfig> = Handle::new(b.alloc_record::<Visor_ControlHintsConfig>(inst, guid));
    b.records.b_vi.visor_control_hints_config.insert(guid, id);
}
fn extract_record_control_hints_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ControlHints_Preset> = Handle::new(b.alloc_record::<ControlHints_Preset>(inst, guid));
    b.records.b_co.control_hints_preset.insert(guid, id);
}
fn extract_record_local_player_speed_throttle_component<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LocalPlayerSpeedThrottleComponent> = Handle::new(b.alloc_record::<LocalPlayerSpeedThrottleComponent>(inst, guid));
    b.records.b_lo.local_player_speed_throttle_component.insert(guid, id);
}
fn extract_record_long_term_persistence_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LongTermPersistenceGlobalParams> = Handle::new(b.alloc_record::<LongTermPersistenceGlobalParams>(inst, guid));
    b.records.b_lo.long_term_persistence_global_params.insert(guid, id);
}
fn extract_record_loot_archetype<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LootArchetype> = Handle::new(b.alloc_record::<LootArchetype>(inst, guid));
    b.records.b_lo.loot_archetype.insert(guid, id);
}
fn extract_record_loot_table<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LootTable> = Handle::new(b.alloc_record::<LootTable>(inst, guid));
    b.records.b_lo.loot_table.insert(guid, id);
}
fn extract_record_pool_filter_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PoolFilterRecord> = Handle::new(b.alloc_record::<PoolFilterRecord>(inst, guid));
    b.records.b_po.pool_filter_record.insert(guid, id);
}
fn extract_record_loot_table_v3_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LootTableV3Record> = Handle::new(b.alloc_record::<LootTableV3Record>(inst, guid));
    b.records.b_lo.loot_table_v3_record.insert(guid, id);
}
fn extract_record_loot_archetype_v3_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LootArchetypeV3Record> = Handle::new(b.alloc_record::<LootArchetypeV3Record>(inst, guid));
    b.records.b_lo.loot_archetype_v3_record.insert(guid, id);
}
fn extract_record_loot_v3_secondary_choices_single_layer_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LootV3SecondaryChoicesSingleLayerRecord> = Handle::new(b.alloc_record::<LootV3SecondaryChoicesSingleLayerRecord>(inst, guid));
    b.records.b_lo.loot_v3_secondary_choices_single_layer_record.insert(guid, id);
}
fn extract_record_loot_v3_secondary_choices_multi_layer_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LootV3SecondaryChoicesMultiLayerRecord> = Handle::new(b.alloc_record::<LootV3SecondaryChoicesMultiLayerRecord>(inst, guid));
    b.records.b_lo.loot_v3_secondary_choices_multi_layer_record.insert(guid, id);
}
fn extract_record_loot_generation_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LootGenerationGlobalParams> = Handle::new(b.alloc_record::<LootGenerationGlobalParams>(inst, guid));
    b.records.b_lo.loot_generation_global_params.insert(guid, id);
}
fn extract_record_interior_map_section_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<InteriorMapSectionDefinition> = Handle::new(b.alloc_record::<InteriorMapSectionDefinition>(inst, guid));
    b.records.b_in.interior_map_section_definition.insert(guid, id);
}
fn extract_record_marker_tracking_view_mode_parameters<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MarkerTrackingViewModeParameters> = Handle::new(b.alloc_record::<MarkerTrackingViewModeParameters>(inst, guid));
    b.records.b_ma.marker_tracking_view_mode_parameters.insert(guid, id);
}
fn extract_record_marker_tracking_common_map_parameters<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MarkerTrackingCommonMapParameters> = Handle::new(b.alloc_record::<MarkerTrackingCommonMapParameters>(inst, guid));
    b.records.b_ma.marker_tracking_common_map_parameters.insert(guid, id);
}
fn extract_record_marker_decluttering_culling_order<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MarkerDeclutteringCullingOrder> = Handle::new(b.alloc_record::<MarkerDeclutteringCullingOrder>(inst, guid));
    b.records.b_ma.marker_decluttering_culling_order.insert(guid, id);
}
fn extract_record_global_marker_configs<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalMarkerConfigs> = Handle::new(b.alloc_record::<GlobalMarkerConfigs>(inst, guid));
    b.records.b_gl.global_marker_configs.insert(guid, id);
}
fn extract_record_marker_configuration<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Marker_Configuration> = Handle::new(b.alloc_record::<Marker_Configuration>(inst, guid));
    b.records.b_ma.marker_configuration.insert(guid, id);
}
fn extract_record_medical_item_tier_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MedicalItemTierConfig> = Handle::new(b.alloc_record::<MedicalItemTierConfig>(inst, guid));
    b.records.b_me.medical_item_tier_config.insert(guid, id);
}
fn extract_record_mega_map<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MegaMap> = Handle::new(b.alloc_record::<MegaMap>(inst, guid));
    b.records.b_me.mega_map.insert(guid, id);
}
fn extract_record_melee_combat_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MeleeCombatConfig> = Handle::new(b.alloc_record::<MeleeCombatConfig>(inst, guid));
    b.records.b_me.melee_combat_config.insert(guid, id);
}
fn extract_record_aimelee_combat_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AIMeleeCombatConfig> = Handle::new(b.alloc_record::<AIMeleeCombatConfig>(inst, guid));
    b.records.b_ai.aimelee_combat_config.insert(guid, id);
}
fn extract_record_mission_location_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionLocationTemplate> = Handle::new(b.alloc_record::<MissionLocationTemplate>(inst, guid));
    b.records.b_mi.mission_location_template.insert(guid, id);
}
fn extract_record_mission_item<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionItem> = Handle::new(b.alloc_record::<MissionItem>(inst, guid));
    b.records.b_mi.mission_item.insert(guid, id);
}
fn extract_record_mission_organization<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionOrganization> = Handle::new(b.alloc_record::<MissionOrganization>(inst, guid));
    b.records.b_mi.mission_organization.insert(guid, id);
}
fn extract_record_mission_fail_conditions_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionFailConditionsList> = Handle::new(b.alloc_record::<MissionFailConditionsList>(inst, guid));
    b.records.b_mi.mission_fail_conditions_list.insert(guid, id);
}
fn extract_record_contract_generator<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ContractGenerator> = Handle::new(b.alloc_record::<ContractGenerator>(inst, guid));
    b.records.b_co.contract_generator.insert(guid, id);
}
fn extract_record_contract_difficulty_profile<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ContractDifficultyProfile> = Handle::new(b.alloc_record::<ContractDifficultyProfile>(inst, guid));
    b.records.b_co.contract_difficulty_profile.insert(guid, id);
}
fn extract_record_item_award_weightings_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ItemAwardWeightingsRecord> = Handle::new(b.alloc_record::<ItemAwardWeightingsRecord>(inst, guid));
    b.records.b_it.item_award_weightings_record.insert(guid, id);
}
fn extract_record_blueprint_pool_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BlueprintPoolRecord> = Handle::new(b.alloc_record::<BlueprintPoolRecord>(inst, guid));
    b.records.b_bl.blueprint_pool_record.insert(guid, id);
}
fn extract_record_contract_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ContractTemplate> = Handle::new(b.alloc_record::<ContractTemplate>(inst, guid));
    b.records.b_co.contract_template.insert(guid, id);
}
fn extract_record_entity_cluster_id<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EntityClusterId> = Handle::new(b.alloc_record::<EntityClusterId>(inst, guid));
    b.records.b_en.entity_cluster_id.insert(guid, id);
}
fn extract_record_entity_cluster_member<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<EntityClusterMember> = Handle::new(b.alloc_record::<EntityClusterMember>(inst, guid));
    b.records.b_en.entity_cluster_member.insert(guid, id);
}
fn extract_record_hauling_entity_classes<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Hauling_EntityClasses> = Handle::new(b.alloc_record::<Hauling_EntityClasses>(inst, guid));
    b.records.b_ha.hauling_entity_classes.insert(guid, id);
}
fn extract_record_location_resource_slot<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LocationResourceSlot> = Handle::new(b.alloc_record::<LocationResourceSlot>(inst, guid));
    b.records.b_lo.location_resource_slot.insert(guid, id);
}
fn extract_record_location_entity_declaration<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LocationEntityDeclaration> = Handle::new(b.alloc_record::<LocationEntityDeclaration>(inst, guid));
    b.records.b_lo.location_entity_declaration.insert(guid, id);
}
fn extract_record_module_declaration<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ModuleDeclaration> = Handle::new(b.alloc_record::<ModuleDeclaration>(inst, guid));
    b.records.b_mo.module_declaration.insert(guid, id);
}
fn extract_record_mission_module_hierarchy<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionModuleHierarchy> = Handle::new(b.alloc_record::<MissionModuleHierarchy>(inst, guid));
    b.records.b_mi.mission_module_hierarchy.insert(guid, id);
}
fn extract_record_mission_scenario<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionScenario> = Handle::new(b.alloc_record::<MissionScenario>(inst, guid));
    b.records.b_mi.mission_scenario.insert(guid, id);
}
fn extract_record_beacons_contracts<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<BeaconsContracts> = Handle::new(b.alloc_record::<BeaconsContracts>(inst, guid));
    b.records.b_be.beacons_contracts.insert(guid, id);
}
fn extract_record_mission_type<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionType> = Handle::new(b.alloc_record::<MissionType>(inst, guid));
    b.records.b_mi.mission_type.insert(guid, id);
}
fn extract_record_mission_locality<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionLocality> = Handle::new(b.alloc_record::<MissionLocality>(inst, guid));
    b.records.b_mi.mission_locality.insert(guid, id);
}
fn extract_record_mission_broker_entry<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionBrokerEntry> = Handle::new(b.alloc_record::<MissionBrokerEntry>(inst, guid));
    b.records.b_mi.mission_broker_entry.insert(guid, id);
}
fn extract_record_global_mission_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalMissionSettings> = Handle::new(b.alloc_record::<GlobalMissionSettings>(inst, guid));
    b.records.b_gl.global_mission_settings.insert(guid, id);
}
fn extract_record_mission_giver<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissionGiver> = Handle::new(b.alloc_record::<MissionGiver>(inst, guid));
    b.records.b_mi.mission_giver.insert(guid, id);
}
fn extract_record_armode_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ARModeSettings> = Handle::new(b.alloc_record::<ARModeSettings>(inst, guid));
    b.records.b_ar.armode_settings.insert(guid, id);
}
fn extract_record_mobi_glas_app<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<mobiGlasApp> = Handle::new(b.alloc_record::<mobiGlasApp>(inst, guid));
    b.records.b_mo.mobi_glas_app.insert(guid, id);
}
fn extract_record_music_logic_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MusicLogicConfig> = Handle::new(b.alloc_record::<MusicLogicConfig>(inst, guid));
    b.records.b_mu.music_logic_config.insert(guid, id);
}
fn extract_record_music_logic_parameter<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MusicLogicParameter> = Handle::new(b.alloc_record::<MusicLogicParameter>(inst, guid));
    b.records.b_mu.music_logic_parameter.insert(guid, id);
}
fn extract_record_music_logic_event<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MusicLogicEvent> = Handle::new(b.alloc_record::<MusicLogicEvent>(inst, guid));
    b.records.b_mu.music_logic_event.insert(guid, id);
}
fn extract_record_music_logic_event_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MusicLogicEventList> = Handle::new(b.alloc_record::<MusicLogicEventList>(inst, guid));
    b.records.b_mu.music_logic_event_list.insert(guid, id);
}
fn extract_record_music_logic_switch_value<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MusicLogicSwitchValue> = Handle::new(b.alloc_record::<MusicLogicSwitchValue>(inst, guid));
    b.records.b_mu.music_logic_switch_value.insert(guid, id);
}
fn extract_record_music_logic_suite<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MusicLogicSuite> = Handle::new(b.alloc_record::<MusicLogicSuite>(inst, guid));
    b.records.b_mu.music_logic_suite.insert(guid, id);
}
fn extract_record_notification_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<NotificationDef> = Handle::new(b.alloc_record::<NotificationDef>(inst, guid));
    b.records.b_no.notification_def.insert(guid, id);
}
fn extract_record_comms_notification_stage<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommsNotificationStage> = Handle::new(b.alloc_record::<CommsNotificationStage>(inst, guid));
    b.records.b_co.comms_notification_stage.insert(guid, id);
}
fn extract_record_comms_notification<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommsNotification> = Handle::new(b.alloc_record::<CommsNotification>(inst, guid));
    b.records.b_co.comms_notification.insert(guid, id);
}
fn extract_record_comms_notifications_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommsNotificationsGlobalParams> = Handle::new(b.alloc_record::<CommsNotificationsGlobalParams>(inst, guid));
    b.records.b_co.comms_notifications_global_params.insert(guid, id);
}
fn extract_record_game_notification_dock_item_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GameNotificationDockItemParams> = Handle::new(b.alloc_record::<GameNotificationDockItemParams>(inst, guid));
    b.records.b_ga.game_notification_dock_item_params.insert(guid, id);
}
fn extract_record_sandbox_trigger_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SandboxTriggerRecord> = Handle::new(b.alloc_record::<SandboxTriggerRecord>(inst, guid));
    b.records.b_sa.sandbox_trigger_record.insert(guid, id);
}
fn extract_record_gpuparticle_audio<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GPUParticleAudio> = Handle::new(b.alloc_record::<GPUParticleAudio>(inst, guid));
    b.records.b_gp.gpuparticle_audio.insert(guid, id);
}
fn extract_record_gpuparticle_audio_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GPUParticleAudioList> = Handle::new(b.alloc_record::<GPUParticleAudioList>(inst, guid));
    b.records.b_gp.gpuparticle_audio_list.insert(guid, id);
}
fn extract_record_sperk_reputation_list_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SPerkReputationListParams> = Handle::new(b.alloc_record::<SPerkReputationListParams>(inst, guid));
    b.records.b_sp.sperk_reputation_list_params.insert(guid, id);
}
fn extract_record_planet_effect_lod<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlanetEffectLOD> = Handle::new(b.alloc_record::<PlanetEffectLOD>(inst, guid));
    b.records.b_pl.planet_effect_lod.insert(guid, id);
}
fn extract_record_player_animated_interaction_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerAnimatedInteractionTemplate> = Handle::new(b.alloc_record::<PlayerAnimatedInteractionTemplate>(inst, guid));
    b.records.b_pl.player_animated_interaction_template.insert(guid, id);
}
fn extract_record_player_animated_interaction_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerAnimatedInteractionConfig> = Handle::new(b.alloc_record::<PlayerAnimatedInteractionConfig>(inst, guid));
    b.records.b_pl.player_animated_interaction_config.insert(guid, id);
}
fn extract_record_player_choice_signal_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerChoice_SignalConfig> = Handle::new(b.alloc_record::<PlayerChoice_SignalConfig>(inst, guid));
    b.records.b_pl.player_choice_signal_config.insert(guid, id);
}
fn extract_record_player_choice_library<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerChoice_Library> = Handle::new(b.alloc_record::<PlayerChoice_Library>(inst, guid));
    b.records.b_pl.player_choice_library.insert(guid, id);
}
fn extract_record_player_choice_imconfig<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerChoice_IMConfig> = Handle::new(b.alloc_record::<PlayerChoice_IMConfig>(inst, guid));
    b.records.b_pl.player_choice_imconfig.insert(guid, id);
}
fn extract_record_player_choice_menu_item<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerChoiceMenuItem> = Handle::new(b.alloc_record::<PlayerChoiceMenuItem>(inst, guid));
    b.records.b_pl.player_choice_menu_item.insert(guid, id);
}
fn extract_record_player_choice_menu_items<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerChoiceMenuItems> = Handle::new(b.alloc_record::<PlayerChoiceMenuItems>(inst, guid));
    b.records.b_pl.player_choice_menu_items.insert(guid, id);
}
fn extract_record_player_choice_menu<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerChoiceMenu> = Handle::new(b.alloc_record::<PlayerChoiceMenu>(inst, guid));
    b.records.b_pl.player_choice_menu.insert(guid, id);
}
fn extract_record_player_choice_menu_type<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerChoiceMenuType> = Handle::new(b.alloc_record::<PlayerChoiceMenuType>(inst, guid));
    b.records.b_pl.player_choice_menu_type.insert(guid, id);
}
fn extract_record_ledge_grabbing_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LedgeGrabbingParams> = Handle::new(b.alloc_record::<LedgeGrabbingParams>(inst, guid));
    b.records.b_le.ledge_grabbing_params.insert(guid, id);
}
fn extract_record_actor_targeted_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorTargetedParams> = Handle::new(b.alloc_record::<ActorTargetedParams>(inst, guid));
    b.records.b_ac.actor_targeted_params.insert(guid, id);
}
fn extract_record_actor_stance_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorStanceConfig> = Handle::new(b.alloc_record::<ActorStanceConfig>(inst, guid));
    b.records.b_ac.actor_stance_config.insert(guid, id);
}
fn extract_record_jump_fall_land_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<JumpFallLandParams> = Handle::new(b.alloc_record::<JumpFallLandParams>(inst, guid));
    b.records.b_ju.jump_fall_land_params.insert(guid, id);
}
fn extract_record_player_dock_context_component_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerDockContextComponentGlobalParams> = Handle::new(b.alloc_record::<PlayerDockContextComponentGlobalParams>(inst, guid));
    b.records.b_pl.player_dock_context_component_global_params.insert(guid, id);
}
fn extract_record_player_group_manager_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerGroupManagerGlobalParams> = Handle::new(b.alloc_record::<PlayerGroupManagerGlobalParams>(inst, guid));
    b.records.b_pl.player_group_manager_global_params.insert(guid, id);
}
fn extract_record_player_limitations_profile<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerLimitationsProfile> = Handle::new(b.alloc_record::<PlayerLimitationsProfile>(inst, guid));
    b.records.b_pl.player_limitations_profile.insert(guid, id);
}
fn extract_record_player_notification_banner_manager_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerNotificationBannerManagerGlobalParams> = Handle::new(b.alloc_record::<PlayerNotificationBannerManagerGlobalParams>(inst, guid));
    b.records.b_pl.player_notification_banner_manager_global_params.insert(guid, id);
}
fn extract_record_player_ship_respawn<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerShipRespawn> = Handle::new(b.alloc_record::<PlayerShipRespawn>(inst, guid));
    b.records.b_pl.player_ship_respawn.insert(guid, id);
}
fn extract_record_player_trade_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerTradeGlobalParams> = Handle::new(b.alloc_record::<PlayerTradeGlobalParams>(inst, guid));
    b.records.b_pl.player_trade_global_params.insert(guid, id);
}
fn extract_record_point_of_interest_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PointOfInterestList> = Handle::new(b.alloc_record::<PointOfInterestList>(inst, guid));
    b.records.b_po.point_of_interest_list.insert(guid, id);
}
fn extract_record_posture_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PostureDatabase> = Handle::new(b.alloc_record::<PostureDatabase>(inst, guid));
    b.records.b_po.posture_database.insert(guid, id);
}
fn extract_record_proc_breathing_curve<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProcBreathingCurve> = Handle::new(b.alloc_record::<ProcBreathingCurve>(inst, guid));
    b.records.b_pr.proc_breathing_curve.insert(guid, id);
}
fn extract_record_proc_breathing_curve_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProcBreathingCurveDatabase> = Handle::new(b.alloc_record::<ProcBreathingCurveDatabase>(inst, guid));
    b.records.b_pr.proc_breathing_curve_database.insert(guid, id);
}
fn extract_record_proc_breathing_setup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProcBreathingSetup> = Handle::new(b.alloc_record::<ProcBreathingSetup>(inst, guid));
    b.records.b_pr.proc_breathing_setup.insert(guid, id);
}
fn extract_record_procedural_aim_rig_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProceduralAimRigRecord> = Handle::new(b.alloc_record::<ProceduralAimRigRecord>(inst, guid));
    b.records.b_pr.procedural_aim_rig_record.insert(guid, id);
}
fn extract_record_procedural_animation<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProceduralAnimation> = Handle::new(b.alloc_record::<ProceduralAnimation>(inst, guid));
    b.records.b_pr.procedural_animation.insert(guid, id);
}
fn extract_record_planet_day_night_temperature_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlanetDayNightTemperatureTemplate> = Handle::new(b.alloc_record::<PlanetDayNightTemperatureTemplate>(inst, guid));
    b.records.b_pl.planet_day_night_temperature_template.insert(guid, id);
}
fn extract_record_procedural_landing_setup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProceduralLandingSetup> = Handle::new(b.alloc_record::<ProceduralLandingSetup>(inst, guid));
    b.records.b_pr.procedural_landing_setup.insert(guid, id);
}
fn extract_record_procedural_layout_graph<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ProceduralLayoutGraph> = Handle::new(b.alloc_record::<ProceduralLayoutGraph>(inst, guid));
    b.records.b_pr.procedural_layout_graph.insert(guid, id);
}
fn extract_record_sprojected_hud_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SProjectedHudParams> = Handle::new(b.alloc_record::<SProjectedHudParams>(inst, guid));
    b.records.b_sp.sprojected_hud_params.insert(guid, id);
}
fn extract_record_svehicle_hud_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SVehicleHudParams> = Handle::new(b.alloc_record::<SVehicleHudParams>(inst, guid));
    b.records.b_sv.svehicle_hud_params.insert(guid, id);
}
fn extract_record_quantum_drive_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<QuantumDriveGlobalParams> = Handle::new(b.alloc_record::<QuantumDriveGlobalParams>(inst, guid));
    b.records.b_qu.quantum_drive_global_params.insert(guid, id);
}
fn extract_record_squantum_drive_effect_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SQuantumDriveEffectTemplate> = Handle::new(b.alloc_record::<SQuantumDriveEffectTemplate>(inst, guid));
    b.records.b_sq.squantum_drive_effect_template.insert(guid, id);
}
fn extract_record_quantum_drive_effect_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<QuantumDriveEffectSettings> = Handle::new(b.alloc_record::<QuantumDriveEffectSettings>(inst, guid));
    b.records.b_qu.quantum_drive_effect_settings.insert(guid, id);
}
fn extract_record_squantum_camera_effects_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SQuantumCameraEffectsDef> = Handle::new(b.alloc_record::<SQuantumCameraEffectsDef>(inst, guid));
    b.records.b_sq.squantum_camera_effects_def.insert(guid, id);
}
fn extract_record_ra_sta_rlibrary_element<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RaSTaRLibraryElement> = Handle::new(b.alloc_record::<RaSTaRLibraryElement>(inst, guid));
    b.records.b_ra.ra_sta_rlibrary_element.insert(guid, id);
}
fn extract_record_ra_sta_rlibrary<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RaSTaRLibrary> = Handle::new(b.alloc_record::<RaSTaRLibrary>(inst, guid));
    b.records.b_ra.ra_sta_rlibrary.insert(guid, id);
}
fn extract_record_world_display_radar<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WorldDisplayRadar> = Handle::new(b.alloc_record::<WorldDisplayRadar>(inst, guid));
    b.records.b_wo.world_display_radar.insert(guid, id);
}
fn extract_record_radar_display_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarDisplay_Config> = Handle::new(b.alloc_record::<RadarDisplay_Config>(inst, guid));
    b.records.b_ra.radar_display_config.insert(guid, id);
}
fn extract_record_radar_system_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarSystemGlobalParams> = Handle::new(b.alloc_record::<RadarSystemGlobalParams>(inst, guid));
    b.records.b_ra.radar_system_global_params.insert(guid, id);
}
fn extract_record_radar_system_shared_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarSystemSharedParams> = Handle::new(b.alloc_record::<RadarSystemSharedParams>(inst, guid));
    b.records.b_ra.radar_system_shared_params.insert(guid, id);
}
fn extract_record_scan_information_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ScanInformationDef> = Handle::new(b.alloc_record::<ScanInformationDef>(inst, guid));
    b.records.b_sc.scan_information_def.insert(guid, id);
}
fn extract_record_scan_information_table<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ScanInformationTable> = Handle::new(b.alloc_record::<ScanInformationTable>(inst, guid));
    b.records.b_sc.scan_information_table.insert(guid, id);
}
fn extract_record_scan_custom_data_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ScanCustomDataDef> = Handle::new(b.alloc_record::<ScanCustomDataDef>(inst, guid));
    b.records.b_sc.scan_custom_data_def.insert(guid, id);
}
fn extract_record_scan_display_instance_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ScanDisplayInstanceParams> = Handle::new(b.alloc_record::<ScanDisplayInstanceParams>(inst, guid));
    b.records.b_sc.scan_display_instance_params.insert(guid, id);
}
fn extract_record_scan_display_layout_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ScanDisplayLayoutParams> = Handle::new(b.alloc_record::<ScanDisplayLayoutParams>(inst, guid));
    b.records.b_sc.scan_display_layout_params.insert(guid, id);
}
fn extract_record_radar_signature_category_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarSignatureCategoryDefinition> = Handle::new(b.alloc_record::<RadarSignatureCategoryDefinition>(inst, guid));
    b.records.b_ra.radar_signature_category_definition.insert(guid, id);
}
fn extract_record_radar_signature_category_entry<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarSignatureCategoryEntry> = Handle::new(b.alloc_record::<RadarSignatureCategoryEntry>(inst, guid));
    b.records.b_ra.radar_signature_category_entry.insert(guid, id);
}
fn extract_record_radar_contact_type_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarContactTypeDefinition> = Handle::new(b.alloc_record::<RadarContactTypeDefinition>(inst, guid));
    b.records.b_ra.radar_contact_type_definition.insert(guid, id);
}
fn extract_record_radar_contact_type_entry<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarContactTypeEntry> = Handle::new(b.alloc_record::<RadarContactTypeEntry>(inst, guid));
    b.records.b_ra.radar_contact_type_entry.insert(guid, id);
}
fn extract_record_radar_contact_group_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarContactGroupDefinition> = Handle::new(b.alloc_record::<RadarContactGroupDefinition>(inst, guid));
    b.records.b_ra.radar_contact_group_definition.insert(guid, id);
}
fn extract_record_radar_contact_group_entry<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarContactGroupEntry> = Handle::new(b.alloc_record::<RadarContactGroupEntry>(inst, guid));
    b.records.b_ra.radar_contact_group_entry.insert(guid, id);
}
fn extract_record_radar_delta_signature_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarDeltaSignatureDefinition> = Handle::new(b.alloc_record::<RadarDeltaSignatureDefinition>(inst, guid));
    b.records.b_ra.radar_delta_signature_definition.insert(guid, id);
}
fn extract_record_radar_delta_signature_entry<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarDeltaSignatureEntry> = Handle::new(b.alloc_record::<RadarDeltaSignatureEntry>(inst, guid));
    b.records.b_ra.radar_delta_signature_entry.insert(guid, id);
}
fn extract_record_refining_process<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RefiningProcess> = Handle::new(b.alloc_record::<RefiningProcess>(inst, guid));
    b.records.b_re.refining_process.insert(guid, id);
}
fn extract_record_refinery_notification_configuration<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RefineryNotificationConfiguration> = Handle::new(b.alloc_record::<RefineryNotificationConfiguration>(inst, guid));
    b.records.b_re.refinery_notification_configuration.insert(guid, id);
}
fn extract_record_rental_notification_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RentalNotificationParams> = Handle::new(b.alloc_record::<RentalNotificationParams>(inst, guid));
    b.records.b_re.rental_notification_params.insert(guid, id);
}
fn extract_record_sreputation_standing_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SReputationStandingParams> = Handle::new(b.alloc_record::<SReputationStandingParams>(inst, guid));
    b.records.b_sr.sreputation_standing_params.insert(guid, id);
}
fn extract_record_sreputation_context_ui<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SReputationContextUI> = Handle::new(b.alloc_record::<SReputationContextUI>(inst, guid));
    b.records.b_sr.sreputation_context_ui.insert(guid, id);
}
fn extract_record_sreputation_global_context_bbparams<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SReputationGlobalContextBBParams> = Handle::new(b.alloc_record::<SReputationGlobalContextBBParams>(inst, guid));
    b.records.b_sr.sreputation_global_context_bbparams.insert(guid, id);
}
fn extract_record_sreputation_state_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SReputationStateParams> = Handle::new(b.alloc_record::<SReputationStateParams>(inst, guid));
    b.records.b_sr.sreputation_state_params.insert(guid, id);
}
fn extract_record_sreputation_state_mission_result_modifier_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SReputationStateMissionResultModifierParams> = Handle::new(b.alloc_record::<SReputationStateMissionResultModifierParams>(inst, guid));
    b.records.b_sr.sreputation_state_mission_result_modifier_params.insert(guid, id);
}
fn extract_record_sreputation_scope_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SReputationScopeParams> = Handle::new(b.alloc_record::<SReputationScopeParams>(inst, guid));
    b.records.b_sr.sreputation_scope_params.insert(guid, id);
}
fn extract_record_sreputation_reward_amount<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SReputationRewardAmount> = Handle::new(b.alloc_record::<SReputationRewardAmount>(inst, guid));
    b.records.b_sr.sreputation_reward_amount.insert(guid, id);
}
fn extract_record_sreputation_mission_reward_bonus_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SReputationMissionRewardBonusParams> = Handle::new(b.alloc_record::<SReputationMissionRewardBonusParams>(inst, guid));
    b.records.b_sr.sreputation_mission_reward_bonus_params.insert(guid, id);
}
fn extract_record_sreputation_journal_entry_handler_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SReputationJournalEntryHandlerParams> = Handle::new(b.alloc_record::<SReputationJournalEntryHandlerParams>(inst, guid));
    b.records.b_sr.sreputation_journal_entry_handler_params.insert(guid, id);
}
fn extract_record_reputation_value_setting<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ReputationValueSetting> = Handle::new(b.alloc_record::<ReputationValueSetting>(inst, guid));
    b.records.b_re.reputation_value_setting.insert(guid, id);
}
fn extract_record_reputation_value_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ReputationValueSettings> = Handle::new(b.alloc_record::<ReputationValueSettings>(inst, guid));
    b.records.b_re.reputation_value_settings.insert(guid, id);
}
fn extract_record_resource_type<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ResourceType> = Handle::new(b.alloc_record::<ResourceType>(inst, guid));
    b.records.b_re.resource_type.insert(guid, id);
}
fn extract_record_resource_type_group<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ResourceTypeGroup> = Handle::new(b.alloc_record::<ResourceTypeGroup>(inst, guid));
    b.records.b_re.resource_type_group.insert(guid, id);
}
fn extract_record_resource_type_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ResourceTypeDatabase> = Handle::new(b.alloc_record::<ResourceTypeDatabase>(inst, guid));
    b.records.b_re.resource_type_database.insert(guid, id);
}
fn extract_record_actor_restrain_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorRestrainConfig> = Handle::new(b.alloc_record::<ActorRestrainConfig>(inst, guid));
    b.records.b_ac.actor_restrain_config.insert(guid, id);
}
fn extract_record_gas_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GasParams> = Handle::new(b.alloc_record::<GasParams>(inst, guid));
    b.records.b_ga.gas_params.insert(guid, id);
}
fn extract_record_atmospheric_composition_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AtmosphericCompositionTemplate> = Handle::new(b.alloc_record::<AtmosphericCompositionTemplate>(inst, guid));
    b.records.b_at.atmospheric_composition_template.insert(guid, id);
}
fn extract_record_global_gas_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalGasParams> = Handle::new(b.alloc_record::<GlobalGasParams>(inst, guid));
    b.records.b_gl.global_gas_params.insert(guid, id);
}
fn extract_record_global_room_state_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalRoomStateParams> = Handle::new(b.alloc_record::<GlobalRoomStateParams>(inst, guid));
    b.records.b_gl.global_room_state_params.insert(guid, id);
}
fn extract_record_asteroid_state_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AsteroidStateTemplate> = Handle::new(b.alloc_record::<AsteroidStateTemplate>(inst, guid));
    b.records.b_as.asteroid_state_template.insert(guid, id);
}
fn extract_record_asteroid_behavior<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AsteroidBehavior> = Handle::new(b.alloc_record::<AsteroidBehavior>(inst, guid));
    b.records.b_as.asteroid_behavior.insert(guid, id);
}
fn extract_record_atmosphere_state_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AtmosphereStateTemplate> = Handle::new(b.alloc_record::<AtmosphereStateTemplate>(inst, guid));
    b.records.b_at.atmosphere_state_template.insert(guid, id);
}
fn extract_record_atmosphere_state_pressure_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AtmosphereStatePressureTemplate> = Handle::new(b.alloc_record::<AtmosphereStatePressureTemplate>(inst, guid));
    b.records.b_at.atmosphere_state_pressure_template.insert(guid, id);
}
fn extract_record_atmosphere_state_temperature_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AtmosphereStateTemperatureTemplate> = Handle::new(b.alloc_record::<AtmosphereStateTemperatureTemplate>(inst, guid));
    b.records.b_at.atmosphere_state_temperature_template.insert(guid, id);
}
fn extract_record_atmosphere_state_humidity_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AtmosphereStateHumidityTemplate> = Handle::new(b.alloc_record::<AtmosphereStateHumidityTemplate>(inst, guid));
    b.records.b_at.atmosphere_state_humidity_template.insert(guid, id);
}
fn extract_record_atmosphere_behavior<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AtmosphereBehavior> = Handle::new(b.alloc_record::<AtmosphereBehavior>(inst, guid));
    b.records.b_at.atmosphere_behavior.insert(guid, id);
}
fn extract_record_electrical_state_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ElectricalStateTemplate> = Handle::new(b.alloc_record::<ElectricalStateTemplate>(inst, guid));
    b.records.b_el.electrical_state_template.insert(guid, id);
}
fn extract_record_electrical_behavior<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ElectricalBehavior> = Handle::new(b.alloc_record::<ElectricalBehavior>(inst, guid));
    b.records.b_el.electrical_behavior.insert(guid, id);
}
fn extract_record_radiation_state_template<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadiationStateTemplate> = Handle::new(b.alloc_record::<RadiationStateTemplate>(inst, guid));
    b.records.b_ra.radiation_state_template.insert(guid, id);
}
fn extract_record_radiation_behavior<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadiationBehavior> = Handle::new(b.alloc_record::<RadiationBehavior>(inst, guid));
    b.records.b_ra.radiation_behavior.insert(guid, id);
}
fn extract_record_actor_view_limit_preset_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorViewLimitPresetDatabase> = Handle::new(b.alloc_record::<ActorViewLimitPresetDatabase>(inst, guid));
    b.records.b_ac.actor_view_limit_preset_database.insert(guid, id);
}
fn extract_record_actor_look_limits<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorLookLimits> = Handle::new(b.alloc_record::<ActorLookLimits>(inst, guid));
    b.records.b_ac.actor_look_limits.insert(guid, id);
}
fn extract_record_actor_aim_limits<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ActorAimLimits> = Handle::new(b.alloc_record::<ActorAimLimits>(inst, guid));
    b.records.b_ac.actor_aim_limits.insert(guid, id);
}
fn extract_record_hazard_awareness_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HazardAwarenessParams> = Handle::new(b.alloc_record::<HazardAwarenessParams>(inst, guid));
    b.records.b_ha.hazard_awareness_params.insert(guid, id);
}
fn extract_record_sanalytics_event<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SAnalyticsEvent> = Handle::new(b.alloc_record::<SAnalyticsEvent>(inst, guid));
    b.records.b_sa.sanalytics_event.insert(guid, id);
}
fn extract_record_sanalytics_event_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SAnalyticsEventDatabase> = Handle::new(b.alloc_record::<SAnalyticsEventDatabase>(inst, guid));
    b.records.b_sa.sanalytics_event_database.insert(guid, id);
}
fn extract_record_scitem_light_amplification<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCItemLightAmplification> = Handle::new(b.alloc_record::<SCItemLightAmplification>(inst, guid));
    b.records.b_sc.scitem_light_amplification.insert(guid, id);
}
fn extract_record_scitem_manufacturer<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCItemManufacturer> = Handle::new(b.alloc_record::<SCItemManufacturer>(inst, guid));
    b.records.b_sc.scitem_manufacturer.insert(guid, id);
}
fn extract_record_saimable_gimbal_mode_labels<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SAimableGimbalModeLabels> = Handle::new(b.alloc_record::<SAimableGimbalModeLabels>(inst, guid));
    b.records.b_sa.saimable_gimbal_mode_labels.insert(guid, id);
}
fn extract_record_saimable_game_mode_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SAimableGameModeParams> = Handle::new(b.alloc_record::<SAimableGameModeParams>(inst, guid));
    b.records.b_sa.saimable_game_mode_params.insert(guid, id);
}
fn extract_record_saimable_controller_hud_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SAimableControllerHudParams> = Handle::new(b.alloc_record::<SAimableControllerHudParams>(inst, guid));
    b.records.b_sa.saimable_controller_hud_params.insert(guid, id);
}
fn extract_record_capacitor_assignment_input_output_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CapacitorAssignmentInputOutputDef> = Handle::new(b.alloc_record::<CapacitorAssignmentInputOutputDef>(inst, guid));
    b.records.b_ca.capacitor_assignment_input_output_def.insert(guid, id);
}
fn extract_record_scitem_visor_dashboard_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCItemVisorDashboardConfig> = Handle::new(b.alloc_record::<SCItemVisorDashboardConfig>(inst, guid));
    b.records.b_sc.scitem_visor_dashboard_config.insert(guid, id);
}
fn extract_record_scitem_suit_fuel_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCItemSuitFuelParams> = Handle::new(b.alloc_record::<SCItemSuitFuelParams>(inst, guid));
    b.records.b_sc.scitem_suit_fuel_params.insert(guid, id);
}
fn extract_record_explosive_ordnance_ping_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ExplosiveOrdnancePingGlobalParams> = Handle::new(b.alloc_record::<ExplosiveOrdnancePingGlobalParams>(inst, guid));
    b.records.b_ex.explosive_ordnance_ping_global_params.insert(guid, id);
}
fn extract_record_move_view_restriction_penalty<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MoveViewRestrictionPenalty> = Handle::new(b.alloc_record::<MoveViewRestrictionPenalty>(inst, guid));
    b.records.b_mo.move_view_restriction_penalty.insert(guid, id);
}
fn extract_record_consumable_type<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ConsumableType> = Handle::new(b.alloc_record::<ConsumableType>(inst, guid));
    b.records.b_co.consumable_type.insert(guid, id);
}
fn extract_record_consumable_subtype<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ConsumableSubtype> = Handle::new(b.alloc_record::<ConsumableSubtype>(inst, guid));
    b.records.b_co.consumable_subtype.insert(guid, id);
}
fn extract_record_consumable_type_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ConsumableTypeDatabase> = Handle::new(b.alloc_record::<ConsumableTypeDatabase>(inst, guid));
    b.records.b_co.consumable_type_database.insert(guid, id);
}
fn extract_record_comms_audio_effect<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommsAudioEffect> = Handle::new(b.alloc_record::<CommsAudioEffect>(inst, guid));
    b.records.b_co.comms_audio_effect.insert(guid, id);
}
fn extract_record_scitem_comms_component_setup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCItemCommsComponentSetup> = Handle::new(b.alloc_record::<SCItemCommsComponentSetup>(inst, guid));
    b.records.b_sc.scitem_comms_component_setup.insert(guid, id);
}
fn extract_record_ship_computer_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ShipComputerDefinition> = Handle::new(b.alloc_record::<ShipComputerDefinition>(inst, guid));
    b.records.b_sh.ship_computer_definition.insert(guid, id);
}
fn extract_record_smovable_limits<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SMovableLimits> = Handle::new(b.alloc_record::<SMovableLimits>(inst, guid));
    b.records.b_sm.smovable_limits.insert(guid, id);
}
fn extract_record_jump_thruster_pack_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<JumpThrusterPackConfig> = Handle::new(b.alloc_record::<JumpThrusterPackConfig>(inst, guid));
    b.records.b_ju.jump_thruster_pack_config.insert(guid, id);
}
fn extract_record_scitem_display_screen_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCItemDisplayScreenPreset> = Handle::new(b.alloc_record::<SCItemDisplayScreenPreset>(inst, guid));
    b.records.b_sc.scitem_display_screen_preset.insert(guid, id);
}
fn extract_record_docking_sensitivity<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<DockingSensitivity> = Handle::new(b.alloc_record::<DockingSensitivity>(inst, guid));
    b.records.b_do.docking_sensitivity.insert(guid, id);
}
fn extract_record_world_display_environment<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WorldDisplayEnvironment> = Handle::new(b.alloc_record::<WorldDisplayEnvironment>(inst, guid));
    b.records.b_wo.world_display_environment.insert(guid, id);
}
fn extract_record_mining_controller_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MiningControllerGlobalParams> = Handle::new(b.alloc_record::<MiningControllerGlobalParams>(inst, guid));
    b.records.b_mi.mining_controller_global_params.insert(guid, id);
}
fn extract_record_vehicle_salvage_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VehicleSalvageGlobalParams> = Handle::new(b.alloc_record::<VehicleSalvageGlobalParams>(inst, guid));
    b.records.b_ve.vehicle_salvage_global_params.insert(guid, id);
}
fn extract_record_seat_reticle_archetype<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SeatReticleArchetype> = Handle::new(b.alloc_record::<SeatReticleArchetype>(inst, guid));
    b.records.b_se.seat_reticle_archetype.insert(guid, id);
}
fn extract_record_scseat_head_pos_adjust_setup<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCSeatHeadPosAdjustSetup> = Handle::new(b.alloc_record::<SCSeatHeadPosAdjustSetup>(inst, guid));
    b.records.b_sc.scseat_head_pos_adjust_setup.insert(guid, id);
}
fn extract_record_scitem_seat_head_tracking_position_limit_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCItemSeatHeadTrackingPositionLimitParams> = Handle::new(b.alloc_record::<SCItemSeatHeadTrackingPositionLimitParams>(inst, guid));
    b.records.b_sc.scitem_seat_head_tracking_position_limit_params.insert(guid, id);
}
fn extract_record_armor_move_view_restrictions<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ArmorMoveViewRestrictions> = Handle::new(b.alloc_record::<ArmorMoveViewRestrictions>(inst, guid));
    b.records.b_ar.armor_move_view_restrictions.insert(guid, id);
}
fn extract_record_seat_ads_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SeatAdsDef> = Handle::new(b.alloc_record::<SeatAdsDef>(inst, guid));
    b.records.b_se.seat_ads_def.insert(guid, id);
}
fn extract_record_seat_user_actor_cdikrecord<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SeatUserActorCDIKRecord> = Handle::new(b.alloc_record::<SeatUserActorCDIKRecord>(inst, guid));
    b.records.b_se.seat_user_actor_cdikrecord.insert(guid, id);
}
fn extract_record_smfdmode_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SMFDModeConfig> = Handle::new(b.alloc_record::<SMFDModeConfig>(inst, guid));
    b.records.b_sm.smfdmode_config.insert(guid, id);
}
fn extract_record_smfdview<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SMFDView> = Handle::new(b.alloc_record::<SMFDView>(inst, guid));
    b.records.b_sm.smfdview.insert(guid, id);
}
fn extract_record_smfdview_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SMFDViewList> = Handle::new(b.alloc_record::<SMFDViewList>(inst, guid));
    b.records.b_sm.smfdview_list.insert(guid, id);
}
fn extract_record_smfdparams_diagnostics<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SMFDParamsDiagnostics> = Handle::new(b.alloc_record::<SMFDParamsDiagnostics>(inst, guid));
    b.records.b_sm.smfdparams_diagnostics.insert(guid, id);
}
fn extract_record_animated_helmet_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<AnimatedHelmetParams> = Handle::new(b.alloc_record::<AnimatedHelmetParams>(inst, guid));
    b.records.b_an.animated_helmet_params.insert(guid, id);
}
fn extract_record_sturret_health_modifier_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<STurretHealthModifierDef> = Handle::new(b.alloc_record::<STurretHealthModifierDef>(inst, guid));
    b.records.b_st.sturret_health_modifier_def.insert(guid, id);
}
fn extract_record_sturret_esp<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<STurretESP> = Handle::new(b.alloc_record::<STurretESP>(inst, guid));
    b.records.b_st.sturret_esp.insert(guid, id);
}
fn extract_record_sturret_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<STurretGlobalParams> = Handle::new(b.alloc_record::<STurretGlobalParams>(inst, guid));
    b.records.b_st.sturret_global_params.insert(guid, id);
}
fn extract_record_weapon_aimable_angles_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WeaponAimableAnglesDef> = Handle::new(b.alloc_record::<WeaponAimableAnglesDef>(inst, guid));
    b.records.b_we.weapon_aimable_angles_def.insert(guid, id);
}
fn extract_record_weapon_gimbal_mode_modifier_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WeaponGimbalModeModifierDef> = Handle::new(b.alloc_record::<WeaponGimbalModeModifierDef>(inst, guid));
    b.records.b_we.weapon_gimbal_mode_modifier_def.insert(guid, id);
}
fn extract_record_weapon_misfire_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WeaponMisfireDef> = Handle::new(b.alloc_record::<WeaponMisfireDef>(inst, guid));
    b.records.b_we.weapon_misfire_def.insert(guid, id);
}
fn extract_record_weapon_armodifier<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WeaponARModifier> = Handle::new(b.alloc_record::<WeaponARModifier>(inst, guid));
    b.records.b_we.weapon_armodifier.insert(guid, id);
}
fn extract_record_player_to_player_comms_call_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerToPlayerCommsCallGlobalParams> = Handle::new(b.alloc_record::<PlayerToPlayerCommsCallGlobalParams>(inst, guid));
    b.records.b_pl.player_to_player_comms_call_global_params.insert(guid, id);
}
fn extract_record_personal_inner_thought_action_rule_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PersonalInnerThoughtActionRulePreset> = Handle::new(b.alloc_record::<PersonalInnerThoughtActionRulePreset>(inst, guid));
    b.records.b_pe.personal_inner_thought_action_rule_preset.insert(guid, id);
}
fn extract_record_corpse_interaction_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CorpseInteractionParams> = Handle::new(b.alloc_record::<CorpseInteractionParams>(inst, guid));
    b.records.b_co.corpse_interaction_params.insert(guid, id);
}
fn extract_record_item_recovery_configuration_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ItemRecoveryConfigurationParams> = Handle::new(b.alloc_record::<ItemRecoveryConfigurationParams>(inst, guid));
    b.records.b_it.item_recovery_configuration_params.insert(guid, id);
}
fn extract_record_player_choice_pitconfig<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerChoice_PITConfig> = Handle::new(b.alloc_record::<PlayerChoice_PITConfig>(inst, guid));
    b.records.b_pl.player_choice_pitconfig.insert(guid, id);
}
fn extract_record_slocal_player_shopping_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SLocalPlayerShoppingData> = Handle::new(b.alloc_record::<SLocalPlayerShoppingData>(inst, guid));
    b.records.b_sl.slocal_player_shopping_data.insert(guid, id);
}
fn extract_record_shop_interaction_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ShopInteractionData> = Handle::new(b.alloc_record::<ShopInteractionData>(inst, guid));
    b.records.b_sh.shop_interaction_data.insert(guid, id);
}
fn extract_record_shop_franchise<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ShopFranchise> = Handle::new(b.alloc_record::<ShopFranchise>(inst, guid));
    b.records.b_sh.shop_franchise.insert(guid, id);
}
fn extract_record_sscsignature_system_audio_ruleset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SSCSignatureSystemAudioRuleset> = Handle::new(b.alloc_record::<SSCSignatureSystemAudioRuleset>(inst, guid));
    b.records.b_ss.sscsignature_system_audio_ruleset.insert(guid, id);
}
fn extract_record_scenario_progress<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ScenarioProgress> = Handle::new(b.alloc_record::<ScenarioProgress>(inst, guid));
    b.records.b_sc.scenario_progress.insert(guid, id);
}
fn extract_record_screen_effects_library<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ScreenEffects_Library> = Handle::new(b.alloc_record::<ScreenEffects_Library>(inst, guid));
    b.records.b_sc.screen_effects_library.insert(guid, id);
}
fn extract_record_screen_effects_effect<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ScreenEffects_Effect> = Handle::new(b.alloc_record::<ScreenEffects_Effect>(inst, guid));
    b.records.b_sc.screen_effects_effect.insert(guid, id);
}
fn extract_record_screen_effects_debug<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ScreenEffects_Debug> = Handle::new(b.alloc_record::<ScreenEffects_Debug>(inst, guid));
    b.records.b_sc.screen_effects_debug.insert(guid, id);
}
fn extract_record_security_clearance_token<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SecurityClearanceToken> = Handle::new(b.alloc_record::<SecurityClearanceToken>(inst, guid));
    b.records.b_se.security_clearance_token.insert(guid, id);
}
fn extract_record_security_network_room_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SecurityNetworkRoomSettings> = Handle::new(b.alloc_record::<SecurityNetworkRoomSettings>(inst, guid));
    b.records.b_se.security_network_room_settings.insert(guid, id);
}
fn extract_record_security_network_manifest<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SecurityNetworkManifest> = Handle::new(b.alloc_record::<SecurityNetworkManifest>(inst, guid));
    b.records.b_se.security_network_manifest.insert(guid, id);
}
fn extract_record_service_beacon_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ServiceBeaconParams> = Handle::new(b.alloc_record::<ServiceBeaconParams>(inst, guid));
    b.records.b_se.service_beacon_params.insert(guid, id);
}
fn extract_record_service_beacon_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ServiceBeaconGlobalParams> = Handle::new(b.alloc_record::<ServiceBeaconGlobalParams>(inst, guid));
    b.records.b_se.service_beacon_global_params.insert(guid, id);
}
fn extract_record_shield_type_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ShieldTypeParams> = Handle::new(b.alloc_record::<ShieldTypeParams>(inst, guid));
    b.records.b_sh.shield_type_params.insert(guid, id);
}
fn extract_record_ship_computer_preset_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ShipComputerPresetList> = Handle::new(b.alloc_record::<ShipComputerPresetList>(inst, guid));
    b.records.b_sh.ship_computer_preset_list.insert(guid, id);
}
fn extract_record_ship_computer_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ShipComputerPreset> = Handle::new(b.alloc_record::<ShipComputerPreset>(inst, guid));
    b.records.b_sh.ship_computer_preset.insert(guid, id);
}
fn extract_record_ssolar_system<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SSolarSystem> = Handle::new(b.alloc_record::<SSolarSystem>(inst, guid));
    b.records.b_ss.ssolar_system.insert(guid, id);
}
fn extract_record_spawn_descriptions<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SpawnDescriptions> = Handle::new(b.alloc_record::<SpawnDescriptions>(inst, guid));
    b.records.b_sp.spawn_descriptions.insert(guid, id);
}
fn extract_record_special_event_manufacturer<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SpecialEventManufacturer> = Handle::new(b.alloc_record::<SpecialEventManufacturer>(inst, guid));
    b.records.b_sp.special_event_manufacturer.insert(guid, id);
}
fn extract_record_special_event_day<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SpecialEventDay> = Handle::new(b.alloc_record::<SpecialEventDay>(inst, guid));
    b.records.b_sp.special_event_day.insert(guid, id);
}
fn extract_record_special_event_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SpecialEventDatabase> = Handle::new(b.alloc_record::<SpecialEventDatabase>(inst, guid));
    b.records.b_sp.special_event_database.insert(guid, id);
}
fn extract_record_star_map_object_type<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<StarMapObjectType> = Handle::new(b.alloc_record::<StarMapObjectType>(inst, guid));
    b.records.b_st.star_map_object_type.insert(guid, id);
}
fn extract_record_star_map_object_types<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<StarMapObjectTypes> = Handle::new(b.alloc_record::<StarMapObjectTypes>(inst, guid));
    b.records.b_st.star_map_object_types.insert(guid, id);
}
fn extract_record_star_map_amenity_type_entry<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<StarMapAmenityTypeEntry> = Handle::new(b.alloc_record::<StarMapAmenityTypeEntry>(inst, guid));
    b.records.b_st.star_map_amenity_type_entry.insert(guid, id);
}
fn extract_record_star_map_amenity_types<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<StarMapAmenityTypes> = Handle::new(b.alloc_record::<StarMapAmenityTypes>(inst, guid));
    b.records.b_st.star_map_amenity_types.insert(guid, id);
}
fn extract_record_star_map_object<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<StarMapObject> = Handle::new(b.alloc_record::<StarMapObject>(inst, guid));
    b.records.b_st.star_map_object.insert(guid, id);
}
fn extract_record_star_map_mission_object<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<StarMapMissionObject> = Handle::new(b.alloc_record::<StarMapMissionObject>(inst, guid));
    b.records.b_st.star_map_mission_object.insert(guid, id);
}
fn extract_record_star_map_party_member_object<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<StarMapPartyMemberObject> = Handle::new(b.alloc_record::<StarMapPartyMemberObject>(inst, guid));
    b.records.b_st.star_map_party_member_object.insert(guid, id);
}
fn extract_record_status_widget_display_preset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<StatusWidgetDisplayPreset> = Handle::new(b.alloc_record::<StatusWidgetDisplayPreset>(inst, guid));
    b.records.b_st.status_widget_display_preset.insert(guid, id);
}
fn extract_record_tactical_query<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TacticalQuery> = Handle::new(b.alloc_record::<TacticalQuery>(inst, guid));
    b.records.b_ta.tactical_query.insert(guid, id);
}
fn extract_record_tqsoption_content_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TQSOptionContentRecord> = Handle::new(b.alloc_record::<TQSOptionContentRecord>(inst, guid));
    b.records.b_tq.tqsoption_content_record.insert(guid, id);
}
fn extract_record_tag<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<Tag> = Handle::new(b.alloc_record::<Tag>(inst, guid));
    b.records.b_ta.tag.insert(guid, id);
}
fn extract_record_tag_database<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TagDatabase> = Handle::new(b.alloc_record::<TagDatabase>(inst, guid));
    b.records.b_ta.tag_database.insert(guid, id);
}
fn extract_record_take_down_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TakeDownConfig> = Handle::new(b.alloc_record::<TakeDownConfig>(inst, guid));
    b.records.b_ta.take_down_config.insert(guid, id);
}
fn extract_record_tint_palette_tree<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TintPaletteTree> = Handle::new(b.alloc_record::<TintPaletteTree>(inst, guid));
    b.records.b_ti.tint_palette_tree.insert(guid, id);
}
fn extract_record_movie_clip_transformation_interpolator<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MovieClipTransformationInterpolator> = Handle::new(b.alloc_record::<MovieClipTransformationInterpolator>(inst, guid));
    b.records.b_mo.movie_clip_transformation_interpolator.insert(guid, id);
}
fn extract_record_transformation_interpolator<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TransformationInterpolator> = Handle::new(b.alloc_record::<TransformationInterpolator>(inst, guid));
    b.records.b_tr.transformation_interpolator.insert(guid, id);
}
fn extract_record_transit_station_announcements<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<TransitStationAnnouncements> = Handle::new(b.alloc_record::<TransitStationAnnouncements>(inst, guid));
    b.records.b_tr.transit_station_announcements.insert(guid, id);
}
fn extract_record_sscene_player_choice_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SScenePlayerChoiceSettings> = Handle::new(b.alloc_record::<SScenePlayerChoiceSettings>(inst, guid));
    b.records.b_ss.sscene_player_choice_settings.insert(guid, id);
}
fn extract_record_global_tutorial_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<GlobalTutorialParams> = Handle::new(b.alloc_record::<GlobalTutorialParams>(inst, guid));
    b.records.b_gl.global_tutorial_params.insert(guid, id);
}
fn extract_record_item_preview_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ItemPreview_Config> = Handle::new(b.alloc_record::<ItemPreview_Config>(inst, guid));
    b.records.b_it.item_preview_config.insert(guid, id);
}
fn extract_record_scobject_data_bank_entry_marker_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCObjectDataBankEntryMarkerConfig> = Handle::new(b.alloc_record::<SCObjectDataBankEntryMarkerConfig>(inst, guid));
    b.records.b_sc.scobject_data_bank_entry_marker_config.insert(guid, id);
}
fn extract_record_uiholo_vehicle_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UIHoloVehicle_Config> = Handle::new(b.alloc_record::<UIHoloVehicle_Config>(inst, guid));
    b.records.b_ui.uiholo_vehicle_config.insert(guid, id);
}
fn extract_record_marker_ar_config_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MarkerAR_ConfigDef> = Handle::new(b.alloc_record::<MarkerAR_ConfigDef>(inst, guid));
    b.records.b_ma.marker_ar_config_def.insert(guid, id);
}
fn extract_record_uiconfig<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UIConfig> = Handle::new(b.alloc_record::<UIConfig>(inst, guid));
    b.records.b_ui.uiconfig.insert(guid, id);
}
fn extract_record_simple_sprite_sheet<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SimpleSpriteSheet> = Handle::new(b.alloc_record::<SimpleSpriteSheet>(inst, guid));
    b.records.b_si.simple_sprite_sheet.insert(guid, id);
}
fn extract_record_hud_colors<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<HudColors> = Handle::new(b.alloc_record::<HudColors>(inst, guid));
    b.records.b_hu.hud_colors.insert(guid, id);
}
fn extract_record_video_comms<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VideoComms> = Handle::new(b.alloc_record::<VideoComms>(inst, guid));
    b.records.b_vi.video_comms.insert(guid, id);
}
fn extract_record_uimode_visibility_settings<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UIModeVisibilitySettings> = Handle::new(b.alloc_record::<UIModeVisibilitySettings>(inst, guid));
    b.records.b_ui.uimode_visibility_settings.insert(guid, id);
}
fn extract_record_loadout_dummy_component_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LoadoutDummyComponentParams> = Handle::new(b.alloc_record::<LoadoutDummyComponentParams>(inst, guid));
    b.records.b_lo.loadout_dummy_component_params.insert(guid, id);
}
fn extract_record_uistate_display<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UIStateDisplay> = Handle::new(b.alloc_record::<UIStateDisplay>(inst, guid));
    b.records.b_ui.uistate_display.insert(guid, id);
}
fn extract_record_popup_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PopupDef> = Handle::new(b.alloc_record::<PopupDef>(inst, guid));
    b.records.b_po.popup_def.insert(guid, id);
}
fn extract_record_player_ecggraph_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<PlayerECGGraph_Config> = Handle::new(b.alloc_record::<PlayerECGGraph_Config>(inst, guid));
    b.records.b_pl.player_ecggraph_config.insert(guid, id);
}
fn extract_record_item_type_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<ItemTypeDefinition> = Handle::new(b.alloc_record::<ItemTypeDefinition>(inst, guid));
    b.records.b_it.item_type_definition.insert(guid, id);
}
fn extract_record_flight_huduiview_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<FlightHUDUIView_Config> = Handle::new(b.alloc_record::<FlightHUDUIView_Config>(inst, guid));
    b.records.b_fl.flight_huduiview_config.insert(guid, id);
}
fn extract_record_uielement<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UIElement> = Handle::new(b.alloc_record::<UIElement>(inst, guid));
    b.records.b_ui.uielement.insert(guid, id);
}
fn extract_record_scitem_uiview_dashboard_canvas_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SCItemUIView_DashboardCanvasDef> = Handle::new(b.alloc_record::<SCItemUIView_DashboardCanvasDef>(inst, guid));
    b.records.b_sc.scitem_uiview_dashboard_canvas_def.insert(guid, id);
}
fn extract_record_missile_lock_reticle_config<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MissileLockReticle_Config> = Handle::new(b.alloc_record::<MissileLockReticle_Config>(inst, guid));
    b.records.b_mi.missile_lock_reticle_config.insert(guid, id);
}
fn extract_record_uielement_sounds_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UIElementSoundsRecord> = Handle::new(b.alloc_record::<UIElementSoundsRecord>(inst, guid));
    b.records.b_ui.uielement_sounds_record.insert(guid, id);
}
fn extract_record_uiaudio_definition<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UIAudioDefinition> = Handle::new(b.alloc_record::<UIAudioDefinition>(inst, guid));
    b.records.b_ui.uiaudio_definition.insert(guid, id);
}
fn extract_record_loadout_editor_component_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LoadoutEditorComponentParams> = Handle::new(b.alloc_record::<LoadoutEditorComponentParams>(inst, guid));
    b.records.b_lo.loadout_editor_component_params.insert(guid, id);
}
fn extract_record_radar_display3_dpreset<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<RadarDisplay3DPreset> = Handle::new(b.alloc_record::<RadarDisplay3DPreset>(inst, guid));
    b.records.b_ra.radar_display3_dpreset.insert(guid, id);
}
fn extract_record_unit_test_sub_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UnitTestSubRecord> = Handle::new(b.alloc_record::<UnitTestSubRecord>(inst, guid));
    b.records.b_un.unit_test_sub_record.insert(guid, id);
}
fn extract_record_unit_test<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UnitTest> = Handle::new(b.alloc_record::<UnitTest>(inst, guid));
    b.records.b_un.unit_test.insert(guid, id);
}
fn extract_record_usable_archetype<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UsableArchetype> = Handle::new(b.alloc_record::<UsableArchetype>(inst, guid));
    b.records.b_us.usable_archetype.insert(guid, id);
}
fn extract_record_use_channel_archetype<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UseChannelArchetype> = Handle::new(b.alloc_record::<UseChannelArchetype>(inst, guid));
    b.records.b_us.use_channel_archetype.insert(guid, id);
}
fn extract_record_usable_archetypes<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<UsableArchetypes> = Handle::new(b.alloc_record::<UsableArchetypes>(inst, guid));
    b.records.b_us.usable_archetypes.insert(guid, id);
}
fn extract_record_svehicle_ai_damage_modifiers<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<SVehicleAiDamageModifiers> = Handle::new(b.alloc_record::<SVehicleAiDamageModifiers>(inst, guid));
    b.records.b_sv.svehicle_ai_damage_modifiers.insert(guid, id);
}
fn extract_record_vehicle_landing_gear_system<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VehicleLandingGearSystem> = Handle::new(b.alloc_record::<VehicleLandingGearSystem>(inst, guid));
    b.records.b_ve.vehicle_landing_gear_system.insert(guid, id);
}
fn extract_record_vehicle_role<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VehicleRole> = Handle::new(b.alloc_record::<VehicleRole>(inst, guid));
    b.records.b_ve.vehicle_role.insert(guid, id);
}
fn extract_record_vehicle_career<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VehicleCareer> = Handle::new(b.alloc_record::<VehicleCareer>(inst, guid));
    b.records.b_ve.vehicle_career.insert(guid, id);
}
fn extract_record_vehicle_career_list<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VehicleCareerList> = Handle::new(b.alloc_record::<VehicleCareerList>(inst, guid));
    b.records.b_ve.vehicle_career_list.insert(guid, id);
}
fn extract_record_comms_channel_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CommsChannelDef> = Handle::new(b.alloc_record::<CommsChannelDef>(inst, guid));
    b.records.b_co.comms_channel_def.insert(guid, id);
}
fn extract_record_video_comms_shader_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VideoCommsShaderParams> = Handle::new(b.alloc_record::<VideoCommsShaderParams>(inst, guid));
    b.records.b_vi.video_comms_shader_params.insert(guid, id);
}
fn extract_record_video_comms_audio_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VideoCommsAudioParams> = Handle::new(b.alloc_record::<VideoCommsAudioParams>(inst, guid));
    b.records.b_vi.video_comms_audio_params.insert(guid, id);
}
fn extract_record_camera_transition_interpolation_curve_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CameraTransitionInterpolationCurveRecord> = Handle::new(b.alloc_record::<CameraTransitionInterpolationCurveRecord>(inst, guid));
    b.records.b_ca.camera_transition_interpolation_curve_record.insert(guid, id);
}
fn extract_record_cinematic_flight_points_record<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<CinematicFlightPointsRecord> = Handle::new(b.alloc_record::<CinematicFlightPointsRecord>(inst, guid));
    b.records.b_ci.cinematic_flight_points_record.insert(guid, id);
}
fn extract_record_visor_lens_layout<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VisorLens_Layout> = Handle::new(b.alloc_record::<VisorLens_Layout>(inst, guid));
    b.records.b_vi.visor_lens_layout.insert(guid, id);
}
fn extract_record_visor_lens_region<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VisorLens_Region> = Handle::new(b.alloc_record::<VisorLens_Region>(inst, guid));
    b.records.b_vi.visor_lens_region.insert(guid, id);
}
fn extract_record_voice_single<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VoiceSingle> = Handle::new(b.alloc_record::<VoiceSingle>(inst, guid));
    b.records.b_vo.voice_single.insert(guid, id);
}
fn extract_record_voice_bundle<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<VoiceBundle> = Handle::new(b.alloc_record::<VoiceBundle>(inst, guid));
    b.records.b_vo.voice_bundle.insert(guid, id);
}
fn extract_record_water_effects_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WaterEffectsGlobalParams> = Handle::new(b.alloc_record::<WaterEffectsGlobalParams>(inst, guid));
    b.records.b_wa.water_effects_global_params.insert(guid, id);
}
fn extract_record_weapon_procedural_animation<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WeaponProceduralAnimation> = Handle::new(b.alloc_record::<WeaponProceduralAnimation>(inst, guid));
    b.records.b_we.weapon_procedural_animation.insert(guid, id);
}
fn extract_record_weapon_procedural_clip<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WeaponProceduralClip> = Handle::new(b.alloc_record::<WeaponProceduralClip>(inst, guid));
    b.records.b_we.weapon_procedural_clip.insert(guid, id);
}
fn extract_record_weapon_procedural_recoil_config_def<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WeaponProceduralRecoilConfigDef> = Handle::new(b.alloc_record::<WeaponProceduralRecoilConfigDef>(inst, guid));
    b.records.b_we.weapon_procedural_recoil_config_def.insert(guid, id);
}
fn extract_record_loadout_kit<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<LoadoutKit> = Handle::new(b.alloc_record::<LoadoutKit>(inst, guid));
    b.records.b_lo.loadout_kit.insert(guid, id);
}
fn extract_record_web_customization_debug<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WebCustomizationDebug> = Handle::new(b.alloc_record::<WebCustomizationDebug>(inst, guid));
    b.records.b_we.web_customization_debug.insert(guid, id);
}
fn extract_record_web_customization_global_params<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<WebCustomizationGlobalParams> = Handle::new(b.alloc_record::<WebCustomizationGlobalParams>(inst, guid));
    b.records.b_we.web_customization_global_params.insert(guid, id);
}
fn extract_record_mobi_glas_app_data<'a>(b: &mut Builder<'a>, guid: CigGuid, inst: Instance<'a>) {
    let id: Handle<MobiGlasAppData> = Handle::new(b.alloc_record::<MobiGlasAppData>(inst, guid));
    b.records.b_mo.mobi_glas_app_data.insert(guid, id);
}

impl<'a> Builder<'a> {
    /// Walk every record in the database and seed the worklist.
    ///
    /// Called from [`Builder::consume_database`]. Resolves the runtime
    /// `struct_index` for every known record type **by name** against
    /// the live DCB so patched binaries remain compatible — game patches
    /// that add or reorder struct types no longer silently drop records.
    ///
    /// # Debug staleness check
    ///
    /// In debug builds (`cfg(debug_assertions)`), this method panics
    /// at the end of the seeding loop if the runtime DCB contains any
    /// record type the generated dispatch table doesn't know about.
    /// That's a direct signal the generator is out of date relative to
    /// the game patch whose DCB is being parsed, and you should
    /// regenerate with `sc-generator`. In release builds the check is
    /// dead-code-eliminated — unknown records are silently skipped and
    /// the app keeps working with whatever subset it does understand.
    pub fn seed_database(&mut self) {
        let n_structs = self.db.struct_definitions().len();
        let name_to_idx: HashMap<&str, usize> = (0..n_structs)
            .filter_map(|i| self.db.struct_name(i).map(|n| (n, i)))
            .collect();

        let mut dispatch: Vec<Option<RecordExtractor<'a>>> = vec![None; n_structs];
        if let Some(&i) = name_to_idx.get(ActivityData::TYPE_NAME) { dispatch[i] = Some(extract_record_activity_data); }
        if let Some(&i) = name_to_idx.get(AIPerceptionProfile::TYPE_NAME) { dispatch[i] = Some(extract_record_aiperception_profile); }
        if let Some(&i) = name_to_idx.get(AIMercyTimerSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_aimercy_timer_settings); }
        if let Some(&i) = name_to_idx.get(AIVisualFieldParams::TYPE_NAME) { dispatch[i] = Some(extract_record_aivisual_field_params); }
        if let Some(&i) = name_to_idx.get(AIVisualFieldProfile::TYPE_NAME) { dispatch[i] = Some(extract_record_aivisual_field_profile); }
        if let Some(&i) = name_to_idx.get(AIObservableFilterFlags::TYPE_NAME) { dispatch[i] = Some(extract_record_aiobservable_filter_flags); }
        if let Some(&i) = name_to_idx.get(AIObservableFiltersProfile::TYPE_NAME) { dispatch[i] = Some(extract_record_aiobservable_filters_profile); }
        if let Some(&i) = name_to_idx.get(MovementSystemAdditionalParamsRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_movement_system_additional_params_record); }
        if let Some(&i) = name_to_idx.get(AITargetableSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_aitargetable_settings); }
        if let Some(&i) = name_to_idx.get(AISpecialRangedAttackConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_aispecial_ranged_attack_config); }
        if let Some(&i) = name_to_idx.get(AIAvailableSpecialRangedAttacksConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_aiavailable_special_ranged_attacks_config); }
        if let Some(&i) = name_to_idx.get(TraversalCostConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_traversal_cost_config); }
        if let Some(&i) = name_to_idx.get(AIFireDisciplineSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_aifire_discipline_settings); }
        if let Some(&i) = name_to_idx.get(AIMotiveList::TYPE_NAME) { dispatch[i] = Some(extract_record_aimotive_list); }
        if let Some(&i) = name_to_idx.get(AIProfile::TYPE_NAME) { dispatch[i] = Some(extract_record_aiprofile); }
        if let Some(&i) = name_to_idx.get(SkillDefinitions::TYPE_NAME) { dispatch[i] = Some(extract_record_skill_definitions); }
        if let Some(&i) = name_to_idx.get(StatDefinitions::TYPE_NAME) { dispatch[i] = Some(extract_record_stat_definitions); }
        if let Some(&i) = name_to_idx.get(AITargetingFormulaSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_aitargeting_formula_settings); }
        if let Some(&i) = name_to_idx.get(AIWaveCollection::TYPE_NAME) { dispatch[i] = Some(extract_record_aiwave_collection); }
        if let Some(&i) = name_to_idx.get(ARMarkerGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_armarker_global_params); }
        if let Some(&i) = name_to_idx.get(ActorAbilityComponent::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_ability_component); }
        if let Some(&i) = name_to_idx.get(ActorDuckingParams::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_ducking_params); }
        if let Some(&i) = name_to_idx.get(EVAGraph::TYPE_NAME) { dispatch[i] = Some(extract_record_evagraph); }
        if let Some(&i) = name_to_idx.get(ActorEnvironmentComponent::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_environment_component); }
        if let Some(&i) = name_to_idx.get(SActorForceReactionsDef::TYPE_NAME) { dispatch[i] = Some(extract_record_sactor_force_reactions_def); }
        if let Some(&i) = name_to_idx.get(SActorHitReactionsDef::TYPE_NAME) { dispatch[i] = Some(extract_record_sactor_hit_reactions_def); }
        if let Some(&i) = name_to_idx.get(SActorExternalForceResponseCameraShakeDef::TYPE_NAME) { dispatch[i] = Some(extract_record_sactor_external_force_response_camera_shake_def); }
        if let Some(&i) = name_to_idx.get(SActorForceReactionsPresetRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_sactor_force_reactions_preset_record); }
        if let Some(&i) = name_to_idx.get(ActorGForceComponent::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_gforce_component); }
        if let Some(&i) = name_to_idx.get(ActorGForceHeadBob::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_gforce_head_bob); }
        if let Some(&i) = name_to_idx.get(ActorGForceCameraEffects::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_gforce_camera_effects); }
        if let Some(&i) = name_to_idx.get(LeanGraph::TYPE_NAME) { dispatch[i] = Some(extract_record_lean_graph); }
        if let Some(&i) = name_to_idx.get(SActorLocomotionFidgetStateFilteredDef::TYPE_NAME) { dispatch[i] = Some(extract_record_sactor_locomotion_fidget_state_filtered_def); }
        if let Some(&i) = name_to_idx.get(SActorLocomotionFidgetDef::TYPE_NAME) { dispatch[i] = Some(extract_record_sactor_locomotion_fidget_def); }
        if let Some(&i) = name_to_idx.get(ActorLocomotionPersonality::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_locomotion_personality); }
        if let Some(&i) = name_to_idx.get(MotionGraph::TYPE_NAME) { dispatch[i] = Some(extract_record_motion_graph); }
        if let Some(&i) = name_to_idx.get(SCProneMotionGraphDef::TYPE_NAME) { dispatch[i] = Some(extract_record_scprone_motion_graph_def); }
        if let Some(&i) = name_to_idx.get(SMannequinActionDefRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_smannequin_action_def_record); }
        if let Some(&i) = name_to_idx.get(ActorMovementModifiers::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_movement_modifiers); }
        if let Some(&i) = name_to_idx.get(ActorMovementSetsConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_movement_sets_config); }
        if let Some(&i) = name_to_idx.get(ActorProceduralRecoilConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_procedural_recoil_config); }
        if let Some(&i) = name_to_idx.get(ActorProceduralRecoilModifiers::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_procedural_recoil_modifiers); }
        if let Some(&i) = name_to_idx.get(ActorSkeletonConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_skeleton_config); }
        if let Some(&i) = name_to_idx.get(ActorSlidingParams::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_sliding_params); }
        if let Some(&i) = name_to_idx.get(ActorSpeedCameraEffects::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_speed_camera_effects); }
        if let Some(&i) = name_to_idx.get(ActorStaminaComponent::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_stamina_component); }
        if let Some(&i) = name_to_idx.get(ActorStanceSpeedsInfo::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_stance_speeds_info); }
        if let Some(&i) = name_to_idx.get(ActorStanceDimensionsInfo::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_stance_dimensions_info); }
        if let Some(&i) = name_to_idx.get(ActorStateValidation::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_state_validation); }
        if let Some(&i) = name_to_idx.get(ActorStatusGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_status_global_params); }
        if let Some(&i) = name_to_idx.get(ActorStatusComponent::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_status_component); }
        if let Some(&i) = name_to_idx.get(ActorZeroGTraversalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_zero_gtraversal_params); }
        if let Some(&i) = name_to_idx.get(ZeroGTraversalGraph::TYPE_NAME) { dispatch[i] = Some(extract_record_zero_gtraversal_graph); }
        if let Some(&i) = name_to_idx.get(AmmoParams::TYPE_NAME) { dispatch[i] = Some(extract_record_ammo_params); }
        if let Some(&i) = name_to_idx.get(AnimatedMarker::TYPE_NAME) { dispatch[i] = Some(extract_record_animated_marker); }
        if let Some(&i) = name_to_idx.get(CombatMarker::TYPE_NAME) { dispatch[i] = Some(extract_record_combat_marker); }
        if let Some(&i) = name_to_idx.get(Announcer::TYPE_NAME) { dispatch[i] = Some(extract_record_announcer); }
        if let Some(&i) = name_to_idx.get(AsteroidFieldComposition::TYPE_NAME) { dispatch[i] = Some(extract_record_asteroid_field_composition); }
        if let Some(&i) = name_to_idx.get(AtmosphericFlightEffects::TYPE_NAME) { dispatch[i] = Some(extract_record_atmospheric_flight_effects); }
        if let Some(&i) = name_to_idx.get(AudioBreathStyleCondition::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_breath_style_condition); }
        if let Some(&i) = name_to_idx.get(AudioBreathStyleConditionList::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_breath_style_condition_list); }
        if let Some(&i) = name_to_idx.get(AudioBreathStyle::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_breath_style); }
        if let Some(&i) = name_to_idx.get(AudioBreathStyleSuite::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_breath_style_suite); }
        if let Some(&i) = name_to_idx.get(AudioBreathDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_breath_definition); }
        if let Some(&i) = name_to_idx.get(AudioBreathInterrupt::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_breath_interrupt); }
        if let Some(&i) = name_to_idx.get(BreathingTriggerDef::TYPE_NAME) { dispatch[i] = Some(extract_record_breathing_trigger_def); }
        if let Some(&i) = name_to_idx.get(EntityAudioControllerTypeParams::TYPE_NAME) { dispatch[i] = Some(extract_record_entity_audio_controller_type_params); }
        if let Some(&i) = name_to_idx.get(EntityAudioControllerManagerParams::TYPE_NAME) { dispatch[i] = Some(extract_record_entity_audio_controller_manager_params); }
        if let Some(&i) = name_to_idx.get(AudioWhitelist::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_whitelist); }
        if let Some(&i) = name_to_idx.get(AudioEnvironment::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_environment); }
        if let Some(&i) = name_to_idx.get(AudioBudgetDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_budget_definition); }
        if let Some(&i) = name_to_idx.get(AudioGameContextGlobals::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_game_context_globals); }
        if let Some(&i) = name_to_idx.get(AudioGameContextSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_game_context_setup); }
        if let Some(&i) = name_to_idx.get(SurfaceAudioPropertiesDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_surface_audio_properties_definition); }
        if let Some(&i) = name_to_idx.get(GlobalAudioSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_global_audio_settings); }
        if let Some(&i) = name_to_idx.get(AudioTagActionList::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_tag_action_list); }
        if let Some(&i) = name_to_idx.get(AudioValueOutputSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_value_output_setup); }
        if let Some(&i) = name_to_idx.get(AwardService_Config::TYPE_NAME) { dispatch[i] = Some(extract_record_award_service_config); }
        if let Some(&i) = name_to_idx.get(BuildingBlocks_Timeline::TYPE_NAME) { dispatch[i] = Some(extract_record_building_blocks_timeline); }
        if let Some(&i) = name_to_idx.get(GrabCameraControlParams::TYPE_NAME) { dispatch[i] = Some(extract_record_grab_camera_control_params); }
        if let Some(&i) = name_to_idx.get(BuildingBlocks_Canvas::TYPE_NAME) { dispatch[i] = Some(extract_record_building_blocks_canvas); }
        if let Some(&i) = name_to_idx.get(BuildingBlocks_FontStyle::TYPE_NAME) { dispatch[i] = Some(extract_record_building_blocks_font_style); }
        if let Some(&i) = name_to_idx.get(BuildingBlocks_LanguageSpecificFontReplacement::TYPE_NAME) { dispatch[i] = Some(extract_record_building_blocks_language_specific_font_replacement); }
        if let Some(&i) = name_to_idx.get(BuildingBlocks_Style::TYPE_NAME) { dispatch[i] = Some(extract_record_building_blocks_style); }
        if let Some(&i) = name_to_idx.get(BuildingBlocks_ExternalColorReference::TYPE_NAME) { dispatch[i] = Some(extract_record_building_blocks_external_color_reference); }
        if let Some(&i) = name_to_idx.get(BuildingBlocks_AspectRatioLibrary::TYPE_NAME) { dispatch[i] = Some(extract_record_building_blocks_aspect_ratio_library); }
        if let Some(&i) = name_to_idx.get(CameraLensParams::TYPE_NAME) { dispatch[i] = Some(extract_record_camera_lens_params); }
        if let Some(&i) = name_to_idx.get(Camera::TYPE_NAME) { dispatch[i] = Some(extract_record_camera); }
        if let Some(&i) = name_to_idx.get(CameraShopConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_camera_shop_config); }
        if let Some(&i) = name_to_idx.get(ActorFOVViewParams::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_fovview_params); }
        if let Some(&i) = name_to_idx.get(SuggestedFOVSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_suggested_fovsetup); }
        if let Some(&i) = name_to_idx.get(CinematicCameraControllerSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_cinematic_camera_controller_setup); }
        if let Some(&i) = name_to_idx.get(CameraFOVChangeData::TYPE_NAME) { dispatch[i] = Some(extract_record_camera_fovchange_data); }
        if let Some(&i) = name_to_idx.get(CargoManifest::TYPE_NAME) { dispatch[i] = Some(extract_record_cargo_manifest); }
        if let Some(&i) = name_to_idx.get(CarryConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_carry_config); }
        if let Some(&i) = name_to_idx.get(Character::TYPE_NAME) { dispatch[i] = Some(extract_record_character); }
        if let Some(&i) = name_to_idx.get(SCharacterGenerationParams::TYPE_NAME) { dispatch[i] = Some(extract_record_scharacter_generation_params); }
        if let Some(&i) = name_to_idx.get(CharacterSerializationSettingsPreset::TYPE_NAME) { dispatch[i] = Some(extract_record_character_serialization_settings_preset); }
        if let Some(&i) = name_to_idx.get(CharacterRandomNameParams::TYPE_NAME) { dispatch[i] = Some(extract_record_character_random_name_params); }
        if let Some(&i) = name_to_idx.get(ChatEmoteRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_chat_emote_record); }
        if let Some(&i) = name_to_idx.get(ChatCommandFastAccess::TYPE_NAME) { dispatch[i] = Some(extract_record_chat_command_fast_access); }
        if let Some(&i) = name_to_idx.get(ChatFilterOptions::TYPE_NAME) { dispatch[i] = Some(extract_record_chat_filter_options); }
        if let Some(&i) = name_to_idx.get(ChatManagerGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_chat_manager_global_params); }
        if let Some(&i) = name_to_idx.get(CockpitResponse::TYPE_NAME) { dispatch[i] = Some(extract_record_cockpit_response); }
        if let Some(&i) = name_to_idx.get(CockpitResponses::TYPE_NAME) { dispatch[i] = Some(extract_record_cockpit_responses); }
        if let Some(&i) = name_to_idx.get(CommunicationConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_communication_config); }
        if let Some(&i) = name_to_idx.get(CommunicationChannelConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_communication_channel_config); }
        if let Some(&i) = name_to_idx.get(CommunicationVariableConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_communication_variable_config); }
        if let Some(&i) = name_to_idx.get(CommunicationATLConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_communication_atlconfig); }
        if let Some(&i) = name_to_idx.get(CommunicationAutoMannequinTagsConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_communication_auto_mannequin_tags_config); }
        if let Some(&i) = name_to_idx.get(ContextualCommunicationConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_contextual_communication_config); }
        if let Some(&i) = name_to_idx.get(CommunicationName::TYPE_NAME) { dispatch[i] = Some(extract_record_communication_name); }
        if let Some(&i) = name_to_idx.get(CommunicationChannelName::TYPE_NAME) { dispatch[i] = Some(extract_record_communication_channel_name); }
        if let Some(&i) = name_to_idx.get(ActorLookAheadVehicle::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_look_ahead_vehicle); }
        if let Some(&i) = name_to_idx.get(IfcsInputDeflectionTimeParams::TYPE_NAME) { dispatch[i] = Some(extract_record_ifcs_input_deflection_time_params); }
        if let Some(&i) = name_to_idx.get(TurretInputDeflectionTimeParams::TYPE_NAME) { dispatch[i] = Some(extract_record_turret_input_deflection_time_params); }
        if let Some(&i) = name_to_idx.get(IntoxicationIFCSModifierParams::TYPE_NAME) { dispatch[i] = Some(extract_record_intoxication_ifcsmodifier_params); }
        if let Some(&i) = name_to_idx.get(IntoxicationTurretModifierParams::TYPE_NAME) { dispatch[i] = Some(extract_record_intoxication_turret_modifier_params); }
        if let Some(&i) = name_to_idx.get(IntoxicationWheeledModifierParams::TYPE_NAME) { dispatch[i] = Some(extract_record_intoxication_wheeled_modifier_params); }
        if let Some(&i) = name_to_idx.get(TargetTrackingAutoZoomDef::TYPE_NAME) { dispatch[i] = Some(extract_record_target_tracking_auto_zoom_def); }
        if let Some(&i) = name_to_idx.get(AudioAllegianceSwitches::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_allegiance_switches); }
        if let Some(&i) = name_to_idx.get(SCDynamicLightingRigGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_scdynamic_lighting_rig_global_params); }
        if let Some(&i) = name_to_idx.get(EntityAudioControllerRtpcSubscriberListDef::TYPE_NAME) { dispatch[i] = Some(extract_record_entity_audio_controller_rtpc_subscriber_list_def); }
        if let Some(&i) = name_to_idx.get(AudioEnvironmentFeedbackZoneSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_environment_feedback_zone_setup); }
        if let Some(&i) = name_to_idx.get(AudioEnvironmentFeedbackPointDef::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_environment_feedback_point_def); }
        if let Some(&i) = name_to_idx.get(AudioHitListenerDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_hit_listener_definition); }
        if let Some(&i) = name_to_idx.get(LegacyCraftingRecipeDefRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_legacy_crafting_recipe_def_record); }
        if let Some(&i) = name_to_idx.get(LegacyCraftingRecipeListRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_legacy_crafting_recipe_list_record); }
        if let Some(&i) = name_to_idx.get(WheelAudioSurfaceMap::TYPE_NAME) { dispatch[i] = Some(extract_record_wheel_audio_surface_map); }
        if let Some(&i) = name_to_idx.get(HandholdGripType::TYPE_NAME) { dispatch[i] = Some(extract_record_handhold_grip_type); }
        if let Some(&i) = name_to_idx.get(HandholdGripDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_handhold_grip_database); }
        if let Some(&i) = name_to_idx.get(SubHarvestableConfigRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_sub_harvestable_config_record); }
        if let Some(&i) = name_to_idx.get(SubHarvestableMultiConfigRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_sub_harvestable_multi_config_record); }
        if let Some(&i) = name_to_idx.get(HarvestablePreset::TYPE_NAME) { dispatch[i] = Some(extract_record_harvestable_preset); }
        if let Some(&i) = name_to_idx.get(HarvestableSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_harvestable_setup); }
        if let Some(&i) = name_to_idx.get(HarvestableClusterPreset::TYPE_NAME) { dispatch[i] = Some(extract_record_harvestable_cluster_preset); }
        if let Some(&i) = name_to_idx.get(HarvestableProviderPreset::TYPE_NAME) { dispatch[i] = Some(extract_record_harvestable_provider_preset); }
        if let Some(&i) = name_to_idx.get(OperatorModeAvailabilityParams::TYPE_NAME) { dispatch[i] = Some(extract_record_operator_mode_availability_params); }
        if let Some(&i) = name_to_idx.get(OperatorModeDefinitionParams::TYPE_NAME) { dispatch[i] = Some(extract_record_operator_mode_definition_params); }
        if let Some(&i) = name_to_idx.get(MasterModeExclusionGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_master_mode_exclusion_global_params); }
        if let Some(&i) = name_to_idx.get(LadderConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_ladder_config); }
        if let Some(&i) = name_to_idx.get(MiningGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_mining_global_params); }
        if let Some(&i) = name_to_idx.get(MiningAudioParams::TYPE_NAME) { dispatch[i] = Some(extract_record_mining_audio_params); }
        if let Some(&i) = name_to_idx.get(MineableElement::TYPE_NAME) { dispatch[i] = Some(extract_record_mineable_element); }
        if let Some(&i) = name_to_idx.get(MineableComposition::TYPE_NAME) { dispatch[i] = Some(extract_record_mineable_composition); }
        if let Some(&i) = name_to_idx.get(MiningLaserGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_mining_laser_global_params); }
        if let Some(&i) = name_to_idx.get(PlacedSurfaceEffects_Emitter::TYPE_NAME) { dispatch[i] = Some(extract_record_placed_surface_effects_emitter); }
        if let Some(&i) = name_to_idx.get(ProceduralPlanetAudioTagAndEventsDef::TYPE_NAME) { dispatch[i] = Some(extract_record_procedural_planet_audio_tag_and_events_def); }
        if let Some(&i) = name_to_idx.get(ProceduralPlanetAudioData::TYPE_NAME) { dispatch[i] = Some(extract_record_procedural_planet_audio_data); }
        if let Some(&i) = name_to_idx.get(ProceduralPlanetAudioRiverData::TYPE_NAME) { dispatch[i] = Some(extract_record_procedural_planet_audio_river_data); }
        if let Some(&i) = name_to_idx.get(PlanetOceanAudioData::TYPE_NAME) { dispatch[i] = Some(extract_record_planet_ocean_audio_data); }
        if let Some(&i) = name_to_idx.get(STargetingMethodRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_stargeting_method_record); }
        if let Some(&i) = name_to_idx.get(STargetableItemTypesRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_stargetable_item_types_record); }
        if let Some(&i) = name_to_idx.get(STargetSelectorHudParams::TYPE_NAME) { dispatch[i] = Some(extract_record_starget_selector_hud_params); }
        if let Some(&i) = name_to_idx.get(STargetSelectorGlobalTargetingParams::TYPE_NAME) { dispatch[i] = Some(extract_record_starget_selector_global_targeting_params); }
        if let Some(&i) = name_to_idx.get(CtxGraph::TYPE_NAME) { dispatch[i] = Some(extract_record_ctx_graph); }
        if let Some(&i) = name_to_idx.get(SOperatorModeLabels::TYPE_NAME) { dispatch[i] = Some(extract_record_soperator_mode_labels); }
        if let Some(&i) = name_to_idx.get(SMasterModeLabels::TYPE_NAME) { dispatch[i] = Some(extract_record_smaster_mode_labels); }
        if let Some(&i) = name_to_idx.get(ConversationStickyFilter::TYPE_NAME) { dispatch[i] = Some(extract_record_conversation_sticky_filter); }
        if let Some(&i) = name_to_idx.get(CinematicConversationSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_cinematic_conversation_settings); }
        if let Some(&i) = name_to_idx.get(Conversation::TYPE_NAME) { dispatch[i] = Some(extract_record_conversation); }
        if let Some(&i) = name_to_idx.get(ConversationBank::TYPE_NAME) { dispatch[i] = Some(extract_record_conversation_bank); }
        if let Some(&i) = name_to_idx.get(SBezierCurveRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_sbezier_curve_record); }
        if let Some(&i) = name_to_idx.get(CraftingGameplayPropertyDef::TYPE_NAME) { dispatch[i] = Some(extract_record_crafting_gameplay_property_def); }
        if let Some(&i) = name_to_idx.get(BlueprintCategoryRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_blueprint_category_record); }
        if let Some(&i) = name_to_idx.get(BlueprintCategoryDatabaseRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_blueprint_category_database_record); }
        if let Some(&i) = name_to_idx.get(CraftingBlueprintRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_crafting_blueprint_record); }
        if let Some(&i) = name_to_idx.get(CraftingQualityDistributionRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_crafting_quality_distribution_record); }
        if let Some(&i) = name_to_idx.get(CraftingQualityLocationOverrideRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_crafting_quality_location_override_record); }
        if let Some(&i) = name_to_idx.get(CraftingGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_crafting_global_params); }
        if let Some(&i) = name_to_idx.get(CrewManifest::TYPE_NAME) { dispatch[i] = Some(extract_record_crew_manifest); }
        if let Some(&i) = name_to_idx.get(DamageMacro::TYPE_NAME) { dispatch[i] = Some(extract_record_damage_macro); }
        if let Some(&i) = name_to_idx.get(DamageResistanceMacro::TYPE_NAME) { dispatch[i] = Some(extract_record_damage_resistance_macro); }
        if let Some(&i) = name_to_idx.get(DamageMapGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_damage_map_global_params); }
        if let Some(&i) = name_to_idx.get(ActorDefaultActionsConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_default_actions_config); }
        if let Some(&i) = name_to_idx.get(DefaultEntitlementRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_default_entitlement_record); }
        if let Some(&i) = name_to_idx.get(DefaultPlayerLoadoutEntitlementRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_default_player_loadout_entitlement_record); }
        if let Some(&i) = name_to_idx.get(DematerializeAnimation::TYPE_NAME) { dispatch[i] = Some(extract_record_dematerialize_animation); }
        if let Some(&i) = name_to_idx.get(DevOwner::TYPE_NAME) { dispatch[i] = Some(extract_record_dev_owner); }
        if let Some(&i) = name_to_idx.get(DialogueExternalSource::TYPE_NAME) { dispatch[i] = Some(extract_record_dialogue_external_source); }
        if let Some(&i) = name_to_idx.get(DialogueContent::TYPE_NAME) { dispatch[i] = Some(extract_record_dialogue_content); }
        if let Some(&i) = name_to_idx.get(DialogueContentBank::TYPE_NAME) { dispatch[i] = Some(extract_record_dialogue_content_bank); }
        if let Some(&i) = name_to_idx.get(DialogueContext::TYPE_NAME) { dispatch[i] = Some(extract_record_dialogue_context); }
        if let Some(&i) = name_to_idx.get(DialogueContextBank::TYPE_NAME) { dispatch[i] = Some(extract_record_dialogue_context_bank); }
        if let Some(&i) = name_to_idx.get(DialogueRealm::TYPE_NAME) { dispatch[i] = Some(extract_record_dialogue_realm); }
        if let Some(&i) = name_to_idx.get(DigitalSignageContentSet::TYPE_NAME) { dispatch[i] = Some(extract_record_digital_signage_content_set); }
        if let Some(&i) = name_to_idx.get(DirectRTT_AfterTonemappingParams::TYPE_NAME) { dispatch[i] = Some(extract_record_direct_rtt_after_tonemapping_params); }
        if let Some(&i) = name_to_idx.get(DockingSlotVisibility::TYPE_NAME) { dispatch[i] = Some(extract_record_docking_slot_visibility); }
        if let Some(&i) = name_to_idx.get(DynamicCameraEffects::TYPE_NAME) { dispatch[i] = Some(extract_record_dynamic_camera_effects); }
        if let Some(&i) = name_to_idx.get(DynamicCameraEffectsList::TYPE_NAME) { dispatch[i] = Some(extract_record_dynamic_camera_effects_list); }
        if let Some(&i) = name_to_idx.get(ConstantDOFGlobalData::TYPE_NAME) { dispatch[i] = Some(extract_record_constant_dofglobal_data); }
        if let Some(&i) = name_to_idx.get(SEAPlayerLoadoutSnapshots::TYPE_NAME) { dispatch[i] = Some(extract_record_seaplayer_loadout_snapshots); }
        if let Some(&i) = name_to_idx.get(SEAGlobalSpecialLoadout::TYPE_NAME) { dispatch[i] = Some(extract_record_seaglobal_special_loadout); }
        if let Some(&i) = name_to_idx.get(SEAGlobalEventLoadouts::TYPE_NAME) { dispatch[i] = Some(extract_record_seaglobal_event_loadouts); }
        if let Some(&i) = name_to_idx.get(EmotionList::TYPE_NAME) { dispatch[i] = Some(extract_record_emotion_list); }
        if let Some(&i) = name_to_idx.get(EntitlementAccountItemGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_entitlement_account_item_global_params); }
        if let Some(&i) = name_to_idx.get(EntitlementNonInventoryStorableItemGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_entitlement_non_inventory_storable_item_global_params); }
        if let Some(&i) = name_to_idx.get(EntityClassDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_entity_class_definition); }
        if let Some(&i) = name_to_idx.get(CommodityType::TYPE_NAME) { dispatch[i] = Some(extract_record_commodity_type); }
        if let Some(&i) = name_to_idx.get(CommoditySubtype::TYPE_NAME) { dispatch[i] = Some(extract_record_commodity_subtype); }
        if let Some(&i) = name_to_idx.get(CommodityTypeDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_commodity_type_database); }
        if let Some(&i) = name_to_idx.get(CommodityDamageConfiguration::TYPE_NAME) { dispatch[i] = Some(extract_record_commodity_damage_configuration); }
        if let Some(&i) = name_to_idx.get(SEntrancesDef::TYPE_NAME) { dispatch[i] = Some(extract_record_sentrances_def); }
        if let Some(&i) = name_to_idx.get(SCarryableIKInteractionList::TYPE_NAME) { dispatch[i] = Some(extract_record_scarryable_ikinteraction_list); }
        if let Some(&i) = name_to_idx.get(CarryableInteractionsMetadataConfigDef::TYPE_NAME) { dispatch[i] = Some(extract_record_carryable_interactions_metadata_config_def); }
        if let Some(&i) = name_to_idx.get(EntityDefaultLoadoutParams::TYPE_NAME) { dispatch[i] = Some(extract_record_entity_default_loadout_params); }
        if let Some(&i) = name_to_idx.get(SLoadoutAssortment::TYPE_NAME) { dispatch[i] = Some(extract_record_sloadout_assortment); }
        if let Some(&i) = name_to_idx.get(SIFCSModifiersLegacy::TYPE_NAME) { dispatch[i] = Some(extract_record_sifcsmodifiers_legacy); }
        if let Some(&i) = name_to_idx.get(ESPParams::TYPE_NAME) { dispatch[i] = Some(extract_record_espparams); }
        if let Some(&i) = name_to_idx.get(SIFCSEsp::TYPE_NAME) { dispatch[i] = Some(extract_record_sifcsesp); }
        if let Some(&i) = name_to_idx.get(SIFCSGameModeParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sifcsgame_mode_params); }
        if let Some(&i) = name_to_idx.get(SVibrationDef::TYPE_NAME) { dispatch[i] = Some(extract_record_svibration_def); }
        if let Some(&i) = name_to_idx.get(SVibrationVehicleDef::TYPE_NAME) { dispatch[i] = Some(extract_record_svibration_vehicle_def); }
        if let Some(&i) = name_to_idx.get(VibrationAudioPointDef::TYPE_NAME) { dispatch[i] = Some(extract_record_vibration_audio_point_def); }
        if let Some(&i) = name_to_idx.get(GlobalGasCloudVDBParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_gas_cloud_vdbparams); }
        if let Some(&i) = name_to_idx.get(SEntityDensityClass::TYPE_NAME) { dispatch[i] = Some(extract_record_sentity_density_class); }
        if let Some(&i) = name_to_idx.get(SEntityDensityClassOverridesRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_sentity_density_class_overrides_record); }
        if let Some(&i) = name_to_idx.get(SEntityTraversingNodeId::TYPE_NAME) { dispatch[i] = Some(extract_record_sentity_traversing_node_id); }
        if let Some(&i) = name_to_idx.get(Faction::TYPE_NAME) { dispatch[i] = Some(extract_record_faction); }
        if let Some(&i) = name_to_idx.get(FactionPalettes::TYPE_NAME) { dispatch[i] = Some(extract_record_faction_palettes); }
        if let Some(&i) = name_to_idx.get(FactionPalette::TYPE_NAME) { dispatch[i] = Some(extract_record_faction_palette); }
        if let Some(&i) = name_to_idx.get(FactionReputation::TYPE_NAME) { dispatch[i] = Some(extract_record_faction_reputation); }
        if let Some(&i) = name_to_idx.get(Faction_LEGACY::TYPE_NAME) { dispatch[i] = Some(extract_record_faction_legacy); }
        if let Some(&i) = name_to_idx.get(FidgetConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_fidget_config); }
        if let Some(&i) = name_to_idx.get(FireHazardGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_fire_hazard_global_params); }
        if let Some(&i) = name_to_idx.get(FlashObjectBindingGroup::TYPE_NAME) { dispatch[i] = Some(extract_record_flash_object_binding_group); }
        if let Some(&i) = name_to_idx.get(FoleyDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_foley_definition); }
        if let Some(&i) = name_to_idx.get(FoleyBone::TYPE_NAME) { dispatch[i] = Some(extract_record_foley_bone); }
        if let Some(&i) = name_to_idx.get(FoleyAxis::TYPE_NAME) { dispatch[i] = Some(extract_record_foley_axis); }
        if let Some(&i) = name_to_idx.get(FoleyFootstepDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_foley_footstep_definition); }
        if let Some(&i) = name_to_idx.get(Formation::TYPE_NAME) { dispatch[i] = Some(extract_record_formation); }
        if let Some(&i) = name_to_idx.get(FriendManagerGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_friend_manager_global_params); }
        if let Some(&i) = name_to_idx.get(FrontendOverrideParams::TYPE_NAME) { dispatch[i] = Some(extract_record_frontend_override_params); }
        if let Some(&i) = name_to_idx.get(AudioSignalList::TYPE_NAME) { dispatch[i] = Some(extract_record_audio_signal_list); }
        if let Some(&i) = name_to_idx.get(TagToAudioRtpcList::TYPE_NAME) { dispatch[i] = Some(extract_record_tag_to_audio_rtpc_list); }
        if let Some(&i) = name_to_idx.get(ForceFeedback::TYPE_NAME) { dispatch[i] = Some(extract_record_force_feedback); }
        if let Some(&i) = name_to_idx.get(GameModule::TYPE_NAME) { dispatch[i] = Some(extract_record_game_module); }
        if let Some(&i) = name_to_idx.get(ChatChannelFilterRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_chat_channel_filter_record); }
        if let Some(&i) = name_to_idx.get(VoiceChannelSettingsRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_voice_channel_settings_record); }
        if let Some(&i) = name_to_idx.get(GameMode::TYPE_NAME) { dispatch[i] = Some(extract_record_game_mode); }
        if let Some(&i) = name_to_idx.get(GameDifficultyModifiers::TYPE_NAME) { dispatch[i] = Some(extract_record_game_difficulty_modifiers); }
        if let Some(&i) = name_to_idx.get(SViewDistanceRatioParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sview_distance_ratio_params); }
        if let Some(&i) = name_to_idx.get(SGeometryViewDistanceRatioCategories::TYPE_NAME) { dispatch[i] = Some(extract_record_sgeometry_view_distance_ratio_categories); }
        if let Some(&i) = name_to_idx.get(SGlobalChargeDrainBeamParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sglobal_charge_drain_beam_params); }
        if let Some(&i) = name_to_idx.get(SGlobalCrosshairParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sglobal_crosshair_params); }
        if let Some(&i) = name_to_idx.get(SGlobalCuttableShapeParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sglobal_cuttable_shape_params); }
        if let Some(&i) = name_to_idx.get(SCuttableShapeDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_scuttable_shape_definition); }
        if let Some(&i) = name_to_idx.get(SGlobalElectronParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sglobal_electron_params); }
        if let Some(&i) = name_to_idx.get(SGlobalHealingBeamParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sglobal_healing_beam_params); }
        if let Some(&i) = name_to_idx.get(SGlobalSalvageRepairBeamParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sglobal_salvage_repair_beam_params); }
        if let Some(&i) = name_to_idx.get(GlobalShopCommodityParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_shop_commodity_params); }
        if let Some(&i) = name_to_idx.get(GlobalShopTerminalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_shop_terminal_params); }
        if let Some(&i) = name_to_idx.get(GlobalShopSellingParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_shop_selling_params); }
        if let Some(&i) = name_to_idx.get(GlobalShopBuyingParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_shop_buying_params); }
        if let Some(&i) = name_to_idx.get(SGlobalTractorBeamParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sglobal_tractor_beam_params); }
        if let Some(&i) = name_to_idx.get(Grip::TYPE_NAME) { dispatch[i] = Some(extract_record_grip); }
        if let Some(&i) = name_to_idx.get(HardwareMouseParams::TYPE_NAME) { dispatch[i] = Some(extract_record_hardware_mouse_params); }
        if let Some(&i) = name_to_idx.get(InitialDamageOverride::TYPE_NAME) { dispatch[i] = Some(extract_record_initial_damage_override); }
        if let Some(&i) = name_to_idx.get(BodyPart::TYPE_NAME) { dispatch[i] = Some(extract_record_body_part); }
        if let Some(&i) = name_to_idx.get(BodyMapping::TYPE_NAME) { dispatch[i] = Some(extract_record_body_mapping); }
        if let Some(&i) = name_to_idx.get(BodyHealthConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_body_health_config); }
        if let Some(&i) = name_to_idx.get(HealthTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_health_template); }
        if let Some(&i) = name_to_idx.get(HintUIData::TYPE_NAME) { dispatch[i] = Some(extract_record_hint_uidata); }
        if let Some(&i) = name_to_idx.get(HintTriggerData::TYPE_NAME) { dispatch[i] = Some(extract_record_hint_trigger_data); }
        if let Some(&i) = name_to_idx.get(SGlobalHitBehaviorParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sglobal_hit_behavior_params); }
        if let Some(&i) = name_to_idx.get(HologramParams::TYPE_NAME) { dispatch[i] = Some(extract_record_hologram_params); }
        if let Some(&i) = name_to_idx.get(InnerThought_Anim::TYPE_NAME) { dispatch[i] = Some(extract_record_inner_thought_anim); }
        if let Some(&i) = name_to_idx.get(InnerThought_ColorParams::TYPE_NAME) { dispatch[i] = Some(extract_record_inner_thought_color_params); }
        if let Some(&i) = name_to_idx.get(GeomFont_Config::TYPE_NAME) { dispatch[i] = Some(extract_record_geom_font_config); }
        if let Some(&i) = name_to_idx.get(InnerThought_Params::TYPE_NAME) { dispatch[i] = Some(extract_record_inner_thought_params); }
        if let Some(&i) = name_to_idx.get(InnerThought_ConversationSystemConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_inner_thought_conversation_system_config); }
        if let Some(&i) = name_to_idx.get(InnerThought_InteractionSystemConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_inner_thought_interaction_system_config); }
        if let Some(&i) = name_to_idx.get(InnerThought_LegacyUseSystemConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_inner_thought_legacy_use_system_config); }
        if let Some(&i) = name_to_idx.get(InputPromptConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_input_prompt_config); }
        if let Some(&i) = name_to_idx.get(QTERequestConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_qterequest_config); }
        if let Some(&i) = name_to_idx.get(InstancedInteriorLocationParams::TYPE_NAME) { dispatch[i] = Some(extract_record_instanced_interior_location_params); }
        if let Some(&i) = name_to_idx.get(InstancedInteriorLocationMap::TYPE_NAME) { dispatch[i] = Some(extract_record_instanced_interior_location_map); }
        if let Some(&i) = name_to_idx.get(ShipInsurancePolicyRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_ship_insurance_policy_record); }
        if let Some(&i) = name_to_idx.get(SkinInteractableTemplates::TYPE_NAME) { dispatch[i] = Some(extract_record_skin_interactable_templates); }
        if let Some(&i) = name_to_idx.get(InteractionConditionPreset::TYPE_NAME) { dispatch[i] = Some(extract_record_interaction_condition_preset); }
        if let Some(&i) = name_to_idx.get(InteractionPointTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_interaction_point_template); }
        if let Some(&i) = name_to_idx.get(InventoryContainerManager::TYPE_NAME) { dispatch[i] = Some(extract_record_inventory_container_manager); }
        if let Some(&i) = name_to_idx.get(InventoryContainer::TYPE_NAME) { dispatch[i] = Some(extract_record_inventory_container); }
        if let Some(&i) = name_to_idx.get(LandingZoneInventory::TYPE_NAME) { dispatch[i] = Some(extract_record_landing_zone_inventory); }
        if let Some(&i) = name_to_idx.get(Item::TYPE_NAME) { dispatch[i] = Some(extract_record_item); }
        if let Some(&i) = name_to_idx.get(ItemKioskBrand::TYPE_NAME) { dispatch[i] = Some(extract_record_item_kiosk_brand); }
        if let Some(&i) = name_to_idx.get(ItemPortTagsDictionary::TYPE_NAME) { dispatch[i] = Some(extract_record_item_port_tags_dictionary); }
        if let Some(&i) = name_to_idx.get(ItemResourceNetworkGlobal::TYPE_NAME) { dispatch[i] = Some(extract_record_item_resource_network_global); }
        if let Some(&i) = name_to_idx.get(JournalEntryType::TYPE_NAME) { dispatch[i] = Some(extract_record_journal_entry_type); }
        if let Some(&i) = name_to_idx.get(JournalEntry::TYPE_NAME) { dispatch[i] = Some(extract_record_journal_entry); }
        if let Some(&i) = name_to_idx.get(JumpFallLandConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_jump_fall_land_config); }
        if let Some(&i) = name_to_idx.get(GlobalJumpPointParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_jump_point_params); }
        if let Some(&i) = name_to_idx.get(GlobalJumpTunnelHostParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_jump_tunnel_host_params); }
        if let Some(&i) = name_to_idx.get(JumpDriveFlightParams::TYPE_NAME) { dispatch[i] = Some(extract_record_jump_drive_flight_params); }
        if let Some(&i) = name_to_idx.get(JumpTunnelForcesParams::TYPE_NAME) { dispatch[i] = Some(extract_record_jump_tunnel_forces_params); }
        if let Some(&i) = name_to_idx.get(GlobalJumpDriveParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_jump_drive_params); }
        if let Some(&i) = name_to_idx.get(JumpTravelCameraParams::TYPE_NAME) { dispatch[i] = Some(extract_record_jump_travel_camera_params); }
        if let Some(&i) = name_to_idx.get(AreaServices::TYPE_NAME) { dispatch[i] = Some(extract_record_area_services); }
        if let Some(&i) = name_to_idx.get(GlobalCargoLoadingParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_cargo_loading_params); }
        if let Some(&i) = name_to_idx.get(LandingPadSize::TYPE_NAME) { dispatch[i] = Some(extract_record_landing_pad_size); }
        if let Some(&i) = name_to_idx.get(InfractionDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_infraction_definition); }
        if let Some(&i) = name_to_idx.get(Jurisdiction::TYPE_NAME) { dispatch[i] = Some(extract_record_jurisdiction); }
        if let Some(&i) = name_to_idx.get(VehicleSerialNumberCharacterType::TYPE_NAME) { dispatch[i] = Some(extract_record_vehicle_serial_number_character_type); }
        if let Some(&i) = name_to_idx.get(VehicleSerialNumberFormat::TYPE_NAME) { dispatch[i] = Some(extract_record_vehicle_serial_number_format); }
        if let Some(&i) = name_to_idx.get(Level::TYPE_NAME) { dispatch[i] = Some(extract_record_level); }
        if let Some(&i) = name_to_idx.get(VisorHUD_Config::TYPE_NAME) { dispatch[i] = Some(extract_record_visor_hud_config); }
        if let Some(&i) = name_to_idx.get(Visor_ControlHintsConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_visor_control_hints_config); }
        if let Some(&i) = name_to_idx.get(ControlHints_Preset::TYPE_NAME) { dispatch[i] = Some(extract_record_control_hints_preset); }
        if let Some(&i) = name_to_idx.get(LocalPlayerSpeedThrottleComponent::TYPE_NAME) { dispatch[i] = Some(extract_record_local_player_speed_throttle_component); }
        if let Some(&i) = name_to_idx.get(LongTermPersistenceGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_long_term_persistence_global_params); }
        if let Some(&i) = name_to_idx.get(LootArchetype::TYPE_NAME) { dispatch[i] = Some(extract_record_loot_archetype); }
        if let Some(&i) = name_to_idx.get(LootTable::TYPE_NAME) { dispatch[i] = Some(extract_record_loot_table); }
        if let Some(&i) = name_to_idx.get(PoolFilterRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_pool_filter_record); }
        if let Some(&i) = name_to_idx.get(LootTableV3Record::TYPE_NAME) { dispatch[i] = Some(extract_record_loot_table_v3_record); }
        if let Some(&i) = name_to_idx.get(LootArchetypeV3Record::TYPE_NAME) { dispatch[i] = Some(extract_record_loot_archetype_v3_record); }
        if let Some(&i) = name_to_idx.get(LootV3SecondaryChoicesSingleLayerRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_loot_v3_secondary_choices_single_layer_record); }
        if let Some(&i) = name_to_idx.get(LootV3SecondaryChoicesMultiLayerRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_loot_v3_secondary_choices_multi_layer_record); }
        if let Some(&i) = name_to_idx.get(LootGenerationGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_loot_generation_global_params); }
        if let Some(&i) = name_to_idx.get(InteriorMapSectionDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_interior_map_section_definition); }
        if let Some(&i) = name_to_idx.get(MarkerTrackingViewModeParameters::TYPE_NAME) { dispatch[i] = Some(extract_record_marker_tracking_view_mode_parameters); }
        if let Some(&i) = name_to_idx.get(MarkerTrackingCommonMapParameters::TYPE_NAME) { dispatch[i] = Some(extract_record_marker_tracking_common_map_parameters); }
        if let Some(&i) = name_to_idx.get(MarkerDeclutteringCullingOrder::TYPE_NAME) { dispatch[i] = Some(extract_record_marker_decluttering_culling_order); }
        if let Some(&i) = name_to_idx.get(GlobalMarkerConfigs::TYPE_NAME) { dispatch[i] = Some(extract_record_global_marker_configs); }
        if let Some(&i) = name_to_idx.get(Marker_Configuration::TYPE_NAME) { dispatch[i] = Some(extract_record_marker_configuration); }
        if let Some(&i) = name_to_idx.get(MedicalItemTierConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_medical_item_tier_config); }
        if let Some(&i) = name_to_idx.get(MegaMap::TYPE_NAME) { dispatch[i] = Some(extract_record_mega_map); }
        if let Some(&i) = name_to_idx.get(MeleeCombatConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_melee_combat_config); }
        if let Some(&i) = name_to_idx.get(AIMeleeCombatConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_aimelee_combat_config); }
        if let Some(&i) = name_to_idx.get(MissionLocationTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_location_template); }
        if let Some(&i) = name_to_idx.get(MissionItem::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_item); }
        if let Some(&i) = name_to_idx.get(MissionOrganization::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_organization); }
        if let Some(&i) = name_to_idx.get(MissionFailConditionsList::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_fail_conditions_list); }
        if let Some(&i) = name_to_idx.get(ContractGenerator::TYPE_NAME) { dispatch[i] = Some(extract_record_contract_generator); }
        if let Some(&i) = name_to_idx.get(ContractDifficultyProfile::TYPE_NAME) { dispatch[i] = Some(extract_record_contract_difficulty_profile); }
        if let Some(&i) = name_to_idx.get(ItemAwardWeightingsRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_item_award_weightings_record); }
        if let Some(&i) = name_to_idx.get(BlueprintPoolRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_blueprint_pool_record); }
        if let Some(&i) = name_to_idx.get(ContractTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_contract_template); }
        if let Some(&i) = name_to_idx.get(EntityClusterId::TYPE_NAME) { dispatch[i] = Some(extract_record_entity_cluster_id); }
        if let Some(&i) = name_to_idx.get(EntityClusterMember::TYPE_NAME) { dispatch[i] = Some(extract_record_entity_cluster_member); }
        if let Some(&i) = name_to_idx.get(Hauling_EntityClasses::TYPE_NAME) { dispatch[i] = Some(extract_record_hauling_entity_classes); }
        if let Some(&i) = name_to_idx.get(LocationResourceSlot::TYPE_NAME) { dispatch[i] = Some(extract_record_location_resource_slot); }
        if let Some(&i) = name_to_idx.get(LocationEntityDeclaration::TYPE_NAME) { dispatch[i] = Some(extract_record_location_entity_declaration); }
        if let Some(&i) = name_to_idx.get(ModuleDeclaration::TYPE_NAME) { dispatch[i] = Some(extract_record_module_declaration); }
        if let Some(&i) = name_to_idx.get(MissionModuleHierarchy::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_module_hierarchy); }
        if let Some(&i) = name_to_idx.get(MissionScenario::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_scenario); }
        if let Some(&i) = name_to_idx.get(BeaconsContracts::TYPE_NAME) { dispatch[i] = Some(extract_record_beacons_contracts); }
        if let Some(&i) = name_to_idx.get(MissionType::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_type); }
        if let Some(&i) = name_to_idx.get(MissionLocality::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_locality); }
        if let Some(&i) = name_to_idx.get(MissionBrokerEntry::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_broker_entry); }
        if let Some(&i) = name_to_idx.get(GlobalMissionSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_global_mission_settings); }
        if let Some(&i) = name_to_idx.get(MissionGiver::TYPE_NAME) { dispatch[i] = Some(extract_record_mission_giver); }
        if let Some(&i) = name_to_idx.get(ARModeSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_armode_settings); }
        if let Some(&i) = name_to_idx.get(mobiGlasApp::TYPE_NAME) { dispatch[i] = Some(extract_record_mobi_glas_app); }
        if let Some(&i) = name_to_idx.get(MusicLogicConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_music_logic_config); }
        if let Some(&i) = name_to_idx.get(MusicLogicParameter::TYPE_NAME) { dispatch[i] = Some(extract_record_music_logic_parameter); }
        if let Some(&i) = name_to_idx.get(MusicLogicEvent::TYPE_NAME) { dispatch[i] = Some(extract_record_music_logic_event); }
        if let Some(&i) = name_to_idx.get(MusicLogicEventList::TYPE_NAME) { dispatch[i] = Some(extract_record_music_logic_event_list); }
        if let Some(&i) = name_to_idx.get(MusicLogicSwitchValue::TYPE_NAME) { dispatch[i] = Some(extract_record_music_logic_switch_value); }
        if let Some(&i) = name_to_idx.get(MusicLogicSuite::TYPE_NAME) { dispatch[i] = Some(extract_record_music_logic_suite); }
        if let Some(&i) = name_to_idx.get(NotificationDef::TYPE_NAME) { dispatch[i] = Some(extract_record_notification_def); }
        if let Some(&i) = name_to_idx.get(CommsNotificationStage::TYPE_NAME) { dispatch[i] = Some(extract_record_comms_notification_stage); }
        if let Some(&i) = name_to_idx.get(CommsNotification::TYPE_NAME) { dispatch[i] = Some(extract_record_comms_notification); }
        if let Some(&i) = name_to_idx.get(CommsNotificationsGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_comms_notifications_global_params); }
        if let Some(&i) = name_to_idx.get(GameNotificationDockItemParams::TYPE_NAME) { dispatch[i] = Some(extract_record_game_notification_dock_item_params); }
        if let Some(&i) = name_to_idx.get(SandboxTriggerRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_sandbox_trigger_record); }
        if let Some(&i) = name_to_idx.get(GPUParticleAudio::TYPE_NAME) { dispatch[i] = Some(extract_record_gpuparticle_audio); }
        if let Some(&i) = name_to_idx.get(GPUParticleAudioList::TYPE_NAME) { dispatch[i] = Some(extract_record_gpuparticle_audio_list); }
        if let Some(&i) = name_to_idx.get(SPerkReputationListParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sperk_reputation_list_params); }
        if let Some(&i) = name_to_idx.get(PlanetEffectLOD::TYPE_NAME) { dispatch[i] = Some(extract_record_planet_effect_lod); }
        if let Some(&i) = name_to_idx.get(PlayerAnimatedInteractionTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_player_animated_interaction_template); }
        if let Some(&i) = name_to_idx.get(PlayerAnimatedInteractionConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_player_animated_interaction_config); }
        if let Some(&i) = name_to_idx.get(PlayerChoice_SignalConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_player_choice_signal_config); }
        if let Some(&i) = name_to_idx.get(PlayerChoice_Library::TYPE_NAME) { dispatch[i] = Some(extract_record_player_choice_library); }
        if let Some(&i) = name_to_idx.get(PlayerChoice_IMConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_player_choice_imconfig); }
        if let Some(&i) = name_to_idx.get(PlayerChoiceMenuItem::TYPE_NAME) { dispatch[i] = Some(extract_record_player_choice_menu_item); }
        if let Some(&i) = name_to_idx.get(PlayerChoiceMenuItems::TYPE_NAME) { dispatch[i] = Some(extract_record_player_choice_menu_items); }
        if let Some(&i) = name_to_idx.get(PlayerChoiceMenu::TYPE_NAME) { dispatch[i] = Some(extract_record_player_choice_menu); }
        if let Some(&i) = name_to_idx.get(PlayerChoiceMenuType::TYPE_NAME) { dispatch[i] = Some(extract_record_player_choice_menu_type); }
        if let Some(&i) = name_to_idx.get(LedgeGrabbingParams::TYPE_NAME) { dispatch[i] = Some(extract_record_ledge_grabbing_params); }
        if let Some(&i) = name_to_idx.get(ActorTargetedParams::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_targeted_params); }
        if let Some(&i) = name_to_idx.get(ActorStanceConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_stance_config); }
        if let Some(&i) = name_to_idx.get(JumpFallLandParams::TYPE_NAME) { dispatch[i] = Some(extract_record_jump_fall_land_params); }
        if let Some(&i) = name_to_idx.get(PlayerDockContextComponentGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_player_dock_context_component_global_params); }
        if let Some(&i) = name_to_idx.get(PlayerGroupManagerGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_player_group_manager_global_params); }
        if let Some(&i) = name_to_idx.get(PlayerLimitationsProfile::TYPE_NAME) { dispatch[i] = Some(extract_record_player_limitations_profile); }
        if let Some(&i) = name_to_idx.get(PlayerNotificationBannerManagerGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_player_notification_banner_manager_global_params); }
        if let Some(&i) = name_to_idx.get(PlayerShipRespawn::TYPE_NAME) { dispatch[i] = Some(extract_record_player_ship_respawn); }
        if let Some(&i) = name_to_idx.get(PlayerTradeGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_player_trade_global_params); }
        if let Some(&i) = name_to_idx.get(PointOfInterestList::TYPE_NAME) { dispatch[i] = Some(extract_record_point_of_interest_list); }
        if let Some(&i) = name_to_idx.get(PostureDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_posture_database); }
        if let Some(&i) = name_to_idx.get(ProcBreathingCurve::TYPE_NAME) { dispatch[i] = Some(extract_record_proc_breathing_curve); }
        if let Some(&i) = name_to_idx.get(ProcBreathingCurveDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_proc_breathing_curve_database); }
        if let Some(&i) = name_to_idx.get(ProcBreathingSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_proc_breathing_setup); }
        if let Some(&i) = name_to_idx.get(ProceduralAimRigRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_procedural_aim_rig_record); }
        if let Some(&i) = name_to_idx.get(ProceduralAnimation::TYPE_NAME) { dispatch[i] = Some(extract_record_procedural_animation); }
        if let Some(&i) = name_to_idx.get(PlanetDayNightTemperatureTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_planet_day_night_temperature_template); }
        if let Some(&i) = name_to_idx.get(ProceduralLandingSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_procedural_landing_setup); }
        if let Some(&i) = name_to_idx.get(ProceduralLayoutGraph::TYPE_NAME) { dispatch[i] = Some(extract_record_procedural_layout_graph); }
        if let Some(&i) = name_to_idx.get(SProjectedHudParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sprojected_hud_params); }
        if let Some(&i) = name_to_idx.get(SVehicleHudParams::TYPE_NAME) { dispatch[i] = Some(extract_record_svehicle_hud_params); }
        if let Some(&i) = name_to_idx.get(QuantumDriveGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_quantum_drive_global_params); }
        if let Some(&i) = name_to_idx.get(SQuantumDriveEffectTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_squantum_drive_effect_template); }
        if let Some(&i) = name_to_idx.get(QuantumDriveEffectSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_quantum_drive_effect_settings); }
        if let Some(&i) = name_to_idx.get(SQuantumCameraEffectsDef::TYPE_NAME) { dispatch[i] = Some(extract_record_squantum_camera_effects_def); }
        if let Some(&i) = name_to_idx.get(RaSTaRLibraryElement::TYPE_NAME) { dispatch[i] = Some(extract_record_ra_sta_rlibrary_element); }
        if let Some(&i) = name_to_idx.get(RaSTaRLibrary::TYPE_NAME) { dispatch[i] = Some(extract_record_ra_sta_rlibrary); }
        if let Some(&i) = name_to_idx.get(WorldDisplayRadar::TYPE_NAME) { dispatch[i] = Some(extract_record_world_display_radar); }
        if let Some(&i) = name_to_idx.get(RadarDisplay_Config::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_display_config); }
        if let Some(&i) = name_to_idx.get(RadarSystemGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_system_global_params); }
        if let Some(&i) = name_to_idx.get(RadarSystemSharedParams::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_system_shared_params); }
        if let Some(&i) = name_to_idx.get(ScanInformationDef::TYPE_NAME) { dispatch[i] = Some(extract_record_scan_information_def); }
        if let Some(&i) = name_to_idx.get(ScanInformationTable::TYPE_NAME) { dispatch[i] = Some(extract_record_scan_information_table); }
        if let Some(&i) = name_to_idx.get(ScanCustomDataDef::TYPE_NAME) { dispatch[i] = Some(extract_record_scan_custom_data_def); }
        if let Some(&i) = name_to_idx.get(ScanDisplayInstanceParams::TYPE_NAME) { dispatch[i] = Some(extract_record_scan_display_instance_params); }
        if let Some(&i) = name_to_idx.get(ScanDisplayLayoutParams::TYPE_NAME) { dispatch[i] = Some(extract_record_scan_display_layout_params); }
        if let Some(&i) = name_to_idx.get(RadarSignatureCategoryDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_signature_category_definition); }
        if let Some(&i) = name_to_idx.get(RadarSignatureCategoryEntry::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_signature_category_entry); }
        if let Some(&i) = name_to_idx.get(RadarContactTypeDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_contact_type_definition); }
        if let Some(&i) = name_to_idx.get(RadarContactTypeEntry::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_contact_type_entry); }
        if let Some(&i) = name_to_idx.get(RadarContactGroupDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_contact_group_definition); }
        if let Some(&i) = name_to_idx.get(RadarContactGroupEntry::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_contact_group_entry); }
        if let Some(&i) = name_to_idx.get(RadarDeltaSignatureDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_delta_signature_definition); }
        if let Some(&i) = name_to_idx.get(RadarDeltaSignatureEntry::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_delta_signature_entry); }
        if let Some(&i) = name_to_idx.get(RefiningProcess::TYPE_NAME) { dispatch[i] = Some(extract_record_refining_process); }
        if let Some(&i) = name_to_idx.get(RefineryNotificationConfiguration::TYPE_NAME) { dispatch[i] = Some(extract_record_refinery_notification_configuration); }
        if let Some(&i) = name_to_idx.get(RentalNotificationParams::TYPE_NAME) { dispatch[i] = Some(extract_record_rental_notification_params); }
        if let Some(&i) = name_to_idx.get(SReputationStandingParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sreputation_standing_params); }
        if let Some(&i) = name_to_idx.get(SReputationContextUI::TYPE_NAME) { dispatch[i] = Some(extract_record_sreputation_context_ui); }
        if let Some(&i) = name_to_idx.get(SReputationGlobalContextBBParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sreputation_global_context_bbparams); }
        if let Some(&i) = name_to_idx.get(SReputationStateParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sreputation_state_params); }
        if let Some(&i) = name_to_idx.get(SReputationStateMissionResultModifierParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sreputation_state_mission_result_modifier_params); }
        if let Some(&i) = name_to_idx.get(SReputationScopeParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sreputation_scope_params); }
        if let Some(&i) = name_to_idx.get(SReputationRewardAmount::TYPE_NAME) { dispatch[i] = Some(extract_record_sreputation_reward_amount); }
        if let Some(&i) = name_to_idx.get(SReputationMissionRewardBonusParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sreputation_mission_reward_bonus_params); }
        if let Some(&i) = name_to_idx.get(SReputationJournalEntryHandlerParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sreputation_journal_entry_handler_params); }
        if let Some(&i) = name_to_idx.get(ReputationValueSetting::TYPE_NAME) { dispatch[i] = Some(extract_record_reputation_value_setting); }
        if let Some(&i) = name_to_idx.get(ReputationValueSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_reputation_value_settings); }
        if let Some(&i) = name_to_idx.get(ResourceType::TYPE_NAME) { dispatch[i] = Some(extract_record_resource_type); }
        if let Some(&i) = name_to_idx.get(ResourceTypeGroup::TYPE_NAME) { dispatch[i] = Some(extract_record_resource_type_group); }
        if let Some(&i) = name_to_idx.get(ResourceTypeDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_resource_type_database); }
        if let Some(&i) = name_to_idx.get(ActorRestrainConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_restrain_config); }
        if let Some(&i) = name_to_idx.get(GasParams::TYPE_NAME) { dispatch[i] = Some(extract_record_gas_params); }
        if let Some(&i) = name_to_idx.get(AtmosphericCompositionTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_atmospheric_composition_template); }
        if let Some(&i) = name_to_idx.get(GlobalGasParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_gas_params); }
        if let Some(&i) = name_to_idx.get(GlobalRoomStateParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_room_state_params); }
        if let Some(&i) = name_to_idx.get(AsteroidStateTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_asteroid_state_template); }
        if let Some(&i) = name_to_idx.get(AsteroidBehavior::TYPE_NAME) { dispatch[i] = Some(extract_record_asteroid_behavior); }
        if let Some(&i) = name_to_idx.get(AtmosphereStateTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_atmosphere_state_template); }
        if let Some(&i) = name_to_idx.get(AtmosphereStatePressureTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_atmosphere_state_pressure_template); }
        if let Some(&i) = name_to_idx.get(AtmosphereStateTemperatureTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_atmosphere_state_temperature_template); }
        if let Some(&i) = name_to_idx.get(AtmosphereStateHumidityTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_atmosphere_state_humidity_template); }
        if let Some(&i) = name_to_idx.get(AtmosphereBehavior::TYPE_NAME) { dispatch[i] = Some(extract_record_atmosphere_behavior); }
        if let Some(&i) = name_to_idx.get(ElectricalStateTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_electrical_state_template); }
        if let Some(&i) = name_to_idx.get(ElectricalBehavior::TYPE_NAME) { dispatch[i] = Some(extract_record_electrical_behavior); }
        if let Some(&i) = name_to_idx.get(RadiationStateTemplate::TYPE_NAME) { dispatch[i] = Some(extract_record_radiation_state_template); }
        if let Some(&i) = name_to_idx.get(RadiationBehavior::TYPE_NAME) { dispatch[i] = Some(extract_record_radiation_behavior); }
        if let Some(&i) = name_to_idx.get(ActorViewLimitPresetDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_view_limit_preset_database); }
        if let Some(&i) = name_to_idx.get(ActorLookLimits::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_look_limits); }
        if let Some(&i) = name_to_idx.get(ActorAimLimits::TYPE_NAME) { dispatch[i] = Some(extract_record_actor_aim_limits); }
        if let Some(&i) = name_to_idx.get(HazardAwarenessParams::TYPE_NAME) { dispatch[i] = Some(extract_record_hazard_awareness_params); }
        if let Some(&i) = name_to_idx.get(SAnalyticsEvent::TYPE_NAME) { dispatch[i] = Some(extract_record_sanalytics_event); }
        if let Some(&i) = name_to_idx.get(SAnalyticsEventDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_sanalytics_event_database); }
        if let Some(&i) = name_to_idx.get(SCItemLightAmplification::TYPE_NAME) { dispatch[i] = Some(extract_record_scitem_light_amplification); }
        if let Some(&i) = name_to_idx.get(SCItemManufacturer::TYPE_NAME) { dispatch[i] = Some(extract_record_scitem_manufacturer); }
        if let Some(&i) = name_to_idx.get(SAimableGimbalModeLabels::TYPE_NAME) { dispatch[i] = Some(extract_record_saimable_gimbal_mode_labels); }
        if let Some(&i) = name_to_idx.get(SAimableGameModeParams::TYPE_NAME) { dispatch[i] = Some(extract_record_saimable_game_mode_params); }
        if let Some(&i) = name_to_idx.get(SAimableControllerHudParams::TYPE_NAME) { dispatch[i] = Some(extract_record_saimable_controller_hud_params); }
        if let Some(&i) = name_to_idx.get(CapacitorAssignmentInputOutputDef::TYPE_NAME) { dispatch[i] = Some(extract_record_capacitor_assignment_input_output_def); }
        if let Some(&i) = name_to_idx.get(SCItemVisorDashboardConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_scitem_visor_dashboard_config); }
        if let Some(&i) = name_to_idx.get(SCItemSuitFuelParams::TYPE_NAME) { dispatch[i] = Some(extract_record_scitem_suit_fuel_params); }
        if let Some(&i) = name_to_idx.get(ExplosiveOrdnancePingGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_explosive_ordnance_ping_global_params); }
        if let Some(&i) = name_to_idx.get(MoveViewRestrictionPenalty::TYPE_NAME) { dispatch[i] = Some(extract_record_move_view_restriction_penalty); }
        if let Some(&i) = name_to_idx.get(ConsumableType::TYPE_NAME) { dispatch[i] = Some(extract_record_consumable_type); }
        if let Some(&i) = name_to_idx.get(ConsumableSubtype::TYPE_NAME) { dispatch[i] = Some(extract_record_consumable_subtype); }
        if let Some(&i) = name_to_idx.get(ConsumableTypeDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_consumable_type_database); }
        if let Some(&i) = name_to_idx.get(CommsAudioEffect::TYPE_NAME) { dispatch[i] = Some(extract_record_comms_audio_effect); }
        if let Some(&i) = name_to_idx.get(SCItemCommsComponentSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_scitem_comms_component_setup); }
        if let Some(&i) = name_to_idx.get(ShipComputerDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_ship_computer_definition); }
        if let Some(&i) = name_to_idx.get(SMovableLimits::TYPE_NAME) { dispatch[i] = Some(extract_record_smovable_limits); }
        if let Some(&i) = name_to_idx.get(JumpThrusterPackConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_jump_thruster_pack_config); }
        if let Some(&i) = name_to_idx.get(SCItemDisplayScreenPreset::TYPE_NAME) { dispatch[i] = Some(extract_record_scitem_display_screen_preset); }
        if let Some(&i) = name_to_idx.get(DockingSensitivity::TYPE_NAME) { dispatch[i] = Some(extract_record_docking_sensitivity); }
        if let Some(&i) = name_to_idx.get(WorldDisplayEnvironment::TYPE_NAME) { dispatch[i] = Some(extract_record_world_display_environment); }
        if let Some(&i) = name_to_idx.get(MiningControllerGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_mining_controller_global_params); }
        if let Some(&i) = name_to_idx.get(VehicleSalvageGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_vehicle_salvage_global_params); }
        if let Some(&i) = name_to_idx.get(SeatReticleArchetype::TYPE_NAME) { dispatch[i] = Some(extract_record_seat_reticle_archetype); }
        if let Some(&i) = name_to_idx.get(SCSeatHeadPosAdjustSetup::TYPE_NAME) { dispatch[i] = Some(extract_record_scseat_head_pos_adjust_setup); }
        if let Some(&i) = name_to_idx.get(SCItemSeatHeadTrackingPositionLimitParams::TYPE_NAME) { dispatch[i] = Some(extract_record_scitem_seat_head_tracking_position_limit_params); }
        if let Some(&i) = name_to_idx.get(ArmorMoveViewRestrictions::TYPE_NAME) { dispatch[i] = Some(extract_record_armor_move_view_restrictions); }
        if let Some(&i) = name_to_idx.get(SeatAdsDef::TYPE_NAME) { dispatch[i] = Some(extract_record_seat_ads_def); }
        if let Some(&i) = name_to_idx.get(SeatUserActorCDIKRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_seat_user_actor_cdikrecord); }
        if let Some(&i) = name_to_idx.get(SMFDModeConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_smfdmode_config); }
        if let Some(&i) = name_to_idx.get(SMFDView::TYPE_NAME) { dispatch[i] = Some(extract_record_smfdview); }
        if let Some(&i) = name_to_idx.get(SMFDViewList::TYPE_NAME) { dispatch[i] = Some(extract_record_smfdview_list); }
        if let Some(&i) = name_to_idx.get(SMFDParamsDiagnostics::TYPE_NAME) { dispatch[i] = Some(extract_record_smfdparams_diagnostics); }
        if let Some(&i) = name_to_idx.get(AnimatedHelmetParams::TYPE_NAME) { dispatch[i] = Some(extract_record_animated_helmet_params); }
        if let Some(&i) = name_to_idx.get(STurretHealthModifierDef::TYPE_NAME) { dispatch[i] = Some(extract_record_sturret_health_modifier_def); }
        if let Some(&i) = name_to_idx.get(STurretESP::TYPE_NAME) { dispatch[i] = Some(extract_record_sturret_esp); }
        if let Some(&i) = name_to_idx.get(STurretGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_sturret_global_params); }
        if let Some(&i) = name_to_idx.get(WeaponAimableAnglesDef::TYPE_NAME) { dispatch[i] = Some(extract_record_weapon_aimable_angles_def); }
        if let Some(&i) = name_to_idx.get(WeaponGimbalModeModifierDef::TYPE_NAME) { dispatch[i] = Some(extract_record_weapon_gimbal_mode_modifier_def); }
        if let Some(&i) = name_to_idx.get(WeaponMisfireDef::TYPE_NAME) { dispatch[i] = Some(extract_record_weapon_misfire_def); }
        if let Some(&i) = name_to_idx.get(WeaponARModifier::TYPE_NAME) { dispatch[i] = Some(extract_record_weapon_armodifier); }
        if let Some(&i) = name_to_idx.get(PlayerToPlayerCommsCallGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_player_to_player_comms_call_global_params); }
        if let Some(&i) = name_to_idx.get(PersonalInnerThoughtActionRulePreset::TYPE_NAME) { dispatch[i] = Some(extract_record_personal_inner_thought_action_rule_preset); }
        if let Some(&i) = name_to_idx.get(CorpseInteractionParams::TYPE_NAME) { dispatch[i] = Some(extract_record_corpse_interaction_params); }
        if let Some(&i) = name_to_idx.get(ItemRecoveryConfigurationParams::TYPE_NAME) { dispatch[i] = Some(extract_record_item_recovery_configuration_params); }
        if let Some(&i) = name_to_idx.get(PlayerChoice_PITConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_player_choice_pitconfig); }
        if let Some(&i) = name_to_idx.get(SLocalPlayerShoppingData::TYPE_NAME) { dispatch[i] = Some(extract_record_slocal_player_shopping_data); }
        if let Some(&i) = name_to_idx.get(ShopInteractionData::TYPE_NAME) { dispatch[i] = Some(extract_record_shop_interaction_data); }
        if let Some(&i) = name_to_idx.get(ShopFranchise::TYPE_NAME) { dispatch[i] = Some(extract_record_shop_franchise); }
        if let Some(&i) = name_to_idx.get(SSCSignatureSystemAudioRuleset::TYPE_NAME) { dispatch[i] = Some(extract_record_sscsignature_system_audio_ruleset); }
        if let Some(&i) = name_to_idx.get(ScenarioProgress::TYPE_NAME) { dispatch[i] = Some(extract_record_scenario_progress); }
        if let Some(&i) = name_to_idx.get(ScreenEffects_Library::TYPE_NAME) { dispatch[i] = Some(extract_record_screen_effects_library); }
        if let Some(&i) = name_to_idx.get(ScreenEffects_Effect::TYPE_NAME) { dispatch[i] = Some(extract_record_screen_effects_effect); }
        if let Some(&i) = name_to_idx.get(ScreenEffects_Debug::TYPE_NAME) { dispatch[i] = Some(extract_record_screen_effects_debug); }
        if let Some(&i) = name_to_idx.get(SecurityClearanceToken::TYPE_NAME) { dispatch[i] = Some(extract_record_security_clearance_token); }
        if let Some(&i) = name_to_idx.get(SecurityNetworkRoomSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_security_network_room_settings); }
        if let Some(&i) = name_to_idx.get(SecurityNetworkManifest::TYPE_NAME) { dispatch[i] = Some(extract_record_security_network_manifest); }
        if let Some(&i) = name_to_idx.get(ServiceBeaconParams::TYPE_NAME) { dispatch[i] = Some(extract_record_service_beacon_params); }
        if let Some(&i) = name_to_idx.get(ServiceBeaconGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_service_beacon_global_params); }
        if let Some(&i) = name_to_idx.get(ShieldTypeParams::TYPE_NAME) { dispatch[i] = Some(extract_record_shield_type_params); }
        if let Some(&i) = name_to_idx.get(ShipComputerPresetList::TYPE_NAME) { dispatch[i] = Some(extract_record_ship_computer_preset_list); }
        if let Some(&i) = name_to_idx.get(ShipComputerPreset::TYPE_NAME) { dispatch[i] = Some(extract_record_ship_computer_preset); }
        if let Some(&i) = name_to_idx.get(SSolarSystem::TYPE_NAME) { dispatch[i] = Some(extract_record_ssolar_system); }
        if let Some(&i) = name_to_idx.get(SpawnDescriptions::TYPE_NAME) { dispatch[i] = Some(extract_record_spawn_descriptions); }
        if let Some(&i) = name_to_idx.get(SpecialEventManufacturer::TYPE_NAME) { dispatch[i] = Some(extract_record_special_event_manufacturer); }
        if let Some(&i) = name_to_idx.get(SpecialEventDay::TYPE_NAME) { dispatch[i] = Some(extract_record_special_event_day); }
        if let Some(&i) = name_to_idx.get(SpecialEventDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_special_event_database); }
        if let Some(&i) = name_to_idx.get(StarMapObjectType::TYPE_NAME) { dispatch[i] = Some(extract_record_star_map_object_type); }
        if let Some(&i) = name_to_idx.get(StarMapObjectTypes::TYPE_NAME) { dispatch[i] = Some(extract_record_star_map_object_types); }
        if let Some(&i) = name_to_idx.get(StarMapAmenityTypeEntry::TYPE_NAME) { dispatch[i] = Some(extract_record_star_map_amenity_type_entry); }
        if let Some(&i) = name_to_idx.get(StarMapAmenityTypes::TYPE_NAME) { dispatch[i] = Some(extract_record_star_map_amenity_types); }
        if let Some(&i) = name_to_idx.get(StarMapObject::TYPE_NAME) { dispatch[i] = Some(extract_record_star_map_object); }
        if let Some(&i) = name_to_idx.get(StarMapMissionObject::TYPE_NAME) { dispatch[i] = Some(extract_record_star_map_mission_object); }
        if let Some(&i) = name_to_idx.get(StarMapPartyMemberObject::TYPE_NAME) { dispatch[i] = Some(extract_record_star_map_party_member_object); }
        if let Some(&i) = name_to_idx.get(StatusWidgetDisplayPreset::TYPE_NAME) { dispatch[i] = Some(extract_record_status_widget_display_preset); }
        if let Some(&i) = name_to_idx.get(TacticalQuery::TYPE_NAME) { dispatch[i] = Some(extract_record_tactical_query); }
        if let Some(&i) = name_to_idx.get(TQSOptionContentRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_tqsoption_content_record); }
        if let Some(&i) = name_to_idx.get(Tag::TYPE_NAME) { dispatch[i] = Some(extract_record_tag); }
        if let Some(&i) = name_to_idx.get(TagDatabase::TYPE_NAME) { dispatch[i] = Some(extract_record_tag_database); }
        if let Some(&i) = name_to_idx.get(TakeDownConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_take_down_config); }
        if let Some(&i) = name_to_idx.get(TintPaletteTree::TYPE_NAME) { dispatch[i] = Some(extract_record_tint_palette_tree); }
        if let Some(&i) = name_to_idx.get(MovieClipTransformationInterpolator::TYPE_NAME) { dispatch[i] = Some(extract_record_movie_clip_transformation_interpolator); }
        if let Some(&i) = name_to_idx.get(TransformationInterpolator::TYPE_NAME) { dispatch[i] = Some(extract_record_transformation_interpolator); }
        if let Some(&i) = name_to_idx.get(TransitStationAnnouncements::TYPE_NAME) { dispatch[i] = Some(extract_record_transit_station_announcements); }
        if let Some(&i) = name_to_idx.get(SScenePlayerChoiceSettings::TYPE_NAME) { dispatch[i] = Some(extract_record_sscene_player_choice_settings); }
        if let Some(&i) = name_to_idx.get(GlobalTutorialParams::TYPE_NAME) { dispatch[i] = Some(extract_record_global_tutorial_params); }
        if let Some(&i) = name_to_idx.get(ItemPreview_Config::TYPE_NAME) { dispatch[i] = Some(extract_record_item_preview_config); }
        if let Some(&i) = name_to_idx.get(SCObjectDataBankEntryMarkerConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_scobject_data_bank_entry_marker_config); }
        if let Some(&i) = name_to_idx.get(UIHoloVehicle_Config::TYPE_NAME) { dispatch[i] = Some(extract_record_uiholo_vehicle_config); }
        if let Some(&i) = name_to_idx.get(MarkerAR_ConfigDef::TYPE_NAME) { dispatch[i] = Some(extract_record_marker_ar_config_def); }
        if let Some(&i) = name_to_idx.get(UIConfig::TYPE_NAME) { dispatch[i] = Some(extract_record_uiconfig); }
        if let Some(&i) = name_to_idx.get(SimpleSpriteSheet::TYPE_NAME) { dispatch[i] = Some(extract_record_simple_sprite_sheet); }
        if let Some(&i) = name_to_idx.get(HudColors::TYPE_NAME) { dispatch[i] = Some(extract_record_hud_colors); }
        if let Some(&i) = name_to_idx.get(VideoComms::TYPE_NAME) { dispatch[i] = Some(extract_record_video_comms); }
        if let Some(&i) = name_to_idx.get(UIModeVisibilitySettings::TYPE_NAME) { dispatch[i] = Some(extract_record_uimode_visibility_settings); }
        if let Some(&i) = name_to_idx.get(LoadoutDummyComponentParams::TYPE_NAME) { dispatch[i] = Some(extract_record_loadout_dummy_component_params); }
        if let Some(&i) = name_to_idx.get(UIStateDisplay::TYPE_NAME) { dispatch[i] = Some(extract_record_uistate_display); }
        if let Some(&i) = name_to_idx.get(PopupDef::TYPE_NAME) { dispatch[i] = Some(extract_record_popup_def); }
        if let Some(&i) = name_to_idx.get(PlayerECGGraph_Config::TYPE_NAME) { dispatch[i] = Some(extract_record_player_ecggraph_config); }
        if let Some(&i) = name_to_idx.get(ItemTypeDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_item_type_definition); }
        if let Some(&i) = name_to_idx.get(FlightHUDUIView_Config::TYPE_NAME) { dispatch[i] = Some(extract_record_flight_huduiview_config); }
        if let Some(&i) = name_to_idx.get(UIElement::TYPE_NAME) { dispatch[i] = Some(extract_record_uielement); }
        if let Some(&i) = name_to_idx.get(SCItemUIView_DashboardCanvasDef::TYPE_NAME) { dispatch[i] = Some(extract_record_scitem_uiview_dashboard_canvas_def); }
        if let Some(&i) = name_to_idx.get(MissileLockReticle_Config::TYPE_NAME) { dispatch[i] = Some(extract_record_missile_lock_reticle_config); }
        if let Some(&i) = name_to_idx.get(UIElementSoundsRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_uielement_sounds_record); }
        if let Some(&i) = name_to_idx.get(UIAudioDefinition::TYPE_NAME) { dispatch[i] = Some(extract_record_uiaudio_definition); }
        if let Some(&i) = name_to_idx.get(LoadoutEditorComponentParams::TYPE_NAME) { dispatch[i] = Some(extract_record_loadout_editor_component_params); }
        if let Some(&i) = name_to_idx.get(RadarDisplay3DPreset::TYPE_NAME) { dispatch[i] = Some(extract_record_radar_display3_dpreset); }
        if let Some(&i) = name_to_idx.get(UnitTestSubRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_unit_test_sub_record); }
        if let Some(&i) = name_to_idx.get(UnitTest::TYPE_NAME) { dispatch[i] = Some(extract_record_unit_test); }
        if let Some(&i) = name_to_idx.get(UsableArchetype::TYPE_NAME) { dispatch[i] = Some(extract_record_usable_archetype); }
        if let Some(&i) = name_to_idx.get(UseChannelArchetype::TYPE_NAME) { dispatch[i] = Some(extract_record_use_channel_archetype); }
        if let Some(&i) = name_to_idx.get(UsableArchetypes::TYPE_NAME) { dispatch[i] = Some(extract_record_usable_archetypes); }
        if let Some(&i) = name_to_idx.get(SVehicleAiDamageModifiers::TYPE_NAME) { dispatch[i] = Some(extract_record_svehicle_ai_damage_modifiers); }
        if let Some(&i) = name_to_idx.get(VehicleLandingGearSystem::TYPE_NAME) { dispatch[i] = Some(extract_record_vehicle_landing_gear_system); }
        if let Some(&i) = name_to_idx.get(VehicleRole::TYPE_NAME) { dispatch[i] = Some(extract_record_vehicle_role); }
        if let Some(&i) = name_to_idx.get(VehicleCareer::TYPE_NAME) { dispatch[i] = Some(extract_record_vehicle_career); }
        if let Some(&i) = name_to_idx.get(VehicleCareerList::TYPE_NAME) { dispatch[i] = Some(extract_record_vehicle_career_list); }
        if let Some(&i) = name_to_idx.get(CommsChannelDef::TYPE_NAME) { dispatch[i] = Some(extract_record_comms_channel_def); }
        if let Some(&i) = name_to_idx.get(VideoCommsShaderParams::TYPE_NAME) { dispatch[i] = Some(extract_record_video_comms_shader_params); }
        if let Some(&i) = name_to_idx.get(VideoCommsAudioParams::TYPE_NAME) { dispatch[i] = Some(extract_record_video_comms_audio_params); }
        if let Some(&i) = name_to_idx.get(CameraTransitionInterpolationCurveRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_camera_transition_interpolation_curve_record); }
        if let Some(&i) = name_to_idx.get(CinematicFlightPointsRecord::TYPE_NAME) { dispatch[i] = Some(extract_record_cinematic_flight_points_record); }
        if let Some(&i) = name_to_idx.get(VisorLens_Layout::TYPE_NAME) { dispatch[i] = Some(extract_record_visor_lens_layout); }
        if let Some(&i) = name_to_idx.get(VisorLens_Region::TYPE_NAME) { dispatch[i] = Some(extract_record_visor_lens_region); }
        if let Some(&i) = name_to_idx.get(VoiceSingle::TYPE_NAME) { dispatch[i] = Some(extract_record_voice_single); }
        if let Some(&i) = name_to_idx.get(VoiceBundle::TYPE_NAME) { dispatch[i] = Some(extract_record_voice_bundle); }
        if let Some(&i) = name_to_idx.get(WaterEffectsGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_water_effects_global_params); }
        if let Some(&i) = name_to_idx.get(WeaponProceduralAnimation::TYPE_NAME) { dispatch[i] = Some(extract_record_weapon_procedural_animation); }
        if let Some(&i) = name_to_idx.get(WeaponProceduralClip::TYPE_NAME) { dispatch[i] = Some(extract_record_weapon_procedural_clip); }
        if let Some(&i) = name_to_idx.get(WeaponProceduralRecoilConfigDef::TYPE_NAME) { dispatch[i] = Some(extract_record_weapon_procedural_recoil_config_def); }
        if let Some(&i) = name_to_idx.get(LoadoutKit::TYPE_NAME) { dispatch[i] = Some(extract_record_loadout_kit); }
        if let Some(&i) = name_to_idx.get(WebCustomizationDebug::TYPE_NAME) { dispatch[i] = Some(extract_record_web_customization_debug); }
        if let Some(&i) = name_to_idx.get(WebCustomizationGlobalParams::TYPE_NAME) { dispatch[i] = Some(extract_record_web_customization_global_params); }
        if let Some(&i) = name_to_idx.get(MobiGlasAppData::TYPE_NAME) { dispatch[i] = Some(extract_record_mobi_glas_app_data); }

        #[cfg(debug_assertions)]
        let mut unknown_record_types: std::collections::HashSet<u32> = std::collections::HashSet::new();

        let records: Vec<(CigGuid, u32, Instance<'a>)> = self
            .db
            .all_records()
            .map(|r| (r.id(), r.struct_index(), r.as_instance()))
            .collect();
        for (guid, struct_idx, inst) in records {
            let i = struct_idx as usize;
            if let Some(Some(f)) = dispatch.get(i) {
                f(self, guid, inst);
            } else {
                #[cfg(debug_assertions)]
                { unknown_record_types.insert(struct_idx); }
            }
        }

        #[cfg(debug_assertions)]
        if !unknown_record_types.is_empty() {
            let mut names: Vec<&str> = unknown_record_types
                .iter()
                .map(|&i| self.db.struct_name(i as usize).unwrap_or("<unnamed>"))
                .collect();
            names.sort_unstable();
            panic!(
                "sc-extract-generated: runtime DCB contains {} record type(s) \
                 the generator doesn't know about — generated bindings are stale. \
                 Regenerate with `cargo run -p sc-generator -- --p4k <path>`.\n\
                 Unknown types: {:?}",
                names.len(),
                names,
            );
        }
    }
}
