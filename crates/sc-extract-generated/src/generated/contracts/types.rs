// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `contracts`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `MissionLocationValidation_EntityTags`
/// Inherits from: `MissionLocationValidation`
pub struct MissionLocationValidation_EntityTags {
    /// `generalTag` (Reference)
    pub general_tag: Option<CigGuid>,
    /// `requiredEntityTags` (Class)
    pub required_entity_tags: Option<Handle<TagList>>,
}

impl Pooled for MissionLocationValidation_EntityTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_location_validation_entity_tags
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_location_validation_entity_tags
    }
}

impl<'a> Extract<'a> for MissionLocationValidation_EntityTags {
    const TYPE_NAME: &'static str = "MissionLocationValidation_EntityTags";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            general_tag: inst
                .get("generalTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            required_entity_tags: match inst.get("requiredEntityTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `ContractGeneratorHandler_LinearSeries`
/// Inherits from: `ContractGeneratorHandlerBase`
pub struct ContractGeneratorHandler_LinearSeries {
    /// `notForRelease` (Boolean)
    pub not_for_release: bool,
    /// `workInProgress` (Boolean)
    pub work_in_progress: bool,
    /// `debugName` (String)
    pub debug_name: String,
    /// `localGeneratorParams` (StrongPointer)
    ///
    /// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
    pub local_generator_params: Option<Handle<LocalContractGeneratorParams>>,
    /// `required_active_scenarios` (Reference (array))
    pub required_active_scenarios: Vec<CigGuid>,
    /// `defaultAvailability` (Class)
    pub default_availability: Option<Handle<ContractAvailability>>,
    /// `contractParams` (Class)
    pub contract_params: Option<Handle<ContractParamOverrides>>,
    /// `contracts` (Class (array))
    pub contracts: Vec<Handle<Contract>>,
}

impl Pooled for ContractGeneratorHandler_LinearSeries {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_generator_handler_linear_series
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_generator_handler_linear_series
    }
}

impl<'a> Extract<'a> for ContractGeneratorHandler_LinearSeries {
    const TYPE_NAME: &'static str = "ContractGeneratorHandler_LinearSeries";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            local_generator_params: match inst.get("localGeneratorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<LocalContractGeneratorParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            required_active_scenarios: inst
                .get_array("required_active_scenarios")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_availability: match inst.get("defaultAvailability") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractAvailability>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contract_params: match inst.get("contractParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractParamOverrides>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contracts: inst
                .get_array("contracts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Contract>(
                            Instance::from_inline_data(b.db, struct_index, data),
                            false,
                        )),
                        Value::ClassRef(r) => Some(b.alloc_nested::<Contract>(
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

/// DCB type: `ContractGeneratorHandler_TutorialSeriesDef`
/// Inherits from: `ContractGeneratorHandler_LinearSeries`
pub struct ContractGeneratorHandler_TutorialSeriesDef {
    /// `notForRelease` (Boolean)
    pub not_for_release: bool,
    /// `workInProgress` (Boolean)
    pub work_in_progress: bool,
    /// `debugName` (String)
    pub debug_name: String,
    /// `localGeneratorParams` (StrongPointer)
    ///
    /// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
    pub local_generator_params: Option<Handle<LocalContractGeneratorParams>>,
    /// `required_active_scenarios` (Reference (array))
    pub required_active_scenarios: Vec<CigGuid>,
    /// `defaultAvailability` (Class)
    pub default_availability: Option<Handle<ContractAvailability>>,
    /// `contractParams` (Class)
    pub contract_params: Option<Handle<ContractParamOverrides>>,
    /// `contracts` (Class (array))
    pub contracts: Vec<Handle<Contract>>,
}

impl Pooled for ContractGeneratorHandler_TutorialSeriesDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contracts
            .contract_generator_handler_tutorial_series_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contracts
            .contract_generator_handler_tutorial_series_def
    }
}

impl<'a> Extract<'a> for ContractGeneratorHandler_TutorialSeriesDef {
    const TYPE_NAME: &'static str = "ContractGeneratorHandler_TutorialSeriesDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            local_generator_params: match inst.get("localGeneratorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<LocalContractGeneratorParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            required_active_scenarios: inst
                .get_array("required_active_scenarios")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_availability: match inst.get("defaultAvailability") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractAvailability>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contract_params: match inst.get("contractParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractParamOverrides>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contracts: inst
                .get_array("contracts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Contract>(
                            Instance::from_inline_data(b.db, struct_index, data),
                            false,
                        )),
                        Value::ClassRef(r) => Some(b.alloc_nested::<Contract>(
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

/// DCB type: `ContractGeneratorHandler_List`
/// Inherits from: `ContractGeneratorHandlerBase`
pub struct ContractGeneratorHandler_List {
    /// `notForRelease` (Boolean)
    pub not_for_release: bool,
    /// `workInProgress` (Boolean)
    pub work_in_progress: bool,
    /// `debugName` (String)
    pub debug_name: String,
    /// `localGeneratorParams` (StrongPointer)
    ///
    /// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
    pub local_generator_params: Option<Handle<LocalContractGeneratorParams>>,
    /// `required_active_scenarios` (Reference (array))
    pub required_active_scenarios: Vec<CigGuid>,
    /// `defaultAvailability` (Class)
    pub default_availability: Option<Handle<ContractAvailability>>,
    /// `contractParams` (Class)
    pub contract_params: Option<Handle<ContractParamOverrides>>,
    /// `contracts` (Class (array))
    pub contracts: Vec<Handle<Contract>>,
}

impl Pooled for ContractGeneratorHandler_List {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_generator_handler_list
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_generator_handler_list
    }
}

impl<'a> Extract<'a> for ContractGeneratorHandler_List {
    const TYPE_NAME: &'static str = "ContractGeneratorHandler_List";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            local_generator_params: match inst.get("localGeneratorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<LocalContractGeneratorParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            required_active_scenarios: inst
                .get_array("required_active_scenarios")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_availability: match inst.get("defaultAvailability") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractAvailability>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contract_params: match inst.get("contractParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractParamOverrides>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contracts: inst
                .get_array("contracts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Contract>(
                            Instance::from_inline_data(b.db, struct_index, data),
                            false,
                        )),
                        Value::ClassRef(r) => Some(b.alloc_nested::<Contract>(
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

/// DCB type: `ContractGeneratorHandler_Legacy`
/// Inherits from: `ContractGeneratorHandlerBase`
pub struct ContractGeneratorHandler_Legacy {
    /// `notForRelease` (Boolean)
    pub not_for_release: bool,
    /// `workInProgress` (Boolean)
    pub work_in_progress: bool,
    /// `debugName` (String)
    pub debug_name: String,
    /// `localGeneratorParams` (StrongPointer)
    ///
    /// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
    pub local_generator_params: Option<Handle<LocalContractGeneratorParams>>,
    /// `required_active_scenarios` (Reference (array))
    pub required_active_scenarios: Vec<CigGuid>,
    /// `defaultAvailability` (Class)
    pub default_availability: Option<Handle<ContractAvailability>>,
    /// `contractParams` (Class)
    pub contract_params: Option<Handle<ContractParamOverrides>>,
    /// `legacyContracts` (Class (array))
    pub legacy_contracts: Vec<Handle<ContractLegacy>>,
}

impl Pooled for ContractGeneratorHandler_Legacy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_generator_handler_legacy
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_generator_handler_legacy
    }
}

impl<'a> Extract<'a> for ContractGeneratorHandler_Legacy {
    const TYPE_NAME: &'static str = "ContractGeneratorHandler_Legacy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            local_generator_params: match inst.get("localGeneratorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<LocalContractGeneratorParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            required_active_scenarios: inst
                .get_array("required_active_scenarios")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_availability: match inst.get("defaultAvailability") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractAvailability>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contract_params: match inst.get("contractParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractParamOverrides>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            legacy_contracts: inst
                .get_array("legacyContracts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<ContractLegacy>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<ContractLegacy>(
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

/// DCB type: `ContractLegacy`
/// Inherits from: `ContractBase`
pub struct ContractLegacy {
    /// `id` (Guid)
    pub id: CigGuid,
    /// `notForRelease` (Boolean)
    pub not_for_release: bool,
    /// `workInProgress` (Boolean)
    pub work_in_progress: bool,
    /// `debugName` (String)
    pub debug_name: String,
    /// `template` (Reference)
    pub template: Option<CigGuid>,
    /// `required_active_scenarios` (Reference (array))
    pub required_active_scenarios: Vec<CigGuid>,
    /// `paramOverrides` (Class)
    pub param_overrides: Option<Handle<ContractParamOverrides>>,
    /// `subContracts` (Class (array))
    pub sub_contracts: Vec<Handle<SubContract>>,
    /// `additionalPrerequisites` (StrongPointer (array))
    pub additional_prerequisites: Vec<ContractPrerequisiteBasePtr>,
    /// `generationParams` (StrongPointer)
    pub generation_params: Option<ContractGenerationParamsBasePtr>,
    /// `contractLifeTime` (StrongPointer)
    pub contract_life_time: Option<Handle<ContractLifeTime>>,
    /// `contractPlugins` (StrongPointer (array))
    pub contract_plugins: Vec<SContractPlugin_BasePtr>,
    /// `contractResults` (Class)
    pub contract_results: Option<Handle<ContractResults>>,
    /// `missionBrokerEntry` (Reference)
    pub mission_broker_entry: Option<CigGuid>,
}

impl Pooled for ContractLegacy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_legacy
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_legacy
    }
}

impl<'a> Extract<'a> for ContractLegacy {
    const TYPE_NAME: &'static str = "ContractLegacy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            template: inst
                .get("template")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            required_active_scenarios: inst
                .get_array("required_active_scenarios")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            param_overrides: match inst.get("paramOverrides") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractParamOverrides>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            sub_contracts: inst
                .get_array("subContracts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SubContract>(
                            Instance::from_inline_data(b.db, struct_index, data),
                            false,
                        )),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SubContract>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            additional_prerequisites: inst
                .get_array("additionalPrerequisites")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(ContractPrerequisiteBasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            generation_params: match inst.get("generationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ContractGenerationParamsBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            contract_life_time: match inst.get("contractLifeTime") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ContractLifeTime>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            contract_plugins: inst
                .get_array("contractPlugins")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(SContractPlugin_BasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            contract_results: match inst.get("contractResults") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractResults>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            mission_broker_entry: inst
                .get("missionBrokerEntry")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `CareerContract`
/// Inherits from: `ContractBase`
pub struct CareerContract {
    /// `id` (Guid)
    pub id: CigGuid,
    /// `notForRelease` (Boolean)
    pub not_for_release: bool,
    /// `workInProgress` (Boolean)
    pub work_in_progress: bool,
    /// `debugName` (String)
    pub debug_name: String,
    /// `template` (Reference)
    pub template: Option<CigGuid>,
    /// `required_active_scenarios` (Reference (array))
    pub required_active_scenarios: Vec<CigGuid>,
    /// `paramOverrides` (Class)
    pub param_overrides: Option<Handle<ContractParamOverrides>>,
    /// `subContracts` (Class (array))
    pub sub_contracts: Vec<Handle<SubContract>>,
    /// `additionalPrerequisites` (StrongPointer (array))
    pub additional_prerequisites: Vec<ContractPrerequisiteBasePtr>,
    /// `generationParams` (StrongPointer)
    pub generation_params: Option<ContractGenerationParamsBasePtr>,
    /// `contractLifeTime` (StrongPointer)
    pub contract_life_time: Option<Handle<ContractLifeTime>>,
    /// `contractPlugins` (StrongPointer (array))
    pub contract_plugins: Vec<SContractPlugin_BasePtr>,
    /// `contractResults` (Class)
    pub contract_results: Option<Handle<ContractResults>>,
    /// `minStanding` (Reference)
    pub min_standing: Option<CigGuid>,
    /// `maxStanding` (Reference)
    pub max_standing: Option<CigGuid>,
}

impl Pooled for CareerContract {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.career_contract
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.career_contract
    }
}

impl<'a> Extract<'a> for CareerContract {
    const TYPE_NAME: &'static str = "CareerContract";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            id: inst.get_guid("id").unwrap_or_default(),
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            template: inst
                .get("template")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            required_active_scenarios: inst
                .get_array("required_active_scenarios")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            param_overrides: match inst.get("paramOverrides") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractParamOverrides>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            sub_contracts: inst
                .get_array("subContracts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SubContract>(
                            Instance::from_inline_data(b.db, struct_index, data),
                            false,
                        )),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SubContract>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            additional_prerequisites: inst
                .get_array("additionalPrerequisites")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(ContractPrerequisiteBasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            generation_params: match inst.get("generationParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ContractGenerationParamsBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            contract_life_time: match inst.get("contractLifeTime") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ContractLifeTime>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            contract_plugins: inst
                .get_array("contractPlugins")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(SContractPlugin_BasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            contract_results: match inst.get("contractResults") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractResults>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            min_standing: inst
                .get("minStanding")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            max_standing: inst
                .get("maxStanding")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractGeneratorHandler_Career`
/// Inherits from: `ContractGeneratorHandlerBase`
pub struct ContractGeneratorHandler_Career {
    /// `notForRelease` (Boolean)
    pub not_for_release: bool,
    /// `workInProgress` (Boolean)
    pub work_in_progress: bool,
    /// `debugName` (String)
    pub debug_name: String,
    /// `localGeneratorParams` (StrongPointer)
    ///
    /// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
    pub local_generator_params: Option<Handle<LocalContractGeneratorParams>>,
    /// `required_active_scenarios` (Reference (array))
    pub required_active_scenarios: Vec<CigGuid>,
    /// `defaultAvailability` (Class)
    pub default_availability: Option<Handle<ContractAvailability>>,
    /// `contractParams` (Class)
    pub contract_params: Option<Handle<ContractParamOverrides>>,
    /// `introContracts` (Class (array))
    pub intro_contracts: Vec<Handle<Contract>>,
    /// `factionReputation` (Reference)
    pub faction_reputation: Option<CigGuid>,
    /// `reputationScope` (Reference)
    pub reputation_scope: Option<CigGuid>,
    /// `contracts` (Class (array))
    pub contracts: Vec<Handle<CareerContract>>,
}

impl Pooled for ContractGeneratorHandler_Career {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_generator_handler_career
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_generator_handler_career
    }
}

impl<'a> Extract<'a> for ContractGeneratorHandler_Career {
    const TYPE_NAME: &'static str = "ContractGeneratorHandler_Career";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            local_generator_params: match inst.get("localGeneratorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<LocalContractGeneratorParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            required_active_scenarios: inst
                .get_array("required_active_scenarios")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_availability: match inst.get("defaultAvailability") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractAvailability>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contract_params: match inst.get("contractParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractParamOverrides>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            intro_contracts: inst
                .get_array("introContracts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Contract>(
                            Instance::from_inline_data(b.db, struct_index, data),
                            false,
                        )),
                        Value::ClassRef(r) => Some(b.alloc_nested::<Contract>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            faction_reputation: inst
                .get("factionReputation")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            reputation_scope: inst
                .get("reputationScope")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            contracts: inst
                .get_array("contracts")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<CareerContract>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<CareerContract>(
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

/// DCB type: `SContractPlugin_SScenarioProgress`
/// Inherits from: `SContractPlugin_Base`
pub struct SContractPlugin_SScenarioProgress {
    /// `missionScenario` (Reference)
    pub mission_scenario: Option<CigGuid>,
    /// `scenarioProgressRecord` (Reference)
    pub scenario_progress_record: Option<CigGuid>,
    /// `faction` (Reference)
    pub faction: Option<CigGuid>,
    /// `splitPointsForParty` (Boolean)
    pub split_points_for_party: bool,
}

impl Pooled for SContractPlugin_SScenarioProgress {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.scontract_plugin_sscenario_progress
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.scontract_plugin_sscenario_progress
    }
}

impl<'a> Extract<'a> for SContractPlugin_SScenarioProgress {
    const TYPE_NAME: &'static str = "SContractPlugin_SScenarioProgress";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_scenario: inst
                .get("missionScenario")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            scenario_progress_record: inst
                .get("scenarioProgressRecord")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            faction: inst
                .get("faction")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            split_points_for_party: inst.get_bool("splitPointsForParty").unwrap_or_default(),
        }
    }
}

/// DCB type: `SContractPlugin_SMissionProvider`
/// Inherits from: `SContractPlugin_Base`
pub struct SContractPlugin_SMissionProvider {
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
    /// `storylineMission` (Boolean)
    pub storyline_mission: bool,
    /// `availableToAcceptFromContractManager` (Boolean)
    pub available_to_accept_from_contract_manager: bool,
}

impl Pooled for SContractPlugin_SMissionProvider {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.scontract_plugin_smission_provider
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.scontract_plugin_smission_provider
    }
}

impl<'a> Extract<'a> for SContractPlugin_SMissionProvider {
    const TYPE_NAME: &'static str = "SContractPlugin_SMissionProvider";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            tag: inst
                .get("tag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            storyline_mission: inst.get_bool("storylineMission").unwrap_or_default(),
            available_to_accept_from_contract_manager: inst
                .get_bool("availableToAcceptFromContractManager")
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractPrerequisite_Locality`
/// Inherits from: `ContractPrerequisiteBase`
pub struct ContractPrerequisite_Locality {
    /// `localityAvailable` (Reference)
    pub locality_available: Option<CigGuid>,
}

impl Pooled for ContractPrerequisite_Locality {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_prerequisite_locality
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_prerequisite_locality
    }
}

impl<'a> Extract<'a> for ContractPrerequisite_Locality {
    const TYPE_NAME: &'static str = "ContractPrerequisite_Locality";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            locality_available: inst
                .get("localityAvailable")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractPrerequisite_Location`
/// Inherits from: `ContractPrerequisiteBase`
pub struct ContractPrerequisite_Location {
    /// `locationAvailable` (Reference)
    pub location_available: Option<CigGuid>,
}

impl Pooled for ContractPrerequisite_Location {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_prerequisite_location
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_prerequisite_location
    }
}

impl<'a> Extract<'a> for ContractPrerequisite_Location {
    const TYPE_NAME: &'static str = "ContractPrerequisite_Location";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_available: inst
                .get("locationAvailable")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractPrerequisite_LocationProperty`
/// Inherits from: `ContractPrerequisiteBase`
pub struct ContractPrerequisite_LocationProperty {
    /// `propertyVariableName` (String)
    pub property_variable_name: String,
    /// `propertyExtendedTextToken` (String)
    pub property_extended_text_token: String,
    /// `locationLevelType` (EnumChoice)
    pub location_level_type: ELocationTypeLevel,
}

impl Pooled for ContractPrerequisite_LocationProperty {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_prerequisite_location_property
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_prerequisite_location_property
    }
}

impl<'a> Extract<'a> for ContractPrerequisite_LocationProperty {
    const TYPE_NAME: &'static str = "ContractPrerequisite_LocationProperty";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            property_variable_name: inst
                .get_str("propertyVariableName")
                .map(String::from)
                .unwrap_or_default(),
            property_extended_text_token: inst
                .get_str("propertyExtendedTextToken")
                .map(String::from)
                .unwrap_or_default(),
            location_level_type: ELocationTypeLevel::from_dcb_str(
                inst.get_str("locationLevelType").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `ContractPrerequisite_CrimeStat`
/// Inherits from: `ContractPrerequisiteBase`
pub struct ContractPrerequisite_CrimeStat {
    /// `includePrerequisiteWhenSharing` (Boolean)
    pub include_prerequisite_when_sharing: bool,
    /// `crimeStatJurisdictionOverride` (Reference)
    pub crime_stat_jurisdiction_override: Option<CigGuid>,
    /// `minCrimeStat` (Single)
    pub min_crime_stat: f32,
    /// `maxCrimeStat` (Single)
    pub max_crime_stat: f32,
}

impl Pooled for ContractPrerequisite_CrimeStat {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_prerequisite_crime_stat
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_prerequisite_crime_stat
    }
}

impl<'a> Extract<'a> for ContractPrerequisite_CrimeStat {
    const TYPE_NAME: &'static str = "ContractPrerequisite_CrimeStat";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            include_prerequisite_when_sharing: inst
                .get_bool("includePrerequisiteWhenSharing")
                .unwrap_or_default(),
            crime_stat_jurisdiction_override: inst
                .get("crimeStatJurisdictionOverride")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            min_crime_stat: inst.get_f32("minCrimeStat").unwrap_or_default(),
            max_crime_stat: inst.get_f32("maxCrimeStat").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractPrerequisite_Reputation`
/// Inherits from: `ContractPrerequisiteBase`
pub struct ContractPrerequisite_Reputation {
    /// `includePrerequisiteWhenSharing` (Boolean)
    pub include_prerequisite_when_sharing: bool,
    /// `factionReputation` (Reference)
    pub faction_reputation: Option<CigGuid>,
    /// `scope` (Reference)
    pub scope: Option<CigGuid>,
    /// `exclude` (Boolean)
    pub exclude: bool,
    /// `minStanding` (Reference)
    pub min_standing: Option<CigGuid>,
    /// `maxStanding` (Reference)
    pub max_standing: Option<CigGuid>,
}

impl Pooled for ContractPrerequisite_Reputation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_prerequisite_reputation
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_prerequisite_reputation
    }
}

impl<'a> Extract<'a> for ContractPrerequisite_Reputation {
    const TYPE_NAME: &'static str = "ContractPrerequisite_Reputation";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            include_prerequisite_when_sharing: inst
                .get_bool("includePrerequisiteWhenSharing")
                .unwrap_or_default(),
            faction_reputation: inst
                .get("factionReputation")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            scope: inst
                .get("scope")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            exclude: inst.get_bool("exclude").unwrap_or_default(),
            min_standing: inst
                .get("minStanding")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            max_standing: inst
                .get("maxStanding")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractPrerequisite_CompletedContractTags`
/// Inherits from: `ContractPrerequisiteBase`
pub struct ContractPrerequisite_CompletedContractTags {
    /// `includePrerequisiteWhenSharing` (Boolean)
    pub include_prerequisite_when_sharing: bool,
    /// `requiredCompletedContractTags` (Class)
    pub required_completed_contract_tags: Option<Handle<TagList>>,
    /// `requiredCountValue` (Int32)
    pub required_count_value: i32,
    /// `excludedCompletedContractTags` (Class)
    pub excluded_completed_contract_tags: Option<Handle<TagList>>,
    /// `excludedCountValue` (Int32)
    pub excluded_count_value: i32,
}

impl Pooled for ContractPrerequisite_CompletedContractTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contracts
            .contract_prerequisite_completed_contract_tags
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contracts
            .contract_prerequisite_completed_contract_tags
    }
}

impl<'a> Extract<'a> for ContractPrerequisite_CompletedContractTags {
    const TYPE_NAME: &'static str = "ContractPrerequisite_CompletedContractTags";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            include_prerequisite_when_sharing: inst
                .get_bool("includePrerequisiteWhenSharing")
                .unwrap_or_default(),
            required_completed_contract_tags: match inst.get("requiredCompletedContractTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            required_count_value: inst.get_i32("requiredCountValue").unwrap_or_default(),
            excluded_completed_contract_tags: match inst.get("excludedCompletedContractTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            excluded_count_value: inst.get_i32("excludedCountValue").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractResult_Reward`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_Reward {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `contractReward` (Class)
    pub contract_reward: Option<Handle<MissionReward>>,
}

impl Pooled for ContractResult_Reward {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_reward
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_reward
    }
}

impl<'a> Extract<'a> for ContractResult_Reward {
    const TYPE_NAME: &'static str = "ContractResult_Reward";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            contract_reward: match inst.get("contractReward") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionReward>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `ContractResult_LegacyReputation`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_LegacyReputation {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `contractResultReputationAmounts` (Class)
    pub contract_result_reputation_amounts: Option<Handle<SReputationAmountParams>>,
}

impl Pooled for ContractResult_LegacyReputation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_legacy_reputation
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_legacy_reputation
    }
}

impl<'a> Extract<'a> for ContractResult_LegacyReputation {
    const TYPE_NAME: &'static str = "ContractResult_LegacyReputation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            contract_result_reputation_amounts: match inst.get("contractResultReputationAmounts") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SReputationAmountParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `ContractResult_CalculatedReputation`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_CalculatedReputation {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `factionReputation` (Reference)
    pub faction_reputation: Option<CigGuid>,
    /// `reputationScope` (Reference)
    pub reputation_scope: Option<CigGuid>,
}

impl Pooled for ContractResult_CalculatedReputation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_calculated_reputation
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_calculated_reputation
    }
}

impl<'a> Extract<'a> for ContractResult_CalculatedReputation {
    const TYPE_NAME: &'static str = "ContractResult_CalculatedReputation";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            faction_reputation: inst
                .get("factionReputation")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            reputation_scope: inst
                .get("reputationScope")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractResult_JournalEntry`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_JournalEntry {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `journalEntriesToAdd` (Reference (array))
    pub journal_entries_to_add: Vec<CigGuid>,
    /// `journalEntriesToRemove` (Reference (array))
    pub journal_entries_to_remove: Vec<CigGuid>,
}

impl Pooled for ContractResult_JournalEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_journal_entry
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_journal_entry
    }
}

impl<'a> Extract<'a> for ContractResult_JournalEntry {
    const TYPE_NAME: &'static str = "ContractResult_JournalEntry";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            journal_entries_to_add: inst
                .get_array("journalEntriesToAdd")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            journal_entries_to_remove: inst
                .get_array("journalEntriesToRemove")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractResult_RefundBuyIn`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_RefundBuyIn {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `refundMultiplier` (Single)
    pub refund_multiplier: f32,
}

impl Pooled for ContractResult_RefundBuyIn {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_refund_buy_in
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_refund_buy_in
    }
}

impl<'a> Extract<'a> for ContractResult_RefundBuyIn {
    const TYPE_NAME: &'static str = "ContractResult_RefundBuyIn";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            refund_multiplier: inst.get_f32("refundMultiplier").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractResult_CompletionTag`
pub struct ContractResult_CompletionTag {
    /// `count` (Int32)
    pub count: i32,
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
}

impl Pooled for ContractResult_CompletionTag {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_completion_tag
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_completion_tag
    }
}

impl<'a> Extract<'a> for ContractResult_CompletionTag {
    const TYPE_NAME: &'static str = "ContractResult_CompletionTag";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            count: inst.get_i32("count").unwrap_or_default(),
            tag: inst
                .get("tag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractResult_CompletionTags`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_CompletionTags {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `completionTags` (Class (array))
    pub completion_tags: Vec<Handle<ContractResult_CompletionTag>>,
    /// `splitPointsForParty` (Boolean)
    ///
    /// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
    pub split_points_for_party: bool,
}

impl Pooled for ContractResult_CompletionTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_completion_tags
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_completion_tags
    }
}

impl<'a> Extract<'a> for ContractResult_CompletionTags {
    const TYPE_NAME: &'static str = "ContractResult_CompletionTags";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            completion_tags: inst
                .get_array("completionTags")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<ContractResult_CompletionTag>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<ContractResult_CompletionTag>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            split_points_for_party: inst.get_bool("splitPointsForParty").unwrap_or_default(),
        }
    }
}

/// DCB type: `MinCompletionTags`
pub struct MinCompletionTags {
    /// `requiredTag` (Reference)
    pub required_tag: Option<CigGuid>,
    /// `minRequired` (Int32)
    pub min_required: i32,
}

impl Pooled for MinCompletionTags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.min_completion_tags
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.min_completion_tags
    }
}

impl<'a> Extract<'a> for MinCompletionTags {
    const TYPE_NAME: &'static str = "MinCompletionTags";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            required_tag: inst
                .get("requiredTag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            min_required: inst.get_i32("minRequired").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractResult_BadgeAward`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_BadgeAward {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `requiredCompletionTags` (Class (array))
    pub required_completion_tags: Vec<Handle<MinCompletionTags>>,
    /// `badgeToAward` (EnumChoice)
    pub badge_to_award: EAwardId,
    /// `commsNotification` (Reference)
    pub comms_notification: Option<CigGuid>,
}

impl Pooled for ContractResult_BadgeAward {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_badge_award
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_badge_award
    }
}

impl<'a> Extract<'a> for ContractResult_BadgeAward {
    const TYPE_NAME: &'static str = "ContractResult_BadgeAward";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            required_completion_tags: inst
                .get_array("requiredCompletionTags")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<MinCompletionTags>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<MinCompletionTags>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            badge_to_award: EAwardId::from_dcb_str(inst.get_str("badgeToAward").unwrap_or("")),
            comms_notification: inst
                .get("commsNotification")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractResult_Item`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_Item {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `entityClass` (Reference)
    pub entity_class: Option<CigGuid>,
    /// `amount` (Int32)
    pub amount: i32,
    /// `sendToPlayerHomeLocation` (Boolean)
    pub send_to_player_home_location: bool,
    /// `awardOnlyToMissionOwner` (Boolean)
    pub award_only_to_mission_owner: bool,
    /// `targetLocation` (WeakPointer)
    pub target_location: Option<Handle<MissionProperty>>,
    /// `notification` (Class)
    pub notification: Option<Handle<RewardNotification>>,
}

impl Pooled for ContractResult_Item {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_item
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_item
    }
}

impl<'a> Extract<'a> for ContractResult_Item {
    const TYPE_NAME: &'static str = "ContractResult_Item";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            entity_class: inst
                .get("entityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            amount: inst.get_i32("amount").unwrap_or_default(),
            send_to_player_home_location: inst
                .get_bool("sendToPlayerHomeLocation")
                .unwrap_or_default(),
            award_only_to_mission_owner: inst
                .get_bool("awardOnlyToMissionOwner")
                .unwrap_or_default(),
            target_location: match inst.get("targetLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<MissionProperty>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            notification: match inst.get("notification") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<RewardNotification>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `ContractResult_CompletionBounty`
/// Inherits from: `ContractResult_CalculatedReward`
pub struct ContractResult_CompletionBounty {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
}

impl Pooled for ContractResult_CompletionBounty {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_completion_bounty
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_completion_bounty
    }
}

impl<'a> Extract<'a> for ContractResult_CompletionBounty {
    const TYPE_NAME: &'static str = "ContractResult_CompletionBounty";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ItemAwardEntityClass`
/// Inherits from: `ItemAwardBase`
pub struct ItemAwardEntityClass {
    /// `amountToAward` (Int32)
    pub amount_to_award: i32,
    /// `entityClass` (Reference)
    pub entity_class: Option<CigGuid>,
}

impl Pooled for ItemAwardEntityClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.item_award_entity_class
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.item_award_entity_class
    }
}

impl<'a> Extract<'a> for ItemAwardEntityClass {
    const TYPE_NAME: &'static str = "ItemAwardEntityClass";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            amount_to_award: inst.get_i32("amountToAward").unwrap_or_default(),
            entity_class: inst
                .get("entityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ItemAwardWeightingsParams`
/// Inherits from: `ItemAwardWeightingsBase`
pub struct ItemAwardWeightingsParams {
    /// `awardsRecord` (Reference)
    pub awards_record: Option<CigGuid>,
}

impl Pooled for ItemAwardWeightingsParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.item_award_weightings_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.item_award_weightings_params
    }
}

impl<'a> Extract<'a> for ItemAwardWeightingsParams {
    const TYPE_NAME: &'static str = "ItemAwardWeightingsParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            awards_record: inst
                .get("awardsRecord")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractResult_ItemsWeighting`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_ItemsWeighting {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `targetLocation` (WeakPointer)
    pub target_location: Option<Handle<MissionProperty>>,
    /// `awardOnlyToMissionOwner` (Boolean)
    pub award_only_to_mission_owner: bool,
    /// `itemAwardStructure` (StrongPointer (array))
    pub item_award_structure: Vec<ItemAwardWeightingsBasePtr>,
    /// `notification` (Class)
    pub notification: Option<Handle<RewardNotification>>,
}

impl Pooled for ContractResult_ItemsWeighting {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_items_weighting
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_items_weighting
    }
}

impl<'a> Extract<'a> for ContractResult_ItemsWeighting {
    const TYPE_NAME: &'static str = "ContractResult_ItemsWeighting";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            target_location: match inst.get("targetLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<MissionProperty>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            award_only_to_mission_owner: inst
                .get_bool("awardOnlyToMissionOwner")
                .unwrap_or_default(),
            item_award_structure: inst
                .get_array("itemAwardStructure")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(ItemAwardWeightingsBasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            notification: match inst.get("notification") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<RewardNotification>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `ContractResult_ScenarioProgress`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_ScenarioProgress {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `PointsToAward` (Int32)
    pub points_to_award: i32,
    /// `scenarioProgressPlugin` (Class)
    pub scenario_progress_plugin: Option<Handle<SContractPlugin_SScenarioProgress>>,
}

impl Pooled for ContractResult_ScenarioProgress {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_scenario_progress
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_scenario_progress
    }
}

impl<'a> Extract<'a> for ContractResult_ScenarioProgress {
    const TYPE_NAME: &'static str = "ContractResult_ScenarioProgress";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            points_to_award: inst.get_i32("PointsToAward").unwrap_or_default(),
            scenario_progress_plugin: match inst.get("scenarioProgressPlugin") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SContractPlugin_SScenarioProgress>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `RewardNotification`
pub struct RewardNotification {
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `enablePrompt` (Boolean)
    pub enable_prompt: bool,
    /// `awardNotification` (Locale)
    pub award_notification: LocaleKey,
    /// `multiAwardNotification` (Locale)
    pub multi_award_notification: LocaleKey,
    /// `selectionPrompt` (Locale)
    pub selection_prompt: LocaleKey,
    /// `shipKioskDestination` (Locale)
    pub ship_kiosk_destination: LocaleKey,
    /// `freightElevatorDestination` (Locale)
    pub freight_elevator_destination: LocaleKey,
    /// `freightElevatorCannotRetrieve` (Locale)
    pub freight_elevator_cannot_retrieve: LocaleKey,
    /// `rewardExtendedTextToken` (String)
    pub reward_extended_text_token: String,
    /// `destinationLocationExtendedTextToken` (String)
    pub destination_location_extended_text_token: String,
    /// `kioskDestinationExtendedTextToken` (String)
    pub kiosk_destination_extended_text_token: String,
    /// `rewardAmountExtendedTextToken` (String)
    pub reward_amount_extended_text_token: String,
    /// `musicWwiseEvent` (Class)
    pub music_wwise_event: Option<Handle<GlobalResourceAudio>>,
}

impl Pooled for RewardNotification {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.reward_notification
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.reward_notification
    }
}

impl<'a> Extract<'a> for RewardNotification {
    const TYPE_NAME: &'static str = "RewardNotification";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            enable_prompt: inst.get_bool("enablePrompt").unwrap_or_default(),
            award_notification: inst
                .get_str("awardNotification")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            multi_award_notification: inst
                .get_str("multiAwardNotification")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            selection_prompt: inst
                .get_str("selectionPrompt")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            ship_kiosk_destination: inst
                .get_str("shipKioskDestination")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            freight_elevator_destination: inst
                .get_str("freightElevatorDestination")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            freight_elevator_cannot_retrieve: inst
                .get_str("freightElevatorCannotRetrieve")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            reward_extended_text_token: inst
                .get_str("rewardExtendedTextToken")
                .map(String::from)
                .unwrap_or_default(),
            destination_location_extended_text_token: inst
                .get_str("destinationLocationExtendedTextToken")
                .map(String::from)
                .unwrap_or_default(),
            kiosk_destination_extended_text_token: inst
                .get_str("kioskDestinationExtendedTextToken")
                .map(String::from)
                .unwrap_or_default(),
            reward_amount_extended_text_token: inst
                .get_str("rewardAmountExtendedTextToken")
                .map(String::from)
                .unwrap_or_default(),
            music_wwise_event: match inst.get("musicWwiseEvent") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<GlobalResourceAudio>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `BlueprintRewards`
/// Inherits from: `ContractResultBase`
pub struct BlueprintRewards {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `chance` (Single)
    pub chance: f32,
    /// `blueprintPool` (Reference)
    pub blueprint_pool: Option<CigGuid>,
}

impl Pooled for BlueprintRewards {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.blueprint_rewards
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.blueprint_rewards
    }
}

impl<'a> Extract<'a> for BlueprintRewards {
    const TYPE_NAME: &'static str = "BlueprintRewards";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            chance: inst.get_f32("chance").unwrap_or_default(),
            blueprint_pool: inst
                .get("blueprintPool")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `ContractResult_VehicleRental`
/// Inherits from: `ContractResultBase`
pub struct ContractResult_VehicleRental {
    /// `missionResults` (Boolean (array))
    pub mission_results: Vec<bool>,
    /// `vehicleEntityClass` (Reference)
    pub vehicle_entity_class: Option<CigGuid>,
    /// `durationMinutes` (Int32)
    pub duration_minutes: i32,
}

impl Pooled for ContractResult_VehicleRental {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_result_vehicle_rental
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_result_vehicle_rental
    }
}

impl<'a> Extract<'a> for ContractResult_VehicleRental {
    const TYPE_NAME: &'static str = "ContractResult_VehicleRental";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_results: inst
                .get_array("missionResults")
                .map(|arr| arr.filter_map(|v| v.as_bool()).collect())
                .unwrap_or_default(),
            vehicle_entity_class: inst
                .get("vehicleEntityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            duration_minutes: inst.get_i32("durationMinutes").unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractClass_PVPBounty`
/// Inherits from: `ContractClassBase`
pub struct ContractClass_PVPBounty {}

impl Pooled for ContractClass_PVPBounty {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_class_pvpbounty
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_class_pvpbounty
    }
}

impl<'a> Extract<'a> for ContractClass_PVPBounty {
    const TYPE_NAME: &'static str = "ContractClass_PVPBounty";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `HaulingOrderContent_Resource`
/// Inherits from: `HaulingOrderContent_ResourceBase`
pub struct HaulingOrderContent_Resource {
    /// `resource` (Reference)
    pub resource: Option<CigGuid>,
    /// `maxContainerSize` (Single)
    pub max_container_size: f32,
    /// `minSCU` (Single)
    pub min_scu: f32,
    /// `maxSCU` (Single)
    pub max_scu: f32,
}

impl Pooled for HaulingOrderContent_Resource {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_content_resource
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_content_resource
    }
}

impl<'a> Extract<'a> for HaulingOrderContent_Resource {
    const TYPE_NAME: &'static str = "HaulingOrderContent_Resource";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            resource: inst
                .get("resource")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            max_container_size: inst.get_f32("maxContainerSize").unwrap_or_default(),
            min_scu: inst.get_f32("minSCU").unwrap_or_default(),
            max_scu: inst.get_f32("maxSCU").unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrderContent_EntityClasses`
/// Inherits from: `HaulingOrderContentBase`
pub struct HaulingOrderContent_EntityClasses {
    /// `haulingEntityClasses` (Reference)
    pub hauling_entity_classes: Option<CigGuid>,
    /// `minAmount` (Int32)
    pub min_amount: i32,
    /// `maxAmount` (Int32)
    pub max_amount: i32,
}

impl Pooled for HaulingOrderContent_EntityClasses {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_content_entity_classes
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_content_entity_classes
    }
}

impl<'a> Extract<'a> for HaulingOrderContent_EntityClasses {
    const TYPE_NAME: &'static str = "HaulingOrderContent_EntityClasses";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hauling_entity_classes: inst
                .get("haulingEntityClasses")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            min_amount: inst.get_i32("minAmount").unwrap_or_default(),
            max_amount: inst.get_i32("maxAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrderContent_EntityClass`
/// Inherits from: `HaulingOrderContentBase`
pub struct HaulingOrderContent_EntityClass {
    /// `entityClass` (Reference)
    pub entity_class: Option<CigGuid>,
    /// `minAmount` (Int32)
    pub min_amount: i32,
    /// `maxAmount` (Int32)
    pub max_amount: i32,
}

impl Pooled for HaulingOrderContent_EntityClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_content_entity_class
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_content_entity_class
    }
}

