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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SArchetypeAssetEntityDef`
/// Inherits from: `SArchetypeEntityAssetDefBase`
pub struct SArchetypeAssetEntityDef {
    /// `debugName` (String)
    pub debug_name: String,
    /// `itemportName` (String)
    pub itemport_name: String,
    /// `parentItemportName` (String)
    pub parent_itemport_name: String,
    /// `entityClass` (Reference)
    pub entity_class: Option<CigGuid>,
}

impl Pooled for SArchetypeAssetEntityDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.gamemode.sarchetype_asset_entity_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.gamemode.sarchetype_asset_entity_def
    }
}

impl<'a> Extract<'a> for SArchetypeAssetEntityDef {
    const TYPE_NAME: &'static str = "SArchetypeAssetEntityDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            itemport_name: inst
                .get_str("itemportName")
                .map(String::from)
                .unwrap_or_default(),
            parent_itemport_name: inst
                .get_str("parentItemportName")
                .map(String::from)
                .unwrap_or_default(),
            entity_class: inst
                .get("entityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `SArchetypeAssetCHFDef`
/// Inherits from: `SArchetypeAssetDefBase`
pub struct SArchetypeAssetCHFDef {
    /// `debugName` (String)
    pub debug_name: String,
    /// `customizationFile` (String)
    pub customization_file: String,
}

impl Pooled for SArchetypeAssetCHFDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.gamemode.sarchetype_asset_chfdef
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.gamemode.sarchetype_asset_chfdef
    }
}

impl<'a> Extract<'a> for SArchetypeAssetCHFDef {
    const TYPE_NAME: &'static str = "SArchetypeAssetCHFDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            customization_file: inst
                .get_str("customizationFile")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SArchetypeAssetLoadoutDef`
/// Inherits from: `SArchetypeAssetDefBase`
pub struct SArchetypeAssetLoadoutDef {
    /// `debugName` (String)
    pub debug_name: String,
    /// `loadout` (StrongPointer)
    pub loadout: Option<SItemPortLoadoutBaseParamsPtr>,
}

impl Pooled for SArchetypeAssetLoadoutDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.gamemode.sarchetype_asset_loadout_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.gamemode.sarchetype_asset_loadout_def
    }
}

impl<'a> Extract<'a> for SArchetypeAssetLoadoutDef {
    const TYPE_NAME: &'static str = "SArchetypeAssetLoadoutDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            loadout: match inst.get("loadout") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SItemPortLoadoutBaseParamsPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SArchetypeAssetTagDef`
/// Inherits from: `SArchetypeEntityAssetDefBase`
pub struct SArchetypeAssetTagDef {
    /// `debugName` (String)
    pub debug_name: String,
    /// `itemportName` (String)
    pub itemport_name: String,
    /// `parentItemportName` (String)
    pub parent_itemport_name: String,
    /// `requiredTags` (Class)
    pub required_tags: Option<Handle<TagList>>,
    /// `forbiddenTags` (Class)
    pub forbidden_tags: Option<Handle<TagList>>,
}

impl Pooled for SArchetypeAssetTagDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.gamemode.sarchetype_asset_tag_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.gamemode.sarchetype_asset_tag_def
    }
}

impl<'a> Extract<'a> for SArchetypeAssetTagDef {
    const TYPE_NAME: &'static str = "SArchetypeAssetTagDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            itemport_name: inst
                .get_str("itemportName")
                .map(String::from)
                .unwrap_or_default(),
            parent_itemport_name: inst
                .get_str("parentItemportName")
                .map(String::from)
                .unwrap_or_default(),
            required_tags: match inst.get("requiredTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            forbidden_tags: match inst.get("forbiddenTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `SEAPlayerLoadoutSnapshotEntry`
pub struct SEAPlayerLoadoutSnapshotEntry {
    /// `loadout` (StrongPointer)
    pub loadout: Option<SItemPortLoadoutBaseParamsPtr>,
}

impl Pooled for SEAPlayerLoadoutSnapshotEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.gamemode.seaplayer_loadout_snapshot_entry
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.gamemode.seaplayer_loadout_snapshot_entry
    }
}

impl<'a> Extract<'a> for SEAPlayerLoadoutSnapshotEntry {
    const TYPE_NAME: &'static str = "SEAPlayerLoadoutSnapshotEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadout: match inst.get("loadout") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SItemPortLoadoutBaseParamsPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SEAPlayerLoadoutSnapshots`
pub struct SEAPlayerLoadoutSnapshots {
    /// `entries` (Class (array))
    pub entries: Vec<Handle<SEAPlayerLoadoutSnapshotEntry>>,
}

impl Pooled for SEAPlayerLoadoutSnapshots {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.gamemode.seaplayer_loadout_snapshots
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.gamemode.seaplayer_loadout_snapshots
    }
}

impl<'a> Extract<'a> for SEAPlayerLoadoutSnapshots {
    const TYPE_NAME: &'static str = "SEAPlayerLoadoutSnapshots";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entries: inst
                .get_array("entries")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SEAPlayerLoadoutSnapshotEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<SEAPlayerLoadoutSnapshotEntry>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityDefaultLoadoutParams`
pub struct EntityDefaultLoadoutParams {
    /// `loadout` (StrongPointer)
    pub loadout: Option<SItemPortLoadoutBaseParamsPtr>,
}

impl Pooled for EntityDefaultLoadoutParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.gamemode.entity_default_loadout_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.gamemode.entity_default_loadout_params
    }
}

impl<'a> Extract<'a> for EntityDefaultLoadoutParams {
    const TYPE_NAME: &'static str = "EntityDefaultLoadoutParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loadout: match inst.get("loadout") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SItemPortLoadoutBaseParamsPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `PlayerShipRespawnShipInfo`
pub struct PlayerShipRespawnShipInfo {
    /// `Name` (String)
    pub name: String,
    /// `RespawnWaitTime` (Int32)
    pub respawn_wait_time: i32,
    /// `InstantRespawnCost` (Int32)
    pub instant_respawn_cost: i32,
}

impl Pooled for PlayerShipRespawnShipInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.gamemode.player_ship_respawn_ship_info
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.gamemode.player_ship_respawn_ship_info
    }
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
pub struct PlayerShipRespawn {
    /// `Ships` (Class (array))
    pub ships: Vec<Handle<PlayerShipRespawnShipInfo>>,
}

impl Pooled for PlayerShipRespawn {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.gamemode.player_ship_respawn
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.gamemode.player_ship_respawn
    }
}

impl<'a> Extract<'a> for PlayerShipRespawn {
    const TYPE_NAME: &'static str = "PlayerShipRespawn";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            ships: inst
                .get_array("Ships")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<PlayerShipRespawnShipInfo>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<PlayerShipRespawnShipInfo>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
