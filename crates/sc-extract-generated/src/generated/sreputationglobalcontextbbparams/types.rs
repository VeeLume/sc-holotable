// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `sreputationglobalcontextbbparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SReputationContextBBEntityListParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationContextBBEntityListParams {
    /// `entityTabName` (Locale)
    #[serde(default)]
    pub entity_tab_name: String,
    /// `entityType` (EnumChoice)
    #[serde(default)]
    pub entity_type: String,
}

impl Pooled for SReputationContextBBEntityListParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sreputationglobalcontextbbparams.sreputation_context_bbentity_list_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sreputationglobalcontextbbparams.sreputation_context_bbentity_list_params }
}

impl<'a> Extract<'a> for SReputationContextBBEntityListParams {
    const TYPE_NAME: &'static str = "SReputationContextBBEntityListParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_tab_name: inst.get_str("entityTabName").map(String::from).unwrap_or_default(),
            entity_type: inst.get_str("entityType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationGlobalContextBBParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationGlobalContextBBParams {
    /// `infoTabNames` (Locale (array))
    #[serde(default)]
    pub info_tab_names: Vec<String>,
    /// `entityTabs` (Class (array))
    #[serde(default)]
    pub entity_tabs: Vec<Handle<SReputationContextBBEntityListParams>>,
    /// `entitySortOrder` (EnumChoice)
    #[serde(default)]
    pub entity_sort_order: String,
}

impl Pooled for SReputationGlobalContextBBParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sreputationglobalcontextbbparams.sreputation_global_context_bbparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sreputationglobalcontextbbparams.sreputation_global_context_bbparams }
}

impl<'a> Extract<'a> for SReputationGlobalContextBBParams {
    const TYPE_NAME: &'static str = "SReputationGlobalContextBBParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            info_tab_names: inst.get_array("infoTabNames")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            entity_tabs: inst.get_array("entityTabs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationContextBBEntityListParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationContextBBEntityListParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            entity_sort_order: inst.get_str("entitySortOrder").map(String::from).unwrap_or_default(),
        }
    }
}

