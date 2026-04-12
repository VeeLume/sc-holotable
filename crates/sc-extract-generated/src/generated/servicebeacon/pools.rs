// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `servicebeacon` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServicebeaconPools {
    #[serde(default)]
    pub mission_property_value_ainame: Vec<Option<MissionPropertyValue_AIName>>,
    #[serde(default)]
    pub beacons_contracts: Vec<Option<BeaconsContracts>>,
    #[serde(default)]
    pub sservice_beacon_creator_params_base: Vec<Option<SServiceBeaconCreatorParamsBase>>,
    #[serde(default)]
    pub sservice_beacon_creator_params: Vec<Option<SServiceBeaconCreatorParams>>,
    #[serde(default)]
    pub service_beacon_base_params: Vec<Option<ServiceBeaconBaseParams>>,
    #[serde(default)]
    pub service_beacon_params: Vec<Option<ServiceBeaconParams>>,
    #[serde(default)]
    pub service_beacon_notification_params: Vec<Option<ServiceBeaconNotificationParams>>,
    #[serde(default)]
    pub service_beacon_global_params: Vec<Option<ServiceBeaconGlobalParams>>,
}