impl<'a> Extract<'a> for HaulingOrderContent_EntityClass {
    const TYPE_NAME: &'static str = "HaulingOrderContent_EntityClass";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            entity_class: inst
                .get("entityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            min_amount: inst.get_i32("minAmount").unwrap_or_default(),
            max_amount: inst.get_i32("maxAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrderContent_MissionItem`
/// Inherits from: `HaulingOrderContentBase`
pub struct HaulingOrderContent_MissionItem {
    /// `item` (WeakPointer)
    pub item: Option<Handle<MissionProperty>>,
    /// `minAmount` (Int32)
    pub min_amount: i32,
    /// `maxAmount` (Int32)
    pub max_amount: i32,
}

impl Pooled for HaulingOrderContent_MissionItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_content_mission_item
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_content_mission_item
    }
}

impl<'a> Extract<'a> for HaulingOrderContent_MissionItem {
    const TYPE_NAME: &'static str = "HaulingOrderContent_MissionItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            item: match inst.get("item") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<MissionProperty>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            min_amount: inst.get_i32("minAmount").unwrap_or_default(),
            max_amount: inst.get_i32("maxAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrder_Property`
/// Inherits from: `HaulingOrder_PropertyBase`
pub struct HaulingOrder_Property {
    /// `pickUpLocation` (WeakPointer)
    pub pick_up_location: Option<ObjectivePropertyBasePtr>,
    /// `dropOffLocation` (WeakPointer)
    pub drop_off_location: Option<ObjectivePropertyBasePtr>,
    /// `pickUpTargetTypes` (Reference (array))
    pub pick_up_target_types: Vec<CigGuid>,
    /// `dropOffTargetTypes` (Reference (array))
    pub drop_off_target_types: Vec<CigGuid>,
    /// `haulingOrdersProperty` (WeakPointer)
    pub hauling_orders_property: Option<ObjectivePropertyBasePtr>,
}

impl Pooled for HaulingOrder_Property {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_property
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_property
    }
}

impl<'a> Extract<'a> for HaulingOrder_Property {
    const TYPE_NAME: &'static str = "HaulingOrder_Property";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pick_up_location: match inst.get("pickUpLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            drop_off_location: match inst.get("dropOffLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            pick_up_target_types: inst
                .get_array("pickUpTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            drop_off_target_types: inst
                .get_array("dropOffTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hauling_orders_property: match inst.get("haulingOrdersProperty") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `HaulingOrder_ResourceUnlimitedDropOff`
/// Inherits from: `HaulingOrder_ResourceBase`
pub struct HaulingOrder_ResourceUnlimitedDropOff {
    /// `pickUpLocation` (WeakPointer)
    pub pick_up_location: Option<ObjectivePropertyBasePtr>,
    /// `dropOffLocation` (WeakPointer)
    pub drop_off_location: Option<ObjectivePropertyBasePtr>,
    /// `pickUpTargetTypes` (Reference (array))
    pub pick_up_target_types: Vec<CigGuid>,
    /// `dropOffTargetTypes` (Reference (array))
    pub drop_off_target_types: Vec<CigGuid>,
    /// `resource` (Reference)
    pub resource: Option<CigGuid>,
    /// `maxContainerSize` (Single)
    pub max_container_size: f32,
}

impl Pooled for HaulingOrder_ResourceUnlimitedDropOff {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_resource_unlimited_drop_off
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_resource_unlimited_drop_off
    }
}

impl<'a> Extract<'a> for HaulingOrder_ResourceUnlimitedDropOff {
    const TYPE_NAME: &'static str = "HaulingOrder_ResourceUnlimitedDropOff";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pick_up_location: match inst.get("pickUpLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            drop_off_location: match inst.get("dropOffLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            pick_up_target_types: inst
                .get_array("pickUpTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            drop_off_target_types: inst
                .get_array("dropOffTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            resource: inst
                .get("resource")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            max_container_size: inst.get_f32("maxContainerSize").unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrder_EntityClasses`
/// Inherits from: `HaulingOrderBase`
pub struct HaulingOrder_EntityClasses {
    /// `pickUpLocation` (WeakPointer)
    pub pick_up_location: Option<ObjectivePropertyBasePtr>,
    /// `dropOffLocation` (WeakPointer)
    pub drop_off_location: Option<ObjectivePropertyBasePtr>,
    /// `pickUpTargetTypes` (Reference (array))
    pub pick_up_target_types: Vec<CigGuid>,
    /// `dropOffTargetTypes` (Reference (array))
    pub drop_off_target_types: Vec<CigGuid>,
    /// `haulingEntityClasses` (Reference)
    pub hauling_entity_classes: Option<CigGuid>,
    /// `minAmount` (Int32)
    pub min_amount: i32,
    /// `maxAmount` (Int32)
    pub max_amount: i32,
}

impl Pooled for HaulingOrder_EntityClasses {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_entity_classes
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_entity_classes
    }
}

impl<'a> Extract<'a> for HaulingOrder_EntityClasses {
    const TYPE_NAME: &'static str = "HaulingOrder_EntityClasses";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pick_up_location: match inst.get("pickUpLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            drop_off_location: match inst.get("dropOffLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            pick_up_target_types: inst
                .get_array("pickUpTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            drop_off_target_types: inst
                .get_array("dropOffTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            hauling_entity_classes: inst
                .get("haulingEntityClasses")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            min_amount: inst.get_i32("minAmount").unwrap_or_default(),
            max_amount: inst.get_i32("maxAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrder_CollectOnly`
/// Inherits from: `HaulingOrderBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct HaulingOrder_CollectOnly {
    /// `pickUpLocation` (WeakPointer)
    pub pick_up_location: Option<ObjectivePropertyBasePtr>,
    /// `dropOffLocation` (WeakPointer)
    pub drop_off_location: Option<ObjectivePropertyBasePtr>,
    /// `pickUpTargetTypes` (Reference (array))
    pub pick_up_target_types: Vec<CigGuid>,
    /// `dropOffTargetTypes` (Reference (array))
    pub drop_off_target_types: Vec<CigGuid>,
    /// `item` (WeakPointer)
    pub item: Option<ObjectivePropertyBasePtr>,
    /// `minAmount` (Int32)
    pub min_amount: i32,
    /// `maxAmount` (Int32)
    pub max_amount: i32,
}

impl Pooled for HaulingOrder_CollectOnly {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_collect_only
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_collect_only
    }
}

impl<'a> Extract<'a> for HaulingOrder_CollectOnly {
    const TYPE_NAME: &'static str = "HaulingOrder_CollectOnly";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pick_up_location: match inst.get("pickUpLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            drop_off_location: match inst.get("dropOffLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            pick_up_target_types: inst
                .get_array("pickUpTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            drop_off_target_types: inst
                .get_array("dropOffTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            item: match inst.get("item") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            min_amount: inst.get_i32("minAmount").unwrap_or_default(),
            max_amount: inst.get_i32("maxAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrder_EntityClass`
/// Inherits from: `HaulingOrderBase`
pub struct HaulingOrder_EntityClass {
    /// `pickUpLocation` (WeakPointer)
    pub pick_up_location: Option<ObjectivePropertyBasePtr>,
    /// `dropOffLocation` (WeakPointer)
    pub drop_off_location: Option<ObjectivePropertyBasePtr>,
    /// `pickUpTargetTypes` (Reference (array))
    pub pick_up_target_types: Vec<CigGuid>,
    /// `dropOffTargetTypes` (Reference (array))
    pub drop_off_target_types: Vec<CigGuid>,
    /// `entityClass` (Reference)
    pub entity_class: Option<CigGuid>,
    /// `minAmount` (Int32)
    pub min_amount: i32,
    /// `maxAmount` (Int32)
    pub max_amount: i32,
}

impl Pooled for HaulingOrder_EntityClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_entity_class
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_entity_class
    }
}

impl<'a> Extract<'a> for HaulingOrder_EntityClass {
    const TYPE_NAME: &'static str = "HaulingOrder_EntityClass";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pick_up_location: match inst.get("pickUpLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            drop_off_location: match inst.get("dropOffLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            pick_up_target_types: inst
                .get_array("pickUpTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            drop_off_target_types: inst
                .get_array("dropOffTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            entity_class: inst
                .get("entityClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            min_amount: inst.get_i32("minAmount").unwrap_or_default(),
            max_amount: inst.get_i32("maxAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrder_MissionItem`
/// Inherits from: `HaulingOrderBase`
pub struct HaulingOrder_MissionItem {
    /// `pickUpLocation` (WeakPointer)
    pub pick_up_location: Option<ObjectivePropertyBasePtr>,
    /// `dropOffLocation` (WeakPointer)
    pub drop_off_location: Option<ObjectivePropertyBasePtr>,
    /// `pickUpTargetTypes` (Reference (array))
    pub pick_up_target_types: Vec<CigGuid>,
    /// `dropOffTargetTypes` (Reference (array))
    pub drop_off_target_types: Vec<CigGuid>,
    /// `item` (WeakPointer)
    pub item: Option<ObjectivePropertyBasePtr>,
    /// `minAmount` (Int32)
    pub min_amount: i32,
    /// `maxAmount` (Int32)
    pub max_amount: i32,
}

impl Pooled for HaulingOrder_MissionItem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_mission_item
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_mission_item
    }
}

impl<'a> Extract<'a> for HaulingOrder_MissionItem {
    const TYPE_NAME: &'static str = "HaulingOrder_MissionItem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pick_up_location: match inst.get("pickUpLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            drop_off_location: match inst.get("dropOffLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            pick_up_target_types: inst
                .get_array("pickUpTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            drop_off_target_types: inst
                .get_array("dropOffTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            item: match inst.get("item") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            min_amount: inst.get_i32("minAmount").unwrap_or_default(),
            max_amount: inst.get_i32("maxAmount").unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrder_MissionItemDropOff`
/// Inherits from: `HaulingOrderBase`
pub struct HaulingOrder_MissionItemDropOff {
    /// `pickUpLocation` (WeakPointer)
    pub pick_up_location: Option<ObjectivePropertyBasePtr>,
    /// `dropOffLocation` (WeakPointer)
    pub drop_off_location: Option<ObjectivePropertyBasePtr>,
    /// `pickUpTargetTypes` (Reference (array))
    pub pick_up_target_types: Vec<CigGuid>,
    /// `dropOffTargetTypes` (Reference (array))
    pub drop_off_target_types: Vec<CigGuid>,
    /// `deliveryOrderInput` (WeakPointer)
    pub delivery_order_input: Option<Handle<ObjectiveProperty_Input>>,
    /// `dropOffInstanceBrokerTags` (Reference (array))
    pub drop_off_instance_broker_tags: Vec<CigGuid>,
}

impl Pooled for HaulingOrder_MissionItemDropOff {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_mission_item_drop_off
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_mission_item_drop_off
    }
}

impl<'a> Extract<'a> for HaulingOrder_MissionItemDropOff {
    const TYPE_NAME: &'static str = "HaulingOrder_MissionItemDropOff";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pick_up_location: match inst.get("pickUpLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            drop_off_location: match inst.get("dropOffLocation") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            pick_up_target_types: inst
                .get_array("pickUpTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            drop_off_target_types: inst
                .get_array("dropOffTargetTypes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            delivery_order_input: match inst.get("deliveryOrderInput") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<ObjectiveProperty_Input>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            drop_off_instance_broker_tags: inst
                .get_array("dropOffInstanceBrokerTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrder_OrOption_And`
/// Inherits from: `HaulingOrder_OrOption_Base`
pub struct HaulingOrder_OrOption_And {
    /// `orders` (StrongPointer (array))
    pub orders: Vec<HaulingOrderContentBasePtr>,
}

impl Pooled for HaulingOrder_OrOption_And {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_or_option_and
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_or_option_and
    }
}

impl<'a> Extract<'a> for HaulingOrder_OrOption_And {
    const TYPE_NAME: &'static str = "HaulingOrder_OrOption_And";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            orders: inst
                .get_array("orders")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(HaulingOrderContentBasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `HaulingOrderContent_Or`
/// Inherits from: `HaulingOrderContentBase`
pub struct HaulingOrderContent_Or {
    /// `options` (StrongPointer (array))
    pub options: Vec<HaulingOrder_OrOption_BasePtr>,
}

impl Pooled for HaulingOrderContent_Or {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.hauling_order_content_or
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.hauling_order_content_or
    }
}

impl<'a> Extract<'a> for HaulingOrderContent_Or {
    const TYPE_NAME: &'static str = "HaulingOrderContent_Or";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            options: inst
                .get_array("options")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(HaulingOrder_OrOption_BasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationCondition_RequiredLocationClass`
/// Inherits from: `LocationCondition_LocationClassBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_RequiredLocationClass {
    /// `locationClass` (Reference)
    pub location_class: Option<CigGuid>,
}

impl Pooled for LocationCondition_RequiredLocationClass {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_required_location_class
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_required_location_class
    }
}

impl<'a> Extract<'a> for LocationCondition_RequiredLocationClass {
    const TYPE_NAME: &'static str = "LocationCondition_RequiredLocationClass";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_class: inst
                .get("locationClass")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `LocationCondition_RequiredLocationClasses`
/// Inherits from: `LocationCondition_LocationClassBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_RequiredLocationClasses {
    /// `locationClasses` (Reference (array))
    pub location_classes: Vec<CigGuid>,
}

impl Pooled for LocationCondition_RequiredLocationClasses {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_required_location_classes
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_required_location_classes
    }
}

impl<'a> Extract<'a> for LocationCondition_RequiredLocationClasses {
    const TYPE_NAME: &'static str = "LocationCondition_RequiredLocationClasses";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_classes: inst
                .get_array("locationClasses")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationCondition_RequiredLocationSize`
/// Inherits from: `LocationCondition_LocationSizeBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_RequiredLocationSize {
    /// `locationSize` (Reference)
    pub location_size: Option<CigGuid>,
}

impl Pooled for LocationCondition_RequiredLocationSize {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_required_location_size
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_required_location_size
    }
}

impl<'a> Extract<'a> for LocationCondition_RequiredLocationSize {
    const TYPE_NAME: &'static str = "LocationCondition_RequiredLocationSize";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_size: inst
                .get("locationSize")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `LocationCondition_ExcludedLocationSize`
/// Inherits from: `LocationCondition_LocationSizeBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_ExcludedLocationSize {
    /// `locationSize` (Reference)
    pub location_size: Option<CigGuid>,
}

impl Pooled for LocationCondition_ExcludedLocationSize {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_excluded_location_size
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_excluded_location_size
    }
}

impl<'a> Extract<'a> for LocationCondition_ExcludedLocationSize {
    const TYPE_NAME: &'static str = "LocationCondition_ExcludedLocationSize";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_size: inst
                .get("locationSize")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `LocationCondition_RequiredLocationSizes`
/// Inherits from: `LocationCondition_LocationSizeBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_RequiredLocationSizes {
    /// `locationSizes` (Reference (array))
    pub location_sizes: Vec<CigGuid>,
}

impl Pooled for LocationCondition_RequiredLocationSizes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_required_location_sizes
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_required_location_sizes
    }
}

impl<'a> Extract<'a> for LocationCondition_RequiredLocationSizes {
    const TYPE_NAME: &'static str = "LocationCondition_RequiredLocationSizes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_sizes: inst
                .get_array("locationSizes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationCondition_ExcludedLocationSizes`
/// Inherits from: `LocationCondition_LocationSizeBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_ExcludedLocationSizes {
    /// `locationSizes` (Reference (array))
    pub location_sizes: Vec<CigGuid>,
}

impl Pooled for LocationCondition_ExcludedLocationSizes {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_excluded_location_sizes
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_excluded_location_sizes
    }
}

impl<'a> Extract<'a> for LocationCondition_ExcludedLocationSizes {
    const TYPE_NAME: &'static str = "LocationCondition_ExcludedLocationSizes";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_sizes: inst
                .get_array("locationSizes")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationCondition_RequiredLocationFunction`
/// Inherits from: `LocationCondition_LocationFunctionBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_RequiredLocationFunction {
    /// `locationFunction` (Reference)
    pub location_function: Option<CigGuid>,
}

impl Pooled for LocationCondition_RequiredLocationFunction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contracts
            .location_condition_required_location_function
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contracts
            .location_condition_required_location_function
    }
}

impl<'a> Extract<'a> for LocationCondition_RequiredLocationFunction {
    const TYPE_NAME: &'static str = "LocationCondition_RequiredLocationFunction";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_function: inst
                .get("locationFunction")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `LocationCondition_ExcludedLocationFunction`
/// Inherits from: `LocationCondition_LocationFunctionBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_ExcludedLocationFunction {
    /// `locationFunction` (Reference)
    pub location_function: Option<CigGuid>,
}

impl Pooled for LocationCondition_ExcludedLocationFunction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contracts
            .location_condition_excluded_location_function
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contracts
            .location_condition_excluded_location_function
    }
}

impl<'a> Extract<'a> for LocationCondition_ExcludedLocationFunction {
    const TYPE_NAME: &'static str = "LocationCondition_ExcludedLocationFunction";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_function: inst
                .get("locationFunction")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `LocationCondition_RequiredLocationFunctions`
/// Inherits from: `LocationCondition_LocationFunctionBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_RequiredLocationFunctions {
    /// `locationFunctions` (Reference (array))
    pub location_functions: Vec<CigGuid>,
}

impl Pooled for LocationCondition_RequiredLocationFunctions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contracts
            .location_condition_required_location_functions
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contracts
            .location_condition_required_location_functions
    }
}

impl<'a> Extract<'a> for LocationCondition_RequiredLocationFunctions {
    const TYPE_NAME: &'static str = "LocationCondition_RequiredLocationFunctions";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_functions: inst
                .get_array("locationFunctions")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationCondition_ExcludedLocationFunctions`
/// Inherits from: `LocationCondition_LocationFunctionBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_ExcludedLocationFunctions {
    /// `locationFunctions` (Reference (array))
    pub location_functions: Vec<CigGuid>,
}

impl Pooled for LocationCondition_ExcludedLocationFunctions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contracts
            .location_condition_excluded_location_functions
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contracts
            .location_condition_excluded_location_functions
    }
}

impl<'a> Extract<'a> for LocationCondition_ExcludedLocationFunctions {
    const TYPE_NAME: &'static str = "LocationCondition_ExcludedLocationFunctions";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_functions: inst
                .get_array("locationFunctions")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `LocationCondition_RequiredFactionHostility`
/// Inherits from: `LocationCondition_FactionBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_RequiredFactionHostility {
    /// `hostility` (EnumChoice)
    pub hostility: ReactionType,
}

impl Pooled for LocationCondition_RequiredFactionHostility {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contracts
            .location_condition_required_faction_hostility
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contracts
            .location_condition_required_faction_hostility
    }
}

impl<'a> Extract<'a> for LocationCondition_RequiredFactionHostility {
    const TYPE_NAME: &'static str = "LocationCondition_RequiredFactionHostility";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hostility: ReactionType::from_dcb_str(inst.get_str("hostility").unwrap_or("")),
        }
    }
}

/// DCB type: `LocationCondition_ExcludedFactionHostility`
/// Inherits from: `LocationCondition_FactionBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_ExcludedFactionHostility {
    /// `hostility` (EnumChoice)
    pub hostility: ReactionType,
}

