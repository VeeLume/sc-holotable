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
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.conversation_sticky_filter.len();
        total += self.cinematic_conversation_settings.len();
        total += self.sscene_player_choice_settings.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
