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

/// DCB type: `StatDefinitions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatDefinitions {
    /// DCB field: `stats` (Class (array))
    #[serde(default)]
    pub stats: Vec<Handle<Stat>>,
}

impl Pooled for StatDefinitions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stat_definitions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stat_definitions }
}

impl<'a> Extract<'a> for StatDefinitions {
    const TYPE_NAME: &'static str = "StatDefinitions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stats: inst.get_array("stats")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Stat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Stat>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Stat`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    /// DCB field: `statTag` (Reference)
    #[serde(default)]
    pub stat_tag: Option<CigGuid>,
    /// DCB field: `minimumValue` (Single)
    #[serde(default)]
    pub minimum_value: f32,
    /// DCB field: `influences` (Class (array))
    #[serde(default)]
    pub influences: Vec<Handle<StatInfluence>>,
}

impl Pooled for Stat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stat }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stat }
}

impl<'a> Extract<'a> for Stat {
    const TYPE_NAME: &'static str = "Stat";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stat_tag: inst.get("statTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            minimum_value: inst.get_f32("minimumValue").unwrap_or_default(),
            influences: inst.get_array("influences")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StatInfluence>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StatInfluence>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StatInfluence`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatInfluence {
    /// DCB field: `skillTag` (Reference)
    #[serde(default)]
    pub skill_tag: Option<CigGuid>,
    /// DCB field: `percentage` (Int32)
    #[serde(default)]
    pub percentage: i32,
}

impl Pooled for StatInfluence {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stat_influence }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stat_influence }
}

impl<'a> Extract<'a> for StatInfluence {
    const TYPE_NAME: &'static str = "StatInfluence";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            skill_tag: inst.get("skillTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            percentage: inst.get_i32("percentage").unwrap_or_default(),
        }
    }
}

/// DCB type: `StaminaCost`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaminaCost {
    /// DCB field: `flatCost` (Single)
    #[serde(default)]
    pub flat_cost: f32,
}

impl Pooled for StaminaCost {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stamina_cost }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stamina_cost }
}

impl<'a> Extract<'a> for StaminaCost {
    const TYPE_NAME: &'static str = "StaminaCost";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            flat_cost: inst.get_f32("flatCost").unwrap_or_default(),
        }
    }
}

/// DCB type: `StaminaCostParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaminaCostParams {
    /// DCB field: `staminaRegenerationBase` (Single)
    #[serde(default)]
    pub stamina_regeneration_base: f32,
    /// DCB field: `staminaRegenerationBonus` (Single)
    #[serde(default)]
    pub stamina_regeneration_bonus: f32,
    /// DCB field: `staminaForMaxRegenBonus` (Single)
    #[serde(default)]
    pub stamina_for_max_regen_bonus: f32,
    /// DCB field: `staminaCostScale` (Single)
    #[serde(default)]
    pub stamina_cost_scale: f32,
    /// DCB field: `staminaRegenScale` (Single)
    #[serde(default)]
    pub stamina_regen_scale: f32,
    /// DCB field: `staminaSlopeCostScale` (Single)
    #[serde(default)]
    pub stamina_slope_cost_scale: f32,
    /// DCB field: `costPerWeight` (Single)
    #[serde(default)]
    pub cost_per_weight: f32,
    /// DCB field: `abilityStaminaStates` (Class (array))
    #[serde(default)]
    pub ability_stamina_states: Vec<Handle<AbilityStaminaStates>>,
    /// DCB field: `actionStaminaCosts` (Class (array))
    #[serde(default)]
    pub action_stamina_costs: Vec<Handle<ActionStaminaCosts>>,
}

impl Pooled for StaminaCostParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stamina_cost_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stamina_cost_params }
}

impl<'a> Extract<'a> for StaminaCostParams {
    const TYPE_NAME: &'static str = "StaminaCostParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stamina_regeneration_base: inst.get_f32("staminaRegenerationBase").unwrap_or_default(),
            stamina_regeneration_bonus: inst.get_f32("staminaRegenerationBonus").unwrap_or_default(),
            stamina_for_max_regen_bonus: inst.get_f32("staminaForMaxRegenBonus").unwrap_or_default(),
            stamina_cost_scale: inst.get_f32("staminaCostScale").unwrap_or_default(),
            stamina_regen_scale: inst.get_f32("staminaRegenScale").unwrap_or_default(),
            stamina_slope_cost_scale: inst.get_f32("staminaSlopeCostScale").unwrap_or_default(),
            cost_per_weight: inst.get_f32("costPerWeight").unwrap_or_default(),
            ability_stamina_states: inst.get_array("abilityStaminaStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AbilityStaminaStates>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AbilityStaminaStates>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            action_stamina_costs: inst.get_array("actionStaminaCosts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActionStaminaCosts>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActionStaminaCosts>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StatusEffectBuffMacro`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectBuffMacro {
    /// DCB field: `threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
    /// DCB field: `buffEffects` (Class (array))
    #[serde(default)]
    pub buff_effects: Vec<Handle<ActorStatusAddBuff>>,
}

impl Pooled for StatusEffectBuffMacro {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_effect_buff_macro }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_effect_buff_macro }
}

impl<'a> Extract<'a> for StatusEffectBuffMacro {
    const TYPE_NAME: &'static str = "StatusEffectBuffMacro";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            threshold: inst.get_f32("threshold").unwrap_or_default(),
            buff_effects: inst.get_array("buffEffects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorStatusAddBuff>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorStatusAddBuff>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StatusEffectValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectValue {
}

impl Pooled for StatusEffectValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_effect_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_effect_value }
}

impl<'a> Extract<'a> for StatusEffectValue {
    const TYPE_NAME: &'static str = "StatusEffectValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `StatusEffectTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectTrigger {
    /// DCB field: `statusEffectType` (EnumChoice)
    #[serde(default)]
    pub status_effect_type: String,
    /// DCB field: `triggerMinThreshold` (Single)
    #[serde(default)]
    pub trigger_min_threshold: f32,
    /// DCB field: `triggerMaxThreshold` (Single)
    #[serde(default)]
    pub trigger_max_threshold: f32,
    /// DCB field: `value` (StrongPointer)
    #[serde(default)]
    pub value: Option<Handle<StatusEffectValue>>,
}

impl Pooled for StatusEffectTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_effect_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_effect_trigger }
}

impl<'a> Extract<'a> for StatusEffectTrigger {
    const TYPE_NAME: &'static str = "StatusEffectTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            status_effect_type: inst.get_str("statusEffectType").map(String::from).unwrap_or_default(),
            trigger_min_threshold: inst.get_f32("triggerMinThreshold").unwrap_or_default(),
            trigger_max_threshold: inst.get_f32("triggerMaxThreshold").unwrap_or_default(),
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusEffectValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusEffectValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StatusTriggerBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusTriggerBase {
    /// DCB field: `fortitudeLevelModifier` (StrongPointer)
    #[serde(default)]
    pub fortitude_level_modifier: Option<Handle<SStatusFortitudeLevelModifier>>,
}

impl Pooled for StatusTriggerBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_trigger_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_trigger_base }
}

impl<'a> Extract<'a> for StatusTriggerBase {
    const TYPE_NAME: &'static str = "StatusTriggerBase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fortitude_level_modifier: match inst.get("fortitudeLevelModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SStatusFortitudeLevelModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SStatusFortitudeLevelModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StatusEffectSetupBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectSetupBase {
}

impl Pooled for StatusEffectSetupBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_effect_setup_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_effect_setup_base }
}

impl<'a> Extract<'a> for StatusEffectSetupBase {
    const TYPE_NAME: &'static str = "StatusEffectSetupBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `StatusEffectSetup`
///
/// Inherits from: `StatusEffectSetupBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectSetup {
    /// DCB field: `actorStatuses` (StrongPointer (array))
    #[serde(default)]
    pub actor_statuses: Vec<Handle<ActorStatusData>>,
}

impl Pooled for StatusEffectSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_effect_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_effect_setup }
}

impl<'a> Extract<'a> for StatusEffectSetup {
    const TYPE_NAME: &'static str = "StatusEffectSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actor_statuses: inst.get_array("actorStatuses")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActorStatusData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActorStatusData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StatusMaskedRetriggerSetupBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMaskedRetriggerSetupBase {
}

impl Pooled for StatusMaskedRetriggerSetupBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_masked_retrigger_setup_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_masked_retrigger_setup_base }
}

