// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `communicationconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CommunicationConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationConfig {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `communications` (Class (array))
    #[serde(default)]
    pub communications: Vec<Handle<CommunicationEntry>>,
    /// `subConfigs` (Reference (array))
    #[serde(default)]
    pub sub_configs: Vec<CigGuid>,
}

impl Pooled for CommunicationConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationconfig.communication_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationconfig.communication_config }
}

impl<'a> Extract<'a> for CommunicationConfig {
    const TYPE_NAME: &'static str = "CommunicationConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            communications: inst.get_array("communications")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            sub_configs: inst.get_array("subConfigs")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationEntry {
    /// `name` (Reference)
    #[serde(default)]
    pub name: Option<CigGuid>,
    /// `channelName` (Reference)
    #[serde(default)]
    pub channel_name: Option<CigGuid>,
    /// `method` (EnumChoice)
    #[serde(default)]
    pub method: String,
    /// `forceAnimation` (Boolean)
    #[serde(default)]
    pub force_animation: bool,
    /// `variations` (Class (array))
    #[serde(default)]
    pub variations: Vec<Handle<CommunicationVariation>>,
    /// `entityRetriggerDelay` (Single)
    #[serde(default)]
    pub entity_retrigger_delay: f32,
    /// `channelRetriggerDelay` (Single)
    #[serde(default)]
    pub channel_retrigger_delay: f32,
    /// `gameTags` (Reference (array))
    #[serde(default)]
    pub game_tags: Vec<CigGuid>,
}

impl Pooled for CommunicationEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationconfig.communication_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationconfig.communication_entry }
}

impl<'a> Extract<'a> for CommunicationEntry {
    const TYPE_NAME: &'static str = "CommunicationEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get("name").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            channel_name: inst.get("channelName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            method: inst.get_str("method").map(String::from).unwrap_or_default(),
            force_animation: inst.get_bool("forceAnimation").unwrap_or_default(),
            variations: inst.get_array("variations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationVariation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationVariation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            entity_retrigger_delay: inst.get_f32("entityRetriggerDelay").unwrap_or_default(),
            channel_retrigger_delay: inst.get_f32("channelRetriggerDelay").unwrap_or_default(),
            game_tags: inst.get_array("gameTags")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationVariation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariation {
    /// `animationFragmentId` (String)
    #[serde(default)]
    pub animation_fragment_id: String,
    /// `animationFragmentTags` (String)
    #[serde(default)]
    pub animation_fragment_tags: String,
    /// `soundName` (String)
    #[serde(default)]
    pub sound_name: String,
    /// `overrideMinSilence` (Single)
    #[serde(default)]
    pub override_min_silence: f32,
    /// `overrideMinSpeakerSilence` (Single)
    #[serde(default)]
    pub override_min_speaker_silence: f32,
    /// `dialogueContext` (Reference)
    #[serde(default)]
    pub dialogue_context: Option<CigGuid>,
    /// `playDialogueAsAnimation` (Boolean)
    #[serde(default)]
    pub play_dialogue_as_animation: bool,
    /// `dialogueExternalSource` (Reference)
    #[serde(default)]
    pub dialogue_external_source: Option<CigGuid>,
    /// `rules` (Class)
    #[serde(default)]
    pub rules: Option<Handle<CommunicationVariationRules>>,
    /// `conditions` (Class)
    #[serde(default)]
    pub conditions: Option<Handle<CommunicationVariationCondition>>,
}

impl Pooled for CommunicationVariation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationconfig.communication_variation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationconfig.communication_variation }
}

impl<'a> Extract<'a> for CommunicationVariation {
    const TYPE_NAME: &'static str = "CommunicationVariation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            animation_fragment_id: inst.get_str("animationFragmentId").map(String::from).unwrap_or_default(),
            animation_fragment_tags: inst.get_str("animationFragmentTags").map(String::from).unwrap_or_default(),
            sound_name: inst.get_str("soundName").map(String::from).unwrap_or_default(),
            override_min_silence: inst.get_f32("overrideMinSilence").unwrap_or_default(),
            override_min_speaker_silence: inst.get_f32("overrideMinSpeakerSilence").unwrap_or_default(),
            dialogue_context: inst.get("dialogueContext").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            play_dialogue_as_animation: inst.get_bool("playDialogueAsAnimation").unwrap_or_default(),
            dialogue_external_source: inst.get("dialogueExternalSource").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rules: match inst.get("rules") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommunicationVariationRules>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommunicationVariationRules>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            conditions: match inst.get("conditions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CommunicationVariationCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CommunicationVariationCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `CommunicationVariationRules`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariationRules {
    /// `timeout` (Single)
    #[serde(default)]
    pub timeout: f32,
    /// `lookAtTarget` (Boolean)
    #[serde(default)]
    pub look_at_target: bool,
    /// `finishAnimation` (Boolean)
    #[serde(default)]
    pub finish_animation: bool,
    /// `finishSound` (Boolean)
    #[serde(default)]
    pub finish_sound: bool,
    /// `finishVoice` (Boolean)
    #[serde(default)]
    pub finish_voice: bool,
    /// `finishTimeout` (Boolean)
    #[serde(default)]
    pub finish_timeout: bool,
    /// `blockMovement` (Boolean)
    #[serde(default)]
    pub block_movement: bool,
    /// `blockFire` (Boolean)
    #[serde(default)]
    pub block_fire: bool,
}

impl Pooled for CommunicationVariationRules {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationconfig.communication_variation_rules }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationconfig.communication_variation_rules }
}

impl<'a> Extract<'a> for CommunicationVariationRules {
    const TYPE_NAME: &'static str = "CommunicationVariationRules";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            timeout: inst.get_f32("timeout").unwrap_or_default(),
            look_at_target: inst.get_bool("lookAtTarget").unwrap_or_default(),
            finish_animation: inst.get_bool("finishAnimation").unwrap_or_default(),
            finish_sound: inst.get_bool("finishSound").unwrap_or_default(),
            finish_voice: inst.get_bool("finishVoice").unwrap_or_default(),
            finish_timeout: inst.get_bool("finishTimeout").unwrap_or_default(),
            block_movement: inst.get_bool("blockMovement").unwrap_or_default(),
            block_fire: inst.get_bool("blockFire").unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationVariationCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariationCondition {
    /// `expression` (String)
    #[serde(default)]
    pub expression: String,
    /// `conditionTags` (Class)
    #[serde(default)]
    pub condition_tags: Option<Handle<TagsDNF>>,
}

impl Pooled for CommunicationVariationCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationconfig.communication_variation_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationconfig.communication_variation_condition }
}

impl<'a> Extract<'a> for CommunicationVariationCondition {
    const TYPE_NAME: &'static str = "CommunicationVariationCondition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            expression: inst.get_str("expression").map(String::from).unwrap_or_default(),
            condition_tags: match inst.get("conditionTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagsDNF>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TagsDNF>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

