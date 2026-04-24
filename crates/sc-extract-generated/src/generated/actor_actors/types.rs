// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-actors`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SDynamicGroupComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SDynamicGroupComponentParams {
}

impl Pooled for SDynamicGroupComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actors.sdynamic_group_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actors.sdynamic_group_component_params }
}

impl<'a> Extract<'a> for SDynamicGroupComponentParams {
    const TYPE_NAME: &'static str = "SDynamicGroupComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ActivityBehaviorComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct ActivityBehaviorComponentParams {
    /// `activityData` (Reference)
    pub activity_data: Option<CigGuid>,
}

impl Pooled for ActivityBehaviorComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actors.activity_behavior_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actors.activity_behavior_component_params }
}

impl<'a> Extract<'a> for ActivityBehaviorComponentParams {
    const TYPE_NAME: &'static str = "ActivityBehaviorComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            activity_data: inst.get("activityData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `TerrainTrashCleanupComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TerrainTrashCleanupComponentParams {
    /// `lifetime` (Single)
    pub lifetime: f32,
    /// `decayType` (EnumChoice)
    pub decay_type: EDecayType,
    /// `decayDuration` (Single)
    pub decay_duration: f32,
    /// `destroyAfterDecay` (Boolean)
    pub destroy_after_decay: bool,
    /// `decayBoundingBoxMultiplier` (Single)
    pub decay_bounding_box_multiplier: f32,
    /// `decayDistanceOverride` (Single)
    pub decay_distance_override: f32,
}

impl Pooled for TerrainTrashCleanupComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actors.terrain_trash_cleanup_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actors.terrain_trash_cleanup_component_params }
}

impl<'a> Extract<'a> for TerrainTrashCleanupComponentParams {
    const TYPE_NAME: &'static str = "TerrainTrashCleanupComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            lifetime: inst.get_f32("lifetime").unwrap_or_default(),
            decay_type: EDecayType::from_dcb_str(inst.get_str("decayType").unwrap_or("")),
            decay_duration: inst.get_f32("decayDuration").unwrap_or_default(),
            destroy_after_decay: inst.get_bool("destroyAfterDecay").unwrap_or_default(),
            decay_bounding_box_multiplier: inst.get_f32("decayBoundingBoxMultiplier").unwrap_or_default(),
            decay_distance_override: inst.get_f32("decayDistanceOverride").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorDeathPoseVariant`
pub struct SActorDeathPoseVariant {
    /// `variantName` (String)
    pub variant_name: String,
    /// `mannequinTag` (String)
    pub mannequin_tag: String,
    /// `geometryTags` (String)
    pub geometry_tags: String,
}

impl Pooled for SActorDeathPoseVariant {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actors.sactor_death_pose_variant }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actors.sactor_death_pose_variant }
}

impl<'a> Extract<'a> for SActorDeathPoseVariant {
    const TYPE_NAME: &'static str = "SActorDeathPoseVariant";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            variant_name: inst.get_str("variantName").map(String::from).unwrap_or_default(),
            mannequin_tag: inst.get_str("mannequinTag").map(String::from).unwrap_or_default(),
            geometry_tags: inst.get_str("geometryTags").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorStaticColliderDeathBehaviour`
/// Inherits from: `SActorDeathBehaviour`
pub struct SActorStaticColliderDeathBehaviour {
    /// `densityClassOverride` (Reference)
    pub density_class_override: Option<CigGuid>,
    /// `deadColliderClass` (Reference)
    pub dead_collider_class: Option<CigGuid>,
    /// `deathPoseVariants` (Class (array))
    pub death_pose_variants: Vec<Handle<SActorDeathPoseVariant>>,
    /// `deathOrientationEntityTag` (Reference)
    pub death_orientation_entity_tag: Option<CigGuid>,
    /// `maxTagQueryDistance` (Single)
    pub max_tag_query_distance: f32,
}

impl Pooled for SActorStaticColliderDeathBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actors.sactor_static_collider_death_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actors.sactor_static_collider_death_behaviour }
}

