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

/// Pool storage for the `conversation` feature.
#[derive(Default)]
pub struct ConversationPools {
    pub sticky_filter_movement_params: Vec<Option<StickyFilterMovementParams>>,
    pub sticky_filter_rotation_params: Vec<Option<StickyFilterRotationParams>>,
    pub sticky_filter_autocenter_params: Vec<Option<StickyFilterAutocenterParams>>,
    pub conversation_sticky_filter: Vec<Option<ConversationStickyFilter>>,
    pub cinematic_conversation_settings: Vec<Option<CinematicConversationSettings>>,
    pub sconversation_icon_params: Vec<Option<SConversationIconParams>>,
    pub sscene_player_choice_settings: Vec<Option<SScenePlayerChoiceSettings>>,
}
