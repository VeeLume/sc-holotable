// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-shadowregionentity`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ShadowRegionEntityComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct ShadowRegionEntityComponentParams {
    /// `sizeTrigger` (Single)
    pub size_trigger: f32,
    /// `regionSize` (Class)
    pub region_size: Option<Handle<Vec3>>,
}

impl Pooled for ShadowRegionEntityComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_shadowregionentity
            .shadow_region_entity_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_shadowregionentity
            .shadow_region_entity_component_params
    }
}

impl<'a> Extract<'a> for ShadowRegionEntityComponentParams {
    const TYPE_NAME: &'static str = "ShadowRegionEntityComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            size_trigger: inst.get_f32("sizeTrigger").unwrap_or_default(),
            region_size: match inst.get("regionSize") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}
