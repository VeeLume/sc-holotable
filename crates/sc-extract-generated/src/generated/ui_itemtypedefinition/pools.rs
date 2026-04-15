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

/// Pool storage for the `ui-itemtypedefinition` feature.
#[derive(Default)]
pub struct UiItemtypedefinitionPools {
    pub item_type_category: Vec<Option<ItemTypeCategory>>,
    pub item_type_category_exception: Vec<Option<ItemTypeCategoryException>>,
    pub item_type_category_map: Vec<Option<ItemTypeCategoryMap>>,
    pub item_type_info: Vec<Option<ItemTypeInfo>>,
    pub item_type_definition: Vec<Option<ItemTypeDefinition>>,
}
