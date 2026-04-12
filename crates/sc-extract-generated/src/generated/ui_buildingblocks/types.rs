// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-buildingblocks`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `BuildingBlocks_TimelineTypeEmbedded`
/// Inherits from: `BuildingBlocks_TimelineTypeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TimelineTypeEmbedded {
    /// `keyframes` (Class (array))
    #[serde(default)]
    pub keyframes: Vec<Handle<BuildingBlocks_Keyframe>>,
}

impl Pooled for BuildingBlocks_TimelineTypeEmbedded {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_timeline_type_embedded }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_timeline_type_embedded }
}

impl<'a> Extract<'a> for BuildingBlocks_TimelineTypeEmbedded {
    const TYPE_NAME: &'static str = "BuildingBlocks_TimelineTypeEmbedded";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            keyframes: inst.get_array("keyframes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_Keyframe>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_Keyframe>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Timeline`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Timeline {
    /// `timeline` (Class)
    #[serde(default)]
    pub timeline: Option<Handle<BuildingBlocks_TimelineTypeEmbedded>>,
}

impl Pooled for BuildingBlocks_Timeline {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_timeline }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_timeline }
}

impl<'a> Extract<'a> for BuildingBlocks_Timeline {
    const TYPE_NAME: &'static str = "BuildingBlocks_Timeline";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            timeline: match inst.get("timeline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TimelineTypeEmbedded>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TimelineTypeEmbedded>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BuildingBlocks_Keyframe`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Keyframe {
    /// `percent` (Single)
    #[serde(default)]
    pub percent: f32,
    /// `modifiers` (Class (array))
    #[serde(default)]
    pub modifiers: Vec<Handle<BuildingBlocks_KeyframeModifierData>>,
}

impl Pooled for BuildingBlocks_Keyframe {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_keyframe }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_keyframe }
}

impl<'a> Extract<'a> for BuildingBlocks_Keyframe {
    const TYPE_NAME: &'static str = "BuildingBlocks_Keyframe";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            percent: inst.get_f32("percent").unwrap_or_default(),
            modifiers: inst.get_array("modifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_KeyframeModifierData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_KeyframeModifierData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_KeyframeModifierData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_KeyframeModifierData {
    /// `timingFunction` (StrongPointer)
    #[serde(default)]
    pub timing_function: Option<Handle<BuildingBlocks_TimingFunctionBase>>,
    /// `modifier` (StrongPointer)
    #[serde(default)]
    pub modifier: Option<Handle<BuildingBlocks_FieldModifierBase>>,
}

impl Pooled for BuildingBlocks_KeyframeModifierData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_keyframe_modifier_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_keyframe_modifier_data }
}

impl<'a> Extract<'a> for BuildingBlocks_KeyframeModifierData {
    const TYPE_NAME: &'static str = "BuildingBlocks_KeyframeModifierData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            timing_function: match inst.get("timingFunction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_TimingFunctionBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_TimingFunctionBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifier: match inst.get("modifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_FieldModifierBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_FieldModifierBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GrabCameraControlParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrabCameraControlParams {
    /// `responsiveness` (Single)
    #[serde(default)]
    pub responsiveness: f32,
    /// `rotationSpeed` (Single)
    #[serde(default)]
    pub rotation_speed: f32,
    /// `rotationSlowdown` (Single)
    #[serde(default)]
    pub rotation_slowdown: f32,
    /// `zoomSpeed` (Single)
    #[serde(default)]
    pub zoom_speed: f32,
    /// `zoomSlowdown` (Single)
    #[serde(default)]
    pub zoom_slowdown: f32,
    /// `minimumZoomDistance` (Single)
    #[serde(default)]
    pub minimum_zoom_distance: f32,
    /// `maximumZoomDistance` (Single)
    #[serde(default)]
    pub maximum_zoom_distance: f32,
    /// `maximumZoomSpeed` (Single)
    #[serde(default)]
    pub maximum_zoom_speed: f32,
    /// `isGrabbableOutOfBounds` (Boolean)
    #[serde(default)]
    pub is_grabbable_out_of_bounds: bool,
    /// `grabRotationMode` (EnumChoice)
    #[serde(default)]
    pub grab_rotation_mode: String,
    /// `panResponsiveness` (Single)
    #[serde(default)]
    pub pan_responsiveness: f32,
    /// `panSpeed` (Single)
    #[serde(default)]
    pub pan_speed: f32,
    /// `panSlowdown` (Single)
    #[serde(default)]
    pub pan_slowdown: f32,
}

