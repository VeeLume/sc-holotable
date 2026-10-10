// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-gameplayinteractables`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `SpawnerPrerequisite_OR`
/// Inherits from: `BaseSpawnerPrerequisite`
pub struct SpawnerPrerequisite_OR {
    /// `prerequisites` (StrongPointer (array))
    pub prerequisites: Vec<BaseSpawnerPrerequisitePtr>,
}

impl Pooled for SpawnerPrerequisite_OR {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_gameplayinteractables
            .spawner_prerequisite_or
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_gameplayinteractables
            .spawner_prerequisite_or
    }
}

impl<'a> Extract<'a> for SpawnerPrerequisite_OR {
    const TYPE_NAME: &'static str = "SpawnerPrerequisite_OR";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            prerequisites: inst
                .get_array("prerequisites")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(BaseSpawnerPrerequisitePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
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
        &pools
            .entities_scitem_gameplayinteractables
            .delivery_locker_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_gameplayinteractables
            .delivery_locker_component_params
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

/// DCB type: `SWeightedRewardEntry`
pub struct SWeightedRewardEntry {
    /// `rewardEntityRecord` (Reference)
    pub reward_entity_record: Option<CigGuid>,
    /// `weight` (Single)
    pub weight: f32,
}

impl Pooled for SWeightedRewardEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_gameplayinteractables
            .sweighted_reward_entry
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_gameplayinteractables
            .sweighted_reward_entry
    }
}

impl<'a> Extract<'a> for SWeightedRewardEntry {
    const TYPE_NAME: &'static str = "SWeightedRewardEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reward_entity_record: inst
                .get("rewardEntityRecord")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

/// DCB type: `SRewardGeneratorComponentParams`
/// Inherits from: `DataForgeComponentParams`
pub struct SRewardGeneratorComponentParams {
    /// `selectRandomRewardInteraction` (WeakPointer)
    pub select_random_reward_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `claimInteraction` (WeakPointer)
    pub claim_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `retrieveInteraction` (WeakPointer)
    pub retrieve_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `cleanupInteraction` (WeakPointer)
    pub cleanup_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `allowCleanupInSameRevolution` (Boolean)
    pub allow_cleanup_in_same_revolution: bool,
    /// `missionScenario` (Reference)
    pub mission_scenario: Option<CigGuid>,
    /// `rewardPool` (Class (array))
    pub reward_pool: Vec<Handle<SWeightedRewardEntry>>,
}

impl Pooled for SRewardGeneratorComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_gameplayinteractables
            .sreward_generator_component_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_gameplayinteractables
            .sreward_generator_component_params
    }
}

impl<'a> Extract<'a> for SRewardGeneratorComponentParams {
    const TYPE_NAME: &'static str = "SRewardGeneratorComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            select_random_reward_interaction: match inst.get("selectRandomRewardInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            claim_interaction: match inst.get("claimInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            retrieve_interaction: match inst.get("retrieveInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            cleanup_interaction: match inst.get("cleanupInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            allow_cleanup_in_same_revolution: inst
                .get_bool("allowCleanupInSameRevolution")
                .unwrap_or_default(),
            mission_scenario: inst
                .get("missionScenario")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            reward_pool: inst
                .get_array("rewardPool")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SWeightedRewardEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SWeightedRewardEntry>(
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

/// DCB type: `SSpawnerAnalyticsEventGameplayTrigger`
/// Inherits from: `SBaseAnalyticsEventGameplayTrigger`
pub struct SSpawnerAnalyticsEventGameplayTrigger {
    /// `analyticsEvent` (Reference)
    pub analytics_event: Option<CigGuid>,
    /// `spawnedObjectFieldName` (String)
    pub spawned_object_field_name: String,
}

impl Pooled for SSpawnerAnalyticsEventGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_gameplayinteractables
            .sspawner_analytics_event_gameplay_trigger
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_gameplayinteractables
            .sspawner_analytics_event_gameplay_trigger
    }
}

impl<'a> Extract<'a> for SSpawnerAnalyticsEventGameplayTrigger {
    const TYPE_NAME: &'static str = "SSpawnerAnalyticsEventGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            analytics_event: inst
                .get("analyticsEvent")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            spawned_object_field_name: inst
                .get_str("spawnedObjectFieldName")
                .map(String::from)
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SelfInteractionTrigger`
/// Inherits from: `SelfCommunicationMessage`
pub struct SelfInteractionTrigger {
    /// `targetSelfInteraction` (WeakPointer)
    pub target_self_interaction: Option<Handle<SSharedInteractionParams>>,
}

impl Pooled for SelfInteractionTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_gameplayinteractables
            .self_interaction_trigger
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_gameplayinteractables
            .self_interaction_trigger
    }
}

impl<'a> Extract<'a> for SelfInteractionTrigger {
    const TYPE_NAME: &'static str = "SelfInteractionTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            target_self_interaction: match inst.get("targetSelfInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<SSharedInteractionParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `GameplayTrigger_Physics_SetParameter_ProxyState`
/// Inherits from: `GameplayTrigger_Physics_SetParameter_Base`
pub struct GameplayTrigger_Physics_SetParameter_ProxyState {
    /// `proxyState` (EnumChoice)
    pub proxy_state: GameplayTrigger_Toggle,
}

impl Pooled for GameplayTrigger_Physics_SetParameter_ProxyState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_gameplayinteractables
            .gameplay_trigger_physics_set_parameter_proxy_state
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_gameplayinteractables
            .gameplay_trigger_physics_set_parameter_proxy_state
    }
}

impl<'a> Extract<'a> for GameplayTrigger_Physics_SetParameter_ProxyState {
    const TYPE_NAME: &'static str = "GameplayTrigger_Physics_SetParameter_ProxyState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            proxy_state: GameplayTrigger_Toggle::from_dcb_str(
                inst.get_str("proxyState").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `PhysicsSetParameterGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
pub struct PhysicsSetParameterGameplayTrigger {
    /// `parametersToChange` (StrongPointer (array))
    pub parameters_to_change: Vec<GameplayTrigger_Physics_SetParameter_BasePtr>,
}

impl Pooled for PhysicsSetParameterGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .entities_scitem_gameplayinteractables
            .physics_set_parameter_gameplay_trigger
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .entities_scitem_gameplayinteractables
            .physics_set_parameter_gameplay_trigger
    }
}

impl<'a> Extract<'a> for PhysicsSetParameterGameplayTrigger {
    const TYPE_NAME: &'static str = "PhysicsSetParameterGameplayTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            parameters_to_change: inst
                .get_array("parametersToChange")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(GameplayTrigger_Physics_SetParameter_BasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
