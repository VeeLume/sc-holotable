// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `closecombat`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ActorStateSkeletonFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStateSkeletonFilter {
    /// `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
}

impl Pooled for ActorStateSkeletonFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.closecombat.actor_state_skeleton_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.closecombat.actor_state_skeleton_filter }
}

impl<'a> Extract<'a> for ActorStateSkeletonFilter {
    const TYPE_NAME: &'static str = "ActorStateSkeletonFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorRestrainPerAttackerConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRestrainPerAttackerConfig {
    /// `attackerSkeleton` (Class)
    #[serde(default)]
    pub attacker_skeleton: Option<Handle<ActorStateSkeletonFilter>>,
    /// `frontalQuadrantAngle` (Single)
    #[serde(default)]
    pub frontal_quadrant_angle: f32,
    /// `immuneToRestrain` (Boolean)
    #[serde(default)]
    pub immune_to_restrain: bool,
    /// `interruptOnHitReaction` (Boolean)
    #[serde(default)]
    pub interrupt_on_hit_reaction: bool,
}

impl Pooled for ActorRestrainPerAttackerConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.closecombat.actor_restrain_per_attacker_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.closecombat.actor_restrain_per_attacker_config }
}

impl<'a> Extract<'a> for ActorRestrainPerAttackerConfig {
    const TYPE_NAME: &'static str = "ActorRestrainPerAttackerConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attacker_skeleton: match inst.get("attackerSkeleton") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateSkeletonFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateSkeletonFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            frontal_quadrant_angle: inst.get_f32("frontalQuadrantAngle").unwrap_or_default(),
            immune_to_restrain: inst.get_bool("immuneToRestrain").unwrap_or_default(),
            interrupt_on_hit_reaction: inst.get_bool("interruptOnHitReaction").unwrap_or_default(),
        }
    }
}

/// DCB type: `ActorRestrainConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorRestrainConfig {
    /// `immuneToRestrain` (Boolean)
    #[serde(default)]
    pub immune_to_restrain: bool,
    /// `perAttackerConfigs` (Class (array))
    #[serde(default)]
    pub per_attacker_configs: Vec<Handle<ActorRestrainPerAttackerConfig>>,
    /// `restrainVisibilityCheckJoints` (String (array))
    #[serde(default)]
    pub restrain_visibility_check_joints: Vec<String>,
}

impl Pooled for ActorRestrainConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.closecombat.actor_restrain_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.closecombat.actor_restrain_config }
}

impl<'a> Extract<'a> for ActorRestrainConfig {
    const TYPE_NAME: &'static str = "ActorRestrainConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            immune_to_restrain: inst.get_bool("immuneToRestrain").unwrap_or_default(),
            per_attacker_configs: inst.get_array("perAttackerConfigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorRestrainPerAttackerConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorRestrainPerAttackerConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            restrain_visibility_check_joints: inst.get_array("restrainVisibilityCheckJoints")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TakeDownMaxDistances`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeDownMaxDistances {
    /// `maxDistanceFront` (Single)
    #[serde(default)]
    pub max_distance_front: f32,
    /// `maxDistanceBack` (Single)
    #[serde(default)]
    pub max_distance_back: f32,
    /// `maxDistanceRight` (Single)
    #[serde(default)]
    pub max_distance_right: f32,
    /// `maxDistanceLeft` (Single)
    #[serde(default)]
    pub max_distance_left: f32,
}

impl Pooled for TakeDownMaxDistances {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.closecombat.take_down_max_distances }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.closecombat.take_down_max_distances }
}

impl<'a> Extract<'a> for TakeDownMaxDistances {
    const TYPE_NAME: &'static str = "TakeDownMaxDistances";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_distance_front: inst.get_f32("maxDistanceFront").unwrap_or_default(),
            max_distance_back: inst.get_f32("maxDistanceBack").unwrap_or_default(),
            max_distance_right: inst.get_f32("maxDistanceRight").unwrap_or_default(),
            max_distance_left: inst.get_f32("maxDistanceLeft").unwrap_or_default(),
        }
    }
}

