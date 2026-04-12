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

/// DCB type: `TQSInput`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TQSInput {
    /// DCB field: `requirements` (Class)
    #[serde(default)]
    pub requirements: Option<Handle<TagsDNF>>,
    /// DCB field: `condition` (String)
    #[serde(default)]
    pub condition: String,
}

impl Pooled for TQSInput {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tq.tqsinput }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tq.tqsinput }
}

impl<'a> Extract<'a> for TQSInput {
    const TYPE_NAME: &'static str = "TQSInput";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            requirements: match inst.get("requirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            condition: inst.get_str("condition").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `TQSWeightInput`
///
/// Inherits from: `TQSInput` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TQSWeightInput {
    /// DCB field: `requirements` (Class)
    #[serde(default)]
    pub requirements: Option<Handle<TagsDNF>>,
    /// DCB field: `condition` (String)
    #[serde(default)]
    pub condition: String,
    /// DCB field: `weight` (Single)
    #[serde(default)]
    pub weight: f32,
}

impl Pooled for TQSWeightInput {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tq.tqsweight_input }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tq.tqsweight_input }
}

impl<'a> Extract<'a> for TQSWeightInput {
    const TYPE_NAME: &'static str = "TQSWeightInput";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            requirements: match inst.get("requirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            condition: inst.get_str("condition").map(String::from).unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

/// DCB type: `TQSOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TQSOption {
    /// DCB field: `requirements` (Class)
    #[serde(default)]
    pub requirements: Option<Handle<TagsDNF>>,
}

impl Pooled for TQSOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tq.tqsoption }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tq.tqsoption }
}

impl<'a> Extract<'a> for TQSOption {
    const TYPE_NAME: &'static str = "TQSOption";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            requirements: match inst.get("requirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TQSOptionContentRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TQSOptionContentRecord {
    /// DCB field: `content` (Class)
    #[serde(default)]
    pub content: Option<Handle<TQSOptionContent>>,
}

impl Pooled for TQSOptionContentRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tq.tqsoption_content_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tq.tqsoption_content_record }
}

impl<'a> Extract<'a> for TQSOptionContentRecord {
    const TYPE_NAME: &'static str = "TQSOptionContentRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            content: match inst.get("content") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TQSOptionContent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TQSOptionContent>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TQSOptionContent`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TQSOptionContent {
    /// DCB field: `parameters` (StrongPointer (array))
    #[serde(default)]
    pub parameters: Vec<Handle<TQSInput>>,
    /// DCB field: `generation` (StrongPointer (array))
    #[serde(default)]
    pub generation: Vec<Handle<TQSInput>>,
    /// DCB field: `conditions` (StrongPointer (array))
    #[serde(default)]
    pub conditions: Vec<Handle<TQSInput>>,
    /// DCB field: `weights` (StrongPointer (array))
    #[serde(default)]
    pub weights: Vec<Handle<TQSWeightInput>>,
}

impl Pooled for TQSOptionContent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_tq.tqsoption_content }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_tq.tqsoption_content }
}

impl<'a> Extract<'a> for TQSOptionContent {
    const TYPE_NAME: &'static str = "TQSOptionContent";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            parameters: inst.get_array("parameters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TQSInput>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TQSInput>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            generation: inst.get_array("generation")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TQSInput>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TQSInput>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            conditions: inst.get_array("conditions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TQSInput>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TQSInput>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            weights: inst.get_array("weights")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TQSWeightInput>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TQSWeightInput>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

