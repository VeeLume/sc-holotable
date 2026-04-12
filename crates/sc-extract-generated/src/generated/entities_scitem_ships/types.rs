// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `entities-scitem-ships`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SIFCSModifierNumber`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSModifierNumber {
    /// `value` (Single)
    #[serde(default)]
    pub value: f32,
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for SIFCSModifierNumber {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sifcsmodifier_number }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sifcsmodifier_number }
}

impl<'a> Extract<'a> for SIFCSModifierNumber {
    const TYPE_NAME: &'static str = "SIFCSModifierNumber";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            value: inst.get_f32("value").unwrap_or_default(),
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSModifierVector`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSModifierVector {
    /// `value` (Class)
    #[serde(default)]
    pub value: Option<Handle<Vec3>>,
    /// `type` (EnumChoice)
    #[serde(default)]
    pub r#type: String,
}

impl Pooled for SIFCSModifierVector {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sifcsmodifier_vector }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sifcsmodifier_vector }
}

impl<'a> Extract<'a> for SIFCSModifierVector {
    const TYPE_NAME: &'static str = "SIFCSModifierVector";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            value: match inst.get("value") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            r#type: inst.get_str("type").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SIFCSModifiersLegacy`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SIFCSModifiersLegacy {
    /// `numbers` (Class)
    #[serde(default)]
    pub numbers: Option<Handle<SIFCSModifierNumber>>,
    /// `vectors` (Class)
    #[serde(default)]
    pub vectors: Option<Handle<SIFCSModifierVector>>,
}

impl Pooled for SIFCSModifiersLegacy {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sifcsmodifiers_legacy }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sifcsmodifiers_legacy }
}

impl<'a> Extract<'a> for SIFCSModifiersLegacy {
    const TYPE_NAME: &'static str = "SIFCSModifiersLegacy";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            numbers: match inst.get("numbers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIFCSModifierNumber>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIFCSModifierNumber>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            vectors: match inst.get("vectors") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SIFCSModifierVector>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SIFCSModifierVector>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SJumpDriveFlightRotationParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightRotationParams {
    /// `pitchYawLimiterType` (EnumChoice)
    #[serde(default)]
    pub pitch_yaw_limiter_type: String,
    /// `maxAngularVelocity` (Class)
    #[serde(default)]
    pub max_angular_velocity: Option<Handle<Vec3>>,
    /// `maxAngularAccelerationPositive` (Class)
    #[serde(default)]
    pub max_angular_acceleration_positive: Option<Handle<Vec3>>,
    /// `maxAngularAccelerationNegative` (Class)
    #[serde(default)]
    pub max_angular_acceleration_negative: Option<Handle<Vec3>>,
    /// `angularAccelerationDecay` (Single)
    #[serde(default)]
    pub angular_acceleration_decay: f32,
    /// `useAngularInputs` (Boolean)
    #[serde(default)]
    pub use_angular_inputs: bool,
    /// `allowUsingBoost` (Boolean)
    #[serde(default)]
    pub allow_using_boost: bool,
}