impl<'a> Extract<'a> for StatusMaskedRetriggerSetupBase {
    const TYPE_NAME: &'static str = "StatusMaskedRetriggerSetupBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `StatusMaskedRetriggerSetup`
///
/// Inherits from: `StatusMaskedRetriggerSetupBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMaskedRetriggerSetup {
    /// DCB field: `statusType` (EnumChoice)
    #[serde(default)]
    pub status_type: String,
    /// DCB field: `statusTrigger` (StrongPointer)
    #[serde(default)]
    pub status_trigger: Option<Handle<StatusTriggerBase>>,
}

impl Pooled for StatusMaskedRetriggerSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_masked_retrigger_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_masked_retrigger_setup }
}

impl<'a> Extract<'a> for StatusMaskedRetriggerSetup {
    const TYPE_NAME: &'static str = "StatusMaskedRetriggerSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            status_type: inst.get_str("statusType").map(String::from).unwrap_or_default(),
            status_trigger: match inst.get("statusTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusTriggerBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusTriggerBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StatusMaskedRetriggerPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMaskedRetriggerPreset {
    /// DCB field: `retriggerSetup` (StrongPointer)
    #[serde(default)]
    pub retrigger_setup: Option<Handle<StatusMaskedRetriggerSetup>>,
}

impl Pooled for StatusMaskedRetriggerPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_masked_retrigger_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_masked_retrigger_preset }
}

impl<'a> Extract<'a> for StatusMaskedRetriggerPreset {
    const TYPE_NAME: &'static str = "StatusMaskedRetriggerPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            retrigger_setup: match inst.get("retriggerSetup") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StatusMaskedRetriggerSetup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StatusMaskedRetriggerSetup>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StatusCost`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCost {
    /// DCB field: `statType` (EnumChoice)
    #[serde(default)]
    pub stat_type: String,
    /// DCB field: `cost` (Single)
    #[serde(default)]
    pub cost: f32,
}

impl Pooled for StatusCost {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_cost }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_cost }
}

impl<'a> Extract<'a> for StatusCost {
    const TYPE_NAME: &'static str = "StatusCost";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            stat_type: inst.get_str("statType").map(String::from).unwrap_or_default(),
            cost: inst.get_f32("cost").unwrap_or_default(),
        }
    }
}

/// DCB type: `StatusBuffTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBuffTypeBase {
}

impl Pooled for StatusBuffTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_buff_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_buff_type_base }
}

impl<'a> Extract<'a> for StatusBuffTypeBase {
    const TYPE_NAME: &'static str = "StatusBuffTypeBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `StatusEffectAbilityLock`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffectAbilityLock {
    /// DCB field: `statusEffectType` (EnumChoice)
    #[serde(default)]
    pub status_effect_type: String,
    /// DCB field: `abilitiesToLock` (EnumChoice (array))
    #[serde(default)]
    pub abilities_to_lock: Vec<String>,
}

impl Pooled for StatusEffectAbilityLock {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_effect_ability_lock }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_effect_ability_lock }
}

impl<'a> Extract<'a> for StatusEffectAbilityLock {
    const TYPE_NAME: &'static str = "StatusEffectAbilityLock";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            status_effect_type: inst.get_str("statusEffectType").map(String::from).unwrap_or_default(),
            abilities_to_lock: inst.get_array("abilitiesToLock")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StatusSweatingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusSweatingParams {
    /// DCB field: `sweatingStaminaThreshold` (Single)
    #[serde(default)]
    pub sweating_stamina_threshold: f32,
    /// DCB field: `sweatMinAccumulationPerSecond` (Single)
    #[serde(default)]
    pub sweat_min_accumulation_per_second: f32,
    /// DCB field: `sweatMaxAccumulationPerSecond` (Single)
    #[serde(default)]
    pub sweat_max_accumulation_per_second: f32,
    /// DCB field: `sweatDecayDelay` (Single)
    #[serde(default)]
    pub sweat_decay_delay: f32,
    /// DCB field: `sweatDecayRate` (Single)
    #[serde(default)]
    pub sweat_decay_rate: f32,
    /// DCB field: `sweatToHygieneMinDecayMultiplier` (Single)
    #[serde(default)]
    pub sweat_to_hygiene_min_decay_multiplier: f32,
    /// DCB field: `sweatToHygieneMaxDecayMultiplier` (Single)
    #[serde(default)]
    pub sweat_to_hygiene_max_decay_multiplier: f32,
}

impl Pooled for StatusSweatingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_sweating_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_sweating_params }
}

impl<'a> Extract<'a> for StatusSweatingParams {
    const TYPE_NAME: &'static str = "StatusSweatingParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sweating_stamina_threshold: inst.get_f32("sweatingStaminaThreshold").unwrap_or_default(),
            sweat_min_accumulation_per_second: inst.get_f32("sweatMinAccumulationPerSecond").unwrap_or_default(),
            sweat_max_accumulation_per_second: inst.get_f32("sweatMaxAccumulationPerSecond").unwrap_or_default(),
            sweat_decay_delay: inst.get_f32("sweatDecayDelay").unwrap_or_default(),
            sweat_decay_rate: inst.get_f32("sweatDecayRate").unwrap_or_default(),
            sweat_to_hygiene_min_decay_multiplier: inst.get_f32("sweatToHygieneMinDecayMultiplier").unwrap_or_default(),
            sweat_to_hygiene_max_decay_multiplier: inst.get_f32("sweatToHygieneMaxDecayMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `StatusBloodParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusBloodParams {
    /// DCB field: `orificeBloodParams` (Class (array))
    #[serde(default)]
    pub orifice_blood_params: Vec<Handle<OrificeBloodParams>>,
    /// DCB field: `globalMinBloodInterval` (Single)
    #[serde(default)]
    pub global_min_blood_interval: f32,
    /// DCB field: `bloodTimeToDryAfterDeath` (Single)
    #[serde(default)]
    pub blood_time_to_dry_after_death: f32,
    /// DCB field: `bloodToHygieneDecayMultiplier` (Single)
    #[serde(default)]
    pub blood_to_hygiene_decay_multiplier: f32,
}

impl Pooled for StatusBloodParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_blood_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_blood_params }
}

impl<'a> Extract<'a> for StatusBloodParams {
    const TYPE_NAME: &'static str = "StatusBloodParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            orifice_blood_params: inst.get_array("orificeBloodParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<OrificeBloodParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<OrificeBloodParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_min_blood_interval: inst.get_f32("globalMinBloodInterval").unwrap_or_default(),
            blood_time_to_dry_after_death: inst.get_f32("bloodTimeToDryAfterDeath").unwrap_or_default(),
            blood_to_hygiene_decay_multiplier: inst.get_f32("bloodToHygieneDecayMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `StanceBreathModifier`
///
/// Inherits from: `ActorStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StanceBreathModifier {
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
    /// DCB field: `modifier` (Single)
    #[serde(default)]
    pub modifier: f32,
}

impl Pooled for StanceBreathModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stance_breath_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stance_breath_modifier }
}

impl<'a> Extract<'a> for StanceBreathModifier {
    const TYPE_NAME: &'static str = "StanceBreathModifier";
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
            modifier: inst.get_f32("modifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `STargetingMethodBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetingMethodBase {
}

impl Pooled for STargetingMethodBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stargeting_method_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stargeting_method_base }
}

impl<'a> Extract<'a> for STargetingMethodBase {
    const TYPE_NAME: &'static str = "STargetingMethodBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `STargetingMethodRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetingMethodRecord {
    /// DCB field: `targetingMethod` (StrongPointer)
    #[serde(default)]
    pub targeting_method: Option<Handle<STargetingMethodBase>>,
}

impl Pooled for STargetingMethodRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stargeting_method_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stargeting_method_record }
}

impl<'a> Extract<'a> for STargetingMethodRecord {
    const TYPE_NAME: &'static str = "STargetingMethodRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            targeting_method: match inst.get("targetingMethod") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetingMethodBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STargetingMethodBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `STargetableItemType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetableItemType {
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `subItemTypes` (EnumChoice (array))
    #[serde(default)]
    pub sub_item_types: Vec<String>,
}

impl Pooled for STargetableItemType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stargetable_item_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stargetable_item_type }
}

