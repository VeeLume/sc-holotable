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

/// Pool storage for the `transportsystem` feature.
#[derive(Default)]
pub struct TransportsystemPools {
    pub transport_event_item_spawner_params: Vec<Option<TransportEventItemSpawnerParams>>,
    pub transport_destination_category: Vec<Option<TransportDestinationCategory>>,
    pub transport_destination_categories: Vec<Option<TransportDestinationCategories>>,
    pub transport_icon_type: Vec<Option<TransportIconType>>,
    pub transport_icon_types: Vec<Option<TransportIconTypes>>,
    pub transport_carriage_generic_announcement: Vec<Option<TransportCarriageGenericAnnouncement>>,
    pub transport_carriage_destination_announcement:
        Vec<Option<TransportCarriageDestinationAnnouncement>>,
    pub transport_carriage_announcements: Vec<Option<TransportCarriageAnnouncements>>,
    pub transport_manager_params: Vec<Option<TransportManagerParams>>,
    pub transport_destination_params: Vec<Option<TransportDestinationParams>>,
    pub transport_gateway_params: Vec<Option<TransportGatewayParams>>,
    pub transport_nav_spline_params: Vec<Option<TransportNavSplineParams>>,
    pub transport_pause_point_params: Vec<Option<TransportPausePointParams>>,
    pub transport_gateway_timer_panel_params: Vec<Option<TransportGatewayTimerPanelParams>>,
    pub transport_gateway_control_panel_params: Vec<Option<TransportGatewayControlPanelParams>>,
    pub transport_carriage_control_panel_params: Vec<Option<TransportCarriageControlPanelParams>>,
    pub transport_carriage_timer_panel_params: Vec<Option<TransportCarriageTimerPanelParams>>,
    pub transport_carriage_audio: Vec<Option<TransportCarriageAudio>>,
    pub transport_carriage_effects: Vec<Option<TransportCarriageEffects>>,
    pub transport_carriage_params: Vec<Option<TransportCarriageParams>>,
}
