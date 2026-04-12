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

/// Pool storage for the `actor-actorviewlimits` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorActorviewlimitsPools {
    #[serde(default)]
    pub actor_aim_limits_state_filter: Vec<Option<ActorAimLimitsStateFilter>>,
    #[serde(default)]
    pub actor_look_limits_state_filter: Vec<Option<ActorLookLimitsStateFilter>>,
    #[serde(default)]
    pub actor_look_limits: Vec<Option<ActorLookLimits>>,
    #[serde(default)]
    pub actor_aim_limits: Vec<Option<ActorAimLimits>>,
}
