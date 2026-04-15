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

/// Pool storage for the `interactionconditions` feature.
#[derive(Default)]
pub struct InteractionconditionsPools {
    pub interaction_condition_can_afford_item: Vec<Option<InteractionConditionCanAffordItem>>,
    pub interaction_condition_can_be_body_dragged: Vec<Option<InteractionConditionCanBeBodyDragged>>,
    pub shop_interaction_data: Vec<Option<ShopInteractionData>>,
}
