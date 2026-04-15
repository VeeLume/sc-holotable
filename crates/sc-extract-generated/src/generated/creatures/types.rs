// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `creatures`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `BoidsGroupComposition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsGroupComposition {
    /// `boidsEntityClasses` (Reference (array))
    #[serde(default)]
    pub boids_entity_classes: Vec<CigGuid>,
    /// `amountOfEntities` (Int32)
    #[serde(default)]
    pub amount_of_entities: i32,
    /// `amountOfEntitiesVariation` (Int32)
    #[serde(default)]
    pub amount_of_entities_variation: i32,
    /// `sizeVariation` (Single)
    #[serde(default)]
    pub size_variation: f32,
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `spawnOnNavmesh` (Boolean)
    #[serde(default)]
    pub spawn_on_navmesh: bool,
}

impl Pooled for BoidsGroupComposition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_group_composition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_group_composition }
}

impl<'a> Extract<'a> for BoidsGroupComposition {
    const TYPE_NAME: &'static str = "BoidsGroupComposition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            boids_entity_classes: inst.get_array("boidsEntityClasses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            amount_of_entities: inst.get_i32("amountOfEntities").unwrap_or_default(),
            amount_of_entities_variation: inst.get_i32("amountOfEntitiesVariation").unwrap_or_default(),
            size_variation: inst.get_f32("sizeVariation").unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            spawn_on_navmesh: inst.get_bool("spawnOnNavmesh").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidTransition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidTransition {
    /// `animationTag` (String)
    #[serde(default)]
    pub animation_tag: String,
    /// `fragmentTag` (String)
    #[serde(default)]
    pub fragment_tag: String,
    /// `transition` (WeakPointer)
    #[serde(default)]
    pub transition: Option<Handle<BoidState>>,
    /// `instantTransition` (Boolean)
    #[serde(default)]
    pub instant_transition: bool,
    /// `allowWhileTransitioning` (Boolean)
    #[serde(default)]
    pub allow_while_transitioning: bool,
}

impl Pooled for BoidTransition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boid_transition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boid_transition }
}

impl<'a> Extract<'a> for BoidTransition {
    const TYPE_NAME: &'static str = "BoidTransition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            animation_tag: inst.get_str("animationTag").map(String::from).unwrap_or_default(),
            fragment_tag: inst.get_str("fragmentTag").map(String::from).unwrap_or_default(),
            transition: match inst.get("transition") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoidState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            instant_transition: inst.get_bool("instantTransition").unwrap_or_default(),
            allow_while_transitioning: inst.get_bool("allowWhileTransitioning").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidState`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidState {
    /// `stateName` (String)
    #[serde(default)]
    pub state_name: String,
    /// `rules` (Class (array))
    #[serde(default)]
    pub rules: Vec<Handle<BoidsBehaviorRuleContainer>>,
    /// `shouldBeOnNavmesh` (Boolean)
    #[serde(default)]
    pub should_be_on_navmesh: bool,
    /// `deterministic` (Boolean)
    #[serde(default)]
    pub deterministic: bool,
    /// `boidTransitions` (StrongPointer (array))
    #[serde(default)]
    pub boid_transitions: Vec<BoidTransitionPtr>,
    /// `animationTag` (String)
    #[serde(default)]
    pub animation_tag: String,
    /// `fragmentTag` (String)
    #[serde(default)]
    pub fragment_tag: String,
    /// `maxLinearSpeed` (Single)
    #[serde(default)]
    pub max_linear_speed: f32,
}

impl Pooled for BoidState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boid_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boid_state }
}

impl<'a> Extract<'a> for BoidState {
    const TYPE_NAME: &'static str = "BoidState";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_name: inst.get_str("stateName").map(String::from).unwrap_or_default(),
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BoidsBehaviorRuleContainer>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<BoidsBehaviorRuleContainer>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            should_be_on_navmesh: inst.get_bool("shouldBeOnNavmesh").unwrap_or_default(),
            deterministic: inst.get_bool("deterministic").unwrap_or_default(),
            boid_transitions: inst.get_array("boidTransitions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(BoidTransitionPtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            animation_tag: inst.get_str("animationTag").map(String::from).unwrap_or_default(),
            fragment_tag: inst.get_str("fragmentTag").map(String::from).unwrap_or_default(),
            max_linear_speed: inst.get_f32("maxLinearSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidRandomTransition`
/// Inherits from: `BoidTransition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidRandomTransition {
    /// `animationTag` (String)
    #[serde(default)]
    pub animation_tag: String,
    /// `fragmentTag` (String)
    #[serde(default)]
    pub fragment_tag: String,
    /// `transition` (WeakPointer)
    #[serde(default)]
    pub transition: Option<Handle<BoidState>>,
    /// `instantTransition` (Boolean)
    #[serde(default)]
    pub instant_transition: bool,
    /// `allowWhileTransitioning` (Boolean)
    #[serde(default)]
    pub allow_while_transitioning: bool,
    /// `interval` (Single)
    #[serde(default)]
    pub interval: f32,
    /// `chance` (Single)
    #[serde(default)]
    pub chance: f32,
}

