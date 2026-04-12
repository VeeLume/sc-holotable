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

/// DCB type: `RoomTraversalOperationsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTraversalOperationsParams {
    /// DCB field: `operationType` (EnumChoice)
    #[serde(default)]
    pub operation_type: String,
    /// DCB field: `traversalValue` (Single)
    #[serde(default)]
    pub traversal_value: f32,
}

impl Pooled for RoomTraversalOperationsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ro.room_traversal_operations_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ro.room_traversal_operations_params }
}

impl<'a> Extract<'a> for RoomTraversalOperationsParams {
    const TYPE_NAME: &'static str = "RoomTraversalOperationsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            operation_type: inst.get_str("operationType").map(String::from).unwrap_or_default(),
            traversal_value: inst.get_f32("traversalValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `RoomTraversalContactTypeEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTraversalContactTypeEntry {
    /// DCB field: `contactType` (Reference)
    #[serde(default)]
    pub contact_type: Option<CigGuid>,
    /// DCB field: `operations` (Class)
    #[serde(default)]
    pub operations: Option<Handle<RoomTraversalOperationsParams>>,
}

impl Pooled for RoomTraversalContactTypeEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ro.room_traversal_contact_type_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ro.room_traversal_contact_type_entry }
}

impl<'a> Extract<'a> for RoomTraversalContactTypeEntry {
    const TYPE_NAME: &'static str = "RoomTraversalContactTypeEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            contact_type: inst.get("contactType").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            operations: match inst.get("operations") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RoomTraversalOperationsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RoomTraversalOperationsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `RoomTraversalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTraversalParams {
    /// DCB field: `maxTraversalNumber` (UInt32)
    #[serde(default)]
    pub max_traversal_number: u32,
    /// DCB field: `contactTypeEntry` (Class (array))
    #[serde(default)]
    pub contact_type_entry: Vec<Handle<RoomTraversalContactTypeEntry>>,
}

impl Pooled for RoomTraversalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ro.room_traversal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ro.room_traversal_params }
}

impl<'a> Extract<'a> for RoomTraversalParams {
    const TYPE_NAME: &'static str = "RoomTraversalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_traversal_number: inst.get_u32("maxTraversalNumber").unwrap_or_default(),
            contact_type_entry: inst.get_array("contactTypeEntry")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<RoomTraversalContactTypeEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<RoomTraversalContactTypeEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `RoomSharedParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomSharedParams {
    /// DCB field: `enableInteriorVsExteriorCheck` (Boolean)
    #[serde(default)]
    pub enable_interior_vs_exterior_check: bool,
    /// DCB field: `roomTraversalParams` (Class)
    #[serde(default)]
    pub room_traversal_params: Option<Handle<RoomTraversalParams>>,
}

impl Pooled for RoomSharedParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ro.room_shared_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ro.room_shared_params }
}

impl<'a> Extract<'a> for RoomSharedParams {
    const TYPE_NAME: &'static str = "RoomSharedParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_interior_vs_exterior_check: inst.get_bool("enableInteriorVsExteriorCheck").unwrap_or_default(),
            room_traversal_params: match inst.get("roomTraversalParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RoomTraversalParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RoomTraversalParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

