// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-human`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `CommodityCrateComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct CommodityCrateComponentParams {}

impl Pooled for CommodityCrateComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.commodity_crate_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.commodity_crate_component_params
    }
}

impl<'a> Extract<'a> for CommodityCrateComponentParams {
    const TYPE_NAME: &'static str = "CommodityCrateComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `MiningShopProviderEntityComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct MiningShopProviderEntityComponentParams {}

impl Pooled for MiningShopProviderEntityComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_human
            .mining_shop_provider_entity_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_human
            .mining_shop_provider_entity_component_params
    }
}

impl<'a> Extract<'a> for MiningShopProviderEntityComponentParams {
    const TYPE_NAME: &'static str = "MiningShopProviderEntityComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `SDespawnRule_OnFallBelow`
/// Inherits from: `SDespawnRule`
pub struct SDespawnRule_OnFallBelow {
    /// `ruleDelaySeconds` (Single)
    pub rule_delay_seconds: f32,
    /// `distance` (Single)
    pub distance: f32,
}

impl Pooled for SDespawnRule_OnFallBelow {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.sdespawn_rule_on_fall_below
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.sdespawn_rule_on_fall_below
    }
}

impl<'a> Extract<'a> for SDespawnRule_OnFallBelow {
    const TYPE_NAME: &'static str = "SDespawnRule_OnFallBelow";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rule_delay_seconds: inst.get_f32("ruleDelaySeconds").unwrap_or_default(),
            distance: inst.get_f32("distance").unwrap_or_default(),
        }
    }
}

/// DCB type: `DeliveryLockerComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct DeliveryLockerComponentParams {
    /// `placeInteraction` (WeakPointer)
    pub place_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `pickupInteraction` (WeakPointer)
    pub pickup_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `dropOffInteraction` (WeakPointer)
    pub drop_off_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `openHatchInteraction` (WeakPointer)
    pub open_hatch_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `closeHatchInteraction` (WeakPointer)
    pub close_hatch_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `hatchOpenState` (WeakPointer)
    pub hatch_open_state: Option<Handle<SInteractionState>>,
    /// `hatchClosedState` (WeakPointer)
    pub hatch_closed_state: Option<Handle<SInteractionState>>,
    /// `homeState` (WeakPointer)
    pub home_state: Option<Handle<SInteractionState>>,
    /// `checkingState` (WeakPointer)
    pub checking_state: Option<Handle<SInteractionState>>,
    /// `collectPackageState` (WeakPointer)
    pub collect_package_state: Option<Handle<SInteractionState>>,
    /// `deliverPackageState` (WeakPointer)
    pub deliver_package_state: Option<Handle<SInteractionState>>,
    /// `completeState` (WeakPointer)
    pub complete_state: Option<Handle<SInteractionState>>,
    /// `timedOutState` (WeakPointer)
    pub timed_out_state: Option<Handle<SInteractionState>>,
    /// `wrongItemState` (WeakPointer)
    pub wrong_item_state: Option<Handle<SInteractionState>>,
    /// `failedRequestState` (WeakPointer)
    pub failed_request_state: Option<Handle<SInteractionState>>,
    /// `spawnTimeOutSeconds` (Single)
    pub spawn_time_out_seconds: f32,
    /// `requestProcessSeconds` (Single)
    pub request_process_seconds: f32,
    /// `waitForPickupSeconds` (Single)
    pub wait_for_pickup_seconds: f32,
    /// `finishedPickupSeconds` (Single)
    pub finished_pickup_seconds: f32,
    /// `despawnFailedPickupSeconds` (Single)
    pub despawn_failed_pickup_seconds: f32,
    /// `pickUpShutterDelaySeconds` (Single)
    pub pick_up_shutter_delay_seconds: f32,
    /// `waitForDropOffSeconds` (Single)
    pub wait_for_drop_off_seconds: f32,
    /// `dropOffShutterDelaySeconds` (Single)
    pub drop_off_shutter_delay_seconds: f32,
    /// `wrongItemPickUpSeconds` (Single)
    pub wrong_item_pick_up_seconds: f32,
}

impl Pooled for DeliveryLockerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.delivery_locker_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.delivery_locker_component_params
    }
}

