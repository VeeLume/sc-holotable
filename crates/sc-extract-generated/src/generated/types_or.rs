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

/// DCB type: `OrificeBloodParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrificeBloodParams {
    /// DCB field: `bloodStatTriggerType` (EnumChoice)
    #[serde(default)]
    pub blood_stat_trigger_type: String,
    /// DCB field: `probabilistic` (Boolean)
    #[serde(default)]
    pub probabilistic: bool,
    /// DCB field: `bloodChance` (Single)
    #[serde(default)]
    pub blood_chance: f32,
    /// DCB field: `orifices` (EnumChoice (array))
    #[serde(default)]
    pub orifices: Vec<String>,
}

impl Pooled for OrificeBloodParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_or.orifice_blood_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_or.orifice_blood_params }
}

impl<'a> Extract<'a> for OrificeBloodParams {
    const TYPE_NAME: &'static str = "OrificeBloodParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            blood_stat_trigger_type: inst.get_str("bloodStatTriggerType").map(String::from).unwrap_or_default(),
            probabilistic: inst.get_bool("probabilistic").unwrap_or_default(),
            blood_chance: inst.get_f32("bloodChance").unwrap_or_default(),
            orifices: inst.get_array("orifices")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

