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

/// Pool storage for the `chatchannelfilters` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatchannelfiltersPools {
    #[serde(default)]
    pub schat_channel_filter_base: Vec<Option<SChatChannelFilterBase>>,
    #[serde(default)]
    pub chat_channel_filter_record: Vec<Option<ChatChannelFilterRecord>>,
}
