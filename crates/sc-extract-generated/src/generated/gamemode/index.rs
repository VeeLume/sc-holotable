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

/// Record index for the `gamemode` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GamemodeIndex {
    #[serde(default)]
    pub scharacter_generation_params: HashMap<CigGuid, Handle<SCharacterGenerationParams>>,
    #[serde(default)]
    pub seaplayer_loadout_snapshots: HashMap<CigGuid, Handle<SEAPlayerLoadoutSnapshots>>,
    #[serde(default)]
    pub seaglobal_special_loadout: HashMap<CigGuid, Handle<SEAGlobalSpecialLoadout>>,
    #[serde(default)]
    pub seaglobal_event_loadouts: HashMap<CigGuid, Handle<SEAGlobalEventLoadouts>>,
    #[serde(default)]
    pub entity_default_loadout_params: HashMap<CigGuid, Handle<EntityDefaultLoadoutParams>>,
    #[serde(default)]
    pub game_mode: HashMap<CigGuid, Handle<GameMode>>,
    #[serde(default)]
    pub game_difficulty_modifiers: HashMap<CigGuid, Handle<GameDifficultyModifiers>>,
    #[serde(default)]
    pub player_ship_respawn: HashMap<CigGuid, Handle<PlayerShipRespawn>>,
}

impl GamemodeIndex {
    pub fn len(&self) -> usize {
        self.scharacter_generation_params.len()
            + self.seaplayer_loadout_snapshots.len()
            + self.seaglobal_special_loadout.len()
            + self.seaglobal_event_loadouts.len()
            + self.entity_default_loadout_params.len()
            + self.game_mode.len()
            + self.game_difficulty_modifiers.len()
            + self.player_ship_respawn.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
