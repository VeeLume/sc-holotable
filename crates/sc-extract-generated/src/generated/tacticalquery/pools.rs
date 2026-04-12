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

/// Pool storage for the `tacticalquery` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TacticalqueryPools {
    #[serde(default)]
    pub tactical_query: Vec<Option<TacticalQuery>>,
    #[serde(default)]
    pub tqsinput: Vec<Option<TQSInput>>,
    #[serde(default)]
    pub tqsweight_input: Vec<Option<TQSWeightInput>>,
    #[serde(default)]
    pub tqsoption: Vec<Option<TQSOption>>,
    #[serde(default)]
    pub tqsoption_content_record: Vec<Option<TQSOptionContentRecord>>,
    #[serde(default)]
    pub tqsoption_content: Vec<Option<TQSOptionContent>>,
}
