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

/// DCB type: `JournalEntryType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryType {
    /// DCB field: `DisplayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// DCB field: `NotificationName` (Locale)
    #[serde(default)]
    pub notification_name: String,
    /// DCB field: `IconName` (String)
    #[serde(default)]
    pub icon_name: String,
}

impl Pooled for JournalEntryType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_jo.journal_entry_type }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_jo.journal_entry_type }
}

impl<'a> Extract<'a> for JournalEntryType {
    const TYPE_NAME: &'static str = "JournalEntryType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("DisplayName").map(String::from).unwrap_or_default(),
            notification_name: inst.get_str("NotificationName").map(String::from).unwrap_or_default(),
            icon_name: inst.get_str("IconName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `JournalEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    /// DCB field: `Title` (Locale)
    #[serde(default)]
    pub title: String,
    /// DCB field: `ShortTitle` (Locale)
    #[serde(default)]
    pub short_title: String,
    /// DCB field: `SubHeading` (Locale)
    #[serde(default)]
    pub sub_heading: String,
    /// DCB field: `autoOpenMobiGlas` (Boolean)
    #[serde(default)]
    pub auto_open_mobi_glas: bool,
    /// DCB field: `showNotification` (Boolean)
    #[serde(default)]
    pub show_notification: bool,
    /// DCB field: `missionSpecificContent` (Boolean)
    #[serde(default)]
    pub mission_specific_content: bool,
    /// DCB field: `removeOnMissionEnd` (Boolean)
    #[serde(default)]
    pub remove_on_mission_end: bool,
    /// DCB field: `Style` (Reference)
    #[serde(default)]
    pub style: Option<CigGuid>,
    /// DCB field: `type` (StrongPointer)
    #[serde(default)]
    pub r#type: Option<Handle<BaseJournalEntry>>,
    /// DCB field: `tutorialEntry` (Boolean)
    #[serde(default)]
    pub tutorial_entry: bool,
}

impl Pooled for JournalEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_jo.journal_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_jo.journal_entry }
}

impl<'a> Extract<'a> for JournalEntry {
    const TYPE_NAME: &'static str = "JournalEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            title: inst.get_str("Title").map(String::from).unwrap_or_default(),
            short_title: inst.get_str("ShortTitle").map(String::from).unwrap_or_default(),
            sub_heading: inst.get_str("SubHeading").map(String::from).unwrap_or_default(),
            auto_open_mobi_glas: inst.get_bool("autoOpenMobiGlas").unwrap_or_default(),
            show_notification: inst.get_bool("showNotification").unwrap_or_default(),
            mission_specific_content: inst.get_bool("missionSpecificContent").unwrap_or_default(),
            remove_on_mission_end: inst.get_bool("removeOnMissionEnd").unwrap_or_default(),
            style: inst.get("Style").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            r#type: match inst.get("type") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BaseJournalEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BaseJournalEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tutorial_entry: inst.get_bool("tutorialEntry").unwrap_or_default(),
        }
    }
}

