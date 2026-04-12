// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `communicationvariableconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `CommunicationVariableConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariableConfig {
    /// `variables` (Class (array))
    #[serde(default)]
    pub variables: Vec<Handle<CommunicationVariableBool>>,
}

impl Pooled for CommunicationVariableConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationvariableconfig.communication_variable_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationvariableconfig.communication_variable_config }
}

impl<'a> Extract<'a> for CommunicationVariableConfig {
    const TYPE_NAME: &'static str = "CommunicationVariableConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            variables: inst.get_array("variables")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<CommunicationVariableBool>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<CommunicationVariableBool>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationVariableBool`
/// Inherits from: `CommunicationVariableBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationVariableBool {
    /// `name` (String)
    #[serde(default)]
    pub name: String,
    /// `global` (Boolean)
    #[serde(default)]
    pub global: bool,
    /// `value` (Boolean)
    #[serde(default)]
    pub value: bool,
}

impl Pooled for CommunicationVariableBool {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.communicationvariableconfig.communication_variable_bool }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.communicationvariableconfig.communication_variable_bool }
}

impl<'a> Extract<'a> for CommunicationVariableBool {
    const TYPE_NAME: &'static str = "CommunicationVariableBool";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            global: inst.get_bool("global").unwrap_or_default(),
            value: inst.get_bool("value").unwrap_or_default(),
        }
    }
}

