// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `servicebeacon`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, LocaleKey, Pooled};

use super::super::*;

/// DCB type: `ContractGeneratorHandler_ServiceBeacon`
/// Inherits from: `ContractGeneratorHandlerBase`
pub struct ContractGeneratorHandler_ServiceBeacon {
    /// `notForRelease` (Boolean)
    pub not_for_release: bool,
    /// `workInProgress` (Boolean)
    pub work_in_progress: bool,
    /// `debugName` (String)
    pub debug_name: String,
    /// `required_active_scenarios` (Reference (array))
    pub required_active_scenarios: Vec<CigGuid>,
    /// `defaultAvailability` (Class)
    pub default_availability: Option<Handle<ContractAvailability>>,
    /// `contractParams` (Class)
    pub contract_params: Option<Handle<ContractParamOverrides>>,
    /// `serviceBeaconContracts` (Class (array))
    pub service_beacon_contracts: Vec<Handle<Contract>>,
}

impl Pooled for ContractGeneratorHandler_ServiceBeacon {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.contract_generator_handler_service_beacon }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.contract_generator_handler_service_beacon }
}

impl<'a> Extract<'a> for ContractGeneratorHandler_ServiceBeacon {
    const TYPE_NAME: &'static str = "ContractGeneratorHandler_ServiceBeacon";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            not_for_release: inst.get_bool("notForRelease").unwrap_or_default(),
            work_in_progress: inst.get_bool("workInProgress").unwrap_or_default(),
            debug_name: inst.get_str("debugName").map(String::from).unwrap_or_default(),
            required_active_scenarios: inst.get_array("required_active_scenarios")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            default_availability: match inst.get("defaultAvailability") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContractAvailability>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            contract_params: match inst.get("contractParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ContractParamOverrides>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            service_beacon_contracts: inst.get_array("serviceBeaconContracts")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<Contract>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<Contract>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ContractClass_ServiceBeacon`
/// Inherits from: `ContractClassBase`
pub struct ContractClass_ServiceBeacon {
    /// `params` (StrongPointer)
    pub params: Option<ServiceBeaconBaseTemplateParamsPtr>,
}

impl Pooled for ContractClass_ServiceBeacon {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.contract_class_service_beacon }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.contract_class_service_beacon }
}

impl<'a> Extract<'a> for ContractClass_ServiceBeacon {
    const TYPE_NAME: &'static str = "ContractClass_ServiceBeacon";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params: match inst.get("params") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(ServiceBeaconBaseTemplateParamsPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MissionPropertyValue_StarMapLocation`
/// Inherits from: `BaseMissionPropertyValue`
pub struct MissionPropertyValue_StarMapLocation {
}

impl Pooled for MissionPropertyValue_StarMapLocation {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.mission_property_value_star_map_location }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.mission_property_value_star_map_location }
}

impl<'a> Extract<'a> for MissionPropertyValue_StarMapLocation {
    const TYPE_NAME: &'static str = "MissionPropertyValue_StarMapLocation";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `BeaconsContracts`
pub struct BeaconsContracts {
    /// `serviceBeacons` (Reference (array))
    pub service_beacons: Vec<CigGuid>,
    /// `serviceBeaconContractGenerator` (Reference)
    pub service_beacon_contract_generator: Option<CigGuid>,
}

impl Pooled for BeaconsContracts {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.beacons_contracts }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.beacons_contracts }
}

