// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `ESPParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESPParams {
    /// DCB field: `triggerZoneRampInCurve` (Class)
    #[serde(default)]
    pub trigger_zone_ramp_in_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `trackingStrength` (Single)
    #[serde(default)]
    pub tracking_strength: f32,
    /// DCB field: `distanceFalloffStart` (Single)
    #[serde(default)]
    pub distance_falloff_start: f32,
    /// DCB field: `distanceFalloffEnd` (Single)
    #[serde(default)]
    pub distance_falloff_end: f32,
    /// DCB field: `outerZoneDeg` (Single)
    #[serde(default)]
    pub outer_zone_deg: f32,
    /// DCB field: `innerZoneRatio` (Single)
    #[serde(default)]
    pub inner_zone_ratio: f32,
    /// DCB field: `adsZoneMinSizeDeg` (Single)
    #[serde(default)]
    pub ads_zone_min_size_deg: f32,
    /// DCB field: `inputDisengageCurve` (Single)
    #[serde(default)]
    pub input_disengage_curve: f32,
    /// DCB field: `directionSimilaritySmoothSpeed` (Single)
    #[serde(default)]
    pub direction_similarity_smooth_speed: f32,
    /// DCB field: `assistRelaxSpeed` (Single)
    #[serde(default)]
    pub assist_relax_speed: f32,
    /// DCB field: `alignmentAngleCurve` (Single)
    #[serde(default)]
    pub alignment_angle_curve: f32,
    /// DCB field: `dampeningMin` (Single)
    #[serde(default)]
    pub dampening_min: f32,
    /// DCB field: `dampeningMax` (Single)
    #[serde(default)]
    pub dampening_max: f32,
    /// DCB field: `allowPulling` (Boolean)
    #[serde(default)]
    pub allow_pulling: bool,
    /// DCB field: `allowWithRelativeMouseModes` (Boolean)
    #[serde(default)]
    pub allow_with_relative_mouse_modes: bool,
}

impl Pooled for ESPParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_es.espparams }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_es.espparams }
}

impl<'a> Extract<'a> for ESPParams {
    const TYPE_NAME: &'static str = "ESPParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_zone_ramp_in_curve: match inst.get("triggerZoneRampInCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            tracking_strength: inst.get_f32("trackingStrength").unwrap_or_default(),
            distance_falloff_start: inst.get_f32("distanceFalloffStart").unwrap_or_default(),
            distance_falloff_end: inst.get_f32("distanceFalloffEnd").unwrap_or_default(),
            outer_zone_deg: inst.get_f32("outerZoneDeg").unwrap_or_default(),
            inner_zone_ratio: inst.get_f32("innerZoneRatio").unwrap_or_default(),
            ads_zone_min_size_deg: inst.get_f32("adsZoneMinSizeDeg").unwrap_or_default(),
            input_disengage_curve: inst.get_f32("inputDisengageCurve").unwrap_or_default(),
            direction_similarity_smooth_speed: inst.get_f32("directionSimilaritySmoothSpeed").unwrap_or_default(),
            assist_relax_speed: inst.get_f32("assistRelaxSpeed").unwrap_or_default(),
            alignment_angle_curve: inst.get_f32("alignmentAngleCurve").unwrap_or_default(),
            dampening_min: inst.get_f32("dampeningMin").unwrap_or_default(),
            dampening_max: inst.get_f32("dampeningMax").unwrap_or_default(),
            allow_pulling: inst.get_bool("allowPulling").unwrap_or_default(),
            allow_with_relative_mouse_modes: inst.get_bool("allowWithRelativeMouseModes").unwrap_or_default(),
        }
    }
}