impl Pooled for SJumpDriveFlightRotationParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sjump_drive_flight_rotation_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sjump_drive_flight_rotation_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightRotationParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightRotationParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pitch_yaw_limiter_type: inst.get_str("pitchYawLimiterType").map(String::from).unwrap_or_default(),
            max_angular_velocity: match inst.get("maxAngularVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_angular_acceleration_positive: match inst.get("maxAngularAccelerationPositive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_angular_acceleration_negative: match inst.get("maxAngularAccelerationNegative") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angular_acceleration_decay: inst.get_f32("angularAccelerationDecay").unwrap_or_default(),
            use_angular_inputs: inst.get_bool("useAngularInputs").unwrap_or_default(),
            allow_using_boost: inst.get_bool("allowUsingBoost").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpDriveFlightLinearParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightLinearParams {
    /// `linearLimiterType` (EnumChoice)
    #[serde(default)]
    pub linear_limiter_type: String,
    /// `linearAccelerationPositive` (Class)
    #[serde(default)]
    pub linear_acceleration_positive: Option<Handle<Vec3>>,
    /// `linearAccelerationNegative` (Class)
    #[serde(default)]
    pub linear_acceleration_negative: Option<Handle<Vec3>>,
    /// `allowUsingBoost` (Boolean)
    #[serde(default)]
    pub allow_using_boost: bool,
    /// `maxSpeed` (Single)
    #[serde(default)]
    pub max_speed: f32,
    /// `maxStrafeInputs` (Single)
    #[serde(default)]
    pub max_strafe_inputs: f32,
    /// `linearAccelDecay` (Single)
    #[serde(default)]
    pub linear_accel_decay: f32,
    /// `useLinearInputs` (Boolean)
    #[serde(default)]
    pub use_linear_inputs: bool,
}

impl Pooled for SJumpDriveFlightLinearParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sjump_drive_flight_linear_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sjump_drive_flight_linear_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightLinearParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightLinearParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            linear_limiter_type: inst.get_str("linearLimiterType").map(String::from).unwrap_or_default(),
            linear_acceleration_positive: match inst.get("linearAccelerationPositive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear_acceleration_negative: match inst.get("linearAccelerationNegative") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            allow_using_boost: inst.get_bool("allowUsingBoost").unwrap_or_default(),
            max_speed: inst.get_f32("maxSpeed").unwrap_or_default(),
            max_strafe_inputs: inst.get_f32("maxStrafeInputs").unwrap_or_default(),
            linear_accel_decay: inst.get_f32("linearAccelDecay").unwrap_or_default(),
            use_linear_inputs: inst.get_bool("useLinearInputs").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpDriveFlightSteeringParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightSteeringParams {
    /// `maxAcceleration` (Single)
    #[serde(default)]
    pub max_acceleration: f32,
    /// `modifierCurve` (Class)
    #[serde(default)]
    pub modifier_curve: Option<Handle<BezierCurve>>,
    /// `maxAccelerationBoost` (Single)
    #[serde(default)]
    pub max_acceleration_boost: f32,
    /// `modifierCurveBoost` (Class)
    #[serde(default)]
    pub modifier_curve_boost: Option<Handle<BezierCurve>>,
    /// `allowUsingBoost` (Boolean)
    #[serde(default)]
    pub allow_using_boost: bool,
    /// `useSteeringParams` (Boolean)
    #[serde(default)]
    pub use_steering_params: bool,
}

impl Pooled for SJumpDriveFlightSteeringParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sjump_drive_flight_steering_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sjump_drive_flight_steering_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightSteeringParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightSteeringParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_acceleration: inst.get_f32("maxAcceleration").unwrap_or_default(),
            modifier_curve: match inst.get("modifierCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_acceleration_boost: inst.get_f32("maxAccelerationBoost").unwrap_or_default(),
            modifier_curve_boost: match inst.get("modifierCurveBoost") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            allow_using_boost: inst.get_bool("allowUsingBoost").unwrap_or_default(),
            use_steering_params: inst.get_bool("useSteeringParams").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpDriveFlightTurbulenceNoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightTurbulenceNoiseParams {
    /// `frequencies` (Class)
    #[serde(default)]
    pub frequencies: Option<Handle<Vec3>>,
    /// `multiplier` (Single)
    #[serde(default)]
    pub multiplier: f32,
    /// `hurstIndex` (Single)
    #[serde(default)]
    pub hurst_index: f32,
    /// `threshold` (Single)
    #[serde(default)]
    pub threshold: f32,
}

impl Pooled for SJumpDriveFlightTurbulenceNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sjump_drive_flight_turbulence_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sjump_drive_flight_turbulence_noise_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightTurbulenceNoiseParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightTurbulenceNoiseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            frequencies: match inst.get("frequencies") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            multiplier: inst.get_f32("multiplier").unwrap_or_default(),
            hurst_index: inst.get_f32("hurstIndex").unwrap_or_default(),
            threshold: inst.get_f32("threshold").unwrap_or_default(),
        }
    }
}

/// DCB type: `SJumpDriveFlightTurbulenceParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJumpDriveFlightTurbulenceParams {
    /// `noiseTurbulence` (Class)
    #[serde(default)]
    pub noise_turbulence: Option<Handle<SJumpDriveFlightTurbulenceNoiseParams>>,
    /// `noiseGust` (Class)
    #[serde(default)]
    pub noise_gust: Option<Handle<SJumpDriveFlightTurbulenceNoiseParams>>,
    /// `turbulenceByDistanceToSpline` (Class)
    #[serde(default)]
    pub turbulence_by_distance_to_spline: Option<Handle<BezierCurve>>,
    /// `gustByDistanceToSpline` (Class)
    #[serde(default)]
    pub gust_by_distance_to_spline: Option<Handle<BezierCurve>>,
    /// `pitchYawLimiterType` (EnumChoice)
    #[serde(default)]
    pub pitch_yaw_limiter_type: String,
    /// `maxAngularVelocity` (Class)
    #[serde(default)]
    pub max_angular_velocity: Option<Handle<Vec3>>,
    /// `maxAngularAcceleration` (Class)
    #[serde(default)]
    pub max_angular_acceleration: Option<Handle<Vec3>>,
    /// `useTurbulenceParams` (Boolean)
    #[serde(default)]
    pub use_turbulence_params: bool,
    /// `useGustParams` (Boolean)
    #[serde(default)]
    pub use_gust_params: bool,
}