impl Pooled for LocationCondition_ExcludedFactionHostility {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contracts
            .location_condition_excluded_faction_hostility
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contracts
            .location_condition_excluded_faction_hostility
    }
}

impl<'a> Extract<'a> for LocationCondition_ExcludedFactionHostility {
    const TYPE_NAME: &'static str = "LocationCondition_ExcludedFactionHostility";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hostility: ReactionType::from_dcb_str(inst.get_str("hostility").unwrap_or("")),
        }
    }
}

/// DCB type: `LocationCondition_NotLocation`
/// Inherits from: `LocationCondition_OtherLocationBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_NotLocation {
    /// `location` (WeakPointer)
    pub location: Option<Handle<MissionProperty>>,
}

impl Pooled for LocationCondition_NotLocation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_not_location
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_not_location
    }
}

impl<'a> Extract<'a> for LocationCondition_NotLocation {
    const TYPE_NAME: &'static str = "LocationCondition_NotLocation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location: match inst.get("location") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<MissionProperty>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `LocationCondition_NotLocations`
/// Inherits from: `LocationCondition_OtherLocationBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_NotLocations {
    /// `locations` (WeakPointer (array))
    pub locations: Vec<Handle<MissionProperty>>,
}

impl Pooled for LocationCondition_NotLocations {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_not_locations
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_not_locations
    }
}

