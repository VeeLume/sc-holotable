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

/// Pool storage for the `ui-playerchoice_library_playerchoicelibrary` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiPlayerchoice_library_playerchoicelibraryPools {
    #[serde(default)]
    pub player_choice_option: Vec<Option<PlayerChoice_Option>>,
    #[serde(default)]
    pub player_choice_option_list: Vec<Option<PlayerChoice_OptionList>>,
    #[serde(default)]
    pub player_choice_library: Vec<Option<PlayerChoice_Library>>,
}
