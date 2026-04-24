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

/// Pool storage for the `entities-scitem-mission_entities` feature.
#[derive(Default)]
pub struct EntitiesScitemMission_entitiesPools {
    pub hackable_params: Vec<Option<HackableParams>>,
    pub sshop_uiprovider_category_icon: Vec<Option<SShopUIProviderCategoryIcon>>,
    pub sshop_uiprovider_params: Vec<Option<SShopUIProviderParams>>,
    pub scommodity_uiprovider_params: Vec<Option<SCommodityUIProviderParams>>,
    pub monitored_zone_controller_params: Vec<Option<MonitoredZoneControllerParams>>,
    pub scriminal_record_state_modifier: Vec<Option<SCriminalRecordStateModifier>>,
    pub scriminal_record_hacking_state_modifier: Vec<Option<SCriminalRecordHackingStateModifier>>,
    pub sgas_tank_filler_state_modifier: Vec<Option<SGasTankFillerStateModifier>>,
    pub shackable_state_modifier: Vec<Option<SHackableStateModifier>>,
    pub svending_machine_state_modifier: Vec<Option<SVendingMachineStateModifier>>,
    pub sremove_crimes_gameplay_trigger: Vec<Option<SRemoveCrimesGameplayTrigger>>,
    pub sstart_remove_infraction_timer_gameplay_trigger:
        Vec<Option<SStartRemoveInfractionTimerGameplayTrigger>>,
    pub vending_machine_shop_params: Vec<Option<VendingMachineShopParams>>,
}
