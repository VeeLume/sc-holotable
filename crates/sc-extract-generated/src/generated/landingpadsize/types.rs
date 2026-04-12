// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `landingpadsize`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `LandingPadSize`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingPadSize {
    /// `id` (Int32)
    #[serde(default)]
    pub id: i32,
    /// `shipSize` (Class)
    #[serde(default)]
    pub ship_size: Option<Handle<Vec3>>,
    /// `groundVehicleSize` (Class)
    #[serde(default)]
    pub ground_vehicle_size: Option<Handle<Vec3>>,
}

impl Pooled for LandingPadSize {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.landingpadsize.landing_pad_size }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.landingpadsize.landing_pad_size }
}

impl<'a> Extract<'a> for LandingPadSize {
    const TYPE_NAME: &'static str = "LandingPadSize";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_i32("id").unwrap_or_default(),
            ship_size: match inst.get("shipSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ground_vehicle_size: match inst.get("groundVehicleSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