impl<'a> Extract<'a> for STargetableItemType {
    const TYPE_NAME: &'static str = "STargetableItemType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            sub_item_types: inst.get_array("subItemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `STargetableItemTypesRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetableItemTypesRecord {
    /// DCB field: `targetableItemTypes` (Class (array))
    #[serde(default)]
    pub targetable_item_types: Vec<Handle<STargetableItemType>>,
    /// DCB field: `precisionTargetingNames` (Class (array))
    #[serde(default)]
    pub precision_targeting_names: Vec<Handle<SPrecisionTargetingItemName>>,
}

impl Pooled for STargetableItemTypesRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stargetable_item_types_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stargetable_item_types_record }
}

impl<'a> Extract<'a> for STargetableItemTypesRecord {
    const TYPE_NAME: &'static str = "STargetableItemTypesRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            targetable_item_types: inst.get_array("targetableItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<STargetableItemType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<STargetableItemType>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            precision_targeting_names: inst.get_array("precisionTargetingNames")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SPrecisionTargetingItemName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SPrecisionTargetingItemName>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `STargetSelectorColorHighlighting`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetSelectorColorHighlighting {
    /// DCB field: `highlightColor` (Class)
    #[serde(default)]
    pub highlight_color: Option<Handle<RGB>>,
    /// DCB field: `occludedAlpha` (Single)
    #[serde(default)]
    pub occluded_alpha: f32,
    /// DCB field: `outlineOnly` (Boolean)
    #[serde(default)]
    pub outline_only: bool,
    /// DCB field: `outlineWidth` (Single)
    #[serde(default)]
    pub outline_width: f32,
    /// DCB field: `interferenceAmount` (Single)
    #[serde(default)]
    pub interference_amount: f32,
    /// DCB field: `interferenceSpeed` (Single)
    #[serde(default)]
    pub interference_speed: f32,
    /// DCB field: `interferenceTiling` (Single)
    #[serde(default)]
    pub interference_tiling: f32,
    /// DCB field: `interferenceBrightness` (Single)
    #[serde(default)]
    pub interference_brightness: f32,
    /// DCB field: `useHostilityColor` (Boolean)
    #[serde(default)]
    pub use_hostility_color: bool,
}

impl Pooled for STargetSelectorColorHighlighting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.starget_selector_color_highlighting }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.starget_selector_color_highlighting }
}

impl<'a> Extract<'a> for STargetSelectorColorHighlighting {
    const TYPE_NAME: &'static str = "STargetSelectorColorHighlighting";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            highlight_color: match inst.get("highlightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            occluded_alpha: inst.get_f32("occludedAlpha").unwrap_or_default(),
            outline_only: inst.get_bool("outlineOnly").unwrap_or_default(),
            outline_width: inst.get_f32("outlineWidth").unwrap_or_default(),
            interference_amount: inst.get_f32("interferenceAmount").unwrap_or_default(),
            interference_speed: inst.get_f32("interferenceSpeed").unwrap_or_default(),
            interference_tiling: inst.get_f32("interferenceTiling").unwrap_or_default(),
            interference_brightness: inst.get_f32("interferenceBrightness").unwrap_or_default(),
            use_hostility_color: inst.get_bool("useHostilityColor").unwrap_or_default(),
        }
    }
}

/// DCB type: `STargetSelectorHudParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetSelectorHudParams {
    /// DCB field: `calculateLockedTargetBracket` (Boolean)
    #[serde(default)]
    pub calculate_locked_target_bracket: bool,
    /// DCB field: `calculateSelectedTargetBracket` (Boolean)
    #[serde(default)]
    pub calculate_selected_target_bracket: bool,
    /// DCB field: `hudTargetPointerAngleOffset` (Single)
    #[serde(default)]
    pub hud_target_pointer_angle_offset: f32,
    /// DCB field: `hudTargetPointerHeadFollowAngleInner` (Single)
    #[serde(default)]
    pub hud_target_pointer_head_follow_angle_inner: f32,
    /// DCB field: `hudTargetPointerHeadFollowAngleOuter` (Single)
    #[serde(default)]
    pub hud_target_pointer_head_follow_angle_outer: f32,
    /// DCB field: `hudTargetPointerHeadFollowSwapTime` (Single)
    #[serde(default)]
    pub hud_target_pointer_head_follow_swap_time: f32,
    /// DCB field: `relativeAttitudePointerPosition` (Single)
    #[serde(default)]
    pub relative_attitude_pointer_position: f32,
    /// DCB field: `showAllSubtargets` (Boolean)
    #[serde(default)]
    pub show_all_subtargets: bool,
    /// DCB field: `targetPointerAlpha` (Class)
    #[serde(default)]
    pub target_pointer_alpha: Option<Handle<BezierCurve>>,
    /// DCB field: `outlineSubtargetsLocked` (Class)
    #[serde(default)]
    pub outline_subtargets_locked: Option<Handle<STargetSelectorColorHighlighting>>,
    /// DCB field: `outlineSubtargetsAvailable` (Class)
    #[serde(default)]
    pub outline_subtargets_available: Option<Handle<STargetSelectorColorHighlighting>>,
    /// DCB field: `outlineSubtargetsObjective` (Class)
    #[serde(default)]
    pub outline_subtargets_objective: Option<Handle<STargetSelectorColorHighlighting>>,
    /// DCB field: `outlineSubtargetsObjectiveLocked` (Class)
    #[serde(default)]
    pub outline_subtargets_objective_locked: Option<Handle<STargetSelectorColorHighlighting>>,
}

impl Pooled for STargetSelectorHudParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.starget_selector_hud_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.starget_selector_hud_params }
}

