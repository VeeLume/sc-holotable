// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-analyticshadow`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SAnalyticShadowComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAnalyticShadowComponentParams {
    /// `active` (Boolean)
    #[serde(default)]
    pub active: bool,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `penumbraWidth` (Single)
    #[serde(default)]
    pub penumbra_width: f32,
    /// `maxRange` (Single)
    #[serde(default)]
    pub max_range: f32,
}

impl Pooled for SAnalyticShadowComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_analyticshadow.sanalytic_shadow_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_analyticshadow.sanalytic_shadow_component_params }
}

impl<'a> Extract<'a> for SAnalyticShadowComponentParams {
    const TYPE_NAME: &'static str = "SAnalyticShadowComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            active: inst.get_bool("active").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            penumbra_width: inst.get_f32("penumbraWidth").unwrap_or_default(),
            max_range: inst.get_f32("maxRange").unwrap_or_default(),
        }
    }
}

