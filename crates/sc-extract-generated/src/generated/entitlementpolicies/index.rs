// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;
use crate::Handle;
use std::collections::HashMap;
use svarog_common::CigGuid;

/// Record index for the `entitlementpolicies` feature.
#[derive(Default)]
pub struct EntitlementpoliciesIndex {
    pub entitlement_account_item_global_params:
        HashMap<CigGuid, Handle<EntitlementAccountItemGlobalParams>>,
    pub entitlement_non_inventory_storable_item_global_params:
        HashMap<CigGuid, Handle<EntitlementNonInventoryStorableItemGlobalParams>>,
    pub corpse_interaction_params: HashMap<CigGuid, Handle<CorpseInteractionParams>>,
    pub item_recovery_configuration_params:
        HashMap<CigGuid, Handle<ItemRecoveryConfigurationParams>>,
    pub web_customization_debug: HashMap<CigGuid, Handle<WebCustomizationDebug>>,
    pub web_customization_global_params: HashMap<CigGuid, Handle<WebCustomizationGlobalParams>>,
}

impl EntitlementpoliciesIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.entitlement_account_item_global_params.len();
        total += self
            .entitlement_non_inventory_storable_item_global_params
            .len();
        total += self.corpse_interaction_params.len();
        total += self.item_recovery_configuration_params.len();
        total += self.web_customization_debug.len();
        total += self.web_customization_global_params.len();
        total
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
