// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `chatemoterecord`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ChatEmoteRecord`
pub struct ChatEmoteRecord {
    /// `packs` (Class (array))
    pub packs: Vec<Handle<ChatEmotePack>>,
}

impl Pooled for ChatEmoteRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatemoterecord.chat_emote_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatemoterecord.chat_emote_record }
}

impl<'a> Extract<'a> for ChatEmoteRecord {
    const TYPE_NAME: &'static str = "ChatEmoteRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            packs: inst.get_array("packs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatEmotePack>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ChatEmotePack>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatEmotePack`
pub struct ChatEmotePack {
    /// `packType` (String)
    pub pack_type: String,
    /// `emotes` (Class (array))
    pub emotes: Vec<Handle<ChatEmoteData>>,
}

impl Pooled for ChatEmotePack {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatemoterecord.chat_emote_pack }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatemoterecord.chat_emote_pack }
}

impl<'a> Extract<'a> for ChatEmotePack {
    const TYPE_NAME: &'static str = "ChatEmotePack";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pack_type: inst.get_str("packType").map(String::from).unwrap_or_default(),
            emotes: inst.get_array("emotes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatEmoteData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ChatEmoteData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatEmoteData`
pub struct ChatEmoteData {
    /// `emoteType` (Locale)
    pub emote_type: LocaleKey,
    /// `alternateEmoteTypes` (Locale (array))
    pub alternate_emote_types: Vec<LocaleKey>,
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `isInterruptable` (Boolean)
    pub is_interruptable: bool,
    /// `animData` (Class)
    pub anim_data: Option<Handle<ChatEmoteAnimData>>,
}

impl Pooled for ChatEmoteData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatemoterecord.chat_emote_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatemoterecord.chat_emote_data }
}

impl<'a> Extract<'a> for ChatEmoteData {
    const TYPE_NAME: &'static str = "ChatEmoteData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            emote_type: inst.get_str("emoteType").map(LocaleKey::from).unwrap_or_default(),
            alternate_emote_types: inst.get_array("alternateEmoteTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(LocaleKey::from)).collect())
                .unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            is_interruptable: inst.get_bool("isInterruptable").unwrap_or_default(),
            anim_data: match inst.get("animData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ChatEmoteAnimData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ChatEmoteAnimData`
pub struct ChatEmoteAnimData {
    /// `fragmentID` (String)
    pub fragment_id: String,
    /// `tagID` (String)
    pub tag_id: String,
    /// `textToDisplay` (Locale)
    pub text_to_display: LocaleKey,
    /// `type` (EnumChoice)
    pub r#type: EChatEmoteType,
}

impl Pooled for ChatEmoteAnimData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatemoterecord.chat_emote_anim_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatemoterecord.chat_emote_anim_data }
}

impl<'a> Extract<'a> for ChatEmoteAnimData {
    const TYPE_NAME: &'static str = "ChatEmoteAnimData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fragment_id: inst.get_str("fragmentID").map(String::from).unwrap_or_default(),
            tag_id: inst.get_str("tagID").map(String::from).unwrap_or_default(),
            text_to_display: inst.get_str("textToDisplay").map(LocaleKey::from).unwrap_or_default(),
            r#type: EChatEmoteType::from_dcb_str(inst.get_str("type").unwrap_or("")),
        }
    }
}