impl<'a> Extract<'a> for LocationCondition_NotLocations {
    const TYPE_NAME: &'static str = "LocationCondition_NotLocations";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            locations: inst
                .get_array("locations")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(b.alloc_nested::<MissionProperty>(
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

/// DCB type: `LocationCondition_SameScope`
/// Inherits from: `LocationCondition_RelativeLocationBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_SameScope {
    /// `scope` (EnumChoice)
    pub scope: ContractScope,
    /// `location` (WeakPointer)
    pub location: Option<Handle<MissionProperty>>,
}

impl Pooled for LocationCondition_SameScope {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_same_scope
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_same_scope
    }
}

impl<'a> Extract<'a> for LocationCondition_SameScope {
    const TYPE_NAME: &'static str = "LocationCondition_SameScope";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope: ContractScope::from_dcb_str(inst.get_str("scope").unwrap_or("")),
            location: match inst.get("location") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<MissionProperty>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `LocationCondition_DifferentScope`
/// Inherits from: `LocationCondition_RelativeLocationBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct LocationCondition_DifferentScope {
    /// `scope` (EnumChoice)
    pub scope: ContractScope,
    /// `location` (WeakPointer)
    pub location: Option<Handle<MissionProperty>>,
}

impl Pooled for LocationCondition_DifferentScope {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.location_condition_different_scope
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.location_condition_different_scope
    }
}

