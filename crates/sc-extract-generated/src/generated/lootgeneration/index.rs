// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `lootgeneration` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LootgenerationIndex {
    #[serde(default)]
    pub loot_archetype: HashMap<CigGuid, Handle<LootArchetype>>,
    #[serde(default)]
    pub loot_table: HashMap<CigGuid, Handle<LootTable>>,
    #[serde(default)]
    pub pool_filter_record: HashMap<CigGuid, Handle<PoolFilterRecord>>,
    #[serde(default)]
    pub loot_table_v3_record: HashMap<CigGuid, Handle<LootTableV3Record>>,
    #[serde(default)]
    pub loot_archetype_v3_record: HashMap<CigGuid, Handle<LootArchetypeV3Record>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_single_layer_record: HashMap<CigGuid, Handle<LootV3SecondaryChoicesSingleLayerRecord>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_multi_layer_record: HashMap<CigGuid, Handle<LootV3SecondaryChoicesMultiLayerRecord>>,
    #[serde(default)]
    pub loot_generation_global_params: HashMap<CigGuid, Handle<LootGenerationGlobalParams>>,
}

impl LootgenerationIndex {
    pub fn len(&self) -> usize {
        self.loot_archetype.len()
            + self.loot_table.len()
            + self.pool_filter_record.len()
            + self.loot_table_v3_record.len()
            + self.loot_archetype_v3_record.len()
            + self.loot_v3_secondary_choices_single_layer_record.len()
            + self.loot_v3_secondary_choices_multi_layer_record.len()
            + self.loot_generation_global_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
