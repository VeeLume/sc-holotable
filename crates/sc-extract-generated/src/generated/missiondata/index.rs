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

/// Record index for the `missiondata` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MissiondataIndex {
    #[serde(default)]
    pub mission_location_template: HashMap<CigGuid, Handle<MissionLocationTemplate>>,
    #[serde(default)]
    pub mission_item: HashMap<CigGuid, Handle<MissionItem>>,
    #[serde(default)]
    pub mission_organization: HashMap<CigGuid, Handle<MissionOrganization>>,
    #[serde(default)]
    pub entity_cluster_id: HashMap<CigGuid, Handle<EntityClusterId>>,
    #[serde(default)]
    pub entity_cluster_member: HashMap<CigGuid, Handle<EntityClusterMember>>,
    #[serde(default)]
    pub location_resource_slot: HashMap<CigGuid, Handle<LocationResourceSlot>>,
    #[serde(default)]
    pub location_entity_declaration: HashMap<CigGuid, Handle<LocationEntityDeclaration>>,
    #[serde(default)]
    pub module_declaration: HashMap<CigGuid, Handle<ModuleDeclaration>>,
    #[serde(default)]
    pub mission_locality: HashMap<CigGuid, Handle<MissionLocality>>,
    #[serde(default)]
    pub spawn_descriptions: HashMap<CigGuid, Handle<SpawnDescriptions>>,
}

impl MissiondataIndex {
    pub fn len(&self) -> usize {
        self.mission_location_template.len()
            + self.mission_item.len()
            + self.mission_organization.len()
            + self.entity_cluster_id.len()
            + self.entity_cluster_member.len()
            + self.location_resource_slot.len()
            + self.location_entity_declaration.len()
            + self.module_declaration.len()
            + self.mission_locality.len()
            + self.spawn_descriptions.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
