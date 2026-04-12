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

/// Record index for the `ui-mobiglas` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiMobiglasIndex {
    #[serde(default)]
    pub armode_settings: HashMap<CigGuid, Handle<ARModeSettings>>,
    #[serde(default)]
    pub mobi_glas_app: HashMap<CigGuid, Handle<mobiGlasApp>>,
    #[serde(default)]
    pub mobi_glas_app_data: HashMap<CigGuid, Handle<MobiGlasAppData>>,
}

impl UiMobiglasIndex {
    pub fn len(&self) -> usize {
        self.armode_settings.len()
            + self.mobi_glas_app.len()
            + self.mobi_glas_app_data.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
