// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `audio`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `AudioBreathStyleConditionList`
pub struct AudioBreathStyleConditionList {
    /// `list` (Reference (array))
    pub list: Vec<CigGuid>,
}

impl Pooled for AudioBreathStyleConditionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_breath_style_condition_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_breath_style_condition_list }
}

impl<'a> Extract<'a> for AudioBreathStyleConditionList {
    const TYPE_NAME: &'static str = "AudioBreathStyleConditionList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            list: inst.get_array("list")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityAudioControllerTypeParams`
pub struct EntityAudioControllerTypeParams {
    /// `audioControllerParams` (StrongPointer)
    pub audio_controller_params: Option<Handle<SEntityAudioControllerParams>>,
}

impl Pooled for EntityAudioControllerTypeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.entity_audio_controller_type_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.entity_audio_controller_type_params }
}

impl<'a> Extract<'a> for EntityAudioControllerTypeParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerTypeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            audio_controller_params: match inst.get("audioControllerParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SEntityAudioControllerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `EntityAudioControllerTypeManagementParams`
pub struct EntityAudioControllerTypeManagementParams {
    /// `audioControllerEntityType` (EnumChoice)
    pub audio_controller_entity_type: EAudioControllerEntityType,
    /// `maxFullLODs` (Int32)
    pub max_full_lods: i32,
    /// `maxLowLODs` (Int32)
    pub max_low_lods: i32,
}

impl Pooled for EntityAudioControllerTypeManagementParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.entity_audio_controller_type_management_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.entity_audio_controller_type_management_params }
}

impl<'a> Extract<'a> for EntityAudioControllerTypeManagementParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerTypeManagementParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            audio_controller_entity_type: EAudioControllerEntityType::from_dcb_str(inst.get_str("audioControllerEntityType").unwrap_or("")),
            max_full_lods: inst.get_i32("maxFullLODs").unwrap_or_default(),
            max_low_lods: inst.get_i32("maxLowLODs").unwrap_or_default(),
        }
    }
}

/// DCB type: `EntityAudioControllerManagerParams`
pub struct EntityAudioControllerManagerParams {
    /// `params` (Class (array))
    pub params: Vec<Handle<EntityAudioControllerTypeManagementParams>>,
}

impl Pooled for EntityAudioControllerManagerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.entity_audio_controller_manager_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.entity_audio_controller_manager_params }
}

impl<'a> Extract<'a> for EntityAudioControllerManagerParams {
    const TYPE_NAME: &'static str = "EntityAudioControllerManagerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params: inst.get_array("params")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<EntityAudioControllerTypeManagementParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<EntityAudioControllerTypeManagementParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioWhitelist`
pub struct AudioWhitelist {
    /// `triggerTypes` (EnumChoice (array))
    pub trigger_types: Vec<EAudioTriggerType>,
}

impl Pooled for AudioWhitelist {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_whitelist }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_whitelist }
}

impl<'a> Extract<'a> for AudioWhitelist {
    const TYPE_NAME: &'static str = "AudioWhitelist";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            trigger_types: inst.get_array("triggerTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(EAudioTriggerType::from_dcb_str)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioEnvironment`
pub struct AudioEnvironment {
    /// `wwiseEnvironmentName` (String)
    pub wwise_environment_name: String,
    /// `interiorityMinimum` (Single)
    pub interiority_minimum: f32,
    /// `interiorityMaximum` (Single)
    pub interiority_maximum: f32,
    /// `sizeMinimum` (Single)
    pub size_minimum: f32,
    /// `sizeMaximum` (Single)
    pub size_maximum: f32,
    /// `primarySurfaceType` (String)
    pub primary_surface_type: String,
    /// `secondarySurfaceType` (String)
    pub secondary_surface_type: String,
}

impl Pooled for AudioEnvironment {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_environment }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_environment }
}

