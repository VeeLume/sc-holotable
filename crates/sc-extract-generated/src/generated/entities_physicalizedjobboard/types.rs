// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-physicalizedjobboard`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SMissionAcceptGameplayTriggerNotifications`
pub struct SMissionAcceptGameplayTriggerNotifications {
    /// `unknownErrorNotification` (Reference)
    pub unknown_error_notification: Option<CigGuid>,
    /// `noMatchingContractsFoundNotification` (Reference)
    pub no_matching_contracts_found_notification: Option<CigGuid>,
    /// `missionAlreadyActiveNotification` (Reference)
    pub mission_already_active_notification: Option<CigGuid>,
}

impl Pooled for SMissionAcceptGameplayTriggerNotifications {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_physicalizedjobboard.smission_accept_gameplay_trigger_notifications }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_physicalizedjobboard.smission_accept_gameplay_trigger_notifications }
}

impl<'a> Extract<'a> for SMissionAcceptGameplayTriggerNotifications {
    const TYPE_NAME: &'static str = "SMissionAcceptGameplayTriggerNotifications";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            unknown_error_notification: inst.get("unknownErrorNotification").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            no_matching_contracts_found_notification: inst.get("noMatchingContractsFoundNotification").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_already_active_notification: inst.get("missionAlreadyActiveNotification").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SMissionAcceptGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
pub struct SMissionAcceptGameplayTrigger {
    /// `contractID` (Guid)
    pub contract_id: CigGuid,
    /// `debugContractName` (String)
    pub debug_contract_name: String,
    /// `notifications` (Class)
    pub notifications: Option<Handle<SMissionAcceptGameplayTriggerNotifications>>,
}

impl Pooled for SMissionAcceptGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_physicalizedjobboard.smission_accept_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_physicalizedjobboard.smission_accept_gameplay_trigger }
}

impl<'a> Extract<'a> for SMissionAcceptGameplayTrigger {
    const TYPE_NAME: &'static str = "SMissionAcceptGameplayTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            contract_id: inst.get_guid("contractID").unwrap_or_default(),
            debug_contract_name: inst.get_str("debugContractName").map(String::from).unwrap_or_default(),
            notifications: match inst.get("notifications") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMissionAcceptGameplayTriggerNotifications>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

