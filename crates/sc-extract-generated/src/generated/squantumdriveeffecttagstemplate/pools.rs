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

/// Pool storage for the `squantumdriveeffecttagstemplate` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SquantumdriveeffecttagstemplatePools {
    #[serde(default)]
    pub squantum_drive_effect_params_legacy: Vec<Option<SQuantumDriveEffectParams_LEGACY>>,
    #[serde(default)]
    pub squantum_drive_effect_template: Vec<Option<SQuantumDriveEffectTemplate>>,
}
