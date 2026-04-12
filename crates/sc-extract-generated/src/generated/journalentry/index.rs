// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `journalentry` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JournalentryIndex {
    #[serde(default)]
    pub journal_entry: HashMap<CigGuid, Handle<JournalEntry>>,
    #[serde(default)]
    pub sreputation_journal_entry_handler_params: HashMap<CigGuid, Handle<SReputationJournalEntryHandlerParams>>,
}

impl JournalentryIndex {
    pub fn len(&self) -> usize {
        self.journal_entry.len()
            + self.sreputation_journal_entry_handler_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
