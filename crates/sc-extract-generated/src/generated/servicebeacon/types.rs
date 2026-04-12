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

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `MissionPropertyValue_AIName`
/// Inherits from: `BaseMissionPropertyValue`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionPropertyValue_AIName {
    /// `randomName` (Boolean)
    #[serde(default)]
    pub random_name: bool,
    /// `randomLastName` (Boolean)
    #[serde(default)]
    pub random_last_name: bool,
    /// `randomNickName` (Boolean)
    #[serde(default)]
    pub random_nick_name: bool,
    /// `characterGivenName` (Locale)
    #[serde(default)]
    pub character_given_name: String,
    /// `characterGivenLastName` (Locale)
    #[serde(default)]
    pub character_given_last_name: String,
    /// `characterGivenNickName` (Locale)
    #[serde(default)]
    pub character_given_nick_name: String,
    /// `characterNameData` (Reference)
    #[serde(default)]
    pub character_name_data: Option<CigGuid>,
    /// `chanceOfNickName` (Single)
    #[serde(default)]
    pub chance_of_nick_name: f32,
}

impl Pooled for MissionPropertyValue_AIName {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.mission_property_value_ainame }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.mission_property_value_ainame }
}

impl<'a> Extract<'a> for MissionPropertyValue_AIName {
    const TYPE_NAME: &'static str = "MissionPropertyValue_AIName";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            random_name: inst.get_bool("randomName").unwrap_or_default(),
            random_last_name: inst.get_bool("randomLastName").unwrap_or_default(),
            random_nick_name: inst.get_bool("randomNickName").unwrap_or_default(),
            character_given_name: inst.get_str("characterGivenName").map(String::from).unwrap_or_default(),
            character_given_last_name: inst.get_str("characterGivenLastName").map(String::from).unwrap_or_default(),
            character_given_nick_name: inst.get_str("characterGivenNickName").map(String::from).unwrap_or_default(),
            character_name_data: inst.get("characterNameData").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            chance_of_nick_name: inst.get_f32("chanceOfNickName").unwrap_or_default(),
        }
    }
}

/// DCB type: `BeaconsContracts`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconsContracts {
    /// `serviceBeacons` (Reference (array))
    #[serde(default)]
    pub service_beacons: Vec<CigGuid>,
    /// `serviceBeaconContractGenerator` (Reference)
    #[serde(default)]
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

/// DCB type: `SServiceBeaconCreatorParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SServiceBeaconCreatorParamsBase {
}

impl Pooled for SServiceBeaconCreatorParamsBase {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.sservice_beacon_creator_params_base }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.sservice_beacon_creator_params_base }
}

impl<'a> Extract<'a> for SServiceBeaconCreatorParamsBase {
    const TYPE_NAME: &'static str = "SServiceBeaconCreatorParamsBase";
    fn extract(_inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
        }
    }
}

/// DCB type: `SServiceBeaconCreatorParams`
/// Inherits from: `SServiceBeaconCreatorParamsBase`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SServiceBeaconCreatorParams {
    /// `missionEntry` (Reference)
    #[serde(default)]
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

/// DCB type: `ServiceBeaconBaseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBeaconBaseParams {
    /// `serviceBeaconType` (EnumChoice)
    #[serde(default)]
    pub service_beacon_type: String,
    /// `serviceBeaconName` (Locale)
    #[serde(default)]
    pub service_beacon_name: String,
    /// `beaconTaxPercentage` (Int32)
    #[serde(default)]
    pub beacon_tax_percentage: i32,
    /// `beaconMaxPaymentAmount` (Int32)
    #[serde(default)]
    pub beacon_max_payment_amount: i32,
    /// `npcRequesterNameDef` (Class)
    #[serde(default)]
    pub npc_requester_name_def: Option<Handle<MissionPropertyValue_AIName>>,
    /// `playerCreatorParams` (StrongPointer)
    #[serde(default)]
    pub player_creator_params: Option<Handle<SServiceBeaconCreatorParams>>,
    /// `npcCreatorParams` (StrongPointer)
    #[serde(default)]
    pub npc_creator_params: Option<Handle<SServiceBeaconCreatorParamsBase>>,
}

impl Pooled for ServiceBeaconBaseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.servicebeacon.service_beacon_base_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.servicebeacon.service_beacon_base_params }
}

