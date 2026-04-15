// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `communicationsystem` feature.
#[derive(Default)]
pub struct CommunicationsystemPools {
    pub communication_channel_config: Vec<Option<CommunicationChannelConfig>>,
    pub communication_channel: Vec<Option<CommunicationChannel>>,
    pub communication_subtitle_settings: Vec<Option<CommunicationSubtitleSettings>>,
    pub communication_audio_rtpc: Vec<Option<CommunicationAudioRTPC>>,
    pub communication_location_auto_tags: Vec<Option<CommunicationLocationAutoTags>>,
    pub communication_auto_mannequin_tags_config: Vec<Option<CommunicationAutoMannequinTagsConfig>>,
}
