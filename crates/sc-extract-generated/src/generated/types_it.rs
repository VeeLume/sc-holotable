// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `ItemCarryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCarryParams {
    /// DCB field: `maxCarryableMass` (Single)
    #[serde(default)]
    pub max_carryable_mass: f32,
}

impl Pooled for ItemCarryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_carry_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_carry_params }
}

impl<'a> Extract<'a> for ItemCarryParams {
    const TYPE_NAME: &'static str = "ItemCarryParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_carryable_mass: inst.get_f32("maxCarryableMass").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemTypeModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeModifier {
    /// DCB field: `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `SubType` (EnumChoice)
    #[serde(default)]
    pub sub_type: String,
    /// DCB field: `matchPercentage` (Single)
    #[serde(default)]
    pub match_percentage: f32,
}

impl Pooled for ItemTypeModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_type_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_type_modifier }
}

impl<'a> Extract<'a> for ItemTypeModifier {
    const TYPE_NAME: &'static str = "ItemTypeModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("Type").map(String::from).unwrap_or_default(),
            sub_type: inst.get_str("SubType").map(String::from).unwrap_or_default(),
            match_percentage: inst.get_f32("matchPercentage").unwrap_or_default(),
        }
    }
}

/// DCB type: `Item`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// DCB field: `type` (StrongPointer)
    #[serde(default)]
    pub r#type: Option<Handle<BaseItem>>,
}

impl Pooled for Item {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item }
}

impl<'a> Extract<'a> for Item {
    const TYPE_NAME: &'static str = "Item";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            r#type: match inst.get("type") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BaseItem>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BaseItem>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemKioskBrand`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemKioskBrand {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `logoPath` (String)
    #[serde(default)]
    pub logo_path: String,
    /// DCB field: `color` (Class)
    #[serde(default)]
    pub color: Option<Handle<SRGBA8>>,
}

impl Pooled for ItemKioskBrand {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_kiosk_brand }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_kiosk_brand }
}

impl<'a> Extract<'a> for ItemKioskBrand {
    const TYPE_NAME: &'static str = "ItemKioskBrand";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            logo_path: inst.get_str("logoPath").map(String::from).unwrap_or_default(),
            color: match inst.get("color") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SRGBA8>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SRGBA8>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemPortTagsElement`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPortTagsElement {
    /// DCB field: `string` (String)
    #[serde(default)]
    pub string: String,
    /// DCB field: `tag` (Reference)
    #[serde(default)]
    pub tag: Option<CigGuid>,
}

impl Pooled for ItemPortTagsElement {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_port_tags_element }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_port_tags_element }
}

impl<'a> Extract<'a> for ItemPortTagsElement {
    const TYPE_NAME: &'static str = "ItemPortTagsElement";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            string: inst.get_str("string").map(String::from).unwrap_or_default(),
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ItemPortTagsDictionary`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPortTagsDictionary {
    /// DCB field: `elements` (Class (array))
    #[serde(default)]
    pub elements: Vec<Handle<ItemPortTagsElement>>,
}

impl Pooled for ItemPortTagsDictionary {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_port_tags_dictionary }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_port_tags_dictionary }
}

impl<'a> Extract<'a> for ItemPortTagsDictionary {
    const TYPE_NAME: &'static str = "ItemPortTagsDictionary";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            elements: inst.get_array("elements")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemPortTagsElement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemPortTagsElement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemResourceTypeData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceTypeData {
    /// DCB field: `resourceTypeName` (Locale)
    #[serde(default)]
    pub resource_type_name: String,
    /// DCB field: `resourceTypeDevName` (String)
    #[serde(default)]
    pub resource_type_dev_name: String,
    /// DCB field: `resourceType` (EnumChoice)
    #[serde(default)]
    pub resource_type: String,
}

impl Pooled for ItemResourceTypeData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_type_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_type_data }
}

impl<'a> Extract<'a> for ItemResourceTypeData {
    const TYPE_NAME: &'static str = "ItemResourceTypeData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            resource_type_name: inst.get_str("resourceTypeName").map(String::from).unwrap_or_default(),
            resource_type_dev_name: inst.get_str("resourceTypeDevName").map(String::from).unwrap_or_default(),
            resource_type: inst.get_str("resourceType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemResourceCompositionMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceCompositionMap {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `containerResources` (Reference (array))
    #[serde(default)]
    pub container_resources: Vec<CigGuid>,
    /// DCB field: `affectsItemFunctionality` (Boolean)
    #[serde(default)]
    pub affects_item_functionality: bool,
    /// DCB field: `isShared` (Boolean)
    #[serde(default)]
    pub is_shared: bool,
    /// DCB field: `itemMustBeOnline` (Boolean)
    #[serde(default)]
    pub item_must_be_online: bool,
}

impl Pooled for ItemResourceCompositionMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_composition_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_composition_map }
}

