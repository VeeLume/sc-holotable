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

/// Pool storage for the `transitsystem` feature.
#[derive(Default)]
pub struct TransitsystemPools {
    pub transit_navigation_link: Vec<Option<TransitNavigationLink>>,
    pub hospital_emergency_screen_component_params:
        Vec<Option<HospitalEmergencyScreenComponentParams>>,
    pub security_clearance_giver_component_params:
        Vec<Option<SecurityClearanceGiverComponentParams>>,
    pub sbinding_trigger_gameplay_trigger: Vec<Option<SBindingTriggerGameplayTrigger>>,
    pub transit_node_dialogue_context: Vec<Option<TransitNodeDialogueContext>>,
    pub transit_carriage_audio: Vec<Option<TransitCarriageAudio>>,
    pub transit_carriage_effects: Vec<Option<TransitCarriageEffects>>,
    pub transit_carriage_params: Vec<Option<TransitCarriageParams>>,
    pub transit_limbo_node_params: Vec<Option<TransitLimboNodeParams>>,
    pub transit_gateway_params: Vec<Option<TransitGatewayParams>>,
    pub transit_interaction_panel_params: Vec<Option<TransitInteractionPanelParams>>,
    pub transit_destination_params: Vec<Option<TransitDestinationParams>>,
    pub transit_dynamic_destination_params: Vec<Option<TransitDynamicDestinationParams>>,
    pub transit_manager_params: Vec<Option<TransitManagerParams>>,
}
