// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-render`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SGeomCacheEntityComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SGeomCacheEntityComponentParams {
    /// `GeomCacheFile` (String)
    pub geom_cache_file: String,
    /// `PlayOnStart` (Boolean)
    pub play_on_start: bool,
    /// `Loop` (Boolean)
    pub r#loop: bool,
    /// `Physicalize` (Boolean)
    pub physicalize: bool,
    /// `StartTime` (Single)
    pub start_time: f32,
    /// `StreamInDistance` (Single)
    pub stream_in_distance: f32,
    /// `StandInDistance` (Single)
    pub stand_in_distance: f32,
    /// `StandInObject` (String)
    pub stand_in_object: String,
    /// `StandInMaterial` (String)
    pub stand_in_material: String,
    /// `FirstFrameStandInObject` (String)
    pub first_frame_stand_in_object: String,
    /// `FirstFrameStandInMaterial` (String)
    pub first_frame_stand_in_material: String,
    /// `LastFrameStandInObject` (String)
    pub last_frame_stand_in_object: String,
    /// `LastFrameStandInMaterial` (String)
    pub last_frame_stand_in_material: String,
}

impl Pooled for SGeomCacheEntityComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_render.sgeom_cache_entity_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_render.sgeom_cache_entity_component_params }
}

impl<'a> Extract<'a> for SGeomCacheEntityComponentParams {
    const TYPE_NAME: &'static str = "SGeomCacheEntityComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            geom_cache_file: inst.get_str("GeomCacheFile").map(String::from).unwrap_or_default(),
            play_on_start: inst.get_bool("PlayOnStart").unwrap_or_default(),
            r#loop: inst.get_bool("Loop").unwrap_or_default(),
            physicalize: inst.get_bool("Physicalize").unwrap_or_default(),
            start_time: inst.get_f32("StartTime").unwrap_or_default(),
            stream_in_distance: inst.get_f32("StreamInDistance").unwrap_or_default(),
            stand_in_distance: inst.get_f32("StandInDistance").unwrap_or_default(),
            stand_in_object: inst.get_str("StandInObject").map(String::from).unwrap_or_default(),
            stand_in_material: inst.get_str("StandInMaterial").map(String::from).unwrap_or_default(),
            first_frame_stand_in_object: inst.get_str("FirstFrameStandInObject").map(String::from).unwrap_or_default(),
            first_frame_stand_in_material: inst.get_str("FirstFrameStandInMaterial").map(String::from).unwrap_or_default(),
            last_frame_stand_in_object: inst.get_str("LastFrameStandInObject").map(String::from).unwrap_or_default(),
            last_frame_stand_in_material: inst.get_str("LastFrameStandInMaterial").map(String::from).unwrap_or_default(),
        }
    }
}

