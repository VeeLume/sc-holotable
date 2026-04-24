// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `commoditytypedatabase`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `CommodityTypeDatabase`
pub struct CommodityTypeDatabase {
    /// `types` (Reference (array))
    pub types: Vec<CigGuid>,
    /// `subtypes` (Reference (array))
    pub subtypes: Vec<CigGuid>,
}

impl Pooled for CommodityTypeDatabase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.commoditytypedatabase.commodity_type_database
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.commoditytypedatabase.commodity_type_database
    }
}

impl<'a> Extract<'a> for CommodityTypeDatabase {
    const TYPE_NAME: &'static str = "CommodityTypeDatabase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            types: inst
                .get_array("types")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            subtypes: inst
                .get_array("subtypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