impl Pooled for SJumpDriveFlightTurbulenceParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.sjump_drive_flight_turbulence_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.sjump_drive_flight_turbulence_params }
}

impl<'a> Extract<'a> for SJumpDriveFlightTurbulenceParams {
    const TYPE_NAME: &'static str = "SJumpDriveFlightTurbulenceParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            noise_turbulence: match inst.get("noiseTurbulence") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise_gust: match inst.get("noiseGust") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turbulence_by_distance_to_spline: match inst.get("turbulenceByDistanceToSpline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            gust_by_distance_to_spline: match inst.get("gustByDistanceToSpline") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitch_yaw_limiter_type: inst.get_str("pitchYawLimiterType").map(String::from).unwrap_or_default(),
            max_angular_velocity: match inst.get("maxAngularVelocity") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_angular_acceleration: match inst.get("maxAngularAcceleration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            use_turbulence_params: inst.get_bool("useTurbulenceParams").unwrap_or_default(),
            use_gust_params: inst.get_bool("useGustParams").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpDriveFlightParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpDriveFlightParams {
    /// `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<SJumpDriveFlightRotationParams>>,
    /// `linear` (Class)
    #[serde(default)]
    pub linear: Option<Handle<SJumpDriveFlightLinearParams>>,
    /// `steering` (Class)
    #[serde(default)]
    pub steering: Option<Handle<SJumpDriveFlightSteeringParams>>,
    /// `afterburner` (Class)
    #[serde(default)]
    pub afterburner: Option<Handle<SVehicleAfterburnerParams>>,
    /// `turbulence` (Class)
    #[serde(default)]
    pub turbulence: Option<Handle<SJumpDriveFlightTurbulenceParams>>,
    /// `respoolTime` (Single)
    #[serde(default)]
    pub respool_time: f32,
    /// `exitRecoverySpeed` (Single)
    #[serde(default)]
    pub exit_recovery_speed: f32,
}

impl Pooled for JumpDriveFlightParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.jump_drive_flight_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.jump_drive_flight_params }
}

impl<'a> Extract<'a> for JumpDriveFlightParams {
    const TYPE_NAME: &'static str = "JumpDriveFlightParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightRotationParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightRotationParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear: match inst.get("linear") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightLinearParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightLinearParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            steering: match inst.get("steering") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightSteeringParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightSteeringParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            afterburner: match inst.get("afterburner") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVehicleAfterburnerParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVehicleAfterburnerParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            turbulence: match inst.get("turbulence") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SJumpDriveFlightTurbulenceParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            respool_time: inst.get_f32("respoolTime").unwrap_or_default(),
            exit_recovery_speed: inst.get_f32("exitRecoverySpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `JumpTunnelForcesParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpTunnelForcesParams {
    /// `angularAccelerationLimit` (Single)
    #[serde(default)]
    pub angular_acceleration_limit: f32,
    /// `angularAccelerationDecay` (Single)
    #[serde(default)]
    pub angular_acceleration_decay: f32,
    /// `angularSpeedLimit` (Single)
    #[serde(default)]
    pub angular_speed_limit: f32,
    /// `angularCorrectionForceCurve` (Class)
    #[serde(default)]
    pub angular_correction_force_curve: Option<Handle<BezierCurve>>,
    /// `useAngularForces` (Boolean)
    #[serde(default)]
    pub use_angular_forces: bool,
    /// `linearAccelerationLimit` (Single)
    #[serde(default)]
    pub linear_acceleration_limit: f32,
    /// `linearAccelerationDecay` (Single)
    #[serde(default)]
    pub linear_acceleration_decay: f32,
    /// `linearSpeedLimit` (Single)
    #[serde(default)]
    pub linear_speed_limit: f32,
    /// `useWallRepelForces` (Boolean)
    #[serde(default)]
    pub use_wall_repel_forces: bool,
}

impl Pooled for JumpTunnelForcesParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.jump_tunnel_forces_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.jump_tunnel_forces_params }
}

