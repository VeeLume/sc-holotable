// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-ledgeobject`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `NavLinkLedgeLocation`
/// Inherits from: `NavLinkLocation`
pub struct NavLinkLedgeLocation {
    /// `relativeTransform` (Class)
    pub relative_transform: Option<Handle<QuatT>>,
}

impl Pooled for NavLinkLedgeLocation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_ledgeobject.nav_link_ledge_location
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_ledgeobject.nav_link_ledge_location
    }
}

impl<'a> Extract<'a> for NavLinkLedgeLocation {
    const TYPE_NAME: &'static str = "NavLinkLedgeLocation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            relative_transform: match inst.get("relativeTransform") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}
