// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `instancedinterior`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `InstancedInteriorLocationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancedInteriorLocationParams {
    /// `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// `devOnly` (Boolean)
    #[serde(default)]
    pub dev_only: bool,
    /// `defaultHangars` (Reference)
    #[serde(default)]
    pub default_hangars: Option<CigGuid>,
}

impl Pooled for InstancedInteriorLocationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.instancedinterior.instanced_interior_location_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.instancedinterior.instanced_interior_location_params }
}

impl<'a> Extract<'a> for InstancedInteriorLocationParams {
    const TYPE_NAME: &'static str = "InstancedInteriorLocationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            dev_only: inst.get_bool("devOnly").unwrap_or_default(),
            default_hangars: inst.get("defaultHangars").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InstancedInteriorLocationMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancedInteriorLocationMap {
    /// `exitTimeBuffer` (Single)
    #[serde(default)]
    pub exit_time_buffer: f32,
    /// `locationInteriors` (Reference (array))
    #[serde(default)]
    pub location_interiors: Vec<CigGuid>,
}

impl Pooled for InstancedInteriorLocationMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.instancedinterior.instanced_interior_location_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.instancedinterior.instanced_interior_location_map }
}

impl<'a> Extract<'a> for InstancedInteriorLocationMap {
    const TYPE_NAME: &'static str = "InstancedInteriorLocationMap";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            exit_time_buffer: inst.get_f32("exitTimeBuffer").unwrap_or_default(),
            location_interiors: inst.get_array("locationInteriors")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

