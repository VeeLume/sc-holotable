// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `turret` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TurretPools {
    #[serde(default)]
    pub sturret_health_modifier_def: Vec<Option<STurretHealthModifierDef>>,
    #[serde(default)]
    pub sturret_esp: Vec<Option<STurretESP>>,
    #[serde(default)]
    pub sturret_global_params: Vec<Option<STurretGlobalParams>>,
}
