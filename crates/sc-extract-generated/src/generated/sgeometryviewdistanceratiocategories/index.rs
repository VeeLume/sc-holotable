// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `sgeometryviewdistanceratiocategories` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SgeometryviewdistanceratiocategoriesIndex {
    #[serde(default)]
    pub sview_distance_ratio_params: HashMap<CigGuid, Handle<SViewDistanceRatioParams>>,
    #[serde(default)]
    pub sgeometry_view_distance_ratio_categories: HashMap<CigGuid, Handle<SGeometryViewDistanceRatioCategories>>,
}

impl SgeometryviewdistanceratiocategoriesIndex {
    pub fn len(&self) -> usize {
        self.sview_distance_ratio_params.len()
            + self.sgeometry_view_distance_ratio_categories.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
