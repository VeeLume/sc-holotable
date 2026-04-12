// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `sgeometryviewdistanceratiocategories` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SgeometryviewdistanceratiocategoriesPools {
    #[serde(default)]
    pub sview_distance_ratio_params: Vec<Option<SViewDistanceRatioParams>>,
    #[serde(default)]
    pub sgeometry_view_distance_ratio_categories: Vec<Option<SGeometryViewDistanceRatioCategories>>,
}
