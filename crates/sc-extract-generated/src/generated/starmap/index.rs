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
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `starmap` feature.
#[derive(Default)]
pub struct StarmapIndex {
    pub star_map_object_types: HashMap<CigGuid, Handle<StarMapObjectTypes>>,
    pub star_map_mission_object: HashMap<CigGuid, Handle<StarMapMissionObject>>,
    pub star_map_party_member_object: HashMap<CigGuid, Handle<StarMapPartyMemberObject>>,
}

impl StarmapIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.star_map_object_types.len();
        total += self.star_map_mission_object.len();
        total += self.star_map_party_member_object.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
