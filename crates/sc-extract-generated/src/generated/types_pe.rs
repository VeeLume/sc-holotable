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

/// DCB type: `PersonalThoughtCameraEffectsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtCameraEffectsParams {
    /// DCB field: `openingDelay` (Single)
    #[serde(default)]
    pub opening_delay: f32,
    /// DCB field: `closingDelay` (Single)
    #[serde(default)]
    pub closing_delay: f32,
    /// DCB field: `backgroundExposure` (Single)
    #[serde(default)]
    pub background_exposure: f32,
    /// DCB field: `dynamicCameraEffectsRecord` (Reference)
    #[serde(default)]
    pub dynamic_camera_effects_record: Option<CigGuid>,
}

impl Pooled for PersonalThoughtCameraEffectsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_camera_effects_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_camera_effects_params }
}

impl<'a> Extract<'a> for PersonalThoughtCameraEffectsParams {
    const TYPE_NAME: &'static str = "PersonalThoughtCameraEffectsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            opening_delay: inst.get_f32("openingDelay").unwrap_or_default(),
            closing_delay: inst.get_f32("closingDelay").unwrap_or_default(),
            background_exposure: inst.get_f32("backgroundExposure").unwrap_or_default(),
            dynamic_camera_effects_record: inst.get("dynamicCameraEffectsRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `PersonalInnerThoughtActionRulePreset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInnerThoughtActionRulePreset {
    /// DCB field: `rules` (StrongPointer (array))
    #[serde(default)]
    pub rules: Vec<Handle<ActionRuleParams>>,
}

impl Pooled for PersonalInnerThoughtActionRulePreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_inner_thought_action_rule_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_inner_thought_action_rule_preset }
}

impl<'a> Extract<'a> for PersonalInnerThoughtActionRulePreset {
    const TYPE_NAME: &'static str = "PersonalInnerThoughtActionRulePreset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActionRuleParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActionRuleParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtActionsRulesParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtActionsRulesParams {
    /// DCB field: `actionRulesList` (StrongPointer (array))
    #[serde(default)]
    pub action_rules_list: Vec<Handle<ActionRuleList>>,
}

impl Pooled for PersonalThoughtActionsRulesParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_actions_rules_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_actions_rules_params }
}

impl<'a> Extract<'a> for PersonalThoughtActionsRulesParams {
    const TYPE_NAME: &'static str = "PersonalThoughtActionsRulesParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            action_rules_list: inst.get_array("actionRulesList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ActionRuleList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ActionRuleList>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtActionDescription`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtActionDescription {
    /// DCB field: `inputAction` (Class)
    #[serde(default)]
    pub input_action: Option<Handle<InputAction>>,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
}

impl Pooled for PersonalThoughtActionDescription {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_action_description }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_action_description }
}

impl<'a> Extract<'a> for PersonalThoughtActionDescription {
    const TYPE_NAME: &'static str = "PersonalThoughtActionDescription";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input_action: match inst.get("inputAction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtActionDescriptionsList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtActionDescriptionsList {
    /// DCB field: `actionDescriptions` (Class (array))
    #[serde(default)]
    pub action_descriptions: Vec<Handle<PersonalThoughtActionDescription>>,
}

impl Pooled for PersonalThoughtActionDescriptionsList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_action_descriptions_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_action_descriptions_list }
}

impl<'a> Extract<'a> for PersonalThoughtActionDescriptionsList {
    const TYPE_NAME: &'static str = "PersonalThoughtActionDescriptionsList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            action_descriptions: inst.get_array("actionDescriptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PersonalThoughtActionDescription>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PersonalThoughtActionDescription>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtHologramActionsList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtHologramActionsList {
    /// DCB field: `hologramActions` (Class (array))
    #[serde(default)]
    pub hologram_actions: Vec<Handle<InputAction>>,
}

impl Pooled for PersonalThoughtHologramActionsList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_hologram_actions_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_hologram_actions_list }
}

