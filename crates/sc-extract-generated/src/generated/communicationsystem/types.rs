// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `communicationsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `CommunicationChannelConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationChannelConfig {
    /// `ChannelConfigName` (String)
    #[serde(default)]
    pub channel_config_name: String,
    /// `Channels` (Class (array))
    #[serde(default)]
    pub channels: Vec<Handle<CommunicationChannel>>,
}

impl Pooled for CommunicationChannelConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationsystem.communication_channel_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationsystem.communication_channel_config }
}

impl<'a> Extract<'a> for CommunicationChannelConfig {
    const TYPE_NAME: &'static str = "CommunicationChannelConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            channel_config_name: inst.get_str("ChannelConfigName").map(String::from).unwrap_or_default(),
            channels: inst.get_array("Channels")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationChannel>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<CommunicationChannel>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationChannel`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationChannel {
    /// `name` (Reference)
    #[serde(default)]
    pub name: Option<CigGuid>,
    /// `audioEventForExternalSources` (String)
    #[serde(default)]
    pub audio_event_for_external_sources: String,
    /// `minSilence` (Single)
    #[serde(default)]
    pub min_silence: f32,
    /// `flushSilence` (Single)
    #[serde(default)]
    pub flush_silence: f32,
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: eCommunicationChannelType,
    /// `priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// `animationPriorityOverride` (Int32)
    #[serde(default)]
    pub animation_priority_override: i32,
    /// `minSpeakerSilence` (Single)
    #[serde(default)]
    pub min_speaker_silence: f32,
    /// `ignoreSpeakerSilence` (Boolean)
    #[serde(default)]
    pub ignore_speaker_silence: bool,
    /// `subtitles` (Class)
    #[serde(default)]
    pub subtitles: Option<Handle<CommunicationSubtitleSettings>>,
    /// `audioRTPC` (StrongPointer)
    #[serde(default)]
    pub audio_rtpc: Option<Handle<CommunicationAudioRTPC>>,
    /// `subChannels` (Class (array))
    #[serde(default)]
    pub sub_channels: Vec<Handle<CommunicationChannel>>,
}

impl Pooled for CommunicationChannel {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationsystem.communication_channel }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationsystem.communication_channel }
}

impl<'a> Extract<'a> for CommunicationChannel {
    const TYPE_NAME: &'static str = "CommunicationChannel";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get("name").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            audio_event_for_external_sources: inst.get_str("audioEventForExternalSources").map(String::from).unwrap_or_default(),
            min_silence: inst.get_f32("minSilence").unwrap_or_default(),
            flush_silence: inst.get_f32("flushSilence").unwrap_or_default(),
            r#type: eCommunicationChannelType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            priority: inst.get_i32("priority").unwrap_or_default(),
            animation_priority_override: inst.get_i32("animationPriorityOverride").unwrap_or_default(),
            min_speaker_silence: inst.get_f32("minSpeakerSilence").unwrap_or_default(),
            ignore_speaker_silence: inst.get_bool("ignoreSpeakerSilence").unwrap_or_default(),
            subtitles: match inst.get("subtitles") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommunicationSubtitleSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_rtpc: match inst.get("audioRTPC") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommunicationAudioRTPC>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            sub_channels: inst.get_array("subChannels")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationChannel>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<CommunicationChannel>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationSubtitleSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationSubtitleSettings {
    /// `allow` (Boolean)
    #[serde(default)]
    pub allow: bool,
    /// `variableName` (String)
    #[serde(default)]
    pub variable_name: String,
}

impl Pooled for CommunicationSubtitleSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationsystem.communication_subtitle_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationsystem.communication_subtitle_settings }
}

impl<'a> Extract<'a> for CommunicationSubtitleSettings {
    const TYPE_NAME: &'static str = "CommunicationSubtitleSettings";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            allow: inst.get_bool("allow").unwrap_or_default(),
            variable_name: inst.get_str("variableName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationAudioRTPC`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationAudioRTPC {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `value` (Single)
    #[serde(default)]
    pub value: f32,
}

impl Pooled for CommunicationAudioRTPC {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationsystem.communication_audio_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationsystem.communication_audio_rtpc }
}

