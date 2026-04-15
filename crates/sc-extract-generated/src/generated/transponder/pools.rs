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

/// Pool storage for the `transponder` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransponderPools {
    #[serde(default)]
    pub stransponder_object_metadata_params: Vec<Option<STransponderObjectMetadataParams>>,
    #[serde(default)]
    pub stransponder_entry_tracker_params: Vec<Option<STransponderEntryTrackerParams>>,
}