impl<'a> Extract<'a> for DeliveryLockerComponentParams {
    const TYPE_NAME: &'static str = "DeliveryLockerComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            place_interaction: match inst.get("placeInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            pickup_interaction: match inst.get("pickupInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_off_interaction: match inst.get("dropOffInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            open_hatch_interaction: match inst.get("openHatchInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            close_hatch_interaction: match inst.get("closeHatchInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            hatch_open_state: match inst.get("hatchOpenState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            hatch_closed_state: match inst.get("hatchClosedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            home_state: match inst.get("homeState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            checking_state: match inst.get("checkingState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            collect_package_state: match inst.get("collectPackageState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            deliver_package_state: match inst.get("deliverPackageState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            complete_state: match inst.get("completeState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            timed_out_state: match inst.get("timedOutState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            wrong_item_state: match inst.get("wrongItemState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            failed_request_state: match inst.get("failedRequestState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SInteractionState>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            spawn_time_out_seconds: inst.get_f32("spawnTimeOutSeconds").unwrap_or_default(),
            request_process_seconds: inst.get_f32("requestProcessSeconds").unwrap_or_default(),
            wait_for_pickup_seconds: inst.get_f32("waitForPickupSeconds").unwrap_or_default(),
            finished_pickup_seconds: inst.get_f32("finishedPickupSeconds").unwrap_or_default(),
            despawn_failed_pickup_seconds: inst
                .get_f32("despawnFailedPickupSeconds")
                .unwrap_or_default(),
            pick_up_shutter_delay_seconds: inst
                .get_f32("pickUpShutterDelaySeconds")
                .unwrap_or_default(),
            wait_for_drop_off_seconds: inst.get_f32("waitForDropOffSeconds").unwrap_or_default(),
            drop_off_shutter_delay_seconds: inst
                .get_f32("dropOffShutterDelaySeconds")
                .unwrap_or_default(),
            wrong_item_pick_up_seconds: inst.get_f32("wrongItemPickUpSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `ClassEntityFilter`
/// Inherits from: `EntityFilter`
pub struct ClassEntityFilter {
    /// `entityClass` (Reference)
    pub entity_class: Option<CigGuid>,
}

impl Pooled for ClassEntityFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.class_entity_filter
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.class_entity_filter
    }
}

impl<'a> Extract<'a> for ClassEntityFilter {
    const TYPE_NAME: &'static str = "ClassEntityFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst
                .get("entityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `GameplayTriggerConditionNOT`
/// Inherits from: `GameplayTriggerCondition`
pub struct GameplayTriggerConditionNOT {
    /// `input` (StrongPointer)
    pub input: Option<GameplayTriggerConditionPtr>,
}

impl Pooled for GameplayTriggerConditionNOT {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.gameplay_trigger_condition_not
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.gameplay_trigger_condition_not
    }
}

impl<'a> Extract<'a> for GameplayTriggerConditionNOT {
    const TYPE_NAME: &'static str = "GameplayTriggerConditionNOT";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input: match inst.get("input") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(GameplayTriggerConditionPtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `UserVariableCheckIntLess`
/// Inherits from: `UserVariableCheck`
pub struct UserVariableCheckIntLess {
    /// `variableName` (String)
    pub variable_name: String,
    /// `valueToCheck` (Int32)
    pub value_to_check: i32,
}

impl Pooled for UserVariableCheckIntLess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.user_variable_check_int_less
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.user_variable_check_int_less
    }
}

impl<'a> Extract<'a> for UserVariableCheckIntLess {
    const TYPE_NAME: &'static str = "UserVariableCheckIntLess";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            value_to_check: inst.get_i32("valueToCheck").unwrap_or_default(),
        }
    }
}

/// DCB type: `UserVariableSubtractIntValue`
/// Inherits from: `UserVariableComputeValueBase`
pub struct UserVariableSubtractIntValue {
    /// `variableName` (String)
    pub variable_name: String,
    /// `valueToSubtract` (Int32)
    pub value_to_subtract: i32,
}

impl Pooled for UserVariableSubtractIntValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.entities_scitem_human.user_variable_subtract_int_value
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.entities_scitem_human.user_variable_subtract_int_value
    }
}

impl<'a> Extract<'a> for UserVariableSubtractIntValue {
    const TYPE_NAME: &'static str = "UserVariableSubtractIntValue";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            variable_name: inst
                .get_str("variableName")
                .map(String::from)
                .unwrap_or_default(),
            value_to_subtract: inst.get_i32("valueToSubtract").unwrap_or_default(),
        }
    }
}