impl<'a> Extract<'a> for PersonalThoughtHologramActionsList {
    const TYPE_NAME: &'static str = "PersonalThoughtHologramActionsList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hologram_actions: inst.get_array("hologramActions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtOption`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtOption {
    /// DCB field: `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
}

impl Pooled for PersonalThoughtOption {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_option }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_option }
}

impl<'a> Extract<'a> for PersonalThoughtOption {
    const TYPE_NAME: &'static str = "PersonalThoughtOption";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtCategoryAction`
///
/// Inherits from: `PersonalThoughtAction` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtCategoryAction {
    /// DCB field: `text` (Locale)
    #[serde(default)]
    pub text: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `inputAction` (Class)
    #[serde(default)]
    pub input_action: Option<Handle<InputAction>>,
    /// DCB field: `optionArray` (StrongPointer (array))
    #[serde(default)]
    pub option_array: Vec<Handle<PersonalThoughtOption>>,
}

impl Pooled for PersonalThoughtCategoryAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_category_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_category_action }
}

impl<'a> Extract<'a> for PersonalThoughtCategoryAction {
    const TYPE_NAME: &'static str = "PersonalThoughtCategoryAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            text: inst.get_str("text").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            input_action: match inst.get("inputAction") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            option_array: inst.get_array("optionArray")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PersonalThoughtOption>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PersonalThoughtOption>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtInventoryFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtInventoryFilter {
    /// DCB field: `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// DCB field: `displayIconPath` (String)
    #[serde(default)]
    pub display_icon_path: String,
    /// DCB field: `disableForFPSKiosk` (Boolean)
    #[serde(default)]
    pub disable_for_fpskiosk: bool,
}

impl Pooled for PersonalThoughtInventoryFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_inventory_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_inventory_filter }
}

impl<'a> Extract<'a> for PersonalThoughtInventoryFilter {
    const TYPE_NAME: &'static str = "PersonalThoughtInventoryFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            display_icon_path: inst.get_str("displayIconPath").map(String::from).unwrap_or_default(),
            disable_for_fpskiosk: inst.get_bool("disableForFPSKiosk").unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtInventoryParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtInventoryParams {
    /// DCB field: `proximityDetectionParams` (Class)
    #[serde(default)]
    pub proximity_detection_params: Option<Handle<ProximityInventoryDetectionParams>>,
    /// DCB field: `lootingScreenParams` (Class)
    #[serde(default)]
    pub looting_screen_params: Option<Handle<PersonalThoughtLootingScreenParams>>,
    /// DCB field: `inventoryDropInteractionConditions` (Reference)
    #[serde(default)]
    pub inventory_drop_interaction_conditions: Option<CigGuid>,
    /// DCB field: `inventoryTitle` (Locale)
    #[serde(default)]
    pub inventory_title: String,
    /// DCB field: `capacityBarTitle` (Locale)
    #[serde(default)]
    pub capacity_bar_title: String,
    /// DCB field: `defaultItemDisplayThumbnail` (String)
    #[serde(default)]
    pub default_item_display_thumbnail: String,
    /// DCB field: `emptyInventoryMessage` (Locale)
    #[serde(default)]
    pub empty_inventory_message: String,
    /// DCB field: `fullInventoryMessage` (Locale)
    #[serde(default)]
    pub full_inventory_message: String,
    /// DCB field: `noSpaceForItemInInventoryMessage` (Locale)
    #[serde(default)]
    pub no_space_for_item_in_inventory_message: String,
    /// DCB field: `itemCantFitInInventoryMessage` (Locale)
    #[serde(default)]
    pub item_cant_fit_in_inventory_message: String,
    /// DCB field: `genericInvalidMovementMessage` (Locale)
    #[serde(default)]
    pub generic_invalid_movement_message: String,
    /// DCB field: `filters` (StrongPointer (array))
    #[serde(default)]
    pub filters: Vec<Handle<PersonalThoughtInventoryFilter>>,
    /// DCB field: `inventorySortMenu` (Class)
    #[serde(default)]
    pub inventory_sort_menu: Option<Handle<PersonalThoughtInventorySortMenu>>,
    /// DCB field: `inventoryActionsParams` (Class)
    #[serde(default)]
    pub inventory_actions_params: Option<Handle<PersonalThoughtInventoryActionsParams>>,
    /// DCB field: `emptyBackpackPopWindowParams` (Class)
    #[serde(default)]
    pub empty_backpack_pop_window_params: Option<Handle<PersonalThoughtPopWindowParams>>,
    /// DCB field: `inventoryGridParams` (Class)
    #[serde(default)]
    pub inventory_grid_params: Option<Handle<PersonalThoughtInventoryGridParams>>,
    /// DCB field: `defaultOrientation` (Class)
    #[serde(default)]
    pub default_orientation: Option<Handle<Vec3>>,
    /// DCB field: `listOfItemsOrientationOffset` (Class (array))
    #[serde(default)]
    pub list_of_items_orientation_offset: Vec<Handle<PersonalThoughtInventoryItemOrientationOffset>>,
    /// DCB field: `defaultLightGroupEntityClass` (Reference)
    #[serde(default)]
    pub default_light_group_entity_class: Option<CigGuid>,
    /// DCB field: `listOfItemLightGroups` (Class (array))
    #[serde(default)]
    pub list_of_item_light_groups: Vec<Handle<PersonalThoughtInventoryItemLightGroup>>,
    /// DCB field: `defaultDisplayIconType` (String)
    #[serde(default)]
    pub default_display_icon_type: String,
    /// DCB field: `itemClassUIIconsList` (Class (array))
    #[serde(default)]
    pub item_class_uiicons_list: Vec<Handle<PersonalThoughtInventoryItemUIIcon>>,
}

impl Pooled for PersonalThoughtInventoryParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_inventory_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_inventory_params }
}

