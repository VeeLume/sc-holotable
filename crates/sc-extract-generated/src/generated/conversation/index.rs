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
use crate::Handle;
use std::collections::HashMap;
use svarog_common::CigGuid;

/// Record index for the `conversation` feature.
#[derive(Default)]
pub struct ConversationIndex {
    pub conversation_sticky_filter: HashMap<CigGuid, Handle<ConversationStickyFilter>>,
    pub cinematic_conversation_settings: HashMap<CigGuid, Handle<CinematicConversationSettings>>,
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
