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

/// DCB type: `PartialContractRewardRepAdjustment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialContractRewardRepAdjustment {
    /// DCB field: `reputationScope` (Reference)
    #[serde(default)]
    pub reputation_scope: Option<CigGuid>,
    /// DCB field: `reputationRewardMultiplier` (Single)
    #[serde(default)]
    pub reputation_reward_multiplier: f32,
}

impl Pooled for PartialContractRewardRepAdjustment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pa.partial_contract_reward_rep_adjustment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pa.partial_contract_reward_rep_adjustment }
}

impl<'a> Extract<'a> for PartialContractRewardRepAdjustment {
    const TYPE_NAME: &'static str = "PartialContractRewardRepAdjustment";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reputation_scope: inst.get("reputationScope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reputation_reward_multiplier: inst.get_f32("reputationRewardMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `PartialContractRewardRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialContractRewardRange {
    /// DCB field: `minPercentage` (Int32)
    #[serde(default)]
    pub min_percentage: i32,
    /// DCB field: `maxPercentage` (Int32)
    #[serde(default)]
    pub max_percentage: i32,
    /// DCB field: `currencyRewardMultiplier` (Single)
    #[serde(default)]
    pub currency_reward_multiplier: f32,
    /// DCB field: `reputationMultipliers` (Class (array))
    #[serde(default)]
    pub reputation_multipliers: Vec<Handle<PartialContractRewardRepAdjustment>>,
}

impl Pooled for PartialContractRewardRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pa.partial_contract_reward_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pa.partial_contract_reward_range }
}

impl<'a> Extract<'a> for PartialContractRewardRange {
    const TYPE_NAME: &'static str = "PartialContractRewardRange";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            min_percentage: inst.get_i32("minPercentage").unwrap_or_default(),
            max_percentage: inst.get_i32("maxPercentage").unwrap_or_default(),
            currency_reward_multiplier: inst.get_f32("currencyRewardMultiplier").unwrap_or_default(),
            reputation_multipliers: inst.get_array("reputationMultipliers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PartialContractRewardRepAdjustment>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PartialContractRewardRepAdjustment>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PartialContractRewards`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialContractRewards {
    /// DCB field: `percentageThresholds` (Class (array))
    #[serde(default)]
    pub percentage_thresholds: Vec<Handle<PartialContractRewardRange>>,
}

impl Pooled for PartialContractRewards {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pa.partial_contract_rewards }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pa.partial_contract_rewards }
}

impl<'a> Extract<'a> for PartialContractRewards {
    const TYPE_NAME: &'static str = "PartialContractRewards";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            percentage_thresholds: inst.get_array("percentageThresholds")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PartialContractRewardRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PartialContractRewardRange>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

