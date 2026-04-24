// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-airlocks`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `AirlockGreenzoneParams`
pub struct AirlockGreenzoneParams {
    /// `GreenZoneToggleDoorIndex` (Int32)
    pub green_zone_toggle_door_index: i32,
}

impl Pooled for AirlockGreenzoneParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_airlocks.airlock_greenzone_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_airlocks.airlock_greenzone_params
    }
}

impl<'a> Extract<'a> for AirlockGreenzoneParams {
    const TYPE_NAME: &'static str = "AirlockGreenzoneParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            green_zone_toggle_door_index: inst
                .get_i32("GreenZoneToggleDoorIndex")
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AirlockAreaParams`
pub struct AirlockAreaParams {
    /// `AreaOffset` (Class)
    pub area_offset: Option<Handle<Vec3>>,
    /// `AreaSize` (Class)
    pub area_size: Option<Handle<Vec3>>,
}

impl Pooled for AirlockAreaParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_airlocks.airlock_area_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_airlocks.airlock_area_params
    }
}

impl<'a> Extract<'a> for AirlockAreaParams {
    const TYPE_NAME: &'static str = "AirlockAreaParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            area_offset: match inst.get("AreaOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            area_size: match inst.get("AreaSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemAirlockParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SCItemAirlockParams {
    /// `MaxTimeToWaitForDoors` (Single)
    pub max_time_to_wait_for_doors: f32,
    /// `MinTimeToWaitAfterDoorsClosed` (Single)
    pub min_time_to_wait_after_doors_closed: f32,
    /// `CycleTime` (Single)
    pub cycle_time: f32,
    /// `GreenzoneParams` (StrongPointer)
    pub greenzone_params: Option<Handle<AirlockGreenzoneParams>>,
    /// `AreaOverride` (StrongPointer)
    pub area_override: Option<Handle<AirlockAreaParams>>,
}

impl Pooled for SCItemAirlockParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_airlocks.scitem_airlock_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_airlocks.scitem_airlock_params
    }
}

impl<'a> Extract<'a> for SCItemAirlockParams {
    const TYPE_NAME: &'static str = "SCItemAirlockParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_time_to_wait_for_doors: inst.get_f32("MaxTimeToWaitForDoors").unwrap_or_default(),
            min_time_to_wait_after_doors_closed: inst
                .get_f32("MinTimeToWaitAfterDoorsClosed")
                .unwrap_or_default(),
            cycle_time: inst.get_f32("CycleTime").unwrap_or_default(),
            greenzone_params: match inst.get("GreenzoneParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<AirlockGreenzoneParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            area_override: match inst.get("AreaOverride") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<AirlockAreaParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}
