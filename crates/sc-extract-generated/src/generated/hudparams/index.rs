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

/// Record index for the `hudparams` feature.
#[derive(Default)]
pub struct HudparamsIndex {
    pub starget_selector_hud_params: HashMap<CigGuid, Handle<STargetSelectorHudParams>>,
    pub sprojected_hud_params: HashMap<CigGuid, Handle<SProjectedHudParams>>,
    pub svehicle_hud_params: HashMap<CigGuid, Handle<SVehicleHudParams>>,
    pub saimable_gimbal_mode_labels: HashMap<CigGuid, Handle<SAimableGimbalModeLabels>>,
    pub saimable_controller_hud_params: HashMap<CigGuid, Handle<SAimableControllerHudParams>>,
}

impl HudparamsIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.starget_selector_hud_params.len();
        total += self.sprojected_hud_params.len();
        total += self.svehicle_hud_params.len();
        total += self.saimable_gimbal_mode_labels.len();
        total += self.saimable_controller_hud_params.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
