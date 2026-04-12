// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `NumResultsConstraints`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumResultsConstraints {
    /// DCB field: `minResults` (Int32)
    #[serde(default)]
    pub min_results: i32,
    /// DCB field: `maxResults` (Int32)
    #[serde(default)]
    pub max_results: i32,
}

impl Pooled for NumResultsConstraints {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_nu.num_results_constraints }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_nu.num_results_constraints }
}

impl<'a> Extract<'a> for NumResultsConstraints {
    const TYPE_NAME: &'static str = "NumResultsConstraints";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            min_results: inst.get_i32("minResults").unwrap_or_default(),
            max_results: inst.get_i32("maxResults").unwrap_or_default(),
        }
    }
}

