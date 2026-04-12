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

/// DCB type: `DialogueBundle`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueBundle {
    /// DCB field: `usePooling` (Boolean)
    #[serde(default)]
    pub use_pooling: bool,
}

impl Pooled for DialogueBundle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.dialogue_bundle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.dialogue_bundle }
}

impl<'a> Extract<'a> for DialogueBundle {
    const TYPE_NAME: &'static str = "DialogueBundle";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            use_pooling: inst.get_bool("usePooling").unwrap_or_default(),
        }
    }
}

/// DCB type: `DialogueExternalSource`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueExternalSource {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `filename` (String)
    #[serde(default)]
    pub filename: String,
    /// DCB field: `localized` (Boolean)
    #[serde(default)]
    pub localized: bool,
}

impl Pooled for DialogueExternalSource {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.dialogue_external_source }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.dialogue_external_source }
}

impl<'a> Extract<'a> for DialogueExternalSource {
    const TYPE_NAME: &'static str = "DialogueExternalSource";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            filename: inst.get_str("filename").map(String::from).unwrap_or_default(),
            localized: inst.get_bool("localized").unwrap_or_default(),
        }
    }
}

/// DCB type: `DialogueContent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContent {
    /// DCB field: `localizedSubtitleText` (Locale)
    #[serde(default)]
    pub localized_subtitle_text: String,
    /// DCB field: `unlocalizedSubtitleText` (Locale)
    #[serde(default)]
    pub unlocalized_subtitle_text: String,
    /// DCB field: `subtitleShowDelay` (Single)
    #[serde(default)]
    pub subtitle_show_delay: f32,
    /// DCB field: `subtitleDisplayDurationOverride` (Single)
    #[serde(default)]
    pub subtitle_display_duration_override: f32,
    /// DCB field: `tempText` (String)
    #[serde(default)]
    pub temp_text: String,
    /// DCB field: `externalSources` (Reference (array))
    #[serde(default)]
    pub external_sources: Vec<CigGuid>,
}

impl Pooled for DialogueContent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.dialogue_content }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.dialogue_content }
}

impl<'a> Extract<'a> for DialogueContent {
    const TYPE_NAME: &'static str = "DialogueContent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            localized_subtitle_text: inst.get_str("localizedSubtitleText").map(String::from).unwrap_or_default(),
            unlocalized_subtitle_text: inst.get_str("unlocalizedSubtitleText").map(String::from).unwrap_or_default(),
            subtitle_show_delay: inst.get_f32("subtitleShowDelay").unwrap_or_default(),
            subtitle_display_duration_override: inst.get_f32("subtitleDisplayDurationOverride").unwrap_or_default(),
            temp_text: inst.get_str("tempText").map(String::from).unwrap_or_default(),
            external_sources: inst.get_array("externalSources")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DialogueContentBank`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContentBank {
    /// DCB field: `realm` (Reference)
    #[serde(default)]
    pub realm: Option<CigGuid>,
    /// DCB field: `character` (Reference)
    #[serde(default)]
    pub character: Option<CigGuid>,
    /// DCB field: `contents` (Reference (array))
    #[serde(default)]
    pub contents: Vec<CigGuid>,
}

impl Pooled for DialogueContentBank {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.dialogue_content_bank }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.dialogue_content_bank }
}

impl<'a> Extract<'a> for DialogueContentBank {
    const TYPE_NAME: &'static str = "DialogueContentBank";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            realm: inst.get("realm").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            character: inst.get("character").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            contents: inst.get_array("contents")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DialogueContext`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContext {
    /// DCB field: `speaker` (Reference)
    #[serde(default)]
    pub speaker: Option<CigGuid>,
    /// DCB field: `localizedSubtitleText` (Locale)
    #[serde(default)]
    pub localized_subtitle_text: String,
    /// DCB field: `unlocalizedSubtitleText` (Locale)
    #[serde(default)]
    pub unlocalized_subtitle_text: String,
    /// DCB field: `subtitleShowDelay` (Single)
    #[serde(default)]
    pub subtitle_show_delay: f32,
    /// DCB field: `subtitleDisplayDurationOverride` (Single)
    #[serde(default)]
    pub subtitle_display_duration_override: f32,
    /// DCB field: `tempText` (String)
    #[serde(default)]
    pub temp_text: String,
    /// DCB field: `audioTrigger` (Class)
    #[serde(default)]
    pub audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `femaleAudioTrigger` (Class)
    #[serde(default)]
    pub female_audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `forceSubtitles` (Boolean)
    #[serde(default)]
    pub force_subtitles: bool,
    /// DCB field: `contents` (Reference (array))
    #[serde(default)]
    pub contents: Vec<CigGuid>,
    /// DCB field: `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
}

impl Pooled for DialogueContext {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.dialogue_context }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.dialogue_context }
}

