// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `friendmanager`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `FriendManagerNotificationsParams`
pub struct FriendManagerNotificationsParams {
    /// `friendAdded` (Class)
    pub friend_added: Option<Handle<PlayerNotificationBannerParams>>,
    /// `friendRequestReceived` (Class)
    pub friend_request_received: Option<Handle<PlayerNotificationBannerParams>>,
    /// `friendRequestDeclined` (Class)
    pub friend_request_declined: Option<Handle<PlayerNotificationBannerParams>>,
}

impl Pooled for FriendManagerNotificationsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.friendmanager.friend_manager_notifications_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.friendmanager.friend_manager_notifications_params
    }
}

impl<'a> Extract<'a> for FriendManagerNotificationsParams {
    const TYPE_NAME: &'static str = "FriendManagerNotificationsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            friend_added: match inst.get("friendAdded") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<PlayerNotificationBannerParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            friend_request_received: match inst.get("friendRequestReceived") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<PlayerNotificationBannerParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            friend_request_declined: match inst.get("friendRequestDeclined") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<PlayerNotificationBannerParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `FriendManagerGlobalParams`
pub struct FriendManagerGlobalParams {
    /// `notificationsParams` (Class)
    pub notifications_params: Option<Handle<FriendManagerNotificationsParams>>,
}

impl Pooled for FriendManagerGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.friendmanager.friend_manager_global_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.friendmanager.friend_manager_global_params
    }
}

impl<'a> Extract<'a> for FriendManagerGlobalParams {
    const TYPE_NAME: &'static str = "FriendManagerGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            notifications_params: match inst.get("notificationsParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<FriendManagerNotificationsParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}
