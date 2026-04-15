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

/// Pool storage for the `areaservices` feature.
#[derive(Default)]
pub struct AreaservicesPools {
    pub base_service: Vec<Option<BaseService>>,
    pub repair_service: Vec<Option<RepairService>>,
    pub quantum_refuel_service: Vec<Option<QuantumRefuelService>>,
    pub hydrogen_refuel_service: Vec<Option<HydrogenRefuelService>>,
    pub restock_service: Vec<Option<RestockService>>,
    pub area_services: Vec<Option<AreaServices>>,
}
