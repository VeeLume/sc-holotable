// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-characters`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `NotKillableState`
/// Inherits from: `VulnerabilityState`
pub struct NotKillableState {
    /// `notKillable` (Boolean)
    pub not_killable: bool,
}

impl Pooled for NotKillableState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_characters.not_killable_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_characters.not_killable_state }
}

impl<'a> Extract<'a> for NotKillableState {
    const TYPE_NAME: &'static str = "NotKillableState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            not_killable: inst.get_bool("notKillable").unwrap_or_default(),
        }
    }
}

/// DCB type: `SetHealthVulnerabilityStateGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
pub struct SetHealthVulnerabilityStateGameplayTrigger {
    /// `vulnerabilityState` (StrongPointer)
    pub vulnerability_state: Option<VulnerabilityStatePtr>,
}

impl Pooled for SetHealthVulnerabilityStateGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_characters.set_health_vulnerability_state_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_characters.set_health_vulnerability_state_gameplay_trigger }
}

impl<'a> Extract<'a> for SetHealthVulnerabilityStateGameplayTrigger {
    const TYPE_NAME: &'static str = "SetHealthVulnerabilityStateGameplayTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            vulnerability_state: match inst.get("vulnerabilityState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(VulnerabilityStatePtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