impl<'a> Extract<'a> for STargetSelectorHudParams {
    const TYPE_NAME: &'static str = "STargetSelectorHudParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            calculate_locked_target_bracket: inst.get_bool("calculateLockedTargetBracket").unwrap_or_default(),
            calculate_selected_target_bracket: inst.get_bool("calculateSelectedTargetBracket").unwrap_or_default(),
            hud_target_pointer_angle_offset: inst.get_f32("hudTargetPointerAngleOffset").unwrap_or_default(),
            hud_target_pointer_head_follow_angle_inner: inst.get_f32("hudTargetPointerHeadFollowAngleInner").unwrap_or_default(),
            hud_target_pointer_head_follow_angle_outer: inst.get_f32("hudTargetPointerHeadFollowAngleOuter").unwrap_or_default(),
            hud_target_pointer_head_follow_swap_time: inst.get_f32("hudTargetPointerHeadFollowSwapTime").unwrap_or_default(),
            relative_attitude_pointer_position: inst.get_f32("relativeAttitudePointerPosition").unwrap_or_default(),
            show_all_subtargets: inst.get_bool("showAllSubtargets").unwrap_or_default(),
            target_pointer_alpha: match inst.get("targetPointerAlpha") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outline_subtargets_locked: match inst.get("outlineSubtargetsLocked") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outline_subtargets_available: match inst.get("outlineSubtargetsAvailable") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outline_subtargets_objective: match inst.get("outlineSubtargetsObjective") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            outline_subtargets_objective_locked: match inst.get("outlineSubtargetsObjectiveLocked") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<STargetSelectorColorHighlighting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `STargetSelectorGlobalTargetingParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STargetSelectorGlobalTargetingParams {
    /// DCB field: `combatTargetingMethodRecord` (Class)
    #[serde(default)]
    pub combat_targeting_method_record: Option<Handle<SCombatTargeting>>,
    /// DCB field: `scanningTargetingMethodRecord` (Class)
    #[serde(default)]
    pub scanning_targeting_method_record: Option<Handle<SScanTargeting>>,
    /// DCB field: `miningTargetingMethodRecord` (Class)
    #[serde(default)]
    pub mining_targeting_method_record: Option<Handle<SMiningTargeting>>,
}

impl Pooled for STargetSelectorGlobalTargetingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.starget_selector_global_targeting_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.starget_selector_global_targeting_params }
}

impl<'a> Extract<'a> for STargetSelectorGlobalTargetingParams {
    const TYPE_NAME: &'static str = "STargetSelectorGlobalTargetingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            combat_targeting_method_record: match inst.get("combatTargetingMethodRecord") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCombatTargeting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCombatTargeting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scanning_targeting_method_record: match inst.get("scanningTargetingMethodRecord") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SScanTargeting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SScanTargeting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            mining_targeting_method_record: match inst.get("miningTargetingMethodRecord") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMiningTargeting>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMiningTargeting>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StickyFilterMovementParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyFilterMovementParams {
    /// DCB field: `matchNPCSpeed` (Boolean)
    #[serde(default)]
    pub match_npcspeed: bool,
    /// DCB field: `constantFOV` (Boolean)
    #[serde(default)]
    pub constant_fov: bool,
    /// DCB field: `approachingOuterSpeed` (Single)
    #[serde(default)]
    pub approaching_outer_speed: f32,
    /// DCB field: `approachingInnerSpeed` (Single)
    #[serde(default)]
    pub approaching_inner_speed: f32,
    /// DCB field: `retreatOuterSpeed` (Single)
    #[serde(default)]
    pub retreat_outer_speed: f32,
    /// DCB field: `retreatIntermediateSpeed` (Single)
    #[serde(default)]
    pub retreat_intermediate_speed: f32,
    /// DCB field: `innerRadius` (Single)
    #[serde(default)]
    pub inner_radius: f32,
    /// DCB field: `intermediateRadius` (Single)
    #[serde(default)]
    pub intermediate_radius: f32,
    /// DCB field: `outerRadius` (Single)
    #[serde(default)]
    pub outer_radius: f32,
    /// DCB field: `breakRadius` (Single)
    #[serde(default)]
    pub break_radius: f32,
    /// DCB field: `nudgeFraction` (Single)
    #[serde(default)]
    pub nudge_fraction: f32,
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// DCB field: `lockOffsetRotation` (Boolean)
    #[serde(default)]
    pub lock_offset_rotation: bool,
    /// DCB field: `lerpTimeToFullSpeedOnStop` (Single)
    #[serde(default)]
    pub lerp_time_to_full_speed_on_stop: f32,
    /// DCB field: `minMovementThreshold` (Single)
    #[serde(default)]
    pub min_movement_threshold: f32,
}

impl Pooled for StickyFilterMovementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.sticky_filter_movement_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.sticky_filter_movement_params }
}

impl<'a> Extract<'a> for StickyFilterMovementParams {
    const TYPE_NAME: &'static str = "StickyFilterMovementParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            match_npcspeed: inst.get_bool("matchNPCSpeed").unwrap_or_default(),
            constant_fov: inst.get_bool("constantFOV").unwrap_or_default(),
            approaching_outer_speed: inst.get_f32("approachingOuterSpeed").unwrap_or_default(),
            approaching_inner_speed: inst.get_f32("approachingInnerSpeed").unwrap_or_default(),
            retreat_outer_speed: inst.get_f32("retreatOuterSpeed").unwrap_or_default(),
            retreat_intermediate_speed: inst.get_f32("retreatIntermediateSpeed").unwrap_or_default(),
            inner_radius: inst.get_f32("innerRadius").unwrap_or_default(),
            intermediate_radius: inst.get_f32("intermediateRadius").unwrap_or_default(),
            outer_radius: inst.get_f32("outerRadius").unwrap_or_default(),
            break_radius: inst.get_f32("breakRadius").unwrap_or_default(),
            nudge_fraction: inst.get_f32("nudgeFraction").unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            lock_offset_rotation: inst.get_bool("lockOffsetRotation").unwrap_or_default(),
            lerp_time_to_full_speed_on_stop: inst.get_f32("lerpTimeToFullSpeedOnStop").unwrap_or_default(),
            min_movement_threshold: inst.get_f32("minMovementThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `StickyFilterRotationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyFilterRotationParams {
    /// DCB field: `outerRotationSpeed` (Single)
    #[serde(default)]
    pub outer_rotation_speed: f32,
    /// DCB field: `innerRotationSpeed` (Single)
    #[serde(default)]
    pub inner_rotation_speed: f32,
    /// DCB field: `minRadiusAngle` (Single)
    #[serde(default)]
    pub min_radius_angle: f32,
    /// DCB field: `maxRadiusAngle` (Single)
    #[serde(default)]
    pub max_radius_angle: f32,
    /// DCB field: `breakRadiusAngle` (Single)
    #[serde(default)]
    pub break_radius_angle: f32,
    /// DCB field: `deadZoneRadiusAngle` (Single)
    #[serde(default)]
    pub dead_zone_radius_angle: f32,
}

impl Pooled for StickyFilterRotationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.sticky_filter_rotation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.sticky_filter_rotation_params }
}

impl<'a> Extract<'a> for StickyFilterRotationParams {
    const TYPE_NAME: &'static str = "StickyFilterRotationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            outer_rotation_speed: inst.get_f32("outerRotationSpeed").unwrap_or_default(),
            inner_rotation_speed: inst.get_f32("innerRotationSpeed").unwrap_or_default(),
            min_radius_angle: inst.get_f32("minRadiusAngle").unwrap_or_default(),
            max_radius_angle: inst.get_f32("maxRadiusAngle").unwrap_or_default(),
            break_radius_angle: inst.get_f32("breakRadiusAngle").unwrap_or_default(),
            dead_zone_radius_angle: inst.get_f32("deadZoneRadiusAngle").unwrap_or_default(),
        }
    }
}

/// DCB type: `StickyFilterAutocenterParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyFilterAutocenterParams {
    /// DCB field: `idleTimeBeforeRecenter` (Single)
    #[serde(default)]
    pub idle_time_before_recenter: f32,
    /// DCB field: `timeRecenterAtMinAngle` (Single)
    #[serde(default)]
    pub time_recenter_at_min_angle: f32,
    /// DCB field: `timeRecenterAtMaxAngle` (Single)
    #[serde(default)]
    pub time_recenter_at_max_angle: f32,
    /// DCB field: `timeRecenterAtMinAngleMoving` (Single)
    #[serde(default)]
    pub time_recenter_at_min_angle_moving: f32,
    /// DCB field: `timeRecenterAtMaxAngleMoving` (Single)
    #[serde(default)]
    pub time_recenter_at_max_angle_moving: f32,
    /// DCB field: `eyeOffsetAtMinDistance` (Class)
    #[serde(default)]
    pub eye_offset_at_min_distance: Option<Handle<Vec3>>,
    /// DCB field: `eyeOffsetAtMaxDistance` (Class)
    #[serde(default)]
    pub eye_offset_at_max_distance: Option<Handle<Vec3>>,
}

impl Pooled for StickyFilterAutocenterParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.sticky_filter_autocenter_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.sticky_filter_autocenter_params }
}

impl<'a> Extract<'a> for StickyFilterAutocenterParams {
    const TYPE_NAME: &'static str = "StickyFilterAutocenterParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            idle_time_before_recenter: inst.get_f32("idleTimeBeforeRecenter").unwrap_or_default(),
            time_recenter_at_min_angle: inst.get_f32("timeRecenterAtMinAngle").unwrap_or_default(),
            time_recenter_at_max_angle: inst.get_f32("timeRecenterAtMaxAngle").unwrap_or_default(),
            time_recenter_at_min_angle_moving: inst.get_f32("timeRecenterAtMinAngleMoving").unwrap_or_default(),
            time_recenter_at_max_angle_moving: inst.get_f32("timeRecenterAtMaxAngleMoving").unwrap_or_default(),
            eye_offset_at_min_distance: match inst.get("eyeOffsetAtMinDistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            eye_offset_at_max_distance: match inst.get("eyeOffsetAtMaxDistance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `STeamScoring`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STeamScoring {
    /// DCB field: `startTeamScore` (Int32)
    #[serde(default)]
    pub start_team_score: i32,
    /// DCB field: `useScoreAsTime` (Boolean)
    #[serde(default)]
    pub use_score_as_time: bool,
    /// DCB field: `teamScoreValue` (Int32)
    #[serde(default)]
    pub team_score_value: i32,
    /// DCB field: `playerToTeamScoring` (EnumChoice (array))
    #[serde(default)]
    pub player_to_team_scoring: Vec<String>,
    /// DCB field: `teamScoreEvents` (Class (array))
    #[serde(default)]
    pub team_score_events: Vec<Handle<SScoreEvent>>,
}

impl Pooled for STeamScoring {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.steam_scoring }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.steam_scoring }
}

