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

/// Record index for the `actor-stanceinfo` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorStanceinfoIndex {
    #[serde(default)]
    pub actor_stance_speeds_info: HashMap<CigGuid, Handle<ActorStanceSpeedsInfo>>,
    #[serde(default)]
    pub actor_stance_dimensions_info: HashMap<CigGuid, Handle<ActorStanceDimensionsInfo>>,
}

impl ActorStanceinfoIndex {
    pub fn len(&self) -> usize {
        self.actor_stance_speeds_info.len()
            + self.actor_stance_dimensions_info.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
