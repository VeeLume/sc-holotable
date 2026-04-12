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

/// Record index for the `crafting` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CraftingIndex {
    #[serde(default)]
    pub legacy_crafting_recipe_def_record: HashMap<CigGuid, Handle<LegacyCraftingRecipeDefRecord>>,
    #[serde(default)]
    pub legacy_crafting_recipe_list_record: HashMap<CigGuid, Handle<LegacyCraftingRecipeListRecord>>,
    #[serde(default)]
    pub crafting_gameplay_property_def: HashMap<CigGuid, Handle<CraftingGameplayPropertyDef>>,
    #[serde(default)]
    pub blueprint_category_record: HashMap<CigGuid, Handle<BlueprintCategoryRecord>>,
    #[serde(default)]
    pub blueprint_category_database_record: HashMap<CigGuid, Handle<BlueprintCategoryDatabaseRecord>>,
    #[serde(default)]
    pub crafting_blueprint_record: HashMap<CigGuid, Handle<CraftingBlueprintRecord>>,
    #[serde(default)]
    pub crafting_quality_distribution_record: HashMap<CigGuid, Handle<CraftingQualityDistributionRecord>>,
    #[serde(default)]
    pub crafting_quality_location_override_record: HashMap<CigGuid, Handle<CraftingQualityLocationOverrideRecord>>,
    #[serde(default)]
    pub crafting_global_params: HashMap<CigGuid, Handle<CraftingGlobalParams>>,
    #[serde(default)]
    pub blueprint_pool_record: HashMap<CigGuid, Handle<BlueprintPoolRecord>>,
}

impl CraftingIndex {
    pub fn len(&self) -> usize {
        self.legacy_crafting_recipe_def_record.len()
            + self.legacy_crafting_recipe_list_record.len()
            + self.crafting_gameplay_property_def.len()
            + self.blueprint_category_record.len()
            + self.blueprint_category_database_record.len()
            + self.crafting_blueprint_record.len()
            + self.crafting_quality_distribution_record.len()
            + self.crafting_quality_location_override_record.len()
            + self.crafting_global_params.len()
            + self.blueprint_pool_record.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
