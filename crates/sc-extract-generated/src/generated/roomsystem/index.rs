// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `roomsystem` feature.
#[derive(Default)]
pub struct RoomsystemIndex {
    pub fire_hazard_global_params: HashMap<CigGuid, Handle<FireHazardGlobalParams>>,
    pub global_gas_params: HashMap<CigGuid, Handle<GlobalGasParams>>,
    pub global_room_state_params: HashMap<CigGuid, Handle<GlobalRoomStateParams>>,
    pub electrical_state_template: HashMap<CigGuid, Handle<ElectricalStateTemplate>>,
    pub electrical_behavior: HashMap<CigGuid, Handle<ElectricalBehavior>>,
}

impl RoomsystemIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.fire_hazard_global_params.len();
        total += self.global_gas_params.len();
        total += self.global_room_state_params.len();
        total += self.electrical_state_template.len();
        total += self.electrical_behavior.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
