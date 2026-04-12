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

/// Record index for the `ui-armarkerconfiguration` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiArmarkerconfigurationIndex {
    #[serde(default)]
    pub marker_decluttering_culling_order: HashMap<CigGuid, Handle<MarkerDeclutteringCullingOrder>>,
    #[serde(default)]
    pub global_marker_configs: HashMap<CigGuid, Handle<GlobalMarkerConfigs>>,
    #[serde(default)]
    pub marker_configuration: HashMap<CigGuid, Handle<Marker_Configuration>>,
}

impl UiArmarkerconfigurationIndex {
    pub fn len(&self) -> usize {
        self.marker_decluttering_culling_order.len()
            + self.global_marker_configs.len()
            + self.marker_configuration.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