impl<'a> Extract<'a> for CommunicationAudioRTPC {
    const TYPE_NAME: &'static str = "CommunicationAudioRTPC";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            value: inst.get_f32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationLocationAutoTags`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationLocationAutoTags {
    /// `starMapObject` (Reference)
    #[serde(default)]
    pub star_map_object: Option<CigGuid>,
    /// `actorInLocationMannequinTags` (String)
    #[serde(default)]
    pub actor_in_location_mannequin_tags: String,
    /// `availableConversationTopics` (Reference (array))
    #[serde(default)]
    pub available_conversation_topics: Vec<CigGuid>,
    /// `conversationTopicsToExclude` (Reference (array))
    #[serde(default)]
    pub conversation_topics_to_exclude: Vec<CigGuid>,
}

impl Pooled for CommunicationLocationAutoTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationsystem.communication_location_auto_tags }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationsystem.communication_location_auto_tags }
}

impl<'a> Extract<'a> for CommunicationLocationAutoTags {
    const TYPE_NAME: &'static str = "CommunicationLocationAutoTags";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            star_map_object: inst.get("starMapObject").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            actor_in_location_mannequin_tags: inst.get_str("actorInLocationMannequinTags").map(String::from).unwrap_or_default(),
            available_conversation_topics: inst.get_array("availableConversationTopics")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            conversation_topics_to_exclude: inst.get_array("conversationTopicsToExclude")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationAutoMannequinTagsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationAutoMannequinTagsConfig {
    /// `actorIsPlayerFragmentTags` (String)
    #[serde(default)]
    pub actor_is_player_fragment_tags: String,
    /// `targetIsPlayerFragmentTags` (String)
    #[serde(default)]
    pub target_is_player_fragment_tags: String,
    /// `targetIsAllyFragmentTags` (String)
    #[serde(default)]
    pub target_is_ally_fragment_tags: String,
    /// `targetIsNeutralFragmentTags` (String)
    #[serde(default)]
    pub target_is_neutral_fragment_tags: String,
    /// `targetIsEnemyFragmentTags` (String)
    #[serde(default)]
    pub target_is_enemy_fragment_tags: String,
    /// `subjectIsPlayerFragmentTags` (String)
    #[serde(default)]
    pub subject_is_player_fragment_tags: String,
    /// `subjectIsAllyFragmentTags` (String)
    #[serde(default)]
    pub subject_is_ally_fragment_tags: String,
    /// `subjectIsNeutralFragmentTags` (String)
    #[serde(default)]
    pub subject_is_neutral_fragment_tags: String,
    /// `subjectIsEnemyFragmentTags` (String)
    #[serde(default)]
    pub subject_is_enemy_fragment_tags: String,
    /// `subjectIsDisguisedFragmentTags` (String)
    #[serde(default)]
    pub subject_is_disguised_fragment_tags: String,
    /// `locationsAutoTags` (Class (array))
    #[serde(default)]
    pub locations_auto_tags: Vec<Handle<CommunicationLocationAutoTags>>,
}

impl Pooled for CommunicationAutoMannequinTagsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationsystem.communication_auto_mannequin_tags_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationsystem.communication_auto_mannequin_tags_config }
}

impl<'a> Extract<'a> for CommunicationAutoMannequinTagsConfig {
    const TYPE_NAME: &'static str = "CommunicationAutoMannequinTagsConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actor_is_player_fragment_tags: inst.get_str("actorIsPlayerFragmentTags").map(String::from).unwrap_or_default(),
            target_is_player_fragment_tags: inst.get_str("targetIsPlayerFragmentTags").map(String::from).unwrap_or_default(),
            target_is_ally_fragment_tags: inst.get_str("targetIsAllyFragmentTags").map(String::from).unwrap_or_default(),
            target_is_neutral_fragment_tags: inst.get_str("targetIsNeutralFragmentTags").map(String::from).unwrap_or_default(),
            target_is_enemy_fragment_tags: inst.get_str("targetIsEnemyFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_player_fragment_tags: inst.get_str("subjectIsPlayerFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_ally_fragment_tags: inst.get_str("subjectIsAllyFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_neutral_fragment_tags: inst.get_str("subjectIsNeutralFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_enemy_fragment_tags: inst.get_str("subjectIsEnemyFragmentTags").map(String::from).unwrap_or_default(),
            subject_is_disguised_fragment_tags: inst.get_str("subjectIsDisguisedFragmentTags").map(String::from).unwrap_or_default(),
            locations_auto_tags: inst.get_array("locationsAutoTags")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationLocationAutoTags>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<CommunicationLocationAutoTags>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

