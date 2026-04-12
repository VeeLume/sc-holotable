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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MarkerTrackingViewModeParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerTrackingViewModeParameters {
    /// `isFullScreen` (Boolean)
    #[serde(default)]
    pub is_full_screen: bool,
    /// `rotateVertical` (Boolean)
    #[serde(default)]
    pub rotate_vertical: bool,
    /// `rotateHorizontal` (Boolean)
    #[serde(default)]
    pub rotate_horizontal: bool,
    /// `pan` (Boolean)
    #[serde(default)]
    pub pan: bool,
    /// `zoom` (Boolean)
    #[serde(default)]
    pub zoom: bool,
    /// `markerActions` (Class)
    #[serde(default)]
    pub marker_actions: Option<Handle<MarkerTrackingActionParameters>>,
    /// `displaySettings` (Class)
    #[serde(default)]
    pub display_settings: Option<Handle<MarkerTrackingDisplayParameters>>,
}

impl Pooled for MarkerTrackingViewModeParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_markertrackingvolumeconfig.marker_tracking_view_mode_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_markertrackingvolumeconfig.marker_tracking_view_mode_parameters }
}

impl<'a> Extract<'a> for MarkerTrackingViewModeParameters {
    const TYPE_NAME: &'static str = "MarkerTrackingViewModeParameters";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            is_full_screen: inst.get_bool("isFullScreen").unwrap_or_default(),
            rotate_vertical: inst.get_bool("rotateVertical").unwrap_or_default(),
            rotate_horizontal: inst.get_bool("rotateHorizontal").unwrap_or_default(),
            pan: inst.get_bool("pan").unwrap_or_default(),
            zoom: inst.get_bool("zoom").unwrap_or_default(),
            marker_actions: match inst.get("markerActions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MarkerTrackingActionParameters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MarkerTrackingActionParameters>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            display_settings: match inst.get("displaySettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MarkerTrackingDisplayParameters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MarkerTrackingDisplayParameters>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MarkerTrackingCommonMapParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerTrackingCommonMapParameters {
    /// `minimumDistanceMultiplierCosmeticScaling` (Single)
    #[serde(default)]
    pub minimum_distance_multiplier_cosmetic_scaling: f32,
    /// `maximumDistanceMultiplierCosmeticScaling` (Single)
    #[serde(default)]
    pub maximum_distance_multiplier_cosmetic_scaling: f32,
    /// `cosmeticScalingSmoothingDistanceMultiplier` (Single)
    #[serde(default)]
    pub cosmetic_scaling_smoothing_distance_multiplier: f32,
    /// `framingRatioOfScreenSize` (Single)
    #[serde(default)]
    pub framing_ratio_of_screen_size: f32,
    /// `focusZoomDistanceMultiplier` (Single)
    #[serde(default)]
    pub focus_zoom_distance_multiplier: f32,
    /// `childlessMarkerRadiusMultiplier` (Single)
    #[serde(default)]
    pub childless_marker_radius_multiplier: f32,
    /// `lightScaleModifier` (Single)
    #[serde(default)]
    pub light_scale_modifier: f32,
    /// `zoomIncrement` (Single)
    #[serde(default)]
    pub zoom_increment: f32,
    /// `cameraBlendTimeInSeconds` (Single)
    #[serde(default)]
    pub camera_blend_time_in_seconds: f32,
    /// `labelParams` (Class)
    #[serde(default)]
    pub label_params: Option<Handle<MarkerTrackingLabelParameters>>,
}

impl Pooled for MarkerTrackingCommonMapParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_markertrackingvolumeconfig.marker_tracking_common_map_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_markertrackingvolumeconfig.marker_tracking_common_map_parameters }
}

impl<'a> Extract<'a> for MarkerTrackingCommonMapParameters {
    const TYPE_NAME: &'static str = "MarkerTrackingCommonMapParameters";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            minimum_distance_multiplier_cosmetic_scaling: inst.get_f32("minimumDistanceMultiplierCosmeticScaling").unwrap_or_default(),
            maximum_distance_multiplier_cosmetic_scaling: inst.get_f32("maximumDistanceMultiplierCosmeticScaling").unwrap_or_default(),
            cosmetic_scaling_smoothing_distance_multiplier: inst.get_f32("cosmeticScalingSmoothingDistanceMultiplier").unwrap_or_default(),
            framing_ratio_of_screen_size: inst.get_f32("framingRatioOfScreenSize").unwrap_or_default(),
            focus_zoom_distance_multiplier: inst.get_f32("focusZoomDistanceMultiplier").unwrap_or_default(),
            childless_marker_radius_multiplier: inst.get_f32("childlessMarkerRadiusMultiplier").unwrap_or_default(),
            light_scale_modifier: inst.get_f32("lightScaleModifier").unwrap_or_default(),
            zoom_increment: inst.get_f32("zoomIncrement").unwrap_or_default(),
            camera_blend_time_in_seconds: inst.get_f32("cameraBlendTimeInSeconds").unwrap_or_default(),
            label_params: match inst.get("labelParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MarkerTrackingLabelParameters>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MarkerTrackingLabelParameters>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MarkerTrackingActionParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerTrackingActionParameters {
    /// `leftClickAction` (EnumChoice)
    #[serde(default)]
    pub left_click_action: String,
    /// `leftDoubleClickAction` (EnumChoice)
    #[serde(default)]
    pub left_double_click_action: String,
}

impl Pooled for MarkerTrackingActionParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_markertrackingvolumeconfig.marker_tracking_action_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_markertrackingvolumeconfig.marker_tracking_action_parameters }
}