impl Pooled for BoidRandomTransition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boid_random_transition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boid_random_transition }
}

impl<'a> Extract<'a> for BoidRandomTransition {
    const TYPE_NAME: &'static str = "BoidRandomTransition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            animation_tag: inst.get_str("animationTag").map(String::from).unwrap_or_default(),
            fragment_tag: inst.get_str("fragmentTag").map(String::from).unwrap_or_default(),
            transition: match inst.get("transition") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoidState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            instant_transition: inst.get_bool("instantTransition").unwrap_or_default(),
            allow_while_transitioning: inst.get_bool("allowWhileTransitioning").unwrap_or_default(),
            interval: inst.get_f32("interval").unwrap_or_default(),
            chance: inst.get_f32("chance").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidAlertedTransition`
/// Inherits from: `BoidTransition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidAlertedTransition {
    /// `animationTag` (String)
    #[serde(default)]
    pub animation_tag: String,
    /// `fragmentTag` (String)
    #[serde(default)]
    pub fragment_tag: String,
    /// `transition` (WeakPointer)
    #[serde(default)]
    pub transition: Option<Handle<BoidState>>,
    /// `instantTransition` (Boolean)
    #[serde(default)]
    pub instant_transition: bool,
    /// `allowWhileTransitioning` (Boolean)
    #[serde(default)]
    pub allow_while_transitioning: bool,
}

impl Pooled for BoidAlertedTransition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boid_alerted_transition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boid_alerted_transition }
}

impl<'a> Extract<'a> for BoidAlertedTransition {
    const TYPE_NAME: &'static str = "BoidAlertedTransition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            animation_tag: inst.get_str("animationTag").map(String::from).unwrap_or_default(),
            fragment_tag: inst.get_str("fragmentTag").map(String::from).unwrap_or_default(),
            transition: match inst.get("transition") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoidState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            instant_transition: inst.get_bool("instantTransition").unwrap_or_default(),
            allow_while_transitioning: inst.get_bool("allowWhileTransitioning").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidActorProximityTransition`
/// Inherits from: `BoidTransition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidActorProximityTransition {
    /// `animationTag` (String)
    #[serde(default)]
    pub animation_tag: String,
    /// `fragmentTag` (String)
    #[serde(default)]
    pub fragment_tag: String,
    /// `transition` (WeakPointer)
    #[serde(default)]
    pub transition: Option<Handle<BoidState>>,
    /// `instantTransition` (Boolean)
    #[serde(default)]
    pub instant_transition: bool,
    /// `allowWhileTransitioning` (Boolean)
    #[serde(default)]
    pub allow_while_transitioning: bool,
    /// `distance` (Single)
    #[serde(default)]
    pub distance: f32,
}

impl Pooled for BoidActorProximityTransition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boid_actor_proximity_transition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boid_actor_proximity_transition }
}

impl<'a> Extract<'a> for BoidActorProximityTransition {
    const TYPE_NAME: &'static str = "BoidActorProximityTransition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            animation_tag: inst.get_str("animationTag").map(String::from).unwrap_or_default(),
            fragment_tag: inst.get_str("fragmentTag").map(String::from).unwrap_or_default(),
            transition: match inst.get("transition") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoidState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            instant_transition: inst.get_bool("instantTransition").unwrap_or_default(),
            allow_while_transitioning: inst.get_bool("allowWhileTransitioning").unwrap_or_default(),
            distance: inst.get_f32("distance").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsBehaviorRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
}