impl<'a> Extract<'a> for PersonalThoughtInventoryParams {
    const TYPE_NAME: &'static str = "PersonalThoughtInventoryParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            proximity_detection_params: match inst.get("proximityDetectionParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ProximityInventoryDetectionParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ProximityInventoryDetectionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            looting_screen_params: match inst.get("lootingScreenParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtLootingScreenParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtLootingScreenParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inventory_drop_interaction_conditions: inst.get("inventoryDropInteractionConditions").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            inventory_title: inst.get_str("inventoryTitle").map(String::from).unwrap_or_default(),
            capacity_bar_title: inst.get_str("capacityBarTitle").map(String::from).unwrap_or_default(),
            default_item_display_thumbnail: inst.get_str("defaultItemDisplayThumbnail").map(String::from).unwrap_or_default(),
            empty_inventory_message: inst.get_str("emptyInventoryMessage").map(String::from).unwrap_or_default(),
            full_inventory_message: inst.get_str("fullInventoryMessage").map(String::from).unwrap_or_default(),
            no_space_for_item_in_inventory_message: inst.get_str("noSpaceForItemInInventoryMessage").map(String::from).unwrap_or_default(),
            item_cant_fit_in_inventory_message: inst.get_str("itemCantFitInInventoryMessage").map(String::from).unwrap_or_default(),
            generic_invalid_movement_message: inst.get_str("genericInvalidMovementMessage").map(String::from).unwrap_or_default(),
            filters: inst.get_array("filters")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PersonalThoughtInventoryFilter>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PersonalThoughtInventoryFilter>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            inventory_sort_menu: match inst.get("inventorySortMenu") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtInventorySortMenu>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtInventorySortMenu>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inventory_actions_params: match inst.get("inventoryActionsParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtInventoryActionsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtInventoryActionsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            empty_backpack_pop_window_params: match inst.get("emptyBackpackPopWindowParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtPopWindowParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtPopWindowParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inventory_grid_params: match inst.get("inventoryGridParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtInventoryGridParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtInventoryGridParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_orientation: match inst.get("defaultOrientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            list_of_items_orientation_offset: inst.get_array("listOfItemsOrientationOffset")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PersonalThoughtInventoryItemOrientationOffset>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PersonalThoughtInventoryItemOrientationOffset>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_light_group_entity_class: inst.get("defaultLightGroupEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            list_of_item_light_groups: inst.get_array("listOfItemLightGroups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PersonalThoughtInventoryItemLightGroup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PersonalThoughtInventoryItemLightGroup>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_display_icon_type: inst.get_str("defaultDisplayIconType").map(String::from).unwrap_or_default(),
            item_class_uiicons_list: inst.get_array("itemClassUIIconsList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PersonalThoughtInventoryItemUIIcon>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PersonalThoughtInventoryItemUIIcon>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtInventorySortMenu`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtInventorySortMenu {
    /// DCB field: `modes` (Class (array))
    #[serde(default)]
    pub modes: Vec<Handle<InventorySortMode>>,
}

impl Pooled for PersonalThoughtInventorySortMenu {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_inventory_sort_menu }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_inventory_sort_menu }
}

impl<'a> Extract<'a> for PersonalThoughtInventorySortMenu {
    const TYPE_NAME: &'static str = "PersonalThoughtInventorySortMenu";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            modes: inst.get_array("modes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventorySortMode>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventorySortMode>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtLootingScreenParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtLootingScreenParams {
    /// DCB field: `itemPortClasses` (Class (array))
    #[serde(default)]
    pub item_port_classes: Vec<Handle<LootingItemPortSizeClass>>,
    /// DCB field: `lootingInventoryParams` (Class)
    #[serde(default)]
    pub looting_inventory_params: Option<Handle<LootingInventoryParams>>,
}

impl Pooled for PersonalThoughtLootingScreenParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_looting_screen_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_looting_screen_params }
}

