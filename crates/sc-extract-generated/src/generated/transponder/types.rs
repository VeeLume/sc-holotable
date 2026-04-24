// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `transponder`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `STransponderObjectMetadataParams`
/// Inherits from: `SObjectMetadataParams`
pub struct STransponderObjectMetadataParams {}

impl Pooled for STransponderObjectMetadataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transponder.stransponder_object_metadata_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transponder.stransponder_object_metadata_params
    }
}

impl<'a> Extract<'a> for STransponderObjectMetadataParams {
    const TYPE_NAME: &'static str = "STransponderObjectMetadataParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `STransponderEntryTrackerParams`
/// Inherits from: `SObjectDataBankEntryTrackerParams`
pub struct STransponderEntryTrackerParams {}

impl Pooled for STransponderEntryTrackerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.transponder.stransponder_entry_tracker_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.transponder.stransponder_entry_tracker_params
    }
}

impl<'a> Extract<'a> for STransponderEntryTrackerParams {
    const TYPE_NAME: &'static str = "STransponderEntryTrackerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
