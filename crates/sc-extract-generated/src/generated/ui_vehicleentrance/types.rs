// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-vehicleentrance`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCEntranceItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCEntranceItem {
    /// `entranceName` (Locale)
    #[serde(default)]
    pub entrance_name: String,
    /// `jointName` (String)
    #[serde(default)]
    pub joint_name: String,
    /// `positionOffset` (Class)
    #[serde(default)]
    pub position_offset: Option<Handle<Vec3>>,
    /// `shipState` (EnumChoice)
    #[serde(default)]
    pub ship_state: String,
}

impl Pooled for SCEntranceItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_vehicleentrance.scentrance_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_vehicleentrance.scentrance_item }
}

impl<'a> Extract<'a> for SCEntranceItem {
    const TYPE_NAME: &'static str = "SCEntranceItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entrance_name: inst.get_str("entranceName").map(String::from).unwrap_or_default(),
            joint_name: inst.get_str("jointName").map(String::from).unwrap_or_default(),
            position_offset: match inst.get("positionOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            ship_state: inst.get_str("shipState").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SEntrancesDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntrancesDef {
    /// `boundingBoxScale` (Class)
    #[serde(default)]
    pub bounding_box_scale: Option<Handle<Vec3>>,
    /// `boundingBoxOffset` (Class)
    #[serde(default)]
    pub bounding_box_offset: Option<Handle<Vec3>>,
    /// `entranceArray` (Class (array))
    #[serde(default)]
    pub entrance_array: Vec<Handle<SCEntranceItem>>,
}

impl Pooled for SEntrancesDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_vehicleentrance.sentrances_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_vehicleentrance.sentrances_def }
}

impl<'a> Extract<'a> for SEntrancesDef {
    const TYPE_NAME: &'static str = "SEntrancesDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bounding_box_scale: match inst.get("boundingBoxScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bounding_box_offset: match inst.get("boundingBoxOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            entrance_array: inst.get_array("entranceArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCEntranceItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCEntranceItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

