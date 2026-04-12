// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `globalshopparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalshopparamsPools {
    #[serde(default)]
    pub sauto_loading_box_size_prices: Vec<Option<SAutoLoadingBoxSizePrices>>,
    #[serde(default)]
    pub global_shop_commodity_params: Vec<Option<GlobalShopCommodityParams>>,
    #[serde(default)]
    pub global_shop_terminal_params: Vec<Option<GlobalShopTerminalParams>>,
    #[serde(default)]
    pub sglobal_shop_errors: Vec<Option<SGlobalShopErrors>>,
    #[serde(default)]
    pub item_type_modifier: Vec<Option<ItemTypeModifier>>,
    #[serde(default)]
    pub global_shop_selling_params: Vec<Option<GlobalShopSellingParams>>,
    #[serde(default)]
    pub licensed_item_modifier: Vec<Option<LicensedItemModifier>>,
    #[serde(default)]
    pub global_shop_buying_params: Vec<Option<GlobalShopBuyingParams>>,
}
