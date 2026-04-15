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

/// Pool storage for the `chatmanager` feature.
#[derive(Default)]
pub struct ChatmanagerPools {
    pub chat_manager_default_channel_color: Vec<Option<ChatManagerDefaultChannelColor>>,
    pub chat_manager_color: Vec<Option<ChatManagerColor>>,
    pub chat_manager_global_params: Vec<Option<ChatManagerGlobalParams>>,
}
