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

/// Pool storage for the `entities-scitem-suit` feature.
#[derive(Default)]
pub struct EntitiesScitemSuitPools {
    pub legacy_mobi_glas_params: Vec<Option<LegacyMobiGlasParams>>,
    pub landing_area_services_params: Vec<Option<LandingAreaServicesParams>>,
    pub entity_component_docking_hub_params: Vec<Option<EntityComponentDockingHubParams>>,
    pub smission_broker_interface_component_params: Vec<Option<SMissionBrokerInterfaceComponentParams>>,
    pub mobi_glas_position_params: Vec<Option<MobiGlasPositionParams>>,
    pub mobi_glas_client_params: Vec<Option<MobiGlasClientParams>>,
    pub mobi_glas_remote_params: Vec<Option<MobiGlasRemoteParams>>,
    pub mobi_glas_effect_params: Vec<Option<MobiGlasEffectParams>>,
    pub scitem_mobi_glas_params: Vec<Option<SCItemMobiGlasParams>>,
    pub player_asset_manager_provider_params: Vec<Option<PlayerAssetManagerProviderParams>>,
    pub player_trade_provider_params: Vec<Option<PlayerTradeProviderParams>>,
    pub reputation_provider_params: Vec<Option<ReputationProviderParams>>,
}
