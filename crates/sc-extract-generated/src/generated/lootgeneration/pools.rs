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

/// Pool storage for the `lootgeneration` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LootgenerationPools {
    #[serde(default)]
    pub entry_optional_data_base: Vec<Option<EntryOptionalData_Base>>,
    #[serde(default)]
    pub loot_archetype_entry_primary: Vec<Option<LootArchetypeEntry_Primary>>,
    #[serde(default)]
    pub loot_archetype_entry_secondary: Vec<Option<LootArchetypeEntry_Secondary>>,
    #[serde(default)]
    pub loot_archetype_or_group_primary: Vec<Option<LootArchetypeOrGroup_Primary>>,
    #[serde(default)]
    pub loot_archetype_or_group_secondary: Vec<Option<LootArchetypeOrGroup_Secondary>>,
    #[serde(default)]
    pub loot_archetype: Vec<Option<LootArchetype>>,
    #[serde(default)]
    pub num_results_constraints: Vec<Option<NumResultsConstraints>>,
    #[serde(default)]
    pub weighted_loot_archetype: Vec<Option<WeightedLootArchetype>>,
    #[serde(default)]
    pub loot_table: Vec<Option<LootTable>>,
    #[serde(default)]
    pub loot_generation_special_event_archetype: Vec<Option<LootGenerationSpecialEventArchetype>>,
    #[serde(default)]
    pub pool_filter_no_ref: Vec<Option<PoolFilter_NoRef>>,
    #[serde(default)]
    pub pool_filter_record: Vec<Option<PoolFilterRecord>>,
    #[serde(default)]
    pub loot_table_v3_no_ref: Vec<Option<LootTableV3_NoRef>>,
    #[serde(default)]
    pub loot_table_v3_record: Vec<Option<LootTableV3Record>>,
    #[serde(default)]
    pub loot_archetype_v3_base: Vec<Option<LootArchetypeV3_Base>>,
    #[serde(default)]
    pub loot_archetype_v3_no_ref: Vec<Option<LootArchetypeV3_NoRef>>,
    #[serde(default)]
    pub loot_archetype_v3_record: Vec<Option<LootArchetypeV3Record>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_single_layer_record: Vec<Option<LootV3SecondaryChoicesSingleLayerRecord>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_multi_layer_record: Vec<Option<LootV3SecondaryChoicesMultiLayerRecord>>,
    #[serde(default)]
    pub loot_v3_secondary_choice_entry_selector_base: Vec<Option<LootV3SecondaryChoiceEntrySelector_Base>>,
    #[serde(default)]
    pub loot_v3_secondary_choice_entry: Vec<Option<LootV3SecondaryChoiceEntry>>,
    #[serde(default)]
    pub loot_generation_global_params: Vec<Option<LootGenerationGlobalParams>>,
}
