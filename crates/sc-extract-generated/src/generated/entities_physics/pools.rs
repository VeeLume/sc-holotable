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

/// Pool storage for the `entities-physics` feature.
#[derive(Default)]
pub struct EntitiesPhysicsPools {
    pub constraint_params: Vec<Option<ConstraintParams>>,
    pub dead_body_params: Vec<Option<DeadBodyParams>>,
    pub srope_proxy_params: Vec<Option<SRopeProxyParams>>,
    pub wind_area_params: Vec<Option<WindAreaParams>>,
    pub sentity_component_constraint_partner_component_params: Vec<Option<SEntityComponentConstraintPartnerComponentParams>>,
    pub slayer_entities_group_component_params: Vec<Option<SLayerEntitiesGroupComponentParams>>,
    pub sentity_articulated_physics_controller_params: Vec<Option<SEntityArticulatedPhysicsControllerParams>>,
}