impl<'a> Extract<'a> for STeamScoring {
    const TYPE_NAME: &'static str = "STeamScoring";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_team_score: inst.get_i32("startTeamScore").unwrap_or_default(),
            use_score_as_time: inst.get_bool("useScoreAsTime").unwrap_or_default(),
            team_score_value: inst.get_i32("teamScoreValue").unwrap_or_default(),
            player_to_team_scoring: inst.get_array("playerToTeamScoring")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            team_score_events: inst.get_array("teamScoreEvents")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SScoreEvent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SScoreEvent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `STractorBeamHoloVisualParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STractorBeamHoloVisualParams {
    /// DCB field: `minAlignmentValidHolo` (Single)
    #[serde(default)]
    pub min_alignment_valid_holo: f32,
    /// DCB field: `validHoloColor` (Class)
    #[serde(default)]
    pub valid_holo_color: Option<Handle<RGB>>,
    /// DCB field: `warningHoloColor` (Class)
    #[serde(default)]
    pub warning_holo_color: Option<Handle<RGB>>,
    /// DCB field: `invalidHoloColor` (Class)
    #[serde(default)]
    pub invalid_holo_color: Option<Handle<RGB>>,
    /// DCB field: `cargoHoloPreviewColor` (Class)
    #[serde(default)]
    pub cargo_holo_preview_color: Option<Handle<RGBA>>,
    /// DCB field: `holoOpacity` (Single)
    #[serde(default)]
    pub holo_opacity: f32,
    /// DCB field: `holoMaterial` (Class)
    #[serde(default)]
    pub holo_material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for STractorBeamHoloVisualParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stractor_beam_holo_visual_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stractor_beam_holo_visual_params }
}

impl<'a> Extract<'a> for STractorBeamHoloVisualParams {
    const TYPE_NAME: &'static str = "STractorBeamHoloVisualParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_alignment_valid_holo: inst.get_f32("minAlignmentValidHolo").unwrap_or_default(),
            valid_holo_color: match inst.get("validHoloColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            warning_holo_color: match inst.get("warningHoloColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invalid_holo_color: match inst.get("invalidHoloColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cargo_holo_preview_color: match inst.get("cargoHoloPreviewColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            holo_opacity: inst.get_f32("holoOpacity").unwrap_or_default(),
            holo_material: match inst.get("holoMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `STractorBeamOutlineVisualParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STractorBeamOutlineVisualParams {
    /// DCB field: `minOutlineWidth` (Single)
    #[serde(default)]
    pub min_outline_width: f32,
    /// DCB field: `maxOutlineWidth` (Single)
    #[serde(default)]
    pub max_outline_width: f32,
    /// DCB field: `outlineOpacity` (Single)
    #[serde(default)]
    pub outline_opacity: f32,
}

impl Pooled for STractorBeamOutlineVisualParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stractor_beam_outline_visual_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stractor_beam_outline_visual_params }
}

impl<'a> Extract<'a> for STractorBeamOutlineVisualParams {
    const TYPE_NAME: &'static str = "STractorBeamOutlineVisualParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_outline_width: inst.get_f32("minOutlineWidth").unwrap_or_default(),
            max_outline_width: inst.get_f32("maxOutlineWidth").unwrap_or_default(),
            outline_opacity: inst.get_f32("outlineOpacity").unwrap_or_default(),
        }
    }
}

/// DCB type: `StanceCheckConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StanceCheckConfig {
    /// DCB field: `standCheckEnabled` (Boolean)
    #[serde(default)]
    pub stand_check_enabled: bool,
    /// DCB field: `crouchCheckEnabled` (Boolean)
    #[serde(default)]
    pub crouch_check_enabled: bool,
    /// DCB field: `proneCheckEnabled` (Boolean)
    #[serde(default)]
    pub prone_check_enabled: bool,
    /// DCB field: `ragdollEnabled` (Boolean)
    #[serde(default)]
    pub ragdoll_enabled: bool,
}

impl Pooled for StanceCheckConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stance_check_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stance_check_config }
}

impl<'a> Extract<'a> for StanceCheckConfig {
    const TYPE_NAME: &'static str = "StanceCheckConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            stand_check_enabled: inst.get_bool("standCheckEnabled").unwrap_or_default(),
            crouch_check_enabled: inst.get_bool("crouchCheckEnabled").unwrap_or_default(),
            prone_check_enabled: inst.get_bool("proneCheckEnabled").unwrap_or_default(),
            ragdoll_enabled: inst.get_bool("ragdollEnabled").unwrap_or_default(),
        }
    }
}

/// DCB type: `STurretHealthModifierDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STurretHealthModifierDef {
    /// DCB field: `damageMovementModifier` (Class)
    #[serde(default)]
    pub damage_movement_modifier: Option<Handle<BezierCurve>>,
}

impl Pooled for STurretHealthModifierDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.sturret_health_modifier_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.sturret_health_modifier_def }
}

impl<'a> Extract<'a> for STurretHealthModifierDef {
    const TYPE_NAME: &'static str = "STurretHealthModifierDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            damage_movement_modifier: match inst.get("damageMovementModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `STurretESP`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STurretESP {
    /// DCB field: `triggerZoneRampInCurve` (Class)
    #[serde(default)]
    pub trigger_zone_ramp_in_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `maxTrackingStrength` (Single)
    #[serde(default)]
    pub max_tracking_strength: f32,
    /// DCB field: `distanceFalloffStart` (Single)
    #[serde(default)]
    pub distance_falloff_start: f32,
    /// DCB field: `distanceFalloffEnd` (Single)
    #[serde(default)]
    pub distance_falloff_end: f32,
    /// DCB field: `outerZoneDeg` (Single)
    #[serde(default)]
    pub outer_zone_deg: f32,
    /// DCB field: `innerZoneRatio` (Single)
    #[serde(default)]
    pub inner_zone_ratio: f32,
    /// DCB field: `adsZoneMinSizeDeg` (Single)
    #[serde(default)]
    pub ads_zone_min_size_deg: f32,
    /// DCB field: `inputScalerMin` (Single)
    #[serde(default)]
    pub input_scaler_min: f32,
    /// DCB field: `inputScalerMax` (Single)
    #[serde(default)]
    pub input_scaler_max: f32,
    /// DCB field: `allowWithRelativeMouseModes` (Boolean)
    #[serde(default)]
    pub allow_with_relative_mouse_modes: bool,
}

impl Pooled for STurretESP {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.sturret_esp }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.sturret_esp }
}

impl<'a> Extract<'a> for STurretESP {
    const TYPE_NAME: &'static str = "STurretESP";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_zone_ramp_in_curve: match inst.get("triggerZoneRampInCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_tracking_strength: inst.get_f32("maxTrackingStrength").unwrap_or_default(),
            distance_falloff_start: inst.get_f32("distanceFalloffStart").unwrap_or_default(),
            distance_falloff_end: inst.get_f32("distanceFalloffEnd").unwrap_or_default(),
            outer_zone_deg: inst.get_f32("outerZoneDeg").unwrap_or_default(),
            inner_zone_ratio: inst.get_f32("innerZoneRatio").unwrap_or_default(),
            ads_zone_min_size_deg: inst.get_f32("adsZoneMinSizeDeg").unwrap_or_default(),
            input_scaler_min: inst.get_f32("inputScalerMin").unwrap_or_default(),
            input_scaler_max: inst.get_f32("inputScalerMax").unwrap_or_default(),
            allow_with_relative_mouse_modes: inst.get_bool("allowWithRelativeMouseModes").unwrap_or_default(),
        }
    }
}

/// DCB type: `STurretGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STurretGlobalParams {
    /// DCB field: `pointerModeAllowed` (Boolean)
    #[serde(default)]
    pub pointer_mode_allowed: bool,
    /// DCB field: `pointerModeInputSmoothing` (Single)
    #[serde(default)]
    pub pointer_mode_input_smoothing: f32,
    /// DCB field: `pointerModeInnerAngle` (Single)
    #[serde(default)]
    pub pointer_mode_inner_angle: f32,
    /// DCB field: `pointerModeInnerAngleMaxSpeedModifier` (Single)
    #[serde(default)]
    pub pointer_mode_inner_angle_max_speed_modifier: f32,
    /// DCB field: `pointerModeInnerAngleTurretSmoothing` (Single)
    #[serde(default)]
    pub pointer_mode_inner_angle_turret_smoothing: f32,
    /// DCB field: `pointerModeMiddleAngle` (Single)
    #[serde(default)]
    pub pointer_mode_middle_angle: f32,
    /// DCB field: `pointerModeOuterAngle` (Single)
    #[serde(default)]
    pub pointer_mode_outer_angle: f32,
    /// DCB field: `pointerModeOuterAngleTurretSmoothing` (Single)
    #[serde(default)]
    pub pointer_mode_outer_angle_turret_smoothing: f32,
    /// DCB field: `pointerModeMaxDegPerSec` (Single)
    #[serde(default)]
    pub pointer_mode_max_deg_per_sec: f32,
    /// DCB field: `relativeInputAllowed` (Boolean)
    #[serde(default)]
    pub relative_input_allowed: bool,
    /// DCB field: `relativeInputSmoothing` (Single)
    #[serde(default)]
    pub relative_input_smoothing: f32,
}

