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

/// Pool storage for the `harvestable` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HarvestablePools {
    #[serde(default)]
    pub harvestable_tag_list_tag_editor: Vec<Option<HarvestableTagListTagEditor>>,
    #[serde(default)]
    pub sub_harvestable_multi_config_record: Vec<Option<SubHarvestableMultiConfigRecord>>,
    #[serde(default)]
    pub sub_harvestable_config_single_ref: Vec<Option<SubHarvestableConfigSingleRef>>,
    #[serde(default)]
    pub harvest_condition_damage_map: Vec<Option<HarvestConditionDamageMap>>,
    #[serde(default)]
    pub harvestable_setup: Vec<Option<HarvestableSetup>>,
    #[serde(default)]
    pub harvestable_cluster_params: Vec<Option<HarvestableClusterParams>>,
    #[serde(default)]
    pub harvestable_cluster_preset: Vec<Option<HarvestableClusterPreset>>,
    #[serde(default)]
    pub harvestable_element: Vec<Option<HarvestableElement>>,
    #[serde(default)]
    pub harvestable_element_group: Vec<Option<HarvestableElementGroup>>,
    #[serde(default)]
    pub harvestable_element_modifier: Vec<Option<HarvestableElementModifier>>,
    #[serde(default)]
    pub harvestable_area_type_object_preset: Vec<Option<HarvestableAreaTypeObjectPreset>>,
    #[serde(default)]
    pub harvestable_area_preset: Vec<Option<HarvestableAreaPreset>>,
    #[serde(default)]
    pub harvestable_provider_preset: Vec<Option<HarvestableProviderPreset>>,
}
