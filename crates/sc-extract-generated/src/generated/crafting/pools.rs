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

/// Pool storage for the `crafting` feature.
#[derive(Default)]
pub struct CraftingPools {
    pub legacy_crafting_output_resource_amount: Vec<Option<LegacyCraftingOutput_ResourceAmount>>,
    pub legacy_crafting_recipe_def_direct: Vec<Option<LegacyCraftingRecipeDef_Direct>>,
    pub blueprint_category_database_record: Vec<Option<BlueprintCategoryDatabaseRecord>>,
    pub generic_crafting_process_dismantle: Vec<Option<GenericCraftingProcess_Dismantle>>,
    pub generic_crafting_blueprint: Vec<Option<GenericCraftingBlueprint>>,
    pub default_blueprint_selection_whitelist: Vec<Option<DefaultBlueprintSelection_Whitelist>>,
    pub crafting_global_params: Vec<Option<CraftingGlobalParams>>,
}
