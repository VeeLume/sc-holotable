// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `trackview`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `CameraTransitionInterpolationCurveRecord`
pub struct CameraTransitionInterpolationCurveRecord {
    /// `curve` (Class)
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for CameraTransitionInterpolationCurveRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.trackview.camera_transition_interpolation_curve_record
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.trackview.camera_transition_interpolation_curve_record
    }
}

impl<'a> Extract<'a> for CameraTransitionInterpolationCurveRecord {
    const TYPE_NAME: &'static str = "CameraTransitionInterpolationCurveRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
        }
    }
}

/// DCB type: `CinematicFlythroughPoint`
pub struct CinematicFlythroughPoint {
    /// `position` (Class)
    pub position: Option<Handle<Vec3>>,
    /// `rotation` (Class)
    pub rotation: Option<Handle<Quat>>,
    /// `duration` (Single)
    pub duration: f32,
    /// `relativeTo` (EnumChoice)
    pub relative_to: ECameraTransitionRelativeTo,
    /// `interpolationToPoint` (Reference)
    pub interpolation_to_point: Option<CigGuid>,
}

impl Pooled for CinematicFlythroughPoint {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.trackview.cinematic_flythrough_point
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.trackview.cinematic_flythrough_point
    }
}

impl<'a> Extract<'a> for CinematicFlythroughPoint {
    const TYPE_NAME: &'static str = "CinematicFlythroughPoint";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Quat>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            duration: inst.get_f32("duration").unwrap_or_default(),
            relative_to: ECameraTransitionRelativeTo::from_dcb_str(
                inst.get_str("relativeTo").unwrap_or(""),
            ),
            interpolation_to_point: inst
                .get("interpolationToPoint")
                .and_then(|v| v.as_record_ref())
                .map(|r| r.guid),
        }
    }
}

/// DCB type: `CinematicFlightPointsRecord`
pub struct CinematicFlightPointsRecord {
    /// `flightPoints` (Class (array))
    pub flight_points: Vec<Handle<CinematicFlythroughPoint>>,
}

impl Pooled for CinematicFlightPointsRecord {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.trackview.cinematic_flight_points_record
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.trackview.cinematic_flight_points_record
    }
}

impl<'a> Extract<'a> for CinematicFlightPointsRecord {
    const TYPE_NAME: &'static str = "CinematicFlightPointsRecord";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            flight_points: inst
                .get_array("flightPoints")
                .map(|arr| {
                    arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => {
                            Some(b.alloc_nested::<CinematicFlythroughPoint>(
                                Instance::from_inline_data(b.db, struct_index, data),
                                false,
                            ))
                        }
                        Value::ClassRef(r) => Some(b.alloc_nested::<CinematicFlythroughPoint>(
                            b.db.instance(r.struct_index, r.instance_index),
                            true,
                        )),
                        _ => None,
                    })
                    .collect()
                })
                .unwrap_or_default(),
        }
    }
}
