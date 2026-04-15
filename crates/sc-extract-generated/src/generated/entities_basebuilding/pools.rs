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

/// Pool storage for the `entities-basebuilding` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesBasebuildingPools {
    #[serde(default)]
    pub blueprint_category_availability_whitelist: Vec<Option<BlueprintCategoryAvailability_Whitelist>>,
    #[serde(default)]
    pub crafter_door_state_event: Vec<Option<CrafterDoorStateEvent>>,
    #[serde(default)]
    pub crafting_queue_core_params: Vec<Option<CraftingQueueCoreParams>>,
    #[serde(default)]
    pub crafter_interaction_params: Vec<Option<CrafterInteractionParams>>,
    #[serde(default)]
    pub crafter_component_params: Vec<Option<CrafterComponentParams>>,
    #[serde(default)]
    pub crafter_paged_uilist_params: Vec<Option<CrafterPagedUIListParams>>,
    #[serde(default)]
    pub crafter_uiprovider_component_params: Vec<Option<CrafterUIProviderComponentParams>>,
    #[serde(default)]
    pub entity_component_heat_connection: Vec<Option<EntityComponentHeatConnection>>,
    #[serde(default)]
    pub smisfire_band_params: Vec<Option<SMisfireBandParams>>,
    #[serde(default)]
    pub smisfire_severity_factors: Vec<Option<SMisfireSeverityFactors>>,
    #[serde(default)]
    pub smisfire_generation_params: Vec<Option<SMisfireGenerationParams>>,
    #[serde(default)]
    pub smisfire_event_params: Vec<Option<SMisfireEventParams>>,
    #[serde(default)]
    pub smisfire_events: Vec<Option<SMisfireEvents>>,
    #[serde(default)]
    pub entity_component_power_connection: Vec<Option<EntityComponentPowerConnection>>,
    #[serde(default)]
    pub crafter_state_modifier: Vec<Option<CrafterStateModifier>>,
}
