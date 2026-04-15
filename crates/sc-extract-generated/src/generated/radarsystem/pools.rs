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

/// Pool storage for the `radarsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RadarsystemPools {
    #[serde(default)]
    pub radar_system_global_params: Vec<Option<RadarSystemGlobalParams>>,
    #[serde(default)]
    pub master_mode_switch_delta_signature_types: Vec<Option<MasterModeSwitchDeltaSignatureTypes>>,
    #[serde(default)]
    pub signature_system_global_params: Vec<Option<SignatureSystemGlobalParams>>,
    #[serde(default)]
    pub contact_state_global_params: Vec<Option<ContactStateGlobalParams>>,
    #[serde(default)]
    pub signature_uiglobal_params: Vec<Option<SignatureUIGlobalParams>>,
    #[serde(default)]
    pub actor_signature_multiplier_global_params: Vec<Option<ActorSignatureMultiplierGlobalParams>>,
    #[serde(default)]
    pub cross_section_global_params: Vec<Option<CrossSectionGlobalParams>>,
    #[serde(default)]
    pub signature_type_global_params: Vec<Option<SignatureTypeGlobalParams>>,
    #[serde(default)]
    pub scan_information_table: Vec<Option<ScanInformationTable>>,
    #[serde(default)]
    pub scan_display_condition_compare_params: Vec<Option<ScanDisplayConditionCompareParams>>,
    #[serde(default)]
    pub scan_display_condition_variable_params: Vec<Option<ScanDisplayConditionVariableParams>>,
    #[serde(default)]
    pub radar_signature_category_definition: Vec<Option<RadarSignatureCategoryDefinition>>,
    #[serde(default)]
    pub radar_signature_category_entry: Vec<Option<RadarSignatureCategoryEntry>>,
    #[serde(default)]
    pub radar_contact_type_definition: Vec<Option<RadarContactTypeDefinition>>,
    #[serde(default)]
    pub radar_contact_group_definition: Vec<Option<RadarContactGroupDefinition>>,
    #[serde(default)]
    pub radar_delta_signature_definition: Vec<Option<RadarDeltaSignatureDefinition>>,
}
