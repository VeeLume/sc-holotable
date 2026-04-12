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

/// Record index for the `turret` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TurretIndex {
    #[serde(default)]
    pub sturret_health_modifier_def: HashMap<CigGuid, Handle<STurretHealthModifierDef>>,
    #[serde(default)]
    pub sturret_esp: HashMap<CigGuid, Handle<STurretESP>>,
    #[serde(default)]
    pub sturret_global_params: HashMap<CigGuid, Handle<STurretGlobalParams>>,
}

impl TurretIndex {
    pub fn len(&self) -> usize {
        self.sturret_health_modifier_def.len()
            + self.sturret_esp.len()
            + self.sturret_global_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
