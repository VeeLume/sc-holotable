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
use crate::Handle;
use std::collections::HashMap;
use svarog_common::CigGuid;

/// Record index for the `transportsystem` feature.
#[derive(Default)]
pub struct TransportsystemIndex {
    pub transport_destination_category: HashMap<CigGuid, Handle<TransportDestinationCategory>>,
    pub transport_destination_categories: HashMap<CigGuid, Handle<TransportDestinationCategories>>,
    pub transport_icon_type: HashMap<CigGuid, Handle<TransportIconType>>,
    pub transport_icon_types: HashMap<CigGuid, Handle<TransportIconTypes>>,
    pub transport_carriage_announcements: HashMap<CigGuid, Handle<TransportCarriageAnnouncements>>,
}

impl TransportsystemIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.transport_destination_category.len();
        total += self.transport_destination_categories.len();
        total += self.transport_icon_type.len();
        total += self.transport_icon_types.len();
        total += self.transport_carriage_announcements.len();
        total
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
