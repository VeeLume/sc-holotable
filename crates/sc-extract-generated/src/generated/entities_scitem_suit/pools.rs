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

/// Pool storage for the `entities-scitem-suit` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemSuitPools {
    #[serde(default)]
    pub legacy_mobi_glas_params: Vec<Option<LegacyMobiGlasParams>>,
    #[serde(default)]
    pub landing_area_services_params: Vec<Option<LandingAreaServicesParams>>,
    #[serde(default)]
    pub entity_component_docking_hub_params: Vec<Option<EntityComponentDockingHubParams>>,
    #[serde(default)]
    pub smission_broker_interface_component_params: Vec<Option<SMissionBrokerInterfaceComponentParams>>,
    #[serde(default)]
    pub mobi_glas_position_params: Vec<Option<MobiGlasPositionParams>>,
    #[serde(default)]
    pub mobi_glas_client_params: Vec<Option<MobiGlasClientParams>>,
    #[serde(default)]
    pub mobi_glas_remote_params: Vec<Option<MobiGlasRemoteParams>>,
    #[serde(default)]
    pub mobi_glas_effect_params: Vec<Option<MobiGlasEffectParams>>,
    #[serde(default)]
    pub scitem_mobi_glas_params: Vec<Option<SCItemMobiGlasParams>>,
    #[serde(default)]
    pub player_asset_manager_provider_params: Vec<Option<PlayerAssetManagerProviderParams>>,
    #[serde(default)]
    pub player_trade_provider_params: Vec<Option<PlayerTradeProviderParams>>,
    #[serde(default)]
    pub reputation_provider_params: Vec<Option<ReputationProviderParams>>,
}
