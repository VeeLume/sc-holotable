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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `JournalEntryAudioLog`
/// Inherits from: `BaseJournalEntry`
pub struct JournalEntryAudioLog {
    /// `AudioLogName` (String)
    pub audio_log_name: String,
    /// `Description` (Locale)
    pub description: LocaleKey,
    /// `Transcript` (Locale)
    pub transcript: LocaleKey,
}

impl Pooled for JournalEntryAudioLog {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.journalentry.journal_entry_audio_log
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.journalentry.journal_entry_audio_log
    }
}

impl<'a> Extract<'a> for JournalEntryAudioLog {
    const TYPE_NAME: &'static str = "JournalEntryAudioLog";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            audio_log_name: inst
                .get_str("AudioLogName")
                .map(String::from)
                .unwrap_or_default(),
            description: inst
                .get_str("Description")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            transcript: inst
                .get_str("Transcript")
                .map(LocaleKey::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `JournalEntryDialogueLog`
/// Inherits from: `BaseJournalEntry`
pub struct JournalEntryDialogueLog {
    /// `Dialogue` (Reference (array))
    pub dialogue: Vec<CigGuid>,
    /// `Description` (Locale)
    pub description: LocaleKey,
}

impl Pooled for JournalEntryDialogueLog {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.journalentry.journal_entry_dialogue_log
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.journalentry.journal_entry_dialogue_log
    }
}

impl<'a> Extract<'a> for JournalEntryDialogueLog {
    const TYPE_NAME: &'static str = "JournalEntryDialogueLog";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            dialogue: inst
                .get_array("Dialogue")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            description: inst
                .get_str("Description")
                .map(LocaleKey::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `JournalEntryVideo`
/// Inherits from: `BaseJournalEntry`
pub struct JournalEntryVideo {
    /// `videoDef` (Reference)
    pub video_def: Option<CigGuid>,
    /// `description` (Locale)
    pub description: LocaleKey,
}

impl Pooled for JournalEntryVideo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.journalentry.journal_entry_video
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.journalentry.journal_entry_video
    }
}

impl<'a> Extract<'a> for JournalEntryVideo {
    const TYPE_NAME: &'static str = "JournalEntryVideo";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            video_def: inst
                .get("videoDef")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            description: inst
                .get_str("description")
                .map(LocaleKey::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationStandingJournalEntryParams`
pub struct SReputationStandingJournalEntryParams {
    /// `changeDirection` (EnumChoice)
    pub change_direction: EReputationChangeReason,
    /// `standing` (Reference)
    pub standing: Option<CigGuid>,
    /// `journalEntry` (Reference)
    pub journal_entry: Option<CigGuid>,
}

impl Pooled for SReputationStandingJournalEntryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.journalentry.sreputation_standing_journal_entry_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.journalentry.sreputation_standing_journal_entry_params
    }
}

impl<'a> Extract<'a> for SReputationStandingJournalEntryParams {
    const TYPE_NAME: &'static str = "SReputationStandingJournalEntryParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            change_direction: EReputationChangeReason::from_dcb_str(
                inst.get_str("changeDirection").unwrap_or(""),
            ),
            standing: inst
                .get("standing")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            journal_entry: inst
                .get("journalEntry")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `SReputationJournalGroupParams`
pub struct SReputationJournalGroupParams {
    /// `standingEntries` (Class (array))
    pub standing_entries: Vec<Handle<SReputationStandingJournalEntryParams>>,
}

impl Pooled for SReputationJournalGroupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.journalentry.sreputation_journal_group_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.journalentry.sreputation_journal_group_params
    }
}

impl<'a> Extract<'a> for SReputationJournalGroupParams {
    const TYPE_NAME: &'static str = "SReputationJournalGroupParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            standing_entries: inst
                .get_array("standingEntries")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SReputationStandingJournalEntryParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<SReputationStandingJournalEntryParams>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationJournalEntriesParams`
pub struct SReputationJournalEntriesParams {
    /// `factionReputation` (Reference)
    pub faction_reputation: Option<CigGuid>,
    /// `reputationScope` (Reference)
    pub reputation_scope: Option<CigGuid>,
    /// `journalGroups` (Class (array))
    pub journal_groups: Vec<Handle<SReputationJournalGroupParams>>,
}

impl Pooled for SReputationJournalEntriesParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.journalentry.sreputation_journal_entries_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.journalentry.sreputation_journal_entries_params
    }
}

impl<'a> Extract<'a> for SReputationJournalEntriesParams {
    const TYPE_NAME: &'static str = "SReputationJournalEntriesParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            faction_reputation: inst
                .get("factionReputation")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            reputation_scope: inst
                .get("reputationScope")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            journal_groups: inst
                .get_array("journalGroups")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SReputationJournalGroupParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<SReputationJournalGroupParams>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SReputationJournalEntryHandlerParams`
pub struct SReputationJournalEntryHandlerParams {
    /// `reputationTypes` (Class (array))
    pub reputation_types: Vec<Handle<SReputationJournalEntriesParams>>,
}

impl Pooled for SReputationJournalEntryHandlerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.journalentry.sreputation_journal_entry_handler_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.journalentry.sreputation_journal_entry_handler_params
    }
}

impl<'a> Extract<'a> for SReputationJournalEntryHandlerParams {
    const TYPE_NAME: &'static str = "SReputationJournalEntryHandlerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reputation_types: inst
                .get_array("reputationTypes")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SReputationJournalEntriesParams>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<SReputationJournalEntriesParams>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
