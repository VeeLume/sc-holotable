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

/// Pool storage for the `ui-itempreview_config` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiItempreview_configPools {
    #[serde(default)]
    pub item_preview_light_intensities: Vec<Option<ItemPreview_LightIntensities>>,
    #[serde(default)]
    pub item_preview_lighting_settings: Vec<Option<ItemPreview_LightingSettings>>,
    #[serde(default)]
    pub item_preview_skinned_loadout_override: Vec<Option<ItemPreview_SkinnedLoadoutOverride>>,
    #[serde(default)]
    pub item_preview_camera_settings: Vec<Option<ItemPreview_CameraSettings>>,
    #[serde(default)]
    pub item_preview_camera_settings_override: Vec<Option<ItemPreview_CameraSettingsOverride>>,
    #[serde(default)]
    pub item_preview_turntable_settings: Vec<Option<ItemPreview_TurntableSettings>>,
    #[serde(default)]
    pub item_preview_turntable_override: Vec<Option<ItemPreview_TurntableOverride>>,
    #[serde(default)]
    pub item_preview_config: Vec<Option<ItemPreview_Config>>,
}
