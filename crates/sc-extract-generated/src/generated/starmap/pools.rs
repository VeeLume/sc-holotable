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

/// Pool storage for the `starmap` feature.
#[derive(Default)]
pub struct StarmapPools {
    pub star_map_object_types: Vec<Option<StarMapObjectTypes>>,
    pub star_map_mission_object: Vec<Option<StarMapMissionObject>>,
    pub star_map_party_member_object: Vec<Option<StarMapPartyMemberObject>>,
}
