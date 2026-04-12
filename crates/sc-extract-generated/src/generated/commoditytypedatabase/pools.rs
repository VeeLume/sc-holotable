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

/// Pool storage for the `commoditytypedatabase` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommoditytypedatabasePools {
    #[serde(default)]
    pub temperature_damage_control: Vec<Option<TemperatureDamageControl>>,
    #[serde(default)]
    pub commodity_temperature_tolerance: Vec<Option<CommodityTemperatureTolerance>>,
    #[serde(default)]
    pub damage_resistance_base: Vec<Option<DamageResistanceBase>>,
    #[serde(default)]
    pub commodity_type: Vec<Option<CommodityType>>,
    #[serde(default)]
    pub commodity_subtype: Vec<Option<CommoditySubtype>>,
    #[serde(default)]
    pub commodity_type_database: Vec<Option<CommodityTypeDatabase>>,
}
