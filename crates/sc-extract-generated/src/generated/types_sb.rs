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

/// DCB type: `SBezierCurveRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBezierCurveRecord {
    /// DCB field: `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SBezierCurveRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sb.sbezier_curve_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sb.sbezier_curve_record }
}

impl<'a> Extract<'a> for SBezierCurveRecord {
    const TYPE_NAME: &'static str = "SBezierCurveRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SBaseCargoUnit`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBaseCargoUnit {
}

impl Pooled for SBaseCargoUnit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sb.sbase_cargo_unit }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sb.sbase_cargo_unit }
}

impl<'a> Extract<'a> for SBaseCargoUnit {
    const TYPE_NAME: &'static str = "SBaseCargoUnit";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SBreakablePhysicsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBreakablePhysicsParams {
    /// DCB field: `maxSimultaneousCracks` (Int32)
    #[serde(default)]
    pub max_simultaneous_cracks: i32,
    /// DCB field: `maxPushForce` (Single)
    #[serde(default)]
    pub max_push_force: f32,
    /// DCB field: `maxPullForce` (Single)
    #[serde(default)]
    pub max_pull_force: f32,
    /// DCB field: `maxShiftForce` (Single)
    #[serde(default)]
    pub max_shift_force: f32,
    /// DCB field: `maxTwistTorque` (Single)
    #[serde(default)]
    pub max_twist_torque: f32,
    /// DCB field: `maxBendTorque` (Single)
    #[serde(default)]
    pub max_bend_torque: f32,
    /// DCB field: `crackWeaken` (Single)
    #[serde(default)]
    pub crack_weaken: f32,
}

impl Pooled for SBreakablePhysicsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sb.sbreakable_physics_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sb.sbreakable_physics_params }
}

impl<'a> Extract<'a> for SBreakablePhysicsParams {
    const TYPE_NAME: &'static str = "SBreakablePhysicsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_simultaneous_cracks: inst.get_i32("maxSimultaneousCracks").unwrap_or_default(),
            max_push_force: inst.get_f32("maxPushForce").unwrap_or_default(),
            max_pull_force: inst.get_f32("maxPullForce").unwrap_or_default(),
            max_shift_force: inst.get_f32("maxShiftForce").unwrap_or_default(),
            max_twist_torque: inst.get_f32("maxTwistTorque").unwrap_or_default(),
            max_bend_torque: inst.get_f32("maxBendTorque").unwrap_or_default(),
            crack_weaken: inst.get_f32("crackWeaken").unwrap_or_default(),
        }
    }
}

/// DCB type: `SBBDynamicPropertyBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SBBDynamicPropertyBase {
}

impl Pooled for SBBDynamicPropertyBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sb.sbbdynamic_property_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sb.sbbdynamic_property_base }
}

impl<'a> Extract<'a> for SBBDynamicPropertyBase {
    const TYPE_NAME: &'static str = "SBBDynamicPropertyBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

