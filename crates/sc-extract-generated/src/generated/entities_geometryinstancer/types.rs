// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-geometryinstancer`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `GeometryInstancer_Serialized`
pub struct GeometryInstancer_Serialized {
    /// `Version` (Int32)
    pub version: i32,
    /// `Flags` (Int32)
    pub flags: i32,
    /// `Geometry` (Class)
    pub geometry: Option<Handle<GlobalResourceCGF>>,
    /// `Material` (Class)
    pub material: Option<Handle<GlobalResourceMaterial>>,
    /// `MinAABB` (Class)
    pub min_aabb: Option<Handle<Vec3>>,
    /// `MaxAABB` (Class)
    pub max_aabb: Option<Handle<Vec3>>,
    /// `EncodedSize` (Int32)
    pub encoded_size: i32,
    /// `EncodedBase64` (String)
    pub encoded_base64: String,
}

impl Pooled for GeometryInstancer_Serialized {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_geometryinstancer.geometry_instancer_serialized }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_geometryinstancer.geometry_instancer_serialized }
}

impl<'a> Extract<'a> for GeometryInstancer_Serialized {
    const TYPE_NAME: &'static str = "GeometryInstancer_Serialized";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            version: inst.get_i32("Version").unwrap_or_default(),
            flags: inst.get_i32("Flags").unwrap_or_default(),
            geometry: match inst.get("Geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceCGF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            material: match inst.get("Material") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceMaterial>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            min_aabb: match inst.get("MinAABB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            max_aabb: match inst.get("MaxAABB") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            encoded_size: inst.get_i32("EncodedSize").unwrap_or_default(),
            encoded_base64: inst.get_str("EncodedBase64").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `GeometryInstancerComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct GeometryInstancerComponentParams {
    /// `Serialized` (Class)
    pub serialized: Option<Handle<GeometryInstancer_Serialized>>,
}

impl Pooled for GeometryInstancerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_geometryinstancer.geometry_instancer_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_geometryinstancer.geometry_instancer_component_params }
}

impl<'a> Extract<'a> for GeometryInstancerComponentParams {
    const TYPE_NAME: &'static str = "GeometryInstancerComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            serialized: match inst.get("Serialized") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GeometryInstancer_Serialized>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