impl<'a> Extract<'a> for AudioEnvironment {
    const TYPE_NAME: &'static str = "AudioEnvironment";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            wwise_environment_name: inst.get_str("wwiseEnvironmentName").map(String::from).unwrap_or_default(),
            interiority_minimum: inst.get_f32("interiorityMinimum").unwrap_or_default(),
            interiority_maximum: inst.get_f32("interiorityMaximum").unwrap_or_default(),
            size_minimum: inst.get_f32("sizeMinimum").unwrap_or_default(),
            size_maximum: inst.get_f32("sizeMaximum").unwrap_or_default(),
            primary_surface_type: inst.get_str("primarySurfaceType").map(String::from).unwrap_or_default(),
            secondary_surface_type: inst.get_str("secondarySurfaceType").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioOneShotManagerBudgetEntry`
pub struct AudioOneShotManagerBudgetEntry {
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
    /// `maxAudioObjects` (Int32)
    pub max_audio_objects: i32,
    /// `priorityFalloffPerSecond` (Single)
    pub priority_falloff_per_second: f32,
}

impl Pooled for AudioOneShotManagerBudgetEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_one_shot_manager_budget_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_one_shot_manager_budget_entry }
}

impl<'a> Extract<'a> for AudioOneShotManagerBudgetEntry {
    const TYPE_NAME: &'static str = "AudioOneShotManagerBudgetEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            max_audio_objects: inst.get_i32("maxAudioObjects").unwrap_or_default(),
            priority_falloff_per_second: inst.get_f32("priorityFalloffPerSecond").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioBudgetDefinition`
pub struct AudioBudgetDefinition {
    /// `oneshotBudget` (Class (array))
    pub oneshot_budget: Vec<Handle<AudioOneShotManagerBudgetEntry>>,
    /// `shipAudioLimit` (Int32)
    pub ship_audio_limit: i32,
    /// `shipThrusterLimit` (Int32)
    pub ship_thruster_limit: i32,
    /// `actorFoleyLimit` (Int32)
    pub actor_foley_limit: i32,
}

impl Pooled for AudioBudgetDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_budget_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_budget_definition }
}

impl<'a> Extract<'a> for AudioBudgetDefinition {
    const TYPE_NAME: &'static str = "AudioBudgetDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            oneshot_budget: inst.get_array("oneshotBudget")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioOneShotManagerBudgetEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioOneShotManagerBudgetEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            ship_audio_limit: inst.get_i32("shipAudioLimit").unwrap_or_default(),
            ship_thruster_limit: inst.get_i32("shipThrusterLimit").unwrap_or_default(),
            actor_foley_limit: inst.get_i32("actorFoleyLimit").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioGameContextGlobals`
pub struct AudioGameContextGlobals {
    /// `globalStates` (Class (array))
    pub global_states: Vec<Handle<AudioSwitch>>,
    /// `globalRTPCs` (Class (array))
    pub global_rtpcs: Vec<Handle<AudioRtpcWithDefault>>,
}

impl Pooled for AudioGameContextGlobals {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_game_context_globals }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_game_context_globals }
}

impl<'a> Extract<'a> for AudioGameContextGlobals {
    const TYPE_NAME: &'static str = "AudioGameContextGlobals";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            global_states: inst.get_array("globalStates")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioSwitch>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            global_rtpcs: inst.get_array("globalRTPCs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioRtpcWithDefault>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioRtpcWithDefault>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioGameContext`
pub struct AudioGameContext {
    /// `budgetDefinition` (Reference)
    pub budget_definition: Option<CigGuid>,
    /// `globalRtpcsAndStates` (Reference)
    pub global_rtpcs_and_states: Option<CigGuid>,
}

impl Pooled for AudioGameContext {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_game_context }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_game_context }
}

impl<'a> Extract<'a> for AudioGameContext {
    const TYPE_NAME: &'static str = "AudioGameContext";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            budget_definition: inst.get("budgetDefinition").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            global_rtpcs_and_states: inst.get("globalRtpcsAndStates").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `AudioGameContextSetup`
pub struct AudioGameContextSetup {
    /// `gameContexts` (Class)
    pub game_contexts: Option<Handle<AudioGameContext>>,
}

impl Pooled for AudioGameContextSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_game_context_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_game_context_setup }
}

