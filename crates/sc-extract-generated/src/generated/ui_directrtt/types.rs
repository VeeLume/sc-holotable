// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-directrtt`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `DirectRTT_ChromaticAberrationParams`
pub struct DirectRTT_ChromaticAberrationParams {
    /// `RedChannelOffset` (Class)
    pub red_channel_offset: Option<Handle<Vec2>>,
    /// `GreenChannelOffset` (Class)
    pub green_channel_offset: Option<Handle<Vec2>>,
    /// `BlueChannelOffset` (Class)
    pub blue_channel_offset: Option<Handle<Vec2>>,
}

impl Pooled for DirectRTT_ChromaticAberrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_directrtt.direct_rtt_chromatic_aberration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_directrtt.direct_rtt_chromatic_aberration_params }
}

impl<'a> Extract<'a> for DirectRTT_ChromaticAberrationParams {
    const TYPE_NAME: &'static str = "DirectRTT_ChromaticAberrationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            red_channel_offset: match inst.get("RedChannelOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            green_channel_offset: match inst.get("GreenChannelOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            blue_channel_offset: match inst.get("BlueChannelOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DirectRTT_DropShadowParams`
pub struct DirectRTT_DropShadowParams {
    /// `SharpShadowIntensity` (Single)
    pub sharp_shadow_intensity: f32,
    /// `SoftShadowIntensity` (Single)
    pub soft_shadow_intensity: f32,
    /// `Spread` (Single)
    pub spread: f32,
    /// `Offset` (Class)
    pub offset: Option<Handle<Vec2>>,
    /// `Color` (Class)
    pub color: Option<Handle<SRGB8>>,
    /// `OpacityInBrightScenes` (Single)
    pub opacity_in_bright_scenes: f32,
    /// `OpacityInDarkScenes` (Single)
    pub opacity_in_dark_scenes: f32,
}

impl Pooled for DirectRTT_DropShadowParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_directrtt.direct_rtt_drop_shadow_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_directrtt.direct_rtt_drop_shadow_params }
}

impl<'a> Extract<'a> for DirectRTT_DropShadowParams {
    const TYPE_NAME: &'static str = "DirectRTT_DropShadowParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sharp_shadow_intensity: inst.get_f32("SharpShadowIntensity").unwrap_or_default(),
            soft_shadow_intensity: inst.get_f32("SoftShadowIntensity").unwrap_or_default(),
            spread: inst.get_f32("Spread").unwrap_or_default(),
            offset: match inst.get("Offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            color: match inst.get("Color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            opacity_in_bright_scenes: inst.get_f32("OpacityInBrightScenes").unwrap_or_default(),
            opacity_in_dark_scenes: inst.get_f32("OpacityInDarkScenes").unwrap_or_default(),
        }
    }
}

/// DCB type: `DirectRTT_BloomParams`
pub struct DirectRTT_BloomParams {
    /// `Cutoff` (Single)
    pub cutoff: f32,
    /// `BloomAdditiveBlendFactor` (Single)
    pub bloom_additive_blend_factor: f32,
    /// `SaturationFactor` (Single)
    pub saturation_factor: f32,
    /// `OpacityInBrightScenes` (Single)
    pub opacity_in_bright_scenes: f32,
    /// `OpacityInDarkScenes` (Single)
    pub opacity_in_dark_scenes: f32,
}

impl Pooled for DirectRTT_BloomParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_directrtt.direct_rtt_bloom_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_directrtt.direct_rtt_bloom_params }
}

impl<'a> Extract<'a> for DirectRTT_BloomParams {
    const TYPE_NAME: &'static str = "DirectRTT_BloomParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            cutoff: inst.get_f32("Cutoff").unwrap_or_default(),
            bloom_additive_blend_factor: inst.get_f32("BloomAdditiveBlendFactor").unwrap_or_default(),
            saturation_factor: inst.get_f32("SaturationFactor").unwrap_or_default(),
            opacity_in_bright_scenes: inst.get_f32("OpacityInBrightScenes").unwrap_or_default(),
            opacity_in_dark_scenes: inst.get_f32("OpacityInDarkScenes").unwrap_or_default(),
        }
    }
}

