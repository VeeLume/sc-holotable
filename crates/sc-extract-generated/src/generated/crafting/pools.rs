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

/// Pool storage for the `crafting` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CraftingPools {
    #[serde(default)]
    pub legacy_crafting_recipe_base: Vec<Option<LegacyCraftingRecipe_Base>>,
    #[serde(default)]
    pub legacy_crafting_recipe_def_base: Vec<Option<LegacyCraftingRecipeDef_Base>>,
    #[serde(default)]
    pub legacy_crafting_recipe_def_record: Vec<Option<LegacyCraftingRecipeDefRecord>>,
    #[serde(default)]
    pub legacy_crafting_recipe_list_record: Vec<Option<LegacyCraftingRecipeListRecord>>,
    #[serde(default)]
    pub crafting_gameplay_property_def: Vec<Option<CraftingGameplayPropertyDef>>,
    #[serde(default)]
    pub blueprint_category_record: Vec<Option<BlueprintCategoryRecord>>,
    #[serde(default)]
    pub blueprint_category_database_record: Vec<Option<BlueprintCategoryDatabaseRecord>>,
    #[serde(default)]
    pub crafting_blueprint_base_non_ref: Vec<Option<CraftingBlueprint_Base_NonRef>>,
    #[serde(default)]
    pub crafting_blueprint_record: Vec<Option<CraftingBlueprintRecord>>,
    #[serde(default)]
    pub default_blueprint_selection_base: Vec<Option<DefaultBlueprintSelection_Base>>,
    #[serde(default)]
    pub crafting_quality_distribution_base_non_ref: Vec<Option<CraftingQualityDistribution_Base_NonRef>>,
    #[serde(default)]
    pub crafting_quality_distribution_record: Vec<Option<CraftingQualityDistributionRecord>>,
    #[serde(default)]
    pub crafting_quality_location_override_base_non_ref: Vec<Option<CraftingQualityLocationOverride_Base_NonRef>>,
    #[serde(default)]
    pub crafting_quality_location_override_record: Vec<Option<CraftingQualityLocationOverrideRecord>>,
    #[serde(default)]
    pub crafting_global_params: Vec<Option<CraftingGlobalParams>>,
    #[serde(default)]
    pub blueprint_reward: Vec<Option<BlueprintReward>>,
    #[serde(default)]
    pub blueprint_pool_record: Vec<Option<BlueprintPoolRecord>>,
}
