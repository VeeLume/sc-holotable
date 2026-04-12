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

/// Pool storage for the `ui-innerthought` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiInnerthoughtPools {
    #[serde(default)]
    pub inner_thought_anim_base: Vec<Option<InnerThought_AnimBase>>,
    #[serde(default)]
    pub inner_thought_anim: Vec<Option<InnerThought_Anim>>,
    #[serde(default)]
    pub inner_thought_layout_base: Vec<Option<InnerThought_LayoutBase>>,
    #[serde(default)]
    pub inner_thought_color_params: Vec<Option<InnerThought_ColorParams>>,
    #[serde(default)]
    pub inner_thought_layout_states: Vec<Option<InnerThought_LayoutStates>>,
    #[serde(default)]
    pub inner_thought_params: Vec<Option<InnerThought_Params>>,
    #[serde(default)]
    pub inner_thought_conversation_system_config: Vec<Option<InnerThought_ConversationSystemConfig>>,
    #[serde(default)]
    pub inner_thought_interaction_system_config: Vec<Option<InnerThought_InteractionSystemConfig>>,
    #[serde(default)]
    pub inner_thought_legacy_use_system_config: Vec<Option<InnerThought_LegacyUseSystemConfig>>,
}
