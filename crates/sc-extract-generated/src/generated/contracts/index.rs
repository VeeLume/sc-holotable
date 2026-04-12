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

/// Record index for the `contracts` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractsIndex {
    #[serde(default)]
    pub contract_difficulty_profile: HashMap<CigGuid, Handle<ContractDifficultyProfile>>,
    #[serde(default)]
    pub mission_module_hierarchy: HashMap<CigGuid, Handle<MissionModuleHierarchy>>,
    #[serde(default)]
    pub global_mission_settings: HashMap<CigGuid, Handle<GlobalMissionSettings>>,
}

impl ContractsIndex {
    pub fn len(&self) -> usize {
        self.contract_difficulty_profile.len()
            + self.mission_module_hierarchy.len()
            + self.global_mission_settings.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
