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

/// Pool storage for the `targetselector` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetselectorPools {
    #[serde(default)]
    pub stargeting_method_base: Vec<Option<STargetingMethodBase>>,
    #[serde(default)]
    pub stargeting_method_record: Vec<Option<STargetingMethodRecord>>,
    #[serde(default)]
    pub sprecision_targeting_item_name: Vec<Option<SPrecisionTargetingItemName>>,
    #[serde(default)]
    pub stargetable_item_type: Vec<Option<STargetableItemType>>,
    #[serde(default)]
    pub stargetable_item_types_record: Vec<Option<STargetableItemTypesRecord>>,
    #[serde(default)]
    pub scombat_targeting: Vec<Option<SCombatTargeting>>,
    #[serde(default)]
    pub sscan_targeting: Vec<Option<SScanTargeting>>,
    #[serde(default)]
    pub smining_targeting: Vec<Option<SMiningTargeting>>,
    #[serde(default)]
    pub starget_selector_global_targeting_params: Vec<Option<STargetSelectorGlobalTargetingParams>>,
}
