// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scarmorymanager`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SArmouryManagerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SArmouryManagerParams {
    /// `armouryData` (Reference)
    pub armoury_data: Option<CigGuid>,
    /// `armourer` (Class)
    pub armourer: Option<Handle<EntityReferenceDef>>,
    /// `weaponRacks` (Class (array))
    pub weapon_racks: Vec<Handle<EntityReferenceDef>>,
    /// `inventories` (Class (array))
    pub inventories: Vec<Handle<EntityReferenceDef>>,
    /// `locker` (Class)
    pub locker: Option<Handle<EntityReferenceDef>>,
    /// `spawnerLocations` (Class (array))
    pub spawner_locations: Vec<Handle<EntityReferenceDef>>,
    /// `requestInteractionRadius` (Single)
    pub request_interaction_radius: f32,
    /// `firingRangeEntities` (Class (array))
    pub firing_range_entities: Vec<Handle<EntityReferenceDef>>,
    /// `cleanupArea` (Class)
    pub cleanup_area: Option<Handle<EntityReferenceDef>>,
}

impl Pooled for SArmouryManagerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scarmorymanager.sarmoury_manager_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scarmorymanager.sarmoury_manager_params }
}

impl<'a> Extract<'a> for SArmouryManagerParams {
    const TYPE_NAME: &'static str = "SArmouryManagerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            armoury_data: inst.get("armouryData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            armourer: match inst.get("armourer") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            weapon_racks: inst.get_array("weaponRacks")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EntityReferenceDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            inventories: inst.get_array("inventories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EntityReferenceDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            locker: match inst.get("locker") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            spawner_locations: inst.get_array("spawnerLocations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EntityReferenceDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            request_interaction_radius: inst.get_f32("requestInteractionRadius").unwrap_or_default(),
            firing_range_entities: inst.get_array("firingRangeEntities")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EntityReferenceDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            cleanup_area: match inst.get("cleanupArea") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EntityReferenceDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