impl<'a> Extract<'a> for AudioGameContextSetup {
    const TYPE_NAME: &'static str = "AudioGameContextSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            game_contexts: match inst.get("gameContexts") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioGameContext>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SurfaceAudioPropertiesDefinition`
pub struct SurfaceAudioPropertiesDefinition {
    /// `surfaceAudioProps` (Class (array))
    pub surface_audio_props: Vec<Handle<SurfaceAudioProperties>>,
}

impl Pooled for SurfaceAudioPropertiesDefinition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.surface_audio_properties_definition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.surface_audio_properties_definition }
}

impl<'a> Extract<'a> for SurfaceAudioPropertiesDefinition {
    const TYPE_NAME: &'static str = "SurfaceAudioPropertiesDefinition";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            surface_audio_props: inst.get_array("surfaceAudioProps")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SurfaceAudioProperties>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SurfaceAudioProperties>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SurfaceAudioProperties`
pub struct SurfaceAudioProperties {
    /// `surfaceType` (String)
    pub surface_type: String,
    /// `damping` (Single)
    pub damping: f32,
    /// `collisionTrigger` (Class)
    pub collision_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `collisionTriggerMinPlayTime` (Single)
    pub collision_trigger_min_play_time: f32,
    /// `slideStartTrigger` (Class)
    pub slide_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `slideStopTrigger` (Class)
    pub slide_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rollStartTrigger` (Class)
    pub roll_start_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rollStopTrigger` (Class)
    pub roll_stop_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `rtpcSlideVelocity` (Class)
    pub rtpc_slide_velocity: Option<Handle<AudioRtpc>>,
    /// `rtpcRollVelocity` (Class)
    pub rtpc_roll_velocity: Option<Handle<AudioRtpc>>,
    /// `rtpcMassOther` (Class)
    pub rtpc_mass_other: Option<Handle<AudioRtpc>>,
    /// `rtpcTimeSinceLastOneshot` (Class)
    pub rtpc_time_since_last_oneshot: Option<Handle<AudioRtpc>>,
    /// `rtpcMomentum` (Class)
    pub rtpc_momentum: Option<Handle<AudioRtpc>>,
    /// `surfaceSwitchAndState` (Class)
    pub surface_switch_and_state: Option<Handle<AudioSwitch>>,
}

impl Pooled for SurfaceAudioProperties {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.surface_audio_properties }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.surface_audio_properties }
}

