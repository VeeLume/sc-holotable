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

/// DCB type: `ApparentTemperatureParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApparentTemperatureParams {
    /// DCB field: `heatIndexBlendThreshold` (Double)
    #[serde(default)]
    pub heat_index_blend_threshold: f64,
    /// DCB field: `heatIndexHardThreshold` (Double)
    #[serde(default)]
    pub heat_index_hard_threshold: f64,
    /// DCB field: `windChillBlendThreshold` (Double)
    #[serde(default)]
    pub wind_chill_blend_threshold: f64,
    /// DCB field: `windChillHardThreshold` (Double)
    #[serde(default)]
    pub wind_chill_hard_threshold: f64,
    /// DCB field: `maxPressureForScaling` (Double)
    #[serde(default)]
    pub max_pressure_for_scaling: f64,
    /// DCB field: `defaultTemperatureForPressureScaling` (Double)
    #[serde(default)]
    pub default_temperature_for_pressure_scaling: f64,
}

impl Pooled for ApparentTemperatureParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ap.apparent_temperature_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ap.apparent_temperature_params }
}

impl<'a> Extract<'a> for ApparentTemperatureParams {
    const TYPE_NAME: &'static str = "ApparentTemperatureParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            heat_index_blend_threshold: inst.get_f64("heatIndexBlendThreshold").unwrap_or_default(),
            heat_index_hard_threshold: inst.get_f64("heatIndexHardThreshold").unwrap_or_default(),
            wind_chill_blend_threshold: inst.get_f64("windChillBlendThreshold").unwrap_or_default(),
            wind_chill_hard_threshold: inst.get_f64("windChillHardThreshold").unwrap_or_default(),
            max_pressure_for_scaling: inst.get_f64("maxPressureForScaling").unwrap_or_default(),
            default_temperature_for_pressure_scaling: inst.get_f64("defaultTemperatureForPressureScaling").unwrap_or_default(),
        }
    }
}

