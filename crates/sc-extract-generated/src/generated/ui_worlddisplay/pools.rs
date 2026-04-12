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

/// Pool storage for the `ui-worlddisplay` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiWorlddisplayPools {
    #[serde(default)]
    pub world_display_radar: Vec<Option<WorldDisplayRadar>>,
    #[serde(default)]
    pub world_display_radar_icon: Vec<Option<WorldDisplayRadar_Icon>>,
    #[serde(default)]
    pub world_display_radar_line: Vec<Option<WorldDisplayRadar_Line>>,
    #[serde(default)]
    pub world_display_environment: Vec<Option<WorldDisplayEnvironment>>,
    #[serde(default)]
    pub world_display_environment_base: Vec<Option<WorldDisplayEnvironmentBase>>,
    #[serde(default)]
    pub world_display_environment_color: Vec<Option<WorldDisplayEnvironmentColor>>,
}