impl<'a> Extract<'a> for SurfaceAudioProperties {
    const TYPE_NAME: &'static str = "SurfaceAudioProperties";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            surface_type: inst.get_str("surfaceType").map(String::from).unwrap_or_default(),
            damping: inst.get_f32("damping").unwrap_or_default(),
            collision_trigger: match inst.get("collisionTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            collision_trigger_min_play_time: inst.get_f32("collisionTriggerMinPlayTime").unwrap_or_default(),
            slide_start_trigger: match inst.get("slideStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            slide_stop_trigger: match inst.get("slideStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            roll_start_trigger: match inst.get("rollStartTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            roll_stop_trigger: match inst.get("rollStopTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rtpc_slide_velocity: match inst.get("rtpcSlideVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rtpc_roll_velocity: match inst.get("rtpcRollVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rtpc_mass_other: match inst.get("rtpcMassOther") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rtpc_time_since_last_oneshot: match inst.get("rtpcTimeSinceLastOneshot") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rtpc_momentum: match inst.get("rtpcMomentum") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            surface_switch_and_state: match inst.get("surfaceSwitchAndState") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `GlobalAudioSettings`
pub struct GlobalAudioSettings {
    /// `enablePropagationPathing` (Boolean)
    pub enable_propagation_pathing: bool,
    /// `enablePropagationPathingActorOnly` (Boolean)
    pub enable_propagation_pathing_actor_only: bool,
    /// `gamePausedTrigger` (Class)
    pub game_paused_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `gameUnpausedTrigger` (Class)
    pub game_unpaused_trigger: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for GlobalAudioSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.global_audio_settings }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.global_audio_settings }
}

impl<'a> Extract<'a> for GlobalAudioSettings {
    const TYPE_NAME: &'static str = "GlobalAudioSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_propagation_pathing: inst.get_bool("enablePropagationPathing").unwrap_or_default(),
            enable_propagation_pathing_actor_only: inst.get_bool("enablePropagationPathingActorOnly").unwrap_or_default(),
            game_paused_trigger: match inst.get("gamePausedTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            game_unpaused_trigger: match inst.get("gameUnpausedTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AudioTagAction`
pub struct AudioTagAction {
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
    /// `audioTrigger` (Class)
    pub audio_trigger: Option<Handle<GlobalResourceAudio>>,
    /// `switch` (Class)
    pub switch: Option<Handle<AudioSwitch>>,
}

impl Pooled for AudioTagAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_tag_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_tag_action }
}

impl<'a> Extract<'a> for AudioTagAction {
    const TYPE_NAME: &'static str = "AudioTagAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            audio_trigger: match inst.get("audioTrigger") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            switch: match inst.get("switch") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `AudioTagActionList`
pub struct AudioTagActionList {
    /// `tagActionList` (Class (array))
    pub tag_action_list: Vec<Handle<AudioTagAction>>,
}

impl Pooled for AudioTagActionList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_tag_action_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_tag_action_list }
}

impl<'a> Extract<'a> for AudioTagActionList {
    const TYPE_NAME: &'static str = "AudioTagActionList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tag_action_list: inst.get_array("tagActionList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioTagAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioTagAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioValueOutputBehaviour`
pub struct AudioValueOutputBehaviour {
    /// `debugName` (String)
    pub debug_name: String,
}

impl Pooled for AudioValueOutputBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_value_output_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_value_output_behaviour }
}

impl<'a> Extract<'a> for AudioValueOutputBehaviour {
    const TYPE_NAME: &'static str = "AudioValueOutputBehaviour";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioValueOutputBehaviourCamera`
/// Inherits from: `AudioValueOutputBehaviour`
pub struct AudioValueOutputBehaviourCamera {
    /// `debugName` (String)
    pub debug_name: String,
    /// `enableShake` (Boolean)
    pub enable_shake: bool,
    /// `localModifier` (Single)
    pub local_modifier: f32,
    /// `offsetAngle` (Class)
    pub offset_angle: Option<Handle<Ang3>>,
    /// `offsetAngleInput` (EnumChoice)
    pub offset_angle_input: EAudioValueOutputCameraInputs,
    /// `offsetAngleModifier` (Class)
    pub offset_angle_modifier: Option<Handle<BezierCurve>>,
    /// `offsetAngleModifierUse` (Boolean)
    pub offset_angle_modifier_use: bool,
    /// `offsetPosition` (Class)
    pub offset_position: Option<Handle<Vec3>>,
    /// `offsetPositionInput` (EnumChoice)
    pub offset_position_input: EAudioValueOutputCameraInputs,
    /// `offsetPositionModifier` (Class)
    pub offset_position_modifier: Option<Handle<BezierCurve>>,
    /// `offsetPositionModifierUse` (Boolean)
    pub offset_position_modifier_use: bool,
    /// `shakesPerSecond` (Single)
    pub shakes_per_second: f32,
    /// `translationNoise` (Single)
    pub translation_noise: f32,
    /// `rotationNoise` (Single)
    pub rotation_noise: f32,
    /// `smoothFactor` (Single)
    pub smooth_factor: f32,
}

impl Pooled for AudioValueOutputBehaviourCamera {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_value_output_behaviour_camera }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_value_output_behaviour_camera }
}

impl<'a> Extract<'a> for AudioValueOutputBehaviourCamera {
    const TYPE_NAME: &'static str = "AudioValueOutputBehaviourCamera";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            enable_shake: inst.get_bool("enableShake").unwrap_or_default(),
            local_modifier: inst.get_f32("localModifier").unwrap_or_default(),
            offset_angle: match inst.get("offsetAngle") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            offset_angle_input: EAudioValueOutputCameraInputs::from_dcb_str(inst.get_str("offsetAngleInput").unwrap_or("")),
            offset_angle_modifier: match inst.get("offsetAngleModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            offset_angle_modifier_use: inst.get_bool("offsetAngleModifierUse").unwrap_or_default(),
            offset_position: match inst.get("offsetPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            offset_position_input: EAudioValueOutputCameraInputs::from_dcb_str(inst.get_str("offsetPositionInput").unwrap_or("")),
            offset_position_modifier: match inst.get("offsetPositionModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            offset_position_modifier_use: inst.get_bool("offsetPositionModifierUse").unwrap_or_default(),
            shakes_per_second: inst.get_f32("shakesPerSecond").unwrap_or_default(),
            translation_noise: inst.get_f32("translationNoise").unwrap_or_default(),
            rotation_noise: inst.get_f32("rotationNoise").unwrap_or_default(),
            smooth_factor: inst.get_f32("smoothFactor").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioValueOutputBehaviourMicrophone`
/// Inherits from: `AudioValueOutputBehaviour`
pub struct AudioValueOutputBehaviourMicrophone {
    /// `debugName` (String)
    pub debug_name: String,
}

impl Pooled for AudioValueOutputBehaviourMicrophone {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_value_output_behaviour_microphone }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_value_output_behaviour_microphone }
}

impl<'a> Extract<'a> for AudioValueOutputBehaviourMicrophone {
    const TYPE_NAME: &'static str = "AudioValueOutputBehaviourMicrophone";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioValueOutput`
pub struct AudioValueOutput {
    /// `debugName` (String)
    pub debug_name: String,
    /// `pluginInstanceID` (Int32)
    pub plugin_instance_id: i32,
    /// `behaviours` (StrongPointer (array))
    pub behaviours: Vec<AudioValueOutputBehaviourPtr>,
}

impl Pooled for AudioValueOutput {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_value_output }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_value_output }
}

impl<'a> Extract<'a> for AudioValueOutput {
    const TYPE_NAME: &'static str = "AudioValueOutput";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            plugin_instance_id: inst.get_i32("pluginInstanceID").unwrap_or_default(),
            behaviours: inst.get_array("behaviours")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(AudioValueOutputBehaviourPtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioValueOutputSetup`
pub struct AudioValueOutputSetup {
    /// `outputs` (Class (array))
    pub outputs: Vec<Handle<AudioValueOutput>>,
}

impl Pooled for AudioValueOutputSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_value_output_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_value_output_setup }
}

impl<'a> Extract<'a> for AudioValueOutputSetup {
    const TYPE_NAME: &'static str = "AudioValueOutputSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            outputs: inst.get_array("outputs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioValueOutput>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioValueOutput>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `QueueingBehaviour`
pub struct QueueingBehaviour {
    /// `canInterrupt` (Boolean)
    pub can_interrupt: bool,
    /// `canBeInterrupted` (Boolean)
    pub can_be_interrupted: bool,
    /// `canBeQueued` (Boolean)
    pub can_be_queued: bool,
}

impl Pooled for QueueingBehaviour {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.queueing_behaviour }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.queueing_behaviour }
}

impl<'a> Extract<'a> for QueueingBehaviour {
    const TYPE_NAME: &'static str = "QueueingBehaviour";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            can_interrupt: inst.get_bool("canInterrupt").unwrap_or_default(),
            can_be_interrupted: inst.get_bool("canBeInterrupted").unwrap_or_default(),
            can_be_queued: inst.get_bool("canBeQueued").unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitResponseVariation`
pub struct CockpitResponseVariation {
    /// `communicationName` (Reference)
    pub communication_name: Option<CigGuid>,
    /// `initialDelay` (Single)
    pub initial_delay: f32,
    /// `hasSfx` (Boolean)
    pub has_sfx: bool,
    /// `refireDelaySfx` (Single)
    pub refire_delay_sfx: f32,
    /// `hasDialogue` (Boolean)
    pub has_dialogue: bool,
    /// `refireDelayDialogue` (Single)
    pub refire_delay_dialogue: f32,
    /// `timeout` (Single)
    pub timeout: f32,
    /// `queuingBehaviour` (Class)
    pub queuing_behaviour: Option<Handle<QueueingBehaviour>>,
    /// `rules` (StrongPointer (array))
    pub rules: Vec<CockpitRuleBasePtr>,
}

impl Pooled for CockpitResponseVariation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_response_variation }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_response_variation }
}

impl<'a> Extract<'a> for CockpitResponseVariation {
    const TYPE_NAME: &'static str = "CockpitResponseVariation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            communication_name: inst.get("communicationName").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            initial_delay: inst.get_f32("initialDelay").unwrap_or_default(),
            has_sfx: inst.get_bool("hasSfx").unwrap_or_default(),
            refire_delay_sfx: inst.get_f32("refireDelaySfx").unwrap_or_default(),
            has_dialogue: inst.get_bool("hasDialogue").unwrap_or_default(),
            refire_delay_dialogue: inst.get_f32("refireDelayDialogue").unwrap_or_default(),
            timeout: inst.get_f32("timeout").unwrap_or_default(),
            queuing_behaviour: match inst.get("queuingBehaviour") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QueueingBehaviour>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            rules: inst.get_array("rules")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => Some(CockpitRuleBasePtr::from_ref(b, r)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitResponse`
pub struct CockpitResponse {
    /// `concept` (String)
    pub concept: String,
    /// `canPlayWhenLanded` (Boolean)
    pub can_play_when_landed: bool,
    /// `canPlayWhenRacing` (Boolean)
    pub can_play_when_racing: bool,
    /// `variations` (Class (array))
    pub variations: Vec<Handle<CockpitResponseVariation>>,
}

impl Pooled for CockpitResponse {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_response }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_response }
}

impl<'a> Extract<'a> for CockpitResponse {
    const TYPE_NAME: &'static str = "CockpitResponse";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            concept: inst.get_str("concept").map(String::from).unwrap_or_default(),
            can_play_when_landed: inst.get_bool("canPlayWhenLanded").unwrap_or_default(),
            can_play_when_racing: inst.get_bool("canPlayWhenRacing").unwrap_or_default(),
            variations: inst.get_array("variations")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CockpitResponseVariation>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<CockpitResponseVariation>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitResponses`
pub struct CockpitResponses {
    /// `responses` (Reference (array))
    pub responses: Vec<CigGuid>,
}

impl Pooled for CockpitResponses {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_responses }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_responses }
}

impl<'a> Extract<'a> for CockpitResponses {
    const TYPE_NAME: &'static str = "CockpitResponses";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            responses: inst.get_array("responses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitRuleBase`
pub struct CockpitRuleBase {
    /// `name` (String)
    pub name: String,
    /// `priority` (Single)
    pub priority: f32,
}

impl Pooled for CockpitRuleBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_rule_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_rule_base }
}

impl<'a> Extract<'a> for CockpitRuleBase {
    const TYPE_NAME: &'static str = "CockpitRuleBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            priority: inst.get_f32("priority").unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitRuleFloat`
/// Inherits from: `CockpitRuleBase`
pub struct CockpitRuleFloat {
    /// `name` (String)
    pub name: String,
    /// `priority` (Single)
    pub priority: f32,
    /// `value` (Single)
    pub value: f32,
}

impl Pooled for CockpitRuleFloat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_rule_float }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_rule_float }
}