impl<'a> Extract<'a> for ItemResourceCompositionMap {
    const TYPE_NAME: &'static str = "ItemResourceCompositionMap";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            container_resources: inst.get_array("containerResources")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            affects_item_functionality: inst.get_bool("affectsItemFunctionality").unwrap_or_default(),
            is_shared: inst.get_bool("isShared").unwrap_or_default(),
            item_must_be_online: inst.get_bool("itemMustBeOnline").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemRoomResourcePair`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRoomResourcePair {
    /// DCB field: `resourceGasRef` (Reference)
    #[serde(default)]
    pub resource_gas_ref: Option<CigGuid>,
    /// DCB field: `roomGasRef` (Reference)
    #[serde(default)]
    pub room_gas_ref: Option<CigGuid>,
}

impl Pooled for ItemRoomResourcePair {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_room_resource_pair }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_room_resource_pair }
}

impl<'a> Extract<'a> for ItemRoomResourcePair {
    const TYPE_NAME: &'static str = "ItemRoomResourcePair";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            resource_gas_ref: inst.get("resourceGasRef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            room_gas_ref: inst.get("roomGasRef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ItemResourceNetworkMapTriggerEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceNetworkMapTriggerEntry {
    /// DCB field: `icon` (String)
    #[serde(default)]
    pub icon: String,
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
}

impl Pooled for ItemResourceNetworkMapTriggerEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_network_map_trigger_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_network_map_trigger_entry }
}

impl<'a> Extract<'a> for ItemResourceNetworkMapTriggerEntry {
    const TYPE_NAME: &'static str = "ItemResourceNetworkMapTriggerEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemResourceNetworkTypeUIData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceNetworkTypeUIData {
    /// DCB field: `toolTip` (Locale)
    #[serde(default)]
    pub tool_tip: String,
    /// DCB field: `typeIcon` (String)
    #[serde(default)]
    pub type_icon: String,
}

impl Pooled for ItemResourceNetworkTypeUIData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_network_type_uidata }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_network_type_uidata }
}

impl<'a> Extract<'a> for ItemResourceNetworkTypeUIData {
    const TYPE_NAME: &'static str = "ItemResourceNetworkTypeUIData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tool_tip: inst.get_str("toolTip").map(String::from).unwrap_or_default(),
            type_icon: inst.get_str("typeIcon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemResourceBoxoutUIParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceBoxoutUIParams {
    /// DCB field: `temperature` (Class)
    #[serde(default)]
    pub temperature: Option<Handle<BoxoutStat>>,
    /// DCB field: `health` (Class)
    #[serde(default)]
    pub health: Option<Handle<BoxoutStat>>,
    /// DCB field: `wear` (Class)
    #[serde(default)]
    pub wear: Option<Handle<BoxoutStat>>,
    /// DCB field: `filter` (Class)
    #[serde(default)]
    pub filter: Option<Handle<BoxoutStat>>,
    /// DCB field: `stateArray` (Class)
    #[serde(default)]
    pub state_array: Option<Handle<BoxoutItemStatus>>,
    /// DCB field: `pressure` (Class)
    #[serde(default)]
    pub pressure: Option<Handle<BoxoutStat>>,
    /// DCB field: `atmosphereGood` (Class)
    #[serde(default)]
    pub atmosphere_good: Option<Handle<BoxoutAtmosphereStatus>>,
    /// DCB field: `atmosphereWarning` (Class)
    #[serde(default)]
    pub atmosphere_warning: Option<Handle<BoxoutAtmosphereStatus>>,
    /// DCB field: `atmosphereDanger` (Class)
    #[serde(default)]
    pub atmosphere_danger: Option<Handle<BoxoutAtmosphereStatus>>,
    /// DCB field: `leftPivot` (Class)
    #[serde(default)]
    pub left_pivot: Option<Handle<Vec3>>,
    /// DCB field: `leftAnchor` (Class)
    #[serde(default)]
    pub left_anchor: Option<Handle<Vec3>>,
    /// DCB field: `rightPivot` (Class)
    #[serde(default)]
    pub right_pivot: Option<Handle<Vec3>>,
    /// DCB field: `rightAnchor` (Class)
    #[serde(default)]
    pub right_anchor: Option<Handle<Vec3>>,
}

impl Pooled for ItemResourceBoxoutUIParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_boxout_uiparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_boxout_uiparams }
}

impl<'a> Extract<'a> for ItemResourceBoxoutUIParams {
    const TYPE_NAME: &'static str = "ItemResourceBoxoutUIParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            temperature: match inst.get("temperature") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BoxoutStat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoxoutStat>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            health: match inst.get("health") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BoxoutStat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoxoutStat>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wear: match inst.get("wear") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BoxoutStat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoxoutStat>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            filter: match inst.get("filter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BoxoutStat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoxoutStat>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            state_array: match inst.get("stateArray") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BoxoutItemStatus>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoxoutItemStatus>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pressure: match inst.get("pressure") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BoxoutStat>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoxoutStat>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            atmosphere_good: match inst.get("atmosphereGood") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BoxoutAtmosphereStatus>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoxoutAtmosphereStatus>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            atmosphere_warning: match inst.get("atmosphereWarning") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BoxoutAtmosphereStatus>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoxoutAtmosphereStatus>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            atmosphere_danger: match inst.get("atmosphereDanger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BoxoutAtmosphereStatus>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BoxoutAtmosphereStatus>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            left_pivot: match inst.get("leftPivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            left_anchor: match inst.get("leftAnchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            right_pivot: match inst.get("rightPivot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            right_anchor: match inst.get("rightAnchor") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemResourceNetworkUIParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceNetworkUIParams {
    /// DCB field: `typeData` (Class)
    #[serde(default)]
    pub type_data: Option<Handle<ItemResourceNetworkTypeUIData>>,
    /// DCB field: `exclusiveTypes` (EnumChoice (array))
    #[serde(default)]
    pub exclusive_types: Vec<String>,
    /// DCB field: `doorFilter` (Class)
    #[serde(default)]
    pub door_filter: Option<Handle<ItemResourceNetworkMapTriggerEntry>>,
    /// DCB field: `itemFilter` (Class)
    #[serde(default)]
    pub item_filter: Option<Handle<ItemResourceNetworkMapTriggerEntry>>,
    /// DCB field: `roomFilter` (Class)
    #[serde(default)]
    pub room_filter: Option<Handle<ItemResourceNetworkMapTriggerEntry>>,
    /// DCB field: `healthRanges` (Single)
    #[serde(default)]
    pub health_ranges: f32,
    /// DCB field: `wearRanges` (Single)
    #[serde(default)]
    pub wear_ranges: f32,
    /// DCB field: `notificationMessages` (Class)
    #[serde(default)]
    pub notification_messages: Option<Handle<EngineeringStateMessages>>,
    /// DCB field: `boxoutParams` (Class)
    #[serde(default)]
    pub boxout_params: Option<Handle<ItemResourceBoxoutUIParams>>,
    /// DCB field: `defaultPreset` (Locale)
    #[serde(default)]
    pub default_preset: String,
    /// DCB field: `unknownPreset` (Locale)
    #[serde(default)]
    pub unknown_preset: String,
}

impl Pooled for ItemResourceNetworkUIParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_network_uiparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_network_uiparams }
}

impl<'a> Extract<'a> for ItemResourceNetworkUIParams {
    const TYPE_NAME: &'static str = "ItemResourceNetworkUIParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            type_data: match inst.get("typeData") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceNetworkTypeUIData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceNetworkTypeUIData>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exclusive_types: inst.get_array("exclusiveTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            door_filter: match inst.get("doorFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceNetworkMapTriggerEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceNetworkMapTriggerEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_filter: match inst.get("itemFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceNetworkMapTriggerEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceNetworkMapTriggerEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            room_filter: match inst.get("roomFilter") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceNetworkMapTriggerEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceNetworkMapTriggerEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            health_ranges: inst.get_f32("healthRanges").unwrap_or_default(),
            wear_ranges: inst.get_f32("wearRanges").unwrap_or_default(),
            notification_messages: match inst.get("notificationMessages") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<EngineeringStateMessages>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<EngineeringStateMessages>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            boxout_params: match inst.get("boxoutParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceBoxoutUIParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceBoxoutUIParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_preset: inst.get_str("defaultPreset").map(String::from).unwrap_or_default(),
            unknown_preset: inst.get_str("unknownPreset").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemResourceNetworkPowerModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceNetworkPowerModifier {
    /// DCB field: `additiveModifier` (Int32)
    #[serde(default)]
    pub additive_modifier: i32,
}

impl Pooled for ItemResourceNetworkPowerModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_network_power_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_network_power_modifier }
}

impl<'a> Extract<'a> for ItemResourceNetworkPowerModifier {
    const TYPE_NAME: &'static str = "ItemResourceNetworkPowerModifier";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            additive_modifier: inst.get_i32("additiveModifier").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemResourceNetworkPowerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceNetworkPowerParams {
    /// DCB field: `modifiers` (Class)
    #[serde(default)]
    pub modifiers: Option<Handle<ItemResourceNetworkPowerModifier>>,
}

impl Pooled for ItemResourceNetworkPowerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_network_power_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_network_power_params }
}

impl<'a> Extract<'a> for ItemResourceNetworkPowerParams {
    const TYPE_NAME: &'static str = "ItemResourceNetworkPowerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            modifiers: match inst.get("modifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceNetworkPowerModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceNetworkPowerModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemResourceNetworkDefaultPowerDistributionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceNetworkDefaultPowerDistributionParams {
    /// DCB field: `powerDistPoolsFillMin` (Boolean)
    #[serde(default)]
    pub power_dist_pools_fill_min: bool,
    /// DCB field: `powerDistCoolerMin` (Single)
    #[serde(default)]
    pub power_dist_cooler_min: f32,
    /// DCB field: `powerDistPoolsProportion` (Single)
    #[serde(default)]
    pub power_dist_pools_proportion: f32,
    /// DCB field: `poolDefaultWeapons` (Single)
    #[serde(default)]
    pub pool_default_weapons: f32,
    /// DCB field: `poolDefaultEngines` (Single)
    #[serde(default)]
    pub pool_default_engines: f32,
    /// DCB field: `poolDefaultShields` (Single)
    #[serde(default)]
    pub pool_default_shields: f32,
}

impl Pooled for ItemResourceNetworkDefaultPowerDistributionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_network_default_power_distribution_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_network_default_power_distribution_params }
}

impl<'a> Extract<'a> for ItemResourceNetworkDefaultPowerDistributionParams {
    const TYPE_NAME: &'static str = "ItemResourceNetworkDefaultPowerDistributionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            power_dist_pools_fill_min: inst.get_bool("powerDistPoolsFillMin").unwrap_or_default(),
            power_dist_cooler_min: inst.get_f32("powerDistCoolerMin").unwrap_or_default(),
            power_dist_pools_proportion: inst.get_f32("powerDistPoolsProportion").unwrap_or_default(),
            pool_default_weapons: inst.get_f32("poolDefaultWeapons").unwrap_or_default(),
            pool_default_engines: inst.get_f32("poolDefaultEngines").unwrap_or_default(),
            pool_default_shields: inst.get_f32("poolDefaultShields").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemResourceNetworkGlobal`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResourceNetworkGlobal {
    /// DCB field: `networkResources` (Class)
    #[serde(default)]
    pub network_resources: Option<Handle<ItemResourceCompositionMap>>,
    /// DCB field: `resourcePairs` (Class (array))
    #[serde(default)]
    pub resource_pairs: Vec<Handle<ItemRoomResourcePair>>,
    /// DCB field: `itemResourceTypeData` (Class (array))
    #[serde(default)]
    pub item_resource_type_data: Vec<Handle<ItemResourceTypeData>>,
    /// DCB field: `maxPowerToCoolantRatio` (Single)
    #[serde(default)]
    pub max_power_to_coolant_ratio: f32,
    /// DCB field: `thrusterCoolantMultiplier` (Single)
    #[serde(default)]
    pub thruster_coolant_multiplier: f32,
    /// DCB field: `powerBaseConversionRate` (Single)
    #[serde(default)]
    pub power_base_conversion_rate: f32,
    /// DCB field: `uiParams` (Class)
    #[serde(default)]
    pub ui_params: Option<Handle<ItemResourceNetworkUIParams>>,
    /// DCB field: `powerParams` (Class)
    #[serde(default)]
    pub power_params: Option<Handle<ItemResourceNetworkPowerParams>>,
    /// DCB field: `defaultPowerDistributionParams` (Class)
    #[serde(default)]
    pub default_power_distribution_params: Option<Handle<ItemResourceNetworkDefaultPowerDistributionParams>>,
    /// DCB field: `misfireRepairGracePeriod` (Single)
    #[serde(default)]
    pub misfire_repair_grace_period: f32,
}

impl Pooled for ItemResourceNetworkGlobal {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_resource_network_global }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_resource_network_global }
}

