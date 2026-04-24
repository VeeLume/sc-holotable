// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `missionbroker`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `MissionModifier_LawLicense`
/// Inherits from: `BaseMissionModifier`
pub struct MissionModifier_LawLicense {
    /// `modifierName` (String)
    pub modifier_name: String,
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `licenseType` (EnumChoice)
    pub license_type: ELawLicenseType,
}

impl Pooled for MissionModifier_LawLicense {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missionbroker.mission_modifier_law_license
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missionbroker.mission_modifier_law_license
    }
}

impl<'a> Extract<'a> for MissionModifier_LawLicense {
    const TYPE_NAME: &'static str = "MissionModifier_LawLicense";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            modifier_name: inst
                .get_str("modifierName")
                .map(String::from)
                .unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            license_type: ELawLicenseType::from_dcb_str(inst.get_str("licenseType").unwrap_or("")),
        }
    }
}

/// DCB type: `MissionModifier_FactionHostility`
/// Inherits from: `BaseMissionModifier`
pub struct MissionModifier_FactionHostility {
    /// `modifierName` (String)
    pub modifier_name: String,
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `faction` (Reference)
    pub faction: Option<CigGuid>,
    /// `myReaction` (EnumChoice)
    pub my_reaction: ReactionType,
    /// `theirReaction` (EnumChoice)
    pub their_reaction: ReactionType,
    /// `ignoreCriminalHostility` (Boolean)
    pub ignore_criminal_hostility: bool,
}

impl Pooled for MissionModifier_FactionHostility {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missionbroker.mission_modifier_faction_hostility
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missionbroker.mission_modifier_faction_hostility
    }
}

impl<'a> Extract<'a> for MissionModifier_FactionHostility {
    const TYPE_NAME: &'static str = "MissionModifier_FactionHostility";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            modifier_name: inst
                .get_str("modifierName")
                .map(String::from)
                .unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            faction: inst
                .get("faction")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            my_reaction: ReactionType::from_dcb_str(inst.get_str("myReaction").unwrap_or("")),
            their_reaction: ReactionType::from_dcb_str(inst.get_str("theirReaction").unwrap_or("")),
            ignore_criminal_hostility: inst.get_bool("ignoreCriminalHostility").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionModifier_HostileMission`
/// Inherits from: `BaseMissionModifier`
pub struct MissionModifier_HostileMission {
    /// `modifierName` (String)
    pub modifier_name: String,
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `missionBrokerEntry` (Reference)
    pub mission_broker_entry: Option<CigGuid>,
    /// `legalToAttack` (Boolean)
    pub legal_to_attack: bool,
}

impl Pooled for MissionModifier_HostileMission {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.missionbroker.mission_modifier_hostile_mission
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.missionbroker.mission_modifier_hostile_mission
    }
}

impl<'a> Extract<'a> for MissionModifier_HostileMission {
    const TYPE_NAME: &'static str = "MissionModifier_HostileMission";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            modifier_name: inst
                .get_str("modifierName")
                .map(String::from)
                .unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            mission_broker_entry: inst
                .get("missionBrokerEntry")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            legal_to_attack: inst.get_bool("legalToAttack").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionModifier_IgnoreMissionPlayerCriminality`
/// Inherits from: `BaseMissionModifier`
pub struct MissionModifier_IgnoreMissionPlayerCriminality {
    /// `modifierName` (String)
    pub modifier_name: String,
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `missionBrokerEntry` (Reference)
    pub mission_broker_entry: Option<CigGuid>,
}

impl Pooled for MissionModifier_IgnoreMissionPlayerCriminality {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .missionbroker
            .mission_modifier_ignore_mission_player_criminality
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .missionbroker
            .mission_modifier_ignore_mission_player_criminality
    }
}

impl<'a> Extract<'a> for MissionModifier_IgnoreMissionPlayerCriminality {
    const TYPE_NAME: &'static str = "MissionModifier_IgnoreMissionPlayerCriminality";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            modifier_name: inst
                .get_str("modifierName")
                .map(String::from)
                .unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            mission_broker_entry: inst
                .get("missionBrokerEntry")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `SReputationMissionRequirementExpression_And`
/// Inherits from: `SReputationMissionRequirementExpressionElement`
pub struct SReputationMissionRequirementExpression_And {}

impl Pooled for SReputationMissionRequirementExpression_And {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .missionbroker
            .sreputation_mission_requirement_expression_and
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .missionbroker
            .sreputation_mission_requirement_expression_and
    }
}

impl<'a> Extract<'a> for SReputationMissionRequirementExpression_And {
    const TYPE_NAME: &'static str = "SReputationMissionRequirementExpression_And";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SReputationMissionRequirementExpression_LeftParenthesis`
/// Inherits from: `SReputationMissionRequirementExpressionElement`
pub struct SReputationMissionRequirementExpression_LeftParenthesis {}

impl Pooled for SReputationMissionRequirementExpression_LeftParenthesis {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .missionbroker
            .sreputation_mission_requirement_expression_left_parenthesis
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .missionbroker
            .sreputation_mission_requirement_expression_left_parenthesis
    }
}

impl<'a> Extract<'a> for SReputationMissionRequirementExpression_LeftParenthesis {
    const TYPE_NAME: &'static str = "SReputationMissionRequirementExpression_LeftParenthesis";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SReputationMissionRequirementExpression_RightParenthesis`
/// Inherits from: `SReputationMissionRequirementExpressionElement`
pub struct SReputationMissionRequirementExpression_RightParenthesis {}

impl Pooled for SReputationMissionRequirementExpression_RightParenthesis {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .missionbroker
            .sreputation_mission_requirement_expression_right_parenthesis
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .missionbroker
            .sreputation_mission_requirement_expression_right_parenthesis
    }
}

impl<'a> Extract<'a> for SReputationMissionRequirementExpression_RightParenthesis {
    const TYPE_NAME: &'static str = "SReputationMissionRequirementExpression_RightParenthesis";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
