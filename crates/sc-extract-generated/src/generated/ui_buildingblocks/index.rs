// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `ui-buildingblocks` feature.
#[derive(Default)]
pub struct UiBuildingblocksIndex {
    pub grab_camera_control_params: HashMap<CigGuid, Handle<GrabCameraControlParams>>,
    pub building_blocks_language_specific_font_replacement: HashMap<CigGuid, Handle<BuildingBlocks_LanguageSpecificFontReplacement>>,
    pub docking_sensitivity: HashMap<CigGuid, Handle<DockingSensitivity>>,
    pub status_widget_display_preset: HashMap<CigGuid, Handle<StatusWidgetDisplayPreset>>,
    pub visor_lens_layout: HashMap<CigGuid, Handle<VisorLens_Layout>>,
    pub visor_lens_region: HashMap<CigGuid, Handle<VisorLens_Region>>,
}

impl UiBuildingblocksIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.grab_camera_control_params.len();
        total += self.building_blocks_language_specific_font_replacement.len();
        total += self.docking_sensitivity.len();
        total += self.status_widget_display_preset.len();
        total += self.visor_lens_layout.len();
        total += self.visor_lens_region.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
