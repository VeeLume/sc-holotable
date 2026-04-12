// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-radar3dpresets`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `UIDataBankDisplay3DSpaceDustParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIDataBankDisplay3DSpaceDustParams {
    /// `sizeMultiplier` (Single)
    #[serde(default)]
    pub size_multiplier: f32,
    /// `minimumSize` (Single)
    #[serde(default)]
    pub minimum_size: f32,
    /// `maximumSize` (Single)
    #[serde(default)]
    pub maximum_size: f32,
}

impl Pooled for UIDataBankDisplay3DSpaceDustParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radar3dpresets.uidata_bank_display3_dspace_dust_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radar3dpresets.uidata_bank_display3_dspace_dust_params }
}

impl<'a> Extract<'a> for UIDataBankDisplay3DSpaceDustParams {
    const TYPE_NAME: &'static str = "UIDataBankDisplay3DSpaceDustParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            size_multiplier: inst.get_f32("sizeMultiplier").unwrap_or_default(),
            minimum_size: inst.get_f32("minimumSize").unwrap_or_default(),
            maximum_size: inst.get_f32("maximumSize").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIDataBankDisplay3DInterpolateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIDataBankDisplay3DInterpolateParams {
    /// `openDuration` (Single)
    #[serde(default)]
    pub open_duration: f32,
    /// `openCurve` (Class)
    #[serde(default)]
    pub open_curve: Option<Handle<BezierCurve>>,
    /// `staggeredDelay` (Single)
    #[serde(default)]
    pub staggered_delay: f32,
}

impl Pooled for UIDataBankDisplay3DInterpolateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radar3dpresets.uidata_bank_display3_dinterpolate_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radar3dpresets.uidata_bank_display3_dinterpolate_params }
}

impl<'a> Extract<'a> for UIDataBankDisplay3DInterpolateParams {
    const TYPE_NAME: &'static str = "UIDataBankDisplay3DInterpolateParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            open_duration: inst.get_f32("openDuration").unwrap_or_default(),
            open_curve: match inst.get("openCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            staggered_delay: inst.get_f32("staggeredDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIDataBankDisplay3DParams`
/// Inherits from: `UIWorldDisplay3DParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIDataBankDisplay3DParams {
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
    /// `renderRadius` (Double)
    #[serde(default)]
    pub render_radius: f64,
    /// `renderNodeMaterial` (Class)
    #[serde(default)]
    pub render_node_material: Option<Handle<GlobalResourceMaterial>>,
    /// `inputParams` (Class)
    #[serde(default)]
    pub input_params: Option<Handle<UI3DDisplayInputParams>>,
    /// `holographicSettings` (StrongPointer)
    #[serde(default)]
    pub holographic_settings: Option<Handle<UIWorldDisplayHolographicSettings>>,
    /// `rotationModeSettings` (Class)
    #[serde(default)]
    pub rotation_mode_settings: Option<Handle<UIWorldDisplayRotationModeParams>>,
    /// `centerToSelf` (Boolean)
    #[serde(default)]
    pub center_to_self: bool,
    /// `focusChangeDuration` (Single)
    #[serde(default)]
    pub focus_change_duration: f32,
    /// `extraZoomScale` (Single)
    #[serde(default)]
    pub extra_zoom_scale: f32,
    /// `childlessExtraZoomScale` (Single)
    #[serde(default)]
    pub childless_extra_zoom_scale: f32,
    /// `youAreHereZoomDiameter` (Single)
    #[serde(default)]
    pub you_are_here_zoom_diameter: f32,
    /// `maxZoomScalingPerUpdate` (Single)
    #[serde(default)]
    pub max_zoom_scaling_per_update: f32,
    /// `autoRotationSettings` (StrongPointer)
    #[serde(default)]
    pub auto_rotation_settings: Option<Handle<UIWorldDisplayAutoRotationParams>>,
    /// `soundSettings` (StrongPointer)
    #[serde(default)]
    pub sound_settings: Option<Handle<UIWorldDisplaySoundParams>>,
    /// `loadoutDummyCameraOffset` (Single)
    #[serde(default)]
    pub loadout_dummy_camera_offset: f32,
    /// `showSpaceDust` (Boolean)
    #[serde(default)]
    pub show_space_dust: bool,
    /// `collapseDistance` (Double)
    #[serde(default)]
    pub collapse_distance: f64,
    /// `labelScale` (Single)
    #[serde(default)]
    pub label_scale: f32,
    /// `labelOffsetMultiplier` (Single)
    #[serde(default)]
    pub label_offset_multiplier: f32,
    /// `overlaySize` (UInt32)
    #[serde(default)]
    pub overlay_size: u32,
    /// `maxRelativeHideSize` (Single)
    #[serde(default)]
    pub max_relative_hide_size: f32,
    /// `minimumDisplaySizeMultiplier` (Single)
    #[serde(default)]
    pub minimum_display_size_multiplier: f32,
    /// `interpolateSettings` (Class)
    #[serde(default)]
    pub interpolate_settings: Option<Handle<UIDataBankDisplay3DInterpolateParams>>,
    /// `overlayGrayedOutColor` (Class)
    #[serde(default)]
    pub overlay_grayed_out_color: Option<Handle<SRGBA8>>,
    /// `overlayDefaultColor` (Class)
    #[serde(default)]
    pub overlay_default_color: Option<Handle<SRGBA8>>,
    /// `overlayHighlightedColor` (Class)
    #[serde(default)]
    pub overlay_highlighted_color: Option<Handle<SRGBA8>>,
    /// `overlaySelectedColor` (Class)
    #[serde(default)]
    pub overlay_selected_color: Option<Handle<SRGBA8>>,
    /// `overlaySelectedHighlightedColor` (Class)
    #[serde(default)]
    pub overlay_selected_highlighted_color: Option<Handle<SRGBA8>>,
    /// `orbitLineWidth` (Single)
    #[serde(default)]
    pub orbit_line_width: f32,
    /// `orbitLineDefaultColor` (Class)
    #[serde(default)]
    pub orbit_line_default_color: Option<Handle<SRGBA8>>,
    /// `orbitLineHighlightedColor` (Class)
    #[serde(default)]
    pub orbit_line_highlighted_color: Option<Handle<SRGBA8>>,
    /// `orbitLineUVStart` (Class)
    #[serde(default)]
    pub orbit_line_uvstart: Option<Handle<Vec2>>,
    /// `orbitLineUVSize` (Class)
    #[serde(default)]
    pub orbit_line_uvsize: Option<Handle<Vec2>>,
    /// `spaceDustSettings` (Class)
    #[serde(default)]
    pub space_dust_settings: Option<Handle<UIDataBankDisplay3DSpaceDustParams>>,
    /// `backdropGeometry` (Class)
    #[serde(default)]
    pub backdrop_geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `backdropMaterial` (Class)
    #[serde(default)]
    pub backdrop_material: Option<Handle<GlobalResourceMaterial>>,
    /// `backdropScale` (Single)
    #[serde(default)]
    pub backdrop_scale: f32,
}

impl Pooled for UIDataBankDisplay3DParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radar3dpresets.uidata_bank_display3_dparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radar3dpresets.uidata_bank_display3_dparams }
}

