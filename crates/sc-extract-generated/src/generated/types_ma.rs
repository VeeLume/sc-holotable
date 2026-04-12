// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `MasterModeExclusion`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterModeExclusion {
    /// DCB field: `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// DCB field: `masterModeExclusions` (EnumChoice (array))
    #[serde(default)]
    pub master_mode_exclusions: Vec<String>,
}

impl Pooled for MasterModeExclusion {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.master_mode_exclusion }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.master_mode_exclusion }
}

impl<'a> Extract<'a> for MasterModeExclusion {
    const TYPE_NAME: &'static str = "MasterModeExclusion";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("itemType").map(String::from).unwrap_or_default(),
            master_mode_exclusions: inst.get_array("masterModeExclusions")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MasterModeExclusionGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterModeExclusionGlobalParams {
    /// DCB field: `exclusions` (Class (array))
    #[serde(default)]
    pub exclusions: Vec<Handle<MasterModeExclusion>>,
}

impl Pooled for MasterModeExclusionGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.master_mode_exclusion_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.master_mode_exclusion_global_params }
}

impl<'a> Extract<'a> for MasterModeExclusionGlobalParams {
    const TYPE_NAME: &'static str = "MasterModeExclusionGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            exclusions: inst.get_array("exclusions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MasterModeExclusion>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MasterModeExclusion>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MaterialEffectEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialEffectEntry {
    /// DCB field: `libraryName` (String)
    #[serde(default)]
    pub library_name: String,
    /// DCB field: `effectName` (String)
    #[serde(default)]
    pub effect_name: String,
}

impl Pooled for MaterialEffectEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.material_effect_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.material_effect_entry }
}

impl<'a> Extract<'a> for MaterialEffectEntry {
    const TYPE_NAME: &'static str = "MaterialEffectEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            library_name: inst.get_str("libraryName").map(String::from).unwrap_or_default(),
            effect_name: inst.get_str("effectName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MarkerTrackingViewModeParameters`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerTrackingViewModeParameters {
    /// DCB field: `isFullScreen` (Boolean)
    #[serde(default)]
    pub is_full_screen: bool,
    /// DCB field: `rotateVertical` (Boolean)
    #[serde(default)]
    pub rotate_vertical: bool,
    /// DCB field: `rotateHorizontal` (Boolean)
    #[serde(default)]
    pub rotate_horizontal: bool,
    /// DCB field: `pan` (Boolean)
    #[serde(default)]
    pub pan: bool,
    /// DCB field: `zoom` (Boolean)
    #[serde(default)]
    pub zoom: bool,
    /// DCB field: `markerActions` (Class)
    #[serde(default)]
    pub marker_actions: Option<Handle<MarkerTrackingActionParameters>>,
    /// DCB field: `displaySettings` (Class)
    #[serde(default)]
    pub display_settings: Option<Handle<MarkerTrackingDisplayParameters>>,
}

impl Pooled for MarkerTrackingViewModeParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_tracking_view_mode_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_tracking_view_mode_parameters }
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
    /// DCB field: `minimumDistanceMultiplierCosmeticScaling` (Single)
    #[serde(default)]
    pub minimum_distance_multiplier_cosmetic_scaling: f32,
    /// DCB field: `maximumDistanceMultiplierCosmeticScaling` (Single)
    #[serde(default)]
    pub maximum_distance_multiplier_cosmetic_scaling: f32,
    /// DCB field: `cosmeticScalingSmoothingDistanceMultiplier` (Single)
    #[serde(default)]
    pub cosmetic_scaling_smoothing_distance_multiplier: f32,
    /// DCB field: `framingRatioOfScreenSize` (Single)
    #[serde(default)]
    pub framing_ratio_of_screen_size: f32,
    /// DCB field: `focusZoomDistanceMultiplier` (Single)
    #[serde(default)]
    pub focus_zoom_distance_multiplier: f32,
    /// DCB field: `childlessMarkerRadiusMultiplier` (Single)
    #[serde(default)]
    pub childless_marker_radius_multiplier: f32,
    /// DCB field: `lightScaleModifier` (Single)
    #[serde(default)]
    pub light_scale_modifier: f32,
    /// DCB field: `zoomIncrement` (Single)
    #[serde(default)]
    pub zoom_increment: f32,
    /// DCB field: `cameraBlendTimeInSeconds` (Single)
    #[serde(default)]
    pub camera_blend_time_in_seconds: f32,
    /// DCB field: `labelParams` (Class)
    #[serde(default)]
    pub label_params: Option<Handle<MarkerTrackingLabelParameters>>,
}

impl Pooled for MarkerTrackingCommonMapParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_tracking_common_map_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_tracking_common_map_parameters }
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
    /// DCB field: `leftClickAction` (EnumChoice)
    #[serde(default)]
    pub left_click_action: String,
    /// DCB field: `leftDoubleClickAction` (EnumChoice)
    #[serde(default)]
    pub left_double_click_action: String,
}

impl Pooled for MarkerTrackingActionParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_tracking_action_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_tracking_action_parameters }
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
    /// DCB field: `showSmallIconOnly` (Boolean)
    #[serde(default)]
    pub show_small_icon_only: bool,
    /// DCB field: `showPanels` (Boolean)
    #[serde(default)]
    pub show_panels: bool,
    /// DCB field: `showEdgeMarkers` (Boolean)
    #[serde(default)]
    pub show_edge_markers: bool,
    /// DCB field: `enableDynamicRadar` (Boolean)
    #[serde(default)]
    pub enable_dynamic_radar: bool,
    /// DCB field: `planeAlignmentMode` (EnumChoice)
    #[serde(default)]
    pub plane_alignment_mode: String,
    /// DCB field: `minimumRadarRangeInMeters` (Single)
    #[serde(default)]
    pub minimum_radar_range_in_meters: f32,
    /// DCB field: `defaultRadarRangeInMeters` (Single)
    #[serde(default)]
    pub default_radar_range_in_meters: f32,
    /// DCB field: `radarPaddingInMeters` (Single)
    #[serde(default)]
    pub radar_padding_in_meters: f32,
    /// DCB field: `playerZoomOffset` (Single)
    #[serde(default)]
    pub player_zoom_offset: f32,
    /// DCB field: `iconOverrideThreshold` (Single)
    #[serde(default)]
    pub icon_override_threshold: f32,
    /// DCB field: `standardIconThreshold` (Single)
    #[serde(default)]
    pub standard_icon_threshold: f32,
    /// DCB field: `modelThreshold` (Single)
    #[serde(default)]
    pub model_threshold: f32,
}

impl Pooled for MarkerTrackingDisplayParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_tracking_display_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_tracking_display_parameters }
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
    /// DCB field: `headerTextScale` (Single)
    #[serde(default)]
    pub header_text_scale: f32,
    /// DCB field: `subTextScale` (Single)
    #[serde(default)]
    pub sub_text_scale: f32,
    /// DCB field: `minimumHeaderTextSize` (Single)
    #[serde(default)]
    pub minimum_header_text_size: f32,
    /// DCB field: `maximumHeaderTextSize` (Single)
    #[serde(default)]
    pub maximum_header_text_size: f32,
    /// DCB field: `minimumSize` (Single)
    #[serde(default)]
    pub minimum_size: f32,
    /// DCB field: `maximumSize` (Single)
    #[serde(default)]
    pub maximum_size: f32,
    /// DCB field: `minimumFadeOffset` (Single)
    #[serde(default)]
    pub minimum_fade_offset: f32,
    /// DCB field: `maximumFadeOffset` (Single)
    #[serde(default)]
    pub maximum_fade_offset: f32,
    /// DCB field: `positionOffsetMultiplier` (Single)
    #[serde(default)]
    pub position_offset_multiplier: f32,
}

impl Pooled for MarkerTrackingLabelParameters {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_tracking_label_parameters }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_tracking_label_parameters }
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

/// DCB type: `MarkerDeclutteringCullingOrder`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerDeclutteringCullingOrder {
    /// DCB field: `systemLimit` (Int32)
    #[serde(default)]
    pub system_limit: i32,
    /// DCB field: `standardLimit` (Int32)
    #[serde(default)]
    pub standard_limit: i32,
    /// DCB field: `revealedLimit` (Int32)
    #[serde(default)]
    pub revealed_limit: i32,
    /// DCB field: `revealDuration` (Single)
    #[serde(default)]
    pub reveal_duration: f32,
    /// DCB field: `referenceDistance` (Single)
    #[serde(default)]
    pub reference_distance: f32,
    /// DCB field: `distanceWeight` (Single)
    #[serde(default)]
    pub distance_weight: f32,
    /// DCB field: `screenDistanceWeight` (Single)
    #[serde(default)]
    pub screen_distance_weight: f32,
    /// DCB field: `categories` (EnumChoice (array))
    #[serde(default)]
    pub categories: Vec<String>,
}

impl Pooled for MarkerDeclutteringCullingOrder {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_decluttering_culling_order }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_decluttering_culling_order }
}

