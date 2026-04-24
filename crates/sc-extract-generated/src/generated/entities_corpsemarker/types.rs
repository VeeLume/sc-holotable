// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-corpsemarker`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SEntityComponentCorpseMarkerParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SEntityComponentCorpseMarkerParams {
    /// `markerConfig` (Reference)
    pub marker_config: Option<CigGuid>,
}

impl Pooled for SEntityComponentCorpseMarkerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_corpsemarker
            .sentity_component_corpse_marker_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_corpsemarker
            .sentity_component_corpse_marker_params
    }
}

impl<'a> Extract<'a> for SEntityComponentCorpseMarkerParams {
    const TYPE_NAME: &'static str = "SEntityComponentCorpseMarkerParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            marker_config: inst
                .get("markerConfig")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}
