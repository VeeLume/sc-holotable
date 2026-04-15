// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `longtermpersistence`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `LongTermPersistenceWhiteListSubTypeEntry`
pub struct LongTermPersistenceWhiteListSubTypeEntry {
    /// `ItemSubType` (EnumChoice)
    pub item_sub_type: EItemSubType,
    /// `NotRemove` (Boolean)
    pub not_remove: bool,
}

impl Pooled for LongTermPersistenceWhiteListSubTypeEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.longtermpersistence.long_term_persistence_white_list_sub_type_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.longtermpersistence.long_term_persistence_white_list_sub_type_entry }
}

impl<'a> Extract<'a> for LongTermPersistenceWhiteListSubTypeEntry {
    const TYPE_NAME: &'static str = "LongTermPersistenceWhiteListSubTypeEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_sub_type: EItemSubType::from_dcb_str(inst.get_str("ItemSubType").unwrap_or("")),
            not_remove: inst.get_bool("NotRemove").unwrap_or_default(),
        }
    }
}

/// DCB type: `LongTermPersistenceSubTypeAll`
/// Inherits from: `LongTermPersistenceSubTypeListOption`
pub struct LongTermPersistenceSubTypeAll {
    /// `NotRemove` (Boolean)
    pub not_remove: bool,
}

impl Pooled for LongTermPersistenceSubTypeAll {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.longtermpersistence.long_term_persistence_sub_type_all }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.longtermpersistence.long_term_persistence_sub_type_all }
}

impl<'a> Extract<'a> for LongTermPersistenceSubTypeAll {
    const TYPE_NAME: &'static str = "LongTermPersistenceSubTypeAll";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            not_remove: inst.get_bool("NotRemove").unwrap_or_default(),
        }
    }
}

/// DCB type: `LongTermPersistenceSubTypeList`
/// Inherits from: `LongTermPersistenceSubTypeListOption`
pub struct LongTermPersistenceSubTypeList {
    /// `ItemSubTypeEntries` (Class (array))
    pub item_sub_type_entries: Vec<Handle<LongTermPersistenceWhiteListSubTypeEntry>>,
}

impl Pooled for LongTermPersistenceSubTypeList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.longtermpersistence.long_term_persistence_sub_type_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.longtermpersistence.long_term_persistence_sub_type_list }
}

impl<'a> Extract<'a> for LongTermPersistenceSubTypeList {
    const TYPE_NAME: &'static str = "LongTermPersistenceSubTypeList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_sub_type_entries: inst.get_array("ItemSubTypeEntries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LongTermPersistenceWhiteListSubTypeEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<LongTermPersistenceWhiteListSubTypeEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LongTermPersistenceWhiteListEntry`
pub struct LongTermPersistenceWhiteListEntry {
    /// `ItemType` (EnumChoice)
    pub item_type: EItemType,
    /// `SubTypeListOption` (StrongPointer)
    pub sub_type_list_option: Option<LongTermPersistenceSubTypeListOptionPtr>,
}

impl Pooled for LongTermPersistenceWhiteListEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.longtermpersistence.long_term_persistence_white_list_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.longtermpersistence.long_term_persistence_white_list_entry }
}

impl<'a> Extract<'a> for LongTermPersistenceWhiteListEntry {
    const TYPE_NAME: &'static str = "LongTermPersistenceWhiteListEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_type: EItemType::from_dcb_str(inst.get_str("ItemType").unwrap_or("")),
            sub_type_list_option: match inst.get("SubTypeListOption") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(LongTermPersistenceSubTypeListOptionPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LongTermPersistenceGlobalParams`
pub struct LongTermPersistenceGlobalParams {
    /// `LongTermPersistenceWhiteList` (Class (array))
    pub long_term_persistence_white_list: Vec<Handle<LongTermPersistenceWhiteListEntry>>,
}

impl Pooled for LongTermPersistenceGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.longtermpersistence.long_term_persistence_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.longtermpersistence.long_term_persistence_global_params }
}

impl<'a> Extract<'a> for LongTermPersistenceGlobalParams {
    const TYPE_NAME: &'static str = "LongTermPersistenceGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            long_term_persistence_white_list: inst.get_array("LongTermPersistenceWhiteList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LongTermPersistenceWhiteListEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<LongTermPersistenceWhiteListEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

