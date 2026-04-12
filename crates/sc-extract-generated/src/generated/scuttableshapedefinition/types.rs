// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `scuttableshapedefinition`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SCuttableShapeDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCuttableShapeDefinition {
    /// `shapeNamePrefix` (String)
    #[serde(default)]
    pub shape_name_prefix: String,
}

impl Pooled for SCuttableShapeDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.scuttableshapedefinition.scuttable_shape_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.scuttableshapedefinition.scuttable_shape_definition }
}

impl<'a> Extract<'a> for SCuttableShapeDefinition {
    const TYPE_NAME: &'static str = "SCuttableShapeDefinition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            shape_name_prefix: inst.get_str("shapeNamePrefix").map(String::from).unwrap_or_default(),
        }
    }
}

