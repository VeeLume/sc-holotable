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

/// DCB type: `SOperatorModeLabels`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOperatorModeLabels {
    /// DCB field: `fullNames` (Locale)
    #[serde(default)]
    pub full_names: String,
    /// DCB field: `shortNames` (Locale)
    #[serde(default)]
    pub short_names: String,
}

impl Pooled for SOperatorModeLabels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_so.soperator_mode_labels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_so.soperator_mode_labels }
}

impl<'a> Extract<'a> for SOperatorModeLabels {
    const TYPE_NAME: &'static str = "SOperatorModeLabels";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            full_names: inst.get_str("fullNames").map(String::from).unwrap_or_default(),
            short_names: inst.get_str("shortNames").map(String::from).unwrap_or_default(),
        }
    }
}

