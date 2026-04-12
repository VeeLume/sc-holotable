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

/// DCB type: `TemperatureUIParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureUIParams {
    /// DCB field: `minTemperatureDisplay` (Single)
    #[serde(default)]
    pub min_temperature_display: f32,
    /// DCB field: `maxTemperatureDisplay` (Single)
    #[serde(default)]
    pub max_temperature_display: f32,
}

impl Pooled for TemperatureUIParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_te.temperature_uiparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_te.temperature_uiparams }
}

impl<'a> Extract<'a> for TemperatureUIParams {
    const TYPE_NAME: &'static str = "TemperatureUIParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_temperature_display: inst.get_f32("minTemperatureDisplay").unwrap_or_default(),
            max_temperature_display: inst.get_f32("maxTemperatureDisplay").unwrap_or_default(),
        }
    }
}

/// DCB type: `TemperatureDamageControl`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureDamageControl {
}

impl Pooled for TemperatureDamageControl {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_te.temperature_damage_control }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_te.temperature_damage_control }
}

impl<'a> Extract<'a> for TemperatureDamageControl {
    const TYPE_NAME: &'static str = "TemperatureDamageControl";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `TestAtomics`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAtomics {
    /// DCB field: `myString` (String)
    #[serde(default)]
    pub my_string: String,
    /// DCB field: `myEnum` (EnumChoice)
    #[serde(default)]
    pub my_enum: String,
    /// DCB field: `myInt8` (SByte)
    #[serde(default)]
    pub my_int8: i32,
    /// DCB field: `myInt16` (Int16)
    #[serde(default)]
    pub my_int16: i32,
    /// DCB field: `myInt32` (Int32)
    #[serde(default)]
    pub my_int32: i32,
    /// DCB field: `myInt64` (Int64)
    #[serde(default)]
    pub my_int64: i64,
    /// DCB field: `myUInt8` (Byte)
    #[serde(default)]
    pub my_uint8: u32,
    /// DCB field: `myUInt16` (UInt16)
    #[serde(default)]
    pub my_uint16: u32,
    /// DCB field: `myUInt32` (UInt32)
    #[serde(default)]
    pub my_uint32: u32,
    /// DCB field: `myUInt64` (UInt64)
    #[serde(default)]
    pub my_uint64: u64,
    /// DCB field: `myBooleanTrue` (Boolean)
    #[serde(default)]
    pub my_boolean_true: bool,
    /// DCB field: `myFloat` (Single)
    #[serde(default)]
    pub my_float: f32,
    /// DCB field: `myDouble` (Double)
    #[serde(default)]
    pub my_double: f64,
}

impl Pooled for TestAtomics {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_te.test_atomics }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_te.test_atomics }
}

impl<'a> Extract<'a> for TestAtomics {
    const TYPE_NAME: &'static str = "TestAtomics";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            my_string: inst.get_str("myString").map(String::from).unwrap_or_default(),
            my_enum: inst.get_str("myEnum").map(String::from).unwrap_or_default(),
            my_int8: inst.get_i32("myInt8").unwrap_or_default(),
            my_int16: inst.get_i32("myInt16").unwrap_or_default(),
            my_int32: inst.get_i32("myInt32").unwrap_or_default(),
            my_int64: inst.get_i64("myInt64").unwrap_or_default(),
            my_uint8: inst.get_u32("myUInt8").unwrap_or_default(),
            my_uint16: inst.get_u32("myUInt16").unwrap_or_default(),
            my_uint32: inst.get_u32("myUInt32").unwrap_or_default(),
            my_uint64: inst.get("myUInt64").and_then(|v| v.as_u64()).unwrap_or_default(),
            my_boolean_true: inst.get_bool("myBooleanTrue").unwrap_or_default(),
            my_float: inst.get_f32("myFloat").unwrap_or_default(),
            my_double: inst.get_f64("myDouble").unwrap_or_default(),
        }
    }
}