impl<'a> Extract<'a> for BeaconsContracts {
    const TYPE_NAME: &'static str = "BeaconsContracts";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            service_beacons: inst.get_array("serviceBeacons")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
            service_beacon_contract_generator: inst.get("serviceBeaconContractGenerator").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ServiceBeaconBaseTemplateParams`
pub struct ServiceBeaconBaseTemplateParams {
    /// `objectiveTitle` (Locale)
    pub objective_title: LocaleKey,
    /// `objectiveDescription` (Locale)
    pub objective_description: LocaleKey,
    /// `canBeCreatedInMobiglas` (Boolean)
    pub can_be_created_in_mobiglas: bool,
    /// `serviceBeaconType` (EnumChoice)
    pub service_beacon_type: EServiceBeaconType,
    /// `serviceBeaconName` (Locale)
    pub service_beacon_name: LocaleKey,
    /// `beaconTaxPercentage` (Int32)
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `npcCreatorParams` (StrongPointer)
    pub npc_creator_params: Option<SServiceBeaconCreatorParamsBasePtr>,
    /// `beaconModuleDeclaration` (Reference)
    pub beacon_module_declaration: Option<CigGuid>,
}

impl Pooled for ServiceBeaconBaseTemplateParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.service_beacon_base_template_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.service_beacon_base_template_params }
}

impl<'a> Extract<'a> for ServiceBeaconBaseTemplateParams {
    const TYPE_NAME: &'static str = "ServiceBeaconBaseTemplateParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            objective_title: inst.get_str("objectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            objective_description: inst.get_str("objectiveDescription").map(LocaleKey::from).unwrap_or_default(),
            can_be_created_in_mobiglas: inst.get_bool("canBeCreatedInMobiglas").unwrap_or_default(),
            service_beacon_type: EServiceBeaconType::from_dcb_str(inst.get_str("serviceBeaconType").unwrap_or("")),
            service_beacon_name: inst.get_str("serviceBeaconName").map(LocaleKey::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SServiceBeaconCreatorParamsBasePtr::from_ref(b, r)),
                _ => None,
            },
            beacon_module_declaration: inst.get("beaconModuleDeclaration").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `PersonalTransportBeaconParams`
/// Inherits from: `ServiceBeaconBaseTemplateParams`
pub struct PersonalTransportBeaconParams {
    /// `objectiveTitle` (Locale)
    pub objective_title: LocaleKey,
    /// `objectiveDescription` (Locale)
    pub objective_description: LocaleKey,
    /// `canBeCreatedInMobiglas` (Boolean)
    pub can_be_created_in_mobiglas: bool,
    /// `serviceBeaconType` (EnumChoice)
    pub service_beacon_type: EServiceBeaconType,
    /// `serviceBeaconName` (Locale)
    pub service_beacon_name: LocaleKey,
    /// `beaconTaxPercentage` (Int32)
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `npcCreatorParams` (StrongPointer)
    pub npc_creator_params: Option<SServiceBeaconCreatorParamsBasePtr>,
    /// `beaconModuleDeclaration` (Reference)
    pub beacon_module_declaration: Option<CigGuid>,
    /// `pickUpObjectiveTitle` (Locale)
    pub pick_up_objective_title: LocaleKey,
    /// `pickUpObjectiveDescription` (Locale)
    pub pick_up_objective_description: LocaleKey,
    /// `deliveryObjectiveTitle` (Locale)
    pub delivery_objective_title: LocaleKey,
    /// `deliveryObjectiveDescription` (Locale)
    pub delivery_objective_description: LocaleKey,
}

impl Pooled for PersonalTransportBeaconParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.personal_transport_beacon_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.personal_transport_beacon_params }
}

impl<'a> Extract<'a> for PersonalTransportBeaconParams {
    const TYPE_NAME: &'static str = "PersonalTransportBeaconParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            objective_title: inst.get_str("objectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            objective_description: inst.get_str("objectiveDescription").map(LocaleKey::from).unwrap_or_default(),
            can_be_created_in_mobiglas: inst.get_bool("canBeCreatedInMobiglas").unwrap_or_default(),
            service_beacon_type: EServiceBeaconType::from_dcb_str(inst.get_str("serviceBeaconType").unwrap_or("")),
            service_beacon_name: inst.get_str("serviceBeaconName").map(LocaleKey::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SServiceBeaconCreatorParamsBasePtr::from_ref(b, r)),
                _ => None,
            },
            beacon_module_declaration: inst.get("beaconModuleDeclaration").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            pick_up_objective_title: inst.get_str("pickUpObjectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            pick_up_objective_description: inst.get_str("pickUpObjectiveDescription").map(LocaleKey::from).unwrap_or_default(),
            delivery_objective_title: inst.get_str("deliveryObjectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            delivery_objective_description: inst.get_str("deliveryObjectiveDescription").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SServiceBeaconNotificationOverride`
pub struct SServiceBeaconNotificationOverride {
    /// `message` (Locale)
    pub message: LocaleKey,
}

impl Pooled for SServiceBeaconNotificationOverride {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.sservice_beacon_notification_override }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.sservice_beacon_notification_override }
}

