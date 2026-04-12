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

/// Pool storage for the `communicationconfig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunicationconfigPools {
    #[serde(default)]
    pub communication_config: Vec<Option<CommunicationConfig>>,
    #[serde(default)]
    pub communication_entry: Vec<Option<CommunicationEntry>>,
    #[serde(default)]
    pub communication_variation: Vec<Option<CommunicationVariation>>,
    #[serde(default)]
    pub communication_variation_rules: Vec<Option<CommunicationVariationRules>>,
    #[serde(default)]
    pub communication_variation_condition: Vec<Option<CommunicationVariationCondition>>,
}
