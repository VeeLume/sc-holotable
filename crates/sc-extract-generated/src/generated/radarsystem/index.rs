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

/// Record index for the `radarsystem` feature.
#[derive(Default)]
pub struct RadarsystemIndex {
    pub radar_system_global_params: HashMap<CigGuid, Handle<RadarSystemGlobalParams>>,
    pub scan_information_table: HashMap<CigGuid, Handle<ScanInformationTable>>,
    pub radar_signature_category_definition:
        HashMap<CigGuid, Handle<RadarSignatureCategoryDefinition>>,
    pub radar_signature_category_entry: HashMap<CigGuid, Handle<RadarSignatureCategoryEntry>>,
    pub radar_contact_type_definition: HashMap<CigGuid, Handle<RadarContactTypeDefinition>>,
    pub radar_contact_group_definition: HashMap<CigGuid, Handle<RadarContactGroupDefinition>>,
    pub radar_delta_signature_definition: HashMap<CigGuid, Handle<RadarDeltaSignatureDefinition>>,
}

impl RadarsystemIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.radar_system_global_params.len();
        total += self.scan_information_table.len();
        total += self.radar_signature_category_definition.len();
        total += self.radar_signature_category_entry.len();
        total += self.radar_contact_type_definition.len();
        total += self.radar_contact_group_definition.len();
        total += self.radar_delta_signature_definition.len();
        total
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
