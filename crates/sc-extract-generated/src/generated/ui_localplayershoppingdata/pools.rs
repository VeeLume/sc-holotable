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

/// Pool storage for the `ui-localplayershoppingdata` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiLocalplayershoppingdataPools {
    #[serde(default)]
    pub sitem_shop_reference: Vec<Option<SItemShopReference>>,
    #[serde(default)]
    pub sitem_shop_arparams: Vec<Option<SItemShopARParams>>,
    #[serde(default)]
    pub slocal_player_shopping_data: Vec<Option<SLocalPlayerShoppingData>>,
    #[serde(default)]
    pub slocal_player_shopping_predefined_arparams: Vec<Option<SLocalPlayerShoppingPredefinedARParams>>,
    #[serde(default)]
    pub slocal_player_shopping_notification_configuration: Vec<Option<SLocalPlayerShoppingNotificationConfiguration>>,
}
