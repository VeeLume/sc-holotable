// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `communicationsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunicationsystemPools {
    #[serde(default)]
    pub communication_channel_config: Vec<Option<CommunicationChannelConfig>>,
    #[serde(default)]
    pub communication_channel: Vec<Option<CommunicationChannel>>,
    #[serde(default)]
    pub communication_subtitle_settings: Vec<Option<CommunicationSubtitleSettings>>,
    #[serde(default)]
    pub communication_audio_rtpc: Vec<Option<CommunicationAudioRTPC>>,
    #[serde(default)]
    pub communication_location_auto_tags: Vec<Option<CommunicationLocationAutoTags>>,
    #[serde(default)]
    pub communication_auto_mannequin_tags_config: Vec<Option<CommunicationAutoMannequinTagsConfig>>,
}