impl<'a> Extract<'a> for ItemResourceNetworkGlobal {
    const TYPE_NAME: &'static str = "ItemResourceNetworkGlobal";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            network_resources: match inst.get("networkResources") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceCompositionMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceCompositionMap>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            resource_pairs: inst.get_array("resourcePairs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemRoomResourcePair>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemRoomResourcePair>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            item_resource_type_data: inst.get_array("itemResourceTypeData")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemResourceTypeData>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemResourceTypeData>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            max_power_to_coolant_ratio: inst.get_f32("maxPowerToCoolantRatio").unwrap_or_default(),
            thruster_coolant_multiplier: inst.get_f32("thrusterCoolantMultiplier").unwrap_or_default(),
            power_base_conversion_rate: inst.get_f32("powerBaseConversionRate").unwrap_or_default(),
            ui_params: match inst.get("uiParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceNetworkUIParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceNetworkUIParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            power_params: match inst.get("powerParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceNetworkPowerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceNetworkPowerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_power_distribution_params: match inst.get("defaultPowerDistributionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemResourceNetworkDefaultPowerDistributionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemResourceNetworkDefaultPowerDistributionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            misfire_repair_grace_period: inst.get_f32("misfireRepairGracePeriod").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemAwardBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAwardBase {
    /// DCB field: `amountToAward` (Int32)
    #[serde(default)]
    pub amount_to_award: i32,
}

impl Pooled for ItemAwardBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_award_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_award_base }
}

