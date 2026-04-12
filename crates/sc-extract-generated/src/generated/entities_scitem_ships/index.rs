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

/// Record index for the `entities-scitem-ships` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemShipsIndex {
    #[serde(default)]
    pub sifcsmodifiers_legacy: HashMap<CigGuid, Handle<SIFCSModifiersLegacy>>,
    #[serde(default)]
    pub jump_drive_flight_params: HashMap<CigGuid, Handle<JumpDriveFlightParams>>,
    #[serde(default)]
    pub jump_tunnel_forces_params: HashMap<CigGuid, Handle<JumpTunnelForcesParams>>,
    #[serde(default)]
    pub scseat_head_pos_adjust_setup: HashMap<CigGuid, Handle<SCSeatHeadPosAdjustSetup>>,
    #[serde(default)]
    pub smfdmode_config: HashMap<CigGuid, Handle<SMFDModeConfig>>,
    #[serde(default)]
    pub smfdview: HashMap<CigGuid, Handle<SMFDView>>,
    #[serde(default)]
    pub smfdview_list: HashMap<CigGuid, Handle<SMFDViewList>>,
    #[serde(default)]
    pub smfdparams_diagnostics: HashMap<CigGuid, Handle<SMFDParamsDiagnostics>>,
    #[serde(default)]
    pub weapon_aimable_angles_def: HashMap<CigGuid, Handle<WeaponAimableAnglesDef>>,
    #[serde(default)]
    pub weapon_gimbal_mode_modifier_def: HashMap<CigGuid, Handle<WeaponGimbalModeModifierDef>>,
    #[serde(default)]
    pub vehicle_landing_gear_system: HashMap<CigGuid, Handle<VehicleLandingGearSystem>>,
}

impl EntitiesScitemShipsIndex {
    pub fn len(&self) -> usize {
        self.sifcsmodifiers_legacy.len()
            + self.jump_drive_flight_params.len()
            + self.jump_tunnel_forces_params.len()
            + self.scseat_head_pos_adjust_setup.len()
            + self.smfdmode_config.len()
            + self.smfdview.len()
            + self.smfdview_list.len()
            + self.smfdparams_diagnostics.len()
            + self.weapon_aimable_angles_def.len()
            + self.weapon_gimbal_mode_modifier_def.len()
            + self.vehicle_landing_gear_system.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
