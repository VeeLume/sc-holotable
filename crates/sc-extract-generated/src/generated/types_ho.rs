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

/// DCB type: `HoldExhaleDuration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldExhaleDuration {
    /// DCB field: `breathHeldRatioMin` (Single)
    #[serde(default)]
    pub breath_held_ratio_min: f32,
    /// DCB field: `breathHeldRatioMax` (Single)
    #[serde(default)]
    pub breath_held_ratio_max: f32,
    /// DCB field: `duration` (Single)
    #[serde(default)]
    pub duration: f32,
}

impl Pooled for HoldExhaleDuration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ho.hold_exhale_duration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ho.hold_exhale_duration }
}

impl<'a> Extract<'a> for HoldExhaleDuration {
    const TYPE_NAME: &'static str = "HoldExhaleDuration";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            breath_held_ratio_min: inst.get_f32("breathHeldRatioMin").unwrap_or_default(),
            breath_held_ratio_max: inst.get_f32("breathHeldRatioMax").unwrap_or_default(),
            duration: inst.get_f32("duration").unwrap_or_default(),
        }
    }
}

/// DCB type: `HologramParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HologramParams {
    /// DCB field: `ValidMaterial` (String)
    #[serde(default)]
    pub valid_material: String,
    /// DCB field: `InvalidMaterial` (String)
    #[serde(default)]
    pub invalid_material: String,
}

impl Pooled for HologramParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ho.hologram_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ho.hologram_params }
}

impl<'a> Extract<'a> for HologramParams {
    const TYPE_NAME: &'static str = "HologramParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            valid_material: inst.get_str("ValidMaterial").map(String::from).unwrap_or_default(),
            invalid_material: inst.get_str("InvalidMaterial").map(String::from).unwrap_or_default(),
        }
    }
}