impl<'a> Extract<'a> for ItemAwardBase {
    const TYPE_NAME: &'static str = "ItemAwardBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            amount_to_award: inst.get_i32("amountToAward").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemAwardWeightings`
///
/// Inherits from: `ItemAwardWeightingsBase` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAwardWeightings {
    /// DCB field: `weighting` (Single)
    #[serde(default)]
    pub weighting: f32,
    /// DCB field: `awards` (StrongPointer (array))
    #[serde(default)]
    pub awards: Vec<Handle<ItemAwardBase>>,
}

impl Pooled for ItemAwardWeightings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_award_weightings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_award_weightings }
}

impl<'a> Extract<'a> for ItemAwardWeightings {
    const TYPE_NAME: &'static str = "ItemAwardWeightings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weighting: inst.get_f32("weighting").unwrap_or_default(),
            awards: inst.get_array("awards")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemAwardBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemAwardBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemAwardWeightingsRecord`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAwardWeightingsRecord {
    /// DCB field: `itemAwardStructure` (Class (array))
    #[serde(default)]
    pub item_award_structure: Vec<Handle<ItemAwardWeightings>>,
}

impl Pooled for ItemAwardWeightingsRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_award_weightings_record }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_award_weightings_record }
}

impl<'a> Extract<'a> for ItemAwardWeightingsRecord {
    const TYPE_NAME: &'static str = "ItemAwardWeightingsRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_award_structure: inst.get_array("itemAwardStructure")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemAwardWeightings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemAwardWeightings>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemRecoveryConfigurationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRecoveryConfigurationParams {
    /// DCB field: `nonEligibleItems` (StrongPointer (array))
    #[serde(default)]
    pub non_eligible_items: Vec<Handle<ItemRecoverySetConditionDef>>,
    /// DCB field: `consumableItems` (StrongPointer (array))
    #[serde(default)]
    pub consumable_items: Vec<Handle<ItemRecoverySetConditionDef>>,
    /// DCB field: `dontEquipBrickedItems` (StrongPointer (array))
    #[serde(default)]
    pub dont_equip_bricked_items: Vec<Handle<ItemRecoverySetConditionDef>>,
    /// DCB field: `economyParams` (Class)
    #[serde(default)]
    pub economy_params: Option<Handle<ItemRecoveryEconomyParams>>,
    /// DCB field: `notificationParams` (Class)
    #[serde(default)]
    pub notification_params: Option<Handle<ItemRecoveryNotificationParams>>,
}

impl Pooled for ItemRecoveryConfigurationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_recovery_configuration_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_recovery_configuration_params }
}

impl<'a> Extract<'a> for ItemRecoveryConfigurationParams {
    const TYPE_NAME: &'static str = "ItemRecoveryConfigurationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            non_eligible_items: inst.get_array("nonEligibleItems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemRecoverySetConditionDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemRecoverySetConditionDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            consumable_items: inst.get_array("consumableItems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemRecoverySetConditionDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemRecoverySetConditionDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            dont_equip_bricked_items: inst.get_array("dontEquipBrickedItems")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemRecoverySetConditionDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemRecoverySetConditionDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            economy_params: match inst.get("economyParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemRecoveryEconomyParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemRecoveryEconomyParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            notification_params: match inst.get("notificationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemRecoveryNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemRecoveryNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemRecoverySetConditionDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRecoverySetConditionDef {
}

impl Pooled for ItemRecoverySetConditionDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_recovery_set_condition_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_recovery_set_condition_def }
}

impl<'a> Extract<'a> for ItemRecoverySetConditionDef {
    const TYPE_NAME: &'static str = "ItemRecoverySetConditionDef";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ItemRecoveryNotificationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRecoveryNotificationParams {
    /// DCB field: `itemBricked_Title` (Locale)
    #[serde(default)]
    pub item_bricked_title: String,
    /// DCB field: `itemBricked_Body` (Locale)
    #[serde(default)]
    pub item_bricked_body: String,
    /// DCB field: `itemBricking_Title` (Locale)
    #[serde(default)]
    pub item_bricking_title: String,
    /// DCB field: `itemBricking_Body` (Locale)
    #[serde(default)]
    pub item_bricking_body: String,
    /// DCB field: `onBrickedItemUseNotificationBuffer` (Single)
    #[serde(default)]
    pub on_bricked_item_use_notification_buffer: f32,
    /// DCB field: `onBrickedItemUsed_Title` (Locale)
    #[serde(default)]
    pub on_bricked_item_used_title: String,
    /// DCB field: `onBrickedItemUsed_CannotEquip` (Locale)
    #[serde(default)]
    pub on_bricked_item_used_cannot_equip: String,
    /// DCB field: `onBrickedItemUsed_CannotFire` (Locale)
    #[serde(default)]
    pub on_bricked_item_used_cannot_fire: String,
    /// DCB field: `onBrickedItemUsed_CannotUse` (Locale)
    #[serde(default)]
    pub on_bricked_item_used_cannot_use: String,
}

impl Pooled for ItemRecoveryNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_recovery_notification_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_recovery_notification_params }
}

impl<'a> Extract<'a> for ItemRecoveryNotificationParams {
    const TYPE_NAME: &'static str = "ItemRecoveryNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_bricked_title: inst.get_str("itemBricked_Title").map(String::from).unwrap_or_default(),
            item_bricked_body: inst.get_str("itemBricked_Body").map(String::from).unwrap_or_default(),
            item_bricking_title: inst.get_str("itemBricking_Title").map(String::from).unwrap_or_default(),
            item_bricking_body: inst.get_str("itemBricking_Body").map(String::from).unwrap_or_default(),
            on_bricked_item_use_notification_buffer: inst.get_f32("onBrickedItemUseNotificationBuffer").unwrap_or_default(),
            on_bricked_item_used_title: inst.get_str("onBrickedItemUsed_Title").map(String::from).unwrap_or_default(),
            on_bricked_item_used_cannot_equip: inst.get_str("onBrickedItemUsed_CannotEquip").map(String::from).unwrap_or_default(),
            on_bricked_item_used_cannot_fire: inst.get_str("onBrickedItemUsed_CannotFire").map(String::from).unwrap_or_default(),
            on_bricked_item_used_cannot_use: inst.get_str("onBrickedItemUsed_CannotUse").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemRecoveryEconomyParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRecoveryEconomyParams {
    /// DCB field: `globalBrickTimer` (Single)
    #[serde(default)]
    pub global_brick_timer: f32,
    /// DCB field: `baseReclaimTime` (Single)
    #[serde(default)]
    pub base_reclaim_time: f32,
    /// DCB field: `scalingPriceFloor` (Single)
    #[serde(default)]
    pub scaling_price_floor: f32,
    /// DCB field: `aUECperSecond` (Single)
    #[serde(default)]
    pub a_uecper_second: f32,
    /// DCB field: `claimCostMultiplier` (Single)
    #[serde(default)]
    pub claim_cost_multiplier: f32,
}

impl Pooled for ItemRecoveryEconomyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_recovery_economy_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_recovery_economy_params }
}

impl<'a> Extract<'a> for ItemRecoveryEconomyParams {
    const TYPE_NAME: &'static str = "ItemRecoveryEconomyParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            global_brick_timer: inst.get_f32("globalBrickTimer").unwrap_or_default(),
            base_reclaim_time: inst.get_f32("baseReclaimTime").unwrap_or_default(),
            scaling_price_floor: inst.get_f32("scalingPriceFloor").unwrap_or_default(),
            a_uecper_second: inst.get_f32("aUECperSecond").unwrap_or_default(),
            claim_cost_multiplier: inst.get_f32("claimCostMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemCategory`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCategory {
    /// DCB field: `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
    /// DCB field: `subType` (EnumChoice)
    #[serde(default)]
    pub sub_type: String,
    /// DCB field: `requiredTag` (Reference)
    #[serde(default)]
    pub required_tag: Option<CigGuid>,
}

impl Pooled for ItemCategory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_category }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_category }
}

