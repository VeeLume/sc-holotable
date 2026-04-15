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

/// Pool storage for the `globalcargoloadingparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalcargoloadingparamsPools {
    #[serde(default)]
    pub sauto_loading_box_size_loading_time: Vec<Option<SAutoLoadingBoxSizeLoadingTime>>,
    #[serde(default)]
    pub global_cargo_loading_params: Vec<Option<GlobalCargoLoadingParams>>,
    #[serde(default)]
    pub cargo_loading_notification_params: Vec<Option<CargoLoadingNotificationParams>>,
}
