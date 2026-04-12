// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `traversalcostconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `TraversalCostConditionTags`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalCostConditionTags {
    /// `tags` (Class)
    #[serde(default)]
    pub tags: Option<Handle<TagsDNF>>,
    /// `costMultiplier` (Single)
    #[serde(default)]
    pub cost_multiplier: f32,
    /// `blocksTraversability` (Boolean)
    #[serde(default)]
    pub blocks_traversability: bool,
    /// `blocksStopping` (Boolean)
    #[serde(default)]
    pub blocks_stopping: bool,
}

impl Pooled for TraversalCostConditionTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.traversalcostconfig.traversal_cost_condition_tags }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.traversalcostconfig.traversal_cost_condition_tags }
}

impl<'a> Extract<'a> for TraversalCostConditionTags {
    const TYPE_NAME: &'static str = "TraversalCostConditionTags";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: match inst.get("tags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cost_multiplier: inst.get_f32("costMultiplier").unwrap_or_default(),
            blocks_traversability: inst.get_bool("blocksTraversability").unwrap_or_default(),
            blocks_stopping: inst.get_bool("blocksStopping").unwrap_or_default(),
        }
    }
}

/// DCB type: `CostModifierPerAgentType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostModifierPerAgentType {
    /// `agentType` (EnumChoice)
    #[serde(default)]
    pub agent_type: String,
    /// `traversalCostVariants` (Class (array))
    #[serde(default)]
    pub traversal_cost_variants: Vec<Handle<TraversalCostConditionTags>>,
    /// `defaultCostMultiplier` (Single)
    #[serde(default)]
    pub default_cost_multiplier: f32,
    /// `blocksTraversability` (Boolean)
    #[serde(default)]
    pub blocks_traversability: bool,
    /// `blocksStopping` (Boolean)
    #[serde(default)]
    pub blocks_stopping: bool,
}

impl Pooled for CostModifierPerAgentType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.traversalcostconfig.cost_modifier_per_agent_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.traversalcostconfig.cost_modifier_per_agent_type }
}

impl<'a> Extract<'a> for CostModifierPerAgentType {
    const TYPE_NAME: &'static str = "CostModifierPerAgentType";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            agent_type: inst.get_str("agentType").map(String::from).unwrap_or_default(),
            traversal_cost_variants: inst.get_array("traversalCostVariants")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TraversalCostConditionTags>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TraversalCostConditionTags>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_cost_multiplier: inst.get_f32("defaultCostMultiplier").unwrap_or_default(),
            blocks_traversability: inst.get_bool("blocksTraversability").unwrap_or_default(),
            blocks_stopping: inst.get_bool("blocksStopping").unwrap_or_default(),
        }
    }
}

/// DCB type: `TraversalCostConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalCostConfig {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
    /// `costs` (Class (array))
    #[serde(default)]
    pub costs: Vec<Handle<CostModifierPerAgentType>>,
}

impl Pooled for TraversalCostConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.traversalcostconfig.traversal_cost_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.traversalcostconfig.traversal_cost_config }
}

impl<'a> Extract<'a> for TraversalCostConfig {
    const TYPE_NAME: &'static str = "TraversalCostConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            costs: inst.get_array("costs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CostModifierPerAgentType>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CostModifierPerAgentType>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