impl<'a> Extract<'a> for ItemCategory {
    const TYPE_NAME: &'static str = "ItemCategory";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
            sub_type: inst.get_str("subType").map(String::from).unwrap_or_default(),
            required_tag: inst.get("requiredTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ItemPreview_LightIntensities`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_LightIntensities {
    /// DCB field: `keyLightTopIntensity` (Single)
    #[serde(default)]
    pub key_light_top_intensity: f32,
    /// DCB field: `fillLightTopIntensity` (Single)
    #[serde(default)]
    pub fill_light_top_intensity: f32,
    /// DCB field: `rimLightTopIntensity` (Single)
    #[serde(default)]
    pub rim_light_top_intensity: f32,
    /// DCB field: `keyLightBottomIntensity` (Single)
    #[serde(default)]
    pub key_light_bottom_intensity: f32,
    /// DCB field: `fillLightBottomIntensity` (Single)
    #[serde(default)]
    pub fill_light_bottom_intensity: f32,
    /// DCB field: `rimLightBottomIntensity` (Single)
    #[serde(default)]
    pub rim_light_bottom_intensity: f32,
}

impl Pooled for ItemPreview_LightIntensities {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_preview_light_intensities }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_preview_light_intensities }
}

impl<'a> Extract<'a> for ItemPreview_LightIntensities {
    const TYPE_NAME: &'static str = "ItemPreview_LightIntensities";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            key_light_top_intensity: inst.get_f32("keyLightTopIntensity").unwrap_or_default(),
            fill_light_top_intensity: inst.get_f32("fillLightTopIntensity").unwrap_or_default(),
            rim_light_top_intensity: inst.get_f32("rimLightTopIntensity").unwrap_or_default(),
            key_light_bottom_intensity: inst.get_f32("keyLightBottomIntensity").unwrap_or_default(),
            fill_light_bottom_intensity: inst.get_f32("fillLightBottomIntensity").unwrap_or_default(),
            rim_light_bottom_intensity: inst.get_f32("rimLightBottomIntensity").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_LightingSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_LightingSettings {
    /// DCB field: `lightIntensities` (Class)
    #[serde(default)]
    pub light_intensities: Option<Handle<ItemPreview_LightIntensities>>,
    /// DCB field: `topAngle` (Single)
    #[serde(default)]
    pub top_angle: f32,
    /// DCB field: `bottomAngle` (Single)
    #[serde(default)]
    pub bottom_angle: f32,
    /// DCB field: `leftRightAngle` (Single)
    #[serde(default)]
    pub left_right_angle: f32,
    /// DCB field: `rimOffsetAngle` (Single)
    #[serde(default)]
    pub rim_offset_angle: f32,
    /// DCB field: `useEnvProbe` (Boolean)
    #[serde(default)]
    pub use_env_probe: bool,
    /// DCB field: `envProbeTexture` (String)
    #[serde(default)]
    pub env_probe_texture: String,
    /// DCB field: `envProbeMultiplier` (Single)
    #[serde(default)]
    pub env_probe_multiplier: f32,
    /// DCB field: `envProbeRadiusMultiplier` (Single)
    #[serde(default)]
    pub env_probe_radius_multiplier: f32,
}

impl Pooled for ItemPreview_LightingSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_preview_lighting_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_preview_lighting_settings }
}

