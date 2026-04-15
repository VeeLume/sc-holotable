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

/// Pool storage for the `contextualcommunicationconfig` feature.
#[derive(Default)]
pub struct ContextualcommunicationconfigPools {
    pub communication_variable_base: Vec<Option<CommunicationVariableBase>>,
    pub contextual_communication_config: Vec<Option<ContextualCommunicationConfig>>,
    pub contextual_communication_response: Vec<Option<ContextualCommunicationResponse>>,
    pub communication_request: Vec<Option<CommunicationRequest>>,
    pub contextual_communication_condition: Vec<Option<ContextualCommunicationCondition>>,
}
