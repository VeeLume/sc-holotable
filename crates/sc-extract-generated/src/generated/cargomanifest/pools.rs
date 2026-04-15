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

/// Pool storage for the `cargomanifest` feature.
#[derive(Default)]
pub struct CargomanifestPools {
    pub cargo_fill_capacity_value_random: Vec<Option<CargoFillCapacityValue_Random>>,
    pub cargo_fill_capacity_value_custom: Vec<Option<CargoFillCapacityValue_Custom>>,
    pub cargo_resource_allocation: Vec<Option<CargoResourceAllocation>>,
}
