// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-watervolume`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `WaterShapeComponentParams`
/// Inherits from: `AreaShapeComponentParams`
pub struct WaterShapeComponentParams {}

impl Pooled for WaterShapeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_watervolume.water_shape_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_watervolume.water_shape_component_params
    }
}

impl<'a> Extract<'a> for WaterShapeComponentParams {
    const TYPE_NAME: &'static str = "WaterShapeComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `EntityComponentWaterVolumeParams`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentWaterVolumeParams {}

impl Pooled for EntityComponentWaterVolumeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_watervolume
            .entity_component_water_volume_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_watervolume
            .entity_component_water_volume_params
    }
}

impl<'a> Extract<'a> for EntityComponentWaterVolumeParams {
    const TYPE_NAME: &'static str = "EntityComponentWaterVolumeParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