impl<'a> Extract<'a> for CockpitRuleFloat {
    const TYPE_NAME: &'static str = "CockpitRuleFloat";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            priority: inst.get_f32("priority").unwrap_or_default(),
            value: inst.get_f32("value").unwrap_or_default(),
        }
    }
}

/// DCB type: `CockpitRuleString`
/// Inherits from: `CockpitRuleBase`
pub struct CockpitRuleString {
    /// `name` (String)
    pub name: String,
    /// `priority` (Single)
    pub priority: f32,
    /// `value` (String)
    pub value: String,
}

impl Pooled for CockpitRuleString {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.cockpit_rule_string }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.cockpit_rule_string }
}

impl<'a> Extract<'a> for CockpitRuleString {
    const TYPE_NAME: &'static str = "CockpitRuleString";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            priority: inst.get_f32("priority").unwrap_or_default(),
            value: inst.get_str("value").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioAllegianceSwitches`
pub struct AudioAllegianceSwitches {
    /// `allegianceRTPC` (Class)
    pub allegiance_rtpc: Option<Handle<AudioRtpc>>,
    /// `neutralRtpcValue` (Single)
    pub neutral_rtpc_value: f32,
    /// `friendlyRtpcValue` (Single)
    pub friendly_rtpc_value: f32,
    /// `hostileRtpcValue` (Single)
    pub hostile_rtpc_value: f32,
}

impl Pooled for AudioAllegianceSwitches {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_allegiance_switches }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_allegiance_switches }
}

impl<'a> Extract<'a> for AudioAllegianceSwitches {
    const TYPE_NAME: &'static str = "AudioAllegianceSwitches";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            allegiance_rtpc: match inst.get("allegianceRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            neutral_rtpc_value: inst.get_f32("neutralRtpcValue").unwrap_or_default(),
            friendly_rtpc_value: inst.get_f32("friendlyRtpcValue").unwrap_or_default(),
            hostile_rtpc_value: inst.get_f32("hostileRtpcValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioSignalList`
pub struct AudioSignalList {
    /// `Signals` (Class (array))
    pub signals: Vec<Handle<AudioSignal>>,
}

impl Pooled for AudioSignalList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_signal_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_signal_list }
}