impl<'a> Extract<'a> for PersonalThoughtLootingScreenParams {
    const TYPE_NAME: &'static str = "PersonalThoughtLootingScreenParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_port_classes: inst.get_array("itemPortClasses")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LootingItemPortSizeClass>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<LootingItemPortSizeClass>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            looting_inventory_params: match inst.get("lootingInventoryParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<LootingInventoryParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<LootingInventoryParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PersonalThoughtPopWindowParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtPopWindowParams {
    /// DCB field: `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// DCB field: `acceptButtonText` (Locale)
    #[serde(default)]
    pub accept_button_text: String,
    /// DCB field: `cancelButtonText` (Locale)
    #[serde(default)]
    pub cancel_button_text: String,
}

impl Pooled for PersonalThoughtPopWindowParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_pop_window_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_pop_window_params }
}

impl<'a> Extract<'a> for PersonalThoughtPopWindowParams {
    const TYPE_NAME: &'static str = "PersonalThoughtPopWindowParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
            accept_button_text: inst.get_str("acceptButtonText").map(String::from).unwrap_or_default(),
            cancel_button_text: inst.get_str("cancelButtonText").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtInventoryActionsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtInventoryActionsParams {
    /// DCB field: `dropItemActionThumbnail` (String)
    #[serde(default)]
    pub drop_item_action_thumbnail: String,
    /// DCB field: `emptyBackpackActionThumbnail` (String)
    #[serde(default)]
    pub empty_backpack_action_thumbnail: String,
}

impl Pooled for PersonalThoughtInventoryActionsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_inventory_actions_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_inventory_actions_params }
}

