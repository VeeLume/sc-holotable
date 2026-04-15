// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-mission_entities`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `HackableParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HackableParams {
    /// `baseDuration` (Single)
    #[serde(default)]
    pub base_duration: f32,
    /// `baseErrorChance` (Single)
    #[serde(default)]
    pub base_error_chance: f32,
    /// `numErrorChecks` (UInt32)
    #[serde(default)]
    pub num_error_checks: u32,
    /// `maxPauseDuration` (Single)
    #[serde(default)]
    pub max_pause_duration: f32,
    /// `updateProgressDeltaTime` (Single)
    #[serde(default)]
    pub update_progress_delta_time: f32,
    /// `resetAfterHackSuccess` (Boolean)
    #[serde(default)]
    pub reset_after_hack_success: bool,
    /// `audioTriggerProgressLoopStart` (Class)
    #[serde(default)]
    pub audio_trigger_progress_loop_start: Option<Handle<GlobalResourceAudio>>,
    /// `audioTriggerProgressLoopStop` (Class)
    #[serde(default)]
    pub audio_trigger_progress_loop_stop: Option<Handle<GlobalResourceAudio>>,
    /// `audioTriggerStart` (Class)
    #[serde(default)]
    pub audio_trigger_start: Option<Handle<GlobalResourceAudio>>,
    /// `audioTriggerPause` (Class)
    #[serde(default)]
    pub audio_trigger_pause: Option<Handle<GlobalResourceAudio>>,
    /// `audioTriggerResume` (Class)
    #[serde(default)]
    pub audio_trigger_resume: Option<Handle<GlobalResourceAudio>>,
    /// `audioTriggerError` (Class)
    #[serde(default)]
    pub audio_trigger_error: Option<Handle<GlobalResourceAudio>>,
    /// `audioTriggerPartSuccess` (Class)
    #[serde(default)]
    pub audio_trigger_part_success: Option<Handle<GlobalResourceAudio>>,
    /// `audioTriggerSuccess` (Class)
    #[serde(default)]
    pub audio_trigger_success: Option<Handle<GlobalResourceAudio>>,
    /// `audioTriggerReset` (Class)
    #[serde(default)]
    pub audio_trigger_reset: Option<Handle<GlobalResourceAudio>>,
    /// `audioRtpcOverallProgress` (Class)
    #[serde(default)]
    pub audio_rtpc_overall_progress: Option<Handle<AudioRtpc>>,
    /// `audioRtpcPartProgress` (Class)
    #[serde(default)]
    pub audio_rtpc_part_progress: Option<Handle<AudioRtpc>>,
    /// `audioRtpcTotalDuration` (Class)
    #[serde(default)]
    pub audio_rtpc_total_duration: Option<Handle<AudioRtpc>>,
    /// `audioRtpcRemainingDuration` (Class)
    #[serde(default)]
    pub audio_rtpc_remaining_duration: Option<Handle<AudioRtpc>>,
}

impl Pooled for HackableParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.hackable_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.hackable_params }
}

