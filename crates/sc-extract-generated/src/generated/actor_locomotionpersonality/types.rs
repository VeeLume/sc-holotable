// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-locomotionpersonality`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SLocomotionPersonalityStateFilter`
/// Inherits from: `ActorMotionStateFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLocomotionPersonalityStateFilter {
    /// `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// `filterByLocomotionSet` (EnumChoice)
    #[serde(default)]
    pub filter_by_locomotion_set: String,
    /// `filterByMannequinGlobalTags` (String)
    #[serde(default)]
    pub filter_by_mannequin_global_tags: String,
}

impl Pooled for SLocomotionPersonalityStateFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.slocomotion_personality_state_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.slocomotion_personality_state_filter }
}

impl<'a> Extract<'a> for SLocomotionPersonalityStateFilter {
    const TYPE_NAME: &'static str = "SLocomotionPersonalityStateFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filter_name: inst.get_str("filterName").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            filter_by_motion_speed: inst.get_str("filterByMotionSpeed").map(String::from).unwrap_or_default(),
            filter_by_pose_state: inst.get_str("filterByPoseState").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_lean_state: inst.get_str("filterByLeanState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
            filter_by_character_type: inst.get_str("filterByCharacterType").map(String::from).unwrap_or_default(),
            filter_by_restrained_state: inst.get_str("filterByRestrainedState").map(String::from).unwrap_or_default(),
            filter_by_player_camera: inst.get_str("filterByPlayerCamera").map(String::from).unwrap_or_default(),
            filter_by_aiming_restriction: inst.get_str("filterByAimingRestriction").map(String::from).unwrap_or_default(),
            filter_by_locomotion_set: inst.get_str("filterByLocomotionSet").map(String::from).unwrap_or_default(),
            filter_by_mannequin_global_tags: inst.get_str("filterByMannequinGlobalTags").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLocomotionRotateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionRotateParams {
    /// `maxDeltaAngleRateNormal` (Single)
    #[serde(default)]
    pub max_delta_angle_rate_normal: f32,
    /// `maxDeltaAngleRateCatchup` (Single)
    #[serde(default)]
    pub max_delta_angle_rate_catchup: f32,
    /// `maxDeltaAngleRateCombat` (Single)
    #[serde(default)]
    pub max_delta_angle_rate_combat: f32,
    /// `maxDeltaAngleRateExactPositioning` (Single)
    #[serde(default)]
    pub max_delta_angle_rate_exact_positioning: f32,
    /// `turnSpeedCurveDefault` (Reference)
    #[serde(default)]
    pub turn_speed_curve_default: Option<CigGuid>,
}

impl Pooled for ActorLocomotionRotateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.actor_locomotion_rotate_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.actor_locomotion_rotate_params }
}

impl<'a> Extract<'a> for ActorLocomotionRotateParams {
    const TYPE_NAME: &'static str = "ActorLocomotionRotateParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_delta_angle_rate_normal: inst.get_f32("maxDeltaAngleRateNormal").unwrap_or_default(),
            max_delta_angle_rate_catchup: inst.get_f32("maxDeltaAngleRateCatchup").unwrap_or_default(),
            max_delta_angle_rate_combat: inst.get_f32("maxDeltaAngleRateCombat").unwrap_or_default(),
            max_delta_angle_rate_exact_positioning: inst.get_f32("maxDeltaAngleRateExactPositioning").unwrap_or_default(),
            turn_speed_curve_default: inst.get("turnSpeedCurveDefault").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ActorLocomotionTurnOnSpotParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionTurnOnSpotParams {
    /// `minAngle` (Single)
    #[serde(default)]
    pub min_angle: f32,
    /// `minAngleForTurnWithoutDelay` (Single)
    #[serde(default)]
    pub min_angle_for_turn_without_delay: f32,
    /// `maxDelayTime` (Single)
    #[serde(default)]
    pub max_delay_time: f32,
    /// `requeueNormTime` (Single)
    #[serde(default)]
    pub requeue_norm_time: f32,
}

impl Pooled for ActorLocomotionTurnOnSpotParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.actor_locomotion_turn_on_spot_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.actor_locomotion_turn_on_spot_params }
}