impl<'a> Extract<'a> for AudioSignalList {
    const TYPE_NAME: &'static str = "AudioSignalList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            signals: inst.get_array("Signals")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioSignal>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioSignal>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TagToAudioRtpcList`
pub struct TagToAudioRtpcList {
    /// `TagsToRtpcs` (Class (array))
    pub tags_to_rtpcs: Vec<Handle<TagToAudioRtpc>>,
}

impl Pooled for TagToAudioRtpcList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.tag_to_audio_rtpc_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.tag_to_audio_rtpc_list }
}

impl<'a> Extract<'a> for TagToAudioRtpcList {
    const TYPE_NAME: &'static str = "TagToAudioRtpcList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            tags_to_rtpcs: inst.get_array("TagsToRtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<TagToAudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<TagToAudioRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioSignalRtpc`
/// Inherits from: `AudioRtpcWithDefault`
pub struct AudioSignalRtpc {
    /// `rtpc` (String)
    pub rtpc: String,
    /// `defaultValue` (Single)
    pub default_value: f32,
    /// `global` (Boolean)
    pub global: bool,
}

impl Pooled for AudioSignalRtpc {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_signal_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_signal_rtpc }
}

impl<'a> Extract<'a> for AudioSignalRtpc {
    const TYPE_NAME: &'static str = "AudioSignalRtpc";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            rtpc: inst.get_str("rtpc").map(String::from).unwrap_or_default(),
            default_value: inst.get_f32("defaultValue").unwrap_or_default(),
            global: inst.get_bool("global").unwrap_or_default(),
        }
    }
}

