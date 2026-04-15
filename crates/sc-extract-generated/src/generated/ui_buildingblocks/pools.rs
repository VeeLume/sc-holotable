// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `ui-buildingblocks` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiBuildingblocksPools {
    #[serde(default)]
    pub bindings_operations_waveform_shape_triangle: Vec<Option<BindingsOperations_WaveformShapeTriangle>>,
    #[serde(default)]
    pub bindings_operations_waveform_shape_square: Vec<Option<BindingsOperations_WaveformShapeSquare>>,
    #[serde(default)]
    pub bindings_operations_waveform_shape_sawtooth: Vec<Option<BindingsOperations_WaveformShapeSawtooth>>,
    #[serde(default)]
    pub building_blocks_bindings_color_from_number_rgba: Vec<Option<BuildingBlocks_BindingsColorFromNumberRGBA>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_max_height_behavior: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeMaxHeightBehavior>>,
    #[serde(default)]
    pub building_blocks_field_modifier_enumerated_type_stroke_alignment: Vec<Option<BuildingBlocks_FieldModifierEnumeratedTypeStrokeAlignment>>,
    #[serde(default)]
    pub building_blocks_timing_function_custom_curve: Vec<Option<BuildingBlocks_TimingFunctionCustomCurve>>,
    #[serde(default)]
    pub building_blocks_integer_widget_pair_def: Vec<Option<BuildingBlocks_IntegerWidgetPairDef>>,
    #[serde(default)]
    pub building_blocks_widget_polymorphic: Vec<Option<BuildingBlocks_WidgetPolymorphic>>,
    #[serde(default)]
    pub building_blocks_widget_particle_effect: Vec<Option<BuildingBlocks_WidgetParticleEffect>>,
    #[serde(default)]
    pub building_blocks_look_at_transformer: Vec<Option<BuildingBlocks_LookAtTransformer>>,
    #[serde(default)]
    pub building_blocks_widget_environment_probe: Vec<Option<BuildingBlocks_WidgetEnvironmentProbe>>,
    #[serde(default)]
    pub grab_camera_control_params: Vec<Option<GrabCameraControlParams>>,
    #[serde(default)]
    pub building_blocks_target_slicer: Vec<Option<BuildingBlocks_TargetSlicer>>,
    #[serde(default)]
    pub building_blocks_widget_pagination: Vec<Option<BuildingBlocks_WidgetPagination>>,
    #[serde(default)]
    pub building_blocks_widget_radio_control: Vec<Option<BuildingBlocks_WidgetRadioControl>>,
    #[serde(default)]
    pub building_blocks_widget_badge: Vec<Option<BuildingBlocks_WidgetBadge>>,
    #[serde(default)]
    pub building_blocks_component_radio_button: Vec<Option<BuildingBlocks_ComponentRadioButton>>,
    #[serde(default)]
    pub building_blocks_component_toggle_list_item: Vec<Option<BuildingBlocks_ComponentToggleListItem>>,
    #[serde(default)]
    pub building_blocks_component_radial_range_slider: Vec<Option<BuildingBlocks_ComponentRadialRangeSlider>>,
    #[serde(default)]
    pub building_blocks_component_notification: Vec<Option<BuildingBlocks_ComponentNotification>>,
    #[serde(default)]
    pub building_blocks_font_replacement_pair: Vec<Option<BuildingBlocks_FontReplacementPair>>,
    #[serde(default)]
    pub building_blocks_language_specific_font_replacement: Vec<Option<BuildingBlocks_LanguageSpecificFontReplacement>>,
    #[serde(default)]
    pub building_blocks_trigger_navigation: Vec<Option<BuildingBlocks_TriggerNavigation>>,
    #[serde(default)]
    pub building_blocks_trigger_subsumption_broadcast: Vec<Option<BuildingBlocks_TriggerSubsumptionBroadcast>>,
    #[serde(default)]
    pub building_blocks_shape_circle: Vec<Option<BuildingBlocks_ShapeCircle>>,
    #[serde(default)]
    pub docking_sensitivity: Vec<Option<DockingSensitivity>>,
    #[serde(default)]
    pub display_state: Vec<Option<DisplayState>>,
    #[serde(default)]
    pub status_widget_display_preset: Vec<Option<StatusWidgetDisplayPreset>>,
    #[serde(default)]
    pub visor_lens_layout: Vec<Option<VisorLens_Layout>>,
    #[serde(default)]
    pub visor_lens_region: Vec<Option<VisorLens_Region>>,
    #[serde(default)]
    pub visor_lens_widget: Vec<Option<VisorLens_Widget>>,
}
