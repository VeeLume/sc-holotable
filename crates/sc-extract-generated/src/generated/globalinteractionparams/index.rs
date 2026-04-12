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

/// Record index for the `globalinteractionparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalinteractionparamsIndex {
    #[serde(default)]
    pub carryable_interactions_metadata_config_def: HashMap<CigGuid, Handle<CarryableInteractionsMetadataConfigDef>>,
    #[serde(default)]
    pub skin_interactable_templates: HashMap<CigGuid, Handle<SkinInteractableTemplates>>,
}

impl GlobalinteractionparamsIndex {
    pub fn len(&self) -> usize {
        self.carryable_interactions_metadata_config_def.len()
            + self.skin_interactable_templates.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