impl<'a> Extract<'a> for SServiceBeaconNotificationOverride {
    const TYPE_NAME: &'static str = "SServiceBeaconNotificationOverride";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            message: inst.get_str("message").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SServiceBeaconCreatorParams`
/// Inherits from: `SServiceBeaconCreatorParamsBase`
pub struct SServiceBeaconCreatorParams {
    /// `missionEntry` (Reference)
    pub mission_entry: Option<CigGuid>,
}

impl Pooled for SServiceBeaconCreatorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.sservice_beacon_creator_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.sservice_beacon_creator_params }
}

impl<'a> Extract<'a> for SServiceBeaconCreatorParams {
    const TYPE_NAME: &'static str = "SServiceBeaconCreatorParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            mission_entry: inst.get("missionEntry").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SServiceBeaconDifficultyEntry`
pub struct SServiceBeaconDifficultyEntry {
    /// `difficulty` (Int32)
    pub difficulty: i32,
    /// `missionEntry` (Reference)
    pub mission_entry: Option<CigGuid>,
    /// `beaconDetectedNotificationOverride` (StrongPointer)
    pub beacon_detected_notification_override: Option<Handle<SServiceBeaconNotificationOverride>>,
}

impl Pooled for SServiceBeaconDifficultyEntry {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.sservice_beacon_difficulty_entry }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.sservice_beacon_difficulty_entry }
}

