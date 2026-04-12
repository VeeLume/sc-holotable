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

/// Record index for the `cameras` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CamerasIndex {
    #[serde(default)]
    pub camera: HashMap<CigGuid, Handle<Camera>>,
    #[serde(default)]
    pub camera_shop_config: HashMap<CigGuid, Handle<CameraShopConfig>>,
    #[serde(default)]
    pub suggested_fovsetup: HashMap<CigGuid, Handle<SuggestedFOVSetup>>,
    #[serde(default)]
    pub cinematic_camera_controller_setup: HashMap<CigGuid, Handle<CinematicCameraControllerSetup>>,
    #[serde(default)]
    pub camera_fovchange_data: HashMap<CigGuid, Handle<CameraFOVChangeData>>,
}

impl CamerasIndex {
    pub fn len(&self) -> usize {
        self.camera.len()
            + self.camera_shop_config.len()
            + self.suggested_fovsetup.len()
            + self.cinematic_camera_controller_setup.len()
            + self.camera_fovchange_data.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
