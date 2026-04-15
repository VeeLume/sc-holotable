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

/// Pool storage for the `turret` feature.
#[derive(Default)]
pub struct TurretPools {
    pub sturret_health_modifier_def: Vec<Option<STurretHealthModifierDef>>,
    pub sturret_esp: Vec<Option<STurretESP>>,
    pub sturret_global_params: Vec<Option<STurretGlobalParams>>,
}
