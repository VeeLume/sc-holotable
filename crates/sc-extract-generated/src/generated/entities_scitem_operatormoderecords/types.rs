// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-operatormoderecords`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `OperatorModeAvailability`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorModeAvailability {
    /// `defaultMode` (EnumChoice)
    #[serde(default)]
    pub default_mode: String,
    /// `availableModes` (Boolean)
    #[serde(default)]
    pub available_modes: bool,
}

impl Pooled for OperatorModeAvailability {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_operatormoderecords.operator_mode_availability }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_operatormoderecords.operator_mode_availability }
}

impl<'a> Extract<'a> for OperatorModeAvailability {
    const TYPE_NAME: &'static str = "OperatorModeAvailability";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            default_mode: inst.get_str("defaultMode").map(String::from).unwrap_or_default(),
            available_modes: inst.get_bool("availableModes").unwrap_or_default(),
        }
    }
}

/// DCB type: `OperatorModeAvailabilityParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorModeAvailabilityParams {
    /// `defaultMode` (EnumChoice)
    #[serde(default)]
    pub default_mode: String,
    /// `availableModes` (EnumChoice (array))
    #[serde(default)]
    pub available_modes: Vec<String>,
    /// `masterModes` (Class)
    #[serde(default)]
    pub master_modes: Option<Handle<OperatorModeAvailability>>,
    /// `operatorModeByMasterMode` (Class)
    #[serde(default)]
    pub operator_mode_by_master_mode: Option<Handle<OperatorModeAvailability>>,
}

impl Pooled for OperatorModeAvailabilityParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_operatormoderecords.operator_mode_availability_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_operatormoderecords.operator_mode_availability_params }
}

impl<'a> Extract<'a> for OperatorModeAvailabilityParams {
    const TYPE_NAME: &'static str = "OperatorModeAvailabilityParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_mode: inst.get_str("defaultMode").map(String::from).unwrap_or_default(),
            available_modes: inst.get_array("availableModes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
            master_modes: match inst.get("masterModes") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<OperatorModeAvailability>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<OperatorModeAvailability>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            operator_mode_by_master_mode: match inst.get("operatorModeByMasterMode") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<OperatorModeAvailability>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<OperatorModeAvailability>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `OperatorModeDefinitions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorModeDefinitions {
    /// `modeName` (EnumChoice)
    #[serde(default)]
    pub mode_name: String,
    /// `parentModeName` (EnumChoice)
    #[serde(default)]
    pub parent_mode_name: String,
    /// `controllerTypes` (EnumChoice (array))
    #[serde(default)]
    pub controller_types: Vec<String>,
}

impl Pooled for OperatorModeDefinitions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_operatormoderecords.operator_mode_definitions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_operatormoderecords.operator_mode_definitions }
}

impl<'a> Extract<'a> for OperatorModeDefinitions {
    const TYPE_NAME: &'static str = "OperatorModeDefinitions";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mode_name: inst.get_str("modeName").map(String::from).unwrap_or_default(),
            parent_mode_name: inst.get_str("parentModeName").map(String::from).unwrap_or_default(),
            controller_types: inst.get_array("controllerTypes")
                .map(|arr| arr.filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `OperatorModeDefinitionParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorModeDefinitionParams {
    /// `operatorModes` (Class (array))
    #[serde(default)]
    pub operator_modes: Vec<Handle<OperatorModeDefinitions>>,
    /// `operatorMode` (Class (array))
    #[serde(default)]
    pub operator_mode: Vec<Handle<OperatorModeDefinitions>>,
}

impl Pooled for OperatorModeDefinitionParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_operatormoderecords.operator_mode_definition_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_operatormoderecords.operator_mode_definition_params }
}

impl<'a> Extract<'a> for OperatorModeDefinitionParams {
    const TYPE_NAME: &'static str = "OperatorModeDefinitionParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            operator_modes: inst.get_array("operatorModes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<OperatorModeDefinitions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<OperatorModeDefinitions>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            operator_mode: inst.get_array("operatorMode")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<OperatorModeDefinitions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<OperatorModeDefinitions>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

