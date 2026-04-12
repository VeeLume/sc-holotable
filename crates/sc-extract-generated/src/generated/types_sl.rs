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

/// DCB type: `SLocomotionPersonalityStateFilter`
///
/// Inherits from: `ActorMotionStateFilter` (fields flattened)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLocomotionPersonalityStateFilter {
    /// DCB field: `filterName` (String)
    #[serde(default)]
    pub filter_name: String,
    /// DCB field: `filterByState` (EnumChoice)
    #[serde(default)]
    pub filter_by_state: String,
    /// DCB field: `filterByMotionSpeed` (EnumChoice)
    #[serde(default)]
    pub filter_by_motion_speed: String,
    /// DCB field: `filterByPoseState` (EnumChoice)
    #[serde(default)]
    pub filter_by_pose_state: String,
    /// DCB field: `filterByStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_stance_state: String,
    /// DCB field: `filterByAimStanceState` (EnumChoice)
    #[serde(default)]
    pub filter_by_aim_stance_state: String,
    /// DCB field: `filterByLeanState` (EnumChoice)
    #[serde(default)]
    pub filter_by_lean_state: String,
    /// DCB field: `filterByHeldItemType` (EnumChoice)
    #[serde(default)]
    pub filter_by_held_item_type: String,
    /// DCB field: `filterBySkeleton` (EnumChoice)
    #[serde(default)]
    pub filter_by_skeleton: String,
    /// DCB field: `filterByCharacterType` (EnumChoice)
    #[serde(default)]
    pub filter_by_character_type: String,
    /// DCB field: `filterByRestrainedState` (EnumChoice)
    #[serde(default)]
    pub filter_by_restrained_state: String,
    /// DCB field: `filterByPlayerCamera` (EnumChoice)
    #[serde(default)]
    pub filter_by_player_camera: String,
    /// DCB field: `filterByAimingRestriction` (EnumChoice)
    #[serde(default)]
    pub filter_by_aiming_restriction: String,
    /// DCB field: `filterByLocomotionSet` (EnumChoice)
    #[serde(default)]
    pub filter_by_locomotion_set: String,
    /// DCB field: `filterByMannequinGlobalTags` (String)
    #[serde(default)]
    pub filter_by_mannequin_global_tags: String,
}

impl Pooled for SLocomotionPersonalityStateFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sl.slocomotion_personality_state_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sl.slocomotion_personality_state_filter }
}

impl<'a> Extract<'a> for SLocomotionPersonalityStateFilter {
    const TYPE_NAME: &'static str = "SLocomotionPersonalityStateFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            filter_name: inst.get_str("filterName").map(String::from).unwrap_or_default(),
            filter_by_state: inst.get_str("filterByState").map(String::from).unwrap_or_default(),
            filter_by_motion_speed: inst.get_str("filterByMotionSpeed").map(String::from).unwrap_or_default(),
            filter_by_pose_state: inst.get_str("filterByPoseState").map(String::from).unwrap_or_default(),
            filter_by_stance_state: inst.get_str("filterByStanceState").map(String::from).unwrap_or_default(),
            filter_by_aim_stance_state: inst.get_str("filterByAimStanceState").map(String::from).unwrap_or_default(),
            filter_by_lean_state: inst.get_str("filterByLeanState").map(String::from).unwrap_or_default(),
            filter_by_held_item_type: inst.get_str("filterByHeldItemType").map(String::from).unwrap_or_default(),
            filter_by_skeleton: inst.get_str("filterBySkeleton").map(String::from).unwrap_or_default(),
            filter_by_character_type: inst.get_str("filterByCharacterType").map(String::from).unwrap_or_default(),
            filter_by_restrained_state: inst.get_str("filterByRestrainedState").map(String::from).unwrap_or_default(),
            filter_by_player_camera: inst.get_str("filterByPlayerCamera").map(String::from).unwrap_or_default(),
            filter_by_aiming_restriction: inst.get_str("filterByAimingRestriction").map(String::from).unwrap_or_default(),
            filter_by_locomotion_set: inst.get_str("filterByLocomotionSet").map(String::from).unwrap_or_default(),
            filter_by_mannequin_global_tags: inst.get_str("filterByMannequinGlobalTags").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SLoadoutInventoryItem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLoadoutInventoryItem {
    /// DCB field: `entityClassName` (String)
    #[serde(default)]
    pub entity_class_name: String,
    /// DCB field: `entityClassReference` (Reference)
    #[serde(default)]
    pub entity_class_reference: Option<CigGuid>,
    /// DCB field: `amount` (Int32)
    #[serde(default)]
    pub amount: i32,
}

impl Pooled for SLoadoutInventoryItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sl.sloadout_inventory_item }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sl.sloadout_inventory_item }
}

