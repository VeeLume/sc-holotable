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

/// DCB type: `ThrowParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrowParams {
    /// DCB field: `force` (Single)
    #[serde(default)]
    pub force: f32,
    /// DCB field: `minSpeed` (Single)
    #[serde(default)]
    pub min_speed: f32,
    /// DCB field: `maxSpeed` (Single)
    #[serde(default)]
    pub max_speed: f32,
}

impl Pooled for ThrowParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_th.throw_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_th.throw_params }
}

impl<'a> Extract<'a> for ThrowParams {
    const TYPE_NAME: &'static str = "ThrowParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            force: inst.get_f32("force").unwrap_or_default(),
            min_speed: inst.get_f32("minSpeed").unwrap_or_default(),
            max_speed: inst.get_f32("maxSpeed").unwrap_or_default(),
        }
    }
}