impl<'a> Extract<'a> for SActorStaticColliderDeathBehaviour {
    const TYPE_NAME: &'static str = "SActorStaticColliderDeathBehaviour";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            density_class_override: inst.get("densityClassOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            dead_collider_class: inst.get("deadColliderClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            death_pose_variants: inst.get_array("deathPoseVariants")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorDeathPoseVariant>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SActorDeathPoseVariant>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            death_orientation_entity_tag: inst.get("deathOrientationEntityTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            max_tag_query_distance: inst.get_f32("maxTagQueryDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `SDummyPlayerComponentParams`
/// Inherits from: `SActorComponentParams`
pub struct SDummyPlayerComponentParams {
    /// `character` (Reference)
    pub character: Option<CigGuid>,
    /// `isAIControlled` (Boolean)
    pub is_aicontrolled: bool,
    /// `flyInGroundState` (Boolean)
    pub fly_in_ground_state: bool,
    /// `aimFOV` (Single)
    pub aim_fov: f32,
    /// `maxLookAimAngle` (Single)
    pub max_look_aim_angle: f32,
    /// `maxDeltaAngleRateNormal` (Single)
    pub max_delta_angle_rate_normal: f32,
    /// `jumpFallLandParams` (Reference)
    pub jump_fall_land_params: Option<CigGuid>,
    /// `aimIKLayer` (Int32)
    pub aim_iklayer: i32,
    /// `lookIKLayer` (Int32)
    pub look_iklayer: i32,
    /// `characterNameData` (Reference)
    pub character_name_data: Option<CigGuid>,
    /// `skeletonConfig` (Reference)
    pub skeleton_config: Option<CigGuid>,
    /// `ledgeGrabbingParams` (Reference)
    pub ledge_grabbing_params: Option<CigGuid>,
    /// `jumpFallLandConfig` (Reference)
    pub jump_fall_land_config: Option<CigGuid>,
    /// `carryConfig` (Reference)
    pub carry_config: Option<CigGuid>,
    /// `playerAnimatedInteractionConfig` (Reference)
    pub player_animated_interaction_config: Option<CigGuid>,
    /// `ladderConfig` (Reference)
    pub ladder_config: Option<CigGuid>,
    /// `ladderConfigV2` (Reference)
    pub ladder_config_v2: Option<CigGuid>,
    /// `proceduralAnimationRecord` (Reference)
    pub procedural_animation_record: Option<CigGuid>,
    /// `movementModifiersRecord` (Reference)
    pub movement_modifiers_record: Option<CigGuid>,
    /// `movementSetsRecord` (Reference)
    pub movement_sets_record: Option<CigGuid>,
    /// `isMinimal` (Boolean)
    pub is_minimal: bool,
    /// `aiDefaultStance` (EnumChoice)
    pub ai_default_stance: ActorStateFilterByStanceState,
    /// `actorType` (EnumChoice)
    pub actor_type: EActorType,
    /// `stancesDataRecord` (Reference)
    pub stances_data_record: Option<CigGuid>,
    /// `actorStatusRecord` (Reference)
    pub actor_status_record: Option<CigGuid>,
    /// `actorStatusRecordEA` (Reference)
    pub actor_status_record_ea: Option<CigGuid>,
    /// `actorStatusRecordSQ42` (Reference)
    pub actor_status_record_sq42: Option<CigGuid>,
    /// `takeDownConfig` (Reference)
    pub take_down_config: Option<CigGuid>,
    /// `misfireFixConfig` (Reference)
    pub misfire_fix_config: Option<CigGuid>,
    /// `hitReactionsDefRecord` (Reference)
    pub hit_reactions_def_record: Option<CigGuid>,
    /// `forceReactionsDefRecord` (Reference)
    pub force_reactions_def_record: Option<CigGuid>,
    /// `shakeRecordFirstPerson` (Reference)
    pub shake_record_first_person: Option<CigGuid>,
    /// `shakeRecordThirdPerson` (Reference)
    pub shake_record_third_person: Option<CigGuid>,
    /// `actorAimLimits` (Reference)
    pub actor_aim_limits: Option<CigGuid>,
    /// `actorLookLimits` (Reference)
    pub actor_look_limits: Option<CigGuid>,
    /// `turnAngles` (Class)
    pub turn_angles: Option<Handle<ActorTurnAngles>>,
    /// `hazardParams` (Reference)
    pub hazard_params: Option<CigGuid>,
    /// `duckingParams` (Reference)
    pub ducking_params: Option<CigGuid>,
    /// `gforceParamsRecord` (Reference)
    pub gforce_params_record: Option<CigGuid>,
    /// `gforceParamsRecordMasterModes` (Reference)
    pub gforce_params_record_master_modes: Option<CigGuid>,
    /// `gforceHeadBobRecord` (Reference)
    pub gforce_head_bob_record: Option<CigGuid>,
    /// `gforceCameraEffectsRecord` (Reference)
    pub gforce_camera_effects_record: Option<CigGuid>,
    /// `adsCameraDefaultParams` (StrongPointer)
    pub ads_camera_default_params: Option<Handle<AdsCameraParams>>,
    /// `abilityParamsRecord` (Reference)
    pub ability_params_record: Option<CigGuid>,
    /// `breathingParamsRecord` (Reference)
    pub breathing_params_record: Option<CigGuid>,
    /// `staminaParamsRecord` (Reference)
    pub stamina_params_record: Option<CigGuid>,
    /// `actorEnvironmentRecord` (Reference)
    pub actor_environment_record: Option<CigGuid>,
    /// `defaultLookAheadRecord` (Reference)
    pub default_look_ahead_record: Option<CigGuid>,
    /// `locomotionPersonalityRecord` (Reference)
    pub locomotion_personality_record: Option<CigGuid>,
    /// `weaponPortsParams` (Class)
    pub weapon_ports_params: Option<Handle<WeaponPortsParams>>,
    /// `actorSpecies` (Locale)
    pub actor_species: LocaleKey,
    /// `swapOutfitInteraction` (WeakPointer)
    pub swap_outfit_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `wingmanOrders` (Reference)
    pub wingman_orders: Option<CigGuid>,
    /// `actorTargetedParams` (Reference)
    pub actor_targeted_params: Option<CigGuid>,
    /// `actorSlidingParams` (Reference)
    pub actor_sliding_params: Option<CigGuid>,
    /// `actorZeroGTraversalParams` (Reference)
    pub actor_zero_gtraversal_params: Option<CigGuid>,
    /// `baseFoleyDef` (Reference)
    pub base_foley_def: Option<CigGuid>,
    /// `baseFootstepDef` (Reference)
    pub base_footstep_def: Option<CigGuid>,
    /// `deathBehaviour` (StrongPointer)
    pub death_behaviour: Option<SActorDeathBehaviourPtr>,
}

impl Pooled for SDummyPlayerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actors.sdummy_player_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actors.sdummy_player_component_params }
}

