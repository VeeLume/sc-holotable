// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-patrolgraph`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `PatrolGraphComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct PatrolGraphComponentParams {
}

impl Pooled for PatrolGraphComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_patrolgraph.patrol_graph_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_patrolgraph.patrol_graph_component_params }
}

impl<'a> Extract<'a> for PatrolGraphComponentParams {
    const TYPE_NAME: &'static str = "PatrolGraphComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

