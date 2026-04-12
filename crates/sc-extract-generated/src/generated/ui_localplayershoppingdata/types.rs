// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-localplayershoppingdata`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SItemShopReference`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemShopReference {
    /// `reference` (EnumChoice)
    #[serde(default)]
    pub reference: String,
    /// `adjustmentMode` (EnumChoice)
    #[serde(default)]
    pub adjustment_mode: String,
    /// `adjustmentValue` (Class)
    #[serde(default)]
    pub adjustment_value: Option<Handle<Vec3>>,
}

impl Pooled for SItemShopReference {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_localplayershoppingdata.sitem_shop_reference }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_localplayershoppingdata.sitem_shop_reference }
}

impl<'a> Extract<'a> for SItemShopReference {
    const TYPE_NAME: &'static str = "SItemShopReference";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            reference: inst.get_str("reference").map(String::from).unwrap_or_default(),
            adjustment_mode: inst.get_str("adjustmentMode").map(String::from).unwrap_or_default(),
            adjustment_value: match inst.get("adjustmentValue") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SItemShopARParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SItemShopARParams {
    /// `showOnCenter` (Boolean)
    #[serde(default)]
    pub show_on_center: bool,
    /// `distancePositionUpdate` (Single)
    #[serde(default)]
    pub distance_position_update: f32,
    /// `updateOrientationEveryFrame` (Boolean)
    #[serde(default)]
    pub update_orientation_every_frame: bool,
    /// `silhouetteWithRack` (Boolean)
    #[serde(default)]
    pub silhouette_with_rack: bool,
    /// `bounds` (Class)
    #[serde(default)]
    pub bounds: Option<Handle<SItemShopReference>>,
    /// `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<SItemShopReference>>,
    /// `orientation` (Class)
    #[serde(default)]
    pub orientation: Option<Handle<SItemShopReference>>,
    /// `faceMinX` (Boolean)
    #[serde(default)]
    pub face_min_x: bool,
    /// `faceMaxX` (Boolean)
    #[serde(default)]
    pub face_max_x: bool,
    /// `faceMinY` (Boolean)
    #[serde(default)]
    pub face_min_y: bool,
    /// `faceMaxY` (Boolean)
    #[serde(default)]
    pub face_max_y: bool,
    /// `faceMinZ` (Boolean)
    #[serde(default)]
    pub face_min_z: bool,
    /// `faceMaxZ` (Boolean)
    #[serde(default)]
    pub face_max_z: bool,
}

impl Pooled for SItemShopARParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_localplayershoppingdata.sitem_shop_arparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_localplayershoppingdata.sitem_shop_arparams }
}

impl<'a> Extract<'a> for SItemShopARParams {
    const TYPE_NAME: &'static str = "SItemShopARParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            show_on_center: inst.get_bool("showOnCenter").unwrap_or_default(),
            distance_position_update: inst.get_f32("distancePositionUpdate").unwrap_or_default(),
            update_orientation_every_frame: inst.get_bool("updateOrientationEveryFrame").unwrap_or_default(),
            silhouette_with_rack: inst.get_bool("silhouetteWithRack").unwrap_or_default(),
            bounds: match inst.get("bounds") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemShopReference>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemShopReference>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemShopReference>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemShopReference>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            orientation: match inst.get("orientation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SItemShopReference>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemShopReference>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            face_min_x: inst.get_bool("faceMinX").unwrap_or_default(),
            face_max_x: inst.get_bool("faceMaxX").unwrap_or_default(),
            face_min_y: inst.get_bool("faceMinY").unwrap_or_default(),
            face_max_y: inst.get_bool("faceMaxY").unwrap_or_default(),
            face_min_z: inst.get_bool("faceMinZ").unwrap_or_default(),
            face_max_z: inst.get_bool("faceMaxZ").unwrap_or_default(),
        }
    }
}

/// DCB type: `SLocalPlayerShoppingData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLocalPlayerShoppingData {
    /// `defaultARParams` (Class)
    #[serde(default)]
    pub default_arparams: Option<Handle<SItemShopARParams>>,
    /// `predefinedARParams` (Class (array))
    #[serde(default)]
    pub predefined_arparams: Vec<Handle<SLocalPlayerShoppingPredefinedARParams>>,
    /// `silhouette` (Class)
    #[serde(default)]
    pub silhouette: Option<Handle<HUDSilhouetteParams>>,
    /// `keyHoldDuration` (Single)
    #[serde(default)]
    pub key_hold_duration: f32,
    /// `notificationConfig` (Class)
    #[serde(default)]
    pub notification_config: Option<Handle<SLocalPlayerShoppingNotificationConfiguration>>,
    /// `transactionComplete` (Locale)
    #[serde(default)]
    pub transaction_complete: String,
    /// `transactionFail` (Locale)
    #[serde(default)]
    pub transaction_fail: String,
}

impl Pooled for SLocalPlayerShoppingData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_localplayershoppingdata.slocal_player_shopping_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_localplayershoppingdata.slocal_player_shopping_data }
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
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `itemTypes` (Class (array))
    #[serde(default)]
    pub item_types: Vec<Handle<SItemPortDefTypes>>,
    /// `arParams` (Class)
    #[serde(default)]
    pub ar_params: Option<Handle<SItemShopARParams>>,
}

impl Pooled for SLocalPlayerShoppingPredefinedARParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_localplayershoppingdata.slocal_player_shopping_predefined_arparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_localplayershoppingdata.slocal_player_shopping_predefined_arparams }
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
    /// `screenTime` (Single)
    #[serde(default)]
    pub screen_time: f32,
    /// `audioEvent` (String)
    #[serde(default)]
    pub audio_event: String,
    /// `iconPath` (String)
    #[serde(default)]
    pub icon_path: String,
    /// `showIcon` (Boolean)
    #[serde(default)]
    pub show_icon: bool,
    /// `isLowPriority` (Boolean)
    #[serde(default)]
    pub is_low_priority: bool,
}

impl Pooled for SLocalPlayerShoppingNotificationConfiguration {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_localplayershoppingdata.slocal_player_shopping_notification_configuration }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_localplayershoppingdata.slocal_player_shopping_notification_configuration }
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

