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

/// Record index for the `mining` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MiningIndex {
    #[serde(default)]
    pub mining_global_params: HashMap<CigGuid, Handle<MiningGlobalParams>>,
    #[serde(default)]
    pub mineable_element: HashMap<CigGuid, Handle<MineableElement>>,
    #[serde(default)]
    pub mineable_composition: HashMap<CigGuid, Handle<MineableComposition>>,
    #[serde(default)]
    pub mining_laser_global_params: HashMap<CigGuid, Handle<MiningLaserGlobalParams>>,
    #[serde(default)]
    pub mining_controller_global_params: HashMap<CigGuid, Handle<MiningControllerGlobalParams>>,
}

impl MiningIndex {
    pub fn len(&self) -> usize {
        self.mining_global_params.len()
            + self.mineable_element.len()
            + self.mineable_composition.len()
            + self.mining_laser_global_params.len()
            + self.mining_controller_global_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
