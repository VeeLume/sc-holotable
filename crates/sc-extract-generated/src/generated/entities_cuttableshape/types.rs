// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-cuttableshape`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `GameShapeComponentParams`
/// Inherits from: `AreaShapeComponentParams`
pub struct GameShapeComponentParams {
}

impl Pooled for GameShapeComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_cuttableshape.game_shape_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_cuttableshape.game_shape_component_params }
}

impl<'a> Extract<'a> for GameShapeComponentParams {
    const TYPE_NAME: &'static str = "GameShapeComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

