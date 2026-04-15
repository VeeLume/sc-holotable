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

/// Pool storage for the `radarsystem` feature.
#[derive(Default)]
pub struct RadarsystemPools {
    pub radar_system_global_params: Vec<Option<RadarSystemGlobalParams>>,
    pub master_mode_switch_delta_signature_types: Vec<Option<MasterModeSwitchDeltaSignatureTypes>>,
    pub signature_system_global_params: Vec<Option<SignatureSystemGlobalParams>>,
    pub contact_state_global_params: Vec<Option<ContactStateGlobalParams>>,
    pub signature_uiglobal_params: Vec<Option<SignatureUIGlobalParams>>,
    pub actor_signature_multiplier_global_params: Vec<Option<ActorSignatureMultiplierGlobalParams>>,
    pub cross_section_global_params: Vec<Option<CrossSectionGlobalParams>>,
    pub signature_type_global_params: Vec<Option<SignatureTypeGlobalParams>>,
    pub scan_information_table: Vec<Option<ScanInformationTable>>,
    pub scan_display_condition_compare_params: Vec<Option<ScanDisplayConditionCompareParams>>,
    pub scan_display_condition_variable_params: Vec<Option<ScanDisplayConditionVariableParams>>,
    pub radar_signature_category_definition: Vec<Option<RadarSignatureCategoryDefinition>>,
    pub radar_signature_category_entry: Vec<Option<RadarSignatureCategoryEntry>>,
    pub radar_contact_type_definition: Vec<Option<RadarContactTypeDefinition>>,
    pub radar_contact_group_definition: Vec<Option<RadarContactGroupDefinition>>,
    pub radar_delta_signature_definition: Vec<Option<RadarDeltaSignatureDefinition>>,
}
