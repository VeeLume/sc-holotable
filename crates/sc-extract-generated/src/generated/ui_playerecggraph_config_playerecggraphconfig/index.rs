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

/// Record index for the `ui-playerecggraph_config_playerecggraphconfig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiPlayerecggraph_config_playerecggraphconfigIndex {
    #[serde(default)]
    pub player_ecggraph_config: HashMap<CigGuid, Handle<PlayerECGGraph_Config>>,
}

impl UiPlayerecggraph_config_playerecggraphconfigIndex {
    pub fn len(&self) -> usize {
        self.player_ecggraph_config.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
