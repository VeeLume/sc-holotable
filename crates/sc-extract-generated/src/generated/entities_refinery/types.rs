// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-refinery`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `EntityComponentRefineryParams`
/// Inherits from: `DataForgeComponentParams`
pub struct EntityComponentRefineryParams {
}

impl Pooled for EntityComponentRefineryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_refinery.entity_component_refinery_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_refinery.entity_component_refinery_params }
}

impl<'a> Extract<'a> for EntityComponentRefineryParams {
    const TYPE_NAME: &'static str = "EntityComponentRefineryParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

