// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `servicebeacon` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServicebeaconPools {
    #[serde(default)]
    pub contract_generator_handler_service_beacon: Vec<Option<ContractGeneratorHandler_ServiceBeacon>>,
    #[serde(default)]
    pub contract_class_service_beacon: Vec<Option<ContractClass_ServiceBeacon>>,
    #[serde(default)]
    pub mission_property_value_star_map_location: Vec<Option<MissionPropertyValue_StarMapLocation>>,
    #[serde(default)]
    pub beacons_contracts: Vec<Option<BeaconsContracts>>,
    #[serde(default)]
    pub service_beacon_base_template_params: Vec<Option<ServiceBeaconBaseTemplateParams>>,
    #[serde(default)]
    pub personal_transport_beacon_params: Vec<Option<PersonalTransportBeaconParams>>,
    #[serde(default)]
    pub sservice_beacon_notification_override: Vec<Option<SServiceBeaconNotificationOverride>>,
    #[serde(default)]
    pub sservice_beacon_creator_params: Vec<Option<SServiceBeaconCreatorParams>>,
    #[serde(default)]
    pub sservice_beacon_difficulty_entry: Vec<Option<SServiceBeaconDifficultyEntry>>,
    #[serde(default)]
    pub sservice_beacon_creator_params_with_difficulty: Vec<Option<SServiceBeaconCreatorParamsWithDifficulty>>,
    #[serde(default)]
    pub service_beacon_base_params: Vec<Option<ServiceBeaconBaseParams>>,
    #[serde(default)]
    pub personal_transport_params: Vec<Option<PersonalTransportParams>>,
    #[serde(default)]
    pub escort_params: Vec<Option<EscortParams>>,
    #[serde(default)]
    pub refuel_params: Vec<Option<RefuelParams>>,
    #[serde(default)]
    pub combat_assistance_params: Vec<Option<CombatAssistanceParams>>,
    #[serde(default)]
    pub revive_params: Vec<Option<ReviveParams>>,
    #[serde(default)]
    pub heal_params: Vec<Option<HealParams>>,
    #[serde(default)]
    pub service_beacon_params: Vec<Option<ServiceBeaconParams>>,
    #[serde(default)]
    pub service_beacon_notification_params: Vec<Option<ServiceBeaconNotificationParams>>,
    #[serde(default)]
    pub service_beacon_global_params: Vec<Option<ServiceBeaconGlobalParams>>,
}