impl<'a> Extract<'a> for JumpTunnelForcesParams {
    const TYPE_NAME: &'static str = "JumpTunnelForcesParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            angular_acceleration_limit: inst.get_f32("angularAccelerationLimit").unwrap_or_default(),
            angular_acceleration_decay: inst.get_f32("angularAccelerationDecay").unwrap_or_default(),
            angular_speed_limit: inst.get_f32("angularSpeedLimit").unwrap_or_default(),
            angular_correction_force_curve: match inst.get("angularCorrectionForceCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            use_angular_forces: inst.get_bool("useAngularForces").unwrap_or_default(),
            linear_acceleration_limit: inst.get_f32("linearAccelerationLimit").unwrap_or_default(),
            linear_acceleration_decay: inst.get_f32("linearAccelerationDecay").unwrap_or_default(),
            linear_speed_limit: inst.get_f32("linearSpeedLimit").unwrap_or_default(),
            use_wall_repel_forces: inst.get_bool("useWallRepelForces").unwrap_or_default(),
        }
    }
}

/// DCB type: `VehicleLandingGearSpring`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLandingGearSpring {
    /// `lengthTarget` (Single)
    #[serde(default)]
    pub length_target: f32,
    /// `lengthMin` (Single)
    #[serde(default)]
    pub length_min: f32,
    /// `lengthMax` (Single)
    #[serde(default)]
    pub length_max: f32,
    /// `stiffness` (Single)
    #[serde(default)]
    pub stiffness: f32,
    /// `damping` (Single)
    #[serde(default)]
    pub damping: f32,
    /// `springBone` (String)
    #[serde(default)]
    pub spring_bone: String,
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// `rotationalOffset` (Class)
    #[serde(default)]
    pub rotational_offset: Option<Handle<Vec3>>,
}

impl Pooled for VehicleLandingGearSpring {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.vehicle_landing_gear_spring }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.vehicle_landing_gear_spring }
}

impl<'a> Extract<'a> for VehicleLandingGearSpring {
    const TYPE_NAME: &'static str = "VehicleLandingGearSpring";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            length_target: inst.get_f32("lengthTarget").unwrap_or_default(),
            length_min: inst.get_f32("lengthMin").unwrap_or_default(),
            length_max: inst.get_f32("lengthMax").unwrap_or_default(),
            stiffness: inst.get_f32("stiffness").unwrap_or_default(),
            damping: inst.get_f32("damping").unwrap_or_default(),
            spring_bone: inst.get_str("springBone").map(String::from).unwrap_or_default(),
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotational_offset: match inst.get("rotationalOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCSeatHeadPosAdjustSetup`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCSeatHeadPosAdjustSetup {
    /// `fpDesiredHeadPosition` (Class)
    #[serde(default)]
    pub fp_desired_head_position: Option<Handle<Vec3>>,
}

impl Pooled for SCSeatHeadPosAdjustSetup {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.scseat_head_pos_adjust_setup }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.scseat_head_pos_adjust_setup }
}

impl<'a> Extract<'a> for SCSeatHeadPosAdjustSetup {
    const TYPE_NAME: &'static str = "SCSeatHeadPosAdjustSetup";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fp_desired_head_position: match inst.get("fpDesiredHeadPosition") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SMFDOperatorModeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDOperatorModeConfig {
    /// `leftCastView` (EnumChoice)
    #[serde(default)]
    pub left_cast_view: String,
    /// `rightCastView` (EnumChoice)
    #[serde(default)]
    pub right_cast_view: String,
    /// `primaryMFDScreenView` (EnumChoice)
    #[serde(default)]
    pub primary_mfdscreen_view: String,
    /// `secondaryMFDScreen1View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen1_view: String,
    /// `secondaryMFDScreen2View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen2_view: String,
    /// `secondaryMFDScreen3View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen3_view: String,
    /// `secondaryMFDScreen4View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen4_view: String,
    /// `secondaryMFDScreen5View` (EnumChoice)
    #[serde(default)]
    pub secondary_mfdscreen5_view: String,
}

impl Pooled for SMFDOperatorModeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smfdoperator_mode_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smfdoperator_mode_config }
}