impl<'a> Extract<'a> for SLoadoutInventoryItem {
    const TYPE_NAME: &'static str = "SLoadoutInventoryItem";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_class_name: inst.get_str("entityClassName").map(String::from).unwrap_or_default(),
            entity_class_reference: inst.get("entityClassReference").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            amount: inst.get_i32("amount").unwrap_or_default(),
        }
    }
}

/// DCB type: `SLoadoutRequirementBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLoadoutRequirementBase {
}

impl Pooled for SLoadoutRequirementBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sl.sloadout_requirement_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sl.sloadout_requirement_base }
}

impl<'a> Extract<'a> for SLoadoutRequirementBase {
    const TYPE_NAME: &'static str = "SLoadoutRequirementBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SLoadoutAssortment`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLoadoutAssortment {
    /// DCB field: `Requirement` (StrongPointer)
    #[serde(default)]
    pub requirement: Option<Handle<SLoadoutRequirementBase>>,
    /// DCB field: `GroupedLoadouts` (Class (array))
    #[serde(default)]
    pub grouped_loadouts: Vec<Handle<SGroupedLoadouts>>,
}

impl Pooled for SLoadoutAssortment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sl.sloadout_assortment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sl.sloadout_assortment }
}

impl<'a> Extract<'a> for SLoadoutAssortment {
    const TYPE_NAME: &'static str = "SLoadoutAssortment";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            requirement: match inst.get("Requirement") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SLoadoutRequirementBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SLoadoutRequirementBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            grouped_loadouts: inst.get_array("GroupedLoadouts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SGroupedLoadouts>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SGroupedLoadouts>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SLoadingScreenInformationDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLoadingScreenInformationDef {
    /// DCB field: `loadingScreenType` (EnumChoice)
    #[serde(default)]
    pub loading_screen_type: String,
    /// DCB field: `imagePath` (String)
    #[serde(default)]
    pub image_path: String,
    /// DCB field: `imagePath_2k` (String)
    #[serde(default)]
    pub image_path_2k: String,
    /// DCB field: `imagePath_4k` (String)
    #[serde(default)]
    pub image_path_4k: String,
    /// DCB field: `primaryTitle` (Locale)
    #[serde(default)]
    pub primary_title: String,
    /// DCB field: `secondaryTitle` (Locale)
    #[serde(default)]
    pub secondary_title: String,
    /// DCB field: `subtitle` (Locale)
    #[serde(default)]
    pub subtitle: String,
    /// DCB field: `description` (Locale)
    #[serde(default)]
    pub description: String,
    /// DCB field: `tips` (Locale (array))
    #[serde(default)]
    pub tips: Vec<String>,
}

impl Pooled for SLoadingScreenInformationDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sl.sloading_screen_information_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sl.sloading_screen_information_def }
}

