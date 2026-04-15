// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `conversation` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversationPools {
    #[serde(default)]
    pub sticky_filter_movement_params: Vec<Option<StickyFilterMovementParams>>,
    #[serde(default)]
    pub sticky_filter_rotation_params: Vec<Option<StickyFilterRotationParams>>,
    #[serde(default)]
    pub sticky_filter_autocenter_params: Vec<Option<StickyFilterAutocenterParams>>,
    #[serde(default)]
    pub conversation_sticky_filter: Vec<Option<ConversationStickyFilter>>,
    #[serde(default)]
    pub cinematic_conversation_settings: Vec<Option<CinematicConversationSettings>>,
    #[serde(default)]
    pub sconversation_icon_params: Vec<Option<SConversationIconParams>>,
    #[serde(default)]
    pub sscene_player_choice_settings: Vec<Option<SScenePlayerChoiceSettings>>,
}
