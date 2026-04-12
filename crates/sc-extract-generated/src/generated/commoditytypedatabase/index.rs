// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `commoditytypedatabase` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommoditytypedatabaseIndex {
    #[serde(default)]
    pub commodity_type: HashMap<CigGuid, Handle<CommodityType>>,
    #[serde(default)]
    pub commodity_subtype: HashMap<CigGuid, Handle<CommoditySubtype>>,
    #[serde(default)]
    pub commodity_type_database: HashMap<CigGuid, Handle<CommodityTypeDatabase>>,
}

impl CommoditytypedatabaseIndex {
    pub fn len(&self) -> usize {
        self.commodity_type.len()
            + self.commodity_subtype.len()
            + self.commodity_type_database.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