impl<'a> Extract<'a> for LocationCondition_DifferentScope {
    const TYPE_NAME: &'static str = "LocationCondition_DifferentScope";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope: ContractScope::from_dcb_str(inst.get_str("scope").unwrap_or("")),
            location: match inst.get("location") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<MissionProperty>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionPropertyValue_ScopedLocation`
/// Inherits from: `BaseMissionPropertyValue`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct MissionPropertyValue_ScopedLocation {
    /// `factionFilter` (StrongPointer)
    pub faction_filter: Option<LocationCondition_FactionBasePtr>,
    /// `locationClassFilter` (StrongPointer)
    pub location_class_filter: Option<LocationCondition_LocationClassBasePtr>,
    /// `locationSizeFilter` (StrongPointer)
    pub location_size_filter: Option<LocationCondition_LocationSizeBasePtr>,
    /// `locationFunctionFilter` (StrongPointer)
    pub location_function_filter: Option<LocationCondition_LocationFunctionBasePtr>,
    /// `otherLocationFilter` (StrongPointer)
    pub other_location_filter: Option<LocationCondition_OtherLocationBasePtr>,
    /// `relativeLocationFilter` (StrongPointer)
    pub relative_location_filter: Option<LocationCondition_RelativeLocationBasePtr>,
}

impl Pooled for MissionPropertyValue_ScopedLocation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_property_value_scoped_location
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_property_value_scoped_location
    }
}

impl<'a> Extract<'a> for MissionPropertyValue_ScopedLocation {
    const TYPE_NAME: &'static str = "MissionPropertyValue_ScopedLocation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            faction_filter: match inst.get("factionFilter") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(LocationCondition_FactionBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            location_class_filter: match inst.get("locationClassFilter") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(LocationCondition_LocationClassBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            location_size_filter: match inst.get("locationSizeFilter") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(LocationCondition_LocationSizeBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            location_function_filter: match inst.get("locationFunctionFilter") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(LocationCondition_LocationFunctionBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            other_location_filter: match inst.get("otherLocationFilter") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(LocationCondition_OtherLocationBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            relative_location_filter: match inst.get("relativeLocationFilter") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(LocationCondition_RelativeLocationBasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionFlowCondition_AllTokensState`
/// Inherits from: `MissionFlowConditionBase`
pub struct MissionFlowCondition_AllTokensState {
    /// `tokenState` (EnumChoice)
    pub token_state: EMissionPhaseStates,
}

impl Pooled for MissionFlowCondition_AllTokensState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_flow_condition_all_tokens_state
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_flow_condition_all_tokens_state
    }
}

