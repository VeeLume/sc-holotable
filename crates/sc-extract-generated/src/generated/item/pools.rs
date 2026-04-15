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

/// Pool storage for the `item` feature.
#[derive(Default)]
pub struct ItemPools {
    pub game_tokens: Vec<Option<GameTokens>>,
    pub item: Vec<Option<Item>>,
    pub vehicle_item_interior_controller: Vec<Option<VehicleItemInteriorController>>,
    pub shelmet_linked_state: Vec<Option<SHelmetLinkedState>>,
    pub shelmet_state_base_params: Vec<Option<SHelmetStateBaseParams>>,
    pub shelmet_state_machine_params: Vec<Option<SHelmetStateMachineParams>>,
    pub animated_helmet_params: Vec<Option<AnimatedHelmetParams>>,
}
