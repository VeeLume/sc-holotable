// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `reputation` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReputationIndex {
    #[serde(default)]
    pub sreputation_state_params: HashMap<CigGuid, Handle<SReputationStateParams>>,
    #[serde(default)]
    pub sreputation_state_mission_result_modifier_params: HashMap<CigGuid, Handle<SReputationStateMissionResultModifierParams>>,
}

impl ReputationIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.sreputation_state_params.len();
        total += self.sreputation_state_mission_result_modifier_params.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
