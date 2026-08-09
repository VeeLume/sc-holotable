// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `instancestreaminghelper`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `InstanceStreamingHelperParams`
/// Inherits from: `DataForgeComponentParams`
pub struct InstanceStreamingHelperParams {}

impl Pooled for InstanceStreamingHelperParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .instancestreaminghelper
            .instance_streaming_helper_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .instancestreaminghelper
            .instance_streaming_helper_params
    }
}

impl<'a> Extract<'a> for InstanceStreamingHelperParams {
    const TYPE_NAME: &'static str = "InstanceStreamingHelperParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}