impl<'a> Extract<'a> for SServiceBeaconDifficultyEntry {
    const TYPE_NAME: &'static str = "SServiceBeaconDifficultyEntry";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            difficulty: inst.get_i32("difficulty").unwrap_or_default(),
            mission_entry: inst.get("missionEntry").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            beacon_detected_notification_override: match inst.get("beaconDetectedNotificationOverride") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconNotificationOverride>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SServiceBeaconCreatorParamsWithDifficulty`
/// Inherits from: `SServiceBeaconCreatorParamsBase`
pub struct SServiceBeaconCreatorParamsWithDifficulty {
    /// `missionEntriesByDifficulty` (Class (array))
    pub mission_entries_by_difficulty: Vec<Handle<SServiceBeaconDifficultyEntry>>,
}

impl Pooled for SServiceBeaconCreatorParamsWithDifficulty {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.sservice_beacon_creator_params_with_difficulty }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.sservice_beacon_creator_params_with_difficulty }
}

impl<'a> Extract<'a> for SServiceBeaconCreatorParamsWithDifficulty {
    const TYPE_NAME: &'static str = "SServiceBeaconCreatorParamsWithDifficulty";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            mission_entries_by_difficulty: inst.get_array("missionEntriesByDifficulty")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SServiceBeaconDifficultyEntry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r) => Some(b.alloc_nested::<SServiceBeaconDifficultyEntry>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `ServiceBeaconBaseParams`
pub struct ServiceBeaconBaseParams {
    /// `serviceBeaconType` (EnumChoice)
    pub service_beacon_type: EServiceBeaconType,
    /// `serviceBeaconName` (Locale)
    pub service_beacon_name: LocaleKey,
    /// `beaconTaxPercentage` (Int32)
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `playerCreatorParams` (StrongPointer)
    pub player_creator_params: Option<Handle<SServiceBeaconCreatorParams>>,
    /// `npcCreatorParams` (StrongPointer)
    pub npc_creator_params: Option<SServiceBeaconCreatorParamsBasePtr>,
}

impl Pooled for ServiceBeaconBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.service_beacon_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.service_beacon_base_params }
}

impl<'a> Extract<'a> for ServiceBeaconBaseParams {
    const TYPE_NAME: &'static str = "ServiceBeaconBaseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            service_beacon_type: EServiceBeaconType::from_dcb_str(inst.get_str("serviceBeaconType").unwrap_or("")),
            service_beacon_name: inst.get_str("serviceBeaconName").map(LocaleKey::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_creator_params: match inst.get("playerCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SServiceBeaconCreatorParamsBasePtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `PersonalTransportParams`
/// Inherits from: `ServiceBeaconBaseParams`
pub struct PersonalTransportParams {
    /// `serviceBeaconType` (EnumChoice)
    pub service_beacon_type: EServiceBeaconType,
    /// `serviceBeaconName` (Locale)
    pub service_beacon_name: LocaleKey,
    /// `beaconTaxPercentage` (Int32)
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `playerCreatorParams` (StrongPointer)
    pub player_creator_params: Option<Handle<SServiceBeaconCreatorParams>>,
    /// `npcCreatorParams` (StrongPointer)
    pub npc_creator_params: Option<SServiceBeaconCreatorParamsBasePtr>,
    /// `pickUpObjectiveTitle` (Locale)
    pub pick_up_objective_title: LocaleKey,
    /// `pickUpObjectiveDescription` (Locale)
    pub pick_up_objective_description: LocaleKey,
    /// `deliveryObjectiveTitle` (Locale)
    pub delivery_objective_title: LocaleKey,
    /// `deliveryObjectiveDescription` (Locale)
    pub delivery_objective_description: LocaleKey,
}

impl Pooled for PersonalTransportParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.personal_transport_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.personal_transport_params }
}

impl<'a> Extract<'a> for PersonalTransportParams {
    const TYPE_NAME: &'static str = "PersonalTransportParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            service_beacon_type: EServiceBeaconType::from_dcb_str(inst.get_str("serviceBeaconType").unwrap_or("")),
            service_beacon_name: inst.get_str("serviceBeaconName").map(LocaleKey::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_creator_params: match inst.get("playerCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SServiceBeaconCreatorParamsBasePtr::from_ref(b, r)),
                _ => None,
            },
            pick_up_objective_title: inst.get_str("pickUpObjectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            pick_up_objective_description: inst.get_str("pickUpObjectiveDescription").map(LocaleKey::from).unwrap_or_default(),
            delivery_objective_title: inst.get_str("deliveryObjectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            delivery_objective_description: inst.get_str("deliveryObjectiveDescription").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `EscortParams`
/// Inherits from: `ServiceBeaconBaseParams`
pub struct EscortParams {
    /// `serviceBeaconType` (EnumChoice)
    pub service_beacon_type: EServiceBeaconType,
    /// `serviceBeaconName` (Locale)
    pub service_beacon_name: LocaleKey,
    /// `beaconTaxPercentage` (Int32)
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `playerCreatorParams` (StrongPointer)
    pub player_creator_params: Option<Handle<SServiceBeaconCreatorParams>>,
    /// `npcCreatorParams` (StrongPointer)
    pub npc_creator_params: Option<SServiceBeaconCreatorParamsBasePtr>,
    /// `objectiveTitle` (Locale)
    pub objective_title: LocaleKey,
    /// `objectiveDescription` (Locale)
    pub objective_description: LocaleKey,
}

impl Pooled for EscortParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.escort_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.escort_params }
}

impl<'a> Extract<'a> for EscortParams {
    const TYPE_NAME: &'static str = "EscortParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            service_beacon_type: EServiceBeaconType::from_dcb_str(inst.get_str("serviceBeaconType").unwrap_or("")),
            service_beacon_name: inst.get_str("serviceBeaconName").map(LocaleKey::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_creator_params: match inst.get("playerCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SServiceBeaconCreatorParamsBasePtr::from_ref(b, r)),
                _ => None,
            },
            objective_title: inst.get_str("objectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            objective_description: inst.get_str("objectiveDescription").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `RefuelParams`
/// Inherits from: `ServiceBeaconBaseParams`
pub struct RefuelParams {
    /// `serviceBeaconType` (EnumChoice)
    pub service_beacon_type: EServiceBeaconType,
    /// `serviceBeaconName` (Locale)
    pub service_beacon_name: LocaleKey,
    /// `beaconTaxPercentage` (Int32)
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `playerCreatorParams` (StrongPointer)
    pub player_creator_params: Option<Handle<SServiceBeaconCreatorParams>>,
    /// `npcCreatorParams` (StrongPointer)
    pub npc_creator_params: Option<SServiceBeaconCreatorParamsBasePtr>,
    /// `objectiveTitle` (Locale)
    pub objective_title: LocaleKey,
    /// `objectiveDescription` (Locale)
    pub objective_description: LocaleKey,
}

impl Pooled for RefuelParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.refuel_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.refuel_params }
}

impl<'a> Extract<'a> for RefuelParams {
    const TYPE_NAME: &'static str = "RefuelParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            service_beacon_type: EServiceBeaconType::from_dcb_str(inst.get_str("serviceBeaconType").unwrap_or("")),
            service_beacon_name: inst.get_str("serviceBeaconName").map(LocaleKey::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_creator_params: match inst.get("playerCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SServiceBeaconCreatorParamsBasePtr::from_ref(b, r)),
                _ => None,
            },
            objective_title: inst.get_str("objectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            objective_description: inst.get_str("objectiveDescription").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `CombatAssistanceParams`
/// Inherits from: `ServiceBeaconBaseParams`
pub struct CombatAssistanceParams {
    /// `serviceBeaconType` (EnumChoice)
    pub service_beacon_type: EServiceBeaconType,
    /// `serviceBeaconName` (Locale)
    pub service_beacon_name: LocaleKey,
    /// `beaconTaxPercentage` (Int32)
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `playerCreatorParams` (StrongPointer)
    pub player_creator_params: Option<Handle<SServiceBeaconCreatorParams>>,
    /// `npcCreatorParams` (StrongPointer)
    pub npc_creator_params: Option<SServiceBeaconCreatorParamsBasePtr>,
    /// `objectiveTitle` (Locale)
    pub objective_title: LocaleKey,
    /// `objectiveDescription` (Locale)
    pub objective_description: LocaleKey,
}

impl Pooled for CombatAssistanceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.combat_assistance_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.combat_assistance_params }
}

impl<'a> Extract<'a> for CombatAssistanceParams {
    const TYPE_NAME: &'static str = "CombatAssistanceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            service_beacon_type: EServiceBeaconType::from_dcb_str(inst.get_str("serviceBeaconType").unwrap_or("")),
            service_beacon_name: inst.get_str("serviceBeaconName").map(LocaleKey::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_creator_params: match inst.get("playerCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SServiceBeaconCreatorParamsBasePtr::from_ref(b, r)),
                _ => None,
            },
            objective_title: inst.get_str("objectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            objective_description: inst.get_str("objectiveDescription").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ReviveParams`
/// Inherits from: `ServiceBeaconBaseParams`
pub struct ReviveParams {
    /// `serviceBeaconType` (EnumChoice)
    pub service_beacon_type: EServiceBeaconType,
    /// `serviceBeaconName` (Locale)
    pub service_beacon_name: LocaleKey,
    /// `beaconTaxPercentage` (Int32)
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `playerCreatorParams` (StrongPointer)
    pub player_creator_params: Option<Handle<SServiceBeaconCreatorParams>>,
    /// `npcCreatorParams` (StrongPointer)
    pub npc_creator_params: Option<SServiceBeaconCreatorParamsBasePtr>,
    /// `objectiveTitle` (Locale)
    pub objective_title: LocaleKey,
    /// `objectiveDescription` (Locale)
    pub objective_description: LocaleKey,
}

impl Pooled for ReviveParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.revive_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.revive_params }
}