impl<'a> Extract<'a> for DialogueContext {
    const TYPE_NAME: &'static str = "DialogueContext";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            speaker: inst.get("speaker").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            localized_subtitle_text: inst.get_str("localizedSubtitleText").map(String::from).unwrap_or_default(),
            unlocalized_subtitle_text: inst.get_str("unlocalizedSubtitleText").map(String::from).unwrap_or_default(),
            subtitle_show_delay: inst.get_f32("subtitleShowDelay").unwrap_or_default(),
            subtitle_display_duration_override: inst.get_f32("subtitleDisplayDurationOverride").unwrap_or_default(),
            temp_text: inst.get_str("tempText").map(String::from).unwrap_or_default(),
            audio_trigger: match inst.get("audioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            female_audio_trigger: match inst.get("femaleAudioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            force_subtitles: inst.get_bool("forceSubtitles").unwrap_or_default(),
            contents: inst.get_array("contents")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DialogueContextBank`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContextBank {
    /// DCB field: `realm` (Reference)
    #[serde(default)]
    pub realm: Option<CigGuid>,
    /// DCB field: `character` (Reference)
    #[serde(default)]
    pub character: Option<CigGuid>,
    /// DCB field: `defaultTrigger` (Class)
    #[serde(default)]
    pub default_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `defaultFemaleTrigger` (Class)
    #[serde(default)]
    pub default_female_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `contexts` (Reference (array))
    #[serde(default)]
    pub contexts: Vec<CigGuid>,
}

impl Pooled for DialogueContextBank {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.dialogue_context_bank }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.dialogue_context_bank }
}

impl<'a> Extract<'a> for DialogueContextBank {
    const TYPE_NAME: &'static str = "DialogueContextBank";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            realm: inst.get("realm").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            character: inst.get("character").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            default_trigger: match inst.get("defaultTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_female_trigger: match inst.get("defaultFemaleTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contexts: inst.get_array("contexts")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DialogueRealm`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueRealm {
    /// DCB field: `defaultTrigger` (Class)
    #[serde(default)]
    pub default_trigger: Option<Handle<GlobalResourceAudio>>,
    /// DCB field: `defaultFemaleTrigger` (Class)
    #[serde(default)]
    pub default_female_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for DialogueRealm {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.dialogue_realm }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.dialogue_realm }
}

impl<'a> Extract<'a> for DialogueRealm {
    const TYPE_NAME: &'static str = "DialogueRealm";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_trigger: match inst.get("defaultTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_female_trigger: match inst.get("defaultFemaleTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DigitalSignageContent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignageContent {
    /// DCB field: `contentAspectRatio` (Reference)
    #[serde(default)]
    pub content_aspect_ratio: Option<CigGuid>,
    /// DCB field: `canvas` (Reference)
    #[serde(default)]
    pub canvas: Option<CigGuid>,
}

impl Pooled for DigitalSignageContent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.digital_signage_content }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.digital_signage_content }
}

impl<'a> Extract<'a> for DigitalSignageContent {
    const TYPE_NAME: &'static str = "DigitalSignageContent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            content_aspect_ratio: inst.get("contentAspectRatio").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            canvas: inst.get("canvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `DigitalSignageContentSet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignageContentSet {
    /// DCB field: `content` (Class (array))
    #[serde(default)]
    pub content: Vec<Handle<DigitalSignageContent>>,
}

