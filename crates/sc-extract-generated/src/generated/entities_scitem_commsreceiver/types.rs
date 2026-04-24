// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-commsreceiver`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SCItemCommsReceiverComponentParams`
/// Inherits from: `SCItemCommsComponentParams`
pub struct SCItemCommsReceiverComponentParams {
    /// `setup` (Reference)
    pub setup: Option<CigGuid>,
    /// `commsChannels` (Reference (array))
    pub comms_channels: Vec<CigGuid>,
    /// `audioCommsEffectOverride` (String)
    pub audio_comms_effect_override: String,
}

impl Pooled for SCItemCommsReceiverComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_commsreceiver
            .scitem_comms_receiver_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_commsreceiver
            .scitem_comms_receiver_component_params
    }
}

impl<'a> Extract<'a> for SCItemCommsReceiverComponentParams {
    const TYPE_NAME: &'static str = "SCItemCommsReceiverComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            setup: inst
                .get("setup")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            comms_channels: inst
                .get_array("commsChannels")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            audio_comms_effect_override: inst
                .get_str("audioCommsEffectOverride")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}
