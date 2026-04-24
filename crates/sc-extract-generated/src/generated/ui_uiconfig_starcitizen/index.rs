// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;
use crate::Handle;
use std::collections::HashMap;
use svarog_common::CigGuid;

/// Record index for the `ui-uiconfig_starcitizen` feature.
#[derive(Default)]
pub struct UiUiconfig_starcitizenIndex {
    pub uiconfig: HashMap<CigGuid, Handle<UIConfig>>,
}

impl UiUiconfig_starcitizenIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.uiconfig.len();
        total
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
