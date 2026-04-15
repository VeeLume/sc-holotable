// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `actor-actors` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActorActorsPools {
    #[serde(default)]
    pub sdynamic_group_component_params: Vec<Option<SDynamicGroupComponentParams>>,
    #[serde(default)]
    pub activity_behavior_component_params: Vec<Option<ActivityBehaviorComponentParams>>,
    #[serde(default)]
    pub terrain_trash_cleanup_component_params: Vec<Option<TerrainTrashCleanupComponentParams>>,
    #[serde(default)]
    pub sactor_death_pose_variant: Vec<Option<SActorDeathPoseVariant>>,
    #[serde(default)]
    pub sactor_static_collider_death_behaviour: Vec<Option<SActorStaticColliderDeathBehaviour>>,
    #[serde(default)]
    pub sdummy_player_component_params: Vec<Option<SDummyPlayerComponentParams>>,
    #[serde(default)]
    pub terrain_trash_cleanup_gameplay_trigger: Vec<Option<TerrainTrashCleanupGameplayTrigger>>,
    #[serde(default)]
    pub underground_creature_helpers_component: Vec<Option<UndergroundCreatureHelpersComponent>>,
}
