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
use crate::Handle;
use std::collections::HashMap;
use svarog_common::CigGuid;

/// Record index for the `servicebeacon` feature.
#[derive(Default)]
pub struct ServicebeaconIndex {
    pub beacons_contracts: HashMap<CigGuid, Handle<BeaconsContracts>>,
    pub service_beacon_params: HashMap<CigGuid, Handle<ServiceBeaconParams>>,
    pub service_beacon_global_params: HashMap<CigGuid, Handle<ServiceBeaconGlobalParams>>,
}

impl ServicebeaconIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.beacons_contracts.len();
        total += self.service_beacon_params.len();
        total += self.service_beacon_global_params.len();
        total
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