impl<'a> Extract<'a> for HackableParams {
    const TYPE_NAME: &'static str = "HackableParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            base_duration: inst.get_f32("baseDuration").unwrap_or_default(),
            base_error_chance: inst.get_f32("baseErrorChance").unwrap_or_default(),
            num_error_checks: inst.get_u32("numErrorChecks").unwrap_or_default(),
            max_pause_duration: inst.get_f32("maxPauseDuration").unwrap_or_default(),
            update_progress_delta_time: inst.get_f32("updateProgressDeltaTime").unwrap_or_default(),
            reset_after_hack_success: inst.get_bool("resetAfterHackSuccess").unwrap_or_default(),
            audio_trigger_progress_loop_start: match inst.get("audioTriggerProgressLoopStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_trigger_progress_loop_stop: match inst.get("audioTriggerProgressLoopStop") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_trigger_start: match inst.get("audioTriggerStart") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_trigger_pause: match inst.get("audioTriggerPause") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_trigger_resume: match inst.get("audioTriggerResume") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_trigger_error: match inst.get("audioTriggerError") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_trigger_part_success: match inst.get("audioTriggerPartSuccess") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_trigger_success: match inst.get("audioTriggerSuccess") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_trigger_reset: match inst.get("audioTriggerReset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_rtpc_overall_progress: match inst.get("audioRtpcOverallProgress") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_rtpc_part_progress: match inst.get("audioRtpcPartProgress") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_rtpc_total_duration: match inst.get("audioRtpcTotalDuration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            audio_rtpc_remaining_duration: match inst.get("audioRtpcRemainingDuration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SShopUIProviderCategoryIcon`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SShopUIProviderCategoryIcon {
    /// `category` (EnumChoice)
    #[serde(default)]
    pub category: EItemType,
    /// `icon` (String)
    #[serde(default)]
    pub icon: String,
}

impl Pooled for SShopUIProviderCategoryIcon {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.sshop_uiprovider_category_icon }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.sshop_uiprovider_category_icon }
}

impl<'a> Extract<'a> for SShopUIProviderCategoryIcon {
    const TYPE_NAME: &'static str = "SShopUIProviderCategoryIcon";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            category: EItemType::from_dcb_str(inst.get_str("category").unwrap_or("")),
            icon: inst.get_str("icon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SShopUIProviderParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SShopUIProviderParams {
    /// `categoryIcons` (Class (array))
    #[serde(default)]
    pub category_icons: Vec<Handle<SShopUIProviderCategoryIcon>>,
    /// `defaultModeBuying` (Boolean)
    #[serde(default)]
    pub default_mode_buying: bool,
    /// `degradationIcon` (String)
    #[serde(default)]
    pub degradation_icon: String,
}

impl Pooled for SShopUIProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.sshop_uiprovider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.sshop_uiprovider_params }
}

impl<'a> Extract<'a> for SShopUIProviderParams {
    const TYPE_NAME: &'static str = "SShopUIProviderParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            category_icons: inst.get_array("categoryIcons")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SShopUIProviderCategoryIcon>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SShopUIProviderCategoryIcon>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            default_mode_buying: inst.get_bool("defaultModeBuying").unwrap_or_default(),
            degradation_icon: inst.get_str("degradationIcon").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SCommodityUIProviderParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCommodityUIProviderParams {
}

impl Pooled for SCommodityUIProviderParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.scommodity_uiprovider_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.scommodity_uiprovider_params }
}

impl<'a> Extract<'a> for SCommodityUIProviderParams {
    const TYPE_NAME: &'static str = "SCommodityUIProviderParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `MonitoredZoneControllerParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoredZoneControllerParams {
    /// `stateToEnable` (WeakPointer)
    #[serde(default)]
    pub state_to_enable: Option<Handle<SInteractionState>>,
    /// `stateToDisable` (WeakPointer)
    #[serde(default)]
    pub state_to_disable: Option<Handle<SInteractionState>>,
}

impl Pooled for MonitoredZoneControllerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.monitored_zone_controller_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.monitored_zone_controller_params }
}

impl<'a> Extract<'a> for MonitoredZoneControllerParams {
    const TYPE_NAME: &'static str = "MonitoredZoneControllerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            state_to_enable: match inst.get("stateToEnable") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            state_to_disable: match inst.get("stateToDisable") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCriminalRecordStateModifier`
/// Inherits from: `SStateModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCriminalRecordStateModifier {
    /// `onInfractionSelectedState` (WeakPointer)
    #[serde(default)]
    pub on_infraction_selected_state: Option<Handle<SInteractionState>>,
    /// `onInfractionAddedState` (WeakPointer)
    #[serde(default)]
    pub on_infraction_added_state: Option<Handle<SInteractionState>>,
    /// `onInfractionRemovedState` (WeakPointer)
    #[serde(default)]
    pub on_infraction_removed_state: Option<Handle<SInteractionState>>,
    /// `onInfractionPaidState` (WeakPointer)
    #[serde(default)]
    pub on_infraction_paid_state: Option<Handle<SInteractionState>>,
    /// `onInfractionRemoveTimeEndState` (WeakPointer)
    #[serde(default)]
    pub on_infraction_remove_time_end_state: Option<Handle<SInteractionState>>,
}

impl Pooled for SCriminalRecordStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.scriminal_record_state_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.scriminal_record_state_modifier }
}

