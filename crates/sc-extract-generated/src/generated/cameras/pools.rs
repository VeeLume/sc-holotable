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

/// Pool storage for the `cameras` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CamerasPools {
    #[serde(default)]
    pub camera_look_behind_config: Vec<Option<CameraLookBehindConfig>>,
    #[serde(default)]
    pub camera_time_cam_config: Vec<Option<CameraTimeCamConfig>>,
    #[serde(default)]
    pub orbit_entity_cinematic_event: Vec<Option<OrbitEntityCinematicEvent>>,
    #[serde(default)]
    pub camera_orbit_entity_cinematic_config: Vec<Option<CameraOrbitEntityCinematicConfig>>,
    #[serde(default)]
    pub camera_orbit_fpsdeath_cam_config: Vec<Option<CameraOrbitFPSDeathCamConfig>>,
    #[serde(default)]
    pub camera_docking_config: Vec<Option<CameraDockingConfig>>,
    #[serde(default)]
    pub camera_shop_item_offset: Vec<Option<CameraShopItemOffset>>,
    #[serde(default)]
    pub camera_shop_config: Vec<Option<CameraShopConfig>>,
    #[serde(default)]
    pub camera_static_config: Vec<Option<CameraStaticConfig>>,
    #[serde(default)]
    pub camera_view2_ships_frame_params: Vec<Option<CameraView2ShipsFrameParams>>,
    #[serde(default)]
    pub camera_trackview_config: Vec<Option<CameraTrackviewConfig>>,
    #[serde(default)]
    pub cinematic_camera_controller_setup: Vec<Option<CinematicCameraControllerSetup>>,
    #[serde(default)]
    pub camera_fovchange_data: Vec<Option<CameraFOVChangeData>>,
}
