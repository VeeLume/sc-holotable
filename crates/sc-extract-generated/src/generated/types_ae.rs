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

/// DCB type: `AerodynamicTrailCalculation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AerodynamicTrailCalculation {
}

impl Pooled for AerodynamicTrailCalculation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_ae.aerodynamic_trail_calculation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_ae.aerodynamic_trail_calculation }
}

impl<'a> Extract<'a> for AerodynamicTrailCalculation {
    const TYPE_NAME: &'static str = "AerodynamicTrailCalculation";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

