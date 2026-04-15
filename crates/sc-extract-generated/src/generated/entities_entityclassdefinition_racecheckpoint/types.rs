// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-entityclassdefinition_racecheckpoint`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `RaceCheckpointComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceCheckpointComponentParams {
    /// `checkpointNumber` (Int32)
    #[serde(default)]
    pub checkpoint_number: i32,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `bounds` (Class)
    #[serde(default)]
    pub bounds: Option<Handle<Vec3>>,
}

impl Pooled for RaceCheckpointComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_entityclassdefinition_racecheckpoint.race_checkpoint_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_entityclassdefinition_racecheckpoint.race_checkpoint_component_params }
}

impl<'a> Extract<'a> for RaceCheckpointComponentParams {
    const TYPE_NAME: &'static str = "RaceCheckpointComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            checkpoint_number: inst.get_i32("checkpointNumber").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            bounds: match inst.get("bounds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SRaceCheckpointObjectMetadataParams`
/// Inherits from: `SObjectMetadataParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRaceCheckpointObjectMetadataParams {
}

impl Pooled for SRaceCheckpointObjectMetadataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_entityclassdefinition_racecheckpoint.srace_checkpoint_object_metadata_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_entityclassdefinition_racecheckpoint.srace_checkpoint_object_metadata_params }
}

impl<'a> Extract<'a> for SRaceCheckpointObjectMetadataParams {
    const TYPE_NAME: &'static str = "SRaceCheckpointObjectMetadataParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SRaceCheckpointEntryTrackerParams`
/// Inherits from: `SObjectDataBankEntryTrackerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRaceCheckpointEntryTrackerParams {
}

impl Pooled for SRaceCheckpointEntryTrackerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_entityclassdefinition_racecheckpoint.srace_checkpoint_entry_tracker_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_entityclassdefinition_racecheckpoint.srace_checkpoint_entry_tracker_params }
}

impl<'a> Extract<'a> for SRaceCheckpointEntryTrackerParams {
    const TYPE_NAME: &'static str = "SRaceCheckpointEntryTrackerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