impl<'a> Extract<'a> for SLoadingScreenInformationDef {
    const TYPE_NAME: &'static str = "SLoadingScreenInformationDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            loading_screen_type: inst.get_str("loadingScreenType").map(String::from).unwrap_or_default(),
            image_path: inst.get_str("imagePath").map(String::from).unwrap_or_default(),
            image_path_2k: inst.get_str("imagePath_2k").map(String::from).unwrap_or_default(),
            image_path_4k: inst.get_str("imagePath_4k").map(String::from).unwrap_or_default(),
            primary_title: inst.get_str("primaryTitle").map(String::from).unwrap_or_default(),
            secondary_title: inst.get_str("secondaryTitle").map(String::from).unwrap_or_default(),
            subtitle: inst.get_str("subtitle").map(String::from).unwrap_or_default(),
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            tips: inst.get_array("tips")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SLocalPlayerShoppingData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLocalPlayerShoppingData {
    /// DCB field: `defaultARParams` (Class)
    #[serde(default)]
    pub default_arparams: Option<Handle<SItemShopARParams>>,
    /// DCB field: `predefinedARParams` (Class (array))
    #[serde(default)]
    pub predefined_arparams: Vec<Handle<SLocalPlayerShoppingPredefinedARParams>>,
    /// DCB field: `silhouette` (Class)
    #[serde(default)]
    pub silhouette: Option<Handle<HUDSilhouetteParams>>,
    /// DCB field: `keyHoldDuration` (Single)
    #[serde(default)]
    pub key_hold_duration: f32,
    /// DCB field: `notificationConfig` (Class)
    #[serde(default)]
    pub notification_config: Option<Handle<SLocalPlayerShoppingNotificationConfiguration>>,
    /// DCB field: `transactionComplete` (Locale)
    #[serde(default)]
    pub transaction_complete: String,
    /// DCB field: `transactionFail` (Locale)
    #[serde(default)]
    pub transaction_fail: String,
}

impl Pooled for SLocalPlayerShoppingData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sl.slocal_player_shopping_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sl.slocal_player_shopping_data }
}

impl<'a> Extract<'a> for SLocalPlayerShoppingData {
    const TYPE_NAME: &'static str = "SLocalPlayerShoppingData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_arparams: match inst.get("defaultARParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemShopARParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemShopARParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            predefined_arparams: inst.get_array("predefinedARParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SLocalPlayerShoppingPredefinedARParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SLocalPlayerShoppingPredefinedARParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            silhouette: match inst.get("silhouette") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<HUDSilhouetteParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<HUDSilhouetteParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            key_hold_duration: inst.get_f32("keyHoldDuration").unwrap_or_default(),
            notification_config: match inst.get("notificationConfig") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SLocalPlayerShoppingNotificationConfiguration>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SLocalPlayerShoppingNotificationConfiguration>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            transaction_complete: inst.get_str("transactionComplete").map(String::from).unwrap_or_default(),
            transaction_fail: inst.get_str("transactionFail").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SLocalPlayerShoppingPredefinedARParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLocalPlayerShoppingPredefinedARParams {
    /// DCB field: `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// DCB field: `itemTypes` (Class (array))
    #[serde(default)]
    pub item_types: Vec<Handle<SItemPortDefTypes>>,
    /// DCB field: `arParams` (Class)
    #[serde(default)]
    pub ar_params: Option<Handle<SItemShopARParams>>,
}

impl Pooled for SLocalPlayerShoppingPredefinedARParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sl.slocal_player_shopping_predefined_arparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sl.slocal_player_shopping_predefined_arparams }
}

impl<'a> Extract<'a> for SLocalPlayerShoppingPredefinedARParams {
    const TYPE_NAME: &'static str = "SLocalPlayerShoppingPredefinedARParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            item_types: inst.get_array("itemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemPortDefTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortDefTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            ar_params: match inst.get("arParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemShopARParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemShopARParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SLocalPlayerShoppingNotificationConfiguration`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLocalPlayerShoppingNotificationConfiguration {
    /// DCB field: `screenTime` (Single)
    #[serde(default)]
    pub screen_time: f32,
    /// DCB field: `audioEvent` (String)
    #[serde(default)]
    pub audio_event: String,
    /// DCB field: `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// DCB field: `showIcon` (Boolean)
    #[serde(default)]
    pub show_icon: bool,
    /// DCB field: `isLowPriority` (Boolean)
    #[serde(default)]
    pub is_low_priority: bool,
}

impl Pooled for SLocalPlayerShoppingNotificationConfiguration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sl.slocal_player_shopping_notification_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sl.slocal_player_shopping_notification_configuration }
}

impl<'a> Extract<'a> for SLocalPlayerShoppingNotificationConfiguration {
    const TYPE_NAME: &'static str = "SLocalPlayerShoppingNotificationConfiguration";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            screen_time: inst.get_f32("screenTime").unwrap_or_default(),
            audio_event: inst.get_str("audioEvent").map(String::from).unwrap_or_default(),
            icon_path: inst.get_str("iconPath").map(String::from).unwrap_or_default(),
            show_icon: inst.get_bool("showIcon").unwrap_or_default(),
            is_low_priority: inst.get_bool("isLowPriority").unwrap_or_default(),
        }
    }
}

