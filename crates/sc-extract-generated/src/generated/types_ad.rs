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

/// DCB type: `AdvancedLootConstraints`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedLootConstraints {
    /// DCB field: `pruningLevel` (EnumChoice)
    #[serde(default)]
    pub pruning_level: String,
    /// DCB field: `fullnessMode` (EnumChoice)
    #[serde(default)]
    pub fullness_mode: String,
}

impl Pooled for AdvancedLootConstraints {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ad.advanced_loot_constraints }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ad.advanced_loot_constraints }
}

impl<'a> Extract<'a> for AdvancedLootConstraints {
    const TYPE_NAME: &'static str = "AdvancedLootConstraints";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            pruning_level: inst.get_str("pruningLevel").map(String::from).unwrap_or_default(),
            fullness_mode: inst.get_str("fullnessMode").map(String::from).unwrap_or_default(),
        }
    }
}

