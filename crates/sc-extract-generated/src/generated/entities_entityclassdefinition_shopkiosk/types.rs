// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-entityclassdefinition_shopkiosk`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `PurchasableVehicleProviderParams`
/// Inherits from: `DataForgeComponentParams`
pub struct PurchasableVehicleProviderParams {
    /// `vehicleUsageTypes` (EnumChoice)
    pub vehicle_usage_types: PurchasableVehicleUsageType,
}

impl Pooled for PurchasableVehicleProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_entityclassdefinition_shopkiosk.purchasable_vehicle_provider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_entityclassdefinition_shopkiosk.purchasable_vehicle_provider_params }
}

impl<'a> Extract<'a> for PurchasableVehicleProviderParams {
    const TYPE_NAME: &'static str = "PurchasableVehicleProviderParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            vehicle_usage_types: PurchasableVehicleUsageType::from_dcb_str(inst.get_str("vehicleUsageTypes").unwrap_or("")),
        }
    }
}

