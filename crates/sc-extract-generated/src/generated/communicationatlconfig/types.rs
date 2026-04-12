// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `communicationatlconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CommunicationATLConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationATLConfig {
    /// `playTriggerPrefix` (String)
    #[serde(default)]
    pub play_trigger_prefix: String,
    /// `stopTriggerPrefix` (String)
    #[serde(default)]
    pub stop_trigger_prefix: String,
    /// `speakerVoiceSwitch` (String)
    #[serde(default)]
    pub speaker_voice_switch: String,
    /// `speakerTypeSwitch` (String)
    #[serde(default)]
    pub speaker_type_switch: String,
}

impl Pooled for CommunicationATLConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationatlconfig.communication_atlconfig }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationatlconfig.communication_atlconfig }
}

impl<'a> Extract<'a> for CommunicationATLConfig {
    const TYPE_NAME: &'static str = "CommunicationATLConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            play_trigger_prefix: inst.get_str("playTriggerPrefix").map(String::from).unwrap_or_default(),
            stop_trigger_prefix: inst.get_str("stopTriggerPrefix").map(String::from).unwrap_or_default(),
            speaker_voice_switch: inst.get_str("speakerVoiceSwitch").map(String::from).unwrap_or_default(),
            speaker_type_switch: inst.get_str("speakerTypeSwitch").map(String::from).unwrap_or_default(),
        }
    }
}