impl<'a> Extract<'a> for MissionFlowCondition_AllTokensState {
    const TYPE_NAME: &'static str = "MissionFlowCondition_AllTokensState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            token_state: EMissionPhaseStates::from_dcb_str(
                inst.get_str("tokenState").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `MissionFlowCondition_AnyTokensState`
/// Inherits from: `MissionFlowConditionBase`
pub struct MissionFlowCondition_AnyTokensState {
    /// `tokenState` (EnumChoice)
    pub token_state: EMissionPhaseStates,
}

impl Pooled for MissionFlowCondition_AnyTokensState {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_flow_condition_any_tokens_state
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_flow_condition_any_tokens_state
    }
}

impl<'a> Extract<'a> for MissionFlowCondition_AnyTokensState {
    const TYPE_NAME: &'static str = "MissionFlowCondition_AnyTokensState";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            token_state: EMissionPhaseStates::from_dcb_str(
                inst.get_str("tokenState").unwrap_or(""),
            ),
        }
    }
}

/// DCB type: `MissionFlowCondition_OnMissionStart`
/// Inherits from: `MissionFlowConditionBase`
pub struct MissionFlowCondition_OnMissionStart {}

impl Pooled for MissionFlowCondition_OnMissionStart {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_flow_condition_on_mission_start
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_flow_condition_on_mission_start
    }
}

