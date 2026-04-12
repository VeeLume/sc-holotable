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

/// Pool storage for the `contextualcommunicationconfig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContextualcommunicationconfigPools {
    #[serde(default)]
    pub communication_variable_base: Vec<Option<CommunicationVariableBase>>,
    #[serde(default)]
    pub contextual_communication_config: Vec<Option<ContextualCommunicationConfig>>,
    #[serde(default)]
    pub contextual_communication_response: Vec<Option<ContextualCommunicationResponse>>,
    #[serde(default)]
    pub communication_request: Vec<Option<CommunicationRequest>>,
    #[serde(default)]
    pub contextual_communication_condition: Vec<Option<ContextualCommunicationCondition>>,
}