impl<'a> Extract<'a> for PersonalThoughtInventoryActionsParams {
    const TYPE_NAME: &'static str = "PersonalThoughtInventoryActionsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            drop_item_action_thumbnail: inst.get_str("dropItemActionThumbnail").map(String::from).unwrap_or_default(),
            empty_backpack_action_thumbnail: inst.get_str("emptyBackpackActionThumbnail").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtInventoryGridParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtInventoryGridParams {
    /// DCB field: `panelWidth` (Single)
    #[serde(default)]
    pub panel_width: f32,
    /// DCB field: `panelPadding` (Single)
    #[serde(default)]
    pub panel_padding: f32,
    /// DCB field: `numberOfGrids` (Int32)
    #[serde(default)]
    pub number_of_grids: i32,
    /// DCB field: `itemPadding` (Single)
    #[serde(default)]
    pub item_padding: f32,
    /// DCB field: `paddingWidth` (Single)
    #[serde(default)]
    pub padding_width: f32,
    /// DCB field: `paddingHeight` (Single)
    #[serde(default)]
    pub padding_height: f32,
    /// DCB field: `tabsNavHeight` (Single)
    #[serde(default)]
    pub tabs_nav_height: f32,
    /// DCB field: `containerTitleHeight` (Single)
    #[serde(default)]
    pub container_title_height: f32,
    /// DCB field: `spaceBetweenContainers` (Single)
    #[serde(default)]
    pub space_between_containers: f32,
    /// DCB field: `sizeMultiplicatorForPit` (Single)
    #[serde(default)]
    pub size_multiplicator_for_pit: f32,
    /// DCB field: `sizeMultiplicatorForKiosk` (Single)
    #[serde(default)]
    pub size_multiplicator_for_kiosk: f32,
    /// DCB field: `fixedGridSizeMultiplicatorV4` (Single)
    #[serde(default)]
    pub fixed_grid_size_multiplicator_v4: f32,
    /// DCB field: `bigContainerCapacity` (StrongPointer)
    #[serde(default)]
    pub big_container_capacity: Option<Handle<SBaseCargoUnit>>,
    /// DCB field: `inventoryContainers` (Class (array))
    #[serde(default)]
    pub inventory_containers: Vec<Handle<InventoryContainerParams>>,
}

impl Pooled for PersonalThoughtInventoryGridParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_inventory_grid_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_inventory_grid_params }
}

impl<'a> Extract<'a> for PersonalThoughtInventoryGridParams {
    const TYPE_NAME: &'static str = "PersonalThoughtInventoryGridParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            panel_width: inst.get_f32("panelWidth").unwrap_or_default(),
            panel_padding: inst.get_f32("panelPadding").unwrap_or_default(),
            number_of_grids: inst.get_i32("numberOfGrids").unwrap_or_default(),
            item_padding: inst.get_f32("itemPadding").unwrap_or_default(),
            padding_width: inst.get_f32("paddingWidth").unwrap_or_default(),
            padding_height: inst.get_f32("paddingHeight").unwrap_or_default(),
            tabs_nav_height: inst.get_f32("tabsNavHeight").unwrap_or_default(),
            container_title_height: inst.get_f32("containerTitleHeight").unwrap_or_default(),
            space_between_containers: inst.get_f32("spaceBetweenContainers").unwrap_or_default(),
            size_multiplicator_for_pit: inst.get_f32("sizeMultiplicatorForPit").unwrap_or_default(),
            size_multiplicator_for_kiosk: inst.get_f32("sizeMultiplicatorForKiosk").unwrap_or_default(),
            fixed_grid_size_multiplicator_v4: inst.get_f32("fixedGridSizeMultiplicatorV4").unwrap_or_default(),
            big_container_capacity: match inst.get("bigContainerCapacity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SBaseCargoUnit>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SBaseCargoUnit>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            inventory_containers: inst.get_array("inventoryContainers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InventoryContainerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InventoryContainerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtInventoryItemOrientationOffset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtInventoryItemOrientationOffset {
    /// DCB field: `orientationOffset` (Class)
    #[serde(default)]
    pub orientation_offset: Option<Handle<Vec3>>,
    /// DCB field: `itemCategories` (Class (array))
    #[serde(default)]
    pub item_categories: Vec<Handle<ItemCategory>>,
}

impl Pooled for PersonalThoughtInventoryItemOrientationOffset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_inventory_item_orientation_offset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_inventory_item_orientation_offset }
}

