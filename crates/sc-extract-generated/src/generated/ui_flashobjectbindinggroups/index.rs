// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `ui-flashobjectbindinggroups` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiFlashobjectbindinggroupsIndex {
    #[serde(default)]
    pub flash_object_binding_group: HashMap<CigGuid, Handle<FlashObjectBindingGroup>>,
}

impl UiFlashobjectbindinggroupsIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.flash_object_binding_group.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
