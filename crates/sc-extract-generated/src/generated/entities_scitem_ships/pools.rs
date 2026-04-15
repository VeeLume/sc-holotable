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

/// Pool storage for the `entities-scitem-ships` feature.
#[derive(Default)]
pub struct EntitiesScitemShipsPools {
    pub item_modifier_timed_life: Vec<Option<ItemModifierTimedLife>>,
    pub item_mining_booster_params: Vec<Option<ItemMiningBoosterParams>>,
    pub smisfire_functionality_condition: Vec<Option<SMisfireFunctionalityCondition>>,
    pub hover_plane: Vec<Option<HoverPlane>>,
    pub suspension_springs: Vec<Option<SuspensionSprings>>,
    pub hover_height: Vec<Option<HoverHeight>>,
    pub hover_tilting: Vec<Option<HoverTilting>>,
    pub hover_collisions: Vec<Option<HoverCollisions>>,
    pub hover_handling: Vec<Option<HoverHandling>>,
    pub gravlev_params: Vec<Option<GravlevParams>>,
    pub sifcsmodifier_number: Vec<Option<SIFCSModifierNumber>>,
    pub sifcsmodifier_vector: Vec<Option<SIFCSModifierVector>>,
    pub sifcsmodifiers_legacy: Vec<Option<SIFCSModifiersLegacy>>,
    pub send_docking_enable_event: Vec<Option<SendDockingEnableEvent>>,
    pub self_destruct_state_modifier: Vec<Option<SelfDestructStateModifier>>,
    pub item_resource_dynamic_amount_fuel_nozzle_fuel: Vec<Option<ItemResourceDynamicAmountFuelNozzleFuel>>,
    pub scitem_empparams: Vec<Option<SCItemEMPParams>>,
    pub scitem_fuel_nozzle_params: Vec<Option<SCItemFuelNozzleParams>>,
    pub scitem_space_mine_params: Vec<Option<SCItemSpaceMineParams>>,
    pub smfdparams_diagnostics: Vec<Option<SMFDParamsDiagnostics>>,
    pub sdummy_launcher: Vec<Option<SDummyLauncher>>,
}
