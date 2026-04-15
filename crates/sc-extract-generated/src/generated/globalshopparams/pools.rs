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

/// Pool storage for the `globalshopparams` feature.
#[derive(Default)]
pub struct GlobalshopparamsPools {
    pub sauto_loading_box_size_prices: Vec<Option<SAutoLoadingBoxSizePrices>>,
    pub global_shop_commodity_params: Vec<Option<GlobalShopCommodityParams>>,
    pub global_shop_terminal_params: Vec<Option<GlobalShopTerminalParams>>,
    pub sglobal_shop_errors: Vec<Option<SGlobalShopErrors>>,
    pub item_type_modifier: Vec<Option<ItemTypeModifier>>,
    pub global_shop_selling_params: Vec<Option<GlobalShopSellingParams>>,
    pub licensed_item_modifier: Vec<Option<LicensedItemModifier>>,
    pub global_shop_buying_params: Vec<Option<GlobalShopBuyingParams>>,
}
