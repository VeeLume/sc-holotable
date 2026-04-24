// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ifcs`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `ESPParams`
pub struct ESPParams {
    /// `triggerZoneRampInCurve` (Class)
    pub trigger_zone_ramp_in_curve: Option<Handle<BezierCurve>>,
    /// `trackingStrength` (Single)
    pub tracking_strength: f32,
    /// `distanceFalloffStart` (Single)
    pub distance_falloff_start: f32,
    /// `distanceFalloffEnd` (Single)
    pub distance_falloff_end: f32,
    /// `outerZoneDeg` (Single)
    pub outer_zone_deg: f32,
    /// `innerZoneRatio` (Single)
    pub inner_zone_ratio: f32,
    /// `adsZoneMinSizeDeg` (Single)
    pub ads_zone_min_size_deg: f32,
    /// `inputDisengageCurve` (Single)
    pub input_disengage_curve: f32,
    /// `directionSimilaritySmoothSpeed` (Single)
    pub direction_similarity_smooth_speed: f32,
    /// `assistRelaxSpeed` (Single)
    pub assist_relax_speed: f32,
    /// `alignmentAngleCurve` (Single)
    pub alignment_angle_curve: f32,
    /// `dampeningMin` (Single)
    pub dampening_min: f32,
    /// `dampeningMax` (Single)
    pub dampening_max: f32,
    /// `allowPulling` (Boolean)
    pub allow_pulling: bool,
    /// `allowWithRelativeMouseModes` (Boolean)
    pub allow_with_relative_mouse_modes: bool,
}

impl Pooled for ESPParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ifcs.espparams
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ifcs.espparams
    }
}

impl<'a> Extract<'a> for ESPParams {
    const TYPE_NAME: &'static str = "ESPParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_zone_ramp_in_curve: match inst.get("triggerZoneRampInCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            tracking_strength: inst.get_f32("trackingStrength").unwrap_or_default(),
            distance_falloff_start: inst.get_f32("distanceFalloffStart").unwrap_or_default(),
            distance_falloff_end: inst.get_f32("distanceFalloffEnd").unwrap_or_default(),
            outer_zone_deg: inst.get_f32("outerZoneDeg").unwrap_or_default(),
            inner_zone_ratio: inst.get_f32("innerZoneRatio").unwrap_or_default(),
            ads_zone_min_size_deg: inst.get_f32("adsZoneMinSizeDeg").unwrap_or_default(),
            input_disengage_curve: inst.get_f32("inputDisengageCurve").unwrap_or_default(),
            direction_similarity_smooth_speed: inst
                .get_f32("directionSimilaritySmoothSpeed")
                .unwrap_or_default(),
            assist_relax_speed: inst.get_f32("assistRelaxSpeed").unwrap_or_default(),
            alignment_angle_curve: inst.get_f32("alignmentAngleCurve").unwrap_or_default(),
            dampening_min: inst.get_f32("dampeningMin").unwrap_or_default(),
            dampening_max: inst.get_f32("dampeningMax").unwrap_or_default(),
            allow_pulling: inst.get_bool("allowPulling").unwrap_or_default(),
            allow_with_relative_mouse_modes: inst
                .get_bool("allowWithRelativeMouseModes")
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSEspParams`
pub struct SIFCSEspParams {
    /// `triggerZoneRampInCurve` (Class)
    pub trigger_zone_ramp_in_curve: Option<Handle<BezierCurve>>,
    /// `maxTrackingStrength` (Single)
    pub max_tracking_strength: f32,
    /// `distanceFalloffStart` (Single)
    pub distance_falloff_start: f32,
    /// `distanceFalloffEnd` (Single)
    pub distance_falloff_end: f32,
    /// `outerZoneDeg` (Single)
    pub outer_zone_deg: f32,
    /// `innerZoneRatio` (Single)
    pub inner_zone_ratio: f32,
    /// `adsZoneMinSizeDeg` (Single)
    pub ads_zone_min_size_deg: f32,
    /// `targetChangedRampTime` (Single)
    pub target_changed_ramp_time: f32,
    /// `dampeningRange` (Single)
    pub dampening_range: f32,
    /// `inputScalerMin` (Single)
    pub input_scaler_min: f32,
    /// `inputScalerMax` (Single)
    pub input_scaler_max: f32,
    /// `inputScalerSmoothTime` (Single)
    pub input_scaler_smooth_time: f32,
    /// `inputRelaxSpeed` (Single)
    pub input_relax_speed: f32,
    /// `allowPulling` (Boolean)
    pub allow_pulling: bool,
    /// `smoothingTime` (Single)
    pub smoothing_time: f32,
    /// `smoothingTimeDecreaseMultiplier` (Single)
    pub smoothing_time_decrease_multiplier: f32,
}

impl Pooled for SIFCSEspParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ifcs.sifcsesp_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ifcs.sifcsesp_params
    }
}

impl<'a> Extract<'a> for SIFCSEspParams {
    const TYPE_NAME: &'static str = "SIFCSEspParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            trigger_zone_ramp_in_curve: match inst.get("triggerZoneRampInCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(
                    Instance::from_inline_data(b.db, struct_index, data),
                    false,
                )),
                _ => None,
            },
            max_tracking_strength: inst.get_f32("maxTrackingStrength").unwrap_or_default(),
            distance_falloff_start: inst.get_f32("distanceFalloffStart").unwrap_or_default(),
            distance_falloff_end: inst.get_f32("distanceFalloffEnd").unwrap_or_default(),
            outer_zone_deg: inst.get_f32("outerZoneDeg").unwrap_or_default(),
            inner_zone_ratio: inst.get_f32("innerZoneRatio").unwrap_or_default(),
            ads_zone_min_size_deg: inst.get_f32("adsZoneMinSizeDeg").unwrap_or_default(),
            target_changed_ramp_time: inst.get_f32("targetChangedRampTime").unwrap_or_default(),
            dampening_range: inst.get_f32("dampeningRange").unwrap_or_default(),
            input_scaler_min: inst.get_f32("inputScalerMin").unwrap_or_default(),
            input_scaler_max: inst.get_f32("inputScalerMax").unwrap_or_default(),
            input_scaler_smooth_time: inst.get_f32("inputScalerSmoothTime").unwrap_or_default(),
            input_relax_speed: inst.get_f32("inputRelaxSpeed").unwrap_or_default(),
            allow_pulling: inst.get_bool("allowPulling").unwrap_or_default(),
            smoothing_time: inst.get_f32("smoothingTime").unwrap_or_default(),
            smoothing_time_decrease_multiplier: inst
                .get_f32("smoothingTimeDecreaseMultiplier")
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSEsp`
pub struct SIFCSEsp {
    /// `espPerType` (Class)
    pub esp_per_type: Option<Handle<SIFCSEspParams>>,
}

impl Pooled for SIFCSEsp {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ifcs.sifcsesp
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ifcs.sifcsesp
    }
}

