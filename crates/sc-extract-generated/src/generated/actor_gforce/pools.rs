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

/// Pool storage for the `actor-gforce` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorGforcePools {
    #[serde(default)]
    pub actor_gforce_head_bob_fake_velocity_gdata: Vec<Option<ActorGForceHeadBobFakeVelocityGData>>,
    #[serde(default)]
    pub actor_gforce_head_bob_data: Vec<Option<ActorGForceHeadBobData>>,
    #[serde(default)]
    pub actor_gforce_head_bob: Vec<Option<ActorGForceHeadBob>>,
    #[serde(default)]
    pub actor_gforce_camera_effects_data: Vec<Option<ActorGForceCameraEffectsData>>,
    #[serde(default)]
    pub actor_gforce_camera_effects: Vec<Option<ActorGForceCameraEffects>>,
}
