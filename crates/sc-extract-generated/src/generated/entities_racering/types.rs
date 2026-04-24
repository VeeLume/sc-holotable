// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-racering`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `RaceRingComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct RaceRingComponentParams {
    /// `radius` (Single)
    pub radius: f32,
    /// `openSpeed` (Single)
    pub open_speed: f32,
    /// `closeSpeed` (Single)
    pub close_speed: f32,
    /// `openAmount` (Single)
    pub open_amount: f32,
    /// `closeAmount` (Single)
    pub close_amount: f32,
    /// `innerRingModel` (String)
    pub inner_ring_model: String,
    /// `outerRingModel` (String)
    pub outer_ring_model: String,
    /// `irisModel` (String)
    pub iris_model: String,
    /// `openAnimName` (String)
    pub open_anim_name: String,
}

impl Pooled for RaceRingComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_racering.race_ring_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_racering.race_ring_component_params
    }
}

impl<'a> Extract<'a> for RaceRingComponentParams {
    const TYPE_NAME: &'static str = "RaceRingComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            radius: inst.get_f32("radius").unwrap_or_default(),
            open_speed: inst.get_f32("openSpeed").unwrap_or_default(),
            close_speed: inst.get_f32("closeSpeed").unwrap_or_default(),
            open_amount: inst.get_f32("openAmount").unwrap_or_default(),
            close_amount: inst.get_f32("closeAmount").unwrap_or_default(),
            inner_ring_model: inst
                .get_str("innerRingModel")
                .map(String::from)
                .unwrap_or_default(),
            outer_ring_model: inst
                .get_str("outerRingModel")
                .map(String::from)
                .unwrap_or_default(),
            iris_model: inst
                .get_str("irisModel")
                .map(String::from)
                .unwrap_or_default(),
            open_anim_name: inst
                .get_str("openAnimName")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}
