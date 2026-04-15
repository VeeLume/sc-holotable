// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `chatcommandfastaccess`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ChatCommandFastAccess`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCommandFastAccess {
    /// `commands` (Class (array))
    #[serde(default)]
    pub commands: Vec<Handle<ChatCommandName>>,
}

impl Pooled for ChatCommandFastAccess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatcommandfastaccess.chat_command_fast_access }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatcommandfastaccess.chat_command_fast_access }
}

impl<'a> Extract<'a> for ChatCommandFastAccess {
    const TYPE_NAME: &'static str = "ChatCommandFastAccess";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            commands: inst.get_array("commands")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatCommandName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ChatCommandName>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatCommandName`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCommandName {
    /// `commandName` (String)
    #[serde(default)]
    pub command_name: String,
}

impl Pooled for ChatCommandName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatcommandfastaccess.chat_command_name }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatcommandfastaccess.chat_command_name }
}

impl<'a> Extract<'a> for ChatCommandName {
    const TYPE_NAME: &'static str = "ChatCommandName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            command_name: inst.get_str("commandName").map(String::from).unwrap_or_default(),
        }
    }
}