impl<'a> Extract<'a> for SIFCSEsp {
    const TYPE_NAME: &'static str = "SIFCSEsp";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            esp_per_type: match inst.get("espPerType") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SIFCSEspParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}

/// DCB type: `SIFCSGameModePhysicsDamping`
pub struct SIFCSGameModePhysicsDamping {
    /// `dampingValue` (Single)
    pub damping_value: f32,
    /// `transitionTime` (Single)
    pub transition_time: f32,
}

impl Pooled for SIFCSGameModePhysicsDamping {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ifcs.sifcsgame_mode_physics_damping
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ifcs.sifcsgame_mode_physics_damping
    }
}

impl<'a> Extract<'a> for SIFCSGameModePhysicsDamping {
    const TYPE_NAME: &'static str = "SIFCSGameModePhysicsDamping";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            damping_value: inst.get_f32("dampingValue").unwrap_or_default(),
            transition_time: inst.get_f32("transitionTime").unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSGameModeParams`
pub struct SIFCSGameModeParams {
    /// `enableNewModel` (Boolean)
    pub enable_new_model: bool,
    /// `enforceLegacyEspForNewModel` (Boolean)
    pub enforce_legacy_esp_for_new_model: bool,
    /// `enableDecoupledGlidingDefault` (Boolean)
    pub enable_decoupled_gliding_default: bool,
    /// `enableDecoupledGlidingCoreOff` (Boolean)
    pub enable_decoupled_gliding_core_off: bool,
    /// `allowDisablingIFCSCore` (Boolean)
    pub allow_disabling_ifcscore: bool,
    /// `cruiseModeOnByDefault` (Boolean)
    pub cruise_mode_on_by_default: bool,
    /// `physicsDamping` (Class)
    pub physics_damping: Option<Handle<SIFCSGameModePhysicsDamping>>,
    /// `legacyIncludeWindInAerodynamics` (Boolean)
    pub legacy_include_wind_in_aerodynamics: bool,
    /// `legacyMaxAcceptedWindSpeed` (Single)
    pub legacy_max_accepted_wind_speed: f32,
}

impl Pooled for SIFCSGameModeParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools.ifcs.sifcsgame_mode_params
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools.ifcs.sifcsgame_mode_params
    }
}

impl<'a> Extract<'a> for SIFCSGameModeParams {
    const TYPE_NAME: &'static str = "SIFCSGameModeParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            enable_new_model: inst.get_bool("enableNewModel").unwrap_or_default(),
            enforce_legacy_esp_for_new_model: inst
                .get_bool("enforceLegacyEspForNewModel")
                .unwrap_or_default(),
            enable_decoupled_gliding_default: inst
                .get_bool("enableDecoupledGlidingDefault")
                .unwrap_or_default(),
            enable_decoupled_gliding_core_off: inst
                .get_bool("enableDecoupledGlidingCoreOff")
                .unwrap_or_default(),
            allow_disabling_ifcscore: inst.get_bool("allowDisablingIFCSCore").unwrap_or_default(),
            cruise_mode_on_by_default: inst.get_bool("cruiseModeOnByDefault").unwrap_or_default(),
            physics_damping: match inst.get("physicsDamping") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<SIFCSGameModePhysicsDamping>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
            legacy_include_wind_in_aerodynamics: inst
                .get_bool("legacyIncludeWindInAerodynamics")
                .unwrap_or_default(),
            legacy_max_accepted_wind_speed: inst
                .get_f32("legacyMaxAcceptedWindSpeed")
                .unwrap_or_default(),
        }
    }
}
