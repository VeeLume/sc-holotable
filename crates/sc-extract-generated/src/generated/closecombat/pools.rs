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

/// Pool storage for the `closecombat` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClosecombatPools {
    #[serde(default)]
    pub actor_state_skeleton_filter: Vec<Option<ActorStateSkeletonFilter>>,
    #[serde(default)]
    pub actor_restrain_per_attacker_config: Vec<Option<ActorRestrainPerAttackerConfig>>,
    #[serde(default)]
    pub actor_restrain_config: Vec<Option<ActorRestrainConfig>>,
    #[serde(default)]
    pub take_down_max_distances: Vec<Option<TakeDownMaxDistances>>,
    #[serde(default)]
    pub take_down_params: Vec<Option<TakeDownParams>>,
    #[serde(default)]
    pub take_down_config: Vec<Option<TakeDownConfig>>,
}
