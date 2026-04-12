// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `grips`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SGrip`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGrip {
    /// `gripID` (String)
    #[serde(default)]
    pub grip_id: String,
    /// `optionalHelper` (String)
    #[serde(default)]
    pub optional_helper: String,
    /// `gripShapeParameters` (Class)
    #[serde(default)]
    pub grip_shape_parameters: Option<Handle<SGripShapeParams>>,
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
}

impl Pooled for SGrip {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.grips.sgrip }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.grips.sgrip }
}

impl<'a> Extract<'a> for SGrip {
    const TYPE_NAME: &'static str = "SGrip";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            grip_id: inst.get_str("gripID").map(String::from).unwrap_or_default(),
            optional_helper: inst.get_str("optionalHelper").map(String::from).unwrap_or_default(),
            grip_shape_parameters: match inst.get("gripShapeParameters") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGripShapeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGripShapeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<QuatT>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SGripShapeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGripShapeParams {
    /// `gripShape` (String)
    #[serde(default)]
    pub grip_shape: String,
    /// `dimension` (Single)
    #[serde(default)]
    pub dimension: f32,
    /// `wristRotation` (Single)
    #[serde(default)]
    pub wrist_rotation: f32,
}

impl Pooled for SGripShapeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.grips.sgrip_shape_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.grips.sgrip_shape_params }
}

impl<'a> Extract<'a> for SGripShapeParams {
    const TYPE_NAME: &'static str = "SGripShapeParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            grip_shape: inst.get_str("gripShape").map(String::from).unwrap_or_default(),
            dimension: inst.get_f32("dimension").unwrap_or_default(),
            wrist_rotation: inst.get_f32("wristRotation").unwrap_or_default(),
        }
    }
}

/// DCB type: `Grip`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grip {
    /// `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
    /// `grip` (Class)
    #[serde(default)]
    pub grip: Option<Handle<SGrip>>,
}

impl Pooled for Grip {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.grips.grip }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.grips.grip }
}

impl<'a> Extract<'a> for Grip {
    const TYPE_NAME: &'static str = "Grip";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            grip: match inst.get("grip") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SGrip>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SGrip>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

