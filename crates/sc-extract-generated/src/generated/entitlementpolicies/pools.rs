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

/// Pool storage for the `entitlementpolicies` feature.
#[derive(Default)]
pub struct EntitlementpoliciesPools {
    pub entitlement_item_type: Vec<Option<EntitlementItemType>>,
    pub entitlement_account_item_global_params: Vec<Option<EntitlementAccountItemGlobalParams>>,
    pub entitlement_non_inventory_storable_item_global_params:
        Vec<Option<EntitlementNonInventoryStorableItemGlobalParams>>,
    pub corpse_interaction_params: Vec<Option<CorpseInteractionParams>>,
    pub item_recovery_configuration_params: Vec<Option<ItemRecoveryConfigurationParams>>,
    pub item_recovery_notification_params: Vec<Option<ItemRecoveryNotificationParams>>,
    pub item_recovery_economy_params: Vec<Option<ItemRecoveryEconomyParams>>,
    pub item_recovery_override_group_def: Vec<Option<ItemRecoveryOverrideGroupDef>>,
    pub item_recovery_cooldown_params: Vec<Option<ItemRecoveryCooldownParams>>,
    pub item_recovery_delivery_time_params: Vec<Option<ItemRecoveryDeliveryTimeParams>>,
    pub item_recovery_repair_params: Vec<Option<ItemRecoveryRepairParams>>,
    pub item_recovery_escalating_claims_params: Vec<Option<ItemRecoveryEscalatingClaimsParams>>,
    pub item_recovery_condition_item_type: Vec<Option<ItemRecoveryCondition_ItemType>>,
    pub item_recovery_condition_entity_class: Vec<Option<ItemRecoveryCondition_EntityClass>>,
    pub debug_loadout_kit: Vec<Option<DebugLoadoutKit>>,
    pub web_customization_debug: Vec<Option<WebCustomizationDebug>>,
    pub web_customization_item_type_name: Vec<Option<WebCustomizationItemTypeName>>,
    pub web_customization_global_params: Vec<Option<WebCustomizationGlobalParams>>,
}