/// DCB type: `AudioSignal`
pub struct AudioSignal {
    /// `Name` (String)
    pub name: String,
    /// `audioTriggers` (Class (array))
    pub audio_triggers: Vec<Handle<GlobalResourceAudio>>,
    /// `switches` (Class (array))
    pub switches: Vec<Handle<AudioSwitch>>,
    /// `rtpcs` (Class (array))
    pub rtpcs: Vec<Handle<AudioSignalRtpc>>,
}

impl Pooled for AudioSignal {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.audio_signal }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.audio_signal }
}

impl<'a> Extract<'a> for AudioSignal {
    const TYPE_NAME: &'static str = "AudioSignal";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            audio_triggers: inst.get_array("audioTriggers")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<GlobalResourceAudio>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<GlobalResourceAudio>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            switches: inst.get_array("switches")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioSwitch>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioSwitch>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            rtpcs: inst.get_array("rtpcs")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<AudioSignalRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<AudioSignalRtpc>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TagToAudioRtpc`
pub struct TagToAudioRtpc {
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
    /// `rtpcName` (String)
    pub rtpc_name: String,
    /// `rtpcValue` (Single)
    pub rtpc_value: f32,
}

impl Pooled for TagToAudioRtpc {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.tag_to_audio_rtpc }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.tag_to_audio_rtpc }
}