impl<'a> Extract<'a> for ActorLocomotionTurnOnSpotParams {
    const TYPE_NAME: &'static str = "ActorLocomotionTurnOnSpotParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_angle: inst.get_f32("minAngle").unwrap_or_default(),
            min_angle_for_turn_without_delay: inst.get_f32("minAngleForTurnWithoutDelay").unwrap_or_default(),
            max_delay_time: inst.get_f32("maxDelayTime").unwrap_or_default(),
            requeue_norm_time: inst.get_f32("requeueNormTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLocomotionSharpTurnParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionSharpTurnParams {
    /// `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// `enableForWalking` (Boolean)
    #[serde(default)]
    pub enable_for_walking: bool,
    /// `enableForRunning` (Boolean)
    #[serde(default)]
    pub enable_for_running: bool,
    /// `enableForSprinting` (Boolean)
    #[serde(default)]
    pub enable_for_sprinting: bool,
    /// `enableForWeapon` (Boolean)
    #[serde(default)]
    pub enable_for_weapon: bool,
    /// `enableForWeaponWalking` (Boolean)
    #[serde(default)]
    pub enable_for_weapon_walking: bool,
    /// `enableForWeaponRunning` (Boolean)
    #[serde(default)]
    pub enable_for_weapon_running: bool,
    /// `enableForWeaponSprinting` (Boolean)
    #[serde(default)]
    pub enable_for_weapon_sprinting: bool,
    /// `enableForNW` (Boolean)
    #[serde(default)]
    pub enable_for_nw: bool,
    /// `enableForNoWeaponWalking` (Boolean)
    #[serde(default)]
    pub enable_for_no_weapon_walking: bool,
    /// `enableForNoWeaponRunning` (Boolean)
    #[serde(default)]
    pub enable_for_no_weapon_running: bool,
    /// `enableForNoWeaponSprinting` (Boolean)
    #[serde(default)]
    pub enable_for_no_weapon_sprinting: bool,
    /// `enableOnGaitChange` (Boolean)
    #[serde(default)]
    pub enable_on_gait_change: bool,
    /// `minActivationAngleDefault` (Single)
    #[serde(default)]
    pub min_activation_angle_default: f32,
    /// `minActivationAngleWalk` (Single)
    #[serde(default)]
    pub min_activation_angle_walk: f32,
}

impl Pooled for ActorLocomotionSharpTurnParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.actor_locomotion_sharp_turn_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.actor_locomotion_sharp_turn_params }
}

impl<'a> Extract<'a> for ActorLocomotionSharpTurnParams {
    const TYPE_NAME: &'static str = "ActorLocomotionSharpTurnParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            enable_for_walking: inst.get_bool("enableForWalking").unwrap_or_default(),
            enable_for_running: inst.get_bool("enableForRunning").unwrap_or_default(),
            enable_for_sprinting: inst.get_bool("enableForSprinting").unwrap_or_default(),
            enable_for_weapon: inst.get_bool("enableForWeapon").unwrap_or_default(),
            enable_for_weapon_walking: inst.get_bool("enableForWeaponWalking").unwrap_or_default(),
            enable_for_weapon_running: inst.get_bool("enableForWeaponRunning").unwrap_or_default(),
            enable_for_weapon_sprinting: inst.get_bool("enableForWeaponSprinting").unwrap_or_default(),
            enable_for_nw: inst.get_bool("enableForNW").unwrap_or_default(),
            enable_for_no_weapon_walking: inst.get_bool("enableForNoWeaponWalking").unwrap_or_default(),
            enable_for_no_weapon_running: inst.get_bool("enableForNoWeaponRunning").unwrap_or_default(),
            enable_for_no_weapon_sprinting: inst.get_bool("enableForNoWeaponSprinting").unwrap_or_default(),
            enable_on_gait_change: inst.get_bool("enableOnGaitChange").unwrap_or_default(),
            min_activation_angle_default: inst.get_f32("minActivationAngleDefault").unwrap_or_default(),
            min_activation_angle_walk: inst.get_f32("minActivationAngleWalk").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLocomotionAvoidanceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionAvoidanceParams {
    /// `enableForceStrafe` (Boolean)
    #[serde(default)]
    pub enable_force_strafe: bool,
    /// `enableAvoidanceTwist` (Boolean)
    #[serde(default)]
    pub enable_avoidance_twist: bool,
    /// `twistAllowedHalfFOV` (Single)
    #[serde(default)]
    pub twist_allowed_half_fov: f32,
    /// `minTwistDistance` (Single)
    #[serde(default)]
    pub min_twist_distance: f32,
    /// `maxTwistDistance` (Single)
    #[serde(default)]
    pub max_twist_distance: f32,
    /// `blendWeightAtMinTwist` (Single)
    #[serde(default)]
    pub blend_weight_at_min_twist: f32,
    /// `blendWeightAtMaxTwist` (Single)
    #[serde(default)]
    pub blend_weight_at_max_twist: f32,
    /// `twistBlendTime` (Single)
    #[serde(default)]
    pub twist_blend_time: f32,
    /// `twistBlendOutTime` (Single)
    #[serde(default)]
    pub twist_blend_out_time: f32,
    /// `maxDistanceFromPath` (Single)
    #[serde(default)]
    pub max_distance_from_path: f32,
}

impl Pooled for ActorLocomotionAvoidanceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.actor_locomotion_avoidance_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.actor_locomotion_avoidance_params }
}

