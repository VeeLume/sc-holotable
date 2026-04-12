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

/// Pool storage for the `vehicle` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehiclePools {
    #[serde(default)]
    pub soperator_mode_labels: Vec<Option<SOperatorModeLabels>>,
    #[serde(default)]
    pub smaster_mode_labels: Vec<Option<SMasterModeLabels>>,
    #[serde(default)]
    pub vehicle_serial_number_character_type: Vec<Option<VehicleSerialNumberCharacterType>>,
    #[serde(default)]
    pub vehicle_serial_number_format: Vec<Option<VehicleSerialNumberFormat>>,
    #[serde(default)]
    pub vehicle_role: Vec<Option<VehicleRole>>,
    #[serde(default)]
    pub vehicle_career: Vec<Option<VehicleCareer>>,
    #[serde(default)]
    pub vehicle_career_list: Vec<Option<VehicleCareerList>>,
}
