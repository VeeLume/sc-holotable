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

/// Pool storage for the `entities-physics` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesPhysicsPools {
    #[serde(default)]
    pub constraint_params: Vec<Option<ConstraintParams>>,
    #[serde(default)]
    pub dead_body_params: Vec<Option<DeadBodyParams>>,
    #[serde(default)]
    pub srope_proxy_params: Vec<Option<SRopeProxyParams>>,
    #[serde(default)]
    pub wind_area_params: Vec<Option<WindAreaParams>>,
    #[serde(default)]
    pub sentity_component_constraint_partner_component_params: Vec<Option<SEntityComponentConstraintPartnerComponentParams>>,
    #[serde(default)]
    pub slayer_entities_group_component_params: Vec<Option<SLayerEntitiesGroupComponentParams>>,
    #[serde(default)]
    pub sentity_articulated_physics_controller_params: Vec<Option<SEntityArticulatedPhysicsControllerParams>>,
}
