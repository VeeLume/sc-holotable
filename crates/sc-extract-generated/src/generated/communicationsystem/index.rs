// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `communicationsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunicationsystemIndex {
    #[serde(default)]
    pub communication_channel_config: HashMap<CigGuid, Handle<CommunicationChannelConfig>>,
    #[serde(default)]
    pub communication_auto_mannequin_tags_config: HashMap<CigGuid, Handle<CommunicationAutoMannequinTagsConfig>>,
}

impl CommunicationsystemIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.communication_channel_config.len();
        total += self.communication_auto_mannequin_tags_config.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
