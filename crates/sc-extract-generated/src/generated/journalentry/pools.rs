// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `journalentry` feature.
#[derive(Default)]
pub struct JournalentryPools {
    pub journal_entry_audio_log: Vec<Option<JournalEntryAudioLog>>,
    pub journal_entry_dialogue_log: Vec<Option<JournalEntryDialogueLog>>,
    pub journal_entry_video: Vec<Option<JournalEntryVideo>>,
    pub sreputation_standing_journal_entry_params: Vec<Option<SReputationStandingJournalEntryParams>>,
    pub sreputation_journal_group_params: Vec<Option<SReputationJournalGroupParams>>,
    pub sreputation_journal_entries_params: Vec<Option<SReputationJournalEntriesParams>>,
    pub sreputation_journal_entry_handler_params: Vec<Option<SReputationJournalEntryHandlerParams>>,
}
