// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-instancedinteriormanager`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `InstancedInteriorManagerComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct InstancedInteriorManagerComponentParams {
    /// `doorTimeoutSeconds` (Single)
    pub door_timeout_seconds: f32,
    /// `transportTagsFilter` (Class)
    pub transport_tags_filter: Option<Handle<TagsDNFTerm>>,
}

impl Pooled for InstancedInteriorManagerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_instancedinteriormanager
            .instanced_interior_manager_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_instancedinteriormanager
            .instanced_interior_manager_component_params
    }
}

impl<'a> Extract<'a> for InstancedInteriorManagerComponentParams {
    const TYPE_NAME: &'static str = "InstancedInteriorManagerComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            door_timeout_seconds: inst.get_f32("doorTimeoutSeconds").unwrap_or_default(),
            transport_tags_filter: match inst.get("transportTagsFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}
