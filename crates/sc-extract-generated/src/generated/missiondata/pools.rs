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

/// Pool storage for the `missiondata` feature.
#[derive(Default)]
pub struct MissiondataPools {
    pub mission_location_classes: Vec<Option<MissionLocationClasses>>,
    pub mission_location_sizes: Vec<Option<MissionLocationSizes>>,
    pub mission_location_functions: Vec<Option<MissionLocationFunctions>>,
    pub contract_scope_item: Vec<Option<ContractScope_Item>>,
    pub contract_scope_global_params: Vec<Option<ContractScopeGlobalParams>>,
    pub entity_cluster_id: Vec<Option<EntityClusterId>>,
    pub entity_cluster_member: Vec<Option<EntityClusterMember>>,
    pub mission_variable_boolean: Vec<Option<MissionVariableBoolean>>,
    pub mission_variable_integer: Vec<Option<MissionVariableInteger>>,
    pub module_declaration_type_location: Vec<Option<ModuleDeclarationType_Location>>,
    pub mission_init_param_boolean: Vec<Option<MissionInitParamBoolean>>,
    pub mission_init_param_integer: Vec<Option<MissionInitParamInteger>>,
    pub mission_init_param_tag: Vec<Option<MissionInitParamTag>>,
    pub mission_init_param_float: Vec<Option<MissionInitParamFloat>>,
    pub spawn_description_entry: Vec<Option<SpawnDescriptionEntry>>,
    pub spawn_descriptions: Vec<Option<SpawnDescriptions>>,
}
