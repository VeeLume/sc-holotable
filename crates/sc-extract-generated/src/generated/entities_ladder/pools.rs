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

/// Pool storage for the `entities-ladder` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesLadderPools {
    #[serde(default)]
    pub ladder_navigation_link: Vec<Option<LadderNavigationLink>>,
    #[serde(default)]
    pub exit_collision_check_override_params: Vec<Option<ExitCollisionCheckOverrideParams>>,
    #[serde(default)]
    pub auto_mount_radius_params: Vec<Option<AutoMountRadiusParams>>,
    #[serde(default)]
    pub ladder_access_climb_params: Vec<Option<LadderAccessClimbParams>>,
    #[serde(default)]
    pub ladder_access_point_params: Vec<Option<LadderAccessPointParams>>,
    #[serde(default)]
    pub ladder_component_params: Vec<Option<LadderComponentParams>>,
}