impl Pooled for DigitalSignageContentSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.digital_signage_content_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.digital_signage_content_set }
}

impl<'a> Extract<'a> for DigitalSignageContentSet {
    const TYPE_NAME: &'static str = "DigitalSignageContentSet";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            content: inst.get_array("content")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<DigitalSignageContent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<DigitalSignageContent>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `DirectRTT_ChromaticAberrationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectRTT_ChromaticAberrationParams {
    /// DCB field: `RedChannelOffset` (Class)
    #[serde(default)]
    pub red_channel_offset: Option<Handle<Vec2>>,
    /// DCB field: `GreenChannelOffset` (Class)
    #[serde(default)]
    pub green_channel_offset: Option<Handle<Vec2>>,
    /// DCB field: `BlueChannelOffset` (Class)
    #[serde(default)]
    pub blue_channel_offset: Option<Handle<Vec2>>,
}

impl Pooled for DirectRTT_ChromaticAberrationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.direct_rtt_chromatic_aberration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.direct_rtt_chromatic_aberration_params }
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
    /// DCB field: `SharpShadowIntensity` (Single)
    #[serde(default)]
    pub sharp_shadow_intensity: f32,
    /// DCB field: `SoftShadowIntensity` (Single)
    #[serde(default)]
    pub soft_shadow_intensity: f32,
    /// DCB field: `Spread` (Single)
    #[serde(default)]
    pub spread: f32,
    /// DCB field: `Offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec2>>,
    /// DCB field: `Color` (Class)
    #[serde(default)]
    pub color: Option<Handle<SRGB8>>,
    /// DCB field: `OpacityInBrightScenes` (Single)
    #[serde(default)]
    pub opacity_in_bright_scenes: f32,
    /// DCB field: `OpacityInDarkScenes` (Single)
    #[serde(default)]
    pub opacity_in_dark_scenes: f32,
}

impl Pooled for DirectRTT_DropShadowParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.direct_rtt_drop_shadow_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.direct_rtt_drop_shadow_params }
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
    /// DCB field: `Cutoff` (Single)
    #[serde(default)]
    pub cutoff: f32,
    /// DCB field: `BloomAdditiveBlendFactor` (Single)
    #[serde(default)]
    pub bloom_additive_blend_factor: f32,
    /// DCB field: `SaturationFactor` (Single)
    #[serde(default)]
    pub saturation_factor: f32,
    /// DCB field: `OpacityInBrightScenes` (Single)
    #[serde(default)]
    pub opacity_in_bright_scenes: f32,
    /// DCB field: `OpacityInDarkScenes` (Single)
    #[serde(default)]
    pub opacity_in_dark_scenes: f32,
}

impl Pooled for DirectRTT_BloomParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.direct_rtt_bloom_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.direct_rtt_bloom_params }
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
    /// DCB field: `TexturePath` (String)
    #[serde(default)]
    pub texture_path: String,
    /// DCB field: `Intensity` (Single)
    #[serde(default)]
    pub intensity: f32,
    /// DCB field: `Tiling` (Single)
    #[serde(default)]
    pub tiling: f32,
    /// DCB field: `ScrollSpeed` (Single)
    #[serde(default)]
    pub scroll_speed: f32,
}

impl Pooled for DirectRTT_PixelGridParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.direct_rtt_pixel_grid_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.direct_rtt_pixel_grid_params }
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
    /// DCB field: `Amount` (Single)
    #[serde(default)]
    pub amount: f32,
    /// DCB field: `Speed` (Single)
    #[serde(default)]
    pub speed: f32,
    /// DCB field: `Tiling` (Single)
    #[serde(default)]
    pub tiling: f32,
    /// DCB field: `Brightness` (Single)
    #[serde(default)]
    pub brightness: f32,
}

