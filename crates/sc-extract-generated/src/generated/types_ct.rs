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

/// DCB type: `CtxGraph_Node`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxGraph_Node {
}

impl Pooled for CtxGraph_Node {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ct.ctx_graph_node }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ct.ctx_graph_node }
}

impl<'a> Extract<'a> for CtxGraph_Node {
    const TYPE_NAME: &'static str = "CtxGraph_Node";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CtxGraph_Dependency`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxGraph_Dependency {
    /// DCB field: `reason` (EnumChoice)
    #[serde(default)]
    pub reason: String,
    /// DCB field: `first` (WeakPointer)
    #[serde(default)]
    pub first: Option<Handle<CtxGraph_Component>>,
    /// DCB field: `second` (WeakPointer)
    #[serde(default)]
    pub second: Option<Handle<CtxGraph_Component>>,
}

impl Pooled for CtxGraph_Dependency {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ct.ctx_graph_dependency }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ct.ctx_graph_dependency }
}

impl<'a> Extract<'a> for CtxGraph_Dependency {
    const TYPE_NAME: &'static str = "CtxGraph_Dependency";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reason: inst.get_str("reason").map(String::from).unwrap_or_default(),
            first: match inst.get("first") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CtxGraph_Component>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CtxGraph_Component>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            second: match inst.get("second") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CtxGraph_Component>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CtxGraph_Component>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CtxGraph_Group`
///
/// Inherits from: `CtxGraph_Node` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxGraph_Group {
    /// DCB field: `contexts` (WeakPointer (array))
    #[serde(default)]
    pub contexts: Vec<Handle<CtxGraph_Context>>,
}

impl Pooled for CtxGraph_Group {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ct.ctx_graph_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ct.ctx_graph_group }
}

impl<'a> Extract<'a> for CtxGraph_Group {
    const TYPE_NAME: &'static str = "CtxGraph_Group";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            contexts: inst.get_array("contexts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CtxGraph_Context>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CtxGraph_Context>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CtxGraph_Context`
///
/// Inherits from: `CtxGraph_Node` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxGraph_Context {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `dependencies` (Class (array))
    #[serde(default)]
    pub dependencies: Vec<Handle<CtxGraph_Dependency>>,
    /// DCB field: `components` (WeakPointer (array))
    #[serde(default)]
    pub components: Vec<Handle<CtxGraph_Component>>,
}

impl Pooled for CtxGraph_Context {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ct.ctx_graph_context }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ct.ctx_graph_context }
}

impl<'a> Extract<'a> for CtxGraph_Context {
    const TYPE_NAME: &'static str = "CtxGraph_Context";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            dependencies: inst.get_array("dependencies")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CtxGraph_Dependency>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CtxGraph_Dependency>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            components: inst.get_array("components")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CtxGraph_Component>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CtxGraph_Component>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CtxGraph_Component`
///
/// Inherits from: `CtxGraph_Node` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxGraph_Component {
}

impl Pooled for CtxGraph_Component {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ct.ctx_graph_component }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ct.ctx_graph_component }
}

impl<'a> Extract<'a> for CtxGraph_Component {
    const TYPE_NAME: &'static str = "CtxGraph_Component";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CtxGraph`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxGraph {
    /// DCB field: `groups` (WeakPointer (array))
    #[serde(default)]
    pub groups: Vec<Handle<CtxGraph_Group>>,
    /// DCB field: `nodes` (StrongPointer (array))
    #[serde(default)]
    pub nodes: Vec<Handle<CtxGraph_Node>>,
}

impl Pooled for CtxGraph {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ct.ctx_graph }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ct.ctx_graph }
}

impl<'a> Extract<'a> for CtxGraph {
    const TYPE_NAME: &'static str = "CtxGraph";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            groups: inst.get_array("groups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CtxGraph_Group>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CtxGraph_Group>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            nodes: inst.get_array("nodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CtxGraph_Node>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CtxGraph_Node>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

