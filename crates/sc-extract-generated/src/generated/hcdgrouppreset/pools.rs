// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `hcdgrouppreset` feature.
#[derive(Default)]
pub struct HcdgrouppresetPools {
    pub location_faction_context_data: Vec<Option<LocationFactionContextData>>,
    pub location_loot_generation_context_data: Vec<Option<LocationLootGenerationContextData>>,
    pub lifetime_override_hierarchical_data: Vec<Option<LifetimeOverrideHierarchicalData>>,
    pub geometry_base_hierarchical_data: Vec<Option<GeometryBaseHierarchicalData>>,
    pub hierarchical_contextual_data_preset_record:
        Vec<Option<HierarchicalContextualDataPresetRecord>>,
}
