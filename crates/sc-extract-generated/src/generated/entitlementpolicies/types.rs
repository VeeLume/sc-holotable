// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entitlementpolicies`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `EntitlementItemType`
pub struct EntitlementItemType {
    /// `Type` (EnumChoice)
    pub r#type: EItemType,
}

impl Pooled for EntitlementItemType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.entitlement_item_type
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.entitlement_item_type
    }
}

impl<'a> Extract<'a> for EntitlementItemType {
    const TYPE_NAME: &'static str = "EntitlementItemType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: EItemType::from_dcb_str(inst.get_str("Type").unwrap_or("")),
        }
    }
}

/// DCB type: `EntitlementAccountItemGlobalParams`
pub struct EntitlementAccountItemGlobalParams {
    /// `accountItemTypes` (Class (array))
    pub account_item_types: Vec<Handle<EntitlementItemType>>,
}

impl Pooled for EntitlementAccountItemGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entitlementpolicies
            .entitlement_account_item_global_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entitlementpolicies
            .entitlement_account_item_global_params
    }
}

impl<'a> Extract<'a> for EntitlementAccountItemGlobalParams {
    const TYPE_NAME: &'static str = "EntitlementAccountItemGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            account_item_types: inst
                .get_array("accountItemTypes")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<EntitlementItemType>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<EntitlementItemType>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntitlementNonInventoryStorableItemGlobalParams`
pub struct EntitlementNonInventoryStorableItemGlobalParams {
    /// `itemTypes` (Class (array))
    pub item_types: Vec<Handle<EntitlementItemType>>,
}

impl Pooled for EntitlementNonInventoryStorableItemGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entitlementpolicies
            .entitlement_non_inventory_storable_item_global_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entitlementpolicies
            .entitlement_non_inventory_storable_item_global_params
    }
}

impl<'a> Extract<'a> for EntitlementNonInventoryStorableItemGlobalParams {
    const TYPE_NAME: &'static str = "EntitlementNonInventoryStorableItemGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item_types: inst
                .get_array("itemTypes")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<EntitlementItemType>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<EntitlementItemType>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CorpseInteractionParams`
pub struct CorpseInteractionParams {
    /// `itemRecoveryBlacklist` (Reference (array))
    pub item_recovery_blacklist: Vec<CigGuid>,
    /// `corpseClasses` (Reference (array))
    pub corpse_classes: Vec<CigGuid>,
    /// `allowedTypes` (EnumChoice (array))
    pub allowed_types: Vec<EItemType>,
    /// `allowedSubTypes` (EnumChoice (array))
    pub allowed_sub_types: Vec<EItemSubType>,
}

impl Pooled for CorpseInteractionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.corpse_interaction_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.corpse_interaction_params
    }
}

