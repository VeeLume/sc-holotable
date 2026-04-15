// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `gamemode`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SEAPlayerLoadoutSnapshotEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAPlayerLoadoutSnapshotEntry {
    /// `loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<SItemPortLoadoutBaseParamsPtr>,
}

impl Pooled for SEAPlayerLoadoutSnapshotEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.seaplayer_loadout_snapshot_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.seaplayer_loadout_snapshot_entry }
}

impl<'a> Extract<'a> for SEAPlayerLoadoutSnapshotEntry {
    const TYPE_NAME: &'static str = "SEAPlayerLoadoutSnapshotEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadout: match inst.get("loadout") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SItemPortLoadoutBaseParamsPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEAPlayerLoadoutSnapshots`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEAPlayerLoadoutSnapshots {
    /// `entries` (Class)
    #[serde(default)]
    pub entries: Option<Handle<SEAPlayerLoadoutSnapshotEntry>>,
}

impl Pooled for SEAPlayerLoadoutSnapshots {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.seaplayer_loadout_snapshots }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.seaplayer_loadout_snapshots }
}

impl<'a> Extract<'a> for SEAPlayerLoadoutSnapshots {
    const TYPE_NAME: &'static str = "SEAPlayerLoadoutSnapshots";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: match inst.get("entries") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEAPlayerLoadoutSnapshotEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityDefaultLoadoutParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDefaultLoadoutParams {
    /// `loadout` (StrongPointer)
    #[serde(default)]
    pub loadout: Option<SItemPortLoadoutBaseParamsPtr>,
}

impl Pooled for EntityDefaultLoadoutParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.entity_default_loadout_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.entity_default_loadout_params }
}

impl<'a> Extract<'a> for EntityDefaultLoadoutParams {
    const TYPE_NAME: &'static str = "EntityDefaultLoadoutParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadout: match inst.get("loadout") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SItemPortLoadoutBaseParamsPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerShipRespawnShipInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerShipRespawnShipInfo {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `RespawnWaitTime` (Int32)
    #[serde(default)]
    pub respawn_wait_time: i32,
    /// `InstantRespawnCost` (Int32)
    #[serde(default)]
    pub instant_respawn_cost: i32,
}

impl Pooled for PlayerShipRespawnShipInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.player_ship_respawn_ship_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.player_ship_respawn_ship_info }
}

impl<'a> Extract<'a> for PlayerShipRespawnShipInfo {
    const TYPE_NAME: &'static str = "PlayerShipRespawnShipInfo";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            respawn_wait_time: inst.get_i32("RespawnWaitTime").unwrap_or_default(),
            instant_respawn_cost: inst.get_i32("InstantRespawnCost").unwrap_or_default(),
        }
    }
}

/// DCB type: `PlayerShipRespawn`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerShipRespawn {
    /// `Ships` (Class (array))
    #[serde(default)]
    pub ships: Vec<Handle<PlayerShipRespawnShipInfo>>,
}

impl Pooled for PlayerShipRespawn {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.gamemode.player_ship_respawn }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.gamemode.player_ship_respawn }
}

impl<'a> Extract<'a> for PlayerShipRespawn {
    const TYPE_NAME: &'static str = "PlayerShipRespawn";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ships: inst.get_array("Ships")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PlayerShipRespawnShipInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<PlayerShipRespawnShipInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

