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

/// Pool storage for the `entities-ladder` feature.
#[derive(Default)]
pub struct EntitiesLadderPools {
    pub ladder_navigation_link: Vec<Option<LadderNavigationLink>>,
    pub exit_collision_check_override_params: Vec<Option<ExitCollisionCheckOverrideParams>>,
    pub auto_mount_radius_params: Vec<Option<AutoMountRadiusParams>>,
    pub ladder_access_climb_params: Vec<Option<LadderAccessClimbParams>>,
    pub ladder_access_point_params: Vec<Option<LadderAccessPointParams>>,
    pub ladder_component_params: Vec<Option<LadderComponentParams>>,
}
