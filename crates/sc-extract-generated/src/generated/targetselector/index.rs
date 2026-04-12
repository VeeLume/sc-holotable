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

/// Record index for the `targetselector` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetselectorIndex {
    #[serde(default)]
    pub stargeting_method_record: HashMap<CigGuid, Handle<STargetingMethodRecord>>,
    #[serde(default)]
    pub stargetable_item_types_record: HashMap<CigGuid, Handle<STargetableItemTypesRecord>>,
    #[serde(default)]
    pub starget_selector_global_targeting_params: HashMap<CigGuid, Handle<STargetSelectorGlobalTargetingParams>>,
}

impl TargetselectorIndex {
    pub fn len(&self) -> usize {
        self.stargeting_method_record.len()
            + self.stargetable_item_types_record.len()
            + self.starget_selector_global_targeting_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
