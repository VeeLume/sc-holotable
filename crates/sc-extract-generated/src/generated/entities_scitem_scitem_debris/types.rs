// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-scitem_debris`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SCItemVehicleDebrisParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SCItemVehicleDebrisParams {}

impl Pooled for SCItemVehicleDebrisParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_scitem_debris
            .scitem_vehicle_debris_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_scitem_debris
            .scitem_vehicle_debris_params
    }
}

impl<'a> Extract<'a> for SCItemVehicleDebrisParams {
    const TYPE_NAME: &'static str = "SCItemVehicleDebrisParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
