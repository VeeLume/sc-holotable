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

/// Record index for the `starmap` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarmapIndex {
    #[serde(default)]
    pub star_map_object_type: HashMap<CigGuid, Handle<StarMapObjectType>>,
    #[serde(default)]
    pub star_map_object_types: HashMap<CigGuid, Handle<StarMapObjectTypes>>,
    #[serde(default)]
    pub star_map_mission_object: HashMap<CigGuid, Handle<StarMapMissionObject>>,
    #[serde(default)]
    pub star_map_party_member_object: HashMap<CigGuid, Handle<StarMapPartyMemberObject>>,
}

impl StarmapIndex {
    pub fn len(&self) -> usize {
        self.star_map_object_type.len()
            + self.star_map_object_types.len()
            + self.star_map_mission_object.len()
            + self.star_map_party_member_object.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
