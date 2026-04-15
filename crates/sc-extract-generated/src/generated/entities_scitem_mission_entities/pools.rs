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

/// Pool storage for the `entities-scitem-mission_entities` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemMission_entitiesPools {
    #[serde(default)]
    pub hackable_params: Vec<Option<HackableParams>>,
    #[serde(default)]
    pub sshop_uiprovider_category_icon: Vec<Option<SShopUIProviderCategoryIcon>>,
    #[serde(default)]
    pub sshop_uiprovider_params: Vec<Option<SShopUIProviderParams>>,
    #[serde(default)]
    pub scommodity_uiprovider_params: Vec<Option<SCommodityUIProviderParams>>,
    #[serde(default)]
    pub monitored_zone_controller_params: Vec<Option<MonitoredZoneControllerParams>>,
    #[serde(default)]
    pub scriminal_record_state_modifier: Vec<Option<SCriminalRecordStateModifier>>,
    #[serde(default)]
    pub scriminal_record_hacking_state_modifier: Vec<Option<SCriminalRecordHackingStateModifier>>,
    #[serde(default)]
    pub sgas_tank_filler_state_modifier: Vec<Option<SGasTankFillerStateModifier>>,
    #[serde(default)]
    pub shackable_state_modifier: Vec<Option<SHackableStateModifier>>,
    #[serde(default)]
    pub svending_machine_state_modifier: Vec<Option<SVendingMachineStateModifier>>,
    #[serde(default)]
    pub sremove_crimes_gameplay_trigger: Vec<Option<SRemoveCrimesGameplayTrigger>>,
    #[serde(default)]
    pub sstart_remove_infraction_timer_gameplay_trigger: Vec<Option<SStartRemoveInfractionTimerGameplayTrigger>>,
    #[serde(default)]
    pub vending_machine_shop_params: Vec<Option<VendingMachineShopParams>>,
}
