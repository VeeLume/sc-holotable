// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-uielements`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `UIElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    /// `sourceFile` (String)
    #[serde(default)]
    pub source_file: String,
    /// `soundsRecord` (Reference)
    #[serde(default)]
    pub sounds_record: Option<CigGuid>,
    /// `fontFile` (String)
    #[serde(default)]
    pub font_file: String,
    /// `alignMode` (EnumChoice)
    #[serde(default)]
    pub align_mode: String,
    /// `alignScale` (Boolean)
    #[serde(default)]
    pub align_scale: bool,
    /// `alignMax` (Boolean)
    #[serde(default)]
    pub align_max: bool,
    /// `layer` (Int32)
    #[serde(default)]
    pub layer: i32,
    /// `skipFirstFrameInit` (Boolean)
    #[serde(default)]
    pub skip_first_frame_init: bool,
    /// `keyEvents` (Boolean)
    #[serde(default)]
    pub key_events: bool,
    /// `eventsExclusive` (Boolean)
    #[serde(default)]
    pub events_exclusive: bool,
    /// `noDepthTest` (Boolean)
    #[serde(default)]
    pub no_depth_test: bool,
    /// `forceNoUnload` (Boolean)
    #[serde(default)]
    pub force_no_unload: bool,
    /// `noPostProcessing` (Boolean)
    #[serde(default)]
    pub no_post_processing: bool,
}

impl Pooled for UIElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_uielements.uielement }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_uielements.uielement }
}

impl<'a> Extract<'a> for UIElement {
    const TYPE_NAME: &'static str = "UIElement";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            source_file: inst.get_str("sourceFile").map(String::from).unwrap_or_default(),
            sounds_record: inst.get("soundsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            font_file: inst.get_str("fontFile").map(String::from).unwrap_or_default(),
            align_mode: inst.get_str("alignMode").map(String::from).unwrap_or_default(),
            align_scale: inst.get_bool("alignScale").unwrap_or_default(),
            align_max: inst.get_bool("alignMax").unwrap_or_default(),
            layer: inst.get_i32("layer").unwrap_or_default(),
            skip_first_frame_init: inst.get_bool("skipFirstFrameInit").unwrap_or_default(),
            key_events: inst.get_bool("keyEvents").unwrap_or_default(),
            events_exclusive: inst.get_bool("eventsExclusive").unwrap_or_default(),
            no_depth_test: inst.get_bool("noDepthTest").unwrap_or_default(),
            force_no_unload: inst.get_bool("forceNoUnload").unwrap_or_default(),
            no_post_processing: inst.get_bool("noPostProcessing").unwrap_or_default(),
        }
    }
}

