// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `traversalcostconfig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraversalcostconfigPools {
    #[serde(default)]
    pub traversal_cost_condition_tags: Vec<Option<TraversalCostConditionTags>>,
    #[serde(default)]
    pub cost_modifier_per_agent_type: Vec<Option<CostModifierPerAgentType>>,
    #[serde(default)]
    pub traversal_cost_config: Vec<Option<TraversalCostConfig>>,
}