impl Pooled for STurretGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.sturret_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.sturret_global_params }
}

impl<'a> Extract<'a> for STurretGlobalParams {
    const TYPE_NAME: &'static str = "STurretGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            pointer_mode_allowed: inst.get_bool("pointerModeAllowed").unwrap_or_default(),
            pointer_mode_input_smoothing: inst.get_f32("pointerModeInputSmoothing").unwrap_or_default(),
            pointer_mode_inner_angle: inst.get_f32("pointerModeInnerAngle").unwrap_or_default(),
            pointer_mode_inner_angle_max_speed_modifier: inst.get_f32("pointerModeInnerAngleMaxSpeedModifier").unwrap_or_default(),
            pointer_mode_inner_angle_turret_smoothing: inst.get_f32("pointerModeInnerAngleTurretSmoothing").unwrap_or_default(),
            pointer_mode_middle_angle: inst.get_f32("pointerModeMiddleAngle").unwrap_or_default(),
            pointer_mode_outer_angle: inst.get_f32("pointerModeOuterAngle").unwrap_or_default(),
            pointer_mode_outer_angle_turret_smoothing: inst.get_f32("pointerModeOuterAngleTurretSmoothing").unwrap_or_default(),
            pointer_mode_max_deg_per_sec: inst.get_f32("pointerModeMaxDegPerSec").unwrap_or_default(),
            relative_input_allowed: inst.get_bool("relativeInputAllowed").unwrap_or_default(),
            relative_input_smoothing: inst.get_f32("relativeInputSmoothing").unwrap_or_default(),
        }
    }
}

/// DCB type: `STierProgressions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STierProgressions {
    /// DCB field: `progressionText` (Locale)
    #[serde(default)]
    pub progression_text: String,
    /// DCB field: `minMultiplierThreshold` (Single)
    #[serde(default)]
    pub min_multiplier_threshold: f32,
    /// DCB field: `showAsEqualPartsOnProgression` (Boolean)
    #[serde(default)]
    pub show_as_equal_parts_on_progression: bool,
    /// DCB field: `lastTierDisplayOnProgression` (Boolean)
    #[serde(default)]
    pub last_tier_display_on_progression: bool,
    /// DCB field: `progressionColor` (String)
    #[serde(default)]
    pub progression_color: String,
    /// DCB field: `completionType` (StrongPointer)
    #[serde(default)]
    pub completion_type: Option<Handle<CompletionTypeBase>>,
    /// DCB field: `periodMissionScenario` (Reference)
    #[serde(default)]
    pub period_mission_scenario: Option<CigGuid>,
    /// DCB field: `tierRewards` (Class (array))
    #[serde(default)]
    pub tier_rewards: Vec<Handle<STierReward>>,
}

impl Pooled for STierProgressions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stier_progressions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stier_progressions }
}

impl<'a> Extract<'a> for STierProgressions {
    const TYPE_NAME: &'static str = "STierProgressions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            progression_text: inst.get_str("progressionText").map(String::from).unwrap_or_default(),
            min_multiplier_threshold: inst.get_f32("minMultiplierThreshold").unwrap_or_default(),
            show_as_equal_parts_on_progression: inst.get_bool("showAsEqualPartsOnProgression").unwrap_or_default(),
            last_tier_display_on_progression: inst.get_bool("lastTierDisplayOnProgression").unwrap_or_default(),
            progression_color: inst.get_str("progressionColor").map(String::from).unwrap_or_default(),
            completion_type: match inst.get("completionType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CompletionTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CompletionTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            period_mission_scenario: inst.get("periodMissionScenario").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            tier_rewards: inst.get_array("tierRewards")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<STierReward>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<STierReward>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `STierReward`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STierReward {
    /// DCB field: `minPoints` (Int32)
    #[serde(default)]
    pub min_points: i32,
    /// DCB field: `badgeToAward` (EnumChoice)
    #[serde(default)]
    pub badge_to_award: String,
    /// DCB field: `globalPointsMultiplier` (Int32)
    #[serde(default)]
    pub global_points_multiplier: i32,
    /// DCB field: `commsNotification` (Reference)
    #[serde(default)]
    pub comms_notification: Option<CigGuid>,
}

impl Pooled for STierReward {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.stier_reward }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.stier_reward }
}

impl<'a> Extract<'a> for STierReward {
    const TYPE_NAME: &'static str = "STierReward";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_points: inst.get_i32("minPoints").unwrap_or_default(),
            badge_to_award: inst.get_str("badgeToAward").map(String::from).unwrap_or_default(),
            global_points_multiplier: inst.get_i32("globalPointsMultiplier").unwrap_or_default(),
            comms_notification: inst.get("commsNotification").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `StarMapObjectType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapObjectType {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `classification` (Locale)
    #[serde(default)]
    pub classification: String,
    /// DCB field: `facingMode` (EnumChoice)
    #[serde(default)]
    pub facing_mode: String,
    /// DCB field: `minimumDisplaySize` (Single)
    #[serde(default)]
    pub minimum_display_size: f32,
    /// DCB field: `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// DCB field: `selectable` (Boolean)
    #[serde(default)]
    pub selectable: bool,
    /// DCB field: `fadeBehindParent` (Boolean)
    #[serde(default)]
    pub fade_behind_parent: bool,
    /// DCB field: `onParentSurface` (Boolean)
    #[serde(default)]
    pub on_parent_surface: bool,
    /// DCB field: `spawnNavPoints` (Boolean)
    #[serde(default)]
    pub spawn_nav_points: bool,
    /// DCB field: `showAsNeighbor` (Boolean)
    #[serde(default)]
    pub show_as_neighbor: bool,
    /// DCB field: `innerCulling` (Boolean)
    #[serde(default)]
    pub inner_culling: bool,
    /// DCB field: `showInMapSelectList` (Boolean)
    #[serde(default)]
    pub show_in_map_select_list: bool,
    /// DCB field: `markerConfig` (Reference)
    #[serde(default)]
    pub marker_config: Option<CigGuid>,
    /// DCB field: `validQuantumTravelDestination` (Boolean)
    #[serde(default)]
    pub valid_quantum_travel_destination: bool,
    /// DCB field: `geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for StarMapObjectType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_object_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_object_type }
}

impl<'a> Extract<'a> for StarMapObjectType {
    const TYPE_NAME: &'static str = "StarMapObjectType";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            classification: inst.get_str("classification").map(String::from).unwrap_or_default(),
            facing_mode: inst.get_str("facingMode").map(String::from).unwrap_or_default(),
            minimum_display_size: inst.get_f32("minimumDisplaySize").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            selectable: inst.get_bool("selectable").unwrap_or_default(),
            fade_behind_parent: inst.get_bool("fadeBehindParent").unwrap_or_default(),
            on_parent_surface: inst.get_bool("onParentSurface").unwrap_or_default(),
            spawn_nav_points: inst.get_bool("spawnNavPoints").unwrap_or_default(),
            show_as_neighbor: inst.get_bool("showAsNeighbor").unwrap_or_default(),
            inner_culling: inst.get_bool("innerCulling").unwrap_or_default(),
            show_in_map_select_list: inst.get_bool("showInMapSelectList").unwrap_or_default(),
            marker_config: inst.get("markerConfig").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            valid_quantum_travel_destination: inst.get_bool("validQuantumTravelDestination").unwrap_or_default(),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StarMapObjectTypes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapObjectTypes {
    /// DCB field: `types` (Reference (array))
    #[serde(default)]
    pub types: Vec<CigGuid>,
}

impl Pooled for StarMapObjectTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_object_types }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_object_types }
}

impl<'a> Extract<'a> for StarMapObjectTypes {
    const TYPE_NAME: &'static str = "StarMapObjectTypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types: inst.get_array("types")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StarMapAmenityTypeEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapAmenityTypeEntry {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
}

impl Pooled for StarMapAmenityTypeEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_amenity_type_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_amenity_type_entry }
}

impl<'a> Extract<'a> for StarMapAmenityTypeEntry {
    const TYPE_NAME: &'static str = "StarMapAmenityTypeEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `StarMapAmenityTypes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapAmenityTypes {
    /// DCB field: `amenityTypes` (Reference (array))
    #[serde(default)]
    pub amenity_types: Vec<CigGuid>,
}