impl<'a> Extract<'a> for ReviveParams {
    const TYPE_NAME: &'static str = "ReviveParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            service_beacon_type: EServiceBeaconType::from_dcb_str(inst.get_str("serviceBeaconType").unwrap_or("")),
            service_beacon_name: inst.get_str("serviceBeaconName").map(LocaleKey::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_creator_params: match inst.get("playerCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SServiceBeaconCreatorParamsBasePtr::from_ref(b, r)),
                _ => None,
            },
            objective_title: inst.get_str("objectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            objective_description: inst.get_str("objectiveDescription").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `HealParams`
/// Inherits from: `ServiceBeaconBaseParams`
pub struct HealParams {
    /// `serviceBeaconType` (EnumChoice)
    pub service_beacon_type: EServiceBeaconType,
    /// `serviceBeaconName` (Locale)
    pub service_beacon_name: LocaleKey,
    /// `beaconTaxPercentage` (Int32)
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `playerCreatorParams` (StrongPointer)
    pub player_creator_params: Option<Handle<SServiceBeaconCreatorParams>>,
    /// `npcCreatorParams` (StrongPointer)
    pub npc_creator_params: Option<SServiceBeaconCreatorParamsBasePtr>,
    /// `objectiveTitle` (Locale)
    pub objective_title: LocaleKey,
    /// `objectiveDescription` (Locale)
    pub objective_description: LocaleKey,
}

impl Pooled for HealParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.heal_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.heal_params }
}

impl<'a> Extract<'a> for HealParams {
    const TYPE_NAME: &'static str = "HealParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            service_beacon_type: EServiceBeaconType::from_dcb_str(inst.get_str("serviceBeaconType").unwrap_or("")),
            service_beacon_name: inst.get_str("serviceBeaconName").map(LocaleKey::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            player_creator_params: match inst.get("playerCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(SServiceBeaconCreatorParamsBasePtr::from_ref(b, r)),
                _ => None,
            },
            objective_title: inst.get_str("objectiveTitle").map(LocaleKey::from).unwrap_or_default(),
            objective_description: inst.get_str("objectiveDescription").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `ServiceBeaconParams`
pub struct ServiceBeaconParams {
    /// `params` (StrongPointer)
    pub params: Option<ServiceBeaconBaseParamsPtr>,
}

impl Pooled for ServiceBeaconParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.service_beacon_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.service_beacon_params }
}

impl<'a> Extract<'a> for ServiceBeaconParams {
    const TYPE_NAME: &'static str = "ServiceBeaconParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            params: match inst.get("params") {
                Some(Value::StrongPointer(Some(r))) | Some(Value::WeakPointer(Some(r))) => Some(ServiceBeaconBaseParamsPtr::from_ref(b, r)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ServiceBeaconNotificationParams`
pub struct ServiceBeaconNotificationParams {
    /// `message` (Locale)
    pub message: LocaleKey,
    /// `screenTimer` (Single)
    pub screen_timer: f32,
    /// `hurryScreenTimer` (Single)
    pub hurry_screen_timer: f32,
    /// `blocking` (Boolean)
    pub blocking: bool,
    /// `dockNotificationParamsOverride` (Reference)
    pub dock_notification_params_override: Option<CigGuid>,
}

impl Pooled for ServiceBeaconNotificationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.service_beacon_notification_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.service_beacon_notification_params }
}

impl<'a> Extract<'a> for ServiceBeaconNotificationParams {
    const TYPE_NAME: &'static str = "ServiceBeaconNotificationParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            message: inst.get_str("message").map(LocaleKey::from).unwrap_or_default(),
            screen_timer: inst.get_f32("screenTimer").unwrap_or_default(),
            hurry_screen_timer: inst.get_f32("hurryScreenTimer").unwrap_or_default(),
            blocking: inst.get_bool("blocking").unwrap_or_default(),
            dock_notification_params_override: inst.get("dockNotificationParamsOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ServiceBeaconGlobalParams`
pub struct ServiceBeaconGlobalParams {
    /// `quantumTravelPointClass` (Reference)
    pub quantum_travel_point_class: Option<CigGuid>,
    /// `missionTypeRecord` (Reference)
    pub mission_type_record: Option<CigGuid>,
    /// `personalTransportDetectedNotification` (Class)
    pub personal_transport_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `combatAssistanceDetectedNotification` (Class)
    pub combat_assistance_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `escortDetectedNotification` (Class)
    pub escort_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `refuelDetectedNotification` (Class)
    pub refuel_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `reviveDetectedNotification` (Class)
    pub revive_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `healDetectedNotification` (Class)
    pub heal_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractAcceptedNotification` (Class)
    pub contract_accepted_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractCancelledNotification` (Class)
    pub contract_cancelled_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractNoLongerAvailableNotification` (Class)
    pub contract_no_longer_available_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractCompletedInitiatorNotification` (Class)
    pub contract_completed_initiator_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractCompletedProviderNotification` (Class)
    pub contract_completed_provider_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `providerWithinRangeNotification` (Class)
    pub provider_within_range_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `spoofedContractOfferedNotification` (Class)
    pub spoofed_contract_offered_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractProviderName` (Locale)
    pub contract_provider_name: LocaleKey,
    /// `vehicleLocationChosenForPersonalTransport` (Locale)
    pub vehicle_location_chosen_for_personal_transport: LocaleKey,
    /// `providerNameToken` (String)
    pub provider_name_token: String,
    /// `initiatorNameToken` (String)
    pub initiator_name_token: String,
    /// `selectedDestinationToken` (String)
    pub selected_destination_token: String,
    /// `contractTypeToken` (String)
    pub contract_type_token: String,
    /// `distanceToInitiatorToken` (String)
    pub distance_to_initiator_token: String,
    /// `initiatorLocationToken` (String)
    pub initiator_location_token: String,
    /// `paymentAmountToken` (String)
    pub payment_amount_token: String,
    /// `openSpaceLocationName` (Locale)
    pub open_space_location_name: LocaleKey,
    /// `allReputationsLabel` (Locale)
    pub all_reputations_label: LocaleKey,
    /// `oneStarReputationLabel` (Locale)
    pub one_star_reputation_label: LocaleKey,
    /// `twoStarReputationLabel` (Locale)
    pub two_star_reputation_label: LocaleKey,
    /// `threeStarReputationLabel` (Locale)
    pub three_star_reputation_label: LocaleKey,
    /// `fourStarReputationLabel` (Locale)
    pub four_star_reputation_label: LocaleKey,
    /// `fiveStarReputationLabel` (Locale)
    pub five_star_reputation_label: LocaleKey,
    /// `invalidTypeErrorMessage` (Locale)
    pub invalid_type_error_message: LocaleKey,
    /// `invalidReputationErrorMessage` (Locale)
    pub invalid_reputation_error_message: LocaleKey,
    /// `priceIsZeroErrorMessage` (Locale)
    pub price_is_zero_error_message: LocaleKey,
    /// `insufficientFundsErrorMessage` (Locale)
    pub insufficient_funds_error_message: LocaleKey,
    /// `invalidLocationSelectedErrorMessage` (Locale)
    pub invalid_location_selected_error_message: LocaleKey,
}

impl Pooled for ServiceBeaconGlobalParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.service_beacon_global_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.service_beacon_global_params }
}

impl<'a> Extract<'a> for ServiceBeaconGlobalParams {
    const TYPE_NAME: &'static str = "ServiceBeaconGlobalParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            quantum_travel_point_class: inst.get("quantumTravelPointClass").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            mission_type_record: inst.get("missionTypeRecord").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            personal_transport_detected_notification: match inst.get("personalTransportDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            combat_assistance_detected_notification: match inst.get("combatAssistanceDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            escort_detected_notification: match inst.get("escortDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            refuel_detected_notification: match inst.get("refuelDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            revive_detected_notification: match inst.get("reviveDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            heal_detected_notification: match inst.get("healDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            contract_accepted_notification: match inst.get("contractAcceptedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            contract_cancelled_notification: match inst.get("contractCancelledNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            contract_no_longer_available_notification: match inst.get("contractNoLongerAvailableNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            contract_completed_initiator_notification: match inst.get("contractCompletedInitiatorNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            contract_completed_provider_notification: match inst.get("contractCompletedProviderNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            provider_within_range_notification: match inst.get("providerWithinRangeNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            spoofed_contract_offered_notification: match inst.get("spoofedContractOfferedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                _ => None,
            },
            contract_provider_name: inst.get_str("contractProviderName").map(LocaleKey::from).unwrap_or_default(),
            vehicle_location_chosen_for_personal_transport: inst.get_str("vehicleLocationChosenForPersonalTransport").map(LocaleKey::from).unwrap_or_default(),
            provider_name_token: inst.get_str("providerNameToken").map(String::from).unwrap_or_default(),
            initiator_name_token: inst.get_str("initiatorNameToken").map(String::from).unwrap_or_default(),
            selected_destination_token: inst.get_str("selectedDestinationToken").map(String::from).unwrap_or_default(),
            contract_type_token: inst.get_str("contractTypeToken").map(String::from).unwrap_or_default(),
            distance_to_initiator_token: inst.get_str("distanceToInitiatorToken").map(String::from).unwrap_or_default(),
            initiator_location_token: inst.get_str("initiatorLocationToken").map(String::from).unwrap_or_default(),
            payment_amount_token: inst.get_str("paymentAmountToken").map(String::from).unwrap_or_default(),
            open_space_location_name: inst.get_str("openSpaceLocationName").map(LocaleKey::from).unwrap_or_default(),
            all_reputations_label: inst.get_str("allReputationsLabel").map(LocaleKey::from).unwrap_or_default(),
            one_star_reputation_label: inst.get_str("oneStarReputationLabel").map(LocaleKey::from).unwrap_or_default(),
            two_star_reputation_label: inst.get_str("twoStarReputationLabel").map(LocaleKey::from).unwrap_or_default(),
            three_star_reputation_label: inst.get_str("threeStarReputationLabel").map(LocaleKey::from).unwrap_or_default(),
            four_star_reputation_label: inst.get_str("fourStarReputationLabel").map(LocaleKey::from).unwrap_or_default(),
            five_star_reputation_label: inst.get_str("fiveStarReputationLabel").map(LocaleKey::from).unwrap_or_default(),
            invalid_type_error_message: inst.get_str("invalidTypeErrorMessage").map(LocaleKey::from).unwrap_or_default(),
            invalid_reputation_error_message: inst.get_str("invalidReputationErrorMessage").map(LocaleKey::from).unwrap_or_default(),
            price_is_zero_error_message: inst.get_str("priceIsZeroErrorMessage").map(LocaleKey::from).unwrap_or_default(),
            insufficient_funds_error_message: inst.get_str("insufficientFundsErrorMessage").map(LocaleKey::from).unwrap_or_default(),
            invalid_location_selected_error_message: inst.get_str("invalidLocationSelectedErrorMessage").map(LocaleKey::from).unwrap_or_default(),
        }
    }
}

