// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-refuelatmospheres`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SRefuelAtmosphereComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SRefuelAtmosphereComponentParams {
    /// `boostRefuelMultiplier` (Single)
    pub boost_refuel_multiplier: f32,
}

impl Pooled for SRefuelAtmosphereComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_refuelatmospheres
            .srefuel_atmosphere_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_refuelatmospheres
            .srefuel_atmosphere_component_params
    }
}

impl<'a> Extract<'a> for SRefuelAtmosphereComponentParams {
    const TYPE_NAME: &'static str = "SRefuelAtmosphereComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            boost_refuel_multiplier: inst.get_f32("boostRefuelMultiplier").unwrap_or_default(),
        }
    }
}
