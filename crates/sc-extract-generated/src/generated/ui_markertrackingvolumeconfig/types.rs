// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-markertrackingvolumeconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `MarkerTrackingCommonMapParameters`
pub struct MarkerTrackingCommonMapParameters {
    /// `minimumDistanceMultiplierCosmeticScaling` (Single)
    pub minimum_distance_multiplier_cosmetic_scaling: f32,
    /// `maximumDistanceMultiplierCosmeticScaling` (Single)
    pub maximum_distance_multiplier_cosmetic_scaling: f32,
    /// `cosmeticScalingSmoothingDistanceMultiplier` (Single)
    pub cosmetic_scaling_smoothing_distance_multiplier: f32,
    /// `framingRatioOfScreenSize` (Single)
    pub framing_ratio_of_screen_size: f32,
    /// `focusZoomDistanceMultiplier` (Single)
    pub focus_zoom_distance_multiplier: f32,
    /// `childlessMarkerRadiusMultiplier` (Single)
    pub childless_marker_radius_multiplier: f32,
    /// `lightScaleModifier` (Single)
    pub light_scale_modifier: f32,
    /// `zoomIncrement` (Single)
    pub zoom_increment: f32,
    /// `cameraBlendTimeInSeconds` (Single)
    pub camera_blend_time_in_seconds: f32,
    /// `labelParams` (Class)
    pub label_params: Option<Handle<MarkerTrackingLabelParameters>>,
}

impl Pooled for MarkerTrackingCommonMapParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_markertrackingvolumeconfig
            .marker_tracking_common_map_parameters
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_markertrackingvolumeconfig
            .marker_tracking_common_map_parameters
    }
}

impl<'a> Extract<'a> for MarkerTrackingCommonMapParameters {
    const TYPE_NAME: &'static str = "MarkerTrackingCommonMapParameters";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minimum_distance_multiplier_cosmetic_scaling: inst
                .get_f32("minimumDistanceMultiplierCosmeticScaling")
                .unwrap_or_default(),
            maximum_distance_multiplier_cosmetic_scaling: inst
                .get_f32("maximumDistanceMultiplierCosmeticScaling")
                .unwrap_or_default(),
            cosmetic_scaling_smoothing_distance_multiplier: inst
                .get_f32("cosmeticScalingSmoothingDistanceMultiplier")
                .unwrap_or_default(),
            framing_ratio_of_screen_size: inst
                .get_f32("framingRatioOfScreenSize")
                .unwrap_or_default(),
            focus_zoom_distance_multiplier: inst
                .get_f32("focusZoomDistanceMultiplier")
                .unwrap_or_default(),
            childless_marker_radius_multiplier: inst
                .get_f32("childlessMarkerRadiusMultiplier")
                .unwrap_or_default(),
            light_scale_modifier: inst.get_f32("lightScaleModifier").unwrap_or_default(),
            zoom_increment: inst.get_f32("zoomIncrement").unwrap_or_default(),
            camera_blend_time_in_seconds: inst
                .get_f32("cameraBlendTimeInSeconds")
                .unwrap_or_default(),
            label_params: match inst.get("labelParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<MarkerTrackingLabelParameters>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `MarkerTrackingLabelParameters`
pub struct MarkerTrackingLabelParameters {
    /// `headerTextScale` (Single)
    pub header_text_scale: f32,
    /// `subTextScale` (Single)
    pub sub_text_scale: f32,
    /// `minimumHeaderTextSize` (Single)
    pub minimum_header_text_size: f32,
    /// `maximumHeaderTextSize` (Single)
    pub maximum_header_text_size: f32,
    /// `minimumSize` (Single)
    pub minimum_size: f32,
    /// `maximumSize` (Single)
    pub maximum_size: f32,
    /// `minimumFadeOffset` (Single)
    pub minimum_fade_offset: f32,
    /// `maximumFadeOffset` (Single)
    pub maximum_fade_offset: f32,
    /// `positionOffsetMultiplier` (Single)
    pub position_offset_multiplier: f32,
}

impl Pooled for MarkerTrackingLabelParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_markertrackingvolumeconfig
            .marker_tracking_label_parameters
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_markertrackingvolumeconfig
            .marker_tracking_label_parameters
    }
}

impl<'a> Extract<'a> for MarkerTrackingLabelParameters {
    const TYPE_NAME: &'static str = "MarkerTrackingLabelParameters";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            header_text_scale: inst.get_f32("headerTextScale").unwrap_or_default(),
            sub_text_scale: inst.get_f32("subTextScale").unwrap_or_default(),
            minimum_header_text_size: inst.get_f32("minimumHeaderTextSize").unwrap_or_default(),
            maximum_header_text_size: inst.get_f32("maximumHeaderTextSize").unwrap_or_default(),
            minimum_size: inst.get_f32("minimumSize").unwrap_or_default(),
            maximum_size: inst.get_f32("maximumSize").unwrap_or_default(),
            minimum_fade_offset: inst.get_f32("minimumFadeOffset").unwrap_or_default(),
            maximum_fade_offset: inst.get_f32("maximumFadeOffset").unwrap_or_default(),
            position_offset_multiplier: inst
                .get_f32("positionOffsetMultiplier")
                .unwrap_or_default(),
        }
    }
}
