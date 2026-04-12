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

/// Pool storage for the `entities-scitem-operatormoderecords` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemOperatormoderecordsPools {
    #[serde(default)]
    pub operator_mode_availability: Vec<Option<OperatorModeAvailability>>,
    #[serde(default)]
    pub operator_mode_availability_params: Vec<Option<OperatorModeAvailabilityParams>>,
    #[serde(default)]
    pub operator_mode_definitions: Vec<Option<OperatorModeDefinitions>>,
    #[serde(default)]
    pub operator_mode_definition_params: Vec<Option<OperatorModeDefinitionParams>>,
}
