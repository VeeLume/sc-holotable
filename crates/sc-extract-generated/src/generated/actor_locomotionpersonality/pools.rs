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

/// Pool storage for the `actor-locomotionpersonality` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorLocomotionpersonalityPools {
    #[serde(default)]
    pub slocomotion_personality_state_filter: Vec<Option<SLocomotionPersonalityStateFilter>>,
    #[serde(default)]
    pub actor_locomotion_rotate_params: Vec<Option<ActorLocomotionRotateParams>>,
    #[serde(default)]
    pub actor_locomotion_turn_on_spot_params: Vec<Option<ActorLocomotionTurnOnSpotParams>>,
    #[serde(default)]
    pub actor_locomotion_sharp_turn_params: Vec<Option<ActorLocomotionSharpTurnParams>>,
    #[serde(default)]
    pub actor_locomotion_avoidance_params: Vec<Option<ActorLocomotionAvoidanceParams>>,
    #[serde(default)]
    pub sactor_locomotion_fidget_severity_params: Vec<Option<SActorLocomotionFidgetSeverityParams>>,
    #[serde(default)]
    pub sactor_locomotion_fidget_state_filtered_def: Vec<Option<SActorLocomotionFidgetStateFilteredDef>>,
    #[serde(default)]
    pub sactor_locomotion_fidget_def: Vec<Option<SActorLocomotionFidgetDef>>,
    #[serde(default)]
    pub sactor_locomotion_feature_def_slope: Vec<Option<SActorLocomotionFeatureDef_Slope>>,
    #[serde(default)]
    pub sactor_locomotion_submerged_creature_params: Vec<Option<SActorLocomotionSubmergedCreatureParams>>,
    #[serde(default)]
    pub actor_locomotion_personality: Vec<Option<ActorLocomotionPersonality>>,
}
