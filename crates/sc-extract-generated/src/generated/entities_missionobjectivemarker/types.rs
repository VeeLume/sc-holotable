// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-missionobjectivemarker`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SMissionEntryTrackerParams`
/// Inherits from: `SObjectDataBankEntryTrackerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMissionEntryTrackerParams {
}

impl Pooled for SMissionEntryTrackerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_missionobjectivemarker.smission_entry_tracker_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_missionobjectivemarker.smission_entry_tracker_params }
}

impl<'a> Extract<'a> for SMissionEntryTrackerParams {
    const TYPE_NAME: &'static str = "SMissionEntryTrackerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

