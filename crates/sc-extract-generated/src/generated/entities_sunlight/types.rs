// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-sunlight`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SunLightComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SunLightComponentParams {
    /// `lightColor` (Class)
    pub light_color: Option<Handle<SRGB8>>,
    /// `radius` (Single)
    pub radius: f32,
    /// `surfaceIntensity` (Single)
    pub surface_intensity: f32,
    /// `surfaceColor` (Class)
    pub surface_color: Option<Handle<SRGB8>>,
    /// `dist1` (Single)
    pub dist1: f32,
    /// `dist2` (Single)
    pub dist2: f32,
    /// `dist3` (Single)
    pub dist3: f32,
    /// `dist4` (Single)
    pub dist4: f32,
    /// `distCull` (Single)
    pub dist_cull: f32,
    /// `intensity1` (Single)
    pub intensity1: f32,
    /// `intensity2` (Single)
    pub intensity2: f32,
    /// `intensity3` (Single)
    pub intensity3: f32,
    /// `intensity4` (Single)
    pub intensity4: f32,
}

impl Pooled for SunLightComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_sunlight.sun_light_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_sunlight.sun_light_component_params
    }
}

impl<'a> Extract<'a> for SunLightComponentParams {
    const TYPE_NAME: &'static str = "SunLightComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_color: match inst.get("lightColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            radius: inst.get_f32("radius").unwrap_or_default(),
            surface_intensity: inst.get_f32("surfaceIntensity").unwrap_or_default(),
            surface_color: match inst.get("surfaceColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            dist1: inst.get_f32("dist1").unwrap_or_default(),
            dist2: inst.get_f32("dist2").unwrap_or_default(),
            dist3: inst.get_f32("dist3").unwrap_or_default(),
            dist4: inst.get_f32("dist4").unwrap_or_default(),
            dist_cull: inst.get_f32("distCull").unwrap_or_default(),
            intensity1: inst.get_f32("intensity1").unwrap_or_default(),
            intensity2: inst.get_f32("intensity2").unwrap_or_default(),
            intensity3: inst.get_f32("intensity3").unwrap_or_default(),
            intensity4: inst.get_f32("intensity4").unwrap_or_default(),
        }
    }
}
