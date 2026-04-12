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

/// Pool storage for the `ui-mobiglas` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiMobiglasPools {
    #[serde(default)]
    pub armode_settings: Vec<Option<ARModeSettings>>,
    #[serde(default)]
    pub mobi_glas_app: Vec<Option<mobiGlasApp>>,
    #[serde(default)]
    pub mobi_glas_app_data: Vec<Option<MobiGlasAppData>>,
    #[serde(default)]
    pub mobi_glas_app_data_base: Vec<Option<MobiGlasAppDataBase>>,
    #[serde(default)]
    pub smobi_glas_app_params_base: Vec<Option<SMobiGlasAppParamsBase>>,
}
