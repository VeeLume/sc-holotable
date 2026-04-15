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

/// Pool storage for the `harvestable` feature.
#[derive(Default)]
pub struct HarvestablePools {
    pub harvestable_tag_list_tag_editor: Vec<Option<HarvestableTagListTagEditor>>,
    pub sub_harvestable_multi_config_record: Vec<Option<SubHarvestableMultiConfigRecord>>,
    pub sub_harvestable_config_single_ref: Vec<Option<SubHarvestableConfigSingleRef>>,
    pub harvest_condition_damage_map: Vec<Option<HarvestConditionDamageMap>>,
    pub harvestable_setup: Vec<Option<HarvestableSetup>>,
    pub harvestable_cluster_params: Vec<Option<HarvestableClusterParams>>,
    pub harvestable_cluster_preset: Vec<Option<HarvestableClusterPreset>>,
    pub harvestable_element: Vec<Option<HarvestableElement>>,
    pub harvestable_element_group: Vec<Option<HarvestableElementGroup>>,
    pub harvestable_element_modifier: Vec<Option<HarvestableElementModifier>>,
    pub harvestable_area_type_object_preset: Vec<Option<HarvestableAreaTypeObjectPreset>>,
    pub harvestable_area_preset: Vec<Option<HarvestableAreaPreset>>,
    pub harvestable_provider_preset: Vec<Option<HarvestableProviderPreset>>,
}
