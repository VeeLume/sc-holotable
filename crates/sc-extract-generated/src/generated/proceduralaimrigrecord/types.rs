// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `proceduralaimrigrecord`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ProcAimBaseJointTypeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcAimBaseJointTypeConfig {
    /// `jointName` (String)
    #[serde(default)]
    pub joint_name: String,
}

impl Pooled for ProcAimBaseJointTypeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.proceduralaimrigrecord.proc_aim_base_joint_type_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.proceduralaimrigrecord.proc_aim_base_joint_type_config }
}

impl<'a> Extract<'a> for ProcAimBaseJointTypeConfig {
    const TYPE_NAME: &'static str = "ProcAimBaseJointTypeConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            joint_name: inst.get_str("jointName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ProcAimRigConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcAimRigConfig {
    /// `rigName` (String)
    #[serde(default)]
    pub rig_name: String,
    /// `aimJointName` (String)
    #[serde(default)]
    pub aim_joint_name: String,
    /// `aimJointDirection` (Class)
    #[serde(default)]
    pub aim_joint_direction: Option<Handle<Vec3>>,
    /// `aimTargetSmoothDuration` (Single)
    #[serde(default)]
    pub aim_target_smooth_duration: f32,
    /// `aimTargetClampAngleDeg` (Single)
    #[serde(default)]
    pub aim_target_clamp_angle_deg: f32,
    /// `aimAngleBlendRange` (Class)
    #[serde(default)]
    pub aim_angle_blend_range: Option<Handle<Range>>,
    /// `rotationJoints` (StrongPointer (array))
    #[serde(default)]
    pub rotation_joints: Vec<Handle<ProcAimBaseJointTypeConfig>>,
}

impl Pooled for ProcAimRigConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.proceduralaimrigrecord.proc_aim_rig_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.proceduralaimrigrecord.proc_aim_rig_config }
}

impl<'a> Extract<'a> for ProcAimRigConfig {
    const TYPE_NAME: &'static str = "ProcAimRigConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rig_name: inst.get_str("rigName").map(String::from).unwrap_or_default(),
            aim_joint_name: inst.get_str("aimJointName").map(String::from).unwrap_or_default(),
            aim_joint_direction: match inst.get("aimJointDirection") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            aim_target_smooth_duration: inst.get_f32("aimTargetSmoothDuration").unwrap_or_default(),
            aim_target_clamp_angle_deg: inst.get_f32("aimTargetClampAngleDeg").unwrap_or_default(),
            aim_angle_blend_range: match inst.get("aimAngleBlendRange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_joints: inst.get_array("rotationJoints")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProcAimBaseJointTypeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProcAimBaseJointTypeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralAimRigRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralAimRigRecord {
    /// `aimLayer` (Int32)
    #[serde(default)]
    pub aim_layer: i32,
    /// `aimRigs` (Class (array))
    #[serde(default)]
    pub aim_rigs: Vec<Handle<ProcAimRigConfig>>,
}

impl Pooled for ProceduralAimRigRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.proceduralaimrigrecord.procedural_aim_rig_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.proceduralaimrigrecord.procedural_aim_rig_record }
}

impl<'a> Extract<'a> for ProceduralAimRigRecord {
    const TYPE_NAME: &'static str = "ProceduralAimRigRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            aim_layer: inst.get_i32("aimLayer").unwrap_or_default(),
            aim_rigs: inst.get_array("aimRigs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProcAimRigConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProcAimRigConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

