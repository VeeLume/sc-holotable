// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `actor-actorblockhelper`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `BlockingHelperComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct BlockingHelperComponentParams {
    /// `offsetL` (Class)
    pub offset_l: Option<Handle<Vec3>>,
    /// `orientationL` (Class)
    pub orientation_l: Option<Handle<Ang3>>,
    /// `scaleL` (Class)
    pub scale_l: Option<Handle<Vec3>>,
    /// `offsetR` (Class)
    pub offset_r: Option<Handle<Vec3>>,
    /// `orientationR` (Class)
    pub orientation_r: Option<Handle<Ang3>>,
    /// `scaleR` (Class)
    pub scale_r: Option<Handle<Vec3>>,
}

impl Pooled for BlockingHelperComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.actor_actorblockhelper.blocking_helper_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.actor_actorblockhelper.blocking_helper_component_params }
}

impl<'a> Extract<'a> for BlockingHelperComponentParams {
    const TYPE_NAME: &'static str = "BlockingHelperComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset_l: match inst.get("offsetL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            orientation_l: match inst.get("orientationL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            scale_l: match inst.get("scaleL") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            offset_r: match inst.get("offsetR") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            orientation_r: match inst.get("orientationR") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            scale_r: match inst.get("scaleR") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