impl<'a> Extract<'a> for CorpseInteractionParams {
    const TYPE_NAME: &'static str = "CorpseInteractionParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_recovery_blacklist: inst
                .get_array("itemRecoveryBlacklist")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            corpse_classes: inst
                .get_array("corpseClasses")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            allowed_types: inst
                .get_array("allowedTypes")
                .map(|arr| {
                    arr.filter_map(|v| v.as_str().map(EItemType::from_dcb_str))
                        .collect()
                })
                .unwrap_or_default(),
            allowed_sub_types: inst
                .get_array("allowedSubTypes")
                .map(|arr| {
                    arr.filter_map(|v| v.as_str().map(EItemSubType::from_dcb_str))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemRecoveryConfigurationParams`
pub struct ItemRecoveryConfigurationParams {
    /// `nonEligibleItems` (StrongPointer (array))
    pub non_eligible_items: Vec<ItemRecoverySetConditionDefPtr>,
    /// `consumableItems` (StrongPointer (array))
    pub consumable_items: Vec<ItemRecoverySetConditionDefPtr>,
    /// `dontEquipBrickedItems` (StrongPointer (array))
    pub dont_equip_bricked_items: Vec<ItemRecoverySetConditionDefPtr>,
    /// `economyParams` (Class)
    pub economy_params: Option<Handle<ItemRecoveryEconomyParams>>,
    /// `notificationParams` (Class)
    pub notification_params: Option<Handle<ItemRecoveryNotificationParams>>,
}

impl Pooled for ItemRecoveryConfigurationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.item_recovery_configuration_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.item_recovery_configuration_params
    }
}

impl<'a> Extract<'a> for ItemRecoveryConfigurationParams {
    const TYPE_NAME: &'static str = "ItemRecoveryConfigurationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            non_eligible_items: inst
                .get_array("nonEligibleItems")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(ItemRecoverySetConditionDefPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            consumable_items: inst
                .get_array("consumableItems")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(ItemRecoverySetConditionDefPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            dont_equip_bricked_items: inst
                .get_array("dontEquipBrickedItems")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(ItemRecoverySetConditionDefPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            economy_params: match inst.get("economyParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ItemRecoveryEconomyParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            notification_params: match inst.get("notificationParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ItemRecoveryNotificationParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `ItemRecoveryNotificationParams`
pub struct ItemRecoveryNotificationParams {
    /// `itemBricked_Title` (Locale)
    pub item_bricked_title: LocaleKey,
    /// `itemBricked_Body` (Locale)
    pub item_bricked_body: LocaleKey,
    /// `itemBricking_Title` (Locale)
    pub item_bricking_title: LocaleKey,
    /// `itemBricking_Body` (Locale)
    pub item_bricking_body: LocaleKey,
    /// `onBrickedItemUseNotificationBuffer` (Single)
    pub on_bricked_item_use_notification_buffer: f32,
    /// `onBrickedItemUsed_Title` (Locale)
    pub on_bricked_item_used_title: LocaleKey,
    /// `onBrickedItemUsed_CannotEquip` (Locale)
    pub on_bricked_item_used_cannot_equip: LocaleKey,
    /// `onBrickedItemUsed_CannotFire` (Locale)
    pub on_bricked_item_used_cannot_fire: LocaleKey,
    /// `onBrickedItemUsed_CannotUse` (Locale)
    pub on_bricked_item_used_cannot_use: LocaleKey,
}

impl Pooled for ItemRecoveryNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.item_recovery_notification_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.item_recovery_notification_params
    }
}

impl<'a> Extract<'a> for ItemRecoveryNotificationParams {
    const TYPE_NAME: &'static str = "ItemRecoveryNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            item_bricked_title: inst
                .get_str("itemBricked_Title")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            item_bricked_body: inst
                .get_str("itemBricked_Body")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            item_bricking_title: inst
                .get_str("itemBricking_Title")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            item_bricking_body: inst
                .get_str("itemBricking_Body")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            on_bricked_item_use_notification_buffer: inst
                .get_f32("onBrickedItemUseNotificationBuffer")
                .unwrap_or_default(),
            on_bricked_item_used_title: inst
                .get_str("onBrickedItemUsed_Title")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            on_bricked_item_used_cannot_equip: inst
                .get_str("onBrickedItemUsed_CannotEquip")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            on_bricked_item_used_cannot_fire: inst
                .get_str("onBrickedItemUsed_CannotFire")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            on_bricked_item_used_cannot_use: inst
                .get_str("onBrickedItemUsed_CannotUse")
                .map(LocaleKey::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemRecoveryEconomyParams`
pub struct ItemRecoveryEconomyParams {
    /// `globalBrickTimer` (Single)
    pub global_brick_timer: f32,
    /// `baseReclaimTime` (Single)
    pub base_reclaim_time: f32,
    /// `scalingPriceFloor` (Single)
    pub scaling_price_floor: f32,
    /// `aUECperSecond` (Single)
    pub a_uecper_second: f32,
    /// `claimCostMultiplier` (Single)
    pub claim_cost_multiplier: f32,
}

impl Pooled for ItemRecoveryEconomyParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.item_recovery_economy_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.item_recovery_economy_params
    }
}

impl<'a> Extract<'a> for ItemRecoveryEconomyParams {
    const TYPE_NAME: &'static str = "ItemRecoveryEconomyParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            global_brick_timer: inst.get_f32("globalBrickTimer").unwrap_or_default(),
            base_reclaim_time: inst.get_f32("baseReclaimTime").unwrap_or_default(),
            scaling_price_floor: inst.get_f32("scalingPriceFloor").unwrap_or_default(),
            a_uecper_second: inst.get_f32("aUECperSecond").unwrap_or_default(),
            claim_cost_multiplier: inst.get_f32("claimCostMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemRecoveryCondition_ItemType`
/// Inherits from: `ItemRecoverySetConditionDef`
pub struct ItemRecoveryCondition_ItemType {
    /// `type` (EnumChoice)
    pub r#type: EItemType,
    /// `subType` (EnumChoice)
    pub sub_type: EItemSubType,
}

impl Pooled for ItemRecoveryCondition_ItemType {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.item_recovery_condition_item_type
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.item_recovery_condition_item_type
    }
}

impl<'a> Extract<'a> for ItemRecoveryCondition_ItemType {
    const TYPE_NAME: &'static str = "ItemRecoveryCondition_ItemType";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r#type: EItemType::from_dcb_str(inst.get_str("type").unwrap_or("")),
            sub_type: EItemSubType::from_dcb_str(inst.get_str("subType").unwrap_or("")),
        }
    }
}

