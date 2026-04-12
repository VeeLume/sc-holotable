// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `fuelparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCItemSuitAtmosphereFuelConversionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSuitAtmosphereFuelConversionParams {
    /// `Gas` (Reference)
    #[serde(default)]
    pub gas: Option<CigGuid>,
    /// `massConversionRatio` (Single)
    #[serde(default)]
    pub mass_conversion_ratio: f32,
}

impl Pooled for SCItemSuitAtmosphereFuelConversionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.fuelparams.scitem_suit_atmosphere_fuel_conversion_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.fuelparams.scitem_suit_atmosphere_fuel_conversion_params }
}

impl<'a> Extract<'a> for SCItemSuitAtmosphereFuelConversionParams {
    const TYPE_NAME: &'static str = "SCItemSuitAtmosphereFuelConversionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            gas: inst.get("Gas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mass_conversion_ratio: inst.get_f32("massConversionRatio").unwrap_or_default(),
        }
    }
}

/// DCB type: `SCItemSuitFuelParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemSuitFuelParams {
    /// `fuelResourcePrimary` (Reference)
    #[serde(default)]
    pub fuel_resource_primary: Option<CigGuid>,
    /// `fuelResourceSecondary` (Reference)
    #[serde(default)]
    pub fuel_resource_secondary: Option<CigGuid>,
    /// `primaryFuelBurnRateMicroSCU` (StrongPointer)
    #[serde(default)]
    pub primary_fuel_burn_rate_micro_scu: Option<Handle<SBaseCargoUnit>>,
    /// `secondaryFuelBurnRateGrams` (Single)
    #[serde(default)]
    pub secondary_fuel_burn_rate_grams: f32,
    /// `fuelRefillRateFromTankMicroSCU` (StrongPointer)
    #[serde(default)]
    pub fuel_refill_rate_from_tank_micro_scu: Option<Handle<SBaseCargoUnit>>,
    /// `fuelRefillRateFromAtmosphereMicroSCU` (StrongPointer)
    #[serde(default)]
    pub fuel_refill_rate_from_atmosphere_micro_scu: Option<Handle<SBaseCargoUnit>>,
    /// `fuelRefillRateFromInjectionMicroSCU` (StrongPointer)
    #[serde(default)]
    pub fuel_refill_rate_from_injection_micro_scu: Option<Handle<SBaseCargoUnit>>,
    /// `gasConversionRatios` (Class (array))
    #[serde(default)]
    pub gas_conversion_ratios: Vec<Handle<SCItemSuitAtmosphereFuelConversionParams>>,
    /// `usingSecondaryFuelMessage` (Locale)
    #[serde(default)]
    pub using_secondary_fuel_message: String,
}

impl Pooled for SCItemSuitFuelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.fuelparams.scitem_suit_fuel_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.fuelparams.scitem_suit_fuel_params }
}

impl<'a> Extract<'a> for SCItemSuitFuelParams {
    const TYPE_NAME: &'static str = "SCItemSuitFuelParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fuel_resource_primary: inst.get("fuelResourcePrimary").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            fuel_resource_secondary: inst.get("fuelResourceSecondary").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            primary_fuel_burn_rate_micro_scu: match inst.get("primaryFuelBurnRateMicroSCU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            secondary_fuel_burn_rate_grams: inst.get_f32("secondaryFuelBurnRateGrams").unwrap_or_default(),
            fuel_refill_rate_from_tank_micro_scu: match inst.get("fuelRefillRateFromTankMicroSCU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fuel_refill_rate_from_atmosphere_micro_scu: match inst.get("fuelRefillRateFromAtmosphereMicroSCU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fuel_refill_rate_from_injection_micro_scu: match inst.get("fuelRefillRateFromInjectionMicroSCU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            gas_conversion_ratios: inst.get_array("gasConversionRatios")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCItemSuitAtmosphereFuelConversionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCItemSuitAtmosphereFuelConversionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            using_secondary_fuel_message: inst.get_str("usingSecondaryFuelMessage").map(String::from).unwrap_or_default(),
        }
    }
}

