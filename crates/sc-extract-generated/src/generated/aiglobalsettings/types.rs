// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `aiglobalsettings`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `AIFormulaScoreModifiers`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFormulaScoreModifiers {
    /// `exponent` (Single)
    #[serde(default)]
    pub exponent: f32,
    /// `weight` (Single)
    #[serde(default)]
    pub weight: f32,
}

impl Pooled for AIFormulaScoreModifiers {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiglobalsettings.aiformula_score_modifiers }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiglobalsettings.aiformula_score_modifiers }
}

impl<'a> Extract<'a> for AIFormulaScoreModifiers {
    const TYPE_NAME: &'static str = "AIFormulaScoreModifiers";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            exponent: inst.get_f32("exponent").unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

/// DCB type: `AITargetingFormulaSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITargetingFormulaSettings {
    /// `onFootRangeMultiplier` (Single)
    #[serde(default)]
    pub on_foot_range_multiplier: f32,
    /// `inVehicleRangeMultiplier` (Single)
    #[serde(default)]
    pub in_vehicle_range_multiplier: f32,
    /// `selfDefenceMaxHealthMultiplier` (Single)
    #[serde(default)]
    pub self_defence_max_health_multiplier: f32,
    /// `protectedMaxHealthMultiplier` (Single)
    #[serde(default)]
    pub protected_max_health_multiplier: f32,
    /// `recentDamageDecayFactorPerSecond` (Single)
    #[serde(default)]
    pub recent_damage_decay_factor_per_second: f32,
    /// `attackerCapacityScore` (Class)
    #[serde(default)]
    pub attacker_capacity_score: Option<Handle<AIFormulaScoreModifiers>>,
    /// `distanceScore` (Class)
    #[serde(default)]
    pub distance_score: Option<Handle<AIFormulaScoreModifiers>>,
    /// `selfDefenceScore` (Class)
    #[serde(default)]
    pub self_defence_score: Option<Handle<AIFormulaScoreModifiers>>,
    /// `protectionScore` (Class)
    #[serde(default)]
    pub protection_score: Option<Handle<AIFormulaScoreModifiers>>,
}

impl Pooled for AITargetingFormulaSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.aiglobalsettings.aitargeting_formula_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.aiglobalsettings.aitargeting_formula_settings }
}

impl<'a> Extract<'a> for AITargetingFormulaSettings {
    const TYPE_NAME: &'static str = "AITargetingFormulaSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            on_foot_range_multiplier: inst.get_f32("onFootRangeMultiplier").unwrap_or_default(),
            in_vehicle_range_multiplier: inst.get_f32("inVehicleRangeMultiplier").unwrap_or_default(),
            self_defence_max_health_multiplier: inst.get_f32("selfDefenceMaxHealthMultiplier").unwrap_or_default(),
            protected_max_health_multiplier: inst.get_f32("protectedMaxHealthMultiplier").unwrap_or_default(),
            recent_damage_decay_factor_per_second: inst.get_f32("recentDamageDecayFactorPerSecond").unwrap_or_default(),
            attacker_capacity_score: match inst.get("attackerCapacityScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance_score: match inst.get("distanceScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            self_defence_score: match inst.get("selfDefenceScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            protection_score: match inst.get("protectionScore") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<AIFormulaScoreModifiers>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

