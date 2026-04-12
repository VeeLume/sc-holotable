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

/// Pool storage for the `harvestable` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HarvestablePools {
    #[serde(default)]
    pub optional_probability: Vec<Option<OptionalProbability>>,
    #[serde(default)]
    pub sub_harvestable_config: Vec<Option<SubHarvestableConfig>>,
    #[serde(default)]
    pub sub_harvestable_config_record: Vec<Option<SubHarvestableConfigRecord>>,
    #[serde(default)]
    pub harvestable_tag_list_base: Vec<Option<HarvestableTagListBase>>,
    #[serde(default)]
    pub tagged_sub_harvestable_config: Vec<Option<TaggedSubHarvestableConfig>>,
    #[serde(default)]
    pub sub_harvestable_multi_config: Vec<Option<SubHarvestableMultiConfig>>,
    #[serde(default)]
    pub sub_harvestable_multi_config_record: Vec<Option<SubHarvestableMultiConfigRecord>>,
    #[serde(default)]
    pub sub_harvestable_config_base: Vec<Option<SubHarvestableConfigBase>>,
    #[serde(default)]
    pub sub_harvestable_config_single_base: Vec<Option<SubHarvestableConfigSingleBase>>,
    #[serde(default)]
    pub sub_harvestable_slot: Vec<Option<SubHarvestableSlot>>,
    #[serde(default)]
    pub harvestable_transform_params: Vec<Option<HarvestableTransformParams>>,
    #[serde(default)]
    pub harvest_condition_base: Vec<Option<HarvestConditionBase>>,
    #[serde(default)]
    pub harvest_despawn_timer_params: Vec<Option<HarvestDespawnTimerParams>>,
    #[serde(default)]
    pub harvest_behaviour_params: Vec<Option<HarvestBehaviourParams>>,
    #[serde(default)]
    pub harvestable_preset: Vec<Option<HarvestablePreset>>,
    #[serde(default)]
    pub harvestable_setup: Vec<Option<HarvestableSetup>>,
    #[serde(default)]
    pub harvestable_cluster_params: Vec<Option<HarvestableClusterParams>>,
    #[serde(default)]
    pub harvestable_cluster_preset: Vec<Option<HarvestableClusterPreset>>,
    #[serde(default)]
    pub harvestable_geometry: Vec<Option<HarvestableGeometry>>,
    #[serde(default)]
    pub harvestable_element: Vec<Option<HarvestableElement>>,
    #[serde(default)]
    pub harvestable_element_group: Vec<Option<HarvestableElementGroup>>,
    #[serde(default)]
    pub harvestable_element_modifier: Vec<Option<HarvestableElementModifier>>,
    #[serde(default)]
    pub harvestable_area_type_base: Vec<Option<HarvestableAreaTypeBase>>,
    #[serde(default)]
    pub harvestable_area_preset: Vec<Option<HarvestableAreaPreset>>,
    #[serde(default)]
    pub harvestable_provider_preset: Vec<Option<HarvestableProviderPreset>>,
    #[serde(default)]
    pub float_factor_range: Vec<Option<FloatFactorRange>>,
    #[serde(default)]
    pub advanced_loot_constraints: Vec<Option<AdvancedLootConstraints>>,
    #[serde(default)]
    pub loot_constraints: Vec<Option<LootConstraints>>,
    #[serde(default)]
    pub loot_config: Vec<Option<LootConfig>>,
    #[serde(default)]
    pub loot_v3_secondary_choices_record_ref_base: Vec<Option<LootV3SecondaryChoicesRecordRef_Base>>,
}
