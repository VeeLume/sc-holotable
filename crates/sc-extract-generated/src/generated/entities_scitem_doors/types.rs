// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-doors`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `SEffectInputParamsParticle`
/// Inherits from: `SEffectInputParamsDC`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEffectInputParamsParticle {
    /// `VarName` (String)
    #[serde(default)]
    pub var_name: String,
    /// `ParamName` (String)
    #[serde(default)]
    pub param_name: String,
    /// `MinRange` (Single)
    #[serde(default)]
    pub min_range: f32,
    /// `MaxRange` (Single)
    #[serde(default)]
    pub max_range: f32,
    /// `Multiplier` (Single)
    #[serde(default)]
    pub multiplier: f32,
    /// `LerpTime` (Single)
    #[serde(default)]
    pub lerp_time: f32,
    /// `VecGoal` (Class)
    #[serde(default)]
    pub vec_goal: Option<Handle<Vec3>>,
    /// `DefaultValue` (Single)
    #[serde(default)]
    pub default_value: f32,
    /// `Axis` (Class)
    #[serde(default)]
    pub axis: Option<Handle<Vec3>>,
    /// `Type` (EnumChoice)
    #[serde(default)]
    pub r#type: EParticleInputs,
}

impl Pooled for SEffectInputParamsParticle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.seffect_input_params_particle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.seffect_input_params_particle }
}

impl<'a> Extract<'a> for SEffectInputParamsParticle {
    const TYPE_NAME: &'static str = "SEffectInputParamsParticle";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            var_name: inst.get_str("VarName").map(String::from).unwrap_or_default(),
            param_name: inst.get_str("ParamName").map(String::from).unwrap_or_default(),
            min_range: inst.get_f32("MinRange").unwrap_or_default(),
            max_range: inst.get_f32("MaxRange").unwrap_or_default(),
            multiplier: inst.get_f32("Multiplier").unwrap_or_default(),
            lerp_time: inst.get_f32("LerpTime").unwrap_or_default(),
            vec_goal: match inst.get("VecGoal") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            default_value: inst.get_f32("DefaultValue").unwrap_or_default(),
            axis: match inst.get("Axis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            r#type: EParticleInputs::from_dcb_str(inst.get_str("Type").unwrap_or("")),
        }
    }
}

/// DCB type: `SEffectParamParticle`
/// Inherits from: `SEffectParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEffectParamParticle {
    /// `Name` (String)
    #[serde(default)]
    pub name: String,
    /// `tag` (String)
    #[serde(default)]
    pub tag: String,
    /// `Helper` (String)
    #[serde(default)]
    pub helper: String,
    /// `Offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<QuatT>>,
    /// `IsLooped` (Boolean)
    #[serde(default)]
    pub is_looped: bool,
    /// `Enabled` (Boolean)
    #[serde(default)]
    pub enabled: bool,
    /// `Prime` (Boolean)
    #[serde(default)]
    pub prime: bool,
    /// `Kill` (Boolean)
    #[serde(default)]
    pub kill: bool,
    /// `Timer` (Single)
    #[serde(default)]
    pub timer: f32,
    /// `RenderSlot` (Int32)
    #[serde(default)]
    pub render_slot: i32,
    /// `ContextFlags` (UInt32)
    #[serde(default)]
    pub context_flags: u32,
    /// `MultiPosition` (Boolean)
    #[serde(default)]
    pub multi_position: bool,
    /// `Axis` (Class)
    #[serde(default)]
    pub axis: Option<Handle<Vec3>>,
    /// `InputVariables` (Class (array))
    #[serde(default)]
    pub input_variables: Vec<Handle<SEffectInputParamsParticle>>,
}

impl Pooled for SEffectParamParticle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.seffect_param_particle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.seffect_param_particle }
}