impl<'a> Extract<'a> for MarkerDeclutteringCullingOrder {
    const TYPE_NAME: &'static str = "MarkerDeclutteringCullingOrder";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            system_limit: inst.get_i32("systemLimit").unwrap_or_default(),
            standard_limit: inst.get_i32("standardLimit").unwrap_or_default(),
            revealed_limit: inst.get_i32("revealedLimit").unwrap_or_default(),
            reveal_duration: inst.get_f32("revealDuration").unwrap_or_default(),
            reference_distance: inst.get_f32("referenceDistance").unwrap_or_default(),
            distance_weight: inst.get_f32("distanceWeight").unwrap_or_default(),
            screen_distance_weight: inst.get_f32("screenDistanceWeight").unwrap_or_default(),
            categories: inst.get_array("categories")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `Marker_Configuration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_Configuration {
    /// DCB field: `defaultMaterial` (String)
    #[serde(default)]
    pub default_material: String,
    /// DCB field: `defaultModel` (String)
    #[serde(default)]
    pub default_model: String,
    /// DCB field: `offScreenModel` (String)
    #[serde(default)]
    pub off_screen_model: String,
    /// DCB field: `objectiveStyleModel` (String)
    #[serde(default)]
    pub objective_style_model: String,
    /// DCB field: `alwaysShowOffScreenModel` (Boolean)
    #[serde(default)]
    pub always_show_off_screen_model: bool,
    /// DCB field: `ARcullingCategory` (EnumChoice)
    #[serde(default)]
    pub arculling_category: String,
    /// DCB field: `useSmallIcon` (Boolean)
    #[serde(default)]
    pub use_small_icon: bool,
    /// DCB field: `useStandardIcon` (Boolean)
    #[serde(default)]
    pub use_standard_icon: bool,
    /// DCB field: `useARIcon` (Boolean)
    #[serde(default)]
    pub use_aricon: bool,
    /// DCB field: `useModel` (Boolean)
    #[serde(default)]
    pub use_model: bool,
    /// DCB field: `smallIconIndex` (Int32)
    #[serde(default)]
    pub small_icon_index: i32,
    /// DCB field: `standardIconIndex` (Int32)
    #[serde(default)]
    pub standard_icon_index: i32,
    /// DCB field: `smallIcon` (String)
    #[serde(default)]
    pub small_icon: String,
    /// DCB field: `standardIcon` (String)
    #[serde(default)]
    pub standard_icon: String,
    /// DCB field: `mapLabelDisplayType` (EnumChoice)
    #[serde(default)]
    pub map_label_display_type: String,
    /// DCB field: `stackPositionAlignment` (EnumChoice)
    #[serde(default)]
    pub stack_position_alignment: String,
    /// DCB field: `layoutCanvas` (Reference)
    #[serde(default)]
    pub layout_canvas: Option<CigGuid>,
    /// DCB field: `abilities` (StrongPointer (array))
    #[serde(default)]
    pub abilities: Vec<Handle<Marker_AbilityBase>>,
    /// DCB field: `showRules` (StrongPointer (array))
    #[serde(default)]
    pub show_rules: Vec<Handle<Marker_ShowRule>>,
    /// DCB field: `mapShowRules` (Class (array))
    #[serde(default)]
    pub map_show_rules: Vec<Handle<Marker_ShowRuleMapDisplayMode>>,
    /// DCB field: `mapBoxoutSectionTypes` (EnumChoice (array))
    #[serde(default)]
    pub map_boxout_section_types: Vec<String>,
    /// DCB field: `markerOffset` (Class)
    #[serde(default)]
    pub marker_offset: Option<Handle<Vec3>>,
}

impl Pooled for Marker_Configuration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_configuration }
}

impl<'a> Extract<'a> for Marker_Configuration {
    const TYPE_NAME: &'static str = "Marker_Configuration";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_material: inst.get_str("defaultMaterial").map(String::from).unwrap_or_default(),
            default_model: inst.get_str("defaultModel").map(String::from).unwrap_or_default(),
            off_screen_model: inst.get_str("offScreenModel").map(String::from).unwrap_or_default(),
            objective_style_model: inst.get_str("objectiveStyleModel").map(String::from).unwrap_or_default(),
            always_show_off_screen_model: inst.get_bool("alwaysShowOffScreenModel").unwrap_or_default(),
            arculling_category: inst.get_str("ARcullingCategory").map(String::from).unwrap_or_default(),
            use_small_icon: inst.get_bool("useSmallIcon").unwrap_or_default(),
            use_standard_icon: inst.get_bool("useStandardIcon").unwrap_or_default(),
            use_aricon: inst.get_bool("useARIcon").unwrap_or_default(),
            use_model: inst.get_bool("useModel").unwrap_or_default(),
            small_icon_index: inst.get_i32("smallIconIndex").unwrap_or_default(),
            standard_icon_index: inst.get_i32("standardIconIndex").unwrap_or_default(),
            small_icon: inst.get_str("smallIcon").map(String::from).unwrap_or_default(),
            standard_icon: inst.get_str("standardIcon").map(String::from).unwrap_or_default(),
            map_label_display_type: inst.get_str("mapLabelDisplayType").map(String::from).unwrap_or_default(),
            stack_position_alignment: inst.get_str("stackPositionAlignment").map(String::from).unwrap_or_default(),
            layout_canvas: inst.get("layoutCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            abilities: inst.get_array("abilities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Marker_AbilityBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Marker_AbilityBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            show_rules: inst.get_array("showRules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Marker_ShowRule>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Marker_ShowRule>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            map_show_rules: inst.get_array("mapShowRules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Marker_ShowRuleMapDisplayMode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Marker_ShowRuleMapDisplayMode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            map_boxout_section_types: inst.get_array("mapBoxoutSectionTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            marker_offset: match inst.get("markerOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `Marker_ShowRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_ShowRule {
}

impl Pooled for Marker_ShowRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_show_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_show_rule }
}

