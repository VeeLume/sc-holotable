// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-mastermodeexclusionglobalparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MasterModeExclusion`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterModeExclusion {
    /// `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// `masterModeExclusions` (EnumChoice (array))
    #[serde(default)]
    pub master_mode_exclusions: Vec<String>,
}

impl Pooled for MasterModeExclusion {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mastermodeexclusionglobalparams.master_mode_exclusion }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mastermodeexclusionglobalparams.master_mode_exclusion }
}

impl<'a> Extract<'a> for MasterModeExclusion {
    const TYPE_NAME: &'static str = "MasterModeExclusion";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("itemType").map(String::from).unwrap_or_default(),
            master_mode_exclusions: inst.get_array("masterModeExclusions")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MasterModeExclusionGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterModeExclusionGlobalParams {
    /// `exclusions` (Class (array))
    #[serde(default)]
    pub exclusions: Vec<Handle<MasterModeExclusion>>,
}

impl Pooled for MasterModeExclusionGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mastermodeexclusionglobalparams.master_mode_exclusion_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mastermodeexclusionglobalparams.master_mode_exclusion_global_params }
}

impl<'a> Extract<'a> for MasterModeExclusionGlobalParams {
    const TYPE_NAME: &'static str = "MasterModeExclusionGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            exclusions: inst.get_array("exclusions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MasterModeExclusion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MasterModeExclusion>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

