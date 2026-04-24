// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;
use crate::Handle;
use std::collections::HashMap;
use svarog_common::CigGuid;

/// Record index for the `globalshopparams` feature.
#[derive(Default)]
pub struct GlobalshopparamsIndex {
    pub global_shop_commodity_params: HashMap<CigGuid, Handle<GlobalShopCommodityParams>>,
    pub global_shop_terminal_params: HashMap<CigGuid, Handle<GlobalShopTerminalParams>>,
    pub global_shop_selling_params: HashMap<CigGuid, Handle<GlobalShopSellingParams>>,
    pub global_shop_buying_params: HashMap<CigGuid, Handle<GlobalShopBuyingParams>>,
}

impl GlobalshopparamsIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.global_shop_commodity_params.len();
        total += self.global_shop_terminal_params.len();
        total += self.global_shop_selling_params.len();
        total += self.global_shop_buying_params.len();
        total
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