impl<'a> Extract<'a> for SMFDOperatorModeConfig {
    const TYPE_NAME: &'static str = "SMFDOperatorModeConfig";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            left_cast_view: inst.get_str("leftCastView").map(String::from).unwrap_or_default(),
            right_cast_view: inst.get_str("rightCastView").map(String::from).unwrap_or_default(),
            primary_mfdscreen_view: inst.get_str("primaryMFDScreenView").map(String::from).unwrap_or_default(),
            secondary_mfdscreen1_view: inst.get_str("secondaryMFDScreen1View").map(String::from).unwrap_or_default(),
            secondary_mfdscreen2_view: inst.get_str("secondaryMFDScreen2View").map(String::from).unwrap_or_default(),
            secondary_mfdscreen3_view: inst.get_str("secondaryMFDScreen3View").map(String::from).unwrap_or_default(),
            secondary_mfdscreen4_view: inst.get_str("secondaryMFDScreen4View").map(String::from).unwrap_or_default(),
            secondary_mfdscreen5_view: inst.get_str("secondaryMFDScreen5View").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMFDViewException`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDViewException {
    /// `viewToReplace` (EnumChoice)
    #[serde(default)]
    pub view_to_replace: String,
    /// `replacementView` (EnumChoice)
    #[serde(default)]
    pub replacement_view: String,
}

impl Pooled for SMFDViewException {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smfdview_exception }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smfdview_exception }
}

impl<'a> Extract<'a> for SMFDViewException {
    const TYPE_NAME: &'static str = "SMFDViewException";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            view_to_replace: inst.get_str("viewToReplace").map(String::from).unwrap_or_default(),
            replacement_view: inst.get_str("replacementView").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMFDMasterModeViewExceptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDMasterModeViewExceptions {
    /// `viewExceptions` (Class (array))
    #[serde(default)]
    pub view_exceptions: Vec<Handle<SMFDViewException>>,
}

impl Pooled for SMFDMasterModeViewExceptions {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smfdmaster_mode_view_exceptions }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smfdmaster_mode_view_exceptions }
}

impl<'a> Extract<'a> for SMFDMasterModeViewExceptions {
    const TYPE_NAME: &'static str = "SMFDMasterModeViewExceptions";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            view_exceptions: inst.get_array("viewExceptions")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SMFDViewException>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SMFDViewException>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SMFDModeConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDModeConfig {
    /// `defaultConfiguration` (Class)
    #[serde(default)]
    pub default_configuration: Option<Handle<SMFDOperatorModeConfig>>,
    /// `operatorModeViewConfigurations` (Class)
    #[serde(default)]
    pub operator_mode_view_configurations: Option<Handle<SMFDOperatorModeConfig>>,
    /// `operatorModeViewConfigurationsNoCasts` (Class)
    #[serde(default)]
    pub operator_mode_view_configurations_no_casts: Option<Handle<SMFDOperatorModeConfig>>,
    /// `masterModeViewExceptions` (Class)
    #[serde(default)]
    pub master_mode_view_exceptions: Option<Handle<SMFDMasterModeViewExceptions>>,
}

impl Pooled for SMFDModeConfig {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smfdmode_config }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smfdmode_config }
}

impl<'a> Extract<'a> for SMFDModeConfig {
    const TYPE_NAME: &'static str = "SMFDModeConfig";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            default_configuration: match inst.get("defaultConfiguration") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            operator_mode_view_configurations: match inst.get("operatorModeViewConfigurations") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            operator_mode_view_configurations_no_casts: match inst.get("operatorModeViewConfigurationsNoCasts") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMFDOperatorModeConfig>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            master_mode_view_exceptions: match inst.get("masterModeViewExceptions") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMFDMasterModeViewExceptions>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMFDMasterModeViewExceptions>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SManufacturerMFDView`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SManufacturerMFDView {
    /// `manufacturerStyle` (Reference)
    #[serde(default)]
    pub manufacturer_style: Option<CigGuid>,
    /// `canvas` (Reference)
    #[serde(default)]
    pub canvas: Option<CigGuid>,
}

impl Pooled for SManufacturerMFDView {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smanufacturer_mfdview }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smanufacturer_mfdview }
}

impl<'a> Extract<'a> for SManufacturerMFDView {
    const TYPE_NAME: &'static str = "SManufacturerMFDView";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            manufacturer_style: inst.get("manufacturerStyle").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            canvas: inst.get("canvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SMFDView`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDView {
    /// `name` (Locale)
    #[serde(default)]
    pub name: String,
    /// `landscapeCanvas` (Reference)
    #[serde(default)]
    pub landscape_canvas: Option<CigGuid>,
    /// `landscapeCanvasStyleOverride` (Class (array))
    #[serde(default)]
    pub landscape_canvas_style_override: Vec<Handle<SManufacturerMFDView>>,
    /// `castsUsePortrait` (Boolean)
    #[serde(default)]
    pub casts_use_portrait: bool,
    /// `portraitCanvas` (Reference)
    #[serde(default)]
    pub portrait_canvas: Option<CigGuid>,
    /// `portraitCanvasStyleOverride` (Class (array))
    #[serde(default)]
    pub portrait_canvas_style_override: Vec<Handle<SManufacturerMFDView>>,
    /// `viewType` (EnumChoice)
    #[serde(default)]
    pub view_type: String,
    /// `urlpostfix` (String)
    #[serde(default)]
    pub urlpostfix: String,
}

