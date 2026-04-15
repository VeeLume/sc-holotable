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

/// Pool storage for the `ui-markertrackingvolumeconfig` feature.
#[derive(Default)]
pub struct UiMarkertrackingvolumeconfigPools {
    pub marker_tracking_common_map_parameters: Vec<Option<MarkerTrackingCommonMapParameters>>,
    pub marker_tracking_label_parameters: Vec<Option<MarkerTrackingLabelParameters>>,
}
