// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-rastar`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `BaseBuildingParams`
/// Inherits from: `DataForgeComponentParams`
pub struct BaseBuildingParams {}

impl Pooled for BaseBuildingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_rastar.base_building_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_rastar.base_building_params
    }
}

impl<'a> Extract<'a> for BaseBuildingParams {
    const TYPE_NAME: &'static str = "BaseBuildingParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `ObjectContainerModifierParams`
/// Inherits from: `DataForgeComponentParams`
pub struct ObjectContainerModifierParams {
    /// `ExternalWear` (Single)
    pub external_wear: f32,
    /// `ExternalDirt` (Single)
    pub external_dirt: f32,
    /// `InternalWear` (Single)
    pub internal_wear: f32,
    /// `InternalDirt` (Single)
    pub internal_dirt: f32,
    /// `TintPaletteOverride` (Reference)
    pub tint_palette_override: Option<CigGuid>,
}

impl Pooled for ObjectContainerModifierParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_rastar.object_container_modifier_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_rastar.object_container_modifier_params
    }
}

impl<'a> Extract<'a> for ObjectContainerModifierParams {
    const TYPE_NAME: &'static str = "ObjectContainerModifierParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            external_wear: inst.get_f32("ExternalWear").unwrap_or_default(),
            external_dirt: inst.get_f32("ExternalDirt").unwrap_or_default(),
            internal_wear: inst.get_f32("InternalWear").unwrap_or_default(),
            internal_dirt: inst.get_f32("InternalDirt").unwrap_or_default(),
            tint_palette_override: inst
                .get("TintPaletteOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `RastarLocationParams`
/// Inherits from: `DataForgeComponentParams`
pub struct RastarLocationParams {
    /// `actionAreaRadius` (Single)
    pub action_area_radius: f32,
    /// `locationSize` (Single)
    pub location_size: f32,
    /// `externalWear` (Single)
    pub external_wear: f32,
    /// `externalDirt` (Single)
    pub external_dirt: f32,
    /// `internalWear` (Single)
    pub internal_wear: f32,
    /// `internalDirt` (Single)
    pub internal_dirt: f32,
    /// `tintPalette` (Reference)
    pub tint_palette: Option<CigGuid>,
}

impl Pooled for RastarLocationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_rastar.rastar_location_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_rastar.rastar_location_params
    }
}

impl<'a> Extract<'a> for RastarLocationParams {
    const TYPE_NAME: &'static str = "RastarLocationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            action_area_radius: inst.get_f32("actionAreaRadius").unwrap_or_default(),
            location_size: inst.get_f32("locationSize").unwrap_or_default(),
            external_wear: inst.get_f32("externalWear").unwrap_or_default(),
            external_dirt: inst.get_f32("externalDirt").unwrap_or_default(),
            internal_wear: inst.get_f32("internalWear").unwrap_or_default(),
            internal_dirt: inst.get_f32("internalDirt").unwrap_or_default(),
            tint_palette: inst
                .get("tintPalette")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `RastarUIParams`
/// Inherits from: `DataForgeComponentParams`
pub struct RastarUIParams {}

impl Pooled for RastarUIParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_rastar.rastar_uiparams
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_rastar.rastar_uiparams
    }
}

impl<'a> Extract<'a> for RastarUIParams {
    const TYPE_NAME: &'static str = "RastarUIParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
