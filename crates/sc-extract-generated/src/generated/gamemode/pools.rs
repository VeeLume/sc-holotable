// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `gamemode` feature.
#[derive(Default)]
pub struct GamemodePools {
    pub seaplayer_loadout_snapshot_entry: Vec<Option<SEAPlayerLoadoutSnapshotEntry>>,
    pub seaplayer_loadout_snapshots: Vec<Option<SEAPlayerLoadoutSnapshots>>,
    pub entity_default_loadout_params: Vec<Option<EntityDefaultLoadoutParams>>,
    pub player_ship_respawn_ship_info: Vec<Option<PlayerShipRespawnShipInfo>>,
    pub player_ship_respawn: Vec<Option<PlayerShipRespawn>>,
}
