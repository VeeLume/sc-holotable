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

/// Pool storage for the `creatures` feature.
#[derive(Default)]
pub struct CreaturesPools {
    pub boids_group_composition: Vec<Option<BoidsGroupComposition>>,
    pub boid_transition: Vec<Option<BoidTransition>>,
    pub boid_state: Vec<Option<BoidState>>,
    pub boid_random_transition: Vec<Option<BoidRandomTransition>>,
    pub boid_alerted_transition: Vec<Option<BoidAlertedTransition>>,
    pub boid_actor_proximity_transition: Vec<Option<BoidActorProximityTransition>>,
    pub boids_behavior_rule: Vec<Option<BoidsBehaviorRule>>,
    pub boids_alignment_rule: Vec<Option<BoidsAlignmentRule>>,
    pub boids_separation_rule: Vec<Option<BoidsSeparationRule>>,
    pub boids_cohesion_rule: Vec<Option<BoidsCohesionRule>>,
    pub boids_spherical_limiter_rule: Vec<Option<BoidsSphericalLimiterRule>>,
    pub boids_ocean_surface_repel_rule: Vec<Option<BoidsOceanSurfaceRepelRule>>,
    pub boids_terrain_surface_repel_rule: Vec<Option<BoidsTerrainSurfaceRepelRule>>,
    pub boids_navmesh_edge_repel_rule: Vec<Option<BoidsNavmeshEdgeRepelRule>>,
    pub boids_alert_point_repel_rule: Vec<Option<BoidsAlertPointRepelRule>>,
    pub boids_behavior_rule_container: Vec<Option<BoidsBehaviorRuleContainer>>,
    pub boids_actor_repel_rule: Vec<Option<BoidsActorRepelRule>>,
    pub boids_vehicle_repel_rule: Vec<Option<BoidsVehicleRepelRule>>,
    pub boids_component_params: Vec<Option<BoidsComponentParams>>,
    pub boid_agent_component_params: Vec<Option<BoidAgentComponentParams>>,
}
