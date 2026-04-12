// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globalarmarkerparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ARMarkerPlayerOffsetParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARMarkerPlayerOffsetParams {
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// `offsetProportionPerMeter` (Class)
    #[serde(default)]
    pub offset_proportion_per_meter: Option<Handle<Vec3>>,
}

impl Pooled for ARMarkerPlayerOffsetParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalarmarkerparams.armarker_player_offset_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalarmarkerparams.armarker_player_offset_params }
}

impl<'a> Extract<'a> for ARMarkerPlayerOffsetParams {
    const TYPE_NAME: &'static str = "ARMarkerPlayerOffsetParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_proportion_per_meter: match inst.get("offsetProportionPerMeter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ARMarkerGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARMarkerGlobalParams {
    /// `ARMarkerPlayerOffset` (Class)
    #[serde(default)]
    pub armarker_player_offset: Option<Handle<ARMarkerPlayerOffsetParams>>,
}

impl Pooled for ARMarkerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalarmarkerparams.armarker_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalarmarkerparams.armarker_global_params }
}

impl<'a> Extract<'a> for ARMarkerGlobalParams {
    const TYPE_NAME: &'static str = "ARMarkerGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            armarker_player_offset: match inst.get("ARMarkerPlayerOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ARMarkerPlayerOffsetParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ARMarkerPlayerOffsetParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

