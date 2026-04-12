// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-visor`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `VisorHUD_Config`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisorHUD_Config {
    /// `controlHintsDef` (Reference)
    #[serde(default)]
    pub control_hints_def: Option<CigGuid>,
}

impl Pooled for VisorHUD_Config {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.visor_hud_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.visor_hud_config }
}

impl<'a> Extract<'a> for VisorHUD_Config {
    const TYPE_NAME: &'static str = "VisorHUD_Config";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            control_hints_def: inst.get("controlHintsDef").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `Visor_ControlHintsConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visor_ControlHintsConfig {
    /// `controlHintsPU` (Class)
    #[serde(default)]
    pub control_hints_pu: Option<Handle<ControlHintGameModeRecords>>,
    /// `controlHintsSQ42` (Class)
    #[serde(default)]
    pub control_hints_sq42: Option<Handle<ControlHintGameModeRecords>>,
}

impl Pooled for Visor_ControlHintsConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.visor_control_hints_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.visor_control_hints_config }
}

impl<'a> Extract<'a> for Visor_ControlHintsConfig {
    const TYPE_NAME: &'static str = "Visor_ControlHintsConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            control_hints_pu: match inst.get("controlHintsPU") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHintGameModeRecords>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHintGameModeRecords>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            control_hints_sq42: match inst.get("controlHintsSQ42") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHintGameModeRecords>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHintGameModeRecords>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ControlHintGameModeRecords`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintGameModeRecords {
    /// `onFootControlHints` (Reference)
    #[serde(default)]
    pub on_foot_control_hints: Option<CigGuid>,
    /// `vehicleControlHints` (Reference)
    #[serde(default)]
    pub vehicle_control_hints: Option<CigGuid>,
    /// `steeringSequenceControlHints` (Reference)
    #[serde(default)]
    pub steering_sequence_control_hints: Option<CigGuid>,
    /// `groundVehicleControlHints` (Reference)
    #[serde(default)]
    pub ground_vehicle_control_hints: Option<CigGuid>,
    /// `turretControlHints` (Reference)
    #[serde(default)]
    pub turret_control_hints: Option<CigGuid>,
    /// `boatVehicleControlHints` (Reference)
    #[serde(default)]
    pub boat_vehicle_control_hints: Option<CigGuid>,
    /// `mobiglasControlHints` (Reference)
    #[serde(default)]
    pub mobiglas_control_hints: Option<CigGuid>,
    /// `transportedControlHints` (Reference)
    #[serde(default)]
    pub transported_control_hints: Option<CigGuid>,
}

impl Pooled for ControlHintGameModeRecords {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.control_hint_game_mode_records }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.control_hint_game_mode_records }
}

impl<'a> Extract<'a> for ControlHintGameModeRecords {
    const TYPE_NAME: &'static str = "ControlHintGameModeRecords";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            on_foot_control_hints: inst.get("onFootControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            vehicle_control_hints: inst.get("vehicleControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            steering_sequence_control_hints: inst.get("steeringSequenceControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            ground_vehicle_control_hints: inst.get("groundVehicleControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            turret_control_hints: inst.get("turretControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            boat_vehicle_control_hints: inst.get("boatVehicleControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mobiglas_control_hints: inst.get("mobiglasControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            transported_control_hints: inst.get("transportedControlHints").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ControlHints_Input`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHints_Input {
    /// `activationMode` (EnumChoice)
    #[serde(default)]
    pub activation_mode: String,
}

impl Pooled for ControlHints_Input {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.control_hints_input }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.control_hints_input }
}

impl<'a> Extract<'a> for ControlHints_Input {
    const TYPE_NAME: &'static str = "ControlHints_Input";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            activation_mode: inst.get_str("activationMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHints_HintDisplayInfoAction`
/// Inherits from: `ControlHints_HintDisplayInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHints_HintDisplayInfoAction {
    /// `overrideName` (Boolean)
    #[serde(default)]
    pub override_name: bool,
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `includeSeparator` (Boolean)
    #[serde(default)]
    pub include_separator: bool,
    /// `separatorInputName` (Locale)
    #[serde(default)]
    pub separator_input_name: String,
    /// `separatorIconPath` (String)
    #[serde(default)]
    pub separator_icon_path: String,
    /// `actions` (StrongPointer (array))
    #[serde(default)]
    pub actions: Vec<Handle<ControlHints_Input>>,
}

impl Pooled for ControlHints_HintDisplayInfoAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.control_hints_hint_display_info_action }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.control_hints_hint_display_info_action }
}

