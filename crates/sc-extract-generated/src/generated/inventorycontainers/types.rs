// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `inventorycontainers`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `OpenInventoryOccupantItemTypeProperties`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenInventoryOccupantItemTypeProperties {
    /// `itemType` (EnumChoice)
    #[serde(default)]
    pub item_type: String,
    /// `defaultProperties` (Class)
    #[serde(default)]
    pub default_properties: Option<Handle<CargoGridOccupantProperties>>,
}

impl Pooled for OpenInventoryOccupantItemTypeProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.inventorycontainers.open_inventory_occupant_item_type_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.inventorycontainers.open_inventory_occupant_item_type_properties }
}

impl<'a> Extract<'a> for OpenInventoryOccupantItemTypeProperties {
    const TYPE_NAME: &'static str = "OpenInventoryOccupantItemTypeProperties";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_type: inst.get_str("itemType").map(String::from).unwrap_or_default(),
            default_properties: match inst.get("defaultProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InventoryContainerManager`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryContainerManager {
    /// `lootBoxes` (Reference (array))
    #[serde(default)]
    pub loot_boxes: Vec<CigGuid>,
    /// `spawnLootGrid` (Class)
    #[serde(default)]
    pub spawn_loot_grid: Option<Handle<Vec3>>,
    /// `closedInventoryNonStorableItemTypes` (Class (array))
    #[serde(default)]
    pub closed_inventory_non_storable_item_types: Vec<Handle<InventoryContainerItemTypeFilter>>,
    /// `closedInventoryNonStorableOutfitItemTypes` (EnumChoice (array))
    #[serde(default)]
    pub closed_inventory_non_storable_outfit_item_types: Vec<String>,
    /// `openInventoryNonStorableItemTypes` (Class (array))
    #[serde(default)]
    pub open_inventory_non_storable_item_types: Vec<Handle<InventoryContainerItemTypeFilter>>,
    /// `itemTypeOpenInventoryOccupantProperties` (Class (array))
    #[serde(default)]
    pub item_type_open_inventory_occupant_properties: Vec<Handle<OpenInventoryOccupantItemTypeProperties>>,
    /// `defaultOpenInventoryOccupantProperties` (Class)
    #[serde(default)]
    pub default_open_inventory_occupant_properties: Option<Handle<CargoGridOccupantProperties>>,
    /// `smallItemDropDist` (Single)
    #[serde(default)]
    pub small_item_drop_dist: f32,
    /// `BigItemDropDist` (Single)
    #[serde(default)]
    pub big_item_drop_dist: f32,
    /// `bigItemVolumeThresholdSCU` (Single)
    #[serde(default)]
    pub big_item_volume_threshold_scu: f32,
    /// `dropItemMaxHeight` (Single)
    #[serde(default)]
    pub drop_item_max_height: f32,
    /// `dropItemSurfaceOffset` (Single)
    #[serde(default)]
    pub drop_item_surface_offset: f32,
    /// `dropItemRetryOffset` (Single)
    #[serde(default)]
    pub drop_item_retry_offset: f32,
    /// `numberOfRetries` (Int32)
    #[serde(default)]
    pub number_of_retries: i32,
}

impl Pooled for InventoryContainerManager {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.inventorycontainers.inventory_container_manager }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.inventorycontainers.inventory_container_manager }
}

impl<'a> Extract<'a> for InventoryContainerManager {
    const TYPE_NAME: &'static str = "InventoryContainerManager";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            loot_boxes: inst.get_array("lootBoxes")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            spawn_loot_grid: match inst.get("spawnLootGrid") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            closed_inventory_non_storable_item_types: inst.get_array("closedInventoryNonStorableItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            closed_inventory_non_storable_outfit_item_types: inst.get_array("closedInventoryNonStorableOutfitItemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            open_inventory_non_storable_item_types: inst.get_array("openInventoryNonStorableItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            item_type_open_inventory_occupant_properties: inst.get_array("itemTypeOpenInventoryOccupantProperties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<OpenInventoryOccupantItemTypeProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<OpenInventoryOccupantItemTypeProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_open_inventory_occupant_properties: match inst.get("defaultOpenInventoryOccupantProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            small_item_drop_dist: inst.get_f32("smallItemDropDist").unwrap_or_default(),
            big_item_drop_dist: inst.get_f32("BigItemDropDist").unwrap_or_default(),
            big_item_volume_threshold_scu: inst.get_f32("bigItemVolumeThresholdSCU").unwrap_or_default(),
            drop_item_max_height: inst.get_f32("dropItemMaxHeight").unwrap_or_default(),
            drop_item_surface_offset: inst.get_f32("dropItemSurfaceOffset").unwrap_or_default(),
            drop_item_retry_offset: inst.get_f32("dropItemRetryOffset").unwrap_or_default(),
            number_of_retries: inst.get_i32("numberOfRetries").unwrap_or_default(),
        }
    }
}

