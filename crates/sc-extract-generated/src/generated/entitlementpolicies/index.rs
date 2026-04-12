// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `entitlementpolicies` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitlementpoliciesIndex {
    #[serde(default)]
    pub default_entitlement_record: HashMap<CigGuid, Handle<DefaultEntitlementRecord>>,
    #[serde(default)]
    pub entitlement_account_item_global_params: HashMap<CigGuid, Handle<EntitlementAccountItemGlobalParams>>,
    #[serde(default)]
    pub entitlement_non_inventory_storable_item_global_params: HashMap<CigGuid, Handle<EntitlementNonInventoryStorableItemGlobalParams>>,
    #[serde(default)]
    pub corpse_interaction_params: HashMap<CigGuid, Handle<CorpseInteractionParams>>,
    #[serde(default)]
    pub item_recovery_configuration_params: HashMap<CigGuid, Handle<ItemRecoveryConfigurationParams>>,
    #[serde(default)]
    pub web_customization_debug: HashMap<CigGuid, Handle<WebCustomizationDebug>>,
    #[serde(default)]
    pub web_customization_global_params: HashMap<CigGuid, Handle<WebCustomizationGlobalParams>>,
}

impl EntitlementpoliciesIndex {
    pub fn len(&self) -> usize {
        self.default_entitlement_record.len()
            + self.entitlement_account_item_global_params.len()
            + self.entitlement_non_inventory_storable_item_global_params.len()
            + self.corpse_interaction_params.len()
            + self.item_recovery_configuration_params.len()
            + self.web_customization_debug.len()
            + self.web_customization_global_params.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