impl Pooled for SMFDView {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smfdview }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smfdview }
}

impl<'a> Extract<'a> for SMFDView {
    const TYPE_NAME: &'static str = "SMFDView";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            name: inst.get_str("name").map(String::from).unwrap_or_default(),
            landscape_canvas: inst.get("landscapeCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            landscape_canvas_style_override: inst.get_array("landscapeCanvasStyleOverride")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SManufacturerMFDView>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SManufacturerMFDView>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            casts_use_portrait: inst.get_bool("castsUsePortrait").unwrap_or_default(),
            portrait_canvas: inst.get("portraitCanvas").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            portrait_canvas_style_override: inst.get_array("portraitCanvasStyleOverride")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SManufacturerMFDView>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SManufacturerMFDView>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            view_type: inst.get_str("viewType").map(String::from).unwrap_or_default(),
            urlpostfix: inst.get_str("urlpostfix").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `SMasterModeMFDViewList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMasterModeMFDViewList {
    /// `MasterModeViews` (Reference)
    #[serde(default)]
    pub master_mode_views: Option<CigGuid>,
}

impl Pooled for SMasterModeMFDViewList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smaster_mode_mfdview_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smaster_mode_mfdview_list }
}

impl<'a> Extract<'a> for SMasterModeMFDViewList {
    const TYPE_NAME: &'static str = "SMasterModeMFDViewList";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            master_mode_views: inst.get("MasterModeViews").and_then(|v| v.as_record_ref()).map(|r| r.guid),
        }
    }
}

/// DCB type: `SMFDViewList`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDViewList {
    /// `views` (Class)
    #[serde(default)]
    pub views: Option<Handle<SMasterModeMFDViewList>>,
}

impl Pooled for SMFDViewList {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smfdview_list }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smfdview_list }
}

impl<'a> Extract<'a> for SMFDViewList {
    const TYPE_NAME: &'static str = "SMFDViewList";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            views: match inst.get("views") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMasterModeMFDViewList>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMasterModeMFDViewList>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SMFDParamsDiagnostics`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMFDParamsDiagnostics {
    /// `healthThresholdDamaged` (Single)
    #[serde(default)]
    pub health_threshold_damaged: f32,
    /// `healthThresholdCritical` (Single)
    #[serde(default)]
    pub health_threshold_critical: f32,
    /// `excludedItemTypes` (Class (array))
    #[serde(default)]
    pub excluded_item_types: Vec<Handle<SItemPortDefTypes>>,
    /// `includedItemTypes` (Class (array))
    #[serde(default)]
    pub included_item_types: Vec<Handle<SItemPortDefTypes>>,
    /// `typeIcons` (String)
    #[serde(default)]
    pub type_icons: String,
}

impl Pooled for SMFDParamsDiagnostics {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.smfdparams_diagnostics }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.smfdparams_diagnostics }
}

impl<'a> Extract<'a> for SMFDParamsDiagnostics {
    const TYPE_NAME: &'static str = "SMFDParamsDiagnostics";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            health_threshold_damaged: inst.get_f32("healthThresholdDamaged").unwrap_or_default(),
            health_threshold_critical: inst.get_f32("healthThresholdCritical").unwrap_or_default(),
            excluded_item_types: inst.get_array("excludedItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemPortDefTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortDefTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            included_item_types: inst.get_array("includedItemTypes")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SItemPortDefTypes>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SItemPortDefTypes>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            type_icons: inst.get_str("typeIcons").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `WeaponAimableAnglesDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponAimableAnglesDef {
    /// `maxNudgeAngle` (Single)
    #[serde(default)]
    pub max_nudge_angle: f32,
    /// `innerThresholdAngle` (Single)
    #[serde(default)]
    pub inner_threshold_angle: f32,
    /// `outerThresholdAngle` (Single)
    #[serde(default)]
    pub outer_threshold_angle: f32,
    /// `closeDistanceMaxRange` (Single)
    #[serde(default)]
    pub close_distance_max_range: f32,
    /// `closeDistanceMinRange` (Single)
    #[serde(default)]
    pub close_distance_min_range: f32,
    /// `closeDistanceOuterAngle` (Single)
    #[serde(default)]
    pub close_distance_outer_angle: f32,
    /// `closeDistanceInnerAngle` (Single)
    #[serde(default)]
    pub close_distance_inner_angle: f32,
    /// `assistNudgeMapping` (Class)
    #[serde(default)]
    pub assist_nudge_mapping: Option<Handle<BezierCurve>>,
}

impl Pooled for WeaponAimableAnglesDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.weapon_aimable_angles_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.weapon_aimable_angles_def }
}

