// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-entityclassdefinition_colorgradient`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ColorGradientComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct ColorGradientComponentParams {
    /// `texturePath` (String)
    pub texture_path: String,
    /// `fadeInTime` (Single)
    pub fade_in_time: f32,
    /// `priority` (Int32)
    pub priority: i32,
    /// `radius` (Single)
    pub radius: f32,
}

impl Pooled for ColorGradientComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_entityclassdefinition_colorgradient
            .color_gradient_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_entityclassdefinition_colorgradient
            .color_gradient_component_params
    }
}

impl<'a> Extract<'a> for ColorGradientComponentParams {
    const TYPE_NAME: &'static str = "ColorGradientComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            texture_path: inst
                .get_str("texturePath")
                .map(String::from)
                .unwrap_or_default(),
            fade_in_time: inst.get_f32("fadeInTime").unwrap_or_default(),
            priority: inst.get_i32("priority").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
        }
    }
}
