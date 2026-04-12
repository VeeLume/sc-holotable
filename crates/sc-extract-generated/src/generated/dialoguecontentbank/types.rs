// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `dialoguecontentbank`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `DialogueExternalSource`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueExternalSource {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `filename` (String)
    #[serde(default)]
    pub filename: String,
    /// `localized` (Boolean)
    #[serde(default)]
    pub localized: bool,
}

impl Pooled for DialogueExternalSource {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dialoguecontentbank.dialogue_external_source }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dialoguecontentbank.dialogue_external_source }
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
    /// `localizedSubtitleText` (Locale)
    #[serde(default)]
    pub localized_subtitle_text: String,
    /// `unlocalizedSubtitleText` (Locale)
    #[serde(default)]
    pub unlocalized_subtitle_text: String,
    /// `subtitleShowDelay` (Single)
    #[serde(default)]
    pub subtitle_show_delay: f32,
    /// `subtitleDisplayDurationOverride` (Single)
    #[serde(default)]
    pub subtitle_display_duration_override: f32,
    /// `tempText` (String)
    #[serde(default)]
    pub temp_text: String,
    /// `externalSources` (Reference (array))
    #[serde(default)]
    pub external_sources: Vec<CigGuid>,
}

impl Pooled for DialogueContent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dialoguecontentbank.dialogue_content }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dialoguecontentbank.dialogue_content }
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
    /// `realm` (Reference)
    #[serde(default)]
    pub realm: Option<CigGuid>,
    /// `character` (Reference)
    #[serde(default)]
    pub character: Option<CigGuid>,
    /// `contents` (Reference (array))
    #[serde(default)]
    pub contents: Vec<CigGuid>,
}

impl Pooled for DialogueContentBank {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.dialoguecontentbank.dialogue_content_bank }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.dialoguecontentbank.dialogue_content_bank }
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