impl<'a> Extract<'a> for PersonalThoughtInventoryItemOrientationOffset {
    const TYPE_NAME: &'static str = "PersonalThoughtInventoryItemOrientationOffset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            orientation_offset: match inst.get("orientationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_categories: inst.get_array("itemCategories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtInventoryItemLightGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtInventoryItemLightGroup {
    /// DCB field: `lightGroupEntityClass` (Reference)
    #[serde(default)]
    pub light_group_entity_class: Option<CigGuid>,
    /// DCB field: `itemCategories` (Class (array))
    #[serde(default)]
    pub item_categories: Vec<Handle<ItemCategory>>,
}

impl Pooled for PersonalThoughtInventoryItemLightGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_inventory_item_light_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_inventory_item_light_group }
}

impl<'a> Extract<'a> for PersonalThoughtInventoryItemLightGroup {
    const TYPE_NAME: &'static str = "PersonalThoughtInventoryItemLightGroup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            light_group_entity_class: inst.get("lightGroupEntityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            item_categories: inst.get_array("itemCategories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ItemCategory>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ItemCategory>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtInventoryItemUIIcon`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtInventoryItemUIIcon {
    /// DCB field: `displayIconType` (String)
    #[serde(default)]
    pub display_icon_type: String,
    /// DCB field: `itemCategories` (Class (array))
    #[serde(default)]
    pub item_categories: Vec<Handle<TypeSubtypeParams>>,
}

impl Pooled for PersonalThoughtInventoryItemUIIcon {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_inventory_item_uiicon }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_inventory_item_uiicon }
}

impl<'a> Extract<'a> for PersonalThoughtInventoryItemUIIcon {
    const TYPE_NAME: &'static str = "PersonalThoughtInventoryItemUIIcon";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            display_icon_type: inst.get_str("displayIconType").map(String::from).unwrap_or_default(),
            item_categories: inst.get_array("itemCategories")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TypeSubtypeParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<TypeSubtypeParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtContextualActionsMenu`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtContextualActionsMenu {
    /// DCB field: `enabledActionMaps` (String (array))
    #[serde(default)]
    pub enabled_action_maps: Vec<String>,
}

impl Pooled for PersonalThoughtContextualActionsMenu {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_contextual_actions_menu }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_contextual_actions_menu }
}

impl<'a> Extract<'a> for PersonalThoughtContextualActionsMenu {
    const TYPE_NAME: &'static str = "PersonalThoughtContextualActionsMenu";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enabled_action_maps: inst.get_array("enabledActionMaps")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtGameModeDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtGameModeDef {
    /// DCB field: `disabledActions` (Class (array))
    #[serde(default)]
    pub disabled_actions: Vec<Handle<InputAction>>,
}

impl Pooled for PersonalThoughtGameModeDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_game_mode_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_game_mode_def }
}

impl<'a> Extract<'a> for PersonalThoughtGameModeDef {
    const TYPE_NAME: &'static str = "PersonalThoughtGameModeDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            disabled_actions: inst.get_array("disabledActions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtContextualActionsMenusParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtContextualActionsMenusParams {
    /// DCB field: `contextualMenus` (Class)
    #[serde(default)]
    pub contextual_menus: Option<Handle<PersonalThoughtContextualActionsMenu>>,
    /// DCB field: `gameModeDef` (Class)
    #[serde(default)]
    pub game_mode_def: Option<Handle<PersonalThoughtGameModeDef>>,
}

impl Pooled for PersonalThoughtContextualActionsMenusParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_contextual_actions_menus_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_contextual_actions_menus_params }
}

impl<'a> Extract<'a> for PersonalThoughtContextualActionsMenusParams {
    const TYPE_NAME: &'static str = "PersonalThoughtContextualActionsMenusParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            contextual_menus: match inst.get("contextualMenus") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtContextualActionsMenu>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtContextualActionsMenu>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            game_mode_def: match inst.get("gameModeDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtGameModeDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtGameModeDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PersonalThoughtQuickAccessWheel`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtQuickAccessWheel {
    /// DCB field: `name` (String)
    #[serde(default)]
    pub name: String,
    /// DCB field: `triggerInputActions` (Class (array))
    #[serde(default)]
    pub trigger_input_actions: Vec<Handle<InputAction>>,
    /// DCB field: `triggerEventHash` (String)
    #[serde(default)]
    pub trigger_event_hash: String,
    /// DCB field: `saveSelection` (Boolean)
    #[serde(default)]
    pub save_selection: bool,
    /// DCB field: `elements` (StrongPointer (array))
    #[serde(default)]
    pub elements: Vec<Handle<QuickAccessWheelElement>>,
}

impl Pooled for PersonalThoughtQuickAccessWheel {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_quick_access_wheel }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_quick_access_wheel }
}

