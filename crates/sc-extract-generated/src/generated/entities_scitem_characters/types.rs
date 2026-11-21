// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-characters`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SEntityAttachableParentContext`
/// Inherits from: `SEntityContextBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct SEntityAttachableParentContext {}

impl Pooled for SEntityAttachableParentContext {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_characters
            .sentity_attachable_parent_context
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_characters
            .sentity_attachable_parent_context
    }
}

impl<'a> Extract<'a> for SEntityAttachableParentContext {
    const TYPE_NAME: &'static str = "SEntityAttachableParentContext";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
