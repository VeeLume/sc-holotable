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
    pub sifcsmodifier_number: Vec<Option<SIFCSModifierNumber>>,
    pub sifcsmodifier_vector: Vec<Option<SIFCSModifierVector>>,
    pub sifcsmodifiers_legacy: Vec<Option<SIFCSModifiersLegacy>>,
    pub self_destruct_state_modifier: Vec<Option<SelfDestructStateModifier>>,
    pub scitem_empparams: Vec<Option<SCItemEMPParams>>,
    pub scitem_space_mine_params: Vec<Option<SCItemSpaceMineParams>>,
    pub smfdparams_diagnostics: Vec<Option<SMFDParamsDiagnostics>>,
    pub sdummy_launcher: Vec<Option<SDummyLauncher>>,
}
