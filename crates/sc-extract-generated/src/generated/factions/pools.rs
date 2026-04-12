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

/// Pool storage for the `factions` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FactionsPools {
    #[serde(default)]
    pub faction: Vec<Option<Faction>>,
    #[serde(default)]
    pub faction_reputation: Vec<Option<FactionReputation>>,
    #[serde(default)]
    pub relation_marker_params: Vec<Option<RelationMarkerParams>>,
    #[serde(default)]
    pub relation_standing_params: Vec<Option<RelationStandingParams>>,
    #[serde(default)]
    pub sandbox_trigger_base_def: Vec<Option<SandboxTriggerBaseDef>>,
}