impl<'a> Extract<'a> for SDummyPlayerComponentParams {
    const TYPE_NAME: &'static str = "SDummyPlayerComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            character: inst.get("character").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            is_aicontrolled: inst.get_bool("isAIControlled").unwrap_or_default(),
            fly_in_ground_state: inst.get_bool("flyInGroundState").unwrap_or_default(),
            aim_fov: inst.get_f32("aimFOV").unwrap_or_default(),
            max_look_aim_angle: inst.get_f32("maxLookAimAngle").unwrap_or_default(),
            max_delta_angle_rate_normal: inst.get_f32("maxDeltaAngleRateNormal").unwrap_or_default(),
            jump_fall_land_params: inst.get("jumpFallLandParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            aim_iklayer: inst.get_i32("aimIKLayer").unwrap_or_default(),
            look_iklayer: inst.get_i32("lookIKLayer").unwrap_or_default(),
            character_name_data: inst.get("characterNameData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            skeleton_config: inst.get("skeletonConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ledge_grabbing_params: inst.get("ledgeGrabbingParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            jump_fall_land_config: inst.get("jumpFallLandConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            carry_config: inst.get("carryConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            player_animated_interaction_config: inst.get("playerAnimatedInteractionConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ladder_config: inst.get("ladderConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ladder_config_v2: inst.get("ladderConfigV2").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            procedural_animation_record: inst.get("proceduralAnimationRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            movement_modifiers_record: inst.get("movementModifiersRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            movement_sets_record: inst.get("movementSetsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            is_minimal: inst.get_bool("isMinimal").unwrap_or_default(),
            ai_default_stance: ActorStateFilterByStanceState::from_dcb_str(inst.get_str("aiDefaultStance").unwrap_or("")),
            actor_type: EActorType::from_dcb_str(inst.get_str("actorType").unwrap_or("")),
            stances_data_record: inst.get("stancesDataRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_status_record: inst.get("actorStatusRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_status_record_ea: inst.get("actorStatusRecordEA").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_status_record_sq42: inst.get("actorStatusRecordSQ42").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            take_down_config: inst.get("takeDownConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            misfire_fix_config: inst.get("misfireFixConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            hit_reactions_def_record: inst.get("hitReactionsDefRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            force_reactions_def_record: inst.get("forceReactionsDefRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            shake_record_first_person: inst.get("shakeRecordFirstPerson").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            shake_record_third_person: inst.get("shakeRecordThirdPerson").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_aim_limits: inst.get("actorAimLimits").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_look_limits: inst.get("actorLookLimits").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            turn_angles: match inst.get("turnAngles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorTurnAngles>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            hazard_params: inst.get("hazardParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ducking_params: inst.get("duckingParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gforce_params_record: inst.get("gforceParamsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gforce_params_record_master_modes: inst.get("gforceParamsRecordMasterModes").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gforce_head_bob_record: inst.get("gforceHeadBobRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            gforce_camera_effects_record: inst.get("gforceCameraEffectsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ads_camera_default_params: match inst.get("adsCameraDefaultParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AdsCameraParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ability_params_record: inst.get("abilityParamsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            breathing_params_record: inst.get("breathingParamsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            stamina_params_record: inst.get("staminaParamsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_environment_record: inst.get("actorEnvironmentRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_look_ahead_record: inst.get("defaultLookAheadRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            locomotion_personality_record: inst.get("locomotionPersonalityRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            weapon_ports_params: match inst.get("weaponPortsParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<WeaponPortsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            actor_species: inst.get_str("actorSpecies").map(LocaleKey::from).unwrap_or_default(),
            swap_outfit_interaction: match inst.get("swapOutfitInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wingman_orders: inst.get("wingmanOrders").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_targeted_params: inst.get("actorTargetedParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_sliding_params: inst.get("actorSlidingParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_zero_gtraversal_params: inst.get("actorZeroGTraversalParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            base_foley_def: inst.get("baseFoleyDef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            base_footstep_def: inst.get("baseFootstepDef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            death_behaviour: match inst.get("deathBehaviour") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SActorDeathBehaviourPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TerrainTrashCleanupGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
pub struct TerrainTrashCleanupGameplayTrigger {
    /// `enable` (Boolean)
    pub enable: bool,
    /// `overrides` (StrongPointer)
    pub overrides: Option<Handle<TerrainTrashCleanupComponentParams>>,
}

impl Pooled for TerrainTrashCleanupGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actors.terrain_trash_cleanup_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actors.terrain_trash_cleanup_gameplay_trigger }
}

impl<'a> Extract<'a> for TerrainTrashCleanupGameplayTrigger {
    const TYPE_NAME: &'static str = "TerrainTrashCleanupGameplayTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            overrides: match inst.get("overrides") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TerrainTrashCleanupComponentParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UndergroundCreatureHelpersComponent`
/// Inherits from: `DataForgeComponentParams`
pub struct UndergroundCreatureHelpersComponent {
    /// `spineJoints` (String (array))
    pub spine_joints: Vec<String>,
    /// `tailJoints` (String (array))
    pub tail_joints: Vec<String>,
    /// `surfaceVFXJoint` (String)
    pub surface_vfxjoint: String,
    /// `groundExitVFXJoint` (String)
    pub ground_exit_vfxjoint: String,
    /// `groundEntryVFXJoint` (String)
    pub ground_entry_vfxjoint: String,
    /// `spawnMolehillEntity` (Boolean)
    pub spawn_molehill_entity: bool,
    /// `numOfStartingMolehillEntities` (Byte)
    pub num_of_starting_molehill_entities: u32,
    /// `startExitDecayingImmediately` (Boolean)
    pub start_exit_decaying_immediately: bool,
    /// `overrideExitDecayTimeToStart` (Single)
    pub override_exit_decay_time_to_start: f32,
    /// `molehillClass` (Reference)
    pub molehill_class: Option<CigGuid>,
    /// `triggerEmergeMolehillAtAnimationStart` (Boolean)
    pub trigger_emerge_molehill_at_animation_start: bool,
}

impl Pooled for UndergroundCreatureHelpersComponent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actors.underground_creature_helpers_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actors.underground_creature_helpers_component }
}

impl<'a> Extract<'a> for UndergroundCreatureHelpersComponent {
    const TYPE_NAME: &'static str = "UndergroundCreatureHelpersComponent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            spine_joints: inst.get_array("spineJoints")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            tail_joints: inst.get_array("tailJoints")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            surface_vfxjoint: inst.get_str("surfaceVFXJoint").map(String::from).unwrap_or_default(),
            ground_exit_vfxjoint: inst.get_str("groundExitVFXJoint").map(String::from).unwrap_or_default(),
            ground_entry_vfxjoint: inst.get_str("groundEntryVFXJoint").map(String::from).unwrap_or_default(),
            spawn_molehill_entity: inst.get_bool("spawnMolehillEntity").unwrap_or_default(),
            num_of_starting_molehill_entities: inst.get_u32("numOfStartingMolehillEntities").unwrap_or_default(),
            start_exit_decaying_immediately: inst.get_bool("startExitDecayingImmediately").unwrap_or_default(),
            override_exit_decay_time_to_start: inst.get_f32("overrideExitDecayTimeToStart").unwrap_or_default(),
            molehill_class: inst.get("molehillClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            trigger_emerge_molehill_at_animation_start: inst.get_bool("triggerEmergeMolehillAtAnimationStart").unwrap_or_default(),
        }
    }
}

