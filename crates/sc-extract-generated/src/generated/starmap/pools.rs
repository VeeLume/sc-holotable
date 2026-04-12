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

/// Pool storage for the `starmap` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarmapPools {
    #[serde(default)]
    pub star_map_object_type: Vec<Option<StarMapObjectType>>,
    #[serde(default)]
    pub star_map_object_types: Vec<Option<StarMapObjectTypes>>,
    #[serde(default)]
    pub star_map_mission_object: Vec<Option<StarMapMissionObject>>,
    #[serde(default)]
    pub star_map_party_member_object: Vec<Option<StarMapPartyMemberObject>>,
}
