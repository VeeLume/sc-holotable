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

/// Pool storage for the `entities-others` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesOthersPools {
    #[serde(default)]
    pub nav_spline_anchor_point_component_params: Vec<Option<NavSplineAnchorPointComponentParams>>,
    #[serde(default)]
    pub group_entity_params: Vec<Option<GroupEntityParams>>,
    #[serde(default)]
    pub loudspeaker_component_params: Vec<Option<LoudspeakerComponentParams>>,
    #[serde(default)]
    pub restricted_area_spline_params: Vec<Option<RestrictedAreaSplineParams>>,
    #[serde(default)]
    pub landing_spline_visual_params: Vec<Option<LandingSplineVisualParams>>,
    #[serde(default)]
    pub entity_component_water_disturbance_noise_params: Vec<Option<EntityComponentWaterDisturbance_NoiseParams>>,
    #[serde(default)]
    pub entity_component_water_disturbance_params: Vec<Option<EntityComponentWaterDisturbanceParams>>,
    #[serde(default)]
    pub entity_component_water_impact_test_params: Vec<Option<EntityComponentWaterImpactTestParams>>,
    #[serde(default)]
    pub transit_nav_spline_data_params: Vec<Option<TransitNavSplineDataParams>>,
}