impl<'a> Extract<'a> for MissionFlowCondition_OnMissionStart {
    const TYPE_NAME: &'static str = "MissionFlowCondition_OnMissionStart";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {}
    }
}

/// DCB type: `MissionFlowCondition_OR`
/// Inherits from: `MissionFlowConditionBase`
pub struct MissionFlowCondition_OR {
    /// `conditions` (StrongPointer (array))
    pub conditions: Vec<MissionFlowConditionBasePtr>,
}

impl Pooled for MissionFlowCondition_OR {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_flow_condition_or
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_flow_condition_or
    }
}

impl<'a> Extract<'a> for MissionFlowCondition_OR {
    const TYPE_NAME: &'static str = "MissionFlowCondition_OR";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            conditions: inst
                .get_array("conditions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(MissionFlowConditionBasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionRandomPhaseEntry`
pub struct MissionRandomPhaseEntry {
    /// `objective` (WeakPointer)
    pub objective: Option<ObjectiveTokenPtr>,
    /// `probability` (Single)
    pub probability: f32,
}

impl Pooled for MissionRandomPhaseEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_random_phase_entry
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_random_phase_entry
    }
}

impl<'a> Extract<'a> for MissionRandomPhaseEntry {
    const TYPE_NAME: &'static str = "MissionRandomPhaseEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            objective: match inst.get("objective") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectiveTokenPtr::from_ref(b, r))
                }
                _ => None,
            },
            probability: inst.get_f32("probability").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionFlowAction_PickRandomMissionPhase`
/// Inherits from: `MissionFlowActionBase`
pub struct MissionFlowAction_PickRandomMissionPhase {
    /// `missionPhases` (Class (array))
    pub mission_phases: Vec<Handle<MissionRandomPhaseEntry>>,
}

impl Pooled for MissionFlowAction_PickRandomMissionPhase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .contracts
            .mission_flow_action_pick_random_mission_phase
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .contracts
            .mission_flow_action_pick_random_mission_phase
    }
}

impl<'a> Extract<'a> for MissionFlowAction_PickRandomMissionPhase {
    const TYPE_NAME: &'static str = "MissionFlowAction_PickRandomMissionPhase";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_phases: inst
                .get_array("missionPhases")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<MissionRandomPhaseEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<MissionRandomPhaseEntry>(
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

/// DCB type: `MissionRandomActionEntry`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct MissionRandomActionEntry {
    /// `action` (StrongPointer)
    pub action: Option<MissionFlowActionBasePtr>,
    /// `probability` (Single)
    pub probability: f32,
}

impl Pooled for MissionRandomActionEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_random_action_entry
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_random_action_entry
    }
}

impl<'a> Extract<'a> for MissionRandomActionEntry {
    const TYPE_NAME: &'static str = "MissionRandomActionEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            action: match inst.get("action") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(MissionFlowActionBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            probability: inst.get_f32("probability").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionFlowAction_PickRandomAction`
/// Inherits from: `MissionFlowActionBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct MissionFlowAction_PickRandomAction {
    /// `actions` (Class (array))
    pub actions: Vec<Handle<MissionRandomActionEntry>>,
}

impl Pooled for MissionFlowAction_PickRandomAction {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_flow_action_pick_random_action
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_flow_action_pick_random_action
    }
}

impl<'a> Extract<'a> for MissionFlowAction_PickRandomAction {
    const TYPE_NAME: &'static str = "MissionFlowAction_PickRandomAction";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actions: inst
                .get_array("actions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<MissionRandomActionEntry>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<MissionRandomActionEntry>(
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

/// DCB type: `MissionFlowAction_MultipleActions`
/// Inherits from: `MissionFlowActionBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct MissionFlowAction_MultipleActions {
    /// `actions` (StrongPointer (array))
    pub actions: Vec<MissionFlowActionBasePtr>,
}

impl Pooled for MissionFlowAction_MultipleActions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_flow_action_multiple_actions
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_flow_action_multiple_actions
    }
}

impl<'a> Extract<'a> for MissionFlowAction_MultipleActions {
    const TYPE_NAME: &'static str = "MissionFlowAction_MultipleActions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            actions: inst
                .get_array("actions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(MissionFlowActionBasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionPropertyValue_Locations`
/// Inherits from: `MissionPropertyValue_Location`
pub struct MissionPropertyValue_Locations {
    /// `matchConditions` (StrongPointer (array))
    pub match_conditions: Vec<BaseDataSetMatchConditionPtr>,
    /// `resourceTags` (Reference (array))
    pub resource_tags: Vec<CigGuid>,
    /// `minLocationsToFind` (Int32)
    pub min_locations_to_find: i32,
    /// `maxLocationsToFind` (Int32)
    pub max_locations_to_find: i32,
    /// `failIfMinAmountNotFound` (Boolean)
    pub fail_if_min_amount_not_found: bool,
    /// `logErrorOnSearchFail` (Boolean)
    #[deprecated(note = "absent from 5.0.0-ptu.13248297")]
    pub log_error_on_search_fail: bool,
}

impl Pooled for MissionPropertyValue_Locations {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_property_value_locations
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_property_value_locations
    }
}

impl<'a> Extract<'a> for MissionPropertyValue_Locations {
    const TYPE_NAME: &'static str = "MissionPropertyValue_Locations";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            match_conditions: inst
                .get_array("matchConditions")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(BaseDataSetMatchConditionPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            resource_tags: inst
                .get_array("resourceTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            min_locations_to_find: inst.get_i32("minLocationsToFind").unwrap_or_default(),
            max_locations_to_find: inst.get_i32("maxLocationsToFind").unwrap_or_default(),
            fail_if_min_amount_not_found: inst
                .get_bool("failIfMinAmountNotFound")
                .unwrap_or_default(),
            log_error_on_search_fail: inst.get_bool("logErrorOnSearchFail").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionPropertyValue_HaulingOrders`
/// Inherits from: `BaseMissionPropertyValue`
pub struct MissionPropertyValue_HaulingOrders {
    /// `haulingOrderContent` (StrongPointer (array))
    pub hauling_order_content: Vec<HaulingOrderContentBasePtr>,
}

impl Pooled for MissionPropertyValue_HaulingOrders {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_property_value_hauling_orders
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_property_value_hauling_orders
    }
}

impl<'a> Extract<'a> for MissionPropertyValue_HaulingOrders {
    const TYPE_NAME: &'static str = "MissionPropertyValue_HaulingOrders";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hauling_order_content: inst
                .get_array("haulingOrderContent")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(HaulingOrderContentBasePtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SpawnClosetSearchOptions_Tags`
/// Inherits from: `SpawnClosetSearchOptionsBase`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct SpawnClosetSearchOptions_Tags {
    /// `closetPositiveTags` (Class)
    pub closet_positive_tags: Option<Handle<TagList>>,
    /// `closetNegativeTags` (Class)
    pub closet_negative_tags: Option<Handle<TagList>>,
}

impl Pooled for SpawnClosetSearchOptions_Tags {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.spawn_closet_search_options_tags
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.spawn_closet_search_options_tags
    }
}

impl<'a> Extract<'a> for SpawnClosetSearchOptions_Tags {
    const TYPE_NAME: &'static str = "SpawnClosetSearchOptions_Tags";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            closet_positive_tags: match inst.get("closetPositiveTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            closet_negative_tags: match inst.get("closetNegativeTags") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TagList>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionPropertyValue_AutoSpawns`
/// Inherits from: `BaseMissionPropertyValue`
///
/// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
pub struct MissionPropertyValue_AutoSpawns {
    /// `autoSpawns` (Reference)
    pub auto_spawns: Option<CigGuid>,
    /// `searchOptions` (StrongPointer)
    pub search_options: Option<SpawnClosetSearchOptionsBasePtr>,
}

impl Pooled for MissionPropertyValue_AutoSpawns {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_property_value_auto_spawns
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_property_value_auto_spawns
    }
}

impl<'a> Extract<'a> for MissionPropertyValue_AutoSpawns {
    const TYPE_NAME: &'static str = "MissionPropertyValue_AutoSpawns";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            auto_spawns: inst
                .get("autoSpawns")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            search_options: match inst.get("searchOptions") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(SpawnClosetSearchOptionsBasePtr::from_ref(b, r))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `ObjectiveHandler_NearLocation`
/// Inherits from: `ObjectiveHandler_WithModule`
pub struct ObjectiveHandler_NearLocation {
    /// `moduleDeclaration` (Reference)
    pub module_declaration: Option<CigGuid>,
    /// `module` (String)
    pub module: String,
    /// `moduleHierarchy` (Reference)
    pub module_hierarchy: Option<CigGuid>,
    /// `disableTravelObjectives` (Boolean)
    pub disable_travel_objectives: bool,
    /// `disableReturnObjectives` (Boolean)
    pub disable_return_objectives: bool,
    /// `travelRadiusKM` (Single)
    pub travel_radius_km: f32,
    /// `allPlayersLeftGracePeriod` (Single)
    pub all_players_left_grace_period: f32,
    /// `travelObjectiveInfo` (Class)
    pub travel_objective_info: Option<Handle<ObjectiveDisplayInfo>>,
    /// `returnObjectiveInfo` (Class)
    pub return_objective_info: Option<Handle<ObjectiveDisplayInfo>>,
    /// `navPointSpawnInfo` (StrongPointer)
    pub nav_point_spawn_info: Option<Handle<NavPointSpawnInformation>>,
    /// `location` (WeakPointer)
    pub location: Option<ObjectivePropertyBasePtr>,
    /// `maxDistance` (Single)
    pub max_distance: f32,
    /// `minDistance` (Single)
    pub min_distance: f32,
}

impl Pooled for ObjectiveHandler_NearLocation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.objective_handler_near_location
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.objective_handler_near_location
    }
}

impl<'a> Extract<'a> for ObjectiveHandler_NearLocation {
    const TYPE_NAME: &'static str = "ObjectiveHandler_NearLocation";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            module_declaration: inst
                .get("moduleDeclaration")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            module: inst.get_str("module").map(String::from).unwrap_or_default(),
            module_hierarchy: inst
                .get("moduleHierarchy")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            disable_travel_objectives: inst.get_bool("disableTravelObjectives").unwrap_or_default(),
            disable_return_objectives: inst.get_bool("disableReturnObjectives").unwrap_or_default(),
            travel_radius_km: inst.get_f32("travelRadiusKM").unwrap_or_default(),
            all_players_left_grace_period: inst
                .get_f32("allPlayersLeftGracePeriod")
                .unwrap_or_default(),
            travel_objective_info: match inst.get("travelObjectiveInfo") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ObjectiveDisplayInfo>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            return_objective_info: match inst.get("returnObjectiveInfo") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ObjectiveDisplayInfo>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            nav_point_spawn_info: match inst.get("navPointSpawnInfo") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<NavPointSpawnInformation>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            location: match inst.get("location") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            min_distance: inst.get_f32("minDistance").unwrap_or_default(),
        }
    }
}

