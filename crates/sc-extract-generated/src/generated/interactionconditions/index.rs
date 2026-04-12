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

/// Record index for the `interactionconditions` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InteractionconditionsIndex {
    #[serde(default)]
    pub interaction_condition_preset: HashMap<CigGuid, Handle<InteractionConditionPreset>>,
    #[serde(default)]
    pub shop_interaction_data: HashMap<CigGuid, Handle<ShopInteractionData>>,
}

impl InteractionconditionsIndex {
    pub fn len(&self) -> usize {
        self.interaction_condition_preset.len()
            + self.shop_interaction_data.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
