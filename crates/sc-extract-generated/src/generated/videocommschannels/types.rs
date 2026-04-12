// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `videocommschannels`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CommsChannelDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsChannelDef {
    /// `channelId` (String)
    #[serde(default)]
    pub channel_id: String,
    /// `displayName` (Locale)
    #[serde(default)]
    pub display_name: String,
    /// `canHaveMultipleInstances` (Boolean)
    #[serde(default)]
    pub can_have_multiple_instances: bool,
    /// `useAreaOfEffect` (Boolean)
    #[serde(default)]
    pub use_area_of_effect: bool,
}

impl Pooled for CommsChannelDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.videocommschannels.comms_channel_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.videocommschannels.comms_channel_def }
}

impl<'a> Extract<'a> for CommsChannelDef {
    const TYPE_NAME: &'static str = "CommsChannelDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            channel_id: inst.get_str("channelId").map(String::from).unwrap_or_default(),
            display_name: inst.get_str("displayName").map(String::from).unwrap_or_default(),
            can_have_multiple_instances: inst.get_bool("canHaveMultipleInstances").unwrap_or_default(),
            use_area_of_effect: inst.get_bool("useAreaOfEffect").unwrap_or_default(),
        }
    }
}