impl<'a> Extract<'a> for SEffectParamParticle {
    const TYPE_NAME: &'static str = "SEffectParamParticle";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("Name").map(String::from).unwrap_or_default(),
            tag: inst.get_str("tag").map(String::from).unwrap_or_default(),
            helper: inst.get_str("Helper").map(String::from).unwrap_or_default(),
            offset: match inst.get("Offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<QuatT>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            is_looped: inst.get_bool("IsLooped").unwrap_or_default(),
            enabled: inst.get_bool("Enabled").unwrap_or_default(),
            prime: inst.get_bool("Prime").unwrap_or_default(),
            kill: inst.get_bool("Kill").unwrap_or_default(),
            timer: inst.get_f32("Timer").unwrap_or_default(),
            render_slot: inst.get_i32("RenderSlot").unwrap_or_default(),
            context_flags: inst.get_u32("ContextFlags").unwrap_or_default(),
            multi_position: inst.get_bool("MultiPosition").unwrap_or_default(),
            axis: match inst.get("Axis") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            input_variables: inst.get_array("InputVariables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEffectInputParamsParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SEffectInputParamsParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SEffectParamsNodeParticle`
/// Inherits from: `SEffectParamsNodeBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEffectParamsNodeParticle {
    /// `GeomTags` (String)
    #[serde(default)]
    pub geom_tags: String,
    /// `ParticleParams` (Class)
    #[serde(default)]
    pub particle_params: Option<Handle<SEffectParamParticle>>,
    /// `SubNodes` (Class (array))
    #[serde(default)]
    pub sub_nodes: Vec<Handle<SEffectParamsNodeParticle>>,
}

impl Pooled for SEffectParamsNodeParticle {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.seffect_params_node_particle }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.seffect_params_node_particle }
}

impl<'a> Extract<'a> for SEffectParamsNodeParticle {
    const TYPE_NAME: &'static str = "SEffectParamsNodeParticle";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            geom_tags: inst.get_str("GeomTags").map(String::from).unwrap_or_default(),
            particle_params: match inst.get("ParticleParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SEffectParamParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            sub_nodes: inst.get_array("SubNodes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SEffectParamsNodeParticle>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SEffectParamsNodeParticle>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GeomEntityGroupParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeomEntityGroupParams {
}

impl Pooled for GeomEntityGroupParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.geom_entity_group_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.geom_entity_group_params }
}

impl<'a> Extract<'a> for GeomEntityGroupParams {
    const TYPE_NAME: &'static str = "GeomEntityGroupParams";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SetDoorPowerStateEvent`
/// Inherits from: `EventDispatcher`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDoorPowerStateEvent {
    /// `newDoorPowerState` (EnumChoice)
    #[serde(default)]
    pub new_door_power_state: EDoorPoweredState,
}

impl Pooled for SetDoorPowerStateEvent {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.set_door_power_state_event }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.set_door_power_state_event }
}

impl<'a> Extract<'a> for SetDoorPowerStateEvent {
    const TYPE_NAME: &'static str = "SetDoorPowerStateEvent";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            new_door_power_state: EDoorPoweredState::from_dcb_str(inst.get_str("newDoorPowerState").unwrap_or("")),
        }
    }
}

/// DCB type: `SEntityTraversingNodeTypeZoneHostEntity`
/// Inherits from: `SEntityTraversingNodeTypeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SEntityTraversingNodeTypeZoneHostEntity {
}

impl Pooled for SEntityTraversingNodeTypeZoneHostEntity {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.sentity_traversing_node_type_zone_host_entity }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.sentity_traversing_node_type_zone_host_entity }
}

impl<'a> Extract<'a> for SEntityTraversingNodeTypeZoneHostEntity {
    const TYPE_NAME: &'static str = "SEntityTraversingNodeTypeZoneHostEntity";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SHydraulicPumpableComponentParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHydraulicPumpableComponentParams {
    /// `PumpPercentagePerSecondPerLever` (Single)
    #[serde(default)]
    pub pump_percentage_per_second_per_lever: f32,
    /// `DecayPercentagePerSecond` (Single)
    #[serde(default)]
    pub decay_percentage_per_second: f32,
    /// `EnabledInteractionWhenFullyCharged` (WeakPointer)
    #[serde(default)]
    pub enabled_interaction_when_fully_charged: Option<Handle<SSharedInteractionParams>>,
    /// `StartChargingEffectTag` (Reference)
    #[serde(default)]
    pub start_charging_effect_tag: Option<CigGuid>,
    /// `StartDecayingEffectTag` (Reference)
    #[serde(default)]
    pub start_decaying_effect_tag: Option<CigGuid>,
    /// `FinishChargingEffectTag` (Reference)
    #[serde(default)]
    pub finish_charging_effect_tag: Option<CigGuid>,
    /// `CurrentChargeRTPC` (Class)
    #[serde(default)]
    pub current_charge_rtpc: Option<Handle<AudioRtpc>>,
}

impl Pooled for SHydraulicPumpableComponentParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.shydraulic_pumpable_component_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.shydraulic_pumpable_component_params }
}

impl<'a> Extract<'a> for SHydraulicPumpableComponentParams {
    const TYPE_NAME: &'static str = "SHydraulicPumpableComponentParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pump_percentage_per_second_per_lever: inst.get_f32("PumpPercentagePerSecondPerLever").unwrap_or_default(),
            decay_percentage_per_second: inst.get_f32("DecayPercentagePerSecond").unwrap_or_default(),
            enabled_interaction_when_fully_charged: match inst.get("EnabledInteractionWhenFullyCharged") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SSharedInteractionParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            start_charging_effect_tag: inst.get("StartChargingEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            start_decaying_effect_tag: inst.get("StartDecayingEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            finish_charging_effect_tag: inst.get("FinishChargingEffectTag").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            current_charge_rtpc: match inst.get("CurrentChargeRTPC") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<AudioRtpc>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SInstancedInteriorGatewayParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SInstancedInteriorGatewayParams {
    /// `gatewaySize` (Int32)
    #[serde(default)]
    pub gateway_size: i32,
    /// `instance` (Reference)
    #[serde(default)]
    pub instance: Option<CigGuid>,
}

impl Pooled for SInstancedInteriorGatewayParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.sinstanced_interior_gateway_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.sinstanced_interior_gateway_params }
}

impl<'a> Extract<'a> for SInstancedInteriorGatewayParams {
    const TYPE_NAME: &'static str = "SInstancedInteriorGatewayParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            gateway_size: inst.get_i32("gatewaySize").unwrap_or_default(),
            instance: inst.get("instance").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `InteractionConditionEligibleForPrisonRelease`
/// Inherits from: `InteractionConditionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConditionEligibleForPrisonRelease {
    /// `conditionDisplay` (StrongPointer)
    #[serde(default)]
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
}

impl Pooled for InteractionConditionEligibleForPrisonRelease {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.interaction_condition_eligible_for_prison_release }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.interaction_condition_eligible_for_prison_release }
}

impl<'a> Extract<'a> for InteractionConditionEligibleForPrisonRelease {
    const TYPE_NAME: &'static str = "InteractionConditionEligibleForPrisonRelease";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConditionDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InteractionConditionAccessReservedRoom`
/// Inherits from: `InteractionConditionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConditionAccessReservedRoom {
    /// `conditionDisplay` (StrongPointer)
    #[serde(default)]
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
}

impl Pooled for InteractionConditionAccessReservedRoom {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.interaction_condition_access_reserved_room }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.interaction_condition_access_reserved_room }
}

impl<'a> Extract<'a> for InteractionConditionAccessReservedRoom {
    const TYPE_NAME: &'static str = "InteractionConditionAccessReservedRoom";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConditionDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `InteractionConditionPlayerInFront`
/// Inherits from: `InteractionConditionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConditionPlayerInFront {
    /// `conditionDisplay` (StrongPointer)
    #[serde(default)]
    pub condition_display: Option<Handle<ConditionDisplayParams>>,
    /// `frontDirection` (Class)
    #[serde(default)]
    pub front_direction: Option<Handle<Vec3>>,
}

impl Pooled for InteractionConditionPlayerInFront {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.interaction_condition_player_in_front }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.interaction_condition_player_in_front }
}

impl<'a> Extract<'a> for InteractionConditionPlayerInFront {
    const TYPE_NAME: &'static str = "InteractionConditionPlayerInFront";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            condition_display: match inst.get("conditionDisplay") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ConditionDisplayParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            front_direction: match inst.get("frontDirection") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SetDoorAutoCloseGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDoorAutoCloseGameplayTrigger {
    /// `useAutoCloseDelay` (Boolean)
    #[serde(default)]
    pub use_auto_close_delay: bool,
}

