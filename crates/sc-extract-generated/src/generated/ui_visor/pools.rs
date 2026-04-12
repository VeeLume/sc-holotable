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

/// Pool storage for the `ui-visor` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiVisorPools {
    #[serde(default)]
    pub visor_hud_config: Vec<Option<VisorHUD_Config>>,
    #[serde(default)]
    pub visor_control_hints_config: Vec<Option<Visor_ControlHintsConfig>>,
    #[serde(default)]
    pub control_hint_game_mode_records: Vec<Option<ControlHintGameModeRecords>>,
    #[serde(default)]
    pub control_hints_input: Vec<Option<ControlHints_Input>>,
    #[serde(default)]
    pub control_hints_hint_display_info_action: Vec<Option<ControlHints_HintDisplayInfoAction>>,
    #[serde(default)]
    pub control_hint_display_info_set: Vec<Option<ControlHint_DisplayInfoSet>>,
    #[serde(default)]
    pub control_hint_condition: Vec<Option<ControlHintCondition>>,
    #[serde(default)]
    pub control_hint_def: Vec<Option<ControlHintDef>>,
    #[serde(default)]
    pub control_hint_always_display_condition: Vec<Option<ControlHintAlwaysDisplayCondition>>,
    #[serde(default)]
    pub control_hint_entry: Vec<Option<ControlHint_Entry>>,
    #[serde(default)]
    pub control_hints_preset: Vec<Option<ControlHints_Preset>>,
}
