// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `journalentry` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JournalentryPools {
    #[serde(default)]
    pub base_journal_entry: Vec<Option<BaseJournalEntry>>,
    #[serde(default)]
    pub journal_entry: Vec<Option<JournalEntry>>,
    #[serde(default)]
    pub sreputation_standing_journal_entry_params: Vec<Option<SReputationStandingJournalEntryParams>>,
    #[serde(default)]
    pub sreputation_journal_group_params: Vec<Option<SReputationJournalGroupParams>>,
    #[serde(default)]
    pub sreputation_journal_entries_params: Vec<Option<SReputationJournalEntriesParams>>,
    #[serde(default)]
    pub sreputation_journal_entry_handler_params: Vec<Option<SReputationJournalEntryHandlerParams>>,
}