impl Pooled for SetDoorAutoCloseGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.set_door_auto_close_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.set_door_auto_close_gameplay_trigger }
}

impl<'a> Extract<'a> for SetDoorAutoCloseGameplayTrigger {
    const TYPE_NAME: &'static str = "SetDoorAutoCloseGameplayTrigger";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            use_auto_close_delay: inst.get_bool("useAutoCloseDelay").unwrap_or_default(),
        }
    }
}

/// DCB type: `LoadoutCheckGroup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutCheckGroup {
    /// `debugName` (String)
    #[serde(default)]
    pub debug_name: String,
    /// `requiredEntities` (Reference (array))
    #[serde(default)]
    pub required_entities: Vec<CigGuid>,
}

impl Pooled for LoadoutCheckGroup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.loadout_check_group }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.loadout_check_group }
}

impl<'a> Extract<'a> for LoadoutCheckGroup {
    const TYPE_NAME: &'static str = "LoadoutCheckGroup";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            required_entities: inst.get_array("requiredEntities")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LoadoutEntityCheck`
/// Inherits from: `LoadoutCheckType`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadoutEntityCheck {
    /// `entityGroup` (Class (array))
    #[serde(default)]
    pub entity_group: Vec<Handle<LoadoutCheckGroup>>,
}

impl Pooled for LoadoutEntityCheck {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.loadout_entity_check }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.loadout_entity_check }
}

impl<'a> Extract<'a> for LoadoutEntityCheck {
    const TYPE_NAME: &'static str = "LoadoutEntityCheck";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            entity_group: inst.get_array("entityGroup")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<LoadoutCheckGroup>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<LoadoutCheckGroup>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CheckEntitiesOnActorsLoadoutWithinAreaGameplayTrigger`
/// Inherits from: `SBaseInteractionGameplayTrigger`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckEntitiesOnActorsLoadoutWithinAreaGameplayTrigger {
    /// `checkType` (StrongPointer)
    #[serde(default)]
    pub check_type: Option<LoadoutCheckTypePtr>,
    /// `successState` (WeakPointer)
    #[serde(default)]
    pub success_state: Option<Handle<SInteractionState>>,
    /// `failState` (WeakPointer)
    #[serde(default)]
    pub fail_state: Option<Handle<SInteractionState>>,
}

impl Pooled for CheckEntitiesOnActorsLoadoutWithinAreaGameplayTrigger {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.check_entities_on_actors_loadout_within_area_gameplay_trigger }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.check_entities_on_actors_loadout_within_area_gameplay_trigger }
}

impl<'a> Extract<'a> for CheckEntitiesOnActorsLoadoutWithinAreaGameplayTrigger {
    const TYPE_NAME: &'static str = "CheckEntitiesOnActorsLoadoutWithinAreaGameplayTrigger";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            check_type: match inst.get("checkType") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(LoadoutCheckTypePtr::from_ref(b, r)),
                _ => None,
            },
            success_state: match inst.get("successState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            fail_state: match inst.get("failState") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SInteractionState>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemProximitySensorBoxParams`
/// Inherits from: `SCItemProximitySensorShapeParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemProximitySensorBoxParams {
    /// `Extent` (Class)
    #[serde(default)]
    pub extent: Option<Handle<Vec3>>,
    /// `Offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
}

impl Pooled for SCItemProximitySensorBoxParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.scitem_proximity_sensor_box_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.scitem_proximity_sensor_box_params }
}

impl<'a> Extract<'a> for SCItemProximitySensorBoxParams {
    const TYPE_NAME: &'static str = "SCItemProximitySensorBoxParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            extent: match inst.get("Extent") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            offset: match inst.get("Offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCItemDoorCodeProceduralParams`
/// Inherits from: `SCItemDoorAnimationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemDoorCodeProceduralParams {
    /// `DefaultAnimationDurationScale` (Single)
    #[serde(default)]
    pub default_animation_duration_scale: f32,
    /// `SecondaryAnimationDurationScale` (Single)
    #[serde(default)]
    pub secondary_animation_duration_scale: f32,
    /// `userAnimationParams` (StrongPointer)
    #[serde(default)]
    pub user_animation_params: Option<Handle<SCItemDoorUserAnimationParams>>,
    /// `OpenedOffset` (Class)
    #[serde(default)]
    pub opened_offset: Option<Handle<Vec3>>,
    /// `OpenCloseTime` (Single)
    #[serde(default)]
    pub open_close_time: f32,
    /// `OpenCloseEasing` (Single)
    #[serde(default)]
    pub open_close_easing: f32,
}

