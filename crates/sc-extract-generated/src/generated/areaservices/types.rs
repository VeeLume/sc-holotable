// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `areaservices`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `BaseService`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseService {
    /// `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// `productName` (Locale)
    #[serde(default)]
    pub product_name: String,
    /// `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// `serviceDelayTime` (Single)
    #[serde(default)]
    pub service_delay_time: f32,
    /// `hudMessage` (Locale)
    #[serde(default)]
    pub hud_message: String,
}

impl Pooled for BaseService {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.areaservices.base_service }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.areaservices.base_service }
}

impl<'a> Extract<'a> for BaseService {
    const TYPE_NAME: &'static str = "BaseService";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            product_name: inst.get_str("productName").map(String::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            service_delay_time: inst.get_f32("serviceDelayTime").unwrap_or_default(),
            hud_message: inst.get_str("hudMessage").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AreaServices`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaServices {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `lingeringTimeout` (Single)
    #[serde(default)]
    pub lingering_timeout: f32,
    /// `service` (StrongPointer (array))
    #[serde(default)]
    pub service: Vec<Handle<BaseService>>,
}

impl Pooled for AreaServices {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.areaservices.area_services }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.areaservices.area_services }
}

impl<'a> Extract<'a> for AreaServices {
    const TYPE_NAME: &'static str = "AreaServices";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            lingering_timeout: inst.get_f32("lingeringTimeout").unwrap_or_default(),
            service: inst.get_array("service")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<BaseService>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<BaseService>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

