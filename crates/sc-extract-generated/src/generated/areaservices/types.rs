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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `BaseService`
pub struct BaseService {
    /// `text` (Locale)
    pub text: LocaleKey,
    /// `description` (Locale)
    pub description: LocaleKey,
    /// `productName` (Locale)
    pub product_name: LocaleKey,
    /// `icon` (String)
    pub icon: String,
    /// `serviceDelayTime` (Single)
    pub service_delay_time: f32,
    /// `hudMessage` (Locale)
    pub hud_message: LocaleKey,
}

impl Pooled for BaseService {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.areaservices.base_service }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.areaservices.base_service }
}

impl<'a> Extract<'a> for BaseService {
    const TYPE_NAME: &'static str = "BaseService";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(LocaleKey::from).unwrap_or_default(),
            description: inst.get_str("description").map(LocaleKey::from).unwrap_or_default(),
            product_name: inst.get_str("productName").map(LocaleKey::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            service_delay_time: inst.get_f32("serviceDelayTime").unwrap_or_default(),
            hud_message: inst.get_str("hudMessage").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RepairService`
/// Inherits from: `BaseService`
pub struct RepairService {
    /// `text` (Locale)
    pub text: LocaleKey,
    /// `description` (Locale)
    pub description: LocaleKey,
    /// `productName` (Locale)
    pub product_name: LocaleKey,
    /// `icon` (String)
    pub icon: String,
    /// `serviceDelayTime` (Single)
    pub service_delay_time: f32,
    /// `hudMessage` (Locale)
    pub hud_message: LocaleKey,
    /// `commodityToHitPoints` (Int32)
    pub commodity_to_hit_points: i32,
    /// `commodityToDegradationLifetime` (Int32)
    pub commodity_to_degradation_lifetime: i32,
    /// `repairCommodity` (Reference)
    pub repair_commodity: Option<CigGuid>,
}

impl Pooled for RepairService {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.areaservices.repair_service }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.areaservices.repair_service }
}

impl<'a> Extract<'a> for RepairService {
    const TYPE_NAME: &'static str = "RepairService";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(LocaleKey::from).unwrap_or_default(),
            description: inst.get_str("description").map(LocaleKey::from).unwrap_or_default(),
            product_name: inst.get_str("productName").map(LocaleKey::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            service_delay_time: inst.get_f32("serviceDelayTime").unwrap_or_default(),
            hud_message: inst.get_str("hudMessage").map(LocaleKey::from).unwrap_or_default(),
            commodity_to_hit_points: inst.get_i32("commodityToHitPoints").unwrap_or_default(),
            commodity_to_degradation_lifetime: inst.get_i32("commodityToDegradationLifetime").unwrap_or_default(),
            repair_commodity: inst.get("repairCommodity").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `QuantumRefuelService`
/// Inherits from: `RefuelBaseService`
pub struct QuantumRefuelService {
    /// `text` (Locale)
    pub text: LocaleKey,
    /// `description` (Locale)
    pub description: LocaleKey,
    /// `productName` (Locale)
    pub product_name: LocaleKey,
    /// `icon` (String)
    pub icon: String,
    /// `serviceDelayTime` (Single)
    pub service_delay_time: f32,
    /// `hudMessage` (Locale)
    pub hud_message: LocaleKey,
    /// `instantRefuel` (Boolean)
    pub instant_refuel: bool,
    /// `refuelUnitPerSecond` (Int32)
    pub refuel_unit_per_second: i32,
    /// `fuelCommodity` (Reference)
    pub fuel_commodity: Option<CigGuid>,
}

impl Pooled for QuantumRefuelService {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.areaservices.quantum_refuel_service }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.areaservices.quantum_refuel_service }
}

impl<'a> Extract<'a> for QuantumRefuelService {
    const TYPE_NAME: &'static str = "QuantumRefuelService";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(LocaleKey::from).unwrap_or_default(),
            description: inst.get_str("description").map(LocaleKey::from).unwrap_or_default(),
            product_name: inst.get_str("productName").map(LocaleKey::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            service_delay_time: inst.get_f32("serviceDelayTime").unwrap_or_default(),
            hud_message: inst.get_str("hudMessage").map(LocaleKey::from).unwrap_or_default(),
            instant_refuel: inst.get_bool("instantRefuel").unwrap_or_default(),
            refuel_unit_per_second: inst.get_i32("refuelUnitPerSecond").unwrap_or_default(),
            fuel_commodity: inst.get("fuelCommodity").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `HydrogenRefuelService`
/// Inherits from: `RefuelBaseService`
pub struct HydrogenRefuelService {
    /// `text` (Locale)
    pub text: LocaleKey,
    /// `description` (Locale)
    pub description: LocaleKey,
    /// `productName` (Locale)
    pub product_name: LocaleKey,
    /// `icon` (String)
    pub icon: String,
    /// `serviceDelayTime` (Single)
    pub service_delay_time: f32,
    /// `hudMessage` (Locale)
    pub hud_message: LocaleKey,
    /// `instantRefuel` (Boolean)
    pub instant_refuel: bool,
    /// `refuelUnitPerSecond` (Int32)
    pub refuel_unit_per_second: i32,
    /// `fuelCommodity` (Reference)
    pub fuel_commodity: Option<CigGuid>,
}

impl Pooled for HydrogenRefuelService {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.areaservices.hydrogen_refuel_service }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.areaservices.hydrogen_refuel_service }
}

impl<'a> Extract<'a> for HydrogenRefuelService {
    const TYPE_NAME: &'static str = "HydrogenRefuelService";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(LocaleKey::from).unwrap_or_default(),
            description: inst.get_str("description").map(LocaleKey::from).unwrap_or_default(),
            product_name: inst.get_str("productName").map(LocaleKey::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            service_delay_time: inst.get_f32("serviceDelayTime").unwrap_or_default(),
            hud_message: inst.get_str("hudMessage").map(LocaleKey::from).unwrap_or_default(),
            instant_refuel: inst.get_bool("instantRefuel").unwrap_or_default(),
            refuel_unit_per_second: inst.get_i32("refuelUnitPerSecond").unwrap_or_default(),
            fuel_commodity: inst.get("fuelCommodity").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `RestockService`
/// Inherits from: `BaseService`
pub struct RestockService {
    /// `text` (Locale)
    pub text: LocaleKey,
    /// `description` (Locale)
    pub description: LocaleKey,
    /// `productName` (Locale)
    pub product_name: LocaleKey,
    /// `icon` (String)
    pub icon: String,
    /// `serviceDelayTime` (Single)
    pub service_delay_time: f32,
    /// `hudMessage` (Locale)
    pub hud_message: LocaleKey,
    /// `AmmoCommodity` (Reference)
    pub ammo_commodity: Option<CigGuid>,
}

impl Pooled for RestockService {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.areaservices.restock_service }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.areaservices.restock_service }
}

impl<'a> Extract<'a> for RestockService {
    const TYPE_NAME: &'static str = "RestockService";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(LocaleKey::from).unwrap_or_default(),
            description: inst.get_str("description").map(LocaleKey::from).unwrap_or_default(),
            product_name: inst.get_str("productName").map(LocaleKey::from).unwrap_or_default(),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            service_delay_time: inst.get_f32("serviceDelayTime").unwrap_or_default(),
            hud_message: inst.get_str("hudMessage").map(LocaleKey::from).unwrap_or_default(),
            ammo_commodity: inst.get("AmmoCommodity").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AreaServices`
pub struct AreaServices {
    /// `name` (String)
    pub name: String,
    /// `lingeringTimeout` (Single)
    pub lingering_timeout: f32,
    /// `service` (StrongPointer (array))
    pub service: Vec<BaseServicePtr>,
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
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(BaseServicePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

