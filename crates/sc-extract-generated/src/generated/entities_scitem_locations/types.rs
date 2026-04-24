// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-locations`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ItemResourceContainerPlaceholderParams`
/// Inherits from: `DataForgeComponentParams`
pub struct ItemResourceContainerPlaceholderParams {
    /// `capacity` (Single)
    pub capacity: f32,
    /// `defaultFullness` (Single)
    pub default_fullness: f32,
    /// `composition` (Class)
    pub composition: Option<Handle<ItemResourceComposition>>,
}

impl Pooled for ItemResourceContainerPlaceholderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_locations
            .item_resource_container_placeholder_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_locations
            .item_resource_container_placeholder_params
    }
}

impl<'a> Extract<'a> for ItemResourceContainerPlaceholderParams {
    const TYPE_NAME: &'static str = "ItemResourceContainerPlaceholderParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            capacity: inst.get_f32("capacity").unwrap_or_default(),
            default_fullness: inst.get_f32("defaultFullness").unwrap_or_default(),
            composition: match inst.get("composition") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ItemResourceComposition>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}
