// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `actor-actors` feature.
#[derive(Default)]
pub struct ActorActorsPools {
    pub sdynamic_group_component_params: Vec<Option<SDynamicGroupComponentParams>>,
    pub activity_behavior_component_params: Vec<Option<ActivityBehaviorComponentParams>>,
    pub terrain_trash_cleanup_component_params: Vec<Option<TerrainTrashCleanupComponentParams>>,
    pub sactor_death_pose_variant: Vec<Option<SActorDeathPoseVariant>>,
    pub sactor_static_collider_death_behaviour: Vec<Option<SActorStaticColliderDeathBehaviour>>,
    pub sdummy_player_component_params: Vec<Option<SDummyPlayerComponentParams>>,
    pub terrain_trash_cleanup_gameplay_trigger: Vec<Option<TerrainTrashCleanupGameplayTrigger>>,
    pub underground_creature_helpers_component: Vec<Option<UndergroundCreatureHelpersComponent>>,
}
