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

/// Pool storage for the `vibrations` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VibrationsPools {
    #[serde(default)]
    pub vibration_type_data: Vec<Option<VibrationTypeData>>,
    #[serde(default)]
    pub svibration_def: Vec<Option<SVibrationDef>>,
    #[serde(default)]
    pub svibration_suppression: Vec<Option<SVibrationSuppression>>,
    #[serde(default)]
    pub jump_point_vibration_params: Vec<Option<JumpPointVibrationParams>>,
    #[serde(default)]
    pub jump_drive_vibration_params: Vec<Option<JumpDriveVibrationParams>>,
    #[serde(default)]
    pub jump_tunnel_vibration_params: Vec<Option<JumpTunnelVibrationParams>>,
    #[serde(default)]
    pub jump_system_vibration_params: Vec<Option<JumpSystemVibrationParams>>,
    #[serde(default)]
    pub svibration_reference_data: Vec<Option<SVibrationReferenceData>>,
    #[serde(default)]
    pub svibration_vehicle_def: Vec<Option<SVibrationVehicleDef>>,
}
