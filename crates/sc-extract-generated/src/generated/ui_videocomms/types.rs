// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-videocomms`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `VideoComms`
pub struct VideoComms {
    /// `filename169` (String)
    pub filename169: String,
    /// `filename43` (String)
    pub filename43: String,
}

impl Pooled for VideoComms {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ui_videocomms.video_comms
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ui_videocomms.video_comms
    }
}

impl<'a> Extract<'a> for VideoComms {
    const TYPE_NAME: &'static str = "VideoComms";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filename169: inst
                .get_str("filename169")
                .map(String::from)
                .unwrap_or_default(),
            filename43: inst
                .get_str("filename43")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}
