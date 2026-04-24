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

/// Pool storage for the `trackview` feature.
#[derive(Default)]
pub struct TrackviewPools {
    pub camera_transition_interpolation_curve_record:
        Vec<Option<CameraTransitionInterpolationCurveRecord>>,
    pub cinematic_flythrough_point: Vec<Option<CinematicFlythroughPoint>>,
    pub cinematic_flight_points_record: Vec<Option<CinematicFlightPointsRecord>>,
}