impl<'a> Extract<'a> for SCriminalRecordStateModifier {
    const TYPE_NAME: &'static str = "SCriminalRecordStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            on_infraction_selected_state: match inst.get("onInfractionSelectedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_infraction_added_state: match inst.get("onInfractionAddedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_infraction_removed_state: match inst.get("onInfractionRemovedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_infraction_paid_state: match inst.get("onInfractionPaidState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_infraction_remove_time_end_state: match inst.get("onInfractionRemoveTimeEndState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCriminalRecordHackingStateModifier`
/// Inherits from: `SStateModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCriminalRecordHackingStateModifier {
    /// `maxUnchallengedRemovals` (Int32)
    #[serde(default)]
    pub max_unchallenged_removals: i32,
    /// `detectionChance` (Single)
    #[serde(default)]
    pub detection_chance: f32,
    /// `onCrimeHackingDetectedState` (WeakPointer)
    #[serde(default)]
    pub on_crime_hacking_detected_state: Option<Handle<SInteractionState>>,
    /// `itemPort` (WeakPointer)
    #[serde(default)]
    pub item_port: Option<Handle<SItemPortDef>>,
    /// `detectionChanceMultiplierName` (String)
    #[serde(default)]
    pub detection_chance_multiplier_name: String,
}

impl Pooled for SCriminalRecordHackingStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.scriminal_record_hacking_state_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.scriminal_record_hacking_state_modifier }
}

impl<'a> Extract<'a> for SCriminalRecordHackingStateModifier {
    const TYPE_NAME: &'static str = "SCriminalRecordHackingStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_unchallenged_removals: inst.get_i32("maxUnchallengedRemovals").unwrap_or_default(),
            detection_chance: inst.get_f32("detectionChance").unwrap_or_default(),
            on_crime_hacking_detected_state: match inst.get("onCrimeHackingDetectedState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            item_port: match inst.get("itemPort") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SItemPortDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            detection_chance_multiplier_name: inst.get_str("detectionChanceMultiplierName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SGasTankFillerStateModifier`
/// Inherits from: `SStateModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SGasTankFillerStateModifier {
    /// `onGasTankFullState` (WeakPointer)
    #[serde(default)]
    pub on_gas_tank_full_state: Option<Handle<SInteractionState>>,
    /// `onGasTankFillerEmptyState` (WeakPointer)
    #[serde(default)]
    pub on_gas_tank_filler_empty_state: Option<Handle<SInteractionState>>,
}

impl Pooled for SGasTankFillerStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.sgas_tank_filler_state_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.sgas_tank_filler_state_modifier }
}

impl<'a> Extract<'a> for SGasTankFillerStateModifier {
    const TYPE_NAME: &'static str = "SGasTankFillerStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            on_gas_tank_full_state: match inst.get("onGasTankFullState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_gas_tank_filler_empty_state: match inst.get("onGasTankFillerEmptyState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SHackableStateModifier`
/// Inherits from: `SStateModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHackableStateModifier {
    /// `onHackingSuccessState` (WeakPointer)
    #[serde(default)]
    pub on_hacking_success_state: Option<Handle<SInteractionState>>,
    /// `onHackingErrorState` (WeakPointer)
    #[serde(default)]
    pub on_hacking_error_state: Option<Handle<SInteractionState>>,
    /// `onHackingResetState` (WeakPointer)
    #[serde(default)]
    pub on_hacking_reset_state: Option<Handle<SInteractionState>>,
}

impl Pooled for SHackableStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.shackable_state_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.shackable_state_modifier }
}

