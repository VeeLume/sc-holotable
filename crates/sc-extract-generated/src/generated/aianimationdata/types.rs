// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `aianimationdata`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `AttackCategoryParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackCategoryParamsBase {
    /// `damageInfo` (Class)
    #[serde(default)]
    pub damage_info: Option<Handle<DamageInfo>>,
}

impl Pooled for AttackCategoryParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aianimationdata.attack_category_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aianimationdata.attack_category_params_base }
}

impl<'a> Extract<'a> for AttackCategoryParamsBase {
    const TYPE_NAME: &'static str = "AttackCategoryParamsBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_info: match inst.get("damageInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DamageInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DamageInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MeleeComboChainLink`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeComboChainLink {
    /// `attackType` (EnumChoice)
    #[serde(default)]
    pub attack_type: String,
    /// `classType` (EnumChoice)
    #[serde(default)]
    pub class_type: String,
}

impl Pooled for MeleeComboChainLink {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aianimationdata.melee_combo_chain_link }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aianimationdata.melee_combo_chain_link }
}

impl<'a> Extract<'a> for MeleeComboChainLink {
    const TYPE_NAME: &'static str = "MeleeComboChainLink";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attack_type: inst.get_str("attackType").map(String::from).unwrap_or_default(),
            class_type: inst.get_str("classType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MeleeAttackCombo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeleeAttackCombo {
    /// `comboChain` (Class (array))
    #[serde(default)]
    pub combo_chain: Vec<Handle<MeleeComboChainLink>>,
    /// `cooldown` (Single)
    #[serde(default)]
    pub cooldown: f32,
}

impl Pooled for MeleeAttackCombo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aianimationdata.melee_attack_combo }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aianimationdata.melee_attack_combo }
}

impl<'a> Extract<'a> for MeleeAttackCombo {
    const TYPE_NAME: &'static str = "MeleeAttackCombo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            combo_chain: inst.get_array("comboChain")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeComboChainLink>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeComboChainLink>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            cooldown: inst.get_f32("cooldown").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMeleeAttack`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMeleeAttack {
    /// `attackType` (EnumChoice)
    #[serde(default)]
    pub attack_type: String,
    /// `globalFragmentTags` (String)
    #[serde(default)]
    pub global_fragment_tags: String,
    /// `minDistanceToTarget2d` (Single)
    #[serde(default)]
    pub min_distance_to_target2d: f32,
    /// `maxDistanceToTarget2d` (Single)
    #[serde(default)]
    pub max_distance_to_target2d: f32,
    /// `minRelativeTargetHeight` (Single)
    #[serde(default)]
    pub min_relative_target_height: f32,
    /// `maxRelativeTargetHeight` (Single)
    #[serde(default)]
    pub max_relative_target_height: f32,
    /// `maxAngleToTarget` (Single)
    #[serde(default)]
    pub max_angle_to_target: f32,
    /// `ignoreAttackObstructionClearance` (Boolean)
    #[serde(default)]
    pub ignore_attack_obstruction_clearance: bool,
}

impl Pooled for AIMeleeAttack {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aianimationdata.aimelee_attack }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aianimationdata.aimelee_attack }
}

impl<'a> Extract<'a> for AIMeleeAttack {
    const TYPE_NAME: &'static str = "AIMeleeAttack";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            attack_type: inst.get_str("attackType").map(String::from).unwrap_or_default(),
            global_fragment_tags: inst.get_str("globalFragmentTags").map(String::from).unwrap_or_default(),
            min_distance_to_target2d: inst.get_f32("minDistanceToTarget2d").unwrap_or_default(),
            max_distance_to_target2d: inst.get_f32("maxDistanceToTarget2d").unwrap_or_default(),
            min_relative_target_height: inst.get_f32("minRelativeTargetHeight").unwrap_or_default(),
            max_relative_target_height: inst.get_f32("maxRelativeTargetHeight").unwrap_or_default(),
            max_angle_to_target: inst.get_f32("maxAngleToTarget").unwrap_or_default(),
            ignore_attack_obstruction_clearance: inst.get_bool("ignoreAttackObstructionClearance").unwrap_or_default(),
        }
    }
}

/// DCB type: `AIMeleeCombatConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMeleeCombatConfig {
    /// `meleeAttackCategoryInfo` (Class (array))
    #[serde(default)]
    pub melee_attack_category_info: Vec<Handle<MeleeAttackCategoryInfo>>,
    /// `attackCategoryParams` (StrongPointer (array))
    #[serde(default)]
    pub attack_category_params: Vec<Handle<AttackCategoryParamsBase>>,
    /// `meleeAttacks` (StrongPointer (array))
    #[serde(default)]
    pub melee_attacks: Vec<Handle<AIMeleeAttack>>,
    /// `combos` (Class (array))
    #[serde(default)]
    pub combos: Vec<Handle<MeleeAttackCombo>>,
}

impl Pooled for AIMeleeCombatConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aianimationdata.aimelee_combat_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aianimationdata.aimelee_combat_config }
}

impl<'a> Extract<'a> for AIMeleeCombatConfig {
    const TYPE_NAME: &'static str = "AIMeleeCombatConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            melee_attack_category_info: inst.get_array("meleeAttackCategoryInfo")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeAttackCategoryInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeAttackCategoryInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            attack_category_params: inst.get_array("attackCategoryParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AttackCategoryParamsBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AttackCategoryParamsBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            melee_attacks: inst.get_array("meleeAttacks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AIMeleeAttack>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AIMeleeAttack>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            combos: inst.get_array("combos")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MeleeAttackCombo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MeleeAttackCombo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

