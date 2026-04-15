// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

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
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.global_jump_point_params.len();
        total += self.global_jump_tunnel_host_params.len();
        total += self.global_jump_drive_params.len();
        total += self.jump_travel_camera_params.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
