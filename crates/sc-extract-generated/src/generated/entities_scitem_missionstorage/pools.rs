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

/// Pool storage for the `entities-scitem-missionstorage` feature.
#[derive(Default)]
pub struct EntitiesScitemMissionstoragePools {
    pub sdespawn_rule_on_parts_detached: Vec<Option<SDespawnRule_OnPartsDetached>>,
    pub interaction_condition_delivery_mission_item:
        Vec<Option<InteractionConditionDeliveryMissionItem>>,
    pub delivery_item_port_component_params: Vec<Option<DeliveryItemPortComponentParams>>,
}
