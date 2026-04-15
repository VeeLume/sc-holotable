// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `cameras` feature.
#[derive(Default)]
pub struct CamerasPools {
    pub camera_look_behind_config: Vec<Option<CameraLookBehindConfig>>,
    pub camera_time_cam_config: Vec<Option<CameraTimeCamConfig>>,
    pub orbit_entity_cinematic_event: Vec<Option<OrbitEntityCinematicEvent>>,
    pub camera_orbit_entity_cinematic_config: Vec<Option<CameraOrbitEntityCinematicConfig>>,
    pub camera_orbit_fpsdeath_cam_config: Vec<Option<CameraOrbitFPSDeathCamConfig>>,
    pub camera_docking_config: Vec<Option<CameraDockingConfig>>,
    pub camera_shop_item_offset: Vec<Option<CameraShopItemOffset>>,
    pub camera_shop_config: Vec<Option<CameraShopConfig>>,
    pub camera_static_config: Vec<Option<CameraStaticConfig>>,
    pub camera_view2_ships_frame_params: Vec<Option<CameraView2ShipsFrameParams>>,
    pub camera_trackview_config: Vec<Option<CameraTrackviewConfig>>,
    pub cinematic_camera_controller_setup: Vec<Option<CinematicCameraControllerSetup>>,
    pub camera_fovchange_data: Vec<Option<CameraFOVChangeData>>,
}