impl Pooled for BoidsBehaviorRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_behavior_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_behavior_rule }
}

impl<'a> Extract<'a> for BoidsBehaviorRule {
    const TYPE_NAME: &'static str = "BoidsBehaviorRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsAlignmentRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsAlignmentRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
}

impl Pooled for BoidsAlignmentRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_alignment_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_alignment_rule }
}

impl<'a> Extract<'a> for BoidsAlignmentRule {
    const TYPE_NAME: &'static str = "BoidsAlignmentRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsSeparationRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsSeparationRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
    /// `decayCoefficient` (Single)
    #[serde(default)]
    pub decay_coefficient: f32,
    /// `maxAcceleration` (Single)
    #[serde(default)]
    pub max_acceleration: f32,
}

impl Pooled for BoidsSeparationRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_separation_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_separation_rule }
}

impl<'a> Extract<'a> for BoidsSeparationRule {
    const TYPE_NAME: &'static str = "BoidsSeparationRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
            decay_coefficient: inst.get_f32("decayCoefficient").unwrap_or_default(),
            max_acceleration: inst.get_f32("maxAcceleration").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsCohesionRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsCohesionRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
}

impl Pooled for BoidsCohesionRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_cohesion_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_cohesion_rule }
}

impl<'a> Extract<'a> for BoidsCohesionRule {
    const TYPE_NAME: &'static str = "BoidsCohesionRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsSphericalLimiterRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsSphericalLimiterRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
    /// `effectDistance` (Single)
    #[serde(default)]
    pub effect_distance: f32,
}

impl Pooled for BoidsSphericalLimiterRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_spherical_limiter_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_spherical_limiter_rule }
}

impl<'a> Extract<'a> for BoidsSphericalLimiterRule {
    const TYPE_NAME: &'static str = "BoidsSphericalLimiterRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
            radius: inst.get_f32("radius").unwrap_or_default(),
            effect_distance: inst.get_f32("effectDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsOceanSurfaceRepelRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsOceanSurfaceRepelRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
    /// `effectDistance` (Single)
    #[serde(default)]
    pub effect_distance: f32,
    /// `belowOcean` (Boolean)
    #[serde(default)]
    pub below_ocean: bool,
}

impl Pooled for BoidsOceanSurfaceRepelRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_ocean_surface_repel_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_ocean_surface_repel_rule }
}

impl<'a> Extract<'a> for BoidsOceanSurfaceRepelRule {
    const TYPE_NAME: &'static str = "BoidsOceanSurfaceRepelRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
            effect_distance: inst.get_f32("effectDistance").unwrap_or_default(),
            below_ocean: inst.get_bool("belowOcean").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsTerrainSurfaceRepelRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsTerrainSurfaceRepelRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
    /// `effectDistance` (Single)
    #[serde(default)]
    pub effect_distance: f32,
}

impl Pooled for BoidsTerrainSurfaceRepelRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_terrain_surface_repel_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_terrain_surface_repel_rule }
}

impl<'a> Extract<'a> for BoidsTerrainSurfaceRepelRule {
    const TYPE_NAME: &'static str = "BoidsTerrainSurfaceRepelRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
            effect_distance: inst.get_f32("effectDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsNavmeshEdgeRepelRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsNavmeshEdgeRepelRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
    /// `effectDistance` (Single)
    #[serde(default)]
    pub effect_distance: f32,
    /// `decayCoefficient` (Single)
    #[serde(default)]
    pub decay_coefficient: f32,
    /// `maxAcceleration` (Single)
    #[serde(default)]
    pub max_acceleration: f32,
}

impl Pooled for BoidsNavmeshEdgeRepelRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_navmesh_edge_repel_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_navmesh_edge_repel_rule }
}

impl<'a> Extract<'a> for BoidsNavmeshEdgeRepelRule {
    const TYPE_NAME: &'static str = "BoidsNavmeshEdgeRepelRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
            effect_distance: inst.get_f32("effectDistance").unwrap_or_default(),
            decay_coefficient: inst.get_f32("decayCoefficient").unwrap_or_default(),
            max_acceleration: inst.get_f32("maxAcceleration").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsAlertPointRepelRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsAlertPointRepelRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
    /// `decayCoefficient` (Single)
    #[serde(default)]
    pub decay_coefficient: f32,
    /// `maxAcceleration` (Single)
    #[serde(default)]
    pub max_acceleration: f32,
}

impl Pooled for BoidsAlertPointRepelRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_alert_point_repel_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_alert_point_repel_rule }
}

impl<'a> Extract<'a> for BoidsAlertPointRepelRule {
    const TYPE_NAME: &'static str = "BoidsAlertPointRepelRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
            decay_coefficient: inst.get_f32("decayCoefficient").unwrap_or_default(),
            max_acceleration: inst.get_f32("maxAcceleration").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsBehaviorRuleContainer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsBehaviorRuleContainer {
    /// `enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `rule` (StrongPointer)
    #[serde(default)]
    pub rule: Option<BoidsBehaviorRulePtr>,
}

impl Pooled for BoidsBehaviorRuleContainer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_behavior_rule_container }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_behavior_rule_container }
}

impl<'a> Extract<'a> for BoidsBehaviorRuleContainer {
    const TYPE_NAME: &'static str = "BoidsBehaviorRuleContainer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            rule: match inst.get("rule") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(BoidsBehaviorRulePtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `BoidsActorRepelRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsActorRepelRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
    /// `effectDistance` (Single)
    #[serde(default)]
    pub effect_distance: f32,
}

impl Pooled for BoidsActorRepelRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_actor_repel_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_actor_repel_rule }
}

impl<'a> Extract<'a> for BoidsActorRepelRule {
    const TYPE_NAME: &'static str = "BoidsActorRepelRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
            effect_distance: inst.get_f32("effectDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsVehicleRepelRule`
/// Inherits from: `BoidsBehaviorRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsVehicleRepelRule {
    /// `velocityScale` (Single)
    #[serde(default)]
    pub velocity_scale: f32,
    /// `effectDistance` (Single)
    #[serde(default)]
    pub effect_distance: f32,
}

impl Pooled for BoidsVehicleRepelRule {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_vehicle_repel_rule }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_vehicle_repel_rule }
}

impl<'a> Extract<'a> for BoidsVehicleRepelRule {
    const TYPE_NAME: &'static str = "BoidsVehicleRepelRule";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            velocity_scale: inst.get_f32("velocityScale").unwrap_or_default(),
            effect_distance: inst.get_f32("effectDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidsComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidsComponentParams {
    /// `boidsGroups` (Class (array))
    #[serde(default)]
    pub boids_groups: Vec<Handle<BoidsGroupComposition>>,
    /// `groupInfluenceRange` (Single)
    #[serde(default)]
    pub group_influence_range: f32,
    /// `boidStates` (Class (array))
    #[serde(default)]
    pub boid_states: Vec<Handle<BoidState>>,
    /// `querySphereRadius` (Single)
    #[serde(default)]
    pub query_sphere_radius: f32,
}

impl Pooled for BoidsComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boids_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boids_component_params }
}

impl<'a> Extract<'a> for BoidsComponentParams {
    const TYPE_NAME: &'static str = "BoidsComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            boids_groups: inst.get_array("boidsGroups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BoidsGroupComposition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<BoidsGroupComposition>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            group_influence_range: inst.get_f32("groupInfluenceRange").unwrap_or_default(),
            boid_states: inst.get_array("boidStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BoidState>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<BoidState>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            query_sphere_radius: inst.get_f32("querySphereRadius").unwrap_or_default(),
        }
    }
}

/// DCB type: `BoidAgentComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoidAgentComponentParams {
    /// `deathMannequinTag` (String)
    #[serde(default)]
    pub death_mannequin_tag: String,
    /// `deathMannequinFragment` (String)
    #[serde(default)]
    pub death_mannequin_fragment: String,
}

impl Pooled for BoidAgentComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.creatures.boid_agent_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.creatures.boid_agent_component_params }
}

impl<'a> Extract<'a> for BoidAgentComponentParams {
    const TYPE_NAME: &'static str = "BoidAgentComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            death_mannequin_tag: inst.get_str("deathMannequinTag").map(String::from).unwrap_or_default(),
            death_mannequin_fragment: inst.get_str("deathMannequinFragment").map(String::from).unwrap_or_default(),
        }
    }
}

