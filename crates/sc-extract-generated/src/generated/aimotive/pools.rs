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

/// Pool storage for the `aimotive` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AimotivePools {
    #[serde(default)]
    pub aimotive_action_details: Vec<Option<AIMotiveActionDetails>>,
    #[serde(default)]
    pub aimotive_action: Vec<Option<AIMotiveAction>>,
    #[serde(default)]
    pub aimotive_condition: Vec<Option<AIMotiveCondition>>,
    #[serde(default)]
    pub aimotive: Vec<Option<AIMotive>>,
    #[serde(default)]
    pub aimotive_list: Vec<Option<AIMotiveList>>,
}
