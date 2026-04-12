// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `contracts`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MissionLocationValidation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionLocationValidation {
}

impl Pooled for MissionLocationValidation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.contracts.mission_location_validation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.contracts.mission_location_validation }
}

impl<'a> Extract<'a> for MissionLocationValidation {
    const TYPE_NAME: &'static str = "MissionLocationValidation";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ContractDifficultyProfile`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDifficultyProfile {
    /// `mechanicalSkillWeight` (Single)
    #[serde(default)]
    pub mechanical_skill_weight: f32,
    /// `mentalLoadWeight` (Single)
    #[serde(default)]
    pub mental_load_weight: f32,
    /// `riskOfLossWeight` (Single)
    #[serde(default)]
    pub risk_of_loss_weight: f32,
    /// `gameKnowledgeWeight` (Single)
    #[serde(default)]
    pub game_knowledge_weight: f32,
}

impl Pooled for ContractDifficultyProfile {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.contracts.contract_difficulty_profile }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.contracts.contract_difficulty_profile }
}

impl<'a> Extract<'a> for ContractDifficultyProfile {
    const TYPE_NAME: &'static str = "ContractDifficultyProfile";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mechanical_skill_weight: inst.get_f32("mechanicalSkillWeight").unwrap_or_default(),
            mental_load_weight: inst.get_f32("mentalLoadWeight").unwrap_or_default(),
            risk_of_loss_weight: inst.get_f32("riskOfLossWeight").unwrap_or_default(),
            game_knowledge_weight: inst.get_f32("gameKnowledgeWeight").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionModuleHierarchySubMission`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionModuleHierarchySubMission {
    /// `subMissionModule` (String)
    #[serde(default)]
    pub sub_mission_module: String,
    /// `subModuleHierarchy` (Reference)
    #[serde(default)]
    pub sub_module_hierarchy: Option<CigGuid>,
}

impl Pooled for MissionModuleHierarchySubMission {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.contracts.mission_module_hierarchy_sub_mission }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.contracts.mission_module_hierarchy_sub_mission }
}

impl<'a> Extract<'a> for MissionModuleHierarchySubMission {
    const TYPE_NAME: &'static str = "MissionModuleHierarchySubMission";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            sub_mission_module: inst.get_str("subMissionModule").map(String::from).unwrap_or_default(),
            sub_module_hierarchy: inst.get("subModuleHierarchy").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MissionModuleHierarchy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionModuleHierarchy {
    /// `missionModule` (String)
    #[serde(default)]
    pub mission_module: String,
    /// `subMissionModules` (Class (array))
    #[serde(default)]
    pub sub_mission_modules: Vec<Handle<MissionModuleHierarchySubMission>>,
}

impl Pooled for MissionModuleHierarchy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.contracts.mission_module_hierarchy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.contracts.mission_module_hierarchy }
}

impl<'a> Extract<'a> for MissionModuleHierarchy {
    const TYPE_NAME: &'static str = "MissionModuleHierarchy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_module: inst.get_str("missionModule").map(String::from).unwrap_or_default(),
            sub_mission_modules: inst.get_array("subMissionModules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionModuleHierarchySubMission>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionModuleHierarchySubMission>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SPVPBountyContractGenerators`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPVPBountyContractGenerators {
    /// `locationAvailable` (Reference)
    #[serde(default)]
    pub location_available: Option<CigGuid>,
    /// `contractGenerator` (Reference)
    #[serde(default)]
    pub contract_generator: Option<CigGuid>,
}

impl Pooled for SPVPBountyContractGenerators {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.contracts.spvpbounty_contract_generators }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.contracts.spvpbounty_contract_generators }
}

impl<'a> Extract<'a> for SPVPBountyContractGenerators {
    const TYPE_NAME: &'static str = "SPVPBountyContractGenerators";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_available: inst.get("locationAvailable").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            contract_generator: inst.get("contractGenerator").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `GlobalMissionSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMissionSettings {
    /// `locationValidation` (StrongPointer (array))
    #[serde(default)]
    pub location_validation: Vec<Handle<MissionLocationValidation>>,
    /// `defaultJurisdictionForPlayerCrimeStats` (Reference)
    #[serde(default)]
    pub default_jurisdiction_for_player_crime_stats: Option<CigGuid>,
    /// `PVPBountyContractGenerators` (Class (array))
    #[serde(default)]
    pub pvpbounty_contract_generators: Vec<Handle<SPVPBountyContractGenerators>>,
}

impl Pooled for GlobalMissionSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.contracts.global_mission_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.contracts.global_mission_settings }
}

impl<'a> Extract<'a> for GlobalMissionSettings {
    const TYPE_NAME: &'static str = "GlobalMissionSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_validation: inst.get_array("locationValidation")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MissionLocationValidation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MissionLocationValidation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_jurisdiction_for_player_crime_stats: inst.get("defaultJurisdictionForPlayerCrimeStats").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pvpbounty_contract_generators: inst.get_array("PVPBountyContractGenerators")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SPVPBountyContractGenerators>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SPVPBountyContractGenerators>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

