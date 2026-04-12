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

/// Record index for the `ui-radar3dpresets` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiRadar3dpresetsIndex {
    #[serde(default)]
    pub radar_display3_dpreset: HashMap<CigGuid, Handle<RadarDisplay3DPreset>>,
}

impl UiRadar3dpresetsIndex {
    pub fn len(&self) -> usize {
        self.radar_display3_dpreset.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
