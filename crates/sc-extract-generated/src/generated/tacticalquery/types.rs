// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `tacticalquery`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `TQSInputIntValue`
/// Inherits from: `TQSInput`
pub struct TQSInputIntValue {
    /// `requirements` (Class)
    pub requirements: Option<Handle<TagsDNF>>,
    /// `condition` (String)
    pub condition: String,
    /// `value` (Int32)
    pub value: i32,
    /// `overrides` (Class (array))
    pub overrides: Vec<Handle<TQSInputIntValue>>,
}

impl Pooled for TQSInputIntValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tacticalquery.tqsinput_int_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tacticalquery.tqsinput_int_value }
}

impl<'a> Extract<'a> for TQSInputIntValue {
    const TYPE_NAME: &'static str = "TQSInputIntValue";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            requirements: match inst.get("requirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            condition: inst.get_str("condition").map(String::from).unwrap_or_default(),
            value: inst.get_i32("value").unwrap_or_default(),
            overrides: inst.get_array("overrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TQSInputIntValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TQSInputIntValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TQSInputDynamicVariableValue`
/// Inherits from: `TQSInput`
pub struct TQSInputDynamicVariableValue {
    /// `requirements` (Class)
    pub requirements: Option<Handle<TagsDNF>>,
    /// `condition` (String)
    pub condition: String,
    /// `value` (String)
    pub value: String,
    /// `overrides` (Class (array))
    pub overrides: Vec<Handle<TQSInputDynamicVariableValue>>,
}

impl Pooled for TQSInputDynamicVariableValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tacticalquery.tqsinput_dynamic_variable_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tacticalquery.tqsinput_dynamic_variable_value }
}

impl<'a> Extract<'a> for TQSInputDynamicVariableValue {
    const TYPE_NAME: &'static str = "TQSInputDynamicVariableValue";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            requirements: match inst.get("requirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            condition: inst.get_str("condition").map(String::from).unwrap_or_default(),
            value: inst.get_str("value").map(String::from).unwrap_or_default(),
            overrides: inst.get_array("overrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TQSInputDynamicVariableValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TQSInputDynamicVariableValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TQSWeightInputFloatValue`
/// Inherits from: `TQSWeightInput`
pub struct TQSWeightInputFloatValue {
    /// `requirements` (Class)
    pub requirements: Option<Handle<TagsDNF>>,
    /// `condition` (String)
    pub condition: String,
    /// `weight` (Single)
    pub weight: f32,
    /// `value` (Single)
    pub value: f32,
    /// `overrides` (Class (array))
    pub overrides: Vec<Handle<TQSWeightInputFloatValue>>,
}

impl Pooled for TQSWeightInputFloatValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tacticalquery.tqsweight_input_float_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tacticalquery.tqsweight_input_float_value }
}

impl<'a> Extract<'a> for TQSWeightInputFloatValue {
    const TYPE_NAME: &'static str = "TQSWeightInputFloatValue";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            requirements: match inst.get("requirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            condition: inst.get_str("condition").map(String::from).unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
            value: inst.get_f32("value").unwrap_or_default(),
            overrides: inst.get_array("overrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TQSWeightInputFloatValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TQSWeightInputFloatValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TQSWeightInputTagValue`
/// Inherits from: `TQSWeightInput`
pub struct TQSWeightInputTagValue {
    /// `requirements` (Class)
    pub requirements: Option<Handle<TagsDNF>>,
    /// `condition` (String)
    pub condition: String,
    /// `weight` (Single)
    pub weight: f32,
    /// `value` (Class)
    pub value: Option<Handle<TagsDNF>>,
    /// `overrides` (Class (array))
    pub overrides: Vec<Handle<TQSWeightInputTagValue>>,
}

impl Pooled for TQSWeightInputTagValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tacticalquery.tqsweight_input_tag_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tacticalquery.tqsweight_input_tag_value }
}

impl<'a> Extract<'a> for TQSWeightInputTagValue {
    const TYPE_NAME: &'static str = "TQSWeightInputTagValue";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            requirements: match inst.get("requirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            condition: inst.get_str("condition").map(String::from).unwrap_or_default(),
            weight: inst.get_f32("weight").unwrap_or_default(),
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            overrides: inst.get_array("overrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TQSWeightInputTagValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TQSWeightInputTagValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TQSOptionContentRecord`
pub struct TQSOptionContentRecord {
    /// `content` (Class)
    pub content: Option<Handle<TQSOptionContent>>,
}

impl Pooled for TQSOptionContentRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tacticalquery.tqsoption_content_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tacticalquery.tqsoption_content_record }
}

impl<'a> Extract<'a> for TQSOptionContentRecord {
    const TYPE_NAME: &'static str = "TQSOptionContentRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            content: match inst.get("content") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TQSOptionContent>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `TQSOptionReference`
/// Inherits from: `TQSOption`
pub struct TQSOptionReference {
    /// `requirements` (Class)
    pub requirements: Option<Handle<TagsDNF>>,
    /// `content` (Reference)
    pub content: Option<CigGuid>,
    /// `optionOverrides` (StrongPointer (array))
    pub option_overrides: Vec<TQSOptionPtr>,
}

impl Pooled for TQSOptionReference {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.tacticalquery.tqsoption_reference }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.tacticalquery.tqsoption_reference }
}

impl<'a> Extract<'a> for TQSOptionReference {
    const TYPE_NAME: &'static str = "TQSOptionReference";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            requirements: match inst.get("requirements") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            content: inst.get("content").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            option_overrides: inst.get_array("optionOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(TQSOptionPtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

