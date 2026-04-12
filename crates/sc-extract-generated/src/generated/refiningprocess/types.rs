// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `refiningprocess`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `RefiningProcess`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefiningProcess {
    /// `refiningSpeed` (EnumChoice)
    #[serde(default)]
    pub refining_speed: String,
    /// `refiningQuality` (EnumChoice)
    #[serde(default)]
    pub refining_quality: String,
    /// `processName` (Locale)
    #[serde(default)]
    pub process_name: String,
}

impl Pooled for RefiningProcess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.refiningprocess.refining_process }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.refiningprocess.refining_process }
}

impl<'a> Extract<'a> for RefiningProcess {
    const TYPE_NAME: &'static str = "RefiningProcess";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            refining_speed: inst.get_str("refiningSpeed").map(String::from).unwrap_or_default(),
            refining_quality: inst.get_str("refiningQuality").map(String::from).unwrap_or_default(),
            process_name: inst.get_str("processName").map(String::from).unwrap_or_default(),
        }
    }
}

