// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `lawsystem`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SecurityNetworkRoomSettings`
pub struct SecurityNetworkRoomSettings {
    /// `defaultProtocol` (Class)
    pub default_protocol: Option<Handle<SecurityNetworkProtocol>>,
    /// `roomIdentifier` (Reference)
    pub room_identifier: Option<CigGuid>,
    /// `trespassWarningSeconds` (Single)
    pub trespass_warning_seconds: f32,
    /// `trespassRevokeWarningSeconds` (Single)
    pub trespass_revoke_warning_seconds: f32,
}

impl Pooled for SecurityNetworkRoomSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.lawsystem.security_network_room_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.lawsystem.security_network_room_settings }
}

impl<'a> Extract<'a> for SecurityNetworkRoomSettings {
    const TYPE_NAME: &'static str = "SecurityNetworkRoomSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_protocol: match inst.get("defaultProtocol") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SecurityNetworkProtocol>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            room_identifier: inst.get("roomIdentifier").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            trespass_warning_seconds: inst.get_f32("trespassWarningSeconds").unwrap_or_default(),
            trespass_revoke_warning_seconds: inst.get_f32("trespassRevokeWarningSeconds").unwrap_or_default(),
        }
    }
}

