// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `globalinteractionparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalinteractionparamsPools {
    #[serde(default)]
    pub carryable_interactions_metadata_def: Vec<Option<CarryableInteractionsMetadataDef>>,
    #[serde(default)]
    pub carryable_interactions_metadata_config_def: Vec<Option<CarryableInteractionsMetadataConfigDef>>,
    #[serde(default)]
    pub skin_interactable_template: Vec<Option<SkinInteractableTemplate>>,
    #[serde(default)]
    pub skin_interactable_templates: Vec<Option<SkinInteractableTemplates>>,
}