impl<'a> Extract<'a> for SHackableStateModifier {
    const TYPE_NAME: &'static str = "SHackableStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            on_hacking_success_state: match inst.get("onHackingSuccessState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_hacking_error_state: match inst.get("onHackingErrorState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_hacking_reset_state: match inst.get("onHackingResetState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SVendingMachineStateModifier`
/// Inherits from: `SStateModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVendingMachineStateModifier {
    /// `onCooldownEnd` (WeakPointer)
    #[serde(default)]
    pub on_cooldown_end: Option<Handle<SInteractionState>>,
    /// `onItemSpawned` (WeakPointer)
    #[serde(default)]
    pub on_item_spawned: Option<Handle<SInteractionState>>,
    /// `onItemDespawned` (WeakPointer)
    #[serde(default)]
    pub on_item_despawned: Option<Handle<SInteractionState>>,
    /// `onItemDetached` (WeakPointer)
    #[serde(default)]
    pub on_item_detached: Option<Handle<SInteractionState>>,
    /// `onItemInteracted` (WeakPointer)
    #[serde(default)]
    pub on_item_interacted: Option<Handle<SInteractionState>>,
    /// `onTransactionComplete` (WeakPointer)
    #[serde(default)]
    pub on_transaction_complete: Option<Handle<SInteractionState>>,
    /// `onHatchShouldOpen` (WeakPointer)
    #[serde(default)]
    pub on_hatch_should_open: Option<Handle<SInteractionState>>,
    /// `onHatchShouldClose` (WeakPointer)
    #[serde(default)]
    pub on_hatch_should_close: Option<Handle<SInteractionState>>,
    /// `onTransactionShouldClose` (WeakPointer)
    #[serde(default)]
    pub on_transaction_should_close: Option<Handle<SInteractionState>>,
}

impl Pooled for SVendingMachineStateModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.svending_machine_state_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.svending_machine_state_modifier }
}

impl<'a> Extract<'a> for SVendingMachineStateModifier {
    const TYPE_NAME: &'static str = "SVendingMachineStateModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            on_cooldown_end: match inst.get("onCooldownEnd") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_item_spawned: match inst.get("onItemSpawned") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_item_despawned: match inst.get("onItemDespawned") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_item_detached: match inst.get("onItemDetached") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_item_interacted: match inst.get("onItemInteracted") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_transaction_complete: match inst.get("onTransactionComplete") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_hatch_should_open: match inst.get("onHatchShouldOpen") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_hatch_should_close: match inst.get("onHatchShouldClose") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            on_transaction_should_close: match inst.get("onTransactionShouldClose") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SRemoveCrimesGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRemoveCrimesGameplayTrigger {
    /// `removeSelectedInfractionOnly` (Boolean)
    #[serde(default)]
    pub remove_selected_infraction_only: bool,
}

impl Pooled for SRemoveCrimesGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.sremove_crimes_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.sremove_crimes_gameplay_trigger }
}

impl<'a> Extract<'a> for SRemoveCrimesGameplayTrigger {
    const TYPE_NAME: &'static str = "SRemoveCrimesGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            remove_selected_infraction_only: inst.get_bool("removeSelectedInfractionOnly").unwrap_or_default(),
        }
    }
}

/// DCB type: `SStartRemoveInfractionTimerGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SStartRemoveInfractionTimerGameplayTrigger {
}

impl Pooled for SStartRemoveInfractionTimerGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.sstart_remove_infraction_timer_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.sstart_remove_infraction_timer_gameplay_trigger }
}

impl<'a> Extract<'a> for SStartRemoveInfractionTimerGameplayTrigger {
    const TYPE_NAME: &'static str = "SStartRemoveInfractionTimerGameplayTrigger";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `VendingMachineShopParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendingMachineShopParams {
}

impl Pooled for VendingMachineShopParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_mission_entities.vending_machine_shop_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_mission_entities.vending_machine_shop_params }
}

impl<'a> Extract<'a> for VendingMachineShopParams {
    const TYPE_NAME: &'static str = "VendingMachineShopParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

