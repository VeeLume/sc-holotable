// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `announcer`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `AnnouncementGameToken`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementGameToken {
    /// `gameToken` (String)
    #[serde(default)]
    pub game_token: String,
    /// `gameTokenType` (EnumChoice)
    #[serde(default)]
    pub game_token_type: String,
}

impl Pooled for AnnouncementGameToken {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.announcer.announcement_game_token }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.announcer.announcement_game_token }
}

impl<'a> Extract<'a> for AnnouncementGameToken {
    const TYPE_NAME: &'static str = "AnnouncementGameToken";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            game_token: inst.get_str("gameToken").map(String::from).unwrap_or_default(),
            game_token_type: inst.get_str("gameTokenType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `Announcement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `conversation` (Reference)
    #[serde(default)]
    pub conversation: Option<CigGuid>,
    /// `gameToken` (String)
    #[serde(default)]
    pub game_token: String,
    /// `gameTokenType` (EnumChoice)
    #[serde(default)]
    pub game_token_type: String,
    /// `gameTokens` (Class (array))
    #[serde(default)]
    pub game_tokens: Vec<Handle<AnnouncementGameToken>>,
    /// `priority` (EnumChoice)
    #[serde(default)]
    pub priority: String,
    /// `retriggerDelay` (Single)
    #[serde(default)]
    pub retrigger_delay: f32,
    /// `playProbability` (Single)
    #[serde(default)]
    pub play_probability: f32,
    /// `playWhenDead` (Boolean)
    #[serde(default)]
    pub play_when_dead: bool,
    /// `playWhenSpectating` (Boolean)
    #[serde(default)]
    pub play_when_spectating: bool,
}

impl Pooled for Announcement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.announcer.announcement }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.announcer.announcement }
}

impl<'a> Extract<'a> for Announcement {
    const TYPE_NAME: &'static str = "Announcement";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            conversation: inst.get("conversation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            game_token: inst.get_str("gameToken").map(String::from).unwrap_or_default(),
            game_token_type: inst.get_str("gameTokenType").map(String::from).unwrap_or_default(),
            game_tokens: inst.get_array("gameTokens")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AnnouncementGameToken>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<AnnouncementGameToken>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            priority: inst.get_str("priority").map(String::from).unwrap_or_default(),
            retrigger_delay: inst.get_f32("retriggerDelay").unwrap_or_default(),
            play_probability: inst.get_f32("playProbability").unwrap_or_default(),
            play_when_dead: inst.get_bool("playWhenDead").unwrap_or_default(),
            play_when_spectating: inst.get_bool("playWhenSpectating").unwrap_or_default(),
        }
    }
}

/// DCB type: `Announcer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcer {
    /// `base` (Reference)
    #[serde(default)]
    pub base: Option<CigGuid>,
    /// `announcements` (Class (array))
    #[serde(default)]
    pub announcements: Vec<Handle<Announcement>>,
}

impl Pooled for Announcer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.announcer.announcer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.announcer.announcer }
}

impl<'a> Extract<'a> for Announcer {
    const TYPE_NAME: &'static str = "Announcer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base: inst.get("base").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            announcements: inst.get_array("announcements")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Announcement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<Announcement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