impl<'a> Extract<'a> for ItemPreview_LightingSettings {
    const TYPE_NAME: &'static str = "ItemPreview_LightingSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_intensities: match inst.get("lightIntensities") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_LightIntensities>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_LightIntensities>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            top_angle: inst.get_f32("topAngle").unwrap_or_default(),
            bottom_angle: inst.get_f32("bottomAngle").unwrap_or_default(),
            left_right_angle: inst.get_f32("leftRightAngle").unwrap_or_default(),
            rim_offset_angle: inst.get_f32("rimOffsetAngle").unwrap_or_default(),
            use_env_probe: inst.get_bool("useEnvProbe").unwrap_or_default(),
            env_probe_texture: inst.get_str("envProbeTexture").map(String::from).unwrap_or_default(),
            env_probe_multiplier: inst.get_f32("envProbeMultiplier").unwrap_or_default(),
            env_probe_radius_multiplier: inst.get_f32("envProbeRadiusMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_SkinnedLoadoutOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_SkinnedLoadoutOverride {
    /// DCB field: `skinnedLoadout` (String)
    #[serde(default)]
    pub skinned_loadout: String,
    /// DCB field: `itemTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_types: Vec<String>,
}

impl Pooled for ItemPreview_SkinnedLoadoutOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_preview_skinned_loadout_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_preview_skinned_loadout_override }
}

