// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `ui-buildingblocks` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiBuildingblocksPools {
    #[serde(default)]
    pub building_blocks_timeline_type_embedded: Vec<Option<BuildingBlocks_TimelineTypeEmbedded>>,
    #[serde(default)]
    pub building_blocks_timeline: Vec<Option<BuildingBlocks_Timeline>>,
    #[serde(default)]
    pub building_blocks_keyframe: Vec<Option<BuildingBlocks_Keyframe>>,
    #[serde(default)]
    pub building_blocks_keyframe_modifier_data: Vec<Option<BuildingBlocks_KeyframeModifierData>>,
    #[serde(default)]
    pub grab_camera_control_params: Vec<Option<GrabCameraControlParams>>,
    #[serde(default)]
    pub building_blocks_font_style: Vec<Option<BuildingBlocks_FontStyle>>,
    #[serde(default)]
    pub building_blocks_font_replacement_pair: Vec<Option<BuildingBlocks_FontReplacementPair>>,
    #[serde(default)]
    pub building_blocks_language_specific_font_replacement: Vec<Option<BuildingBlocks_LanguageSpecificFontReplacement>>,
    #[serde(default)]
    pub building_blocks_text_format_modifier_base: Vec<Option<BuildingBlocks_TextFormatModifierBase>>,
    #[serde(default)]
    pub building_blocks_text_emphasis_modifier_list: Vec<Option<BuildingBlocks_TextEmphasisModifierList>>,
    #[serde(default)]
    pub building_blocks_style: Vec<Option<BuildingBlocks_Style>>,
    #[serde(default)]
    pub geom_font_letter_node: Vec<Option<GeomFont_LetterNode>>,
    #[serde(default)]
    pub geom_font_config: Vec<Option<GeomFont_Config>>,
    #[serde(default)]
    pub docking_sensitivity: Vec<Option<DockingSensitivity>>,
    #[serde(default)]
    pub display_state: Vec<Option<DisplayState>>,
    #[serde(default)]
    pub status_widget_display_preset: Vec<Option<StatusWidgetDisplayPreset>>,
    #[serde(default)]
    pub scitem_uiview_dashboard_canvas_view_def: Vec<Option<SCItemUIView_DashboardCanvasViewDef>>,
    #[serde(default)]
    pub scitem_uiview_dashboard_canvas_def: Vec<Option<SCItemUIView_DashboardCanvasDef>>,
    #[serde(default)]
    pub visor_lens_layout: Vec<Option<VisorLens_Layout>>,
    #[serde(default)]
    pub visor_lens_region: Vec<Option<VisorLens_Region>>,
    #[serde(default)]
    pub visor_lens_widget: Vec<Option<VisorLens_Widget>>,
}
