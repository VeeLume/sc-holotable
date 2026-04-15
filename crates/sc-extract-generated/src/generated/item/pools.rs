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

/// Pool storage for the `item` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemPools {
    #[serde(default)]
    pub game_tokens: Vec<Option<GameTokens>>,
    #[serde(default)]
    pub item: Vec<Option<Item>>,
    #[serde(default)]
    pub vehicle_item_interior_controller: Vec<Option<VehicleItemInteriorController>>,
    #[serde(default)]
    pub shelmet_linked_state: Vec<Option<SHelmetLinkedState>>,
    #[serde(default)]
    pub shelmet_state_base_params: Vec<Option<SHelmetStateBaseParams>>,
    #[serde(default)]
    pub shelmet_state_machine_params: Vec<Option<SHelmetStateMachineParams>>,
    #[serde(default)]
    pub animated_helmet_params: Vec<Option<AnimatedHelmetParams>>,
}
