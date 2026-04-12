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

/// Pool storage for the `consumabletypesdatabase` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsumabletypesdatabasePools {
    #[serde(default)]
    pub consumable_type: Vec<Option<ConsumableType>>,
    #[serde(default)]
    pub consumable_subtype: Vec<Option<ConsumableSubtype>>,
    #[serde(default)]
    pub consumable_type_database: Vec<Option<ConsumableTypeDatabase>>,
    #[serde(default)]
    pub consumable_effect: Vec<Option<ConsumableEffect>>,
}
