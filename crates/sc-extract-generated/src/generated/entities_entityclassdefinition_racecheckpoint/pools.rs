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

/// Pool storage for the `entities-entityclassdefinition_racecheckpoint` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesEntityclassdefinition_racecheckpointPools {
    #[serde(default)]
    pub race_checkpoint_component_params: Vec<Option<RaceCheckpointComponentParams>>,
    #[serde(default)]
    pub srace_checkpoint_object_metadata_params: Vec<Option<SRaceCheckpointObjectMetadataParams>>,
    #[serde(default)]
    pub srace_checkpoint_entry_tracker_params: Vec<Option<SRaceCheckpointEntryTrackerParams>>,
}
