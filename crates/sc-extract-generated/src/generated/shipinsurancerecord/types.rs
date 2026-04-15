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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SUninsuredItem`
pub struct SUninsuredItem {
    /// `Type` (EnumChoice)
    pub r#type: EItemType,
    /// `SubTypes` (EnumChoice (array))
    pub sub_types: Vec<EItemSubType>,
}

impl Pooled for SUninsuredItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.shipinsurancerecord.suninsured_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.shipinsurancerecord.suninsured_item }
}

impl<'a> Extract<'a> for SUninsuredItem {
    const TYPE_NAME: &'static str = "SUninsuredItem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: EItemType::from_dcb_str(inst.get_str("Type").unwrap_or("")),
            sub_types: inst.get_array("SubTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(EItemSubType::from_dcb_str)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ShipInsurancePolicyRecord`
pub struct ShipInsurancePolicyRecord {
    /// `uninsuredItemTypes` (Class (array))
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
                        Value::ClassRef(r) => Some(b.alloc_nested::<SUninsuredItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

