// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-quantumtrail`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SQuantumTrailParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SQuantumTrailParams {
    /// `destructionTime` (Single)
    pub destruction_time: f32,
    /// `trailLength` (Single)
    pub trail_length: f32,
    /// `viewableDistance` (Single)
    pub viewable_distance: f32,
    /// `maxVisibleLength` (Single)
    pub max_visible_length: f32,
}

impl Pooled for SQuantumTrailParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_quantumtrail.squantum_trail_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_quantumtrail.squantum_trail_params }
}

impl<'a> Extract<'a> for SQuantumTrailParams {
    const TYPE_NAME: &'static str = "SQuantumTrailParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            destruction_time: inst.get_f32("destructionTime").unwrap_or_default(),
            trail_length: inst.get_f32("trailLength").unwrap_or_default(),
            viewable_distance: inst.get_f32("viewableDistance").unwrap_or_default(),
            max_visible_length: inst.get_f32("maxVisibleLength").unwrap_or_default(),
        }
    }
}