impl<'a> Extract<'a> for WeaponAimableAnglesDef {
    const TYPE_NAME: &'static str = "WeaponAimableAnglesDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max_nudge_angle: inst.get_f32("maxNudgeAngle").unwrap_or_default(),
            inner_threshold_angle: inst.get_f32("innerThresholdAngle").unwrap_or_default(),
            outer_threshold_angle: inst.get_f32("outerThresholdAngle").unwrap_or_default(),
            close_distance_max_range: inst.get_f32("closeDistanceMaxRange").unwrap_or_default(),
            close_distance_min_range: inst.get_f32("closeDistanceMinRange").unwrap_or_default(),
            close_distance_outer_angle: inst.get_f32("closeDistanceOuterAngle").unwrap_or_default(),
            close_distance_inner_angle: inst.get_f32("closeDistanceInnerAngle").unwrap_or_default(),
            assist_nudge_mapping: match inst.get("assistNudgeMapping") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeaponGimbalModeModifierDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponGimbalModeModifierDef {
    /// `weaponGimbalModeModifiers` (StrongPointer)
    #[serde(default)]
    pub weapon_gimbal_mode_modifiers: Option<Handle<SWeaponModifierParams>>,
}

impl Pooled for WeaponGimbalModeModifierDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.weapon_gimbal_mode_modifier_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.weapon_gimbal_mode_modifier_def }
}

impl<'a> Extract<'a> for WeaponGimbalModeModifierDef {
    const TYPE_NAME: &'static str = "WeaponGimbalModeModifierDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weapon_gimbal_mode_modifiers: match inst.get("weaponGimbalModeModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponModifierParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponModifierParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `VehicleLandingGear`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLandingGear {
    /// `scopeContext` (String)
    #[serde(default)]
    pub scope_context: String,
    /// `vehicleScopeContext` (String)
    #[serde(default)]
    pub vehicle_scope_context: String,
    /// `bone` (String)
    #[serde(default)]
    pub bone: String,
    /// `mass` (Single)
    #[serde(default)]
    pub mass: f32,
    /// `alwaysVisible` (Boolean)
    #[serde(default)]
    pub always_visible: bool,
    /// `geometry` (Class)
    #[serde(default)]
    pub geometry: Option<Handle<GlobalResourceGeometry>>,
    /// `spring` (StrongPointer)
    #[serde(default)]
    pub spring: Option<Handle<VehicleLandingGearSpring>>,
}

impl Pooled for VehicleLandingGear {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.vehicle_landing_gear }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.vehicle_landing_gear }
}

impl<'a> Extract<'a> for VehicleLandingGear {
    const TYPE_NAME: &'static str = "VehicleLandingGear";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            scope_context: inst.get_str("scopeContext").map(String::from).unwrap_or_default(),
            vehicle_scope_context: inst.get_str("vehicleScopeContext").map(String::from).unwrap_or_default(),
            bone: inst.get_str("bone").map(String::from).unwrap_or_default(),
            mass: inst.get_f32("mass").unwrap_or_default(),
            always_visible: inst.get_bool("alwaysVisible").unwrap_or_default(),
            geometry: match inst.get("geometry") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<GlobalResourceGeometry>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<GlobalResourceGeometry>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            spring: match inst.get("spring") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<VehicleLandingGearSpring>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<VehicleLandingGearSpring>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `VehicleLandingGearSystem`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLandingGearSystem {
    /// `fragment` (String)
    #[serde(default)]
    pub fragment: String,
    /// `fragmentDeploy` (String)
    #[serde(default)]
    pub fragment_deploy: String,
    /// `fragmentRetract` (String)
    #[serde(default)]
    pub fragment_retract: String,
    /// `fragmentCompress` (String)
    #[serde(default)]
    pub fragment_compress: String,
    /// `altitudeToExtraGears` (Single)
    #[serde(default)]
    pub altitude_to_extra_gears: f32,
    /// `gears` (StrongPointer (array))
    #[serde(default)]
    pub gears: Vec<Handle<VehicleLandingGear>>,
}

impl Pooled for VehicleLandingGearSystem {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.vehicle_landing_gear_system }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.vehicle_landing_gear_system }
}

