// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `proceduralanimations`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ProceduralAnimationBone`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralAnimationBone {
    /// `bone` (EnumChoice)
    #[serde(default)]
    pub bone: String,
    /// `chainLength` (Int32)
    #[serde(default)]
    pub chain_length: i32,
    /// `layer` (Int32)
    #[serde(default)]
    pub layer: i32,
    /// `operation` (EnumChoice)
    #[serde(default)]
    pub operation: String,
    /// `relativeTo` (EnumChoice)
    #[serde(default)]
    pub relative_to: String,
    /// `values` (Class)
    #[serde(default)]
    pub values: Option<Handle<Vec3>>,
    /// `delay` (Single)
    #[serde(default)]
    pub delay: f32,
}

impl Pooled for ProceduralAnimationBone {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.proceduralanimations.procedural_animation_bone }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.proceduralanimations.procedural_animation_bone }
}

impl<'a> Extract<'a> for ProceduralAnimationBone {
    const TYPE_NAME: &'static str = "ProceduralAnimationBone";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bone: inst.get_str("bone").map(String::from).unwrap_or_default(),
            chain_length: inst.get_i32("chainLength").unwrap_or_default(),
            layer: inst.get_i32("layer").unwrap_or_default(),
            operation: inst.get_str("operation").map(String::from).unwrap_or_default(),
            relative_to: inst.get_str("relativeTo").map(String::from).unwrap_or_default(),
            values: match inst.get("values") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            delay: inst.get_f32("delay").unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralAnimationSequence`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralAnimationSequence {
    /// `duration` (Single)
    #[serde(default)]
    pub duration: f32,
    /// `animationCurve` (Class)
    #[serde(default)]
    pub animation_curve: Option<Handle<BezierCurve>>,
    /// `boneAnimations` (Class (array))
    #[serde(default)]
    pub bone_animations: Vec<Handle<ProceduralAnimationBone>>,
}

impl Pooled for ProceduralAnimationSequence {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.proceduralanimations.procedural_animation_sequence }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.proceduralanimations.procedural_animation_sequence }
}

impl<'a> Extract<'a> for ProceduralAnimationSequence {
    const TYPE_NAME: &'static str = "ProceduralAnimationSequence";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            duration: inst.get_f32("duration").unwrap_or_default(),
            animation_curve: match inst.get("animationCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bone_animations: inst.get_array("boneAnimations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralAnimationBone>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralAnimationBone>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ProceduralAnimation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralAnimation {
    /// `sequences` (Class (array))
    #[serde(default)]
    pub sequences: Vec<Handle<ProceduralAnimationSequence>>,
}

impl Pooled for ProceduralAnimation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.proceduralanimations.procedural_animation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.proceduralanimations.procedural_animation }
}

impl<'a> Extract<'a> for ProceduralAnimation {
    const TYPE_NAME: &'static str = "ProceduralAnimation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sequences: inst.get_array("sequences")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ProceduralAnimationSequence>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ProceduralAnimationSequence>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

