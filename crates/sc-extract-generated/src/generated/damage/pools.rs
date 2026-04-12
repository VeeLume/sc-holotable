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

/// Pool storage for the `damage` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DamagePools {
    #[serde(default)]
    pub damage_macro: Vec<Option<DamageMacro>>,
    #[serde(default)]
    pub damage_resistance_entry: Vec<Option<DamageResistanceEntry>>,
    #[serde(default)]
    pub damage_resistance: Vec<Option<DamageResistance>>,
    #[serde(default)]
    pub impact_force_resistance: Vec<Option<ImpactForceResistance>>,
    #[serde(default)]
    pub damage_resistance_macro: Vec<Option<DamageResistanceMacro>>,
}
