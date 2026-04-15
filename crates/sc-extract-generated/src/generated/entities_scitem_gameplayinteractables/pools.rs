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

/// Pool storage for the `entities-scitem-gameplayinteractables` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemGameplayinteractablesPools {
    #[serde(default)]
    pub spawner_prerequisite_or: Vec<Option<SpawnerPrerequisite_OR>>,
    #[serde(default)]
    pub sweighted_reward_entry: Vec<Option<SWeightedRewardEntry>>,
    #[serde(default)]
    pub sreward_generator_component_params: Vec<Option<SRewardGeneratorComponentParams>>,
    #[serde(default)]
    pub sspawner_analytics_event_gameplay_trigger: Vec<Option<SSpawnerAnalyticsEventGameplayTrigger>>,
    #[serde(default)]
    pub sdissolve_self_gameplay_trigger: Vec<Option<SDissolveSelfGameplayTrigger>>,
    #[serde(default)]
    pub self_interaction_trigger: Vec<Option<SelfInteractionTrigger>>,
    #[serde(default)]
    pub gameplay_trigger_physics_set_parameter_proxy_state: Vec<Option<GameplayTrigger_Physics_SetParameter_ProxyState>>,
    #[serde(default)]
    pub physics_set_parameter_gameplay_trigger: Vec<Option<PhysicsSetParameterGameplayTrigger>>,
}
