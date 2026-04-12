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

/// DCB type: `DriftingConsciousnessConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftingConsciousnessConfig {
    /// DCB field: `minValue` (Single)
    #[serde(default)]
    pub min_value: f32,
    /// DCB field: `maxValue` (Single)
    #[serde(default)]
    pub max_value: f32,
    /// DCB field: `fadeInSpeed` (Single)
    #[serde(default)]
    pub fade_in_speed: f32,
    /// DCB field: `fadeOutSpeed` (Single)
    #[serde(default)]
    pub fade_out_speed: f32,
    /// DCB field: `fadeInDelay` (Single)
    #[serde(default)]
    pub fade_in_delay: f32,
    /// DCB field: `fadeOutDelay` (Single)
    #[serde(default)]
    pub fade_out_delay: f32,
}

impl Pooled for DriftingConsciousnessConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dr.drifting_consciousness_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dr.drifting_consciousness_config }
}

impl<'a> Extract<'a> for DriftingConsciousnessConfig {
    const TYPE_NAME: &'static str = "DriftingConsciousnessConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_value: inst.get_f32("minValue").unwrap_or_default(),
            max_value: inst.get_f32("maxValue").unwrap_or_default(),
            fade_in_speed: inst.get_f32("fadeInSpeed").unwrap_or_default(),
            fade_out_speed: inst.get_f32("fadeOutSpeed").unwrap_or_default(),
            fade_in_delay: inst.get_f32("fadeInDelay").unwrap_or_default(),
            fade_out_delay: inst.get_f32("fadeOutDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `DriftingDrunkBDLEffects`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftingDrunkBDLEffects {
    /// DCB field: `stumbleMinCooldown` (Single)
    #[serde(default)]
    pub stumble_min_cooldown: f32,
    /// DCB field: `stumbleMaxCooldown` (Single)
    #[serde(default)]
    pub stumble_max_cooldown: f32,
    /// DCB field: `stumbleMinDuration` (Single)
    #[serde(default)]
    pub stumble_min_duration: f32,
    /// DCB field: `stumbleMaxDuration` (Single)
    #[serde(default)]
    pub stumble_max_duration: f32,
    /// DCB field: `stumbleFrequency` (Single)
    #[serde(default)]
    pub stumble_frequency: f32,
    /// DCB field: `stumbleMinMagnitude` (Single)
    #[serde(default)]
    pub stumble_min_magnitude: f32,
    /// DCB field: `stumbleMaxMagnitude` (Single)
    #[serde(default)]
    pub stumble_max_magnitude: f32,
    /// DCB field: `minValue` (Single)
    #[serde(default)]
    pub min_value: f32,
    /// DCB field: `maxValue` (Single)
    #[serde(default)]
    pub max_value: f32,
    /// DCB field: `fadeSpeedMin` (Single)
    #[serde(default)]
    pub fade_speed_min: f32,
    /// DCB field: `fadeSpeedMax` (Single)
    #[serde(default)]
    pub fade_speed_max: f32,
    /// DCB field: `fadeFrequency` (Single)
    #[serde(default)]
    pub fade_frequency: f32,
    /// DCB field: `fovScaleAdjust` (Single)
    #[serde(default)]
    pub fov_scale_adjust: f32,
    /// DCB field: `breathingAmplifier` (Single)
    #[serde(default)]
    pub breathing_amplifier: f32,
}

impl Pooled for DriftingDrunkBDLEffects {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dr.drifting_drunk_bdleffects }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dr.drifting_drunk_bdleffects }
}

impl<'a> Extract<'a> for DriftingDrunkBDLEffects {
    const TYPE_NAME: &'static str = "DriftingDrunkBDLEffects";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            stumble_min_cooldown: inst.get_f32("stumbleMinCooldown").unwrap_or_default(),
            stumble_max_cooldown: inst.get_f32("stumbleMaxCooldown").unwrap_or_default(),
            stumble_min_duration: inst.get_f32("stumbleMinDuration").unwrap_or_default(),
            stumble_max_duration: inst.get_f32("stumbleMaxDuration").unwrap_or_default(),
            stumble_frequency: inst.get_f32("stumbleFrequency").unwrap_or_default(),
            stumble_min_magnitude: inst.get_f32("stumbleMinMagnitude").unwrap_or_default(),
            stumble_max_magnitude: inst.get_f32("stumbleMaxMagnitude").unwrap_or_default(),
            min_value: inst.get_f32("minValue").unwrap_or_default(),
            max_value: inst.get_f32("maxValue").unwrap_or_default(),
            fade_speed_min: inst.get_f32("fadeSpeedMin").unwrap_or_default(),
            fade_speed_max: inst.get_f32("fadeSpeedMax").unwrap_or_default(),
            fade_frequency: inst.get_f32("fadeFrequency").unwrap_or_default(),
            fov_scale_adjust: inst.get_f32("fovScaleAdjust").unwrap_or_default(),
            breathing_amplifier: inst.get_f32("breathingAmplifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `DriftingDrunkConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftingDrunkConfig {
    /// DCB field: `stumbleForwardWeight` (Single)
    #[serde(default)]
    pub stumble_forward_weight: f32,
    /// DCB field: `stumbleBackwardWeight` (Single)
    #[serde(default)]
    pub stumble_backward_weight: f32,
    /// DCB field: `stumbleLeftWeight` (Single)
    #[serde(default)]
    pub stumble_left_weight: f32,
    /// DCB field: `stumbleRightWeight` (Single)
    #[serde(default)]
    pub stumble_right_weight: f32,
    /// DCB field: `minBDLEffects` (Class)
    #[serde(default)]
    pub min_bdleffects: Option<Handle<DriftingDrunkBDLEffects>>,
    /// DCB field: `maxBDLEffects` (Class)
    #[serde(default)]
    pub max_bdleffects: Option<Handle<DriftingDrunkBDLEffects>>,
}

impl Pooled for DriftingDrunkConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dr.drifting_drunk_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dr.drifting_drunk_config }
}

