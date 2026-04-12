// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `TypeSubtypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeSubtypeParams {
    /// DCB field: `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// DCB field: `itemSubType` (EnumChoice)
    #[serde(default)]
    pub item_sub_type: String,
}

impl Pooled for TypeSubtypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ty.type_subtype_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ty.type_subtype_params }
}

impl<'a> Extract<'a> for TypeSubtypeParams {
    const TYPE_NAME: &'static str = "TypeSubtypeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("itemType").map(String::from).unwrap_or_default(),
            item_sub_type: inst.get_str("itemSubType").map(String::from).unwrap_or_default(),
        }
    }
}

