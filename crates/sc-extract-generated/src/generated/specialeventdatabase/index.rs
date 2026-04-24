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

/// Record index for the `specialeventdatabase` feature.
#[derive(Default)]
pub struct SpecialeventdatabaseIndex {
    pub special_event_manufacturer: HashMap<CigGuid, Handle<SpecialEventManufacturer>>,
    pub special_event_day: HashMap<CigGuid, Handle<SpecialEventDay>>,
    pub special_event_database: HashMap<CigGuid, Handle<SpecialEventDatabase>>,
}

impl SpecialeventdatabaseIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.special_event_manufacturer.len();
        total += self.special_event_day.len();
        total += self.special_event_database.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
