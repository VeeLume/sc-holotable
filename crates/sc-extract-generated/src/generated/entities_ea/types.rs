// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-ea`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `EAScenarioComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EAScenarioComponentParams {
    /// `precachingRadius` (Single)
    #[serde(default)]
    pub precaching_radius: f32,
}

impl Pooled for EAScenarioComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ea.eascenario_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ea.eascenario_component_params }
}

impl<'a> Extract<'a> for EAScenarioComponentParams {
    const TYPE_NAME: &'static str = "EAScenarioComponentParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            precaching_radius: inst.get_f32("precachingRadius").unwrap_or_default(),
        }
    }
}

/// DCB type: `EATransportTransitionGroupParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EATransportTransitionGroupParams {
    /// `transitionTypeParams` (StrongPointer)
    #[serde(default)]
    pub transition_type_params: Option<EATransportBaseTransitionParamsPtr>,
    /// `onFinished` (EnumChoice)
    #[serde(default)]
    pub on_finished: EATransportOnTransitionFinished,
    /// `nextTransitionOverride` (Int32)
    #[serde(default)]
    pub next_transition_override: i32,
    /// `landingWaitTime` (Single)
    #[serde(default)]
    pub landing_wait_time: f32,
    /// `landingWaitTimeForTransfer` (Single)
    #[serde(default)]
    pub landing_wait_time_for_transfer: f32,
    /// `timeToDisableSpawnSelection` (Single)
    #[serde(default)]
    pub time_to_disable_spawn_selection: f32,
}

impl Pooled for EATransportTransitionGroupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ea.eatransport_transition_group_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ea.eatransport_transition_group_params }
}

impl<'a> Extract<'a> for EATransportTransitionGroupParams {
    const TYPE_NAME: &'static str = "EATransportTransitionGroupParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            transition_type_params: match inst.get("transitionTypeParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(EATransportBaseTransitionParamsPtr::from_ref(b, r)),
                _ => None,
            },
            on_finished: EATransportOnTransitionFinished::from_dcb_str(inst.get_str("onFinished").unwrap_or("")),
            next_transition_override: inst.get_i32("nextTransitionOverride").unwrap_or_default(),
            landing_wait_time: inst.get_f32("landingWaitTime").unwrap_or_default(),
            landing_wait_time_for_transfer: inst.get_f32("landingWaitTimeForTransfer").unwrap_or_default(),
            time_to_disable_spawn_selection: inst.get_f32("timeToDisableSpawnSelection").unwrap_or_default(),
        }
    }
}

/// DCB type: `EATransportControllerComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EATransportControllerComponentParams {
    /// `activePhase` (Int32)
    #[serde(default)]
    pub active_phase: i32,
    /// `spawnDelay` (Single)
    #[serde(default)]
    pub spawn_delay: f32,
    /// `onlyTransferActiveSpawns` (Boolean)
    #[serde(default)]
    pub only_transfer_active_spawns: bool,
    /// `replaceSpawningTransport` (Boolean)
    #[serde(default)]
    pub replace_spawning_transport: bool,
    /// `transitionGroups` (Class (array))
    #[serde(default)]
    pub transition_groups: Vec<Handle<EATransportTransitionGroupParams>>,
}

impl Pooled for EATransportControllerComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_ea.eatransport_controller_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_ea.eatransport_controller_component_params }
}

impl<'a> Extract<'a> for EATransportControllerComponentParams {
    const TYPE_NAME: &'static str = "EATransportControllerComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            active_phase: inst.get_i32("activePhase").unwrap_or_default(),
            spawn_delay: inst.get_f32("spawnDelay").unwrap_or_default(),
            only_transfer_active_spawns: inst.get_bool("onlyTransferActiveSpawns").unwrap_or_default(),
            replace_spawning_transport: inst.get_bool("replaceSpawningTransport").unwrap_or_default(),
            transition_groups: inst.get_array("transitionGroups")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EATransportTransitionGroupParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EATransportTransitionGroupParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