impl Pooled for GrabCameraControlParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.grab_camera_control_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.grab_camera_control_params }
}

impl<'a> Extract<'a> for GrabCameraControlParams {
    const TYPE_NAME: &'static str = "GrabCameraControlParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            responsiveness: inst.get_f32("responsiveness").unwrap_or_default(),
            rotation_speed: inst.get_f32("rotationSpeed").unwrap_or_default(),
            rotation_slowdown: inst.get_f32("rotationSlowdown").unwrap_or_default(),
            zoom_speed: inst.get_f32("zoomSpeed").unwrap_or_default(),
            zoom_slowdown: inst.get_f32("zoomSlowdown").unwrap_or_default(),
            minimum_zoom_distance: inst.get_f32("minimumZoomDistance").unwrap_or_default(),
            maximum_zoom_distance: inst.get_f32("maximumZoomDistance").unwrap_or_default(),
            maximum_zoom_speed: inst.get_f32("maximumZoomSpeed").unwrap_or_default(),
            is_grabbable_out_of_bounds: inst.get_bool("isGrabbableOutOfBounds").unwrap_or_default(),
            grab_rotation_mode: inst.get_str("grabRotationMode").map(String::from).unwrap_or_default(),
            pan_responsiveness: inst.get_f32("panResponsiveness").unwrap_or_default(),
            pan_speed: inst.get_f32("panSpeed").unwrap_or_default(),
            pan_slowdown: inst.get_f32("panSlowdown").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_FontStyle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FontStyle {
    /// `font` (String)
    #[serde(default)]
    pub font: String,
    /// `paintFile` (String)
    #[serde(default)]
    pub paint_file: String,
    /// `isBold` (Boolean)
    #[serde(default)]
    pub is_bold: bool,
    /// `imageSizePercent` (Single)
    #[serde(default)]
    pub image_size_percent: f32,
    /// `scaleModifier` (Single)
    #[serde(default)]
    pub scale_modifier: f32,
    /// `leadingModifier` (Single)
    #[serde(default)]
    pub leading_modifier: f32,
    /// `topMarginModifier` (Single)
    #[serde(default)]
    pub top_margin_modifier: f32,
    /// `leftMarginModifier` (Single)
    #[serde(default)]
    pub left_margin_modifier: f32,
    /// `numImageReplacementSpaces` (Int32)
    #[serde(default)]
    pub num_image_replacement_spaces: i32,
}

impl Pooled for BuildingBlocks_FontStyle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_font_style }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_font_style }
}

