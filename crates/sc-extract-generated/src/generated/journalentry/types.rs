// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `journalentry`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `BaseJournalEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseJournalEntry {
}

impl Pooled for BaseJournalEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.journalentry.base_journal_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.journalentry.base_journal_entry }
}

impl<'a> Extract<'a> for BaseJournalEntry {
    const TYPE_NAME: &'static str = "BaseJournalEntry";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `JournalEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    /// `Title` (Locale)
    #[serde(default)]
    pub title: String,
    /// `ShortTitle` (Locale)
    #[serde(default)]
    pub short_title: String,
    /// `SubHeading` (Locale)
    #[serde(default)]
    pub sub_heading: String,
    /// `autoOpenMobiGlas` (Boolean)
    #[serde(default)]
    pub auto_open_mobi_glas: bool,
    /// `showNotification` (Boolean)
    #[serde(default)]
    pub show_notification: bool,
    /// `missionSpecificContent` (Boolean)
    #[serde(default)]
    pub mission_specific_content: bool,
    /// `removeOnMissionEnd` (Boolean)
    #[serde(default)]
    pub remove_on_mission_end: bool,
    /// `Style` (Reference)
    #[serde(default)]
    pub style: Option<CigGuid>,
    /// `type` (StrongPointer)
    #[serde(default)]
    pub r#type: Option<Handle<BaseJournalEntry>>,
    /// `tutorialEntry` (Boolean)
    #[serde(default)]
    pub tutorial_entry: bool,
}

impl Pooled for JournalEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.journalentry.journal_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.journalentry.journal_entry }
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

/// DCB type: `SReputationStandingJournalEntryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationStandingJournalEntryParams {
    /// `changeDirection` (EnumChoice)
    #[serde(default)]
    pub change_direction: String,
    /// `standing` (Reference)
    #[serde(default)]
    pub standing: Option<CigGuid>,
    /// `journalEntry` (Reference)
    #[serde(default)]
    pub journal_entry: Option<CigGuid>,
}

impl Pooled for SReputationStandingJournalEntryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.journalentry.sreputation_standing_journal_entry_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.journalentry.sreputation_standing_journal_entry_params }
}

impl<'a> Extract<'a> for SReputationStandingJournalEntryParams {
    const TYPE_NAME: &'static str = "SReputationStandingJournalEntryParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            change_direction: inst.get_str("changeDirection").map(String::from).unwrap_or_default(),
            standing: inst.get("standing").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            journal_entry: inst.get("journalEntry").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SReputationJournalGroupParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationJournalGroupParams {
    /// `standingEntries` (Class (array))
    #[serde(default)]
    pub standing_entries: Vec<Handle<SReputationStandingJournalEntryParams>>,
}

impl Pooled for SReputationJournalGroupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.journalentry.sreputation_journal_group_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.journalentry.sreputation_journal_group_params }
}

impl<'a> Extract<'a> for SReputationJournalGroupParams {
    const TYPE_NAME: &'static str = "SReputationJournalGroupParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            standing_entries: inst.get_array("standingEntries")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationStandingJournalEntryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationStandingJournalEntryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationJournalEntriesParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationJournalEntriesParams {
    /// `factionReputation` (Reference)
    #[serde(default)]
    pub faction_reputation: Option<CigGuid>,
    /// `reputationScope` (Reference)
    #[serde(default)]
    pub reputation_scope: Option<CigGuid>,
    /// `journalGroups` (Class (array))
    #[serde(default)]
    pub journal_groups: Vec<Handle<SReputationJournalGroupParams>>,
}

impl Pooled for SReputationJournalEntriesParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.journalentry.sreputation_journal_entries_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.journalentry.sreputation_journal_entries_params }
}

impl<'a> Extract<'a> for SReputationJournalEntriesParams {
    const TYPE_NAME: &'static str = "SReputationJournalEntriesParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            faction_reputation: inst.get("factionReputation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reputation_scope: inst.get("reputationScope").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            journal_groups: inst.get_array("journalGroups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationJournalGroupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationJournalGroupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationJournalEntryHandlerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SReputationJournalEntryHandlerParams {
    /// `reputationTypes` (Class (array))
    #[serde(default)]
    pub reputation_types: Vec<Handle<SReputationJournalEntriesParams>>,
}

impl Pooled for SReputationJournalEntryHandlerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.journalentry.sreputation_journal_entry_handler_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.journalentry.sreputation_journal_entry_handler_params }
}

impl<'a> Extract<'a> for SReputationJournalEntryHandlerParams {
    const TYPE_NAME: &'static str = "SReputationJournalEntryHandlerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reputation_types: inst.get_array("reputationTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SReputationJournalEntriesParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SReputationJournalEntriesParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