impl<'a> Extract<'a> for Marker_ShowRule {
    const TYPE_NAME: &'static str = "Marker_ShowRule";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `Marker_ShowRuleMapDisplayMode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_ShowRuleMapDisplayMode {
    /// DCB field: `displayModeMap` (EnumChoice)
    #[serde(default)]
    pub display_mode_map: String,
}

impl Pooled for Marker_ShowRuleMapDisplayMode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_show_rule_map_display_mode }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_show_rule_map_display_mode }
}

impl<'a> Extract<'a> for Marker_ShowRuleMapDisplayMode {
    const TYPE_NAME: &'static str = "Marker_ShowRuleMapDisplayMode";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_mode_map: inst.get_str("displayModeMap").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `Marker_AbilityBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker_AbilityBase {
}

impl Pooled for Marker_AbilityBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_ability_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_ability_base }
}

impl<'a> Extract<'a> for Marker_AbilityBase {
    const TYPE_NAME: &'static str = "Marker_AbilityBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MasterModeSwitchDeltaSignatureTypes`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterModeSwitchDeltaSignatureTypes {
    /// DCB field: `scmToNav` (Reference)
    #[serde(default)]
    pub scm_to_nav: Option<CigGuid>,
    /// DCB field: `navToScm` (Reference)
    #[serde(default)]
    pub nav_to_scm: Option<CigGuid>,
}

impl Pooled for MasterModeSwitchDeltaSignatureTypes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.master_mode_switch_delta_signature_types }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.master_mode_switch_delta_signature_types }
}

impl<'a> Extract<'a> for MasterModeSwitchDeltaSignatureTypes {
    const TYPE_NAME: &'static str = "MasterModeSwitchDeltaSignatureTypes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            scm_to_nav: inst.get("scmToNav").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            nav_to_scm: inst.get("navToScm").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `MarkerAR_ConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerAR_ConfigDef {
    /// DCB field: `maxVisibleDistance` (Single)
    #[serde(default)]
    pub max_visible_distance: f32,
    /// DCB field: `minFocusAngle` (Single)
    #[serde(default)]
    pub min_focus_angle: f32,
    /// DCB field: `clipScreenMin` (Class)
    #[serde(default)]
    pub clip_screen_min: Option<Handle<Vec2>>,
    /// DCB field: `clipScreenMax` (Class)
    #[serde(default)]
    pub clip_screen_max: Option<Handle<Vec2>>,
    /// DCB field: `clipRadius` (Single)
    #[serde(default)]
    pub clip_radius: f32,
    /// DCB field: `clipAspect` (Single)
    #[serde(default)]
    pub clip_aspect: f32,
}

impl Pooled for MarkerAR_ConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ma.marker_ar_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ma.marker_ar_config_def }
}

impl<'a> Extract<'a> for MarkerAR_ConfigDef {
    const TYPE_NAME: &'static str = "MarkerAR_ConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_visible_distance: inst.get_f32("maxVisibleDistance").unwrap_or_default(),
            min_focus_angle: inst.get_f32("minFocusAngle").unwrap_or_default(),
            clip_screen_min: match inst.get("clipScreenMin") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            clip_screen_max: match inst.get("clipScreenMax") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            clip_radius: inst.get_f32("clipRadius").unwrap_or_default(),
            clip_aspect: inst.get_f32("clipAspect").unwrap_or_default(),
        }
    }
}

