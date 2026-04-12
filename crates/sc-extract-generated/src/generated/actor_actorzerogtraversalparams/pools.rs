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

/// Pool storage for the `actor-actorzerogtraversalparams` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorActorzerogtraversalparamsPools {
    #[serde(default)]
    pub sczero_glaunch_params: Vec<Option<SCZeroGLaunchParams>>,
    #[serde(default)]
    pub scdefault_zero_gtraversal_params: Vec<Option<SCDefaultZeroGTraversalParams>>,
    #[serde(default)]
    pub scoptional_zero_gtraversal_params: Vec<Option<SCOptionalZeroGTraversalParams>>,
    #[serde(default)]
    pub actor_zero_gtraversal_params: Vec<Option<ActorZeroGTraversalParams>>,
}
