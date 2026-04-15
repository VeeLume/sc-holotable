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

/// Pool storage for the `specialeventdatabase` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpecialeventdatabasePools {
    #[serde(default)]
    pub special_event_manufacturer: Vec<Option<SpecialEventManufacturer>>,
    #[serde(default)]
    pub special_event_day: Vec<Option<SpecialEventDay>>,
    #[serde(default)]
    pub special_event_database: Vec<Option<SpecialEventDatabase>>,
}