impl<'a> Extract<'a> for BuildingBlocks_FontStyle {
    const TYPE_NAME: &'static str = "BuildingBlocks_FontStyle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            font: inst.get_str("font").map(String::from).unwrap_or_default(),
            paint_file: inst.get_str("paintFile").map(String::from).unwrap_or_default(),
            is_bold: inst.get_bool("isBold").unwrap_or_default(),
            image_size_percent: inst.get_f32("imageSizePercent").unwrap_or_default(),
            scale_modifier: inst.get_f32("scaleModifier").unwrap_or_default(),
            leading_modifier: inst.get_f32("leadingModifier").unwrap_or_default(),
            top_margin_modifier: inst.get_f32("topMarginModifier").unwrap_or_default(),
            left_margin_modifier: inst.get_f32("leftMarginModifier").unwrap_or_default(),
            num_image_replacement_spaces: inst.get_i32("numImageReplacementSpaces").unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_FontReplacementPair`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_FontReplacementPair {
    /// `englishFont` (Reference)
    #[serde(default)]
    pub english_font: Option<CigGuid>,
    /// `replacementFontName` (String)
    #[serde(default)]
    pub replacement_font_name: String,
    /// `replacementFontPaintFile` (String)
    #[serde(default)]
    pub replacement_font_paint_file: String,
}

impl Pooled for BuildingBlocks_FontReplacementPair {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_font_replacement_pair }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_font_replacement_pair }
}

impl<'a> Extract<'a> for BuildingBlocks_FontReplacementPair {
    const TYPE_NAME: &'static str = "BuildingBlocks_FontReplacementPair";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            english_font: inst.get("englishFont").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            replacement_font_name: inst.get_str("replacementFontName").map(String::from).unwrap_or_default(),
            replacement_font_paint_file: inst.get_str("replacementFontPaintFile").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_LanguageSpecificFontReplacement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_LanguageSpecificFontReplacement {
    /// `fontReplacementList` (Class (array))
    #[serde(default)]
    pub font_replacement_list: Vec<Handle<BuildingBlocks_FontReplacementPair>>,
}

impl Pooled for BuildingBlocks_LanguageSpecificFontReplacement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_language_specific_font_replacement }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_language_specific_font_replacement }
}

impl<'a> Extract<'a> for BuildingBlocks_LanguageSpecificFontReplacement {
    const TYPE_NAME: &'static str = "BuildingBlocks_LanguageSpecificFontReplacement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            font_replacement_list: inst.get_array("fontReplacementList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_FontReplacementPair>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_FontReplacementPair>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_TextFormatModifierBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TextFormatModifierBase {
}

impl Pooled for BuildingBlocks_TextFormatModifierBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_text_format_modifier_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_text_format_modifier_base }
}

impl<'a> Extract<'a> for BuildingBlocks_TextFormatModifierBase {
    const TYPE_NAME: &'static str = "BuildingBlocks_TextFormatModifierBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BuildingBlocks_TextEmphasisModifierList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_TextEmphasisModifierList {
    /// `textEmphasis` (StrongPointer (array))
    #[serde(default)]
    pub text_emphasis: Vec<Handle<BuildingBlocks_TextFormatModifierBase>>,
}

impl Pooled for BuildingBlocks_TextEmphasisModifierList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_text_emphasis_modifier_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_text_emphasis_modifier_list }
}

impl<'a> Extract<'a> for BuildingBlocks_TextEmphasisModifierList {
    const TYPE_NAME: &'static str = "BuildingBlocks_TextEmphasisModifierList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            text_emphasis: inst.get_array("textEmphasis")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TextFormatModifierBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TextFormatModifierBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `BuildingBlocks_Style`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBlocks_Style {
    /// `entries` (Class (array))
    #[serde(default)]
    pub entries: Vec<Handle<BuildingBlocks_StyleEntry>>,
    /// `colorStyles` (StrongPointer)
    #[serde(default)]
    pub color_styles: Option<Handle<BuildingBlocks_ColorBase>>,
    /// `textFieldModifiers` (Class (array))
    #[serde(default)]
    pub text_field_modifiers: Vec<Handle<BuildingBlocks_TextEmphasisModifierList>>,
}

impl Pooled for BuildingBlocks_Style {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.building_blocks_style }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.building_blocks_style }
}

impl<'a> Extract<'a> for BuildingBlocks_Style {
    const TYPE_NAME: &'static str = "BuildingBlocks_Style";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: inst.get_array("entries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_StyleEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            color_styles: match inst.get("colorStyles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BuildingBlocks_ColorBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            text_field_modifiers: inst.get_array("textFieldModifiers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BuildingBlocks_TextEmphasisModifierList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BuildingBlocks_TextEmphasisModifierList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GeomFont_LetterNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomFont_LetterNode {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `code` (UInt16)
    #[serde(default)]
    pub code: u32,
}

impl Pooled for GeomFont_LetterNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.geom_font_letter_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.geom_font_letter_node }
}

impl<'a> Extract<'a> for GeomFont_LetterNode {
    const TYPE_NAME: &'static str = "GeomFont_LetterNode";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            code: inst.get_u32("code").unwrap_or_default(),
        }
    }
}

/// DCB type: `GeomFont_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomFont_Config {
    /// `geometryFile` (String)
    #[serde(default)]
    pub geometry_file: String,
    /// `metricsFile` (String)
    #[serde(default)]
    pub metrics_file: String,
    /// `rootNode` (String)
    #[serde(default)]
    pub root_node: String,
    /// `letterNodes` (Class (array))
    #[serde(default)]
    pub letter_nodes: Vec<Handle<GeomFont_LetterNode>>,
}

impl Pooled for GeomFont_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.geom_font_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.geom_font_config }
}

