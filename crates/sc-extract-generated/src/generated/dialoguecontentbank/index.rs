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

/// Record index for the `dialoguecontentbank` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DialoguecontentbankIndex {
    #[serde(default)]
    pub dialogue_external_source: HashMap<CigGuid, Handle<DialogueExternalSource>>,
    #[serde(default)]
    pub dialogue_content: HashMap<CigGuid, Handle<DialogueContent>>,
    #[serde(default)]
    pub dialogue_content_bank: HashMap<CigGuid, Handle<DialogueContentBank>>,
}

impl DialoguecontentbankIndex {
    pub fn len(&self) -> usize {
        self.dialogue_external_source.len()
            + self.dialogue_content.len()
            + self.dialogue_content_bank.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