impl Pooled for DirectRTT_InterferenceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.direct_rtt_interference_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.direct_rtt_interference_params }
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
    /// DCB field: `BlurRadius` (Single)
    #[serde(default)]
    pub blur_radius: f32,
    /// DCB field: `OpacityInBrightScenes` (Single)
    #[serde(default)]
    pub opacity_in_bright_scenes: f32,
    /// DCB field: `OpacityInDarkScenes` (Single)
    #[serde(default)]
    pub opacity_in_dark_scenes: f32,
    /// DCB field: `AdditiveBlendFactor` (Single)
    #[serde(default)]
    pub additive_blend_factor: f32,
    /// DCB field: `ChromaticAberrationParams` (Class)
    #[serde(default)]
    pub chromatic_aberration_params: Option<Handle<DirectRTT_ChromaticAberrationParams>>,
    /// DCB field: `DropShadowParams` (Class)
    #[serde(default)]
    pub drop_shadow_params: Option<Handle<DirectRTT_DropShadowParams>>,
    /// DCB field: `BloomParams` (Class)
    #[serde(default)]
    pub bloom_params: Option<Handle<DirectRTT_BloomParams>>,
    /// DCB field: `PixelGridParams` (Class)
    #[serde(default)]
    pub pixel_grid_params: Option<Handle<DirectRTT_PixelGridParams>>,
    /// DCB field: `ScreenInterferenceParams` (Class)
    #[serde(default)]
    pub screen_interference_params: Option<Handle<DirectRTT_InterferenceParams>>,
}

impl Pooled for DirectRTT_AfterTonemappingParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.direct_rtt_after_tonemapping_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.direct_rtt_after_tonemapping_params }
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

/// DCB type: `DifficultyModifierRange`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyModifierRange {
    /// DCB field: `min` (Single)
    #[serde(default)]
    pub min: f32,
    /// DCB field: `max` (Single)
    #[serde(default)]
    pub max: f32,
    /// DCB field: `exposedToPlayer` (Boolean)
    #[serde(default)]
    pub exposed_to_player: bool,
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
}

impl Pooled for DifficultyModifierRange {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.difficulty_modifier_range }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.difficulty_modifier_range }
}

impl<'a> Extract<'a> for DifficultyModifierRange {
    const TYPE_NAME: &'static str = "DifficultyModifierRange";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min: inst.get_f32("min").unwrap_or_default(),
            max: inst.get_f32("max").unwrap_or_default(),
            exposed_to_player: inst.get_bool("exposedToPlayer").unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `DifficultyLevelParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyLevelParams {
    /// DCB field: `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `displayHeader` (Locale)
    #[serde(default)]
    pub display_header: String,
    /// DCB field: `displayBody` (Locale)
    #[serde(default)]
    pub display_body: String,
    /// DCB field: `vehicleParams` (Class)
    #[serde(default)]
    pub vehicle_params: Option<Handle<VehicleDifficultyParams>>,
    /// DCB field: `modifiers` (Class)
    #[serde(default)]
    pub modifiers: Option<Handle<DifficultyModifierRange>>,
}

impl Pooled for DifficultyLevelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.difficulty_level_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.difficulty_level_params }
}

impl<'a> Extract<'a> for DifficultyLevelParams {
    const TYPE_NAME: &'static str = "DifficultyLevelParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            display_header: inst.get_str("displayHeader").map(String::from).unwrap_or_default(),
            display_body: inst.get_str("displayBody").map(String::from).unwrap_or_default(),
            vehicle_params: match inst.get("vehicleParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VehicleDifficultyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VehicleDifficultyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            modifiers: match inst.get("modifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<DifficultyModifierRange>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<DifficultyModifierRange>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DisplayState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayState {
    /// DCB field: `minimumValue` (Single)
    #[serde(default)]
    pub minimum_value: f32,
    /// DCB field: `maximumValue` (Single)
    #[serde(default)]
    pub maximum_value: f32,
    /// DCB field: `displayDuration` (Single)
    #[serde(default)]
    pub display_duration: f32,
    /// DCB field: `activeRange` (EnumChoice)
    #[serde(default)]
    pub active_range: String,
    /// DCB field: `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
}

impl Pooled for DisplayState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_di.display_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_di.display_state }
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