impl<'a> Extract<'a> for ItemPreview_SkinnedLoadoutOverride {
    const TYPE_NAME: &'static str = "ItemPreview_SkinnedLoadoutOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            skinned_loadout: inst.get_str("skinnedLoadout").map(String::from).unwrap_or_default(),
            item_types: inst.get_array("itemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_CameraSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_CameraSettings {
    /// DCB field: `bones` (EnumChoice (array))
    #[serde(default)]
    pub bones: Vec<String>,
    /// DCB field: `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// DCB field: `distanceScaler` (Single)
    #[serde(default)]
    pub distance_scaler: f32,
    /// DCB field: `fieldOfView` (Single)
    #[serde(default)]
    pub field_of_view: f32,
    /// DCB field: `pitch` (Single)
    #[serde(default)]
    pub pitch: f32,
}

impl Pooled for ItemPreview_CameraSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_preview_camera_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_preview_camera_settings }
}

impl<'a> Extract<'a> for ItemPreview_CameraSettings {
    const TYPE_NAME: &'static str = "ItemPreview_CameraSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            bones: inst.get_array("bones")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            distance_scaler: inst.get_f32("distanceScaler").unwrap_or_default(),
            field_of_view: inst.get_f32("fieldOfView").unwrap_or_default(),
            pitch: inst.get_f32("pitch").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_CameraSettingsOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_CameraSettingsOverride {
    /// DCB field: `settings` (Class)
    #[serde(default)]
    pub settings: Option<Handle<ItemPreview_CameraSettings>>,
    /// DCB field: `itemTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_types: Vec<String>,
}

impl Pooled for ItemPreview_CameraSettingsOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_preview_camera_settings_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_preview_camera_settings_override }
}

impl<'a> Extract<'a> for ItemPreview_CameraSettingsOverride {
    const TYPE_NAME: &'static str = "ItemPreview_CameraSettingsOverride";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            settings: match inst.get("settings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_CameraSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_CameraSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_types: inst.get_array("itemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_TurntableSettings`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_TurntableSettings {
    /// DCB field: `initialRotation` (Class)
    #[serde(default)]
    pub initial_rotation: Option<Handle<Ang3>>,
    /// DCB field: `rotationChange` (Class)
    #[serde(default)]
    pub rotation_change: Option<Handle<Vec3>>,
}

impl Pooled for ItemPreview_TurntableSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_preview_turntable_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_preview_turntable_settings }
}

impl<'a> Extract<'a> for ItemPreview_TurntableSettings {
    const TYPE_NAME: &'static str = "ItemPreview_TurntableSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            initial_rotation: match inst.get("initialRotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_change: match inst.get("rotationChange") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemPreview_TurntableOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_TurntableOverride {
    /// DCB field: `settings` (Class)
    #[serde(default)]
    pub settings: Option<Handle<ItemPreview_TurntableSettings>>,
    /// DCB field: `itemTypes` (EnumChoice (array))
    #[serde(default)]
    pub item_types: Vec<String>,
}

impl Pooled for ItemPreview_TurntableOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_preview_turntable_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_preview_turntable_override }
}

impl<'a> Extract<'a> for ItemPreview_TurntableOverride {
    const TYPE_NAME: &'static str = "ItemPreview_TurntableOverride";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            settings: match inst.get("settings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_TurntableSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_TurntableSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_types: inst.get_array("itemTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPreview_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPreview_Config {
    /// DCB field: `dummyPoseAnim` (String)
    #[serde(default)]
    pub dummy_pose_anim: String,
    /// DCB field: `dummyBaseClass` (Reference)
    #[serde(default)]
    pub dummy_base_class: Option<CigGuid>,
    /// DCB field: `skinnedLoadoutDefault` (String)
    #[serde(default)]
    pub skinned_loadout_default: String,
    /// DCB field: `cameraSettingsDefault` (Class)
    #[serde(default)]
    pub camera_settings_default: Option<Handle<ItemPreview_CameraSettings>>,
    /// DCB field: `turntableSettingsDefault` (Class)
    #[serde(default)]
    pub turntable_settings_default: Option<Handle<ItemPreview_TurntableSettings>>,
    /// DCB field: `skinnedLoadoutOverrides` (Class (array))
    #[serde(default)]
    pub skinned_loadout_overrides: Vec<Handle<ItemPreview_SkinnedLoadoutOverride>>,
    /// DCB field: `cameraSettingsOverrides` (Class (array))
    #[serde(default)]
    pub camera_settings_overrides: Vec<Handle<ItemPreview_CameraSettingsOverride>>,
    /// DCB field: `turntableSettingsOverrides` (Class (array))
    #[serde(default)]
    pub turntable_settings_overrides: Vec<Handle<ItemPreview_TurntableOverride>>,
    /// DCB field: `lightingSettings` (Class)
    #[serde(default)]
    pub lighting_settings: Option<Handle<ItemPreview_LightingSettings>>,
    /// DCB field: `fadeDelay` (Single)
    #[serde(default)]
    pub fade_delay: f32,
    /// DCB field: `fadeTime` (Single)
    #[serde(default)]
    pub fade_time: f32,
}

impl Pooled for ItemPreview_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_preview_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_preview_config }
}

