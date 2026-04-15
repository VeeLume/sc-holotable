// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `servicebeacon` feature.
#[derive(Default)]
pub struct ServicebeaconPools {
    pub contract_generator_handler_service_beacon: Vec<Option<ContractGeneratorHandler_ServiceBeacon>>,
    pub contract_class_service_beacon: Vec<Option<ContractClass_ServiceBeacon>>,
    pub mission_property_value_star_map_location: Vec<Option<MissionPropertyValue_StarMapLocation>>,
    pub beacons_contracts: Vec<Option<BeaconsContracts>>,
    pub service_beacon_base_template_params: Vec<Option<ServiceBeaconBaseTemplateParams>>,
    pub personal_transport_beacon_params: Vec<Option<PersonalTransportBeaconParams>>,
    pub sservice_beacon_notification_override: Vec<Option<SServiceBeaconNotificationOverride>>,
    pub sservice_beacon_creator_params: Vec<Option<SServiceBeaconCreatorParams>>,
    pub sservice_beacon_difficulty_entry: Vec<Option<SServiceBeaconDifficultyEntry>>,
    pub sservice_beacon_creator_params_with_difficulty: Vec<Option<SServiceBeaconCreatorParamsWithDifficulty>>,
    pub service_beacon_base_params: Vec<Option<ServiceBeaconBaseParams>>,
    pub personal_transport_params: Vec<Option<PersonalTransportParams>>,
    pub escort_params: Vec<Option<EscortParams>>,
    pub refuel_params: Vec<Option<RefuelParams>>,
    pub combat_assistance_params: Vec<Option<CombatAssistanceParams>>,
    pub revive_params: Vec<Option<ReviveParams>>,
    pub heal_params: Vec<Option<HealParams>>,
    pub service_beacon_params: Vec<Option<ServiceBeaconParams>>,
    pub service_beacon_notification_params: Vec<Option<ServiceBeaconNotificationParams>>,
    pub service_beacon_global_params: Vec<Option<ServiceBeaconGlobalParams>>,
}
