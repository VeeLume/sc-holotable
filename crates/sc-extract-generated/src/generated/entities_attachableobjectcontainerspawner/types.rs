// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-attachableobjectcontainerspawner`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SAttachableObjectContainerData`
pub struct SAttachableObjectContainerData {
    /// `fileName` (String)
    pub file_name: String,
    /// `guid` (String)
    pub guid: String,
    /// `Offset` (Class)
    pub offset: Option<Handle<QuatT>>,
}

impl Pooled for SAttachableObjectContainerData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_attachableobjectcontainerspawner
            .sattachable_object_container_data
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_attachableobjectcontainerspawner
            .sattachable_object_container_data
    }
}

impl<'a> Extract<'a> for SAttachableObjectContainerData {
    const TYPE_NAME: &'static str = "SAttachableObjectContainerData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            file_name: inst
                .get_str("fileName")
                .map(String::from)
                .unwrap_or_default(),
            guid: inst.get_str("guid").map(String::from).unwrap_or_default(),
            offset: match inst.get("Offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAttachableObjectContainerSpawnerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SAttachableObjectContainerSpawnerParams {
    /// `ObjectContainers` (Class (array))
    pub object_containers: Vec<Handle<SAttachableObjectContainerData>>,
}

impl Pooled for SAttachableObjectContainerSpawnerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_attachableobjectcontainerspawner
            .sattachable_object_container_spawner_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_attachableobjectcontainerspawner
            .sattachable_object_container_spawner_params
    }
}

impl<'a> Extract<'a> for SAttachableObjectContainerSpawnerParams {
    const TYPE_NAME: &'static str = "SAttachableObjectContainerSpawnerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            object_containers: inst
                .get_array("ObjectContainers")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SAttachableObjectContainerData>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<SAttachableObjectContainerData>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
