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

/// DCB type: `TurretInputDeflectionTimeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurretInputDeflectionTimeParams {
    /// DCB field: `angularMovement` (Class)
    #[serde(default)]
    pub angular_movement: Option<Handle<SInputDeflectionTimeParams>>,
}

impl Pooled for TurretInputDeflectionTimeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tu.turret_input_deflection_time_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tu.turret_input_deflection_time_params }
}

impl<'a> Extract<'a> for TurretInputDeflectionTimeParams {
    const TYPE_NAME: &'static str = "TurretInputDeflectionTimeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            angular_movement: match inst.get("angularMovement") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SInputDeflectionTimeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInputDeflectionTimeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

