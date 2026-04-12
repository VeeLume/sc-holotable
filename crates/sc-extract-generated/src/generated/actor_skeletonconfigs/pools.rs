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

/// Pool storage for the `actor-skeletonconfigs` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorSkeletonconfigsPools {
    #[serde(default)]
    pub ragdoll_recovery_config: Vec<Option<RagdollRecoveryConfig>>,
    #[serde(default)]
    pub locomotion_anim_sync_config: Vec<Option<LocomotionAnimSyncConfig>>,
    #[serde(default)]
    pub actor_foot_joint_pair_def: Vec<Option<ActorFootJointPairDef>>,
    #[serde(default)]
    pub actor_melee_def: Vec<Option<ActorMeleeDef>>,
    #[serde(default)]
    pub actor_skeleton_config: Vec<Option<ActorSkeletonConfig>>,
    #[serde(default)]
    pub ang_ypr: Vec<Option<AngYPR>>,
}
