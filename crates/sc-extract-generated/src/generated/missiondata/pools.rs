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

/// Pool storage for the `missiondata` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MissiondataPools {
    #[serde(default)]
    pub mission_location_template: Vec<Option<MissionLocationTemplate>>,
    #[serde(default)]
    pub entity_cluster_id: Vec<Option<EntityClusterId>>,
    #[serde(default)]
    pub entity_cluster_member: Vec<Option<EntityClusterMember>>,
    #[serde(default)]
    pub mission_variable_boolean: Vec<Option<MissionVariableBoolean>>,
    #[serde(default)]
    pub mission_variable_integer: Vec<Option<MissionVariableInteger>>,
    #[serde(default)]
    pub location_resource_slot: Vec<Option<LocationResourceSlot>>,
    #[serde(default)]
    pub location_entity_type_static_entity: Vec<Option<LocationEntityType_StaticEntity>>,
    #[serde(default)]
    pub module_location_entities: Vec<Option<ModuleLocationEntities>>,
    #[serde(default)]
    pub module_location_entities_static: Vec<Option<ModuleLocationEntities_Static>>,
    #[serde(default)]
    pub module_declaration_type_mission: Vec<Option<ModuleDeclarationType_Mission>>,
    #[serde(default)]
    pub module_declaration_type_location: Vec<Option<ModuleDeclarationType_Location>>,
    #[serde(default)]
    pub module_declaration: Vec<Option<ModuleDeclaration>>,
    #[serde(default)]
    pub mission_init_param_boolean: Vec<Option<MissionInitParamBoolean>>,
    #[serde(default)]
    pub mission_init_param_integer: Vec<Option<MissionInitParamInteger>>,
    #[serde(default)]
    pub mission_init_param_tag: Vec<Option<MissionInitParamTag>>,
    #[serde(default)]
    pub mission_init_param_float: Vec<Option<MissionInitParamFloat>>,
    #[serde(default)]
    pub spawn_description_entry: Vec<Option<SpawnDescriptionEntry>>,
    #[serde(default)]
    pub spawn_descriptions: Vec<Option<SpawnDescriptions>>,
}
