// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-itemtypedefinition`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ItemTypeCategory`
pub struct ItemTypeCategory {
    /// `name` (Locale)
    pub name: LocaleKey,
}

impl Pooled for ItemTypeCategory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itemtypedefinition.item_type_category }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itemtypedefinition.item_type_category }
}

impl<'a> Extract<'a> for ItemTypeCategory {
    const TYPE_NAME: &'static str = "ItemTypeCategory";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemTypeCategoryException`
pub struct ItemTypeCategoryException {
    /// `subType` (EnumChoice)
    pub sub_type: EItemSubType,
    /// `category` (WeakPointer)
    pub category: Option<Handle<ItemTypeCategory>>,
    /// `showInElectronicAccess` (Boolean)
    pub show_in_electronic_access: bool,
}

impl Pooled for ItemTypeCategoryException {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itemtypedefinition.item_type_category_exception }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itemtypedefinition.item_type_category_exception }
}

impl<'a> Extract<'a> for ItemTypeCategoryException {
    const TYPE_NAME: &'static str = "ItemTypeCategoryException";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sub_type: EItemSubType::from_dcb_str(inst.get_str("subType").unwrap_or("")),
            category: match inst.get("category") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemTypeCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            show_in_electronic_access: inst.get_bool("showInElectronicAccess").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemTypeCategoryMap`
pub struct ItemTypeCategoryMap {
    /// `defaultCategory` (WeakPointer)
    pub default_category: Option<Handle<ItemTypeCategory>>,
    /// `exceptions` (Class (array))
    pub exceptions: Vec<Handle<ItemTypeCategoryException>>,
    /// `showInElectronicAccess` (Boolean)
    pub show_in_electronic_access: bool,
}

impl Pooled for ItemTypeCategoryMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itemtypedefinition.item_type_category_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itemtypedefinition.item_type_category_map }
}

impl<'a> Extract<'a> for ItemTypeCategoryMap {
    const TYPE_NAME: &'static str = "ItemTypeCategoryMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_category: match inst.get("defaultCategory") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemTypeCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exceptions: inst.get_array("exceptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemTypeCategoryException>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ItemTypeCategoryException>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            show_in_electronic_access: inst.get_bool("showInElectronicAccess").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemTypeInfo`
pub struct ItemTypeInfo {
    /// `typeName` (Locale)
    pub type_name: LocaleKey,
    /// `showInPlayerAssetManager` (Boolean)
    pub show_in_player_asset_manager: bool,
    /// `categoryMap` (Class)
    pub category_map: Option<Handle<ItemTypeCategoryMap>>,
}

impl Pooled for ItemTypeInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itemtypedefinition.item_type_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itemtypedefinition.item_type_info }
}

impl<'a> Extract<'a> for ItemTypeInfo {
    const TYPE_NAME: &'static str = "ItemTypeInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(LocaleKey::from).unwrap_or_default(),
            show_in_player_asset_manager: inst.get_bool("showInPlayerAssetManager").unwrap_or_default(),
            category_map: match inst.get("categoryMap") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemTypeCategoryMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemTypeDefinition`
pub struct ItemTypeDefinition {
    /// `defaultCategory` (WeakPointer)
    pub default_category: Option<Handle<ItemTypeCategory>>,
    /// `categories` (Class (array))
    pub categories: Vec<Handle<ItemTypeCategory>>,
    /// `typeInfo` (Class)
    pub type_info: Option<Handle<ItemTypeInfo>>,
    /// `allCategoriesLabel` (Locale)
    pub all_categories_label: LocaleKey,
    /// `allTypesLabel` (Locale)
    pub all_types_label: LocaleKey,
}

impl Pooled for ItemTypeDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_itemtypedefinition.item_type_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_itemtypedefinition.item_type_definition }
}

impl<'a> Extract<'a> for ItemTypeDefinition {
    const TYPE_NAME: &'static str = "ItemTypeDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_category: match inst.get("defaultCategory") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemTypeCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            categories: inst.get_array("categories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemTypeCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ItemTypeCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            type_info: match inst.get("typeInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemTypeInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            all_categories_label: inst.get_str("allCategoriesLabel").map(LocaleKey::from).unwrap_or_default(),
            all_types_label: inst.get_str("allTypesLabel").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

