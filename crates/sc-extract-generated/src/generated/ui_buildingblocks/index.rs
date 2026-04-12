// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `ui-buildingblocks` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiBuildingblocksIndex {
    #[serde(default)]
    pub building_blocks_timeline: HashMap<CigGuid, Handle<BuildingBlocks_Timeline>>,
    #[serde(default)]
    pub grab_camera_control_params: HashMap<CigGuid, Handle<GrabCameraControlParams>>,
    #[serde(default)]
    pub building_blocks_font_style: HashMap<CigGuid, Handle<BuildingBlocks_FontStyle>>,
    #[serde(default)]
    pub building_blocks_language_specific_font_replacement: HashMap<CigGuid, Handle<BuildingBlocks_LanguageSpecificFontReplacement>>,
    #[serde(default)]
    pub building_blocks_style: HashMap<CigGuid, Handle<BuildingBlocks_Style>>,
    #[serde(default)]
    pub geom_font_config: HashMap<CigGuid, Handle<GeomFont_Config>>,
    #[serde(default)]
    pub docking_sensitivity: HashMap<CigGuid, Handle<DockingSensitivity>>,
    #[serde(default)]
    pub status_widget_display_preset: HashMap<CigGuid, Handle<StatusWidgetDisplayPreset>>,
    #[serde(default)]
    pub scitem_uiview_dashboard_canvas_def: HashMap<CigGuid, Handle<SCItemUIView_DashboardCanvasDef>>,
    #[serde(default)]
    pub visor_lens_layout: HashMap<CigGuid, Handle<VisorLens_Layout>>,
    #[serde(default)]
    pub visor_lens_region: HashMap<CigGuid, Handle<VisorLens_Region>>,
}

impl UiBuildingblocksIndex {
    pub fn len(&self) -> usize {
        self.building_blocks_timeline.len()
            + self.grab_camera_control_params.len()
            + self.building_blocks_font_style.len()
            + self.building_blocks_language_specific_font_replacement.len()
            + self.building_blocks_style.len()
            + self.geom_font_config.len()
            + self.docking_sensitivity.len()
            + self.status_widget_display_preset.len()
            + self.scitem_uiview_dashboard_canvas_def.len()
            + self.visor_lens_layout.len()
            + self.visor_lens_region.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
