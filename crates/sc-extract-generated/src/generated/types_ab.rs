// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `AbilityBreathAction`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityBreathAction {
    /// DCB field: `audioBreathInterrupt` (Reference)
    #[serde(default)]
    pub audio_breath_interrupt: Option<CigGuid>,
    /// DCB field: `interruptParam` (Single)
    #[serde(default)]
    pub interrupt_param: f32,
    /// DCB field: `resumeBreathingWhenAudioEnds` (Boolean)
    #[serde(default)]
    pub resume_breathing_when_audio_ends: bool,
    /// DCB field: `forceInhaleAfterResume` (Boolean)
    #[serde(default)]
    pub force_inhale_after_resume: bool,
    /// DCB field: `forceExhaleAfterResume` (Boolean)
    #[serde(default)]
    pub force_exhale_after_resume: bool,
}

impl Pooled for AbilityBreathAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ab.ability_breath_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ab.ability_breath_action }
}

impl<'a> Extract<'a> for AbilityBreathAction {
    const TYPE_NAME: &'static str = "AbilityBreathAction";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            audio_breath_interrupt: inst.get("audioBreathInterrupt").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            interrupt_param: inst.get_f32("interruptParam").unwrap_or_default(),
            resume_breathing_when_audio_ends: inst.get_bool("resumeBreathingWhenAudioEnds").unwrap_or_default(),
            force_inhale_after_resume: inst.get_bool("forceInhaleAfterResume").unwrap_or_default(),
            force_exhale_after_resume: inst.get_bool("forceExhaleAfterResume").unwrap_or_default(),
        }
    }
}

/// DCB type: `AbilityBreathingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityBreathingParams {
    /// DCB field: `breathActions` (Class)
    #[serde(default)]
    pub breath_actions: Option<Handle<AbilityBreathAction>>,
}

impl Pooled for AbilityBreathingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ab.ability_breathing_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ab.ability_breathing_params }
}

impl<'a> Extract<'a> for AbilityBreathingParams {
    const TYPE_NAME: &'static str = "AbilityBreathingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            breath_actions: match inst.get("breathActions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AbilityBreathAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AbilityBreathAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AbilityDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityDefinition {
    /// DCB field: `abilitiesToLock` (EnumChoice (array))
    #[serde(default)]
    pub abilities_to_lock: Vec<String>,
    /// DCB field: `abilitiesToInterrupt` (EnumChoice (array))
    #[serde(default)]
    pub abilities_to_interrupt: Vec<String>,
    /// DCB field: `breathingParams` (StrongPointer)
    #[serde(default)]
    pub breathing_params: Option<Handle<AbilityBreathingParams>>,
}

impl Pooled for AbilityDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ab.ability_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ab.ability_definition }
}

impl<'a> Extract<'a> for AbilityDefinition {
    const TYPE_NAME: &'static str = "AbilityDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            abilities_to_lock: inst.get_array("abilitiesToLock")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            abilities_to_interrupt: inst.get_array("abilitiesToInterrupt")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            breathing_params: match inst.get("breathingParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AbilityBreathingParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AbilityBreathingParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AbilityStaminaStates`
///
/// Inherits from: `ActorStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityStaminaStates {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// DCB field: `ability` (EnumChoice)
    #[serde(default)]
    pub ability: String,
    /// DCB field: `lastingStaminaCost` (StrongPointer)
    #[serde(default)]
    pub lasting_stamina_cost: Option<Handle<StaminaCost>>,
    /// DCB field: `instantStaminaCost` (StrongPointer)
    #[serde(default)]
    pub instant_stamina_cost: Option<Handle<StaminaCost>>,
    /// DCB field: `staminaToStart` (Single)
    #[serde(default)]
    pub stamina_to_start: f32,
    /// DCB field: `staminaToInterrupt` (Single)
    #[serde(default)]
    pub stamina_to_interrupt: f32,
    /// DCB field: `staminaUsageCap` (Single)
    #[serde(default)]
    pub stamina_usage_cap: f32,
    /// DCB field: `staminaRegenModifier` (Single)
    #[serde(default)]
    pub stamina_regen_modifier: f32,
}

impl Pooled for AbilityStaminaStates {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ab.ability_stamina_states }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ab.ability_stamina_states }
}

impl<'a> Extract<'a> for AbilityStaminaStates {
    const TYPE_NAME: &'static str = "AbilityStaminaStates";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
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
            ability: inst.get_str("ability").map(String::from).unwrap_or_default(),
            lasting_stamina_cost: match inst.get("lastingStaminaCost") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StaminaCost>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StaminaCost>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            instant_stamina_cost: match inst.get("instantStaminaCost") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StaminaCost>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StaminaCost>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            stamina_to_start: inst.get_f32("staminaToStart").unwrap_or_default(),
            stamina_to_interrupt: inst.get_f32("staminaToInterrupt").unwrap_or_default(),
            stamina_usage_cap: inst.get_f32("staminaUsageCap").unwrap_or_default(),
            stamina_regen_modifier: inst.get_f32("staminaRegenModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `AbilityStatusCosts`
///
/// Inherits from: `ActorStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityStatusCosts {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// DCB field: `ability` (EnumChoice)
    #[serde(default)]
    pub ability: String,
    /// DCB field: `lastingStatusCost` (Class (array))
    #[serde(default)]
    pub lasting_status_cost: Vec<Handle<StatusCost>>,
    /// DCB field: `instantStatusCost` (Class (array))
    #[serde(default)]
    pub instant_status_cost: Vec<Handle<StatusCost>>,
}

impl Pooled for AbilityStatusCosts {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ab.ability_status_costs }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ab.ability_status_costs }
}

impl<'a> Extract<'a> for AbilityStatusCosts {
    const TYPE_NAME: &'static str = "AbilityStatusCosts";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
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
            ability: inst.get_str("ability").map(String::from).unwrap_or_default(),
            lasting_status_cost: inst.get_array("lastingStatusCost")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StatusCost>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StatusCost>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            instant_status_cost: inst.get_array("instantStatusCost")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StatusCost>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StatusCost>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AbstractMissionInitParam`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractMissionInitParam {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
}

impl Pooled for AbstractMissionInitParam {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ab.abstract_mission_init_param }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ab.abstract_mission_init_param }
}

impl<'a> Extract<'a> for AbstractMissionInitParam {
    const TYPE_NAME: &'static str = "AbstractMissionInitParam";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