/// DCB type: `TestArrays`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestArrays {
    /// DCB field: `myStringArray` (String (array))
    #[serde(default)]
    pub my_string_array: Vec<String>,
    /// DCB field: `myInt8Array` (SByte (array))
    #[serde(default)]
    pub my_int8_array: Vec<i32>,
    /// DCB field: `myInt16Array` (Int16 (array))
    #[serde(default)]
    pub my_int16_array: Vec<i32>,
    /// DCB field: `myInt32Array` (Int32 (array))
    #[serde(default)]
    pub my_int32_array: Vec<i32>,
    /// DCB field: `myInt64Array` (Int64 (array))
    #[serde(default)]
    pub my_int64_array: Vec<i64>,
    /// DCB field: `myUInt8Array` (Byte (array))
    #[serde(default)]
    pub my_uint8_array: Vec<u32>,
    /// DCB field: `myUInt16Array` (UInt16 (array))
    #[serde(default)]
    pub my_uint16_array: Vec<u32>,
    /// DCB field: `myUInt32Array` (UInt32 (array))
    #[serde(default)]
    pub my_uint32_array: Vec<u32>,
    /// DCB field: `myUInt64Array` (UInt64 (array))
    #[serde(default)]
    pub my_uint64_array: Vec<u64>,
    /// DCB field: `myBooleanArray` (Boolean (array))
    #[serde(default)]
    pub my_boolean_array: Vec<bool>,
    /// DCB field: `myFloatArray` (Single (array))
    #[serde(default)]
    pub my_float_array: Vec<f32>,
    /// DCB field: `myDoubleArray` (Double (array))
    #[serde(default)]
    pub my_double_array: Vec<f64>,
    /// DCB field: `myEnumArray` (EnumChoice (array))
    #[serde(default)]
    pub my_enum_array: Vec<String>,
    /// DCB field: `myStructArray` (Class (array))
    #[serde(default)]
    pub my_struct_array: Vec<Handle<TestAtomics>>,
    /// DCB field: `myWeakPtrArray` (WeakPointer (array))
    #[serde(default)]
    pub my_weak_ptr_array: Vec<Handle<TestAtomics>>,
    /// DCB field: `myReferenceArray` (Reference (array))
    #[serde(default)]
    pub my_reference_array: Vec<CigGuid>,
    /// DCB field: `localeArray` (Locale (array))
    #[serde(default)]
    pub locale_array: Vec<String>,
    /// DCB field: `enumFixedArray` (String)
    #[serde(default)]
    pub enum_fixed_array: String,
}

impl Pooled for TestArrays {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_te.test_arrays }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_te.test_arrays }
}

impl<'a> Extract<'a> for TestArrays {
    const TYPE_NAME: &'static str = "TestArrays";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            my_string_array: inst.get_array("myStringArray")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            my_int8_array: inst.get_array("myInt8Array")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
            my_int16_array: inst.get_array("myInt16Array")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
            my_int32_array: inst.get_array("myInt32Array")
                .map(|arr| arr.filter_map(|v| v.as_i32()).collect())
                .unwrap_or_default(),
            my_int64_array: inst.get_array("myInt64Array")
                .map(|arr| arr.filter_map(|v| v.as_i64()).collect())
                .unwrap_or_default(),
            my_uint8_array: inst.get_array("myUInt8Array")
                .map(|arr| arr.filter_map(|v| v.as_u32()).collect())
                .unwrap_or_default(),
            my_uint16_array: inst.get_array("myUInt16Array")
                .map(|arr| arr.filter_map(|v| v.as_u32()).collect())
                .unwrap_or_default(),
            my_uint32_array: inst.get_array("myUInt32Array")
                .map(|arr| arr.filter_map(|v| v.as_u32()).collect())
                .unwrap_or_default(),
            my_uint64_array: inst.get_array("myUInt64Array")
                .map(|arr| arr.filter_map(|v| v.as_u64()).collect())
                .unwrap_or_default(),
            my_boolean_array: inst.get_array("myBooleanArray")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            my_float_array: inst.get_array("myFloatArray")
                .map(|arr| arr.filter_map(|v| v.as_f32()).collect())
                .unwrap_or_default(),
            my_double_array: inst.get_array("myDoubleArray")
                .map(|arr| arr.filter_map(|v| v.as_f64()).collect())
                .unwrap_or_default(),
            my_enum_array: inst.get_array("myEnumArray")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            my_struct_array: inst.get_array("myStructArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TestAtomics>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TestAtomics>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            my_weak_ptr_array: inst.get_array("myWeakPtrArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TestAtomics>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TestAtomics>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            my_reference_array: inst.get_array("myReferenceArray")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            locale_array: inst.get_array("localeArray")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            enum_fixed_array: inst.get_str("enumFixedArray").map(String::from).unwrap_or_default(),
        }
    }
}