/// DCB type: `LandingZoneInventoryRedirect`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingZoneInventoryRedirect {
    /// `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// `inventoryLocation` (Reference)
    #[serde(default)]
    pub inventory_location: Option<CigGuid>,
}

impl Pooled for LandingZoneInventoryRedirect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.inventorycontainers.landing_zone_inventory_redirect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.inventorycontainers.landing_zone_inventory_redirect }
}

impl<'a> Extract<'a> for LandingZoneInventoryRedirect {
    const TYPE_NAME: &'static str = "LandingZoneInventoryRedirect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            inventory_location: inst.get("inventoryLocation").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InventoryLocation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryLocation {
    /// `location` (Reference)
    #[serde(default)]
    pub location: Option<CigGuid>,
    /// `visibleToPlayer` (Boolean)
    #[serde(default)]
    pub visible_to_player: bool,
    /// `lawful` (Boolean)
    #[serde(default)]
    pub lawful: bool,
    /// `impoundLocation` (Boolean)
    #[serde(default)]
    pub impound_location: bool,
}

impl Pooled for InventoryLocation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.inventorycontainers.inventory_location }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.inventorycontainers.inventory_location }
}

impl<'a> Extract<'a> for InventoryLocation {
    const TYPE_NAME: &'static str = "InventoryLocation";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location: inst.get("location").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            visible_to_player: inst.get_bool("visibleToPlayer").unwrap_or_default(),
            lawful: inst.get_bool("lawful").unwrap_or_default(),
            impound_location: inst.get_bool("impoundLocation").unwrap_or_default(),
        }
    }
}

/// DCB type: `LandingZoneInventory`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandingZoneInventory {
    /// `containerParams` (Reference)
    #[serde(default)]
    pub container_params: Option<CigGuid>,
    /// `locations` (Class (array))
    #[serde(default)]
    pub locations: Vec<Handle<InventoryLocation>>,
    /// `locationRedirects` (Class (array))
    #[serde(default)]
    pub location_redirects: Vec<Handle<LandingZoneInventoryRedirect>>,
}

impl Pooled for LandingZoneInventory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.inventorycontainers.landing_zone_inventory }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.inventorycontainers.landing_zone_inventory }
}

impl<'a> Extract<'a> for LandingZoneInventory {
    const TYPE_NAME: &'static str = "LandingZoneInventory";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            container_params: inst.get("containerParams").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            locations: inst.get_array("locations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryLocation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventoryLocation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            location_redirects: inst.get_array("locationRedirects")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LandingZoneInventoryRedirect>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LandingZoneInventoryRedirect>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CargoGridOccupantFace`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoGridOccupantFace {
    /// `faceUpwardAllowed` (Boolean)
    #[serde(default)]
    pub face_upward_allowed: bool,
    /// `stackingSupport` (EnumChoice)
    #[serde(default)]
    pub stacking_support: String,
}

impl Pooled for CargoGridOccupantFace {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.inventorycontainers.cargo_grid_occupant_face }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.inventorycontainers.cargo_grid_occupant_face }
}

impl<'a> Extract<'a> for CargoGridOccupantFace {
    const TYPE_NAME: &'static str = "CargoGridOccupantFace";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            face_upward_allowed: inst.get_bool("faceUpwardAllowed").unwrap_or_default(),
            stacking_support: inst.get_str("stackingSupport").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CargoGridOccupantProperties`
/// Inherits from: `EntityClassStaticDataParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoGridOccupantProperties {
    /// `Top` (Class)
    #[serde(default)]
    pub top: Option<Handle<CargoGridOccupantFace>>,
    /// `Bottom` (Class)
    #[serde(default)]
    pub bottom: Option<Handle<CargoGridOccupantFace>>,
    /// `Front` (Class)
    #[serde(default)]
    pub front: Option<Handle<CargoGridOccupantFace>>,
    /// `Back` (Class)
    #[serde(default)]
    pub back: Option<Handle<CargoGridOccupantFace>>,
    /// `Right` (Class)
    #[serde(default)]
    pub right: Option<Handle<CargoGridOccupantFace>>,
    /// `Left` (Class)
    #[serde(default)]
    pub left: Option<Handle<CargoGridOccupantFace>>,
}

impl Pooled for CargoGridOccupantProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.inventorycontainers.cargo_grid_occupant_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.inventorycontainers.cargo_grid_occupant_properties }
}

impl<'a> Extract<'a> for CargoGridOccupantProperties {
    const TYPE_NAME: &'static str = "CargoGridOccupantProperties";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            top: match inst.get("Top") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            bottom: match inst.get("Bottom") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            front: match inst.get("Front") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            back: match inst.get("Back") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            right: match inst.get("Right") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            left: match inst.get("Left") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantFace>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<CargoGridOccupantFace>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

