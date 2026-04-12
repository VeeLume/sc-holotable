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

/// Pool storage for the `initialdamageoverrides` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InitialdamageoverridesPools {
    #[serde(default)]
    pub sinitial_damage_specifier_base: Vec<Option<SInitialDamageSpecifierBase>>,
    #[serde(default)]
    pub sinitial_damage: Vec<Option<SInitialDamage>>,
    #[serde(default)]
    pub initial_damage_override: Vec<Option<InitialDamageOverride>>,
}
