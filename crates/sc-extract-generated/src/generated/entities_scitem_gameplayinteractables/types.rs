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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SpawnerPrerequisite_OR`
/// Inherits from: `BaseSpawnerPrerequisite`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnerPrerequisite_OR {
    /// `prerequisites` (StrongPointer (array))
    #[serde(default)]
    pub prerequisites: Vec<BaseSpawnerPrerequisitePtr>,
}

impl Pooled for SpawnerPrerequisite_OR {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_gameplayinteractables.spawner_prerequisite_or }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_gameplayinteractables.spawner_prerequisite_or }
}

impl<'a> Extract<'a> for SpawnerPrerequisite_OR {
    const TYPE_NAME: &'static str = "SpawnerPrerequisite_OR";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            prerequisites: inst.get_array("prerequisites")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(BaseSpawnerPrerequisitePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SWeightedRewardEntry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeightedRewardEntry {
    /// `rewardEntityRecord` (Reference)
    #[serde(default)]
    pub reward_entity_record: Option<CigGuid>,
    /// `weight` (Single)
    #[serde(default)]
    pub weight: f32,
}

impl Pooled for SWeightedRewardEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_gameplayinteractables.sweighted_reward_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_gameplayinteractables.sweighted_reward_entry }
}

impl<'a> Extract<'a> for SWeightedRewardEntry {
    const TYPE_NAME: &'static str = "SWeightedRewardEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            reward_entity_record: inst.get("rewardEntityRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            weight: inst.get_f32("weight").unwrap_or_default(),
        }
    }
}

/// DCB type: `SRewardGeneratorComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRewardGeneratorComponentParams {
    /// `selectRandomRewardInteraction` (WeakPointer)
    #[serde(default)]
    pub select_random_reward_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `claimInteraction` (WeakPointer)
    #[serde(default)]
    pub claim_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `retrieveInteraction` (WeakPointer)
    #[serde(default)]
    pub retrieve_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `cleanupInteraction` (WeakPointer)
    #[serde(default)]
    pub cleanup_interaction: Option<Handle<SSharedInteractionParams>>,
    /// `allowCleanupInSameRevolution` (Boolean)
    #[serde(default)]
    pub allow_cleanup_in_same_revolution: bool,
    /// `missionScenario` (Reference)
    #[serde(default)]
    pub mission_scenario: Option<CigGuid>,
    /// `rewardPool` (Class (array))
    #[serde(default)]
    pub reward_pool: Vec<Handle<SWeightedRewardEntry>>,
}

impl Pooled for SRewardGeneratorComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_gameplayinteractables.sreward_generator_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_gameplayinteractables.sreward_generator_component_params }
}

impl<'a> Extract<'a> for SRewardGeneratorComponentParams {
    const TYPE_NAME: &'static str = "SRewardGeneratorComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            select_random_reward_interaction: match inst.get("selectRandomRewardInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            claim_interaction: match inst.get("claimInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            retrieve_interaction: match inst.get("retrieveInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            cleanup_interaction: match inst.get("cleanupInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            allow_cleanup_in_same_revolution: inst.get_bool("allowCleanupInSameRevolution").unwrap_or_default(),
            mission_scenario: inst.get("missionScenario").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            reward_pool: inst.get_array("rewardPool")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SWeightedRewardEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SWeightedRewardEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SSpawnerAnalyticsEventGameplayTrigger`
/// Inherits from: `SBaseAnalyticsEventGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSpawnerAnalyticsEventGameplayTrigger {
    /// `analyticsEvent` (Reference)
    #[serde(default)]
    pub analytics_event: Option<CigGuid>,
    /// `spawnedObjectFieldName` (String)
    #[serde(default)]
    pub spawned_object_field_name: String,
}

impl Pooled for SSpawnerAnalyticsEventGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_gameplayinteractables.sspawner_analytics_event_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_gameplayinteractables.sspawner_analytics_event_gameplay_trigger }
}

impl<'a> Extract<'a> for SSpawnerAnalyticsEventGameplayTrigger {
    const TYPE_NAME: &'static str = "SSpawnerAnalyticsEventGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            analytics_event: inst.get("analyticsEvent").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            spawned_object_field_name: inst.get_str("spawnedObjectFieldName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SDissolveSelfGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDissolveSelfGameplayTrigger {
    /// `enableDissolve` (Boolean)
    #[serde(default)]
    pub enable_dissolve: bool,
}

impl Pooled for SDissolveSelfGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_gameplayinteractables.sdissolve_self_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_gameplayinteractables.sdissolve_self_gameplay_trigger }
}

impl<'a> Extract<'a> for SDissolveSelfGameplayTrigger {
    const TYPE_NAME: &'static str = "SDissolveSelfGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            enable_dissolve: inst.get_bool("enableDissolve").unwrap_or_default(),
        }
    }
}

/// DCB type: `SelfInteractionTrigger`
/// Inherits from: `SelfCommunicationMessage`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfInteractionTrigger {
    /// `targetSelfInteraction` (WeakPointer)
    #[serde(default)]
    pub target_self_interaction: Option<Handle<SSharedInteractionParams>>,
}

impl Pooled for SelfInteractionTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_gameplayinteractables.self_interaction_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_gameplayinteractables.self_interaction_trigger }
}

impl<'a> Extract<'a> for SelfInteractionTrigger {
    const TYPE_NAME: &'static str = "SelfInteractionTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            target_self_interaction: match inst.get("targetSelfInteraction") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GameplayTrigger_Physics_SetParameter_ProxyState`
/// Inherits from: `GameplayTrigger_Physics_SetParameter_Base`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameplayTrigger_Physics_SetParameter_ProxyState {
    /// `proxyState` (EnumChoice)
    #[serde(default)]
    pub proxy_state: GameplayTrigger_Toggle,
}

impl Pooled for GameplayTrigger_Physics_SetParameter_ProxyState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_gameplayinteractables.gameplay_trigger_physics_set_parameter_proxy_state }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_gameplayinteractables.gameplay_trigger_physics_set_parameter_proxy_state }
}

impl<'a> Extract<'a> for GameplayTrigger_Physics_SetParameter_ProxyState {
    const TYPE_NAME: &'static str = "GameplayTrigger_Physics_SetParameter_ProxyState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            proxy_state: GameplayTrigger_Toggle::from_dcb_str(inst.get_str("proxyState").unwrap_or("")),
        }
    }
}

/// DCB type: `PhysicsSetParameterGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsSetParameterGameplayTrigger {
    /// `parametersToChange` (StrongPointer (array))
    #[serde(default)]
    pub parameters_to_change: Vec<GameplayTrigger_Physics_SetParameter_BasePtr>,
}

impl Pooled for PhysicsSetParameterGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_gameplayinteractables.physics_set_parameter_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_gameplayinteractables.physics_set_parameter_gameplay_trigger }
}

impl<'a> Extract<'a> for PhysicsSetParameterGameplayTrigger {
    const TYPE_NAME: &'static str = "PhysicsSetParameterGameplayTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            parameters_to_change: inst.get_array("parametersToChange")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(GameplayTrigger_Physics_SetParameter_BasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