impl Pooled for SCItemDoorCodeProceduralParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.scitem_door_code_procedural_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.scitem_door_code_procedural_params }
}

impl<'a> Extract<'a> for SCItemDoorCodeProceduralParams {
    const TYPE_NAME: &'static str = "SCItemDoorCodeProceduralParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_animation_duration_scale: inst.get_f32("DefaultAnimationDurationScale").unwrap_or_default(),
            secondary_animation_duration_scale: inst.get_f32("SecondaryAnimationDurationScale").unwrap_or_default(),
            user_animation_params: match inst.get("userAnimationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SCItemDoorUserAnimationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            opened_offset: match inst.get("OpenedOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            open_close_time: inst.get_f32("OpenCloseTime").unwrap_or_default(),
            open_close_easing: inst.get_f32("OpenCloseEasing").unwrap_or_default(),
        }
    }
}

/// DCB type: `SDoorCollisionReactionToggleParams`
/// Inherits from: `SDoorCollisionReactionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDoorCollisionReactionToggleParams {
    /// `CollisionReactionDirection` (EnumChoice)
    #[serde(default)]
    pub collision_reaction_direction: EDoorCollisionReactionDirection,
}

impl Pooled for SDoorCollisionReactionToggleParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.sdoor_collision_reaction_toggle_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.sdoor_collision_reaction_toggle_params }
}

impl<'a> Extract<'a> for SDoorCollisionReactionToggleParams {
    const TYPE_NAME: &'static str = "SDoorCollisionReactionToggleParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            collision_reaction_direction: EDoorCollisionReactionDirection::from_dcb_str(inst.get_str("CollisionReactionDirection").unwrap_or("")),
        }
    }
}

/// DCB type: `SDoorCollisionReactionOpenParams`
/// Inherits from: `SDoorCollisionReactionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDoorCollisionReactionOpenParams {
    /// `CollisionReactionDirection` (EnumChoice)
    #[serde(default)]
    pub collision_reaction_direction: EDoorCollisionReactionDirection,
}

impl Pooled for SDoorCollisionReactionOpenParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.sdoor_collision_reaction_open_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.sdoor_collision_reaction_open_params }
}

impl<'a> Extract<'a> for SDoorCollisionReactionOpenParams {
    const TYPE_NAME: &'static str = "SDoorCollisionReactionOpenParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            collision_reaction_direction: EDoorCollisionReactionDirection::from_dcb_str(inst.get_str("CollisionReactionDirection").unwrap_or("")),
        }
    }
}

/// DCB type: `SCItemDoorHazardLightsParams`
/// Inherits from: `DataForgeComponentParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCItemDoorHazardLightsParams {
    /// `ProbeOffset` (Class)
    #[serde(default)]
    pub probe_offset: Option<Handle<Vec3>>,
    /// `NoHazardEffectGroup` (String)
    #[serde(default)]
    pub no_hazard_effect_group: String,
    /// `MinorHazardEffectGroup` (String)
    #[serde(default)]
    pub minor_hazard_effect_group: String,
    /// `MajorHazardEffectGroup` (String)
    #[serde(default)]
    pub major_hazard_effect_group: String,
}

impl Pooled for SCItemDoorHazardLightsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_doors.scitem_door_hazard_lights_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_doors.scitem_door_hazard_lights_params }
}

impl<'a> Extract<'a> for SCItemDoorHazardLightsParams {
    const TYPE_NAME: &'static str = "SCItemDoorHazardLightsParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            probe_offset: match inst.get("ProbeOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            no_hazard_effect_group: inst.get_str("NoHazardEffectGroup").map(String::from).unwrap_or_default(),
            minor_hazard_effect_group: inst.get_str("MinorHazardEffectGroup").map(String::from).unwrap_or_default(),
            major_hazard_effect_group: inst.get_str("MajorHazardEffectGroup").map(String::from).unwrap_or_default(),
        }
    }
}