/// DCB type: `DirectRTT_PixelGridParams`
pub struct DirectRTT_PixelGridParams {
    /// `TexturePath` (String)
    pub texture_path: String,
    /// `Intensity` (Single)
    pub intensity: f32,
    /// `Tiling` (Single)
    pub tiling: f32,
    /// `ScrollSpeed` (Single)
    pub scroll_speed: f32,
}

impl Pooled for DirectRTT_PixelGridParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_directrtt.direct_rtt_pixel_grid_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_directrtt.direct_rtt_pixel_grid_params }
}

impl<'a> Extract<'a> for DirectRTT_PixelGridParams {
    const TYPE_NAME: &'static str = "DirectRTT_PixelGridParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            texture_path: inst.get_str("TexturePath").map(String::from).unwrap_or_default(),
            intensity: inst.get_f32("Intensity").unwrap_or_default(),
            tiling: inst.get_f32("Tiling").unwrap_or_default(),
            scroll_speed: inst.get_f32("ScrollSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `DirectRTT_InterferenceParams`
pub struct DirectRTT_InterferenceParams {
    /// `Amount` (Single)
    pub amount: f32,
    /// `Speed` (Single)
    pub speed: f32,
    /// `Tiling` (Single)
    pub tiling: f32,
    /// `Brightness` (Single)
    pub brightness: f32,
}

impl Pooled for DirectRTT_InterferenceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_directrtt.direct_rtt_interference_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_directrtt.direct_rtt_interference_params }
}

impl<'a> Extract<'a> for DirectRTT_InterferenceParams {
    const TYPE_NAME: &'static str = "DirectRTT_InterferenceParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            amount: inst.get_f32("Amount").unwrap_or_default(),
            speed: inst.get_f32("Speed").unwrap_or_default(),
            tiling: inst.get_f32("Tiling").unwrap_or_default(),
            brightness: inst.get_f32("Brightness").unwrap_or_default(),
        }
    }
}

/// DCB type: `DirectRTT_AfterTonemappingParams`
pub struct DirectRTT_AfterTonemappingParams {
    /// `BlurRadius` (Single)
    pub blur_radius: f32,
    /// `OpacityInBrightScenes` (Single)
    pub opacity_in_bright_scenes: f32,
    /// `OpacityInDarkScenes` (Single)
    pub opacity_in_dark_scenes: f32,
    /// `AdditiveBlendFactor` (Single)
    pub additive_blend_factor: f32,
    /// `ChromaticAberrationParams` (Class)
    pub chromatic_aberration_params: Option<Handle<DirectRTT_ChromaticAberrationParams>>,
    /// `DropShadowParams` (Class)
    pub drop_shadow_params: Option<Handle<DirectRTT_DropShadowParams>>,
    /// `BloomParams` (Class)
    pub bloom_params: Option<Handle<DirectRTT_BloomParams>>,
    /// `PixelGridParams` (Class)
    pub pixel_grid_params: Option<Handle<DirectRTT_PixelGridParams>>,
    /// `ScreenInterferenceParams` (Class)
    pub screen_interference_params: Option<Handle<DirectRTT_InterferenceParams>>,
}

impl Pooled for DirectRTT_AfterTonemappingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_directrtt.direct_rtt_after_tonemapping_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_directrtt.direct_rtt_after_tonemapping_params }
}

impl<'a> Extract<'a> for DirectRTT_AfterTonemappingParams {
    const TYPE_NAME: &'static str = "DirectRTT_AfterTonemappingParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            blur_radius: inst.get_f32("BlurRadius").unwrap_or_default(),
            opacity_in_bright_scenes: inst.get_f32("OpacityInBrightScenes").unwrap_or_default(),
            opacity_in_dark_scenes: inst.get_f32("OpacityInDarkScenes").unwrap_or_default(),
            additive_blend_factor: inst.get_f32("AdditiveBlendFactor").unwrap_or_default(),
            chromatic_aberration_params: match inst.get("ChromaticAberrationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DirectRTT_ChromaticAberrationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            drop_shadow_params: match inst.get("DropShadowParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DirectRTT_DropShadowParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            bloom_params: match inst.get("BloomParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DirectRTT_BloomParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            pixel_grid_params: match inst.get("PixelGridParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DirectRTT_PixelGridParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            screen_interference_params: match inst.get("ScreenInterferenceParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DirectRTT_InterferenceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

