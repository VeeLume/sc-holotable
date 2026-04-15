// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `commoditytypedatabase` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommoditytypedatabasePools {
    #[serde(default)]
    pub commodity_type_database: Vec<Option<CommodityTypeDatabase>>,
}
