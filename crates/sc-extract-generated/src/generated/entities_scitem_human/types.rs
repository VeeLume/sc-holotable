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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `CommodityCrateComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityCrateComponentParams {
}

impl Pooled for CommodityCrateComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_human.commodity_crate_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_human.commodity_crate_component_params }
}

impl<'a> Extract<'a> for CommodityCrateComponentParams {
    const TYPE_NAME: &'static str = "CommodityCrateComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MiningShopProviderEntityComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningShopProviderEntityComponentParams {
}

impl Pooled for MiningShopProviderEntityComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_human.mining_shop_provider_entity_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_human.mining_shop_provider_entity_component_params }
}

impl<'a> Extract<'a> for MiningShopProviderEntityComponentParams {
    const TYPE_NAME: &'static str = "MiningShopProviderEntityComponentParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SDespawnRule_OnFallBelow`
/// Inherits from: `SDespawnRule`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDespawnRule_OnFallBelow {
    /// `ruleDelaySeconds` (Single)
    #[serde(default)]
    pub rule_delay_seconds: f32,
    /// `distance` (Single)
    #[serde(default)]
    pub distance: f32,
}

impl Pooled for SDespawnRule_OnFallBelow {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_human.sdespawn_rule_on_fall_below }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_human.sdespawn_rule_on_fall_below }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLockerComponentParams {
    /// `placeInteraction` (WeakPointer)
    #[serde(default)]
    pub place_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `pickupInteraction` (WeakPointer)
    #[serde(default)]
    pub pickup_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `dropOffInteraction` (WeakPointer)
    #[serde(default)]
    pub drop_off_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `openHatchInteraction` (WeakPointer)
    #[serde(default)]
    pub open_hatch_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `closeHatchInteraction` (WeakPointer)
    #[serde(default)]
    pub close_hatch_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `hatchOpenState` (WeakPointer)
    #[serde(default)]
    pub hatch_open_state: Option<Handle<SInteractionState>>,
    /// `hatchClosedState` (WeakPointer)
    #[serde(default)]
    pub hatch_closed_state: Option<Handle<SInteractionState>>,
    /// `homeState` (WeakPointer)
    #[serde(default)]
    pub home_state: Option<Handle<SInteractionState>>,
    /// `checkingState` (WeakPointer)
    #[serde(default)]
    pub checking_state: Option<Handle<SInteractionState>>,
    /// `collectPackageState` (WeakPointer)
    #[serde(default)]
    pub collect_package_state: Option<Handle<SInteractionState>>,
    /// `deliverPackageState` (WeakPointer)
    #[serde(default)]
    pub deliver_package_state: Option<Handle<SInteractionState>>,
    /// `completeState` (WeakPointer)
    #[serde(default)]
    pub complete_state: Option<Handle<SInteractionState>>,
    /// `timedOutState` (WeakPointer)
    #[serde(default)]
    pub timed_out_state: Option<Handle<SInteractionState>>,
    /// `wrongItemState` (WeakPointer)
    #[serde(default)]
    pub wrong_item_state: Option<Handle<SInteractionState>>,
    /// `failedRequestState` (WeakPointer)
    #[serde(default)]
    pub failed_request_state: Option<Handle<SInteractionState>>,
    /// `spawnTimeOutSeconds` (Single)
    #[serde(default)]
    pub spawn_time_out_seconds: f32,
    /// `requestProcessSeconds` (Single)
    #[serde(default)]
    pub request_process_seconds: f32,
    /// `waitForPickupSeconds` (Single)
    #[serde(default)]
    pub wait_for_pickup_seconds: f32,
    /// `finishedPickupSeconds` (Single)
    #[serde(default)]
    pub finished_pickup_seconds: f32,
    /// `despawnFailedPickupSeconds` (Single)
    #[serde(default)]
    pub despawn_failed_pickup_seconds: f32,
    /// `pickUpShutterDelaySeconds` (Single)
    #[serde(default)]
    pub pick_up_shutter_delay_seconds: f32,
    /// `waitForDropOffSeconds` (Single)
    #[serde(default)]
    pub wait_for_drop_off_seconds: f32,
    /// `dropOffShutterDelaySeconds` (Single)
    #[serde(default)]
    pub drop_off_shutter_delay_seconds: f32,
    /// `wrongItemPickUpSeconds` (Single)
    #[serde(default)]
    pub wrong_item_pick_up_seconds: f32,
}

impl Pooled for DeliveryLockerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_human.delivery_locker_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_human.delivery_locker_component_params }
}

