// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `transportsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `TransportNavSplineParams`
/// Inherits from: `DataForgeComponentParams`
pub struct TransportNavSplineParams {
    /// `startTags` (Class)
    pub start_tags: Option<Handle<TagList>>,
    /// `startTagFilter` (Class)
    pub start_tag_filter: Option<Handle<TagsDNFTerm>>,
    /// `endTags` (Class)
    pub end_tags: Option<Handle<TagList>>,
    /// `endTagFilter` (Class)
    pub end_tag_filter: Option<Handle<TagsDNFTerm>>,
    /// `reversible` (Boolean)
    pub reversible: bool,
    /// `invertForward` (Boolean)
    pub invert_forward: bool,
}

impl Pooled for TransportNavSplineParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transportsystem.transport_nav_spline_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transportsystem.transport_nav_spline_params
    }
}

impl<'a> Extract<'a> for TransportNavSplineParams {
    const TYPE_NAME: &'static str = "TransportNavSplineParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_tags: match inst.get("startTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            start_tag_filter: match inst.get("startTagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            end_tags: match inst.get("endTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            end_tag_filter: match inst.get("endTagFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNFTerm>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            reversible: inst.get_bool("reversible").unwrap_or_default(),
            invert_forward: inst.get_bool("invertForward").unwrap_or_default(),
        }
    }
}
