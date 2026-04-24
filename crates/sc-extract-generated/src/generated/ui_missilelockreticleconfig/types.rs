// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-missilelockreticleconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `MissileLockReticleSegmentDef`
pub struct MissileLockReticleSegmentDef {
    /// `widthRatio` (Single)
    pub width_ratio: f32,
    /// `heightRatio` (Single)
    pub height_ratio: f32,
    /// `anchorX` (Single)
    pub anchor_x: f32,
    /// `anchorY` (Single)
    pub anchor_y: f32,
    /// `geometryPath` (String)
    pub geometry_path: String,
}

impl Pooled for MissileLockReticleSegmentDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_missilelockreticleconfig
            .missile_lock_reticle_segment_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_missilelockreticleconfig
            .missile_lock_reticle_segment_def
    }
}

impl<'a> Extract<'a> for MissileLockReticleSegmentDef {
    const TYPE_NAME: &'static str = "MissileLockReticleSegmentDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            width_ratio: inst.get_f32("widthRatio").unwrap_or_default(),
            height_ratio: inst.get_f32("heightRatio").unwrap_or_default(),
            anchor_x: inst.get_f32("anchorX").unwrap_or_default(),
            anchor_y: inst.get_f32("anchorY").unwrap_or_default(),
            geometry_path: inst
                .get_str("geometryPath")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissileLockReticle_Config`
pub struct MissileLockReticle_Config {
    /// `segments` (Class (array))
    pub segments: Vec<Handle<MissileLockReticleSegmentDef>>,
}

impl Pooled for MissileLockReticle_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_missilelockreticleconfig
            .missile_lock_reticle_config
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_missilelockreticleconfig
            .missile_lock_reticle_config
    }
}

impl<'a> Extract<'a> for MissileLockReticle_Config {
    const TYPE_NAME: &'static str = "MissileLockReticle_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            segments: inst
                .get_array("segments")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<MissileLockReticleSegmentDef>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<MissileLockReticleSegmentDef>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