impl<'a> Extract<'a> for DeliveryLockerComponentParams {
    const TYPE_NAME: &'static str = "DeliveryLockerComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            place_interaction: match inst.get("placeInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pickup_interaction: match inst.get("pickupInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            drop_off_interaction: match inst.get("dropOffInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            open_hatch_interaction: match inst.get("openHatchInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            close_hatch_interaction: match inst.get("closeHatchInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hatch_open_state: match inst.get("hatchOpenState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hatch_closed_state: match inst.get("hatchClosedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            home_state: match inst.get("homeState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            checking_state: match inst.get("checkingState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            collect_package_state: match inst.get("collectPackageState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            deliver_package_state: match inst.get("deliverPackageState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            complete_state: match inst.get("completeState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            timed_out_state: match inst.get("timedOutState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            wrong_item_state: match inst.get("wrongItemState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            failed_request_state: match inst.get("failedRequestState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spawn_time_out_seconds: inst.get_f32("spawnTimeOutSeconds").unwrap_or_default(),
            request_process_seconds: inst.get_f32("requestProcessSeconds").unwrap_or_default(),
            wait_for_pickup_seconds: inst.get_f32("waitForPickupSeconds").unwrap_or_default(),
            finished_pickup_seconds: inst.get_f32("finishedPickupSeconds").unwrap_or_default(),
            despawn_failed_pickup_seconds: inst.get_f32("despawnFailedPickupSeconds").unwrap_or_default(),
            pick_up_shutter_delay_seconds: inst.get_f32("pickUpShutterDelaySeconds").unwrap_or_default(),
            wait_for_drop_off_seconds: inst.get_f32("waitForDropOffSeconds").unwrap_or_default(),
            drop_off_shutter_delay_seconds: inst.get_f32("dropOffShutterDelaySeconds").unwrap_or_default(),
            wrong_item_pick_up_seconds: inst.get_f32("wrongItemPickUpSeconds").unwrap_or_default(),
        }
    }
}

/// DCB type: `ClassEntityFilter`
/// Inherits from: `EntityFilter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassEntityFilter {
    /// `entityClass` (Reference)
    #[serde(default)]
    pub entity_class: Option<CigGuid>,
}

impl Pooled for ClassEntityFilter {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_human.class_entity_filter }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_human.class_entity_filter }
}

impl<'a> Extract<'a> for ClassEntityFilter {
    const TYPE_NAME: &'static str = "ClassEntityFilter";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst.get("entityClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `GameplayTriggerConditionNOT`
/// Inherits from: `GameplayTriggerCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayTriggerConditionNOT {
    /// `input` (StrongPointer)
    #[serde(default)]
    pub input: Option<GameplayTriggerConditionPtr>,
}

impl Pooled for GameplayTriggerConditionNOT {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_human.gameplay_trigger_condition_not }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_human.gameplay_trigger_condition_not }
}

impl<'a> Extract<'a> for GameplayTriggerConditionNOT {
    const TYPE_NAME: &'static str = "GameplayTriggerConditionNOT";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            input: match inst.get("input") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(GameplayTriggerConditionPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `UserVariableCheckIntLess`
/// Inherits from: `UserVariableCheck`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVariableCheckIntLess {
    /// `variableName` (String)
    #[serde(default)]
    pub variable_name: String,
    /// `valueToCheck` (Int32)
    #[serde(default)]
    pub value_to_check: i32,
}

impl Pooled for UserVariableCheckIntLess {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_human.user_variable_check_int_less }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_human.user_variable_check_int_less }
}

impl<'a> Extract<'a> for UserVariableCheckIntLess {
    const TYPE_NAME: &'static str = "UserVariableCheckIntLess";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            variable_name: inst.get_str("variableName").map(String::from).unwrap_or_default(),
            value_to_check: inst.get_i32("valueToCheck").unwrap_or_default(),
        }
    }
}

/// DCB type: `UserVariableSubtractIntValue`
/// Inherits from: `UserVariableComputeValueBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVariableSubtractIntValue {
    /// `variableName` (String)
    #[serde(default)]
    pub variable_name: String,
    /// `valueToSubtract` (Int32)
    #[serde(default)]
    pub value_to_subtract: i32,
}

impl Pooled for UserVariableSubtractIntValue {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_human.user_variable_subtract_int_value }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_human.user_variable_subtract_int_value }
}

impl<'a> Extract<'a> for UserVariableSubtractIntValue {
    const TYPE_NAME: &'static str = "UserVariableSubtractIntValue";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            variable_name: inst.get_str("variableName").map(String::from).unwrap_or_default(),
            value_to_subtract: inst.get_i32("valueToSubtract").unwrap_or_default(),
        }
    }
}