impl<'a> Extract<'a> for DriftingDrunkConfig {
    const TYPE_NAME: &'static str = "DriftingDrunkConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            stumble_forward_weight: inst.get_f32("stumbleForwardWeight").unwrap_or_default(),
            stumble_backward_weight: inst.get_f32("stumbleBackwardWeight").unwrap_or_default(),
            stumble_left_weight: inst.get_f32("stumbleLeftWeight").unwrap_or_default(),
            stumble_right_weight: inst.get_f32("stumbleRightWeight").unwrap_or_default(),
            min_bdleffects: match inst.get("minBDLEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DriftingDrunkBDLEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DriftingDrunkBDLEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_bdleffects: match inst.get("maxBDLEffects") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DriftingDrunkBDLEffects>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DriftingDrunkBDLEffects>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DrugEfficacy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugEfficacy {
    /// DCB field: `maxDose` (Single)
    #[serde(default)]
    pub max_dose: f32,
    /// DCB field: `BDLMultiplier` (Single)
    #[serde(default)]
    pub bdlmultiplier: f32,
    /// DCB field: `minsPerDosageMultiplier` (Single)
    #[serde(default)]
    pub mins_per_dosage_multiplier: f32,
    /// DCB field: `ValuePerDosageMultiplier` (Single)
    #[serde(default)]
    pub value_per_dosage_multiplier: f32,
}

impl Pooled for DrugEfficacy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dr.drug_efficacy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dr.drug_efficacy }
}

impl<'a> Extract<'a> for DrugEfficacy {
    const TYPE_NAME: &'static str = "DrugEfficacy";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_dose: inst.get_f32("maxDose").unwrap_or_default(),
            bdlmultiplier: inst.get_f32("BDLMultiplier").unwrap_or_default(),
            mins_per_dosage_multiplier: inst.get_f32("minsPerDosageMultiplier").unwrap_or_default(),
            value_per_dosage_multiplier: inst.get_f32("ValuePerDosageMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `DrugEfficacyForConsumableType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugEfficacyForConsumableType {
    /// DCB field: `consumableSubtype` (Reference)
    #[serde(default)]
    pub consumable_subtype: Option<CigGuid>,
    /// DCB field: `drugEfficacyForConsumableType` (Class)
    #[serde(default)]
    pub drug_efficacy_for_consumable_type: Option<Handle<DrugEfficacy>>,
}

impl Pooled for DrugEfficacyForConsumableType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dr.drug_efficacy_for_consumable_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dr.drug_efficacy_for_consumable_type }
}

impl<'a> Extract<'a> for DrugEfficacyForConsumableType {
    const TYPE_NAME: &'static str = "DrugEfficacyForConsumableType";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            consumable_subtype: inst.get("consumableSubtype").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            drug_efficacy_for_consumable_type: match inst.get("drugEfficacyForConsumableType") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DrugEfficacy>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DrugEfficacy>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DrugEfficacyConfigForItemSubTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugEfficacyConfigForItemSubTypeBase {
    /// DCB field: `itemSubType` (EnumChoice)
    #[serde(default)]
    pub item_sub_type: String,
}

impl Pooled for DrugEfficacyConfigForItemSubTypeBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dr.drug_efficacy_config_for_item_sub_type_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dr.drug_efficacy_config_for_item_sub_type_base }
}

impl<'a> Extract<'a> for DrugEfficacyConfigForItemSubTypeBase {
    const TYPE_NAME: &'static str = "DrugEfficacyConfigForItemSubTypeBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_sub_type: inst.get_str("itemSubType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `DrugEfficacyForItemType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugEfficacyForItemType {
    /// DCB field: `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// DCB field: `drugEfficacyForItemSubType` (StrongPointer (array))
    #[serde(default)]
    pub drug_efficacy_for_item_sub_type: Vec<Handle<DrugEfficacyConfigForItemSubTypeBase>>,
}

impl Pooled for DrugEfficacyForItemType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dr.drug_efficacy_for_item_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dr.drug_efficacy_for_item_type }
}

impl<'a> Extract<'a> for DrugEfficacyForItemType {
    const TYPE_NAME: &'static str = "DrugEfficacyForItemType";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("itemType").map(String::from).unwrap_or_default(),
            drug_efficacy_for_item_sub_type: inst.get_array("drugEfficacyForItemSubType")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DrugEfficacyConfigForItemSubTypeBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DrugEfficacyConfigForItemSubTypeBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DrugTypeToApply`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugTypeToApply {
    /// DCB field: `consumableSubtype` (Reference)
    #[serde(default)]
    pub consumable_subtype: Option<CigGuid>,
}

impl Pooled for DrugTypeToApply {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_dr.drug_type_to_apply }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_dr.drug_type_to_apply }
}

impl<'a> Extract<'a> for DrugTypeToApply {
    const TYPE_NAME: &'static str = "DrugTypeToApply";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            consumable_subtype: inst.get("consumableSubtype").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