impl<'a> Extract<'a> for PersonalThoughtQuickAccessWheel {
    const TYPE_NAME: &'static str = "PersonalThoughtQuickAccessWheel";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            trigger_input_actions: inst.get_array("triggerInputActions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<InputAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<InputAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            trigger_event_hash: inst.get_str("triggerEventHash").map(String::from).unwrap_or_default(),
            save_selection: inst.get_bool("saveSelection").unwrap_or_default(),
            elements: inst.get_array("elements")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<QuickAccessWheelElement>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<QuickAccessWheelElement>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtQuickAccessWheels`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtQuickAccessWheels {
    /// DCB field: `mouseCursorRadius` (Single)
    #[serde(default)]
    pub mouse_cursor_radius: f32,
    /// DCB field: `nonMouseCursorRadius` (Single)
    #[serde(default)]
    pub non_mouse_cursor_radius: f32,
    /// DCB field: `quickAccessWheels` (Class (array))
    #[serde(default)]
    pub quick_access_wheels: Vec<Handle<PersonalThoughtQuickAccessWheel>>,
    /// DCB field: `angularSelectWheelRadius` (Single)
    #[serde(default)]
    pub angular_select_wheel_radius: f32,
    /// DCB field: `angularSelectWheelRadiusSubmenu` (Single)
    #[serde(default)]
    pub angular_select_wheel_radius_submenu: f32,
    /// DCB field: `angularSelectWheelRadiusV2` (Single)
    #[serde(default)]
    pub angular_select_wheel_radius_v2: f32,
    /// DCB field: `angularSelectWheelRadiusSubmenuV2` (Single)
    #[serde(default)]
    pub angular_select_wheel_radius_submenu_v2: f32,
}

impl Pooled for PersonalThoughtQuickAccessWheels {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_quick_access_wheels }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_quick_access_wheels }
}

impl<'a> Extract<'a> for PersonalThoughtQuickAccessWheels {
    const TYPE_NAME: &'static str = "PersonalThoughtQuickAccessWheels";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mouse_cursor_radius: inst.get_f32("mouseCursorRadius").unwrap_or_default(),
            non_mouse_cursor_radius: inst.get_f32("nonMouseCursorRadius").unwrap_or_default(),
            quick_access_wheels: inst.get_array("quickAccessWheels")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<PersonalThoughtQuickAccessWheel>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<PersonalThoughtQuickAccessWheel>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            angular_select_wheel_radius: inst.get_f32("angularSelectWheelRadius").unwrap_or_default(),
            angular_select_wheel_radius_submenu: inst.get_f32("angularSelectWheelRadiusSubmenu").unwrap_or_default(),
            angular_select_wheel_radius_v2: inst.get_f32("angularSelectWheelRadiusV2").unwrap_or_default(),
            angular_select_wheel_radius_submenu_v2: inst.get_f32("angularSelectWheelRadiusSubmenuV2").unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtHologramAnimationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtHologramAnimationParams {
    /// DCB field: `selectedMaxScaling` (Single)
    #[serde(default)]
    pub selected_max_scaling: f32,
    /// DCB field: `scalingAnimationDuration` (Single)
    #[serde(default)]
    pub scaling_animation_duration: f32,
    /// DCB field: `selectedMaxYaw` (Single)
    #[serde(default)]
    pub selected_max_yaw: f32,
    /// DCB field: `rotationAnimationRate` (Single)
    #[serde(default)]
    pub rotation_animation_rate: f32,
    /// DCB field: `rotationRampDuration` (Single)
    #[serde(default)]
    pub rotation_ramp_duration: f32,
}

impl Pooled for PersonalThoughtHologramAnimationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_hologram_animation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_hologram_animation_params }
}

impl<'a> Extract<'a> for PersonalThoughtHologramAnimationParams {
    const TYPE_NAME: &'static str = "PersonalThoughtHologramAnimationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            selected_max_scaling: inst.get_f32("selectedMaxScaling").unwrap_or_default(),
            scaling_animation_duration: inst.get_f32("scalingAnimationDuration").unwrap_or_default(),
            selected_max_yaw: inst.get_f32("selectedMaxYaw").unwrap_or_default(),
            rotation_animation_rate: inst.get_f32("rotationAnimationRate").unwrap_or_default(),
            rotation_ramp_duration: inst.get_f32("rotationRampDuration").unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtHologramParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtHologramParams {
    /// DCB field: `mainColour` (Class)
    #[serde(default)]
    pub main_colour: Option<Handle<RGBA>>,
    /// DCB field: `selectedColour` (Class)
    #[serde(default)]
    pub selected_colour: Option<Handle<RGBA>>,
    /// DCB field: `defaultHoloMaterialName` (String)
    #[serde(default)]
    pub default_holo_material_name: String,
    /// DCB field: `selectedHoloMaterialName` (String)
    #[serde(default)]
    pub selected_holo_material_name: String,
    /// DCB field: `animationParams` (Class)
    #[serde(default)]
    pub animation_params: Option<Handle<PersonalThoughtHologramAnimationParams>>,
}

impl Pooled for PersonalThoughtHologramParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_hologram_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_hologram_params }
}

impl<'a> Extract<'a> for PersonalThoughtHologramParams {
    const TYPE_NAME: &'static str = "PersonalThoughtHologramParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            main_colour: match inst.get("mainColour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            selected_colour: match inst.get("selectedColour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<RGBA>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<RGBA>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            default_holo_material_name: inst.get_str("defaultHoloMaterialName").map(String::from).unwrap_or_default(),
            selected_holo_material_name: inst.get_str("selectedHoloMaterialName").map(String::from).unwrap_or_default(),
            animation_params: match inst.get("animationParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtHologramAnimationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtHologramAnimationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PersonalThoughtForceCloseActionList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtForceCloseActionList {
    /// DCB field: `actions` (String (array))
    #[serde(default)]
    pub actions: Vec<String>,
}

impl Pooled for PersonalThoughtForceCloseActionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_force_close_action_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_force_close_action_list }
}

impl<'a> Extract<'a> for PersonalThoughtForceCloseActionList {
    const TYPE_NAME: &'static str = "PersonalThoughtForceCloseActionList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            actions: inst.get_array("actions")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `PersonalThoughtForceCloseActionsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalThoughtForceCloseActionsParams {
    /// DCB field: `globalActions` (Class)
    #[serde(default)]
    pub global_actions: Option<Handle<PersonalThoughtForceCloseActionList>>,
    /// DCB field: `actionsPerGameMode` (Class)
    #[serde(default)]
    pub actions_per_game_mode: Option<Handle<PersonalThoughtForceCloseActionList>>,
}

impl Pooled for PersonalThoughtForceCloseActionsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_pe.personal_thought_force_close_actions_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_pe.personal_thought_force_close_actions_params }
}

impl<'a> Extract<'a> for PersonalThoughtForceCloseActionsParams {
    const TYPE_NAME: &'static str = "PersonalThoughtForceCloseActionsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            global_actions: match inst.get("globalActions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtForceCloseActionList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtForceCloseActionList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            actions_per_game_mode: match inst.get("actionsPerGameMode") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PersonalThoughtForceCloseActionList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PersonalThoughtForceCloseActionList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

