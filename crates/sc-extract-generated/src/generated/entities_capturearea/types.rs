// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-capturearea`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `AreaOutdoorMaterialParams`
pub struct AreaOutdoorMaterialParams {
    /// `gridOpacity` (Single)
    pub grid_opacity: f32,
    /// `gridOuterNoisePower` (Single)
    pub grid_outer_noise_power: f32,
    /// `beamOuterAlpha` (Single)
    pub beam_outer_alpha: f32,
    /// `beamGlowAmount` (Single)
    pub beam_glow_amount: f32,
}

impl Pooled for AreaOutdoorMaterialParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_capturearea.area_outdoor_material_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_capturearea.area_outdoor_material_params
    }
}

impl<'a> Extract<'a> for AreaOutdoorMaterialParams {
    const TYPE_NAME: &'static str = "AreaOutdoorMaterialParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            grid_opacity: inst.get_f32("gridOpacity").unwrap_or_default(),
            grid_outer_noise_power: inst.get_f32("gridOuterNoisePower").unwrap_or_default(),
            beam_outer_alpha: inst.get_f32("beamOuterAlpha").unwrap_or_default(),
            beam_glow_amount: inst.get_f32("beamGlowAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `CaptureAreaUIParams`
/// Inherits from: `DataForgeComponentParams`
pub struct CaptureAreaUIParams {
    /// `hidden` (Boolean)
    pub hidden: bool,
    /// `outdoorAreaUI` (Boolean)
    pub outdoor_area_ui: bool,
    /// `timeToLoop` (Single)
    pub time_to_loop: f32,
    /// `gridLayerCount` (Int32)
    pub grid_layer_count: i32,
    /// `dimensions` (Class)
    pub dimensions: Option<Handle<Vec3>>,
    /// `drawFarDistance` (Single)
    pub draw_far_distance: f32,
    /// `gridAreaHeightPerc` (Single)
    pub grid_area_height_perc: f32,
    /// `baseLineWidthPerc` (Single)
    pub base_line_width_perc: f32,
    /// `basePaddingInnerPerc` (Single)
    pub base_padding_inner_perc: f32,
    /// `baseOffset` (Single)
    pub base_offset: f32,
    /// `baseAlpha` (Single)
    pub base_alpha: f32,
    /// `baseNoisePower` (Single)
    pub base_noise_power: f32,
    /// `gridPaddingInnerPerc` (Single)
    pub grid_padding_inner_perc: f32,
    /// `gridLineWidthPerc` (Single)
    pub grid_line_width_perc: f32,
    /// `gridHighlightFeatherPerc` (Single)
    pub grid_highlight_feather_perc: f32,
    /// `gridFalloffPerc` (Single)
    pub grid_falloff_perc: f32,
    /// `indoorAreaParams` (Class)
    pub indoor_area_params: Option<Handle<AreaOutdoorMaterialParams>>,
    /// `outdoorAreaParams` (Class)
    pub outdoor_area_params: Option<Handle<AreaOutdoorMaterialParams>>,
    /// `baseLineMaterial` (String)
    pub base_line_material: String,
    /// `beamMaterial` (String)
    pub beam_material: String,
    /// `gridLineMaterial` (String)
    pub grid_line_material: String,
}

impl Pooled for CaptureAreaUIParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_capturearea.capture_area_uiparams
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_capturearea.capture_area_uiparams
    }
}

impl<'a> Extract<'a> for CaptureAreaUIParams {
    const TYPE_NAME: &'static str = "CaptureAreaUIParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hidden: inst.get_bool("hidden").unwrap_or_default(),
            outdoor_area_ui: inst.get_bool("outdoorAreaUI").unwrap_or_default(),
            time_to_loop: inst.get_f32("timeToLoop").unwrap_or_default(),
            grid_layer_count: inst.get_i32("gridLayerCount").unwrap_or_default(),
            dimensions: match inst.get("dimensions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            draw_far_distance: inst.get_f32("drawFarDistance").unwrap_or_default(),
            grid_area_height_perc: inst.get_f32("gridAreaHeightPerc").unwrap_or_default(),
            base_line_width_perc: inst.get_f32("baseLineWidthPerc").unwrap_or_default(),
            base_padding_inner_perc: inst.get_f32("basePaddingInnerPerc").unwrap_or_default(),
            base_offset: inst.get_f32("baseOffset").unwrap_or_default(),
            base_alpha: inst.get_f32("baseAlpha").unwrap_or_default(),
            base_noise_power: inst.get_f32("baseNoisePower").unwrap_or_default(),
            grid_padding_inner_perc: inst.get_f32("gridPaddingInnerPerc").unwrap_or_default(),
            grid_line_width_perc: inst.get_f32("gridLineWidthPerc").unwrap_or_default(),
            grid_highlight_feather_perc: inst
                .get_f32("gridHighlightFeatherPerc")
                .unwrap_or_default(),
            grid_falloff_perc: inst.get_f32("gridFalloffPerc").unwrap_or_default(),
            indoor_area_params: match inst.get("indoorAreaParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<AreaOutdoorMaterialParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            outdoor_area_params: match inst.get("outdoorAreaParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<AreaOutdoorMaterialParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            base_line_material: inst
                .get_str("baseLineMaterial")
                .map(String::from)
                .unwrap_or_default(),
            beam_material: inst
                .get_str("beamMaterial")
                .map(String::from)
                .unwrap_or_default(),
            grid_line_material: inst
                .get_str("gridLineMaterial")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}
