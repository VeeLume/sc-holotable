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

/// Record index for the `actor-actorviewlimits` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorActorviewlimitsIndex {
    #[serde(default)]
    pub actor_look_limits: HashMap<CigGuid, Handle<ActorLookLimits>>,
    #[serde(default)]
    pub actor_aim_limits: HashMap<CigGuid, Handle<ActorAimLimits>>,
}

impl ActorActorviewlimitsIndex {
    pub fn len(&self) -> usize {
        self.actor_look_limits.len()
            + self.actor_aim_limits.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