impl<'a> Extract<'a> for GeomFont_Config {
    const TYPE_NAME: &'static str = "GeomFont_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            geometry_file: inst.get_str("geometryFile").map(String::from).unwrap_or_default(),
            metrics_file: inst.get_str("metricsFile").map(String::from).unwrap_or_default(),
            root_node: inst.get_str("rootNode").map(String::from).unwrap_or_default(),
            letter_nodes: inst.get_array("letterNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<GeomFont_LetterNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<GeomFont_LetterNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DockingSensitivity`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockingSensitivity {
    /// `DockingUIRotationalSensitivity` (Single)
    #[serde(default)]
    pub docking_uirotational_sensitivity: f32,
    /// `DockingUILinearSensitivity` (Single)
    #[serde(default)]
    pub docking_uilinear_sensitivity: f32,
}

impl Pooled for DockingSensitivity {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.docking_sensitivity }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.docking_sensitivity }
}

impl<'a> Extract<'a> for DockingSensitivity {
    const TYPE_NAME: &'static str = "DockingSensitivity";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            docking_uirotational_sensitivity: inst.get_f32("DockingUIRotationalSensitivity").unwrap_or_default(),
            docking_uilinear_sensitivity: inst.get_f32("DockingUILinearSensitivity").unwrap_or_default(),
        }
    }
}

/// DCB type: `DisplayState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayState {
    /// `minimumValue` (Single)
    #[serde(default)]
    pub minimum_value: f32,
    /// `maximumValue` (Single)
    #[serde(default)]
    pub maximum_value: f32,
    /// `displayDuration` (Single)
    #[serde(default)]
    pub display_duration: f32,
    /// `activeRange` (EnumChoice)
    #[serde(default)]
    pub active_range: String,
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
}

impl Pooled for DisplayState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.display_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.display_state }
}

impl<'a> Extract<'a> for DisplayState {
    const TYPE_NAME: &'static str = "DisplayState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            minimum_value: inst.get_f32("minimumValue").unwrap_or_default(),
            maximum_value: inst.get_f32("maximumValue").unwrap_or_default(),
            display_duration: inst.get_f32("displayDuration").unwrap_or_default(),
            active_range: inst.get_str("activeRange").map(String::from).unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
        }
    }
}

/// DCB type: `StatusWidgetDisplayPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusWidgetDisplayPreset {
    /// `ranges` (Class)
    #[serde(default)]
    pub ranges: Option<Handle<DisplayState>>,
    /// `incrementDisplayDuration` (Single)
    #[serde(default)]
    pub increment_display_duration: f32,
    /// `incrementStep` (Single)
    #[serde(default)]
    pub increment_step: f32,
    /// `maximumChangePerSecond` (Single)
    #[serde(default)]
    pub maximum_change_per_second: f32,
    /// `historySeconds` (Int32)
    #[serde(default)]
    pub history_seconds: i32,
    /// `historySamplesPerSecond` (Int32)
    #[serde(default)]
    pub history_samples_per_second: i32,
    /// `shownOnLens` (Boolean)
    #[serde(default)]
    pub shown_on_lens: bool,
    /// `shownOnVisor` (Boolean)
    #[serde(default)]
    pub shown_on_visor: bool,
}

impl Pooled for StatusWidgetDisplayPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.status_widget_display_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.status_widget_display_preset }
}

impl<'a> Extract<'a> for StatusWidgetDisplayPreset {
    const TYPE_NAME: &'static str = "StatusWidgetDisplayPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ranges: match inst.get("ranges") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DisplayState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DisplayState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            increment_display_duration: inst.get_f32("incrementDisplayDuration").unwrap_or_default(),
            increment_step: inst.get_f32("incrementStep").unwrap_or_default(),
            maximum_change_per_second: inst.get_f32("maximumChangePerSecond").unwrap_or_default(),
            history_seconds: inst.get_i32("historySeconds").unwrap_or_default(),
            history_samples_per_second: inst.get_i32("historySamplesPerSecond").unwrap_or_default(),
            shown_on_lens: inst.get_bool("shownOnLens").unwrap_or_default(),
            shown_on_visor: inst.get_bool("shownOnVisor").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemUIView_DashboardCanvasViewDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemUIView_DashboardCanvasViewDef {
    /// `screens` (Reference)
    #[serde(default)]
    pub screens: Option<CigGuid>,
}

impl Pooled for SCItemUIView_DashboardCanvasViewDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.scitem_uiview_dashboard_canvas_view_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.scitem_uiview_dashboard_canvas_view_def }
}

