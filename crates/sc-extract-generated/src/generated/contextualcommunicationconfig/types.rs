// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `contextualcommunicationconfig`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `CommunicationVariableBase`
pub struct CommunicationVariableBase {
    /// `name` (String)
    pub name: String,
    /// `global` (Boolean)
    pub global: bool,
}

impl Pooled for CommunicationVariableBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contextualcommunicationconfig
            .communication_variable_base
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contextualcommunicationconfig
            .communication_variable_base
    }
}

impl<'a> Extract<'a> for CommunicationVariableBase {
    const TYPE_NAME: &'static str = "CommunicationVariableBase";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            global: inst.get_bool("global").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContextualCommunicationConfig`
pub struct ContextualCommunicationConfig {
    /// `responseEntries` (Class (array))
    pub response_entries: Vec<Handle<ContextualCommunicationResponse>>,
}

impl Pooled for ContextualCommunicationConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contextualcommunicationconfig
            .contextual_communication_config
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contextualcommunicationconfig
            .contextual_communication_config
    }
}

impl<'a> Extract<'a> for ContextualCommunicationConfig {
    const TYPE_NAME: &'static str = "ContextualCommunicationConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            response_entries: inst
                .get_array("responseEntries")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<ContextualCommunicationResponse>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<ContextualCommunicationResponse>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ContextualCommunicationResponse`
pub struct ContextualCommunicationResponse {
    /// `name` (String)
    pub name: String,
    /// `concept` (EnumChoice)
    pub concept: eContextualCommunicationConcept,
    /// `customConcept` (String)
    pub custom_concept: String,
    /// `refireDelay` (Single)
    pub refire_delay: f32,
    /// `rules` (Class (array))
    pub rules: Vec<Handle<ContextualCommunicationCondition>>,
    /// `response` (Class)
    pub response: Option<Handle<CommunicationRequest>>,
    /// `memoryVariables` (StrongPointer (array))
    pub memory_variables: Vec<CommunicationVariableBasePtr>,
}

impl Pooled for ContextualCommunicationResponse {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contextualcommunicationconfig
            .contextual_communication_response
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contextualcommunicationconfig
            .contextual_communication_response
    }
}

impl<'a> Extract<'a> for ContextualCommunicationResponse {
    const TYPE_NAME: &'static str = "ContextualCommunicationResponse";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            concept: eContextualCommunicationConcept::from_dcb_str(
                inst.get_str("concept").unwrap_or(""),
            ),
            custom_concept: inst
                .get_str("customConcept")
                .map(String::from)
                .unwrap_or_default(),
            refire_delay: inst.get_f32("refireDelay").unwrap_or_default(),
            rules: inst
                .get_array("rules")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<ContextualCommunicationCondition>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => {
                            Some(b.alloc_nested::<ContextualCommunicationCondition>(
                                b.db.instance(r.struct_index, r.instance_index),
                                true,
                            ))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            response: match inst.get("response") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<CommunicationRequest>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            memory_variables: inst
                .get_array("memoryVariables")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(CommunicationVariableBasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `CommunicationRequest`
pub struct CommunicationRequest {
    /// `communication` (Reference)
    pub communication: Option<CigGuid>,
    /// `channelName` (Reference)
    pub channel_name: Option<CigGuid>,
}

impl Pooled for CommunicationRequest {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contextualcommunicationconfig.communication_request
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contextualcommunicationconfig.communication_request
    }
}

impl<'a> Extract<'a> for CommunicationRequest {
    const TYPE_NAME: &'static str = "CommunicationRequest";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            communication: inst
                .get("communication")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            channel_name: inst
                .get("channelName")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContextualCommunicationCondition`
pub struct ContextualCommunicationCondition {
    /// `criteriaType` (EnumChoice)
    pub criteria_type: eContextualCommunicationCriteria,
    /// `customCriteria` (String)
    pub custom_criteria: String,
    /// `numberValue` (Single)
    pub number_value: f32,
    /// `stringValue` (String)
    pub string_value: String,
    /// `operation` (EnumChoice)
    pub operation: eCommunicationCriteriaOperant,
}

impl Pooled for ContextualCommunicationCondition {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contextualcommunicationconfig
            .contextual_communication_condition
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contextualcommunicationconfig
            .contextual_communication_condition
    }
}

impl<'a> Extract<'a> for ContextualCommunicationCondition {
    const TYPE_NAME: &'static str = "ContextualCommunicationCondition";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            criteria_type: eContextualCommunicationCriteria::from_dcb_str(
                inst.get_str("criteriaType").unwrap_or(""),
            ),
            custom_criteria: inst
                .get_str("customCriteria")
                .map(String::from)
                .unwrap_or_default(),
            number_value: inst.get_f32("numberValue").unwrap_or_default(),
            string_value: inst
                .get_str("stringValue")
                .map(String::from)
                .unwrap_or_default(),
            operation: eCommunicationCriteriaOperant::from_dcb_str(
                inst.get_str("operation").unwrap_or(""),
            ),
        }
    }
}
