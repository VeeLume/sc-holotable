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

/// Record index for the `actor-externalforceresponse` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorExternalforceresponseIndex {
    #[serde(default)]
    pub sactor_force_reactions_def: HashMap<CigGuid, Handle<SActorForceReactionsDef>>,
    #[serde(default)]
    pub sactor_hit_reactions_def: HashMap<CigGuid, Handle<SActorHitReactionsDef>>,
    #[serde(default)]
    pub sactor_external_force_response_camera_shake_def: HashMap<CigGuid, Handle<SActorExternalForceResponseCameraShakeDef>>,
    #[serde(default)]
    pub sactor_force_reactions_preset_record: HashMap<CigGuid, Handle<SActorForceReactionsPresetRecord>>,
}

impl ActorExternalforceresponseIndex {
    pub fn len(&self) -> usize {
        self.sactor_force_reactions_def.len()
            + self.sactor_hit_reactions_def.len()
            + self.sactor_external_force_response_camera_shake_def.len()
            + self.sactor_force_reactions_preset_record.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
