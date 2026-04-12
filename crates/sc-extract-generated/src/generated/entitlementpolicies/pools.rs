// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `entitlementpolicies` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitlementpoliciesPools {
    #[serde(default)]
    pub default_entitlement_record: Vec<Option<DefaultEntitlementRecord>>,
    #[serde(default)]
    pub entitlement_item_type: Vec<Option<EntitlementItemType>>,
    #[serde(default)]
    pub entitlement_account_item_global_params: Vec<Option<EntitlementAccountItemGlobalParams>>,
    #[serde(default)]
    pub entitlement_non_inventory_storable_item_global_params: Vec<Option<EntitlementNonInventoryStorableItemGlobalParams>>,
    #[serde(default)]
    pub corpse_interaction_params: Vec<Option<CorpseInteractionParams>>,
    #[serde(default)]
    pub item_recovery_configuration_params: Vec<Option<ItemRecoveryConfigurationParams>>,
    #[serde(default)]
    pub item_recovery_set_condition_def: Vec<Option<ItemRecoverySetConditionDef>>,
    #[serde(default)]
    pub item_recovery_notification_params: Vec<Option<ItemRecoveryNotificationParams>>,
    #[serde(default)]
    pub item_recovery_economy_params: Vec<Option<ItemRecoveryEconomyParams>>,
    #[serde(default)]
    pub debug_loadout_kit: Vec<Option<DebugLoadoutKit>>,
    #[serde(default)]
    pub web_customization_debug: Vec<Option<WebCustomizationDebug>>,
    #[serde(default)]
    pub web_customization_item_type_name: Vec<Option<WebCustomizationItemTypeName>>,
    #[serde(default)]
    pub web_customization_global_params: Vec<Option<WebCustomizationGlobalParams>>,
}
