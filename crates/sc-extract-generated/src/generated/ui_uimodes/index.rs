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

/// Record index for the `ui-uimodes` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiUimodesIndex {
    #[serde(default)]
    pub uimode_visibility_settings: HashMap<CigGuid, Handle<UIModeVisibilitySettings>>,
}

impl UiUimodesIndex {
    pub fn len(&self) -> usize {
        self.uimode_visibility_settings.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
