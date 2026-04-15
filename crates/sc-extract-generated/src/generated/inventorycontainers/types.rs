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

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `OpenInventoryOccupantItemTypeProperties`
pub struct OpenInventoryOccupantItemTypeProperties {
    /// `itemType` (EnumChoice)
    pub item_type: EItemType,
    /// `defaultProperties` (Class)
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
            item_type: EItemType::from_dcb_str(inst.get_str("itemType").unwrap_or("")),
            default_properties: match inst.get("defaultProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InventoryContainerManager`
pub struct InventoryContainerManager {
    /// `lootBoxes` (Reference (array))
    pub loot_boxes: Vec<CigGuid>,
    /// `spawnLootGrid` (Class)
    pub spawn_loot_grid: Option<Handle<Vec3>>,
    /// `closedInventoryNonStorableItemTypes` (Class (array))
    pub closed_inventory_non_storable_item_types: Vec<Handle<InventoryContainerItemTypeFilter>>,
    /// `closedInventoryNonStorableOutfitItemTypes` (EnumChoice (array))
    pub closed_inventory_non_storable_outfit_item_types: Vec<EItemType>,
    /// `openInventoryNonStorableItemTypes` (Class (array))
    pub open_inventory_non_storable_item_types: Vec<Handle<InventoryContainerItemTypeFilter>>,
    /// `itemTypeOpenInventoryOccupantProperties` (Class (array))
    pub item_type_open_inventory_occupant_properties: Vec<Handle<OpenInventoryOccupantItemTypeProperties>>,
    /// `defaultOpenInventoryOccupantProperties` (Class)
    pub default_open_inventory_occupant_properties: Option<Handle<CargoGridOccupantProperties>>,
    /// `smallItemDropDist` (Single)
    pub small_item_drop_dist: f32,
    /// `BigItemDropDist` (Single)
    pub big_item_drop_dist: f32,
    /// `bigItemVolumeThresholdSCU` (Single)
    pub big_item_volume_threshold_scu: f32,
    /// `dropItemMaxHeight` (Single)
    pub drop_item_max_height: f32,
    /// `dropItemSurfaceOffset` (Single)
    pub drop_item_surface_offset: f32,
    /// `dropItemRetryOffset` (Single)
    pub drop_item_retry_offset: f32,
    /// `numberOfRetries` (Int32)
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
                _ => None,
            },
            closed_inventory_non_storable_item_types: inst.get_array("closedInventoryNonStorableItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            closed_inventory_non_storable_outfit_item_types: inst.get_array("closedInventoryNonStorableOutfitItemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(EItemType::from_dcb_str)).collect())
                .unwrap_or_default(),
            open_inventory_non_storable_item_types: inst.get_array("openInventoryNonStorableItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<InventoryContainerItemTypeFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            item_type_open_inventory_occupant_properties: inst.get_array("itemTypeOpenInventoryOccupantProperties")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<OpenInventoryOccupantItemTypeProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<OpenInventoryOccupantItemTypeProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_open_inventory_occupant_properties: match inst.get("defaultOpenInventoryOccupantProperties") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<CargoGridOccupantProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
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