impl<'a> Extract<'a> for UIDataBankDisplay3DParams {
    const TYPE_NAME: &'static str = "UIDataBankDisplay3DParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            render_radius: inst.get_f64("renderRadius").unwrap_or_default(),
            render_node_material: match inst.get("renderNodeMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            input_params: match inst.get("inputParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UI3DDisplayInputParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UI3DDisplayInputParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            holographic_settings: match inst.get("holographicSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayHolographicSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayHolographicSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_mode_settings: match inst.get("rotationModeSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayRotationModeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayRotationModeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            center_to_self: inst.get_bool("centerToSelf").unwrap_or_default(),
            focus_change_duration: inst.get_f32("focusChangeDuration").unwrap_or_default(),
            extra_zoom_scale: inst.get_f32("extraZoomScale").unwrap_or_default(),
            childless_extra_zoom_scale: inst.get_f32("childlessExtraZoomScale").unwrap_or_default(),
            you_are_here_zoom_diameter: inst.get_f32("youAreHereZoomDiameter").unwrap_or_default(),
            max_zoom_scaling_per_update: inst.get_f32("maxZoomScalingPerUpdate").unwrap_or_default(),
            auto_rotation_settings: match inst.get("autoRotationSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayAutoRotationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayAutoRotationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sound_settings: match inst.get("soundSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplaySoundParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplaySoundParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            loadout_dummy_camera_offset: inst.get_f32("loadoutDummyCameraOffset").unwrap_or_default(),
            show_space_dust: inst.get_bool("showSpaceDust").unwrap_or_default(),
            collapse_distance: inst.get_f64("collapseDistance").unwrap_or_default(),
            label_scale: inst.get_f32("labelScale").unwrap_or_default(),
            label_offset_multiplier: inst.get_f32("labelOffsetMultiplier").unwrap_or_default(),
            overlay_size: inst.get_u32("overlaySize").unwrap_or_default(),
            max_relative_hide_size: inst.get_f32("maxRelativeHideSize").unwrap_or_default(),
            minimum_display_size_multiplier: inst.get_f32("minimumDisplaySizeMultiplier").unwrap_or_default(),
            interpolate_settings: match inst.get("interpolateSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIDataBankDisplay3DInterpolateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIDataBankDisplay3DInterpolateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_grayed_out_color: match inst.get("overlayGrayedOutColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_default_color: match inst.get("overlayDefaultColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_highlighted_color: match inst.get("overlayHighlightedColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_selected_color: match inst.get("overlaySelectedColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            overlay_selected_highlighted_color: match inst.get("overlaySelectedHighlightedColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orbit_line_width: inst.get_f32("orbitLineWidth").unwrap_or_default(),
            orbit_line_default_color: match inst.get("orbitLineDefaultColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orbit_line_highlighted_color: match inst.get("orbitLineHighlightedColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orbit_line_uvstart: match inst.get("orbitLineUVStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orbit_line_uvsize: match inst.get("orbitLineUVSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            space_dust_settings: match inst.get("spaceDustSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIDataBankDisplay3DSpaceDustParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIDataBankDisplay3DSpaceDustParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            backdrop_geometry: match inst.get("backdropGeometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            backdrop_material: match inst.get("backdropMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            backdrop_scale: inst.get_f32("backdropScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplayPathStateParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayPathStateParams {
    /// `renderNodeMaterial` (Class)
    #[serde(default)]
    pub render_node_material: Option<Handle<GlobalResourceMaterial>>,
    /// `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<RGBA>>,
    /// `scrollUvSpeed` (Single)
    #[serde(default)]
    pub scroll_uv_speed: f32,
}

impl Pooled for UIWorldDisplayPathStateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radar3dpresets.uiworld_display_path_state_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radar3dpresets.uiworld_display_path_state_params }
}

impl<'a> Extract<'a> for UIWorldDisplayPathStateParams {
    const TYPE_NAME: &'static str = "UIWorldDisplayPathStateParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            render_node_material: match inst.get("renderNodeMaterial") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceMaterial>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            scroll_uv_speed: inst.get_f32("scrollUvSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplayPathLineParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayPathLineParams {
    /// `lineWidth` (Single)
    #[serde(default)]
    pub line_width: f32,
    /// `uvStart` (Class)
    #[serde(default)]
    pub uv_start: Option<Handle<Vec2>>,
    /// `uvSize` (Class)
    #[serde(default)]
    pub uv_size: Option<Handle<Vec2>>,
    /// `cutOffExtraLengthDivision` (Int32)
    #[serde(default)]
    pub cut_off_extra_length_division: i32,
}

impl Pooled for UIWorldDisplayPathLineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radar3dpresets.uiworld_display_path_line_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radar3dpresets.uiworld_display_path_line_params }
}

impl<'a> Extract<'a> for UIWorldDisplayPathLineParams {
    const TYPE_NAME: &'static str = "UIWorldDisplayPathLineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            line_width: inst.get_f32("lineWidth").unwrap_or_default(),
            uv_start: match inst.get("uvStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            uv_size: match inst.get("uvSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cut_off_extra_length_division: inst.get_i32("cutOffExtraLengthDivision").unwrap_or_default(),
        }
    }
}

/// DCB type: `UIWorldDisplayPathParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIWorldDisplayPathParams {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `validStateSettings` (Class)
    #[serde(default)]
    pub valid_state_settings: Option<Handle<UIWorldDisplayPathStateParams>>,
    /// `invalidStateSettings` (Class)
    #[serde(default)]
    pub invalid_state_settings: Option<Handle<UIWorldDisplayPathStateParams>>,
    /// `pathLineSettings` (Class)
    #[serde(default)]
    pub path_line_settings: Option<Handle<UIWorldDisplayPathLineParams>>,
}

impl Pooled for UIWorldDisplayPathParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radar3dpresets.uiworld_display_path_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radar3dpresets.uiworld_display_path_params }
}

impl<'a> Extract<'a> for UIWorldDisplayPathParams {
    const TYPE_NAME: &'static str = "UIWorldDisplayPathParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            valid_state_settings: match inst.get("validStateSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayPathStateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayPathStateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            invalid_state_settings: match inst.get("invalidStateSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayPathStateParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayPathStateParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            path_line_settings: match inst.get("pathLineSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayPathLineParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayPathLineParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RadarDisplay3DPreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarDisplay3DPreset {
    /// `displayParams` (Class)
    #[serde(default)]
    pub display_params: Option<Handle<UIDataBankDisplay3DParams>>,
    /// `quantumPathDisplaySettings` (Class)
    #[serde(default)]
    pub quantum_path_display_settings: Option<Handle<UIWorldDisplayPathParams>>,
}

impl Pooled for RadarDisplay3DPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_radar3dpresets.radar_display3_dpreset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_radar3dpresets.radar_display3_dpreset }
}

impl<'a> Extract<'a> for RadarDisplay3DPreset {
    const TYPE_NAME: &'static str = "RadarDisplay3DPreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_params: match inst.get("displayParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIDataBankDisplay3DParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIDataBankDisplay3DParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            quantum_path_display_settings: match inst.get("quantumPathDisplaySettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UIWorldDisplayPathParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UIWorldDisplayPathParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

