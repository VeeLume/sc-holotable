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

/// Record index for the `ui-innerthought` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiInnerthoughtIndex {
    #[serde(default)]
    pub inner_thought_anim: HashMap<CigGuid, Handle<InnerThought_Anim>>,
    #[serde(default)]
    pub inner_thought_color_params: HashMap<CigGuid, Handle<InnerThought_ColorParams>>,
    #[serde(default)]
    pub inner_thought_params: HashMap<CigGuid, Handle<InnerThought_Params>>,
    #[serde(default)]
    pub inner_thought_conversation_system_config: HashMap<CigGuid, Handle<InnerThought_ConversationSystemConfig>>,
    #[serde(default)]
    pub inner_thought_interaction_system_config: HashMap<CigGuid, Handle<InnerThought_InteractionSystemConfig>>,
    #[serde(default)]
    pub inner_thought_legacy_use_system_config: HashMap<CigGuid, Handle<InnerThought_LegacyUseSystemConfig>>,
}

impl UiInnerthoughtIndex {
    pub fn len(&self) -> usize {
        self.inner_thought_anim.len()
            + self.inner_thought_color_params.len()
            + self.inner_thought_params.len()
            + self.inner_thought_conversation_system_config.len()
            + self.inner_thought_interaction_system_config.len()
            + self.inner_thought_legacy_use_system_config.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
