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

/// Pool storage for the `transitsystem` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransitsystemPools {
    #[serde(default)]
    pub transit_navigation_link: Vec<Option<TransitNavigationLink>>,
    #[serde(default)]
    pub hospital_emergency_screen_component_params: Vec<Option<HospitalEmergencyScreenComponentParams>>,
    #[serde(default)]
    pub security_clearance_giver_component_params: Vec<Option<SecurityClearanceGiverComponentParams>>,
    #[serde(default)]
    pub sbinding_trigger_gameplay_trigger: Vec<Option<SBindingTriggerGameplayTrigger>>,
    #[serde(default)]
    pub transit_node_dialogue_context: Vec<Option<TransitNodeDialogueContext>>,
    #[serde(default)]
    pub transit_carriage_audio: Vec<Option<TransitCarriageAudio>>,
    #[serde(default)]
    pub transit_carriage_effects: Vec<Option<TransitCarriageEffects>>,
    #[serde(default)]
    pub transit_carriage_params: Vec<Option<TransitCarriageParams>>,
    #[serde(default)]
    pub transit_limbo_node_params: Vec<Option<TransitLimboNodeParams>>,
    #[serde(default)]
    pub transit_gateway_params: Vec<Option<TransitGatewayParams>>,
    #[serde(default)]
    pub transit_interaction_panel_params: Vec<Option<TransitInteractionPanelParams>>,
    #[serde(default)]
    pub transit_destination_params: Vec<Option<TransitDestinationParams>>,
    #[serde(default)]
    pub transit_dynamic_destination_params: Vec<Option<TransitDynamicDestinationParams>>,
    #[serde(default)]
    pub transit_manager_params: Vec<Option<TransitManagerParams>>,
}
