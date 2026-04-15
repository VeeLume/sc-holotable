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

/// Pool storage for the `entities-scitem-ships` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesScitemShipsPools {
    #[serde(default)]
    pub item_modifier_timed_life: Vec<Option<ItemModifierTimedLife>>,
    #[serde(default)]
    pub item_mining_booster_params: Vec<Option<ItemMiningBoosterParams>>,
    #[serde(default)]
    pub smisfire_functionality_condition: Vec<Option<SMisfireFunctionalityCondition>>,
    #[serde(default)]
    pub hover_plane: Vec<Option<HoverPlane>>,
    #[serde(default)]
    pub suspension_springs: Vec<Option<SuspensionSprings>>,
    #[serde(default)]
    pub hover_height: Vec<Option<HoverHeight>>,
    #[serde(default)]
    pub hover_tilting: Vec<Option<HoverTilting>>,
    #[serde(default)]
    pub hover_collisions: Vec<Option<HoverCollisions>>,
    #[serde(default)]
    pub hover_handling: Vec<Option<HoverHandling>>,
    #[serde(default)]
    pub gravlev_params: Vec<Option<GravlevParams>>,
    #[serde(default)]
    pub sifcsmodifier_number: Vec<Option<SIFCSModifierNumber>>,
    #[serde(default)]
    pub sifcsmodifier_vector: Vec<Option<SIFCSModifierVector>>,
    #[serde(default)]
    pub sifcsmodifiers_legacy: Vec<Option<SIFCSModifiersLegacy>>,
    #[serde(default)]
    pub send_docking_enable_event: Vec<Option<SendDockingEnableEvent>>,
    #[serde(default)]
    pub self_destruct_state_modifier: Vec<Option<SelfDestructStateModifier>>,
    #[serde(default)]
    pub item_resource_dynamic_amount_fuel_nozzle_fuel: Vec<Option<ItemResourceDynamicAmountFuelNozzleFuel>>,
    #[serde(default)]
    pub scitem_empparams: Vec<Option<SCItemEMPParams>>,
    #[serde(default)]
    pub scitem_fuel_nozzle_params: Vec<Option<SCItemFuelNozzleParams>>,
    #[serde(default)]
    pub scitem_space_mine_params: Vec<Option<SCItemSpaceMineParams>>,
    #[serde(default)]
    pub smfdparams_diagnostics: Vec<Option<SMFDParamsDiagnostics>>,
    #[serde(default)]
    pub sdummy_launcher: Vec<Option<SDummyLauncher>>,
}
