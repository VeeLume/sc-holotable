// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `chatmanager` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatmanagerPools {
    #[serde(default)]
    pub chat_manager_default_channel_color: Vec<Option<ChatManagerDefaultChannelColor>>,
    #[serde(default)]
    pub chat_manager_color: Vec<Option<ChatManagerColor>>,
    #[serde(default)]
    pub chat_manager_global_params: Vec<Option<ChatManagerGlobalParams>>,
}
