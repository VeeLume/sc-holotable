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

/// Pool storage for the `entities-scitem-ships` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemShipsPools {
    #[serde(default)]
    pub sifcsmodifier_number: Vec<Option<SIFCSModifierNumber>>,
    #[serde(default)]
    pub sifcsmodifier_vector: Vec<Option<SIFCSModifierVector>>,
    #[serde(default)]
    pub sifcsmodifiers_legacy: Vec<Option<SIFCSModifiersLegacy>>,
    #[serde(default)]
    pub sjump_drive_flight_rotation_params: Vec<Option<SJumpDriveFlightRotationParams>>,
    #[serde(default)]
    pub sjump_drive_flight_linear_params: Vec<Option<SJumpDriveFlightLinearParams>>,
    #[serde(default)]
    pub sjump_drive_flight_steering_params: Vec<Option<SJumpDriveFlightSteeringParams>>,
    #[serde(default)]
    pub sjump_drive_flight_turbulence_noise_params: Vec<Option<SJumpDriveFlightTurbulenceNoiseParams>>,
    #[serde(default)]
    pub sjump_drive_flight_turbulence_params: Vec<Option<SJumpDriveFlightTurbulenceParams>>,
    #[serde(default)]
    pub jump_drive_flight_params: Vec<Option<JumpDriveFlightParams>>,
    #[serde(default)]
    pub jump_tunnel_forces_params: Vec<Option<JumpTunnelForcesParams>>,
    #[serde(default)]
    pub vehicle_landing_gear_spring: Vec<Option<VehicleLandingGearSpring>>,
    #[serde(default)]
    pub scseat_head_pos_adjust_setup: Vec<Option<SCSeatHeadPosAdjustSetup>>,
    #[serde(default)]
    pub smfdoperator_mode_config: Vec<Option<SMFDOperatorModeConfig>>,
    #[serde(default)]
    pub smfdview_exception: Vec<Option<SMFDViewException>>,
    #[serde(default)]
    pub smfdmaster_mode_view_exceptions: Vec<Option<SMFDMasterModeViewExceptions>>,
    #[serde(default)]
    pub smfdmode_config: Vec<Option<SMFDModeConfig>>,
    #[serde(default)]
    pub smanufacturer_mfdview: Vec<Option<SManufacturerMFDView>>,
    #[serde(default)]
    pub smfdview: Vec<Option<SMFDView>>,
    #[serde(default)]
    pub smaster_mode_mfdview_list: Vec<Option<SMasterModeMFDViewList>>,
    #[serde(default)]
    pub smfdview_list: Vec<Option<SMFDViewList>>,
    #[serde(default)]
    pub smfdparams_diagnostics: Vec<Option<SMFDParamsDiagnostics>>,
    #[serde(default)]
    pub weapon_aimable_angles_def: Vec<Option<WeaponAimableAnglesDef>>,
    #[serde(default)]
    pub weapon_gimbal_mode_modifier_def: Vec<Option<WeaponGimbalModeModifierDef>>,
    #[serde(default)]
    pub vehicle_landing_gear: Vec<Option<VehicleLandingGear>>,
    #[serde(default)]
    pub vehicle_landing_gear_system: Vec<Option<VehicleLandingGearSystem>>,
    #[serde(default)]
    pub svehicle_afterburner_params: Vec<Option<SVehicleAfterburnerParams>>,
}
