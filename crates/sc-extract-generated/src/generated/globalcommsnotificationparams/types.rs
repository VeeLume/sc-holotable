// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `globalcommsnotificationparams`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `CommsNotificationsGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommsNotificationsGlobalParams {
    /// `channelName` (Reference)
    #[serde(default)]
    pub channel_name: Option<CigGuid>,
    /// `expiry` (Single)
    #[serde(default)]
    pub expiry: f32,
    /// `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// `fakeCommsAudioEntityClass3D` (Reference)
    #[serde(default)]
    pub fake_comms_audio_entity_class3_d: Option<CigGuid>,
}

impl Pooled for CommsNotificationsGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.globalcommsnotificationparams.comms_notifications_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.globalcommsnotificationparams.comms_notifications_global_params }
}

impl<'a> Extract<'a> for CommsNotificationsGlobalParams {
    const TYPE_NAME: &'static str = "CommsNotificationsGlobalParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            channel_name: inst.get("channelName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            expiry: inst.get_f32("expiry").unwrap_or_default(),
            priority: inst.get_i32("priority").unwrap_or_default(),
            fake_comms_audio_entity_class3_d: inst.get("fakeCommsAudioEntityClass3D").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

