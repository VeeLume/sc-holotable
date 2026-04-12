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

/// Pool storage for the `dialoguecontentbank` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DialoguecontentbankPools {
    #[serde(default)]
    pub dialogue_external_source: Vec<Option<DialogueExternalSource>>,
    #[serde(default)]
    pub dialogue_content: Vec<Option<DialogueContent>>,
    #[serde(default)]
    pub dialogue_content_bank: Vec<Option<DialogueContentBank>>,
}
