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

/// Record index for the `scarryableikinteractionlist` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScarryableikinteractionlistIndex {
    #[serde(default)]
    pub scarryable_ikinteraction_list: HashMap<CigGuid, Handle<SCarryableIKInteractionList>>,
}

impl ScarryableikinteractionlistIndex {
    pub fn len(&self) -> usize {
        self.scarryable_ikinteraction_list.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
