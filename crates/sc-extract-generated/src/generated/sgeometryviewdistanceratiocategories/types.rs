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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SGeometryViewDistanceRatioCategories`
pub struct SGeometryViewDistanceRatioCategories {
    /// `categories` (Reference (array))
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