impl<'a> Extract<'a> for ItemPreview_Config {
    const TYPE_NAME: &'static str = "ItemPreview_Config";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            dummy_pose_anim: inst.get_str("dummyPoseAnim").map(String::from).unwrap_or_default(),
            dummy_base_class: inst.get("dummyBaseClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            skinned_loadout_default: inst.get_str("skinnedLoadoutDefault").map(String::from).unwrap_or_default(),
            camera_settings_default: match inst.get("cameraSettingsDefault") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_CameraSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_CameraSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turntable_settings_default: match inst.get("turntableSettingsDefault") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_TurntableSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_TurntableSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            skinned_loadout_overrides: inst.get_array("skinnedLoadoutOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemPreview_SkinnedLoadoutOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemPreview_SkinnedLoadoutOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            camera_settings_overrides: inst.get_array("cameraSettingsOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemPreview_CameraSettingsOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemPreview_CameraSettingsOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            turntable_settings_overrides: inst.get_array("turntableSettingsOverrides")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemPreview_TurntableOverride>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemPreview_TurntableOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            lighting_settings: match inst.get("lightingSettings") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemPreview_LightingSettings>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemPreview_LightingSettings>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fade_delay: inst.get_f32("fadeDelay").unwrap_or_default(),
            fade_time: inst.get_f32("fadeTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemTypeCategory`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeCategory {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
}

impl Pooled for ItemTypeCategory {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_type_category }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_type_category }
}

impl<'a> Extract<'a> for ItemTypeCategory {
    const TYPE_NAME: &'static str = "ItemTypeCategory";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemTypeCategoryException`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeCategoryException {
    /// DCB field: `subType` (EnumChoice)
    #[serde(default)]
    pub sub_type: String,
    /// DCB field: `category` (WeakPointer)
    #[serde(default)]
    pub category: Option<Handle<ItemTypeCategory>>,
    /// DCB field: `showInElectronicAccess` (Boolean)
    #[serde(default)]
    pub show_in_electronic_access: bool,
}

impl Pooled for ItemTypeCategoryException {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_type_category_exception }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_type_category_exception }
}

impl<'a> Extract<'a> for ItemTypeCategoryException {
    const TYPE_NAME: &'static str = "ItemTypeCategoryException";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            sub_type: inst.get_str("subType").map(String::from).unwrap_or_default(),
            category: match inst.get("category") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemTypeCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemTypeCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            show_in_electronic_access: inst.get_bool("showInElectronicAccess").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemTypeCategoryMap`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeCategoryMap {
    /// DCB field: `defaultCategory` (WeakPointer)
    #[serde(default)]
    pub default_category: Option<Handle<ItemTypeCategory>>,
    /// DCB field: `exceptions` (Class (array))
    #[serde(default)]
    pub exceptions: Vec<Handle<ItemTypeCategoryException>>,
    /// DCB field: `showInElectronicAccess` (Boolean)
    #[serde(default)]
    pub show_in_electronic_access: bool,
}

impl Pooled for ItemTypeCategoryMap {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_type_category_map }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_type_category_map }
}

impl<'a> Extract<'a> for ItemTypeCategoryMap {
    const TYPE_NAME: &'static str = "ItemTypeCategoryMap";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_category: match inst.get("defaultCategory") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemTypeCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemTypeCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            exceptions: inst.get_array("exceptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemTypeCategoryException>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemTypeCategoryException>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            show_in_electronic_access: inst.get_bool("showInElectronicAccess").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemTypeInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeInfo {
    /// DCB field: `typeName` (Locale)
    #[serde(default)]
    pub type_name: String,
    /// DCB field: `showInPlayerAssetManager` (Boolean)
    #[serde(default)]
    pub show_in_player_asset_manager: bool,
    /// DCB field: `categoryMap` (Class)
    #[serde(default)]
    pub category_map: Option<Handle<ItemTypeCategoryMap>>,
}

impl Pooled for ItemTypeInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_type_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_type_info }
}

impl<'a> Extract<'a> for ItemTypeInfo {
    const TYPE_NAME: &'static str = "ItemTypeInfo";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            type_name: inst.get_str("typeName").map(String::from).unwrap_or_default(),
            show_in_player_asset_manager: inst.get_bool("showInPlayerAssetManager").unwrap_or_default(),
            category_map: match inst.get("categoryMap") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemTypeCategoryMap>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemTypeCategoryMap>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemTypeDefinition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeDefinition {
    /// DCB field: `defaultCategory` (WeakPointer)
    #[serde(default)]
    pub default_category: Option<Handle<ItemTypeCategory>>,
    /// DCB field: `categories` (Class (array))
    #[serde(default)]
    pub categories: Vec<Handle<ItemTypeCategory>>,
    /// DCB field: `typeInfo` (Class)
    #[serde(default)]
    pub type_info: Option<Handle<ItemTypeInfo>>,
    /// DCB field: `allCategoriesLabel` (Locale)
    #[serde(default)]
    pub all_categories_label: String,
    /// DCB field: `allTypesLabel` (Locale)
    #[serde(default)]
    pub all_types_label: String,
}

impl Pooled for ItemTypeDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_type_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_type_definition }
}

impl<'a> Extract<'a> for ItemTypeDefinition {
    const TYPE_NAME: &'static str = "ItemTypeDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_category: match inst.get("defaultCategory") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemTypeCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemTypeCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            categories: inst.get_array("categories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemTypeCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemTypeCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            type_info: match inst.get("typeInfo") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ItemTypeInfo>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ItemTypeInfo>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            all_categories_label: inst.get_str("allCategoriesLabel").map(String::from).unwrap_or_default(),
            all_types_label: inst.get_str("allTypesLabel").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemPortViewInformation`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPortViewInformation {
}

impl Pooled for ItemPortViewInformation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_it.item_port_view_information }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_it.item_port_view_information }
}

impl<'a> Extract<'a> for ItemPortViewInformation {
    const TYPE_NAME: &'static str = "ItemPortViewInformation";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

