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

/// Pool storage for the `foley` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoleyPools {
    #[serde(default)]
    pub foley_definition: Vec<Option<FoleyDefinition>>,
    #[serde(default)]
    pub foley_bone: Vec<Option<FoleyBone>>,
    #[serde(default)]
    pub foley_axis: Vec<Option<FoleyAxis>>,
    #[serde(default)]
    pub foley_one_shot: Vec<Option<FoleyOneShot>>,
    #[serde(default)]
    pub foley_collision: Vec<Option<FoleyCollision>>,
    #[serde(default)]
    pub foley_loop: Vec<Option<FoleyLoop>>,
    #[serde(default)]
    pub foley_item: Vec<Option<FoleyItem>>,
    #[serde(default)]
    pub audio_footstep_surfaces_definition: Vec<Option<AudioFootstepSurfacesDefinition>>,
    #[serde(default)]
    pub audio_footstep_surface_mapping: Vec<Option<AudioFootstepSurfaceMapping>>,
    #[serde(default)]
    pub user_rtpc: Vec<Option<UserRTPC>>,
    #[serde(default)]
    pub foley_footstep_definition: Vec<Option<FoleyFootstepDefinition>>,
}
