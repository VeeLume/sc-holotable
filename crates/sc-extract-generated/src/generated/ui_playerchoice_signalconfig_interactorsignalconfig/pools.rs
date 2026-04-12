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

/// Pool storage for the `ui-playerchoice_signalconfig_interactorsignalconfig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiPlayerchoice_signalconfig_interactorsignalconfigPools {
    #[serde(default)]
    pub player_choice_interaction_modifier: Vec<Option<PlayerChoice_InteractionModifier>>,
    #[serde(default)]
    pub player_choice_signal_config: Vec<Option<PlayerChoice_SignalConfig>>,
}
