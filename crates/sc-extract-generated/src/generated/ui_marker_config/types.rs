// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-marker_config`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MarkerAR_ConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerAR_ConfigDef {
    /// `maxVisibleDistance` (Single)
    #[serde(default)]
    pub max_visible_distance: f32,
    /// `minFocusAngle` (Single)
    #[serde(default)]
    pub min_focus_angle: f32,
    /// `clipScreenMin` (Class)
    #[serde(default)]
    pub clip_screen_min: Option<Handle<Vec2>>,
    /// `clipScreenMax` (Class)
    #[serde(default)]
    pub clip_screen_max: Option<Handle<Vec2>>,
    /// `clipRadius` (Single)
    #[serde(default)]
    pub clip_radius: f32,
    /// `clipAspect` (Single)
    #[serde(default)]
    pub clip_aspect: f32,
}

impl Pooled for MarkerAR_ConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_marker_config.marker_ar_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_marker_config.marker_ar_config_def }
}

impl<'a> Extract<'a> for MarkerAR_ConfigDef {
    const TYPE_NAME: &'static str = "MarkerAR_ConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_visible_distance: inst.get_f32("maxVisibleDistance").unwrap_or_default(),
            min_focus_angle: inst.get_f32("minFocusAngle").unwrap_or_default(),
            clip_screen_min: match inst.get("clipScreenMin") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            clip_screen_max: match inst.get("clipScreenMax") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            clip_radius: inst.get_f32("clipRadius").unwrap_or_default(),
            clip_aspect: inst.get_f32("clipAspect").unwrap_or_default(),
        }
    }
}

