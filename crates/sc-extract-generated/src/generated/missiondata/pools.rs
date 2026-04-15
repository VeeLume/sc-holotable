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
    pub mission_location_template: Vec<Option<MissionLocationTemplate>>,
    pub entity_cluster_id: Vec<Option<EntityClusterId>>,
    pub entity_cluster_member: Vec<Option<EntityClusterMember>>,
    pub mission_variable_boolean: Vec<Option<MissionVariableBoolean>>,
    pub mission_variable_integer: Vec<Option<MissionVariableInteger>>,
    pub location_resource_slot: Vec<Option<LocationResourceSlot>>,
    pub location_entity_type_static_entity: Vec<Option<LocationEntityType_StaticEntity>>,
    pub module_location_entities: Vec<Option<ModuleLocationEntities>>,
    pub module_location_entities_static: Vec<Option<ModuleLocationEntities_Static>>,
    pub module_declaration_type_mission: Vec<Option<ModuleDeclarationType_Mission>>,
    pub module_declaration_type_location: Vec<Option<ModuleDeclarationType_Location>>,
    pub module_declaration: Vec<Option<ModuleDeclaration>>,
    pub mission_init_param_boolean: Vec<Option<MissionInitParamBoolean>>,
    pub mission_init_param_integer: Vec<Option<MissionInitParamInteger>>,
    pub mission_init_param_tag: Vec<Option<MissionInitParamTag>>,
    pub mission_init_param_float: Vec<Option<MissionInitParamFloat>>,
    pub spawn_description_entry: Vec<Option<SpawnDescriptionEntry>>,
    pub spawn_descriptions: Vec<Option<SpawnDescriptions>>,
}
