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

/// Pool storage for the `actor-playeranimatedinteractionconfig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorPlayeranimatedinteractionconfigPools {
    #[serde(default)]
    pub walk_to_align_params: Vec<Option<WalkToAlignParams>>,
    #[serde(default)]
    pub animated_action: Vec<Option<AnimatedAction>>,
    #[serde(default)]
    pub player_animated_interaction_walking_request_params: Vec<Option<PlayerAnimatedInteractionWalkingRequestParams>>,
    #[serde(default)]
    pub player_animated_interaction_filtered: Vec<Option<PlayerAnimatedInteractionFiltered>>,
    #[serde(default)]
    pub player_animated_interaction_config: Vec<Option<PlayerAnimatedInteractionConfig>>,
}
