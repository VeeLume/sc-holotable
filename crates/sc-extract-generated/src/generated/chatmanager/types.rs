// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `chatmanager`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `ChatManagerDefaultChannelColor`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatManagerDefaultChannelColor {
    /// `global` (EnumChoice)
    #[serde(default)]
    pub global: String,
    /// `party` (EnumChoice)
    #[serde(default)]
    pub party: String,
    /// `gameEntity` (EnumChoice)
    #[serde(default)]
    pub game_entity: String,
    /// `whisper` (EnumChoice)
    #[serde(default)]
    pub whisper: String,
    /// `team` (EnumChoice)
    #[serde(default)]
    pub team: String,
    /// `squad` (EnumChoice)
    #[serde(default)]
    pub squad: String,
}

impl Pooled for ChatManagerDefaultChannelColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatmanager.chat_manager_default_channel_color }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatmanager.chat_manager_default_channel_color }
}

impl<'a> Extract<'a> for ChatManagerDefaultChannelColor {
    const TYPE_NAME: &'static str = "ChatManagerDefaultChannelColor";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            global: inst.get_str("global").map(String::from).unwrap_or_default(),
            party: inst.get_str("party").map(String::from).unwrap_or_default(),
            game_entity: inst.get_str("gameEntity").map(String::from).unwrap_or_default(),
            whisper: inst.get_str("whisper").map(String::from).unwrap_or_default(),
            team: inst.get_str("team").map(String::from).unwrap_or_default(),
            squad: inst.get_str("squad").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatManagerColor`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatManagerColor {
    /// `colorType` (EnumChoice)
    #[serde(default)]
    pub color_type: String,
    /// `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<SRGB8>>,
}

impl Pooled for ChatManagerColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatmanager.chat_manager_color }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatmanager.chat_manager_color }
}

impl<'a> Extract<'a> for ChatManagerColor {
    const TYPE_NAME: &'static str = "ChatManagerColor";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color_type: inst.get_str("colorType").map(String::from).unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGB8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ChatManagerGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatManagerGlobalParams {
    /// `defaultChannelColor` (Class)
    #[serde(default)]
    pub default_channel_color: Option<Handle<ChatManagerDefaultChannelColor>>,
    /// `colorOptions` (Class (array))
    #[serde(default)]
    pub color_options: Vec<Handle<ChatManagerColor>>,
}

impl Pooled for ChatManagerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatmanager.chat_manager_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatmanager.chat_manager_global_params }
}

impl<'a> Extract<'a> for ChatManagerGlobalParams {
    const TYPE_NAME: &'static str = "ChatManagerGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_channel_color: match inst.get("defaultChannelColor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ChatManagerDefaultChannelColor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ChatManagerDefaultChannelColor>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            color_options: inst.get_array("colorOptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatManagerColor>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ChatManagerColor>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

