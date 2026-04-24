// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-partymembermarker`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SPartyMemberMarkerObjectMetadataParams`
/// Inherits from: `SObjectMetadataParams`
pub struct SPartyMemberMarkerObjectMetadataParams {}

impl Pooled for SPartyMemberMarkerObjectMetadataParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_partymembermarker
            .sparty_member_marker_object_metadata_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_partymembermarker
            .sparty_member_marker_object_metadata_params
    }
}

impl<'a> Extract<'a> for SPartyMemberMarkerObjectMetadataParams {
    const TYPE_NAME: &'static str = "SPartyMemberMarkerObjectMetadataParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SEntityComponentPartyMarkerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentPartyMarkerParams {}

impl Pooled for SEntityComponentPartyMarkerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_partymembermarker
            .sentity_component_party_marker_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_partymembermarker
            .sentity_component_party_marker_params
    }
}

impl<'a> Extract<'a> for SEntityComponentPartyMarkerParams {
    const TYPE_NAME: &'static str = "SEntityComponentPartyMarkerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SPartyMemberEntryTrackerParams`
/// Inherits from: `SObjectDataBankEntryTrackerParams`
pub struct SPartyMemberEntryTrackerParams {}

impl Pooled for SPartyMemberEntryTrackerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_partymembermarker
            .sparty_member_entry_tracker_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_partymembermarker
            .sparty_member_entry_tracker_params
    }
}

impl<'a> Extract<'a> for SPartyMemberEntryTrackerParams {
    const TYPE_NAME: &'static str = "SPartyMemberEntryTrackerParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
