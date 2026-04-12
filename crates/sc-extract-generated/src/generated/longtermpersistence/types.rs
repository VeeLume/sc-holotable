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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `LongTermPersistenceSubTypeListOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermPersistenceSubTypeListOption {
}

impl Pooled for LongTermPersistenceSubTypeListOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.longtermpersistence.long_term_persistence_sub_type_list_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.longtermpersistence.long_term_persistence_sub_type_list_option }
}

impl<'a> Extract<'a> for LongTermPersistenceSubTypeListOption {
    const TYPE_NAME: &'static str = "LongTermPersistenceSubTypeListOption";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `LongTermPersistenceWhiteListEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermPersistenceWhiteListEntry {
    /// `ItemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// `SubTypeListOption` (StrongPointer)
    #[serde(default)]
    pub sub_type_list_option: Option<Handle<LongTermPersistenceSubTypeListOption>>,
}

impl Pooled for LongTermPersistenceWhiteListEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.longtermpersistence.long_term_persistence_white_list_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.longtermpersistence.long_term_persistence_white_list_entry }
}

impl<'a> Extract<'a> for LongTermPersistenceWhiteListEntry {
    const TYPE_NAME: &'static str = "LongTermPersistenceWhiteListEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("ItemType").map(String::from).unwrap_or_default(),
            sub_type_list_option: match inst.get("SubTypeListOption") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LongTermPersistenceSubTypeListOption>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LongTermPersistenceSubTypeListOption>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `LongTermPersistenceGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermPersistenceGlobalParams {
    /// `LongTermPersistenceWhiteList` (Class (array))
    #[serde(default)]
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
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LongTermPersistenceWhiteListEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

