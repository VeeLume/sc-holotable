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

/// DCB type: `UnitTestSubRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTestSubRecord {
}

impl Pooled for UnitTestSubRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_un.unit_test_sub_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_un.unit_test_sub_record }
}

impl<'a> Extract<'a> for UnitTestSubRecord {
    const TYPE_NAME: &'static str = "UnitTestSubRecord";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `UnitTest_BaseTest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTest_BaseTest {
    /// DCB field: `myBaseString` (String)
    #[serde(default)]
    pub my_base_string: String,
}

impl Pooled for UnitTest_BaseTest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_un.unit_test_base_test }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_un.unit_test_base_test }
}

impl<'a> Extract<'a> for UnitTest_BaseTest {
    const TYPE_NAME: &'static str = "UnitTest_BaseTest";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            my_base_string: inst.get_str("myBaseString").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `UnitTest_Inheritance`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTest_Inheritance {
    /// DCB field: `baseArray` (StrongPointer (array))
    #[serde(default)]
    pub base_array: Vec<Handle<UnitTest_BaseTest>>,
    /// DCB field: `weakPtrBase` (WeakPointer)
    #[serde(default)]
    pub weak_ptr_base: Option<Handle<UnitTest_BaseTest>>,
}

impl Pooled for UnitTest_Inheritance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_un.unit_test_inheritance }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_un.unit_test_inheritance }
}

impl<'a> Extract<'a> for UnitTest_Inheritance {
    const TYPE_NAME: &'static str = "UnitTest_Inheritance";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_array: inst.get_array("baseArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<UnitTest_BaseTest>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<UnitTest_BaseTest>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            weak_ptr_base: match inst.get("weakPtrBase") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UnitTest_BaseTest>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UnitTest_BaseTest>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UnitTest_OverrideDefaultsTest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTest_OverrideDefaultsTest {
    /// DCB field: `myIntTest` (Int32)
    #[serde(default)]
    pub my_int_test: i32,
    /// DCB field: `offsetAngle` (Class)
    #[serde(default)]
    pub offset_angle: Option<Handle<Ang3>>,
    /// DCB field: `anotherAngle` (Class)
    #[serde(default)]
    pub another_angle: Option<Handle<Ang3>>,
}

impl Pooled for UnitTest_OverrideDefaultsTest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_un.unit_test_override_defaults_test }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_un.unit_test_override_defaults_test }
}

impl<'a> Extract<'a> for UnitTest_OverrideDefaultsTest {
    const TYPE_NAME: &'static str = "UnitTest_OverrideDefaultsTest";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            my_int_test: inst.get_i32("myIntTest").unwrap_or_default(),
            offset_angle: match inst.get("offsetAngle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            another_angle: match inst.get("anotherAngle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UnitTest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitTest {
    /// DCB field: `atomics` (Class)
    #[serde(default)]
    pub atomics: Option<Handle<TestAtomics>>,
    /// DCB field: `arrays` (Class)
    #[serde(default)]
    pub arrays: Option<Handle<TestArrays>>,
    /// DCB field: `reference` (Reference)
    #[serde(default)]
    pub reference: Option<CigGuid>,
    /// DCB field: `weakptr` (WeakPointer)
    #[serde(default)]
    pub weakptr: Option<Handle<TestAtomics>>,
    /// DCB field: `weakptrnull` (WeakPointer)
    #[serde(default)]
    pub weakptrnull: Option<Handle<TestAtomics>>,
    /// DCB field: `strongptr` (StrongPointer)
    #[serde(default)]
    pub strongptr: Option<Handle<TestAtomics>>,
    /// DCB field: `strongptrnull` (StrongPointer)
    #[serde(default)]
    pub strongptrnull: Option<Handle<TestAtomics>>,
    /// DCB field: `inheritance` (Class)
    #[serde(default)]
    pub inheritance: Option<Handle<UnitTest_Inheritance>>,
    /// DCB field: `locale` (Locale)
    #[serde(default)]
    pub locale: String,
    /// DCB field: `colour` (Class)
    #[serde(default)]
    pub colour: Option<Handle<RGB8>>,
    /// DCB field: `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
    /// DCB field: `baseOverrideDefault` (StrongPointer)
    #[serde(default)]
    pub base_override_default: Option<Handle<UnitTest_OverrideDefaultsTest>>,
    /// DCB field: `recordReference` (Reference)
    #[serde(default)]
    pub record_reference: Option<CigGuid>,
    /// DCB field: `embeddedRecordArray` (Reference (array))
    #[serde(default)]
    pub embedded_record_array: Vec<CigGuid>,
}

impl Pooled for UnitTest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_un.unit_test }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_un.unit_test }
}

impl<'a> Extract<'a> for UnitTest {
    const TYPE_NAME: &'static str = "UnitTest";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            atomics: match inst.get("atomics") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TestAtomics>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TestAtomics>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            arrays: match inst.get("arrays") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TestArrays>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TestArrays>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            reference: inst.get("reference").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            weakptr: match inst.get("weakptr") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TestAtomics>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TestAtomics>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weakptrnull: match inst.get("weakptrnull") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TestAtomics>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TestAtomics>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            strongptr: match inst.get("strongptr") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TestAtomics>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TestAtomics>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            strongptrnull: match inst.get("strongptrnull") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TestAtomics>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TestAtomics>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inheritance: match inst.get("inheritance") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UnitTest_Inheritance>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UnitTest_Inheritance>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            locale: inst.get_str("locale").map(String::from).unwrap_or_default(),
            colour: match inst.get("colour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            base_override_default: match inst.get("baseOverrideDefault") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<UnitTest_OverrideDefaultsTest>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<UnitTest_OverrideDefaultsTest>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            record_reference: inst.get("recordReference").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            embedded_record_array: inst.get_array("embeddedRecordArray")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

