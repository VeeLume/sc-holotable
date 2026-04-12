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

/// Record index for the `globalshopparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalshopparamsIndex {
    #[serde(default)]
    pub global_shop_commodity_params: HashMap<CigGuid, Handle<GlobalShopCommodityParams>>,
    #[serde(default)]
    pub global_shop_terminal_params: HashMap<CigGuid, Handle<GlobalShopTerminalParams>>,
    #[serde(default)]
    pub global_shop_selling_params: HashMap<CigGuid, Handle<GlobalShopSellingParams>>,
    #[serde(default)]
    pub global_shop_buying_params: HashMap<CigGuid, Handle<GlobalShopBuyingParams>>,
}

impl GlobalshopparamsIndex {
    pub fn len(&self) -> usize {
        self.global_shop_commodity_params.len()
            + self.global_shop_terminal_params.len()
            + self.global_shop_selling_params.len()
            + self.global_shop_buying_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
