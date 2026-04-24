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

/// Pool storage for the `entities-others` feature.
#[derive(Default)]
pub struct EntitiesOthersPools {
    pub nav_spline_anchor_point_component_params: Vec<Option<NavSplineAnchorPointComponentParams>>,
    pub group_entity_params: Vec<Option<GroupEntityParams>>,
    pub loudspeaker_component_params: Vec<Option<LoudspeakerComponentParams>>,
    pub restricted_area_spline_params: Vec<Option<RestrictedAreaSplineParams>>,
    pub landing_spline_visual_params: Vec<Option<LandingSplineVisualParams>>,
    pub entity_component_water_disturbance_noise_params:
        Vec<Option<EntityComponentWaterDisturbance_NoiseParams>>,
    pub entity_component_water_disturbance_params:
        Vec<Option<EntityComponentWaterDisturbanceParams>>,
    pub entity_component_water_impact_test_params:
        Vec<Option<EntityComponentWaterImpactTestParams>>,
    pub transit_nav_spline_data_params: Vec<Option<TransitNavSplineDataParams>>,
}
