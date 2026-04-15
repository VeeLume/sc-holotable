// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `sglobalhealingbeamparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SglobalhealingbeamparamsPools {
    #[serde(default)]
    pub shealing_beam_bone_entry_params: Vec<Option<SHealingBeamBoneEntryParams>>,
    #[serde(default)]
    pub shealing_beam_body_part_highlighting_params: Vec<Option<SHealingBeamBodyPartHighlightingParams>>,
    #[serde(default)]
    pub shealing_beam_body_part_params: Vec<Option<SHealingBeamBodyPartParams>>,
    #[serde(default)]
    pub sglobal_healing_beam_params: Vec<Option<SGlobalHealingBeamParams>>,
}
