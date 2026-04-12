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

/// Pool storage for the `actor-stanceinfo` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorStanceinfoPools {
    #[serde(default)]
    pub actor_stance_speeds: Vec<Option<ActorStanceSpeeds>>,
    #[serde(default)]
    pub actor_stance_dimensions: Vec<Option<ActorStanceDimensions>>,
    #[serde(default)]
    pub actor_stance_speeds_info: Vec<Option<ActorStanceSpeedsInfo>>,
    #[serde(default)]
    pub actor_stance_dimensions_info: Vec<Option<ActorStanceDimensionsInfo>>,
    #[serde(default)]
    pub sactor_stance_dimensions_extra_def: Vec<Option<SActorStanceDimensionsExtraDef>>,
}
