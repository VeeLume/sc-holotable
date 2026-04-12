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

/// Pool storage for the `character` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterPools {
    #[serde(default)]
    pub character_personal_data: Vec<Option<CharacterPersonalData>>,
    #[serde(default)]
    pub character: Vec<Option<Character>>,
    #[serde(default)]
    pub movement_speed_override: Vec<Option<MovementSpeedOverride>>,
    #[serde(default)]
    pub default_player_loadout_entitlement_params: Vec<Option<DefaultPlayerLoadoutEntitlementParams>>,
    #[serde(default)]
    pub default_player_loadout_entitlement_record: Vec<Option<DefaultPlayerLoadoutEntitlementRecord>>,
}
