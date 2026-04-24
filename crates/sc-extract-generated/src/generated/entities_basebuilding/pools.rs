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

/// Pool storage for the `entities-basebuilding` feature.
#[derive(Default)]
pub struct EntitiesBasebuildingPools {
    pub blueprint_category_availability_whitelist: Vec<Option<BlueprintCategoryAvailability_Whitelist>>,
    pub crafter_door_state_event: Vec<Option<CrafterDoorStateEvent>>,
    pub crafting_queue_core_params: Vec<Option<CraftingQueueCoreParams>>,
    pub crafter_interaction_params: Vec<Option<CrafterInteractionParams>>,
    pub crafter_component_params: Vec<Option<CrafterComponentParams>>,
    pub crafter_paged_uilist_params: Vec<Option<CrafterPagedUIListParams>>,
    pub crafter_uiprovider_component_params: Vec<Option<CrafterUIProviderComponentParams>>,
    pub entity_component_heat_connection: Vec<Option<EntityComponentHeatConnection>>,
    pub smisfire_band_params: Vec<Option<SMisfireBandParams>>,
    pub smisfire_severity_factors: Vec<Option<SMisfireSeverityFactors>>,
    pub smisfire_generation_params: Vec<Option<SMisfireGenerationParams>>,
    pub smisfire_event_params: Vec<Option<SMisfireEventParams>>,
    pub smisfire_events: Vec<Option<SMisfireEvents>>,
    pub entity_component_power_connection: Vec<Option<EntityComponentPowerConnection>>,
    pub crafter_state_modifier: Vec<Option<CrafterStateModifier>>,
}
