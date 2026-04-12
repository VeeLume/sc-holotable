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

/// Pool storage for the `contracts` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractsPools {
    #[serde(default)]
    pub mission_location_validation: Vec<Option<MissionLocationValidation>>,
    #[serde(default)]
    pub contract_difficulty_profile: Vec<Option<ContractDifficultyProfile>>,
    #[serde(default)]
    pub mission_module_hierarchy_sub_mission: Vec<Option<MissionModuleHierarchySubMission>>,
    #[serde(default)]
    pub mission_module_hierarchy: Vec<Option<MissionModuleHierarchy>>,
    #[serde(default)]
    pub spvpbounty_contract_generators: Vec<Option<SPVPBountyContractGenerators>>,
    #[serde(default)]
    pub global_mission_settings: Vec<Option<GlobalMissionSettings>>,
}
