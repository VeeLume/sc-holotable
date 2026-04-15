// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `chatfilteroptions`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ChatFilterOptions`
pub struct ChatFilterOptions {
    /// `options` (Class (array))
    pub options: Vec<Handle<ChatFilter>>,
}

impl Pooled for ChatFilterOptions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatfilteroptions.chat_filter_options }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatfilteroptions.chat_filter_options }
}

impl<'a> Extract<'a> for ChatFilterOptions {
    const TYPE_NAME: &'static str = "ChatFilterOptions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            options: inst.get_array("options")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ChatFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<ChatFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ChatFilter`
pub struct ChatFilter {
    /// `tagId` (Int32)
    pub tag_id: i32,
    /// `localizedString` (String)
    pub localized_string: String,
}

impl Pooled for ChatFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.chatfilteroptions.chat_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.chatfilteroptions.chat_filter }
}

impl<'a> Extract<'a> for ChatFilter {
    const TYPE_NAME: &'static str = "ChatFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag_id: inst.get_i32("tagId").unwrap_or_default(),
            localized_string: inst.get_str("localizedString").map(String::from).unwrap_or_default(),
        }
    }
}

