// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-popups`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `PopupDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupDef {
    /// `title` (Locale)
    #[serde(default)]
    pub title: String,
    /// `message1` (Locale)
    #[serde(default)]
    pub message1: String,
    /// `message2` (Locale)
    #[serde(default)]
    pub message2: String,
    /// `message3` (Locale)
    #[serde(default)]
    pub message3: String,
    /// `hasCancelButton` (Boolean)
    #[serde(default)]
    pub has_cancel_button: bool,
    /// `hasConfirmButton` (Boolean)
    #[serde(default)]
    pub has_confirm_button: bool,
    /// `cancelOverrideString` (Locale)
    #[serde(default)]
    pub cancel_override_string: String,
    /// `confirmOverrideString` (Locale)
    #[serde(default)]
    pub confirm_override_string: String,
    /// `popupFrame` (String)
    #[serde(default)]
    pub popup_frame: String,
    /// `popupHeaderFrame` (String)
    #[serde(default)]
    pub popup_header_frame: String,
}

impl Pooled for PopupDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_popups.popup_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_popups.popup_def }
}

impl<'a> Extract<'a> for PopupDef {
    const TYPE_NAME: &'static str = "PopupDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            title: inst.get_str("title").map(String::from).unwrap_or_default(),
            message1: inst.get_str("message1").map(String::from).unwrap_or_default(),
            message2: inst.get_str("message2").map(String::from).unwrap_or_default(),
            message3: inst.get_str("message3").map(String::from).unwrap_or_default(),
            has_cancel_button: inst.get_bool("hasCancelButton").unwrap_or_default(),
            has_confirm_button: inst.get_bool("hasConfirmButton").unwrap_or_default(),
            cancel_override_string: inst.get_str("cancelOverrideString").map(String::from).unwrap_or_default(),
            confirm_override_string: inst.get_str("confirmOverrideString").map(String::from).unwrap_or_default(),
            popup_frame: inst.get_str("popupFrame").map(String::from).unwrap_or_default(),
            popup_header_frame: inst.get_str("popupHeaderFrame").map(String::from).unwrap_or_default(),
        }
    }
}

