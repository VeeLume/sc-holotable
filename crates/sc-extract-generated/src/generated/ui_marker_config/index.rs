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

/// Record index for the `ui-marker_config` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiMarker_configIndex {
    #[serde(default)]
    pub marker_ar_config_def: HashMap<CigGuid, Handle<MarkerAR_ConfigDef>>,
}

impl UiMarker_configIndex {
    pub fn len(&self) -> usize {
        self.marker_ar_config_def.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
