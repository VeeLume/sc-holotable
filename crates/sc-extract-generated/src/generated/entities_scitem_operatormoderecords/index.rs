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

/// Record index for the `entities-scitem-operatormoderecords` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemOperatormoderecordsIndex {
    #[serde(default)]
    pub operator_mode_availability_params: HashMap<CigGuid, Handle<OperatorModeAvailabilityParams>>,
    #[serde(default)]
    pub operator_mode_definition_params: HashMap<CigGuid, Handle<OperatorModeDefinitionParams>>,
}

impl EntitiesScitemOperatormoderecordsIndex {
    pub fn len(&self) -> usize {
        self.operator_mode_availability_params.len()
            + self.operator_mode_definition_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
