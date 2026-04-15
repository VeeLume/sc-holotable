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

/// Pool storage for the `vehicle` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehiclePools {
    #[serde(default)]
    pub soperator_mode_labels: Vec<Option<SOperatorModeLabels>>,
    #[serde(default)]
    pub smaster_mode_labels: Vec<Option<SMasterModeLabels>>,
    #[serde(default)]
    pub vehicle_career_list: Vec<Option<VehicleCareerList>>,
}