impl<'a> Extract<'a> for ServiceBeaconBaseParams {
    const TYPE_NAME: &'static str = "ServiceBeaconBaseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            service_beacon_type: inst.get_str("serviceBeaconType").map(String::from).unwrap_or_default(),
            service_beacon_name: inst.get_str("serviceBeaconName").map(String::from).unwrap_or_default(),
            beacon_tax_percentage: inst.get_i32("beaconTaxPercentage").unwrap_or_default(),
            beacon_max_payment_amount: inst.get_i32("beaconMaxPaymentAmount").unwrap_or_default(),
            npc_requester_name_def: match inst.get("npcRequesterNameDef") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<MissionPropertyValue_AIName>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            player_creator_params: match inst.get("playerCreatorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            npc_creator_params: match inst.get("npcCreatorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SServiceBeaconCreatorParamsBase>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SServiceBeaconCreatorParamsBase>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ServiceBeaconParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBeaconParams {
    /// `params` (StrongPointer)
    #[serde(default)]
    pub params: Option<Handle<ServiceBeaconBaseParams>>,
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
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconBaseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconBaseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `ServiceBeaconNotificationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBeaconNotificationParams {
    /// `message` (Locale)
    #[serde(default)]
    pub message: String,
    /// `screenTimer` (Single)
    #[serde(default)]
    pub screen_timer: f32,
    /// `hurryScreenTimer` (Single)
    #[serde(default)]
    pub hurry_screen_timer: f32,
    /// `blocking` (Boolean)
    #[serde(default)]
    pub blocking: bool,
    /// `dockNotificationParamsOverride` (Reference)
    #[serde(default)]
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
            message: inst.get_str("message").map(String::from).unwrap_or_default(),
            screen_timer: inst.get_f32("screenTimer").unwrap_or_default(),
            hurry_screen_timer: inst.get_f32("hurryScreenTimer").unwrap_or_default(),
            blocking: inst.get_bool("blocking").unwrap_or_default(),
            dock_notification_params_override: inst.get("dockNotificationParamsOverride").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `ServiceBeaconGlobalParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBeaconGlobalParams {
    /// `quantumTravelPointClass` (Reference)
    #[serde(default)]
    pub quantum_travel_point_class: Option<CigGuid>,
    /// `missionTypeRecord` (Reference)
    #[serde(default)]
    pub mission_type_record: Option<CigGuid>,
    /// `personalTransportDetectedNotification` (Class)
    #[serde(default)]
    pub personal_transport_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `combatAssistanceDetectedNotification` (Class)
    #[serde(default)]
    pub combat_assistance_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `escortDetectedNotification` (Class)
    #[serde(default)]
    pub escort_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `refuelDetectedNotification` (Class)
    #[serde(default)]
    pub refuel_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `reviveDetectedNotification` (Class)
    #[serde(default)]
    pub revive_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `healDetectedNotification` (Class)
    #[serde(default)]
    pub heal_detected_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractAcceptedNotification` (Class)
    #[serde(default)]
    pub contract_accepted_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractCancelledNotification` (Class)
    #[serde(default)]
    pub contract_cancelled_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractNoLongerAvailableNotification` (Class)
    #[serde(default)]
    pub contract_no_longer_available_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractCompletedInitiatorNotification` (Class)
    #[serde(default)]
    pub contract_completed_initiator_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractCompletedProviderNotification` (Class)
    #[serde(default)]
    pub contract_completed_provider_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `providerWithinRangeNotification` (Class)
    #[serde(default)]
    pub provider_within_range_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `spoofedContractOfferedNotification` (Class)
    #[serde(default)]
    pub spoofed_contract_offered_notification: Option<Handle<ServiceBeaconNotificationParams>>,
    /// `contractProviderName` (Locale)
    #[serde(default)]
    pub contract_provider_name: String,
    /// `vehicleLocationChosenForPersonalTransport` (Locale)
    #[serde(default)]
    pub vehicle_location_chosen_for_personal_transport: String,
    /// `providerNameToken` (String)
    #[serde(default)]
    pub provider_name_token: String,
    /// `initiatorNameToken` (String)
    #[serde(default)]
    pub initiator_name_token: String,
    /// `selectedDestinationToken` (String)
    #[serde(default)]
    pub selected_destination_token: String,
    /// `contractTypeToken` (String)
    #[serde(default)]
    pub contract_type_token: String,
    /// `distanceToInitiatorToken` (String)
    #[serde(default)]
    pub distance_to_initiator_token: String,
    /// `initiatorLocationToken` (String)
    #[serde(default)]
    pub initiator_location_token: String,
    /// `paymentAmountToken` (String)
    #[serde(default)]
    pub payment_amount_token: String,
    /// `openSpaceLocationName` (Locale)
    #[serde(default)]
    pub open_space_location_name: String,
    /// `allReputationsLabel` (Locale)
    #[serde(default)]
    pub all_reputations_label: String,
    /// `oneStarReputationLabel` (Locale)
    #[serde(default)]
    pub one_star_reputation_label: String,
    /// `twoStarReputationLabel` (Locale)
    #[serde(default)]
    pub two_star_reputation_label: String,
    /// `threeStarReputationLabel` (Locale)
    #[serde(default)]
    pub three_star_reputation_label: String,
    /// `fourStarReputationLabel` (Locale)
    #[serde(default)]
    pub four_star_reputation_label: String,
    /// `fiveStarReputationLabel` (Locale)
    #[serde(default)]
    pub five_star_reputation_label: String,
    /// `invalidTypeErrorMessage` (Locale)
    #[serde(default)]
    pub invalid_type_error_message: String,
    /// `invalidReputationErrorMessage` (Locale)
    #[serde(default)]
    pub invalid_reputation_error_message: String,
    /// `priceIsZeroErrorMessage` (Locale)
    #[serde(default)]
    pub price_is_zero_error_message: String,
    /// `insufficientFundsErrorMessage` (Locale)
    #[serde(default)]
    pub insufficient_funds_error_message: String,
    /// `invalidLocationSelectedErrorMessage` (Locale)
    #[serde(default)]
    pub invalid_location_selected_error_message: String,
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
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            combat_assistance_detected_notification: match inst.get("combatAssistanceDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            escort_detected_notification: match inst.get("escortDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            refuel_detected_notification: match inst.get("refuelDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            revive_detected_notification: match inst.get("reviveDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            heal_detected_notification: match inst.get("healDetectedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_accepted_notification: match inst.get("contractAcceptedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_cancelled_notification: match inst.get("contractCancelledNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_no_longer_available_notification: match inst.get("contractNoLongerAvailableNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_completed_initiator_notification: match inst.get("contractCompletedInitiatorNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_completed_provider_notification: match inst.get("contractCompletedProviderNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            provider_within_range_notification: match inst.get("providerWithinRangeNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spoofed_contract_offered_notification: match inst.get("spoofedContractOfferedNotification") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<ServiceBeaconNotificationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            contract_provider_name: inst.get_str("contractProviderName").map(String::from).unwrap_or_default(),
            vehicle_location_chosen_for_personal_transport: inst.get_str("vehicleLocationChosenForPersonalTransport").map(String::from).unwrap_or_default(),
            provider_name_token: inst.get_str("providerNameToken").map(String::from).unwrap_or_default(),
            initiator_name_token: inst.get_str("initiatorNameToken").map(String::from).unwrap_or_default(),
            selected_destination_token: inst.get_str("selectedDestinationToken").map(String::from).unwrap_or_default(),
            contract_type_token: inst.get_str("contractTypeToken").map(String::from).unwrap_or_default(),
            distance_to_initiator_token: inst.get_str("distanceToInitiatorToken").map(String::from).unwrap_or_default(),
            initiator_location_token: inst.get_str("initiatorLocationToken").map(String::from).unwrap_or_default(),
            payment_amount_token: inst.get_str("paymentAmountToken").map(String::from).unwrap_or_default(),
            open_space_location_name: inst.get_str("openSpaceLocationName").map(String::from).unwrap_or_default(),
            all_reputations_label: inst.get_str("allReputationsLabel").map(String::from).unwrap_or_default(),
            one_star_reputation_label: inst.get_str("oneStarReputationLabel").map(String::from).unwrap_or_default(),
            two_star_reputation_label: inst.get_str("twoStarReputationLabel").map(String::from).unwrap_or_default(),
            three_star_reputation_label: inst.get_str("threeStarReputationLabel").map(String::from).unwrap_or_default(),
            four_star_reputation_label: inst.get_str("fourStarReputationLabel").map(String::from).unwrap_or_default(),
            five_star_reputation_label: inst.get_str("fiveStarReputationLabel").map(String::from).unwrap_or_default(),
            invalid_type_error_message: inst.get_str("invalidTypeErrorMessage").map(String::from).unwrap_or_default(),
            invalid_reputation_error_message: inst.get_str("invalidReputationErrorMessage").map(String::from).unwrap_or_default(),
            price_is_zero_error_message: inst.get_str("priceIsZeroErrorMessage").map(String::from).unwrap_or_default(),
            insufficient_funds_error_message: inst.get_str("insufficientFundsErrorMessage").map(String::from).unwrap_or_default(),
            invalid_location_selected_error_message: inst.get_str("invalidLocationSelectedErrorMessage").map(String::from).unwrap_or_default(),
        }
    }
}