impl<'a> Extract<'a> for ControlHints_HintDisplayInfoAction {
    const TYPE_NAME: &'static str = "ControlHints_HintDisplayInfoAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            override_name: inst.get_bool("overrideName").unwrap_or_default(),
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            include_separator: inst.get_bool("includeSeparator").unwrap_or_default(),
            separator_input_name: inst.get_str("separatorInputName").map(String::from).unwrap_or_default(),
            separator_icon_path: inst.get_str("separatorIconPath").map(String::from).unwrap_or_default(),
            actions: inst.get_array("actions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ControlHints_Input>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ControlHints_Input>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHint_DisplayInfoSet`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHint_DisplayInfoSet {
    /// `overrideHint` (Class)
    #[serde(default)]
    pub override_hint: Option<Handle<ControlHints_HintDisplayInfoAction>>,
    /// `gamepadHint` (StrongPointer)
    #[serde(default)]
    pub gamepad_hint: Option<Handle<ControlHints_HintDisplayInfoAction>>,
    /// `joystickHint` (StrongPointer)
    #[serde(default)]
    pub joystick_hint: Option<Handle<ControlHints_HintDisplayInfoAction>>,
    /// `alwaysShowIfBound` (Boolean)
    #[serde(default)]
    pub always_show_if_bound: bool,
}

impl Pooled for ControlHint_DisplayInfoSet {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.control_hint_display_info_set }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.control_hint_display_info_set }
}

impl<'a> Extract<'a> for ControlHint_DisplayInfoSet {
    const TYPE_NAME: &'static str = "ControlHint_DisplayInfoSet";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            override_hint: match inst.get("overrideHint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            gamepad_hint: match inst.get("gamepadHint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            joystick_hint: match inst.get("joystickHint") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHints_HintDisplayInfoAction>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            always_show_if_bound: inst.get_bool("alwaysShowIfBound").unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHintCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintCondition {
}

impl Pooled for ControlHintCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.control_hint_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.control_hint_condition }
}

impl<'a> Extract<'a> for ControlHintCondition {
    const TYPE_NAME: &'static str = "ControlHintCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ControlHintDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintDef {
    /// `description` (String)
    #[serde(default)]
    pub description: String,
    /// `condition` (StrongPointer)
    #[serde(default)]
    pub condition: Option<Handle<ControlHintCondition>>,
    /// `alwaysDisplayCondition` (StrongPointer)
    #[serde(default)]
    pub always_display_condition: Option<Handle<ControlHintAlwaysDisplayCondition>>,
    /// `hintDisplayInfoSet` (Class (array))
    #[serde(default)]
    pub hint_display_info_set: Vec<Handle<ControlHint_DisplayInfoSet>>,
}

impl Pooled for ControlHintDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.control_hint_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.control_hint_def }
}

impl<'a> Extract<'a> for ControlHintDef {
    const TYPE_NAME: &'static str = "ControlHintDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            description: inst.get_str("description").map(String::from).unwrap_or_default(),
            condition: match inst.get("condition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHintCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHintCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            always_display_condition: match inst.get("alwaysDisplayCondition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ControlHintAlwaysDisplayCondition>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ControlHintAlwaysDisplayCondition>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hint_display_info_set: inst.get_array("hintDisplayInfoSet")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ControlHint_DisplayInfoSet>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ControlHint_DisplayInfoSet>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHintAlwaysDisplayCondition`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHintAlwaysDisplayCondition {
}

impl Pooled for ControlHintAlwaysDisplayCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.control_hint_always_display_condition }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.control_hint_always_display_condition }
}

impl<'a> Extract<'a> for ControlHintAlwaysDisplayCondition {
    const TYPE_NAME: &'static str = "ControlHintAlwaysDisplayCondition";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `ControlHint_Entry`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHint_Entry {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `controlHintList` (Class (array))
    #[serde(default)]
    pub control_hint_list: Vec<Handle<ControlHintDef>>,
}

impl Pooled for ControlHint_Entry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.control_hint_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.control_hint_entry }
}

impl<'a> Extract<'a> for ControlHint_Entry {
    const TYPE_NAME: &'static str = "ControlHint_Entry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            control_hint_list: inst.get_array("controlHintList")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ControlHintDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ControlHintDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ControlHints_Preset`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHints_Preset {
    /// `hintSlots` (Class (array))
    #[serde(default)]
    pub hint_slots: Vec<Handle<ControlHint_Entry>>,
}

impl Pooled for ControlHints_Preset {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_visor.control_hints_preset }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_visor.control_hints_preset }
}

impl<'a> Extract<'a> for ControlHints_Preset {
    const TYPE_NAME: &'static str = "ControlHints_Preset";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hint_slots: inst.get_array("hintSlots")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<ControlHint_Entry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<ControlHint_Entry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

