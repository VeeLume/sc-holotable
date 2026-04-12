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

/// Pool storage for the `actor-lookahead` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorLookaheadPools {
    #[serde(default)]
    pub actor_look_ahead_point: Vec<Option<ActorLookAheadPoint>>,
    #[serde(default)]
    pub actor_look_ahead_roll: Vec<Option<ActorLookAheadRoll>>,
    #[serde(default)]
    pub actor_look_ahead_target_tracking: Vec<Option<ActorLookAheadTargetTracking>>,
    #[serde(default)]
    pub actor_look_ahead_vehicle: Vec<Option<ActorLookAheadVehicle>>,
}
