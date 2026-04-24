// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use std::collections::HashMap;
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `trackview` feature.
#[derive(Default)]
pub struct TrackviewIndex {
    pub camera_transition_interpolation_curve_record: HashMap<CigGuid, Handle<CameraTransitionInterpolationCurveRecord>>,
    pub cinematic_flight_points_record: HashMap<CigGuid, Handle<CinematicFlightPointsRecord>>,
}

impl TrackviewIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut total = 0usize;
        total += self.camera_transition_interpolation_curve_record.len();
        total += self.cinematic_flight_points_record.len();
        total
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
