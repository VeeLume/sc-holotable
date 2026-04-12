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

/// Record index for the `radarsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RadarsystemIndex {
    #[serde(default)]
    pub radar_system_global_params: HashMap<CigGuid, Handle<RadarSystemGlobalParams>>,
    #[serde(default)]
    pub radar_system_shared_params: HashMap<CigGuid, Handle<RadarSystemSharedParams>>,
    #[serde(default)]
    pub scan_information_def: HashMap<CigGuid, Handle<ScanInformationDef>>,
    #[serde(default)]
    pub scan_information_table: HashMap<CigGuid, Handle<ScanInformationTable>>,
    #[serde(default)]
    pub scan_custom_data_def: HashMap<CigGuid, Handle<ScanCustomDataDef>>,
    #[serde(default)]
    pub scan_display_layout_params: HashMap<CigGuid, Handle<ScanDisplayLayoutParams>>,
    #[serde(default)]
    pub radar_signature_category_definition: HashMap<CigGuid, Handle<RadarSignatureCategoryDefinition>>,
    #[serde(default)]
    pub radar_signature_category_entry: HashMap<CigGuid, Handle<RadarSignatureCategoryEntry>>,
    #[serde(default)]
    pub radar_contact_type_definition: HashMap<CigGuid, Handle<RadarContactTypeDefinition>>,
    #[serde(default)]
    pub radar_contact_type_entry: HashMap<CigGuid, Handle<RadarContactTypeEntry>>,
    #[serde(default)]
    pub radar_contact_group_definition: HashMap<CigGuid, Handle<RadarContactGroupDefinition>>,
    #[serde(default)]
    pub radar_contact_group_entry: HashMap<CigGuid, Handle<RadarContactGroupEntry>>,
    #[serde(default)]
    pub radar_delta_signature_definition: HashMap<CigGuid, Handle<RadarDeltaSignatureDefinition>>,
    #[serde(default)]
    pub radar_delta_signature_entry: HashMap<CigGuid, Handle<RadarDeltaSignatureEntry>>,
}

impl RadarsystemIndex {
    pub fn len(&self) -> usize {
        self.radar_system_global_params.len()
            + self.radar_system_shared_params.len()
            + self.scan_information_def.len()
            + self.scan_information_table.len()
            + self.scan_custom_data_def.len()
            + self.scan_display_layout_params.len()
            + self.radar_signature_category_definition.len()
            + self.radar_signature_category_entry.len()
            + self.radar_contact_type_definition.len()
            + self.radar_contact_type_entry.len()
            + self.radar_contact_group_definition.len()
            + self.radar_contact_group_entry.len()
            + self.radar_delta_signature_definition.len()
            + self.radar_delta_signature_entry.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