/// DCB type: `ObjectiveHandler_TransitToInstance`
/// Inherits from: `ObjectiveHandlerBase`
pub struct ObjectiveHandler_TransitToInstance {
    /// `location` (WeakPointer)
    pub location: Option<ObjectivePropertyBasePtr>,
    /// `ocTagsToSearchInstanceEnter` (Reference (array))
    pub oc_tags_to_search_instance_enter: Vec<CigGuid>,
    /// `completePhaseOnPlayerEnterInstance` (Boolean)
    pub complete_phase_on_player_enter_instance: bool,
    /// `instanceTagIdentifier` (Reference)
    pub instance_tag_identifier: Option<CigGuid>,
    /// `instanceEnterTravelRadiusKM` (Single)
    pub instance_enter_travel_radius_km: f32,
    /// `transitToInstanceObjectiveMarker` (Locale)
    pub transit_to_instance_objective_marker: LocaleKey,
    /// `instanceHUDObjectiveText` (Locale)
    pub instance_hudobjective_text: LocaleKey,
    /// `travelObjectiveInfo` (Class)
    pub travel_objective_info: Option<Handle<ObjectiveDisplayInfo>>,
}

impl Pooled for ObjectiveHandler_TransitToInstance {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.objective_handler_transit_to_instance
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.objective_handler_transit_to_instance
    }
}

impl<'a> Extract<'a> for ObjectiveHandler_TransitToInstance {
    const TYPE_NAME: &'static str = "ObjectiveHandler_TransitToInstance";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location: match inst.get("location") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(ObjectivePropertyBasePtr::from_ref(b, r))
                }
                _ => None,
            },
            oc_tags_to_search_instance_enter: inst
                .get_array("ocTagsToSearchInstanceEnter")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            complete_phase_on_player_enter_instance: inst
                .get_bool("completePhaseOnPlayerEnterInstance")
                .unwrap_or_default(),
            instance_tag_identifier: inst
                .get("instanceTagIdentifier")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            instance_enter_travel_radius_km: inst
                .get_f32("instanceEnterTravelRadiusKM")
                .unwrap_or_default(),
            transit_to_instance_objective_marker: inst
                .get_str("transitToInstanceObjectiveMarker")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            instance_hudobjective_text: inst
                .get_str("instanceHUDObjectiveText")
                .map(LocaleKey::from)
                .unwrap_or_default(),
            travel_objective_info: match inst.get("travelObjectiveInfo") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ObjectiveDisplayInfo>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `ContractGeneratorHandler_PVPBountyDef`
/// Inherits from: `ContractGeneratorHandlerBase`
pub struct ContractGeneratorHandler_PVPBountyDef {
    /// `notForRelease` (Boolean)
    pub not_for_release: bool,
    /// `workInProgress` (Boolean)
    pub work_in_progress: bool,
    /// `debugName` (String)
    pub debug_name: String,
    /// `localGeneratorParams` (StrongPointer)
    ///
    /// New in 5.0.0-ptu.13248297; absent from 4.10.3-live.13220821.
    pub local_generator_params: Option<Handle<LocalContractGeneratorParams>>,
    /// `required_active_scenarios` (Reference (array))
    pub required_active_scenarios: Vec<CigGuid>,
    /// `defaultAvailability` (Class)
    pub default_availability: Option<Handle<ContractAvailability>>,
    /// `contractParams` (Class)
    pub contract_params: Option<Handle<ContractParamOverrides>>,
    /// `PVPBountyContract` (Class (array))
    pub pvpbounty_contract: Vec<Handle<Contract>>,
    /// `escapedConvicts` (Boolean)
    pub escaped_convicts: bool,
}

impl Pooled for ContractGeneratorHandler_PVPBountyDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.contract_generator_handler_pvpbounty_def
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.contract_generator_handler_pvpbounty_def
    }
}

impl<'a> Extract<'a> for ContractGeneratorHandler_PVPBountyDef {
    const TYPE_NAME: &'static str = "ContractGeneratorHandler_PVPBountyDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst
                .get_str("debugName")
                .map(String::from)
                .unwrap_or_default(),
            local_generator_params: match inst.get("localGeneratorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => {
                    Some(b.alloc_nested::<LocalContractGeneratorParams>(
                        b.db.instance(r.struct_index, r.instance_index),
                        true,
                    ))
                }
                _ => None,
            },
            required_active_scenarios: inst
                .get_array("required_active_scenarios")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_availability: match inst.get("defaultAvailability") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractAvailability>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            contract_params: match inst.get("contractParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<ContractParamOverrides>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            pvpbounty_contract: inst
                .get_array("PVPBountyContract")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Contract>(
                            Instance::from_inline_data(b.db, struct_index, data),
                            false,
                        )),
                        Value::ClassRef(r) => Some(b.alloc_nested::<Contract>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            escaped_convicts: inst.get_bool("escapedConvicts").unwrap_or_default(),
        }
    }
}

/// DCB type: `MissionModifier_TemporaryBlueprint`
/// Inherits from: `BaseMissionModifier`
pub struct MissionModifier_TemporaryBlueprint {
    /// `modifierName` (String)
    pub modifier_name: String,
    /// `enabled` (Boolean)
    pub enabled: bool,
    /// `blueprintPool` (Reference)
    pub blueprint_pool: Option<CigGuid>,
}

impl Pooled for MissionModifier_TemporaryBlueprint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.mission_modifier_temporary_blueprint
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.mission_modifier_temporary_blueprint
    }
}

impl<'a> Extract<'a> for MissionModifier_TemporaryBlueprint {
    const TYPE_NAME: &'static str = "MissionModifier_TemporaryBlueprint";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            modifier_name: inst
                .get_str("modifierName")
                .map(String::from)
                .unwrap_or_default(),
            enabled: inst.get_bool("enabled").unwrap_or_default(),
            blueprint_pool: inst
                .get("blueprintPool")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `SPVPBountyContractGenerators`
pub struct SPVPBountyContractGenerators {
    /// `locationAvailable` (Reference)
    pub location_available: Option<CigGuid>,
    /// `contractGenerator` (Reference)
    pub contract_generator: Option<CigGuid>,
}

impl Pooled for SPVPBountyContractGenerators {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.spvpbounty_contract_generators
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.spvpbounty_contract_generators
    }
}

impl<'a> Extract<'a> for SPVPBountyContractGenerators {
    const TYPE_NAME: &'static str = "SPVPBountyContractGenerators";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_available: inst
                .get("locationAvailable")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            contract_generator: inst
                .get("contractGenerator")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `OneTimeComms`
pub struct OneTimeComms {
    /// `locationAvailable` (Reference)
    pub location_available: Option<CigGuid>,
    /// `locationExcluded` (Reference (array))
    pub location_excluded: Vec<CigGuid>,
    /// `prerequisitesTags` (Reference (array))
    pub prerequisites_tags: Vec<CigGuid>,
    /// `tag` (Reference)
    pub tag: Option<CigGuid>,
    /// `delayBeforePlaying` (Single)
    pub delay_before_playing: f32,
    /// `comms` (Reference (array))
    pub comms: Vec<CigGuid>,
}

impl Pooled for OneTimeComms {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.one_time_comms
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.one_time_comms
    }
}

impl<'a> Extract<'a> for OneTimeComms {
    const TYPE_NAME: &'static str = "OneTimeComms";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            location_available: inst
                .get("locationAvailable")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            location_excluded: inst
                .get_array("locationExcluded")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            prerequisites_tags: inst
                .get_array("prerequisitesTags")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
            tag: inst
                .get("tag")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            delay_before_playing: inst.get_f32("delayBeforePlaying").unwrap_or_default(),
            comms: inst
                .get_array("comms")
                .map(|arr| {
                    arr.filter_map(|v| {
                        if let Value::Reference(Some(r)) = v {
                            Some(r.guid)
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `GlobalMissionSettings`
pub struct GlobalMissionSettings {
    /// `locationValidation` (StrongPointer (array))
    pub location_validation: Vec<MissionLocationValidationPtr>,
    /// `defaultJurisdictionForPlayerCrimeStats` (Reference)
    pub default_jurisdiction_for_player_crime_stats: Option<CigGuid>,
    /// `PVPBountyContractGenerators` (Class (array))
    pub pvpbounty_contract_generators: Vec<Handle<SPVPBountyContractGenerators>>,
    /// `oneTimeComms` (Class (array))
    pub one_time_comms: Vec<Handle<OneTimeComms>>,
    /// `brokeredInstanceTagIdentifier` (Reference)
    pub brokered_instance_tag_identifier: Option<CigGuid>,
}

impl Pooled for GlobalMissionSettings {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.contracts.global_mission_settings
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.contracts.global_mission_settings
    }
}

impl<'a> Extract<'a> for GlobalMissionSettings {
    const TYPE_NAME: &'static str = "GlobalMissionSettings";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            location_validation: inst
                .get_array("locationValidation")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::StrongPointer(Some(r)) | Value::WeakPointer(Some(r)) => {
                            Some(MissionLocationValidationPtr::from_ref(b, r))
                        }
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            default_jurisdiction_for_player_crime_stats: inst
                .get("defaultJurisdictionForPlayerCrimeStats")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
            pvpbounty_contract_generators: inst
                .get_array("PVPBountyContractGenerators")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<SPVPBountyContractGenerators>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<SPVPBountyContractGenerators>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            one_time_comms: inst
                .get_array("oneTimeComms")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<OneTimeComms>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<OneTimeComms>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
            brokered_instance_tag_identifier: inst
                .get("brokeredInstanceTagIdentifier")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}
