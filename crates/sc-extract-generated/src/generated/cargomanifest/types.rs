// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `cargomanifest`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `BaseCargoFillCapacityValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCargoFillCapacityValue {
}

impl Pooled for BaseCargoFillCapacityValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cargomanifest.base_cargo_fill_capacity_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cargomanifest.base_cargo_fill_capacity_value }
}

impl<'a> Extract<'a> for BaseCargoFillCapacityValue {
    const TYPE_NAME: &'static str = "BaseCargoFillCapacityValue";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `CargoManifest`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoManifest {
    /// `descriptionTags` (Class)
    #[serde(default)]
    pub description_tags: Option<Handle<TagList>>,
    /// `cargoFillCapacity` (StrongPointer)
    #[serde(default)]
    pub cargo_fill_capacity: Option<Handle<BaseCargoFillCapacityValue>>,
    /// `tags` (Reference (array))
    #[serde(default)]
    pub tags: Vec<CigGuid>,
}

impl Pooled for CargoManifest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.cargomanifest.cargo_manifest }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.cargomanifest.cargo_manifest }
}

impl<'a> Extract<'a> for CargoManifest {
    const TYPE_NAME: &'static str = "CargoManifest";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description_tags: match inst.get("descriptionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cargo_fill_capacity: match inst.get("cargoFillCapacity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BaseCargoFillCapacityValue>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BaseCargoFillCapacityValue>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tags: inst.get_array("tags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

