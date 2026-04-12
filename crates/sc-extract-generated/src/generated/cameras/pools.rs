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

/// Pool storage for the `cameras` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CamerasPools {
    #[serde(default)]
    pub camera: Vec<Option<Camera>>,
    #[serde(default)]
    pub camera_base_settings_config: Vec<Option<CameraBaseSettingsConfig>>,
    #[serde(default)]
    pub camera_blend_config: Vec<Option<CameraBlendConfig>>,
    #[serde(default)]
    pub camera_fovconfig: Vec<Option<CameraFOVConfig>>,
    #[serde(default)]
    pub camera_base_config: Vec<Option<CameraBaseConfig>>,
    #[serde(default)]
    pub camera_shop_item_offset: Vec<Option<CameraShopItemOffset>>,
    #[serde(default)]
    pub camera_shop_config: Vec<Option<CameraShopConfig>>,
    #[serde(default)]
    pub suggested_fovsetup: Vec<Option<SuggestedFOVSetup>>,
    #[serde(default)]
    pub cinematic_camera_controller_setup: Vec<Option<CinematicCameraControllerSetup>>,
    #[serde(default)]
    pub camera_fovchange_data: Vec<Option<CameraFOVChangeData>>,
}
