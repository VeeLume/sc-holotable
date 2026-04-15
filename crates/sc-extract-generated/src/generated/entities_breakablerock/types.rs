// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-breakablerock`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `GeometryAsteroidModelTag`
/// Inherits from: `SGeometryModelTagBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryAsteroidModelTag {
    /// `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
}

impl Pooled for GeometryAsteroidModelTag {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_breakablerock.geometry_asteroid_model_tag }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_breakablerock.geometry_asteroid_model_tag }
}

impl<'a> Extract<'a> for GeometryAsteroidModelTag {
    const TYPE_NAME: &'static str = "GeometryAsteroidModelTag";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `LightningTargetParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningTargetParams {
    /// `overrideSurfaceTypeConductivity` (Boolean)
    #[serde(default)]
    pub override_surface_type_conductivity: bool,
    /// `conductivityOverride` (Single)
    #[serde(default)]
    pub conductivity_override: f32,
    /// `conductivityMultiplier` (Single)
    #[serde(default)]
    pub conductivity_multiplier: f32,
    /// `resistance` (Single)
    #[serde(default)]
    pub resistance: f32,
    /// `targetRadiusOverride` (Single)
    #[serde(default)]
    pub target_radius_override: f32,
}

impl Pooled for LightningTargetParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_breakablerock.lightning_target_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_breakablerock.lightning_target_params }
}

impl<'a> Extract<'a> for LightningTargetParams {
    const TYPE_NAME: &'static str = "LightningTargetParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            override_surface_type_conductivity: inst.get_bool("overrideSurfaceTypeConductivity").unwrap_or_default(),
            conductivity_override: inst.get_f32("conductivityOverride").unwrap_or_default(),
            conductivity_multiplier: inst.get_f32("conductivityMultiplier").unwrap_or_default(),
            resistance: inst.get_f32("resistance").unwrap_or_default(),
            target_radius_override: inst.get_f32("targetRadiusOverride").unwrap_or_default(),
        }
    }
}

