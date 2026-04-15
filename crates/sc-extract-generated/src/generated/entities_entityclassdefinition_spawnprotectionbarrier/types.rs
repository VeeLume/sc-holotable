// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-entityclassdefinition_spawnprotectionbarrier`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SSpawnProtectionBarrierParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSpawnProtectionBarrierParams {
    /// `team` (Reference)
    #[serde(default)]
    pub team: Option<CigGuid>,
    /// `phase` (Int32)
    #[serde(default)]
    pub phase: i32,
    /// `deadZoneInactiveTime` (Int32)
    #[serde(default)]
    pub dead_zone_inactive_time: i32,
}

impl Pooled for SSpawnProtectionBarrierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_entityclassdefinition_spawnprotectionbarrier.sspawn_protection_barrier_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_entityclassdefinition_spawnprotectionbarrier.sspawn_protection_barrier_params }
}

impl<'a> Extract<'a> for SSpawnProtectionBarrierParams {
    const TYPE_NAME: &'static str = "SSpawnProtectionBarrierParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            team: inst.get("team").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            phase: inst.get_i32("phase").unwrap_or_default(),
            dead_zone_inactive_time: inst.get_i32("deadZoneInactiveTime").unwrap_or_default(),
        }
    }
}

