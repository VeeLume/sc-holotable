// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

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
    pub entity_cluster_id: HashMap<CigGuid, Handle<EntityClusterId>>,
    #[serde(default)]
    pub entity_cluster_member: HashMap<CigGuid, Handle<EntityClusterMember>>,
    #[serde(default)]
    pub location_resource_slot: HashMap<CigGuid, Handle<LocationResourceSlot>>,
    #[serde(default)]
    pub module_declaration: HashMap<CigGuid, Handle<ModuleDeclaration>>,
    #[serde(default)]
    pub spawn_descriptions: HashMap<CigGuid, Handle<SpawnDescriptions>>,
}

impl MissiondataIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.mission_location_template.len();
        total += self.entity_cluster_id.len();
        total += self.entity_cluster_member.len();
        total += self.location_resource_slot.len();
        total += self.module_declaration.len();
        total += self.spawn_descriptions.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
