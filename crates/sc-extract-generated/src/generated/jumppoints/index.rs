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

/// Record index for the `jumppoints` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JumppointsIndex {
    #[serde(default)]
    pub global_jump_point_params: HashMap<CigGuid, Handle<GlobalJumpPointParams>>,
    #[serde(default)]
    pub global_jump_tunnel_host_params: HashMap<CigGuid, Handle<GlobalJumpTunnelHostParams>>,
    #[serde(default)]
    pub global_jump_drive_params: HashMap<CigGuid, Handle<GlobalJumpDriveParams>>,
    #[serde(default)]
    pub jump_travel_camera_params: HashMap<CigGuid, Handle<JumpTravelCameraParams>>,
}

impl JumppointsIndex {
    pub fn len(&self) -> usize {
        self.global_jump_point_params.len()
            + self.global_jump_tunnel_host_params.len()
            + self.global_jump_drive_params.len()
            + self.jump_travel_camera_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
