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

/// Record index for the `specialeventdatabase` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpecialeventdatabaseIndex {
    #[serde(default)]
    pub special_event_manufacturer: HashMap<CigGuid, Handle<SpecialEventManufacturer>>,
    #[serde(default)]
    pub special_event_day: HashMap<CigGuid, Handle<SpecialEventDay>>,
    #[serde(default)]
    pub special_event_database: HashMap<CigGuid, Handle<SpecialEventDatabase>>,
}

impl SpecialeventdatabaseIndex {
    pub fn len(&self) -> usize {
        self.special_event_manufacturer.len()
            + self.special_event_day.len()
            + self.special_event_database.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