impl<'a> Extract<'a> for SCItemUIView_DashboardCanvasViewDef {
    const TYPE_NAME: &'static str = "SCItemUIView_DashboardCanvasViewDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            screens: inst.get("screens").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SCItemUIView_DashboardCanvasDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemUIView_DashboardCanvasDef {
    /// `views` (Class)
    #[serde(default)]
    pub views: Option<Handle<SCItemUIView_DashboardCanvasViewDef>>,
}

impl Pooled for SCItemUIView_DashboardCanvasDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.scitem_uiview_dashboard_canvas_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.scitem_uiview_dashboard_canvas_def }
}

impl<'a> Extract<'a> for SCItemUIView_DashboardCanvasDef {
    const TYPE_NAME: &'static str = "SCItemUIView_DashboardCanvasDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            views: match inst.get("views") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SCItemUIView_DashboardCanvasViewDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCItemUIView_DashboardCanvasViewDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `VisorLens_Layout`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisorLens_Layout {
    /// `regions` (Reference (array))
    #[serde(default)]
    pub regions: Vec<CigGuid>,
}

impl Pooled for VisorLens_Layout {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.visor_lens_layout }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.visor_lens_layout }
}

impl<'a> Extract<'a> for VisorLens_Layout {
    const TYPE_NAME: &'static str = "VisorLens_Layout";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            regions: inst.get_array("regions")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VisorLens_Region`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisorLens_Region {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `orientation` (Class)
    #[serde(default)]
    pub orientation: Option<Handle<Deg3>>,
    /// `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
    /// `anchor` (Class)
    #[serde(default)]
    pub anchor: Option<Handle<Vec3>>,
    /// `pivot` (Class)
    #[serde(default)]
    pub pivot: Option<Handle<Vec3>>,
    /// `flexDirection` (EnumChoice)
    #[serde(default)]
    pub flex_direction: String,
    /// `flexAxisJustification` (EnumChoice)
    #[serde(default)]
    pub flex_axis_justification: String,
    /// `flexCrossAxisJustification` (EnumChoice)
    #[serde(default)]
    pub flex_cross_axis_justification: String,
    /// `flexItemAlignment` (EnumChoice)
    #[serde(default)]
    pub flex_item_alignment: String,
    /// `widgets` (Class (array))
    #[serde(default)]
    pub widgets: Vec<Handle<VisorLens_Widget>>,
}

impl Pooled for VisorLens_Region {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.visor_lens_region }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.visor_lens_region }
}

impl<'a> Extract<'a> for VisorLens_Region {
    const TYPE_NAME: &'static str = "VisorLens_Region";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Deg3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Deg3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            anchor: match inst.get("anchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pivot: match inst.get("pivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            flex_direction: inst.get_str("flexDirection").map(String::from).unwrap_or_default(),
            flex_axis_justification: inst.get_str("flexAxisJustification").map(String::from).unwrap_or_default(),
            flex_cross_axis_justification: inst.get_str("flexCrossAxisJustification").map(String::from).unwrap_or_default(),
            flex_item_alignment: inst.get_str("flexItemAlignment").map(String::from).unwrap_or_default(),
            widgets: inst.get_array("widgets")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<VisorLens_Widget>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<VisorLens_Widget>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VisorLens_Widget`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisorLens_Widget {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `namespace` (String)
    #[serde(default)]
    pub namespace: String,
    /// `canvas` (Reference)
    #[serde(default)]
    pub canvas: Option<CigGuid>,
    /// `size` (Class)
    #[serde(default)]
    pub size: Option<Handle<Vec3>>,
    /// `orientation` (Class)
    #[serde(default)]
    pub orientation: Option<Handle<Ang3>>,
    /// `slot` (Int32)
    #[serde(default)]
    pub slot: i32,
    /// `showTags` (Reference (array))
    #[serde(default)]
    pub show_tags: Vec<CigGuid>,
    /// `hideTags` (Reference (array))
    #[serde(default)]
    pub hide_tags: Vec<CigGuid>,
}

impl Pooled for VisorLens_Widget {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_buildingblocks.visor_lens_widget }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_buildingblocks.visor_lens_widget }
}

impl<'a> Extract<'a> for VisorLens_Widget {
    const TYPE_NAME: &'static str = "VisorLens_Widget";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            namespace: inst.get_str("namespace").map(String::from).unwrap_or_default(),
            canvas: inst.get("canvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            size: match inst.get("size") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            slot: inst.get_i32("slot").unwrap_or_default(),
            show_tags: inst.get_array("showTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            hide_tags: inst.get_array("hideTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

