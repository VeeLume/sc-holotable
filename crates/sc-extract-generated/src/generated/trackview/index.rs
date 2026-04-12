// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `trackview` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrackviewIndex {
    #[serde(default)]
    pub camera_transition_interpolation_curve_record: HashMap<CigGuid, Handle<CameraTransitionInterpolationCurveRecord>>,
    #[serde(default)]
    pub cinematic_flight_points_record: HashMap<CigGuid, Handle<CinematicFlightPointsRecord>>,
}

impl TrackviewIndex {
    pub fn len(&self) -> usize {
        self.camera_transition_interpolation_curve_record.len()
            + self.cinematic_flight_points_record.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