impl Pooled for StarMapAmenityTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_amenity_types }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_amenity_types }
}

impl<'a> Extract<'a> for StarMapAmenityTypes {
    const TYPE_NAME: &'static str = "StarMapAmenityTypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            amenity_types: inst.get_array("amenityTypes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StarMapObjectLocationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapObjectLocationParams {
    /// DCB field: `setEntityLocationOnEnter` (Boolean)
    #[serde(default)]
    pub set_entity_location_on_enter: bool,
    /// DCB field: `exposeForPlayerCreatedMissions` (Boolean)
    #[serde(default)]
    pub expose_for_player_created_missions: bool,
    /// DCB field: `excludeFromLevelLoad` (Boolean)
    #[serde(default)]
    pub exclude_from_level_load: bool,
}

impl Pooled for StarMapObjectLocationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_object_location_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_object_location_params }
}

impl<'a> Extract<'a> for StarMapObjectLocationParams {
    const TYPE_NAME: &'static str = "StarMapObjectLocationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            set_entity_location_on_enter: inst.get_bool("setEntityLocationOnEnter").unwrap_or_default(),
            expose_for_player_created_missions: inst.get_bool("exposeForPlayerCreatedMissions").unwrap_or_default(),
            exclude_from_level_load: inst.get_bool("excludeFromLevelLoad").unwrap_or_default(),
        }
    }
}

/// DCB type: `StarMapAssetManagerLocationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapAssetManagerLocationParams {
    /// DCB field: `previewImagePath` (String)
    #[serde(default)]
    pub preview_image_path: String,
    /// DCB field: `previewIconPath` (String)
    #[serde(default)]
    pub preview_icon_path: String,
}

impl Pooled for StarMapAssetManagerLocationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_asset_manager_location_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_asset_manager_location_params }
}

impl<'a> Extract<'a> for StarMapAssetManagerLocationParams {
    const TYPE_NAME: &'static str = "StarMapAssetManagerLocationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            preview_image_path: inst.get_str("previewImagePath").map(String::from).unwrap_or_default(),
            preview_icon_path: inst.get_str("previewIconPath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `StarMapObject`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapObject {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `affiliation` (Reference)
    #[serde(default)]
    pub affiliation: Option<CigGuid>,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `callout1` (Locale)
    #[serde(default)]
    pub callout1: String,
    /// DCB field: `callout2` (Locale)
    #[serde(default)]
    pub callout2: String,
    /// DCB field: `callout3` (Locale)
    #[serde(default)]
    pub callout3: String,
    /// DCB field: `respawnLocationType` (EnumChoice)
    #[serde(default)]
    pub respawn_location_type: String,
    /// DCB field: `jurisdiction` (Reference)
    #[serde(default)]
    pub jurisdiction: Option<CigGuid>,
    /// DCB field: `locationHierarchyTag` (Reference)
    #[serde(default)]
    pub location_hierarchy_tag: Option<CigGuid>,
    /// DCB field: `type` (Reference)
    #[serde(default)]
    pub r#type: Option<CigGuid>,
    /// DCB field: `radarProperties` (StrongPointer)
    #[serde(default)]
    pub radar_properties: Option<Handle<SSCRadarContactProperites>>,
    /// DCB field: `navIcon` (EnumChoice)
    #[serde(default)]
    pub nav_icon: String,
    /// DCB field: `parent` (Reference)
    #[serde(default)]
    pub parent: Option<CigGuid>,
    /// DCB field: `isScannable` (Boolean)
    #[serde(default)]
    pub is_scannable: bool,
    /// DCB field: `size` (Double)
    #[serde(default)]
    pub size: f64,
    /// DCB field: `hideInStarmap` (Boolean)
    #[serde(default)]
    pub hide_in_starmap: bool,
    /// DCB field: `hideInWorld` (Boolean)
    #[serde(default)]
    pub hide_in_world: bool,
    /// DCB field: `hideWhenInAdoptionRadius` (Boolean)
    #[serde(default)]
    pub hide_when_in_adoption_radius: bool,
    /// DCB field: `blockTravel` (Boolean)
    #[serde(default)]
    pub block_travel: bool,
    /// DCB field: `onlyShowWhenParentSelected` (Boolean)
    #[serde(default)]
    pub only_show_when_parent_selected: bool,
    /// DCB field: `overrideShowInAllZones` (EnumChoice)
    #[serde(default)]
    pub override_show_in_all_zones: String,
    /// DCB field: `overridePermanent` (EnumChoice)
    #[serde(default)]
    pub override_permanent: String,
    /// DCB field: `minimumDisplaySize` (Single)
    #[serde(default)]
    pub minimum_display_size: f32,
    /// DCB field: `overrideRotationSpeed` (Boolean)
    #[serde(default)]
    pub override_rotation_speed: bool,
    /// DCB field: `overrideRotationSpeedValue` (Single)
    #[serde(default)]
    pub override_rotation_speed_value: f32,
    /// DCB field: `showOrbitLine` (Boolean)
    #[serde(default)]
    pub show_orbit_line: bool,
    /// DCB field: `useHoloMaterial` (Boolean)
    #[serde(default)]
    pub use_holo_material: bool,
    /// DCB field: `noAutoBodyRecovery` (Boolean)
    #[serde(default)]
    pub no_auto_body_recovery: bool,
    /// DCB field: `starMapGeomPath` (String)
    #[serde(default)]
    pub star_map_geom_path: String,
    /// DCB field: `starMapMaterialPath` (String)
    #[serde(default)]
    pub star_map_material_path: String,
    /// DCB field: `starMapShapePath` (String)
    #[serde(default)]
    pub star_map_shape_path: String,
    /// DCB field: `assetManagerLocationParams` (Class)
    #[serde(default)]
    pub asset_manager_location_params: Option<Handle<StarMapAssetManagerLocationParams>>,
    /// DCB field: `asteroidRings` (Class (array))
    #[serde(default)]
    pub asteroid_rings: Vec<Handle<StarMapAsteroidRing>>,
    /// DCB field: `quantumTravelData` (StrongPointer)
    #[serde(default)]
    pub quantum_travel_data: Option<Handle<StarMapQuantumTravelDataParams>>,
    /// DCB field: `locationParams` (StrongPointer)
    #[serde(default)]
    pub location_params: Option<Handle<StarMapObjectLocationParams>>,
    /// DCB field: `locationImagePath` (String)
    #[serde(default)]
    pub location_image_path: String,
    /// DCB field: `locationMedicalImagePath` (String)
    #[serde(default)]
    pub location_medical_image_path: String,
    /// DCB field: `locationAudioPlayTrigger` (Class)
    #[serde(default)]
    pub location_audio_play_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `locationAudioStopTrigger` (Class)
    #[serde(default)]
    pub location_audio_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `amenities` (Reference (array))
    #[serde(default)]
    pub amenities: Vec<CigGuid>,
}

impl Pooled for StarMapObject {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_object }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_object }
}

