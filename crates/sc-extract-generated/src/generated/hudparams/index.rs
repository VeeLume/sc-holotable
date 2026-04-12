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

/// Record index for the `hudparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HudparamsIndex {
    #[serde(default)]
    pub starget_selector_hud_params: HashMap<CigGuid, Handle<STargetSelectorHudParams>>,
    #[serde(default)]
    pub sprojected_hud_params: HashMap<CigGuid, Handle<SProjectedHudParams>>,
    #[serde(default)]
    pub svehicle_hud_params: HashMap<CigGuid, Handle<SVehicleHudParams>>,
    #[serde(default)]
    pub saimable_gimbal_mode_labels: HashMap<CigGuid, Handle<SAimableGimbalModeLabels>>,
    #[serde(default)]
    pub saimable_controller_hud_params: HashMap<CigGuid, Handle<SAimableControllerHudParams>>,
}

impl HudparamsIndex {
    pub fn len(&self) -> usize {
        self.starget_selector_hud_params.len()
            + self.sprojected_hud_params.len()
            + self.svehicle_hud_params.len()
            + self.saimable_gimbal_mode_labels.len()
            + self.saimable_controller_hud_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