/// DCB type: `TakeDownParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeDownParams {
    /// `attackerSkeletonFilter` (Class)
    #[serde(default)]
    pub attacker_skeleton_filter: Option<Handle<ActorStateSkeletonFilter>>,
    /// `maxDistances` (Class)
    #[serde(default)]
    pub max_distances: Option<Handle<TakeDownMaxDistances>>,
    /// `angleRange` (Single)
    #[serde(default)]
    pub angle_range: f32,
    /// `surpriseDelay` (Single)
    #[serde(default)]
    pub surprise_delay: f32,
    /// `maxHeightDiffUp` (Single)
    #[serde(default)]
    pub max_height_diff_up: f32,
    /// `maxHeightDiffDown` (Single)
    #[serde(default)]
    pub max_height_diff_down: f32,
    /// `victimStance` (EnumChoice)
    #[serde(default)]
    pub victim_stance: String,
    /// `attackerQuadrant` (EnumChoice)
    #[serde(default)]
    pub attacker_quadrant: String,
    /// `isTakeDownAlwaysLethal` (Boolean)
    #[serde(default)]
    pub is_take_down_always_lethal: bool,
    /// `interruptOnHitReaction` (Boolean)
    #[serde(default)]
    pub interrupt_on_hit_reaction: bool,
    /// `isDodgeable` (Boolean)
    #[serde(default)]
    pub is_dodgeable: bool,
    /// `dodgeWeaponRequiredTag` (Reference)
    #[serde(default)]
    pub dodge_weapon_required_tag: Option<CigGuid>,
    /// `animSpeedupOnDodge` (Single)
    #[serde(default)]
    pub anim_speedup_on_dodge: f32,
    /// `maxSpeedForRangeBoost` (Single)
    #[serde(default)]
    pub max_speed_for_range_boost: f32,
    /// `minSpeedForRangeBoost` (Single)
    #[serde(default)]
    pub min_speed_for_range_boost: f32,
    /// `RangeBoostForSpeed` (Single)
    #[serde(default)]
    pub range_boost_for_speed: f32,
}

impl Pooled for TakeDownParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.closecombat.take_down_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.closecombat.take_down_params }
}

impl<'a> Extract<'a> for TakeDownParams {
    const TYPE_NAME: &'static str = "TakeDownParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            attacker_skeleton_filter: match inst.get("attackerSkeletonFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ActorStateSkeletonFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ActorStateSkeletonFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_distances: match inst.get("maxDistances") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TakeDownMaxDistances>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TakeDownMaxDistances>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angle_range: inst.get_f32("angleRange").unwrap_or_default(),
            surprise_delay: inst.get_f32("surpriseDelay").unwrap_or_default(),
            max_height_diff_up: inst.get_f32("maxHeightDiffUp").unwrap_or_default(),
            max_height_diff_down: inst.get_f32("maxHeightDiffDown").unwrap_or_default(),
            victim_stance: inst.get_str("victimStance").map(String::from).unwrap_or_default(),
            attacker_quadrant: inst.get_str("attackerQuadrant").map(String::from).unwrap_or_default(),
            is_take_down_always_lethal: inst.get_bool("isTakeDownAlwaysLethal").unwrap_or_default(),
            interrupt_on_hit_reaction: inst.get_bool("interruptOnHitReaction").unwrap_or_default(),
            is_dodgeable: inst.get_bool("isDodgeable").unwrap_or_default(),
            dodge_weapon_required_tag: inst.get("dodgeWeaponRequiredTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            anim_speedup_on_dodge: inst.get_f32("animSpeedupOnDodge").unwrap_or_default(),
            max_speed_for_range_boost: inst.get_f32("maxSpeedForRangeBoost").unwrap_or_default(),
            min_speed_for_range_boost: inst.get_f32("minSpeedForRangeBoost").unwrap_or_default(),
            range_boost_for_speed: inst.get_f32("RangeBoostForSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `TakeDownConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeDownConfig {
    /// `takeDownParams` (Class (array))
    #[serde(default)]
    pub take_down_params: Vec<Handle<TakeDownParams>>,
    /// `isTakedownImmune` (Boolean)
    #[serde(default)]
    pub is_takedown_immune: bool,
    /// `takedownStimulusRange` (Single)
    #[serde(default)]
    pub takedown_stimulus_range: f32,
    /// `QTEConfig_Knife` (Reference)
    #[serde(default)]
    pub qteconfig_knife: Option<CigGuid>,
    /// `QTEConfig_Unarmed` (Reference)
    #[serde(default)]
    pub qteconfig_unarmed: Option<CigGuid>,
    /// `QTEConfig_AI` (Reference)
    #[serde(default)]
    pub qteconfig_ai: Option<CigGuid>,
}

impl Pooled for TakeDownConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.closecombat.take_down_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.closecombat.take_down_config }
}

impl<'a> Extract<'a> for TakeDownConfig {
    const TYPE_NAME: &'static str = "TakeDownConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            take_down_params: inst.get_array("takeDownParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TakeDownParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TakeDownParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            is_takedown_immune: inst.get_bool("isTakedownImmune").unwrap_or_default(),
            takedown_stimulus_range: inst.get_f32("takedownStimulusRange").unwrap_or_default(),
            qteconfig_knife: inst.get("QTEConfig_Knife").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            qteconfig_unarmed: inst.get("QTEConfig_Unarmed").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            qteconfig_ai: inst.get("QTEConfig_AI").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

