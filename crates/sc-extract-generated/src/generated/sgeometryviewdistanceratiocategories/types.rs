// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `sgeometryviewdistanceratiocategories`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SViewDistanceRatioParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SViewDistanceRatioParams {
    /// `viewDistRatio` (Int32)
    #[serde(default)]
    pub view_dist_ratio: i32,
}

impl Pooled for SViewDistanceRatioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sgeometryviewdistanceratiocategories.sview_distance_ratio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sgeometryviewdistanceratiocategories.sview_distance_ratio_params }
}

impl<'a> Extract<'a> for SViewDistanceRatioParams {
    const TYPE_NAME: &'static str = "SViewDistanceRatioParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            view_dist_ratio: inst.get_i32("viewDistRatio").unwrap_or_default(),
        }
    }
}

/// DCB type: `SGeometryViewDistanceRatioCategories`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGeometryViewDistanceRatioCategories {
    /// `categories` (Reference (array))
    #[serde(default)]
    pub categories: Vec<CigGuid>,
}

impl Pooled for SGeometryViewDistanceRatioCategories {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.sgeometryviewdistanceratiocategories.sgeometry_view_distance_ratio_categories }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.sgeometryviewdistanceratiocategories.sgeometry_view_distance_ratio_categories }
}

impl<'a> Extract<'a> for SGeometryViewDistanceRatioCategories {
    const TYPE_NAME: &'static str = "SGeometryViewDistanceRatioCategories";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            categories: inst.get_array("categories")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