/// DCB type: `ItemRecoveryCondition_EntityClass`
/// Inherits from: `ItemRecoverySetConditionDef`
pub struct ItemRecoveryCondition_EntityClass {
    /// `classDef` (Reference)
    pub class_def: Option<CigGuid>,
}

impl Pooled for ItemRecoveryCondition_EntityClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entitlementpolicies
            .item_recovery_condition_entity_class
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entitlementpolicies
            .item_recovery_condition_entity_class
    }
}

impl<'a> Extract<'a> for ItemRecoveryCondition_EntityClass {
    const TYPE_NAME: &'static str = "ItemRecoveryCondition_EntityClass";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            class_def: inst
                .get("classDef")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `DebugLoadoutKit`
pub struct DebugLoadoutKit {
    /// `entityClass` (Reference)
    pub entity_class: Option<CigGuid>,
    /// `loadoutKit` (Reference)
    pub loadout_kit: Option<CigGuid>,
}

impl Pooled for DebugLoadoutKit {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.debug_loadout_kit
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.debug_loadout_kit
    }
}

impl<'a> Extract<'a> for DebugLoadoutKit {
    const TYPE_NAME: &'static str = "DebugLoadoutKit";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst
                .get("entityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            loadout_kit: inst
                .get("loadoutKit")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `WebCustomizationDebug`
pub struct WebCustomizationDebug {
    /// `debugLoadoutKits` (Class (array))
    pub debug_loadout_kits: Vec<Handle<DebugLoadoutKit>>,
}

impl Pooled for WebCustomizationDebug {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.web_customization_debug
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.web_customization_debug
    }
}

impl<'a> Extract<'a> for WebCustomizationDebug {
    const TYPE_NAME: &'static str = "WebCustomizationDebug";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_loadout_kits: inst
                .get_array("debugLoadoutKits")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<DebugLoadoutKit>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<DebugLoadoutKit>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `WebCustomizationItemTypeName`
pub struct WebCustomizationItemTypeName {
    /// `name` (Locale)
    pub name: LocaleKey,
    /// `itemTypes` (EnumChoice (array))
    pub item_types: Vec<EItemType>,
}

impl Pooled for WebCustomizationItemTypeName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.web_customization_item_type_name
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.web_customization_item_type_name
    }
}

impl<'a> Extract<'a> for WebCustomizationItemTypeName {
    const TYPE_NAME: &'static str = "WebCustomizationItemTypeName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst
                .get_str("name")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            item_types: inst
                .get_array("itemTypes")
                .map(|arr| {
                    arr.filter_map(|v| v.as_str().map(EItemType::from_dcb_str))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `WebCustomizationGlobalParams`
pub struct WebCustomizationGlobalParams {
    /// `defaultLoadoutKitName` (Locale)
    pub default_loadout_kit_name: LocaleKey,
    /// `typeNames` (Class (array))
    pub type_names: Vec<Handle<WebCustomizationItemTypeName>>,
}

impl Pooled for WebCustomizationGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entitlementpolicies.web_customization_global_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entitlementpolicies.web_customization_global_params
    }
}

impl<'a> Extract<'a> for WebCustomizationGlobalParams {
    const TYPE_NAME: &'static str = "WebCustomizationGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_loadout_kit_name: inst
                .get_str("defaultLoadoutKitName")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            type_names: inst
                .get_array("typeNames")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<WebCustomizationItemTypeName>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<WebCustomizationItemTypeName>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
