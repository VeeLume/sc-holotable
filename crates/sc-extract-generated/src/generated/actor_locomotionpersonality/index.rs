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

/// Record index for the `actor-locomotionpersonality` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorLocomotionpersonalityIndex {
    #[serde(default)]
    pub sactor_locomotion_fidget_state_filtered_def: HashMap<CigGuid, Handle<SActorLocomotionFidgetStateFilteredDef>>,
    #[serde(default)]
    pub sactor_locomotion_fidget_def: HashMap<CigGuid, Handle<SActorLocomotionFidgetDef>>,
    #[serde(default)]
    pub actor_locomotion_personality: HashMap<CigGuid, Handle<ActorLocomotionPersonality>>,
}

impl ActorLocomotionpersonalityIndex {
    pub fn len(&self) -> usize {
        self.sactor_locomotion_fidget_state_filtered_def.len()
            + self.sactor_locomotion_fidget_def.len()
            + self.actor_locomotion_personality.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
