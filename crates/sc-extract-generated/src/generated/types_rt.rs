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

/// DCB type: `RTTSunlightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RTTSunlightParams {
    /// DCB field: `applySunlight` (Boolean)
    #[serde(default)]
    pub apply_sunlight: bool,
    /// DCB field: `sunPos` (Class)
    #[serde(default)]
    pub sun_pos: Option<Handle<Vec3>>,
    /// DCB field: `sunColor` (Class)
    #[serde(default)]
    pub sun_color: Option<Handle<SRGBA8>>,
    /// DCB field: `ambientColor` (Class)
    #[serde(default)]
    pub ambient_color: Option<Handle<SRGBA8>>,
}

impl Pooled for RTTSunlightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_rt.rttsunlight_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_rt.rttsunlight_params }
}

impl<'a> Extract<'a> for RTTSunlightParams {
    const TYPE_NAME: &'static str = "RTTSunlightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            apply_sunlight: inst.get_bool("applySunlight").unwrap_or_default(),
            sun_pos: match inst.get("sunPos") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sun_color: match inst.get("sunColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ambient_color: match inst.get("ambientColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

