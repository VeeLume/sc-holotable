// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globaltutorialparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `GlobalTutorialParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalTutorialParams {
    /// `validStartingAreas` (Locale (array))
    #[serde(default)]
    pub valid_starting_areas: Vec<LocaleKey>,
}

impl Pooled for GlobalTutorialParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globaltutorialparams.global_tutorial_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globaltutorialparams.global_tutorial_params }
}

impl<'a> Extract<'a> for GlobalTutorialParams {
    const TYPE_NAME: &'static str = "GlobalTutorialParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            valid_starting_areas: inst.get_array("validStartingAreas")
                .map(|arr| arr.filter_map(|v| v.as_str().map(LocaleKey::from)).collect())
                .unwrap_or_default(),
        }
    }
}

