// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `lootgeneration` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LootgenerationPools {
    #[serde(default)]
    pub loot_generation_special_event_archetype: Vec<Option<LootGenerationSpecialEventArchetype>>,
    #[serde(default)]
    pub loot_generation_global_params: Vec<Option<LootGenerationGlobalParams>>,
}