impl<'a> Extract<'a> for StarMapObject {
    const TYPE_NAME: &'static str = "StarMapObject";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            affiliation: inst.get("affiliation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            callout1: inst.get_str("callout1").map(String::from).unwrap_or_default(),
            callout2: inst.get_str("callout2").map(String::from).unwrap_or_default(),
            callout3: inst.get_str("callout3").map(String::from).unwrap_or_default(),
            respawn_location_type: inst.get_str("respawnLocationType").map(String::from).unwrap_or_default(),
            jurisdiction: inst.get("jurisdiction").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            location_hierarchy_tag: inst.get("locationHierarchyTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            r#type: inst.get("type").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            radar_properties: match inst.get("radarProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SSCRadarContactProperites>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSCRadarContactProperites>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            nav_icon: inst.get_str("navIcon").map(String::from).unwrap_or_default(),
            parent: inst.get("parent").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            is_scannable: inst.get_bool("isScannable").unwrap_or_default(),
            size: inst.get_f64("size").unwrap_or_default(),
            hide_in_starmap: inst.get_bool("hideInStarmap").unwrap_or_default(),
            hide_in_world: inst.get_bool("hideInWorld").unwrap_or_default(),
            hide_when_in_adoption_radius: inst.get_bool("hideWhenInAdoptionRadius").unwrap_or_default(),
            block_travel: inst.get_bool("blockTravel").unwrap_or_default(),
            only_show_when_parent_selected: inst.get_bool("onlyShowWhenParentSelected").unwrap_or_default(),
            override_show_in_all_zones: inst.get_str("overrideShowInAllZones").map(String::from).unwrap_or_default(),
            override_permanent: inst.get_str("overridePermanent").map(String::from).unwrap_or_default(),
            minimum_display_size: inst.get_f32("minimumDisplaySize").unwrap_or_default(),
            override_rotation_speed: inst.get_bool("overrideRotationSpeed").unwrap_or_default(),
            override_rotation_speed_value: inst.get_f32("overrideRotationSpeedValue").unwrap_or_default(),
            show_orbit_line: inst.get_bool("showOrbitLine").unwrap_or_default(),
            use_holo_material: inst.get_bool("useHoloMaterial").unwrap_or_default(),
            no_auto_body_recovery: inst.get_bool("noAutoBodyRecovery").unwrap_or_default(),
            star_map_geom_path: inst.get_str("starMapGeomPath").map(String::from).unwrap_or_default(),
            star_map_material_path: inst.get_str("starMapMaterialPath").map(String::from).unwrap_or_default(),
            star_map_shape_path: inst.get_str("starMapShapePath").map(String::from).unwrap_or_default(),
            asset_manager_location_params: match inst.get("assetManagerLocationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StarMapAssetManagerLocationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StarMapAssetManagerLocationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            asteroid_rings: inst.get_array("asteroidRings")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<StarMapAsteroidRing>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<StarMapAsteroidRing>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            quantum_travel_data: match inst.get("quantumTravelData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StarMapQuantumTravelDataParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StarMapQuantumTravelDataParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            location_params: match inst.get("locationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<StarMapObjectLocationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<StarMapObjectLocationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            location_image_path: inst.get_str("locationImagePath").map(String::from).unwrap_or_default(),
            location_medical_image_path: inst.get_str("locationMedicalImagePath").map(String::from).unwrap_or_default(),
            location_audio_play_trigger: match inst.get("locationAudioPlayTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            location_audio_stop_trigger: match inst.get("locationAudioStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            amenities: inst.get_array("amenities")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `StarMapAsteroidRing`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapAsteroidRing {
    /// DCB field: `densityScale` (Single)
    #[serde(default)]
    pub density_scale: f32,
    /// DCB field: `sizeScale` (Single)
    #[serde(default)]
    pub size_scale: f32,
    /// DCB field: `innerRadius` (Double)
    #[serde(default)]
    pub inner_radius: f64,
    /// DCB field: `outerRadius` (Double)
    #[serde(default)]
    pub outer_radius: f64,
    /// DCB field: `depth` (Single)
    #[serde(default)]
    pub depth: f32,
}

impl Pooled for StarMapAsteroidRing {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_asteroid_ring }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_asteroid_ring }
}

impl<'a> Extract<'a> for StarMapAsteroidRing {
    const TYPE_NAME: &'static str = "StarMapAsteroidRing";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            density_scale: inst.get_f32("densityScale").unwrap_or_default(),
            size_scale: inst.get_f32("sizeScale").unwrap_or_default(),
            inner_radius: inst.get_f64("innerRadius").unwrap_or_default(),
            outer_radius: inst.get_f64("outerRadius").unwrap_or_default(),
            depth: inst.get_f32("depth").unwrap_or_default(),
        }
    }
}

/// DCB type: `StarMapQuantumTravelDataParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapQuantumTravelDataParams {
    /// DCB field: `obstructionRadius` (Single)
    #[serde(default)]
    pub obstruction_radius: f32,
    /// DCB field: `arrivalRadius` (Single)
    #[serde(default)]
    pub arrival_radius: f32,
    /// DCB field: `arrivalPointDetectionOffset` (Single)
    #[serde(default)]
    pub arrival_point_detection_offset: f32,
    /// DCB field: `adoptionRadius` (Single)
    #[serde(default)]
    pub adoption_radius: f32,
    /// DCB field: `subPointRadiusMultiplier` (Single)
    #[serde(default)]
    pub sub_point_radius_multiplier: f32,
}

impl Pooled for StarMapQuantumTravelDataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_quantum_travel_data_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_quantum_travel_data_params }
}

impl<'a> Extract<'a> for StarMapQuantumTravelDataParams {
    const TYPE_NAME: &'static str = "StarMapQuantumTravelDataParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            obstruction_radius: inst.get_f32("obstructionRadius").unwrap_or_default(),
            arrival_radius: inst.get_f32("arrivalRadius").unwrap_or_default(),
            arrival_point_detection_offset: inst.get_f32("arrivalPointDetectionOffset").unwrap_or_default(),
            adoption_radius: inst.get_f32("adoptionRadius").unwrap_or_default(),
            sub_point_radius_multiplier: inst.get_f32("subPointRadiusMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `StarMapMissionObject`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapMissionObject {
    /// DCB field: `minimumDisplaySize` (Single)
    #[serde(default)]
    pub minimum_display_size: f32,
    /// DCB field: `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// DCB field: `facingMode` (EnumChoice)
    #[serde(default)]
    pub facing_mode: String,
    /// DCB field: `geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for StarMapMissionObject {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_mission_object }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_mission_object }
}

impl<'a> Extract<'a> for StarMapMissionObject {
    const TYPE_NAME: &'static str = "StarMapMissionObject";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minimum_display_size: inst.get_f32("minimumDisplaySize").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            facing_mode: inst.get_str("facingMode").map(String::from).unwrap_or_default(),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StarMapPartyMemberObject`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMapPartyMemberObject {
    /// DCB field: `minimumDisplaySize` (Single)
    #[serde(default)]
    pub minimum_display_size: f32,
    /// DCB field: `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// DCB field: `facingMode` (EnumChoice)
    #[serde(default)]
    pub facing_mode: String,
    /// DCB field: `geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// DCB field: `material` (Class)
    #[serde(default)]
    pub material: Option<Handle<GlobalResourceMaterial>>,
}

impl Pooled for StarMapPartyMemberObject {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.star_map_party_member_object }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.star_map_party_member_object }
}

impl<'a> Extract<'a> for StarMapPartyMemberObject {
    const TYPE_NAME: &'static str = "StarMapPartyMemberObject";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minimum_display_size: inst.get_f32("minimumDisplaySize").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            facing_mode: inst.get_str("facingMode").map(String::from).unwrap_or_default(),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            material: match inst.get("material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `StatusWidgetDisplayPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusWidgetDisplayPreset {
    /// DCB field: `ranges` (Class)
    #[serde(default)]
    pub ranges: Option<Handle<DisplayState>>,
    /// DCB field: `incrementDisplayDuration` (Single)
    #[serde(default)]
    pub increment_display_duration: f32,
    /// DCB field: `incrementStep` (Single)
    #[serde(default)]
    pub increment_step: f32,
    /// DCB field: `maximumChangePerSecond` (Single)
    #[serde(default)]
    pub maximum_change_per_second: f32,
    /// DCB field: `historySeconds` (Int32)
    #[serde(default)]
    pub history_seconds: i32,
    /// DCB field: `historySamplesPerSecond` (Int32)
    #[serde(default)]
    pub history_samples_per_second: i32,
    /// DCB field: `shownOnLens` (Boolean)
    #[serde(default)]
    pub shown_on_lens: bool,
    /// DCB field: `shownOnVisor` (Boolean)
    #[serde(default)]
    pub shown_on_visor: bool,
}

impl Pooled for StatusWidgetDisplayPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_st.status_widget_display_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_st.status_widget_display_preset }
}

impl<'a> Extract<'a> for StatusWidgetDisplayPreset {
    const TYPE_NAME: &'static str = "StatusWidgetDisplayPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ranges: match inst.get("ranges") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DisplayState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DisplayState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            increment_display_duration: inst.get_f32("incrementDisplayDuration").unwrap_or_default(),
            increment_step: inst.get_f32("incrementStep").unwrap_or_default(),
            maximum_change_per_second: inst.get_f32("maximumChangePerSecond").unwrap_or_default(),
            history_seconds: inst.get_i32("historySeconds").unwrap_or_default(),
            history_samples_per_second: inst.get_i32("historySamplesPerSecond").unwrap_or_default(),
            shown_on_lens: inst.get_bool("shownOnLens").unwrap_or_default(),
            shown_on_visor: inst.get_bool("shownOnVisor").unwrap_or_default(),
        }
    }
}

