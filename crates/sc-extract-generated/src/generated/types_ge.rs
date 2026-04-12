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

/// DCB type: `GeometryTransformParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryTransformParams {
    /// DCB field: `initAngles` (Class)
    #[serde(default)]
    pub init_angles: Option<Handle<Vec3>>,
    /// DCB field: `randomAngles` (Class)
    #[serde(default)]
    pub random_angles: Option<Handle<Vec3>>,
    /// DCB field: `rotationRate` (Class)
    #[serde(default)]
    pub rotation_rate: Option<Handle<Vec3>>,
    /// DCB field: `randomRotation` (Class)
    #[serde(default)]
    pub random_rotation: Option<Handle<Vec3>>,
    /// DCB field: `randomScale` (Class)
    #[serde(default)]
    pub random_scale: Option<Handle<Range>>,
}

impl Pooled for GeometryTransformParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ge.geometry_transform_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ge.geometry_transform_params }
}

impl<'a> Extract<'a> for GeometryTransformParams {
    const TYPE_NAME: &'static str = "GeometryTransformParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            init_angles: match inst.get("initAngles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_angles: match inst.get("randomAngles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_rate: match inst.get("rotationRate") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_rotation: match inst.get("randomRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_scale: match inst.get("randomScale") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Range>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Range>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GeomFont_LetterNode`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomFont_LetterNode {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `code` (UInt16)
    #[serde(default)]
    pub code: u32,
}

impl Pooled for GeomFont_LetterNode {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ge.geom_font_letter_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ge.geom_font_letter_node }
}

impl<'a> Extract<'a> for GeomFont_LetterNode {
    const TYPE_NAME: &'static str = "GeomFont_LetterNode";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            code: inst.get_u32("code").unwrap_or_default(),
        }
    }
}

/// DCB type: `GeomFont_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomFont_Config {
    /// DCB field: `geometryFile` (String)
    #[serde(default)]
    pub geometry_file: String,
    /// DCB field: `metricsFile` (String)
    #[serde(default)]
    pub metrics_file: String,
    /// DCB field: `rootNode` (String)
    #[serde(default)]
    pub root_node: String,
    /// DCB field: `letterNodes` (Class (array))
    #[serde(default)]
    pub letter_nodes: Vec<Handle<GeomFont_LetterNode>>,
}

impl Pooled for GeomFont_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ge.geom_font_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ge.geom_font_config }
}

impl<'a> Extract<'a> for GeomFont_Config {
    const TYPE_NAME: &'static str = "GeomFont_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            geometry_file: inst.get_str("geometryFile").map(String::from).unwrap_or_default(),
            metrics_file: inst.get_str("metricsFile").map(String::from).unwrap_or_default(),
            root_node: inst.get_str("rootNode").map(String::from).unwrap_or_default(),
            letter_nodes: inst.get_array("letterNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<GeomFont_LetterNode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<GeomFont_LetterNode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

