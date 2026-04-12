// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `DuckPose`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuckPose {
    /// DCB field: `spineMaxBendAngle` (Single)
    #[serde(default)]
    pub spine_max_bend_angle: f32,
    /// DCB field: `spineToHipRatio` (Single)
    #[serde(default)]
    pub spine_to_hip_ratio: f32,
    /// DCB field: `hipHorOffsetScale` (Class)
    #[serde(default)]
    pub hip_hor_offset_scale: Option<Handle<Vec2>>,
}

impl Pooled for DuckPose {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_du.duck_pose }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_du.duck_pose }
}

impl<'a> Extract<'a> for DuckPose {
    const TYPE_NAME: &'static str = "DuckPose";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            spine_max_bend_angle: inst.get_f32("spineMaxBendAngle").unwrap_or_default(),
            spine_to_hip_ratio: inst.get_f32("spineToHipRatio").unwrap_or_default(),
            hip_hor_offset_scale: match inst.get("hipHorOffsetScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `DurationTags`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationTags {
    /// DCB field: `tags` (String (array))
    #[serde(default)]
    pub tags: Vec<String>,
    /// DCB field: `minDuration` (Single)
    #[serde(default)]
    pub min_duration: f32,
}

impl Pooled for DurationTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_du.duration_tags }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_du.duration_tags }
}

impl<'a> Extract<'a> for DurationTags {
    const TYPE_NAME: &'static str = "DurationTags";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            min_duration: inst.get_f32("minDuration").unwrap_or_default(),
        }
    }
}

