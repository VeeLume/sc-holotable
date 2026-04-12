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

/// Record index for the `conversation` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversationIndex {
    #[serde(default)]
    pub conversation_sticky_filter: HashMap<CigGuid, Handle<ConversationStickyFilter>>,
    #[serde(default)]
    pub cinematic_conversation_settings: HashMap<CigGuid, Handle<CinematicConversationSettings>>,
    #[serde(default)]
    pub sscene_player_choice_settings: HashMap<CigGuid, Handle<SScenePlayerChoiceSettings>>,
}

impl ConversationIndex {
    pub fn len(&self) -> usize {
        self.conversation_sticky_filter.len()
            + self.cinematic_conversation_settings.len()
            + self.sscene_player_choice_settings.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
