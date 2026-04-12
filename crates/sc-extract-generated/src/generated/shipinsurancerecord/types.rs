// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `shipinsurancerecord`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SUninsuredItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SUninsuredItem {
    /// `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// `SubTypes` (EnumChoice (array))
    #[serde(default)]
    pub sub_types: Vec<String>,
}

impl Pooled for SUninsuredItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.shipinsurancerecord.suninsured_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.shipinsurancerecord.suninsured_item }
}

impl<'a> Extract<'a> for SUninsuredItem {
    const TYPE_NAME: &'static str = "SUninsuredItem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
            sub_types: inst.get_array("SubTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ShipInsurancePolicyRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipInsurancePolicyRecord {
    /// `uninsuredItemTypes` (Class (array))
    #[serde(default)]
    pub uninsured_item_types: Vec<Handle<SUninsuredItem>>,
}

impl Pooled for ShipInsurancePolicyRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.shipinsurancerecord.ship_insurance_policy_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.shipinsurancerecord.ship_insurance_policy_record }
}

impl<'a> Extract<'a> for ShipInsurancePolicyRecord {
    const TYPE_NAME: &'static str = "ShipInsurancePolicyRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            uninsured_item_types: inst.get_array("uninsuredItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SUninsuredItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SUninsuredItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

