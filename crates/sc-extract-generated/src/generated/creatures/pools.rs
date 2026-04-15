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

/// Pool storage for the `creatures` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreaturesPools {
    #[serde(default)]
    pub boids_group_composition: Vec<Option<BoidsGroupComposition>>,
    #[serde(default)]
    pub boid_transition: Vec<Option<BoidTransition>>,
    #[serde(default)]
    pub boid_state: Vec<Option<BoidState>>,
    #[serde(default)]
    pub boid_random_transition: Vec<Option<BoidRandomTransition>>,
    #[serde(default)]
    pub boid_alerted_transition: Vec<Option<BoidAlertedTransition>>,
    #[serde(default)]
    pub boid_actor_proximity_transition: Vec<Option<BoidActorProximityTransition>>,
    #[serde(default)]
    pub boids_behavior_rule: Vec<Option<BoidsBehaviorRule>>,
    #[serde(default)]
    pub boids_alignment_rule: Vec<Option<BoidsAlignmentRule>>,
    #[serde(default)]
    pub boids_separation_rule: Vec<Option<BoidsSeparationRule>>,
    #[serde(default)]
    pub boids_cohesion_rule: Vec<Option<BoidsCohesionRule>>,
    #[serde(default)]
    pub boids_spherical_limiter_rule: Vec<Option<BoidsSphericalLimiterRule>>,
    #[serde(default)]
    pub boids_ocean_surface_repel_rule: Vec<Option<BoidsOceanSurfaceRepelRule>>,
    #[serde(default)]
    pub boids_terrain_surface_repel_rule: Vec<Option<BoidsTerrainSurfaceRepelRule>>,
    #[serde(default)]
    pub boids_navmesh_edge_repel_rule: Vec<Option<BoidsNavmeshEdgeRepelRule>>,
    #[serde(default)]
    pub boids_alert_point_repel_rule: Vec<Option<BoidsAlertPointRepelRule>>,
    #[serde(default)]
    pub boids_behavior_rule_container: Vec<Option<BoidsBehaviorRuleContainer>>,
    #[serde(default)]
    pub boids_actor_repel_rule: Vec<Option<BoidsActorRepelRule>>,
    #[serde(default)]
    pub boids_vehicle_repel_rule: Vec<Option<BoidsVehicleRepelRule>>,
    #[serde(default)]
    pub boids_component_params: Vec<Option<BoidsComponentParams>>,
    #[serde(default)]
    pub boid_agent_component_params: Vec<Option<BoidAgentComponentParams>>,
}
