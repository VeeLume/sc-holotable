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

/// Record index for the `ui-worlddisplay` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiWorlddisplayIndex {
    #[serde(default)]
    pub world_display_radar: HashMap<CigGuid, Handle<WorldDisplayRadar>>,
    #[serde(default)]
    pub world_display_environment: HashMap<CigGuid, Handle<WorldDisplayEnvironment>>,
}

impl UiWorlddisplayIndex {
    pub fn len(&self) -> usize {
        self.world_display_radar.len()
            + self.world_display_environment.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