impl<'a> Extract<'a> for ActorLocomotionAvoidanceParams {
    const TYPE_NAME: &'static str = "ActorLocomotionAvoidanceParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable_force_strafe: inst.get_bool("enableForceStrafe").unwrap_or_default(),
            enable_avoidance_twist: inst.get_bool("enableAvoidanceTwist").unwrap_or_default(),
            twist_allowed_half_fov: inst.get_f32("twistAllowedHalfFOV").unwrap_or_default(),
            min_twist_distance: inst.get_f32("minTwistDistance").unwrap_or_default(),
            max_twist_distance: inst.get_f32("maxTwistDistance").unwrap_or_default(),
            blend_weight_at_min_twist: inst.get_f32("blendWeightAtMinTwist").unwrap_or_default(),
            blend_weight_at_max_twist: inst.get_f32("blendWeightAtMaxTwist").unwrap_or_default(),
            twist_blend_time: inst.get_f32("twistBlendTime").unwrap_or_default(),
            twist_blend_out_time: inst.get_f32("twistBlendOutTime").unwrap_or_default(),
            max_distance_from_path: inst.get_f32("maxDistanceFromPath").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorLocomotionFidgetSeverityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionFidgetSeverityParams {
    /// `fragmentTags` (String)
    #[serde(default)]
    pub fragment_tags: String,
    /// `severityWeight` (Single)
    #[serde(default)]
    pub severity_weight: f32,
}

impl Pooled for SActorLocomotionFidgetSeverityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.sactor_locomotion_fidget_severity_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.sactor_locomotion_fidget_severity_params }
}

impl<'a> Extract<'a> for SActorLocomotionFidgetSeverityParams {
    const TYPE_NAME: &'static str = "SActorLocomotionFidgetSeverityParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fragment_tags: inst.get_str("fragmentTags").map(String::from).unwrap_or_default(),
            severity_weight: inst.get_f32("severityWeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorLocomotionFidgetStateFilteredDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionFidgetStateFilteredDef {
    /// `stateFilter` (Class)
    #[serde(default)]
    pub state_filter: Option<Handle<SLocomotionPersonalityStateFilter>>,
    /// `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// `cooldownMin` (Single)
    #[serde(default)]
    pub cooldown_min: f32,
    /// `cooldownMax` (Single)
    #[serde(default)]
    pub cooldown_max: f32,
    /// `severities` (Class (array))
    #[serde(default)]
    pub severities: Vec<Handle<SActorLocomotionFidgetSeverityParams>>,
}

impl Pooled for SActorLocomotionFidgetStateFilteredDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.sactor_locomotion_fidget_state_filtered_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.sactor_locomotion_fidget_state_filtered_def }
}

impl<'a> Extract<'a> for SActorLocomotionFidgetStateFilteredDef {
    const TYPE_NAME: &'static str = "SActorLocomotionFidgetStateFilteredDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_filter: match inst.get("stateFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SLocomotionPersonalityStateFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SLocomotionPersonalityStateFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            enable: inst.get_bool("enable").unwrap_or_default(),
            cooldown_min: inst.get_f32("cooldownMin").unwrap_or_default(),
            cooldown_max: inst.get_f32("cooldownMax").unwrap_or_default(),
            severities: inst.get_array("severities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SActorLocomotionFidgetSeverityParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SActorLocomotionFidgetSeverityParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorLocomotionFidgetDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionFidgetDef {
    /// `enable` (Boolean)
    #[serde(default)]
    pub enable: bool,
    /// `stateDefs` (Reference (array))
    #[serde(default)]
    pub state_defs: Vec<CigGuid>,
}

impl Pooled for SActorLocomotionFidgetDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.sactor_locomotion_fidget_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.sactor_locomotion_fidget_def }
}

