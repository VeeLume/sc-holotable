// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `ui-markertrackingvolumeconfig` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiMarkertrackingvolumeconfigPools {
    #[serde(default)]
    pub marker_tracking_view_mode_parameters: Vec<Option<MarkerTrackingViewModeParameters>>,
    #[serde(default)]
    pub marker_tracking_common_map_parameters: Vec<Option<MarkerTrackingCommonMapParameters>>,
    #[serde(default)]
    pub marker_tracking_action_parameters: Vec<Option<MarkerTrackingActionParameters>>,
    #[serde(default)]
    pub marker_tracking_display_parameters: Vec<Option<MarkerTrackingDisplayParameters>>,
    #[serde(default)]
    pub marker_tracking_label_parameters: Vec<Option<MarkerTrackingLabelParameters>>,
}
