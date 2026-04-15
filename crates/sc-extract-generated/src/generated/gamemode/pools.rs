// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `gamemode` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GamemodePools {
    #[serde(default)]
    pub seaplayer_loadout_snapshot_entry: Vec<Option<SEAPlayerLoadoutSnapshotEntry>>,
    #[serde(default)]
    pub seaplayer_loadout_snapshots: Vec<Option<SEAPlayerLoadoutSnapshots>>,
    #[serde(default)]
    pub entity_default_loadout_params: Vec<Option<EntityDefaultLoadoutParams>>,
    #[serde(default)]
    pub player_ship_respawn_ship_info: Vec<Option<PlayerShipRespawnShipInfo>>,
    #[serde(default)]
    pub player_ship_respawn: Vec<Option<PlayerShipRespawn>>,
}
