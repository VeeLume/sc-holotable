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

/// DCB type: `FriendlyFireReactionOverride`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendlyFireReactionOverride {
    /// DCB field: `reactionType` (EnumChoice)
    #[serde(default)]
    pub reaction_type: String,
    /// DCB field: `shouldAllowFriendlyFire` (Boolean)
    #[serde(default)]
    pub should_allow_friendly_fire: bool,
}

impl Pooled for FriendlyFireReactionOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fr.friendly_fire_reaction_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fr.friendly_fire_reaction_override }
}

impl<'a> Extract<'a> for FriendlyFireReactionOverride {
    const TYPE_NAME: &'static str = "FriendlyFireReactionOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reaction_type: inst.get_str("reactionType").map(String::from).unwrap_or_default(),
            should_allow_friendly_fire: inst.get_bool("shouldAllowFriendlyFire").unwrap_or_default(),
        }
    }
}

/// DCB type: `FriendManagerNotificationsParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendManagerNotificationsParams {
    /// DCB field: `friendAdded` (Class)
    #[serde(default)]
    pub friend_added: Option<Handle<PlayerNotificationBannerParams>>,
    /// DCB field: `friendRequestReceived` (Class)
    #[serde(default)]
    pub friend_request_received: Option<Handle<PlayerNotificationBannerParams>>,
    /// DCB field: `friendRequestDeclined` (Class)
    #[serde(default)]
    pub friend_request_declined: Option<Handle<PlayerNotificationBannerParams>>,
}

impl Pooled for FriendManagerNotificationsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fr.friend_manager_notifications_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fr.friend_manager_notifications_params }
}

impl<'a> Extract<'a> for FriendManagerNotificationsParams {
    const TYPE_NAME: &'static str = "FriendManagerNotificationsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            friend_added: match inst.get("friendAdded") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerNotificationBannerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerNotificationBannerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            friend_request_received: match inst.get("friendRequestReceived") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerNotificationBannerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerNotificationBannerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            friend_request_declined: match inst.get("friendRequestDeclined") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<PlayerNotificationBannerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<PlayerNotificationBannerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FriendManagerGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendManagerGlobalParams {
    /// DCB field: `notificationsParams` (Class)
    #[serde(default)]
    pub notifications_params: Option<Handle<FriendManagerNotificationsParams>>,
}

impl Pooled for FriendManagerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fr.friend_manager_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fr.friend_manager_global_params }
}

impl<'a> Extract<'a> for FriendManagerGlobalParams {
    const TYPE_NAME: &'static str = "FriendManagerGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            notifications_params: match inst.get("notificationsParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<FriendManagerNotificationsParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<FriendManagerNotificationsParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `FrontendOverrideParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendOverrideParams {
    /// DCB field: `persistentUniverseActive` (Boolean)
    #[serde(default)]
    pub persistent_universe_active: bool,
    /// DCB field: `arenaCommanderActive` (Boolean)
    #[serde(default)]
    pub arena_commander_active: bool,
    /// DCB field: `tutorialDisabled` (Boolean)
    #[serde(default)]
    pub tutorial_disabled: bool,
    /// DCB field: `disableResidenceSelectionWarning` (Boolean)
    #[serde(default)]
    pub disable_residence_selection_warning: bool,
    /// DCB field: `backgroundVideoPath` (String)
    #[serde(default)]
    pub background_video_path: String,
    /// DCB field: `disabledSystems` (Reference (array))
    #[serde(default)]
    pub disabled_systems: Vec<CigGuid>,
}

impl Pooled for FrontendOverrideParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fr.frontend_override_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fr.frontend_override_params }
}

impl<'a> Extract<'a> for FrontendOverrideParams {
    const TYPE_NAME: &'static str = "FrontendOverrideParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            persistent_universe_active: inst.get_bool("persistentUniverseActive").unwrap_or_default(),
            arena_commander_active: inst.get_bool("arenaCommanderActive").unwrap_or_default(),
            tutorial_disabled: inst.get_bool("tutorialDisabled").unwrap_or_default(),
            disable_residence_selection_warning: inst.get_bool("disableResidenceSelectionWarning").unwrap_or_default(),
            background_video_path: inst.get_str("backgroundVideoPath").map(String::from).unwrap_or_default(),
            disabled_systems: inst.get_array("disabledSystems")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FragmentInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentInfo {
    /// DCB field: `fragment` (String)
    #[serde(default)]
    pub fragment: String,
    /// DCB field: `additionalTags` (String (array))
    #[serde(default)]
    pub additional_tags: Vec<String>,
}

impl Pooled for FragmentInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fr.fragment_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fr.fragment_info }
}

impl<'a> Extract<'a> for FragmentInfo {
    const TYPE_NAME: &'static str = "FragmentInfo";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fragment: inst.get_str("fragment").map(String::from).unwrap_or_default(),
            additional_tags: inst.get_array("additionalTags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `FragmentRequiredInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentRequiredInfo {
    /// DCB field: `fragment` (String)
    #[serde(default)]
    pub fragment: String,
    /// DCB field: `additionalTags` (String (array))
    #[serde(default)]
    pub additional_tags: Vec<String>,
}

impl Pooled for FragmentRequiredInfo {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_fr.fragment_required_info }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_fr.fragment_required_info }
}

impl<'a> Extract<'a> for FragmentRequiredInfo {
    const TYPE_NAME: &'static str = "FragmentRequiredInfo";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            fragment: inst.get_str("fragment").map(String::from).unwrap_or_default(),
            additional_tags: inst.get_array("additionalTags")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