impl<'a> Extract<'a> for SActorLocomotionFidgetDef {
    const TYPE_NAME: &'static str = "SActorLocomotionFidgetDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable: inst.get_bool("enable").unwrap_or_default(),
            state_defs: inst.get_array("stateDefs")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorLocomotionFeatureDef_Slope`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionFeatureDef_Slope {
    /// `enableStairAnimSet` (Boolean)
    #[serde(default)]
    pub enable_stair_anim_set: bool,
}

impl Pooled for SActorLocomotionFeatureDef_Slope {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.sactor_locomotion_feature_def_slope }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.sactor_locomotion_feature_def_slope }
}

impl<'a> Extract<'a> for SActorLocomotionFeatureDef_Slope {
    const TYPE_NAME: &'static str = "SActorLocomotionFeatureDef_Slope";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable_stair_anim_set: inst.get_bool("enableStairAnimSet").unwrap_or_default(),
        }
    }
}

/// DCB type: `SActorLocomotionSubmergedCreatureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SActorLocomotionSubmergedCreatureParams {
    /// `allowMotionActionTurnOnSpot` (Boolean)
    #[serde(default)]
    pub allow_motion_action_turn_on_spot: bool,
}

impl Pooled for SActorLocomotionSubmergedCreatureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.sactor_locomotion_submerged_creature_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.sactor_locomotion_submerged_creature_params }
}

impl<'a> Extract<'a> for SActorLocomotionSubmergedCreatureParams {
    const TYPE_NAME: &'static str = "SActorLocomotionSubmergedCreatureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            allow_motion_action_turn_on_spot: inst.get_bool("allowMotionActionTurnOnSpot").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorLocomotionPersonality`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorLocomotionPersonality {
    /// `locomotionStyleMannequinTag` (String)
    #[serde(default)]
    pub locomotion_style_mannequin_tag: String,
    /// `basePlayerMovementMannequinAction` (Reference)
    #[serde(default)]
    pub base_player_movement_mannequin_action: Option<CigGuid>,
    /// `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<ActorLocomotionRotateParams>>,
    /// `turnOnSpotNormal` (Class)
    #[serde(default)]
    pub turn_on_spot_normal: Option<Handle<ActorLocomotionTurnOnSpotParams>>,
    /// `turnOnSpotCombat` (Class)
    #[serde(default)]
    pub turn_on_spot_combat: Option<Handle<ActorLocomotionTurnOnSpotParams>>,
    /// `sharpTurns` (Class)
    #[serde(default)]
    pub sharp_turns: Option<Handle<ActorLocomotionSharpTurnParams>>,
    /// `avoidance` (Class)
    #[serde(default)]
    pub avoidance: Option<Handle<ActorLocomotionAvoidanceParams>>,
    /// `fidget` (Reference)
    #[serde(default)]
    pub fidget: Option<CigGuid>,
    /// `slope` (Class)
    #[serde(default)]
    pub slope: Option<Handle<SActorLocomotionFeatureDef_Slope>>,
    /// `submergedCreatureConfig` (StrongPointer)
    #[serde(default)]
    pub submerged_creature_config: Option<Handle<SActorLocomotionSubmergedCreatureParams>>,
}

impl Pooled for ActorLocomotionPersonality {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_locomotionpersonality.actor_locomotion_personality }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_locomotionpersonality.actor_locomotion_personality }
}

impl<'a> Extract<'a> for ActorLocomotionPersonality {
    const TYPE_NAME: &'static str = "ActorLocomotionPersonality";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            locomotion_style_mannequin_tag: inst.get_str("locomotionStyleMannequinTag").map(String::from).unwrap_or_default(),
            base_player_movement_mannequin_action: inst.get("basePlayerMovementMannequinAction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionRotateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionRotateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turn_on_spot_normal: match inst.get("turnOnSpotNormal") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionTurnOnSpotParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionTurnOnSpotParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turn_on_spot_combat: match inst.get("turnOnSpotCombat") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionTurnOnSpotParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionTurnOnSpotParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sharp_turns: match inst.get("sharpTurns") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionSharpTurnParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionSharpTurnParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            avoidance: match inst.get("avoidance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorLocomotionAvoidanceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorLocomotionAvoidanceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fidget: inst.get("fidget").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            slope: match inst.get("slope") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorLocomotionFeatureDef_Slope>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorLocomotionFeatureDef_Slope>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            submerged_creature_config: match inst.get("submergedCreatureConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SActorLocomotionSubmergedCreatureParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SActorLocomotionSubmergedCreatureParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

