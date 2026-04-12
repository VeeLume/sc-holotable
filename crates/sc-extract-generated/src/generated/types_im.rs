// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `IMannequinActionDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IMannequinActionDef {
}

impl Pooled for IMannequinActionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_im.imannequin_action_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_im.imannequin_action_def }
}

impl<'a> Extract<'a> for IMannequinActionDef {
    const TYPE_NAME: &'static str = "IMannequinActionDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ImpactForceResistance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactForceResistance {
    /// DCB field: `impactForceResistance` (Single)
    #[serde(default)]
    pub impact_force_resistance: f32,
}

impl Pooled for ImpactForceResistance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_im.impact_force_resistance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_im.impact_force_resistance }
}

impl<'a> Extract<'a> for ImpactForceResistance {
    const TYPE_NAME: &'static str = "ImpactForceResistance";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            impact_force_resistance: inst.get_f32("impactForceResistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `ImpoundingDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpoundingDefinition {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `trigger` (EnumChoice)
    #[serde(default)]
    pub trigger: String,
    /// DCB field: `impoundingTimeSeconds` (Single)
    #[serde(default)]
    pub impounding_time_seconds: f32,
    /// DCB field: `impoundingFineUEC` (Int32)
    #[serde(default)]
    pub impounding_fine_uec: i32,
    /// DCB field: `ignoreIfAgainstPartyMember` (Boolean)
    #[serde(default)]
    pub ignore_if_against_party_member: bool,
}

impl Pooled for ImpoundingDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_im.impounding_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_im.impounding_definition }
}

impl<'a> Extract<'a> for ImpoundingDefinition {
    const TYPE_NAME: &'static str = "ImpoundingDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            trigger: inst.get_str("trigger").map(String::from).unwrap_or_default(),
            impounding_time_seconds: inst.get_f32("impoundingTimeSeconds").unwrap_or_default(),
            impounding_fine_uec: inst.get_i32("impoundingFineUEC").unwrap_or_default(),
            ignore_if_against_party_member: inst.get_bool("ignoreIfAgainstPartyMember").unwrap_or_default(),
        }
    }
}

