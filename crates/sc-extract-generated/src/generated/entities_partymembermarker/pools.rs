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

/// Pool storage for the `entities-partymembermarker` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesPartymembermarkerPools {
    #[serde(default)]
    pub sparty_member_marker_object_metadata_params: Vec<Option<SPartyMemberMarkerObjectMetadataParams>>,
    #[serde(default)]
    pub sentity_component_party_marker_params: Vec<Option<SEntityComponentPartyMarkerParams>>,
    #[serde(default)]
    pub sparty_member_entry_tracker_params: Vec<Option<SPartyMemberEntryTrackerParams>>,
}
