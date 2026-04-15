// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-vehicleobjectcontainer`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `VehicleInteriorObjectContainer`
/// Inherits from: `DataForgeComponentParams`
pub struct VehicleInteriorObjectContainer {
}

impl Pooled for VehicleInteriorObjectContainer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_vehicleobjectcontainer.vehicle_interior_object_container }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_vehicleobjectcontainer.vehicle_interior_object_container }
}

impl<'a> Extract<'a> for VehicleInteriorObjectContainer {
    const TYPE_NAME: &'static str = "VehicleInteriorObjectContainer";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

