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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ChatManagerDefaultChannelColor`
pub struct ChatManagerDefaultChannelColor {
    /// `global` (EnumChoice)
    pub global: ChannelColor,
    /// `party` (EnumChoice)
    pub party: ChannelColor,
    /// `gameEntity` (EnumChoice)
    pub game_entity: ChannelColor,
    /// `whisper` (EnumChoice)
    pub whisper: ChannelColor,
    /// `team` (EnumChoice)
    pub team: ChannelColor,
    /// `squad` (EnumChoice)
    pub squad: ChannelColor,
}

impl Pooled for ChatManagerDefaultChannelColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.chatmanager.chat_manager_default_channel_color
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.chatmanager.chat_manager_default_channel_color
    }
}

impl<'a> Extract<'a> for ChatManagerDefaultChannelColor {
    const TYPE_NAME: &'static str = "ChatManagerDefaultChannelColor";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            global: ChannelColor::from_dcb_str(inst.get_str("global").unwrap_or("")),
            party: ChannelColor::from_dcb_str(inst.get_str("party").unwrap_or("")),
            game_entity: ChannelColor::from_dcb_str(inst.get_str("gameEntity").unwrap_or("")),
            whisper: ChannelColor::from_dcb_str(inst.get_str("whisper").unwrap_or("")),
            team: ChannelColor::from_dcb_str(inst.get_str("team").unwrap_or("")),
            squad: ChannelColor::from_dcb_str(inst.get_str("squad").unwrap_or("")),
        }
    }
}

/// DCB type: `ChatManagerColor`
pub struct ChatManagerColor {
    /// `colorType` (EnumChoice)
    pub color_type: ChannelColor,
    /// `color` (Class)
    pub color: Option<Handle<SRGB8>>,
}

impl Pooled for ChatManagerColor {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.chatmanager.chat_manager_color
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.chatmanager.chat_manager_color
    }
}

impl<'a> Extract<'a> for ChatManagerColor {
    const TYPE_NAME: &'static str = "ChatManagerColor";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            color_type: ChannelColor::from_dcb_str(inst.get_str("colorType").unwrap_or("")),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGB8>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `ChatManagerGlobalParams`
pub struct ChatManagerGlobalParams {
    /// `defaultChannelColor` (Class)
    pub default_channel_color: Option<Handle<ChatManagerDefaultChannelColor>>,
    /// `colorOptions` (Class (array))
    pub color_options: Vec<Handle<ChatManagerColor>>,
}

impl Pooled for ChatManagerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.chatmanager.chat_manager_global_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.chatmanager.chat_manager_global_params
    }
}

impl<'a> Extract<'a> for ChatManagerGlobalParams {
    const TYPE_NAME: &'static str = "ChatManagerGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_channel_color: match inst.get("defaultChannelColor") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ChatManagerDefaultChannelColor>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            color_options: inst
                .get_array("colorOptions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<ChatManagerColor>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<ChatManagerColor>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
