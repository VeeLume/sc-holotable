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

/// DCB type: `SYawPitchRollCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SYawPitchRollCurves {
    /// DCB field: `yawCurve` (Class)
    #[serde(default)]
    pub yaw_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `pitchCurve` (Class)
    #[serde(default)]
    pub pitch_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `rollCurve` (Class)
    #[serde(default)]
    pub roll_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SYawPitchRollCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sy.syaw_pitch_roll_curves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sy.syaw_pitch_roll_curves }
}

impl<'a> Extract<'a> for SYawPitchRollCurves {
    const TYPE_NAME: &'static str = "SYawPitchRollCurves";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            yaw_curve: match inst.get("yawCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitch_curve: match inst.get("pitchCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll_curve: match inst.get("rollCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

