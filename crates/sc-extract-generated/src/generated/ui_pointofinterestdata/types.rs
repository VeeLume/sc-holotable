// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-pointofinterestdata`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `PointOfInterestData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointOfInterestData {
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `displayInfoText` (Locale)
    #[serde(default)]
    pub display_info_text: String,
    /// `imagePath` (String)
    #[serde(default)]
    pub image_path: String,
    /// `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<Vec2>>,
    /// `radius` (Single)
    #[serde(default)]
    pub radius: f32,
}

impl Pooled for PointOfInterestData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_pointofinterestdata.point_of_interest_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_pointofinterestdata.point_of_interest_data }
}

impl<'a> Extract<'a> for PointOfInterestData {
    const TYPE_NAME: &'static str = "PointOfInterestData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            display_info_text: inst.get_str("displayInfoText").map(String::from).unwrap_or_default(),
            image_path: inst.get_str("imagePath").map(String::from).unwrap_or_default(),
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            radius: inst.get_f32("radius").unwrap_or_default(),
        }
    }
}

/// DCB type: `PointOfInterestList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointOfInterestList {
    /// `pointsOfInterest` (Class (array))
    #[serde(default)]
    pub points_of_interest: Vec<Handle<PointOfInterestData>>,
}

impl Pooled for PointOfInterestList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_pointofinterestdata.point_of_interest_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_pointofinterestdata.point_of_interest_list }
}

impl<'a> Extract<'a> for PointOfInterestList {
    const TYPE_NAME: &'static str = "PointOfInterestList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            points_of_interest: inst.get_array("pointsOfInterest")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PointOfInterestData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PointOfInterestData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

