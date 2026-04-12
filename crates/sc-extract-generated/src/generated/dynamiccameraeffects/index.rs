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

/// Record index for the `dynamiccameraeffects` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DynamiccameraeffectsIndex {
    #[serde(default)]
    pub dynamic_camera_effects: HashMap<CigGuid, Handle<DynamicCameraEffects>>,
    #[serde(default)]
    pub dynamic_camera_effects_list: HashMap<CigGuid, Handle<DynamicCameraEffectsList>>,
    #[serde(default)]
    pub constant_dofglobal_data: HashMap<CigGuid, Handle<ConstantDOFGlobalData>>,
}

impl DynamiccameraeffectsIndex {
    pub fn len(&self) -> usize {
        self.dynamic_camera_effects.len()
            + self.dynamic_camera_effects_list.len()
            + self.constant_dofglobal_data.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