impl<'a> Extract<'a> for VehicleLandingGearSystem {
    const TYPE_NAME: &'static str = "VehicleLandingGearSystem";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            fragment: inst.get_str("fragment").map(String::from).unwrap_or_default(),
            fragment_deploy: inst.get_str("fragmentDeploy").map(String::from).unwrap_or_default(),
            fragment_retract: inst.get_str("fragmentRetract").map(String::from).unwrap_or_default(),
            fragment_compress: inst.get_str("fragmentCompress").map(String::from).unwrap_or_default(),
            altitude_to_extra_gears: inst.get_f32("altitudeToExtraGears").unwrap_or_default(),
            gears: inst.get_array("gears")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<VehicleLandingGear>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<VehicleLandingGear>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SVehicleAfterburnerParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVehicleAfterburnerParams {
    /// `preDelayTime` (Single)
    #[serde(default)]
    pub pre_delay_time: f32,
    /// `rampUpTime` (Single)
    #[serde(default)]
    pub ramp_up_time: f32,
    /// `rampDownTime` (Single)
    #[serde(default)]
    pub ramp_down_time: f32,
    /// `thresholdRatio` (Single)
    #[serde(default)]
    pub threshold_ratio: f32,
    /// `capacity` (Single)
    #[serde(default)]
    pub capacity: f32,
    /// `costPerSec` (Single)
    #[serde(default)]
    pub cost_per_sec: f32,
    /// `assignmentCostModifier` (Class)
    #[serde(default)]
    pub assignment_cost_modifier: Option<Handle<BezierCurve>>,
    /// `regenPerSec` (Single)
    #[serde(default)]
    pub regen_per_sec: f32,
    /// `assignmentRegenModifier` (Class)
    #[serde(default)]
    pub assignment_regen_modifier: Option<Handle<BezierCurve>>,
    /// `regenDelayAfterUse` (Single)
    #[serde(default)]
    pub regen_delay_after_use: f32,
    /// `linearAccelMultiplierPositive` (Class)
    #[serde(default)]
    pub linear_accel_multiplier_positive: Option<Handle<Vec3>>,
    /// `linearAccelMultiplierNegative` (Class)
    #[serde(default)]
    pub linear_accel_multiplier_negative: Option<Handle<Vec3>>,
    /// `assignmentModifierLin` (Class)
    #[serde(default)]
    pub assignment_modifier_lin: Option<Handle<BezierCurve>>,
    /// `angularAccelMultiplier` (Class)
    #[serde(default)]
    pub angular_accel_multiplier: Option<Handle<Vec3>>,
    /// `assignmentModifierAng` (Class)
    #[serde(default)]
    pub assignment_modifier_ang: Option<Handle<BezierCurve>>,
    /// `allowTriggerDuringRampdown` (Boolean)
    #[serde(default)]
    pub allow_trigger_during_rampdown: bool,
    /// `enableAntiSpam` (Boolean)
    #[serde(default)]
    pub enable_anti_spam: bool,
    /// `startAtFullCapacity` (Boolean)
    #[serde(default)]
    pub start_at_full_capacity: bool,
}

impl Pooled for SVehicleAfterburnerParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.entities_scitem_ships.svehicle_afterburner_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.entities_scitem_ships.svehicle_afterburner_params }
}

impl<'a> Extract<'a> for SVehicleAfterburnerParams {
    const TYPE_NAME: &'static str = "SVehicleAfterburnerParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            pre_delay_time: inst.get_f32("preDelayTime").unwrap_or_default(),
            ramp_up_time: inst.get_f32("rampUpTime").unwrap_or_default(),
            ramp_down_time: inst.get_f32("rampDownTime").unwrap_or_default(),
            threshold_ratio: inst.get_f32("thresholdRatio").unwrap_or_default(),
            capacity: inst.get_f32("capacity").unwrap_or_default(),
            cost_per_sec: inst.get_f32("costPerSec").unwrap_or_default(),
            assignment_cost_modifier: match inst.get("assignmentCostModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            regen_per_sec: inst.get_f32("regenPerSec").unwrap_or_default(),
            assignment_regen_modifier: match inst.get("assignmentRegenModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            regen_delay_after_use: inst.get_f32("regenDelayAfterUse").unwrap_or_default(),
            linear_accel_multiplier_positive: match inst.get("linearAccelMultiplierPositive") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            linear_accel_multiplier_negative: match inst.get("linearAccelMultiplierNegative") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            assignment_modifier_lin: match inst.get("assignmentModifierLin") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            angular_accel_multiplier: match inst.get("angularAccelMultiplier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            assignment_modifier_ang: match inst.get("assignmentModifierAng") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            allow_trigger_during_rampdown: inst.get_bool("allowTriggerDuringRampdown").unwrap_or_default(),
            enable_anti_spam: inst.get_bool("enableAntiSpam").unwrap_or_default(),
            start_at_full_capacity: inst.get_bool("startAtFullCapacity").unwrap_or_default(),
        }
    }
}

