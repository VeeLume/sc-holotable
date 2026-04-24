// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `gamemode` feature.
#[derive(Default)]
pub struct GamemodeIndex {
    pub seaplayer_loadout_snapshots: HashMap<CigGuid, Handle<SEAPlayerLoadoutSnapshots>>,
    pub entity_default_loadout_params: HashMap<CigGuid, Handle<EntityDefaultLoadoutParams>>,
    pub player_ship_respawn: HashMap<CigGuid, Handle<PlayerShipRespawn>>,
}

impl GamemodeIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.seaplayer_loadout_snapshots.len();
        total += self.entity_default_loadout_params.len();
        total += self.player_ship_respawn.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
