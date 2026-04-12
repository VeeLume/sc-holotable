// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `vehicle` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleIndex {
    #[serde(default)]
    pub soperator_mode_labels: HashMap<CigGuid, Handle<SOperatorModeLabels>>,
    #[serde(default)]
    pub smaster_mode_labels: HashMap<CigGuid, Handle<SMasterModeLabels>>,
    #[serde(default)]
    pub vehicle_serial_number_character_type: HashMap<CigGuid, Handle<VehicleSerialNumberCharacterType>>,
    #[serde(default)]
    pub vehicle_serial_number_format: HashMap<CigGuid, Handle<VehicleSerialNumberFormat>>,
    #[serde(default)]
    pub vehicle_role: HashMap<CigGuid, Handle<VehicleRole>>,
    #[serde(default)]
    pub vehicle_career: HashMap<CigGuid, Handle<VehicleCareer>>,
    #[serde(default)]
    pub vehicle_career_list: HashMap<CigGuid, Handle<VehicleCareerList>>,
}

impl VehicleIndex {
    pub fn len(&self) -> usize {
        self.soperator_mode_labels.len()
            + self.smaster_mode_labels.len()
            + self.vehicle_serial_number_character_type.len()
            + self.vehicle_serial_number_format.len()
            + self.vehicle_role.len()
            + self.vehicle_career.len()
            + self.vehicle_career_list.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
