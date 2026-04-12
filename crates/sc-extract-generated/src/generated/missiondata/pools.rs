// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `missiondata` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MissiondataPools {
    #[serde(default)]
    pub spawn_settings_inventory_item: Vec<Option<SpawnSettingsInventoryItem>>,
    #[serde(default)]
    pub auto_spawn_settings: Vec<Option<AutoSpawnSettings>>,
    #[serde(default)]
    pub mission_location_tags: Vec<Option<MissionLocationTags>>,
    #[serde(default)]
    pub mission_string_variant: Vec<Option<MissionStringVariant>>,
    #[serde(default)]
    pub mission_string_variants: Vec<Option<MissionStringVariants>>,
    #[serde(default)]
    pub location_mission_limit: Vec<Option<LocationMissionLimit>>,
    #[serde(default)]
    pub smission_location_module: Vec<Option<SMissionLocationModule>>,
    #[serde(default)]
    pub mission_location_data: Vec<Option<MissionLocationData>>,
    #[serde(default)]
    pub mission_location_template: Vec<Option<MissionLocationTemplate>>,
    #[serde(default)]
    pub mission_item: Vec<Option<MissionItem>>,
    #[serde(default)]
    pub mission_organization: Vec<Option<MissionOrganization>>,
    #[serde(default)]
    pub entity_cluster_id: Vec<Option<EntityClusterId>>,
    #[serde(default)]
    pub entity_cluster_member: Vec<Option<EntityClusterMember>>,
    #[serde(default)]
    pub location_resource_slot: Vec<Option<LocationResourceSlot>>,
    #[serde(default)]
    pub location_entity_type_base: Vec<Option<LocationEntityType_Base>>,
    #[serde(default)]
    pub location_entity_declaration: Vec<Option<LocationEntityDeclaration>>,
    #[serde(default)]
    pub module_declaration_type_base: Vec<Option<ModuleDeclarationType_Base>>,
    #[serde(default)]
    pub module_declaration: Vec<Option<ModuleDeclaration>>,
    #[serde(default)]
    pub mission_locality: Vec<Option<MissionLocality>>,
    #[serde(default)]
    pub spawn_description_entry: Vec<Option<SpawnDescriptionEntry>>,
    #[serde(default)]
    pub spawn_descriptions: Vec<Option<SpawnDescriptions>>,
}