impl<'a> Extract<'a> for MarkerTrackingActionParameters {
    const TYPE_NAME: &'static str = "MarkerTrackingActionParameters";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            left_click_action: inst.get_str("leftClickAction").map(String::from).unwrap_or_default(),
            left_double_click_action: inst.get_str("leftDoubleClickAction").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MarkerTrackingDisplayParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerTrackingDisplayParameters {
    /// `showSmallIconOnly` (Boolean)
    #[serde(default)]
    pub show_small_icon_only: bool,
    /// `showPanels` (Boolean)
    #[serde(default)]
    pub show_panels: bool,
    /// `showEdgeMarkers` (Boolean)
    #[serde(default)]
    pub show_edge_markers: bool,
    /// `enableDynamicRadar` (Boolean)
    #[serde(default)]
    pub enable_dynamic_radar: bool,
    /// `planeAlignmentMode` (EnumChoice)
    #[serde(default)]
    pub plane_alignment_mode: String,
    /// `minimumRadarRangeInMeters` (Single)
    #[serde(default)]
    pub minimum_radar_range_in_meters: f32,
    /// `defaultRadarRangeInMeters` (Single)
    #[serde(default)]
    pub default_radar_range_in_meters: f32,
    /// `radarPaddingInMeters` (Single)
    #[serde(default)]
    pub radar_padding_in_meters: f32,
    /// `playerZoomOffset` (Single)
    #[serde(default)]
    pub player_zoom_offset: f32,
    /// `iconOverrideThreshold` (Single)
    #[serde(default)]
    pub icon_override_threshold: f32,
    /// `standardIconThreshold` (Single)
    #[serde(default)]
    pub standard_icon_threshold: f32,
    /// `modelThreshold` (Single)
    #[serde(default)]
    pub model_threshold: f32,
}

impl Pooled for MarkerTrackingDisplayParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_markertrackingvolumeconfig.marker_tracking_display_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_markertrackingvolumeconfig.marker_tracking_display_parameters }
}

impl<'a> Extract<'a> for MarkerTrackingDisplayParameters {
    const TYPE_NAME: &'static str = "MarkerTrackingDisplayParameters";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            show_small_icon_only: inst.get_bool("showSmallIconOnly").unwrap_or_default(),
            show_panels: inst.get_bool("showPanels").unwrap_or_default(),
            show_edge_markers: inst.get_bool("showEdgeMarkers").unwrap_or_default(),
            enable_dynamic_radar: inst.get_bool("enableDynamicRadar").unwrap_or_default(),
            plane_alignment_mode: inst.get_str("planeAlignmentMode").map(String::from).unwrap_or_default(),
            minimum_radar_range_in_meters: inst.get_f32("minimumRadarRangeInMeters").unwrap_or_default(),
            default_radar_range_in_meters: inst.get_f32("defaultRadarRangeInMeters").unwrap_or_default(),
            radar_padding_in_meters: inst.get_f32("radarPaddingInMeters").unwrap_or_default(),
            player_zoom_offset: inst.get_f32("playerZoomOffset").unwrap_or_default(),
            icon_override_threshold: inst.get_f32("iconOverrideThreshold").unwrap_or_default(),
            standard_icon_threshold: inst.get_f32("standardIconThreshold").unwrap_or_default(),
            model_threshold: inst.get_f32("modelThreshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `MarkerTrackingLabelParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerTrackingLabelParameters {
    /// `headerTextScale` (Single)
    #[serde(default)]
    pub header_text_scale: f32,
    /// `subTextScale` (Single)
    #[serde(default)]
    pub sub_text_scale: f32,
    /// `minimumHeaderTextSize` (Single)
    #[serde(default)]
    pub minimum_header_text_size: f32,
    /// `maximumHeaderTextSize` (Single)
    #[serde(default)]
    pub maximum_header_text_size: f32,
    /// `minimumSize` (Single)
    #[serde(default)]
    pub minimum_size: f32,
    /// `maximumSize` (Single)
    #[serde(default)]
    pub maximum_size: f32,
    /// `minimumFadeOffset` (Single)
    #[serde(default)]
    pub minimum_fade_offset: f32,
    /// `maximumFadeOffset` (Single)
    #[serde(default)]
    pub maximum_fade_offset: f32,
    /// `positionOffsetMultiplier` (Single)
    #[serde(default)]
    pub position_offset_multiplier: f32,
}

impl Pooled for MarkerTrackingLabelParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_markertrackingvolumeconfig.marker_tracking_label_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_markertrackingvolumeconfig.marker_tracking_label_parameters }
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
            position_offset_multiplier: inst.get_f32("positionOffsetMultiplier").unwrap_or_default(),
        }
    }
}