impl<'a> Extract<'a> for TagToAudioRtpc {
    const TYPE_NAME: &'static str = "TagToAudioRtpc";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst.get("tag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            rtpc_name: inst.get_str("rtpcName").map(String::from).unwrap_or_default(),
            rtpc_value: inst.get_f32("rtpcValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `GPUParticleAudioList`
pub struct GPUParticleAudioList {
    /// `particleAudioList` (Reference (array))
    pub particle_audio_list: Vec<CigGuid>,
}

impl Pooled for GPUParticleAudioList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.gpuparticle_audio_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.gpuparticle_audio_list }
}

impl<'a> Extract<'a> for GPUParticleAudioList {
    const TYPE_NAME: &'static str = "GPUParticleAudioList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            particle_audio_list: inst.get_array("particleAudioList")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommsAudioEffect`
pub struct CommsAudioEffect {
    /// `busName` (String)
    pub bus_name: String,
    /// `locationId` (EnumChoice)
    pub location_id: ECommsRTTLocation,
}

impl Pooled for CommsAudioEffect {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.comms_audio_effect }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.comms_audio_effect }
}

impl<'a> Extract<'a> for CommsAudioEffect {
    const TYPE_NAME: &'static str = "CommsAudioEffect";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            bus_name: inst.get_str("busName").map(String::from).unwrap_or_default(),
            location_id: ECommsRTTLocation::from_dcb_str(inst.get_str("locationId").unwrap_or("")),
        }
    }
}

/// DCB type: `ShipComputerPresetList`
pub struct ShipComputerPresetList {
    /// `presets` (Reference (array))
    pub presets: Vec<CigGuid>,
}

impl Pooled for ShipComputerPresetList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.ship_computer_preset_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.ship_computer_preset_list }
}

impl<'a> Extract<'a> for ShipComputerPresetList {
    const TYPE_NAME: &'static str = "ShipComputerPresetList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            presets: inst.get_array("presets")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ShipComputerPreset`
pub struct ShipComputerPreset {
    /// `name` (String)
    pub name: String,
    /// `base` (Reference)
    pub base: Option<CigGuid>,
    /// `displayText` (Locale)
    pub display_text: LocaleKey,
    /// `responses` (Reference (array))
    pub responses: Vec<CigGuid>,
}

impl Pooled for ShipComputerPreset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.ship_computer_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.ship_computer_preset }
}

impl<'a> Extract<'a> for ShipComputerPreset {
    const TYPE_NAME: &'static str = "ShipComputerPreset";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            base: inst.get("base").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            display_text: inst.get_str("displayText").map(LocaleKey::from).unwrap_or_default(),
            responses: inst.get_array("responses")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `VideoCommsAudioParams`
pub struct VideoCommsAudioParams {
    /// `lowTechInterferenceAudioRTPC` (Class)
    pub low_tech_interference_audio_rtpc: Option<Handle<AudioRtpc>>,
    /// `highTechInterferenceAudioRTPC` (Class)
    pub high_tech_interference_audio_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for VideoCommsAudioParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.audio.video_comms_audio_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.audio.video_comms_audio_params }
}

impl<'a> Extract<'a> for VideoCommsAudioParams {
    const TYPE_NAME: &'static str = "VideoCommsAudioParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            low_tech_interference_audio_rtpc: match inst.get("lowTechInterferenceAudioRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            high_tech_interference_audio_rtpc: match inst.get("highTechInterferenceAudioRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

