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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `PopupDef`
pub struct PopupDef {
    /// `title` (Locale)
    pub title: LocaleKey,
    /// `message1` (Locale)
    pub message1: LocaleKey,
    /// `message2` (Locale)
    pub message2: LocaleKey,
    /// `message3` (Locale)
    pub message3: LocaleKey,
    /// `hasCancelButton` (Boolean)
    pub has_cancel_button: bool,
    /// `hasConfirmButton` (Boolean)
    pub has_confirm_button: bool,
    /// `cancelOverrideString` (Locale)
    pub cancel_override_string: LocaleKey,
    /// `confirmOverrideString` (Locale)
    pub confirm_override_string: LocaleKey,
    /// `popupFrame` (String)
    pub popup_frame: String,
    /// `popupHeaderFrame` (String)
    pub popup_header_frame: String,
}

impl Pooled for PopupDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_popups.popup_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_popups.popup_def
    }
}

impl<'a> Extract<'a> for PopupDef {
    const TYPE_NAME: &'static str = "PopupDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            title: inst
                .get_str("title")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            message1: inst
                .get_str("message1")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            message2: inst
                .get_str("message2")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            message3: inst
                .get_str("message3")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            has_cancel_button: inst.get_bool("hasCancelButton").unwrap_or_default(),
            has_confirm_button: inst.get_bool("hasConfirmButton").unwrap_or_default(),
            cancel_override_string: inst
                .get_str("cancelOverrideString")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            confirm_override_string: inst
                .get_str("confirmOverrideString")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            popup_frame: inst
                .get_str("popupFrame")
                .map(String::from)
                .unwrap_or_default(),
            popup_header_frame: inst
                .get_str("popupHeaderFrame")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}
