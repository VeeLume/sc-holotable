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

/// Pool storage for the `ui-playerchoice_imconfig_playerchoiceim` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiPlayerchoice_imconfig_playerchoiceimPools {
    #[serde(default)]
    pub player_choice_remote_comms_config: Vec<Option<PlayerChoice_RemoteCommsConfig>>,
    #[serde(default)]
    pub player_choice_head_look_params: Vec<Option<PlayerChoice_HeadLookParams>>,
    #[serde(default)]
    pub player_choice_imconfig: Vec<Option<PlayerChoice_IMConfig>>,
}
