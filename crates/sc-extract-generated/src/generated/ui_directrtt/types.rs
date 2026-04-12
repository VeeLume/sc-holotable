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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `DirectRTT_ChromaticAberrationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectRTT_ChromaticAberrationParams {
    /// `RedChannelOffset` (Class)
    #[serde(default)]
    pub red_channel_offset: Option<Handle<Vec2>>,
    /// `GreenChannelOffset` (Class)
    #[serde(default)]
    pub green_channel_offset: Option<Handle<Vec2>>,
    /// `BlueChannelOffset` (Class)
    #[serde(default)]
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            green_channel_offset: match inst.get("GreenChannelOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            blue_channel_offset: match inst.get("BlueChannelOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DirectRTT_DropShadowParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectRTT_DropShadowParams {
    /// `SharpShadowIntensity` (Single)
    #[serde(default)]
    pub sharp_shadow_intensity: f32,
    /// `SoftShadowIntensity` (Single)
    #[serde(default)]
    pub soft_shadow_intensity: f32,
    /// `Spread` (Single)
    #[serde(default)]
    pub spread: f32,
    /// `Offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec2>>,
    /// `Color` (Class)
    #[serde(default)]
    pub color: Option<Handle<SRGB8>>,
    /// `OpacityInBrightScenes` (Single)
    #[serde(default)]
    pub opacity_in_bright_scenes: f32,
    /// `OpacityInDarkScenes` (Single)
    #[serde(default)]
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            color: match inst.get("Color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            opacity_in_bright_scenes: inst.get_f32("OpacityInBrightScenes").unwrap_or_default(),
            opacity_in_dark_scenes: inst.get_f32("OpacityInDarkScenes").unwrap_or_default(),
        }
    }
}

/// DCB type: `DirectRTT_BloomParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectRTT_BloomParams {
    /// `Cutoff` (Single)
    #[serde(default)]
    pub cutoff: f32,
    /// `BloomAdditiveBlendFactor` (Single)
    #[serde(default)]
    pub bloom_additive_blend_factor: f32,
    /// `SaturationFactor` (Single)
    #[serde(default)]
    pub saturation_factor: f32,
    /// `OpacityInBrightScenes` (Single)
    #[serde(default)]
    pub opacity_in_bright_scenes: f32,
    /// `OpacityInDarkScenes` (Single)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectRTT_PixelGridParams {
    /// `TexturePath` (String)
    #[serde(default)]
    pub texture_path: String,
    /// `Intensity` (Single)
    #[serde(default)]
    pub intensity: f32,
    /// `Tiling` (Single)
    #[serde(default)]
    pub tiling: f32,
    /// `ScrollSpeed` (Single)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectRTT_InterferenceParams {
    /// `Amount` (Single)
    #[serde(default)]
    pub amount: f32,
    /// `Speed` (Single)
    #[serde(default)]
    pub speed: f32,
    /// `Tiling` (Single)
    #[serde(default)]
    pub tiling: f32,
    /// `Brightness` (Single)
    #[serde(default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectRTT_AfterTonemappingParams {
    /// `BlurRadius` (Single)
    #[serde(default)]
    pub blur_radius: f32,
    /// `OpacityInBrightScenes` (Single)
    #[serde(default)]
    pub opacity_in_bright_scenes: f32,
    /// `OpacityInDarkScenes` (Single)
    #[serde(default)]
    pub opacity_in_dark_scenes: f32,
    /// `AdditiveBlendFactor` (Single)
    #[serde(default)]
    pub additive_blend_factor: f32,
    /// `ChromaticAberrationParams` (Class)
    #[serde(default)]
    pub chromatic_aberration_params: Option<Handle<DirectRTT_ChromaticAberrationParams>>,
    /// `DropShadowParams` (Class)
    #[serde(default)]
    pub drop_shadow_params: Option<Handle<DirectRTT_DropShadowParams>>,
    /// `BloomParams` (Class)
    #[serde(default)]
    pub bloom_params: Option<Handle<DirectRTT_BloomParams>>,
    /// `PixelGridParams` (Class)
    #[serde(default)]
    pub pixel_grid_params: Option<Handle<DirectRTT_PixelGridParams>>,
    /// `ScreenInterferenceParams` (Class)
    #[serde(default)]
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DirectRTT_ChromaticAberrationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drop_shadow_params: match inst.get("DropShadowParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DirectRTT_DropShadowParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DirectRTT_DropShadowParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bloom_params: match inst.get("BloomParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DirectRTT_BloomParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DirectRTT_BloomParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pixel_grid_params: match inst.get("PixelGridParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DirectRTT_PixelGridParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DirectRTT_PixelGridParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            screen_interference_params: match inst.get("ScreenInterferenceParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DirectRTT_InterferenceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DirectRTT_InterferenceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

