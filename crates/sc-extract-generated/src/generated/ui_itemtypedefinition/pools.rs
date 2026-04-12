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

/// Pool storage for the `ui-itemtypedefinition` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiItemtypedefinitionPools {
    #[serde(default)]
    pub item_type_category: Vec<Option<ItemTypeCategory>>,
    #[serde(default)]
    pub item_type_category_exception: Vec<Option<ItemTypeCategoryException>>,
    #[serde(default)]
    pub item_type_category_map: Vec<Option<ItemTypeCategoryMap>>,
    #[serde(default)]
    pub item_type_info: Vec<Option<ItemTypeInfo>>,
    #[serde(default)]
    pub item_type_definition: Vec<Option<ItemTypeDefinition>>,
}
