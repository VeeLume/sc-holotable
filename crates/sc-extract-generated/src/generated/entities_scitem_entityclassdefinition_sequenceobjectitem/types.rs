// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-entityclassdefinition_sequenceobjectitem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SEntityComponentTrackViewParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentTrackViewParams {}

impl Pooled for SEntityComponentTrackViewParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_entityclassdefinition_sequenceobjectitem
            .sentity_component_track_view_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_entityclassdefinition_sequenceobjectitem
            .sentity_component_track_view_params
    }
}

impl<'a> Extract<'a> for SEntityComponentTrackViewParams {
    const TYPE_NAME: &'static str = "SEntityComponentTrackViewParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `STrackviewInterruptComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct STrackviewInterruptComponentParams {}

impl Pooled for STrackviewInterruptComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_entityclassdefinition_sequenceobjectitem
            .strackview_interrupt_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_entityclassdefinition_sequenceobjectitem
            .strackview_interrupt_component_params
    }
}

impl<'a> Extract<'a> for STrackviewInterruptComponentParams {
    const TYPE_NAME: &'static str = "STrackviewInterruptComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
