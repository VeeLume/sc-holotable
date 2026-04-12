// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `musiclogic` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MusiclogicIndex {
    #[serde(default)]
    pub music_logic_config: HashMap<CigGuid, Handle<MusicLogicConfig>>,
    #[serde(default)]
    pub music_logic_parameter: HashMap<CigGuid, Handle<MusicLogicParameter>>,
    #[serde(default)]
    pub music_logic_event: HashMap<CigGuid, Handle<MusicLogicEvent>>,
    #[serde(default)]
    pub music_logic_event_list: HashMap<CigGuid, Handle<MusicLogicEventList>>,
    #[serde(default)]
    pub music_logic_switch_value: HashMap<CigGuid, Handle<MusicLogicSwitchValue>>,
    #[serde(default)]
    pub music_logic_suite: HashMap<CigGuid, Handle<MusicLogicSuite>>,
}

impl MusiclogicIndex {
    pub fn len(&self) -> usize {
        self.music_logic_config.len()
            + self.music_logic_parameter.len()
            + self.music_logic_event.len()
            + self.music_logic_event_list.len()
            + self.music_logic_switch_value.len()
            + self.music_logic_suite.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
