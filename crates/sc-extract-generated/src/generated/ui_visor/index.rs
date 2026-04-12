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

/// Record index for the `ui-visor` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiVisorIndex {
    #[serde(default)]
    pub visor_hud_config: HashMap<CigGuid, Handle<VisorHUD_Config>>,
    #[serde(default)]
    pub visor_control_hints_config: HashMap<CigGuid, Handle<Visor_ControlHintsConfig>>,
    #[serde(default)]
    pub control_hints_preset: HashMap<CigGuid, Handle<ControlHints_Preset>>,
}

impl UiVisorIndex {
    pub fn len(&self) -> usize {
        self.visor_hud_config.len()
            + self.visor_control_hints_config.len()
            + self.control_hints_preset.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
