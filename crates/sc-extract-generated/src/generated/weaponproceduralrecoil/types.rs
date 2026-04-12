// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `weaponproceduralrecoil`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `SXYZCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SXYZCurves {
    /// `xCurve` (Class)
    #[serde(default)]
    pub x_curve: Option<Handle<BezierCurve>>,
    /// `yCurve` (Class)
    #[serde(default)]
    pub y_curve: Option<Handle<BezierCurve>>,
    /// `zCurve` (Class)
    #[serde(default)]
    pub z_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SXYZCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sxyzcurves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sxyzcurves }
}

impl<'a> Extract<'a> for SXYZCurves {
    const TYPE_NAME: &'static str = "SXYZCurves";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            x_curve: match inst.get("xCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            y_curve: match inst.get("yCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            z_curve: match inst.get("zCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SCurve`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCurve {
    /// `curve` (Class)
    #[serde(default)]
    pub curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SCurve {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.scurve }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.scurve }
}

impl<'a> Extract<'a> for SCurve {
    const TYPE_NAME: &'static str = "SCurve";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            curve: match inst.get("curve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SXYZCurvesArrays`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SXYZCurvesArrays {
    /// `xCurves` (Class (array))
    #[serde(default)]
    pub x_curves: Vec<Handle<SCurve>>,
    /// `yCurves` (Class (array))
    #[serde(default)]
    pub y_curves: Vec<Handle<SCurve>>,
    /// `zCurves` (Class (array))
    #[serde(default)]
    pub z_curves: Vec<Handle<SCurve>>,
}

impl Pooled for SXYZCurvesArrays {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sxyzcurves_arrays }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sxyzcurves_arrays }
}

impl<'a> Extract<'a> for SXYZCurvesArrays {
    const TYPE_NAME: &'static str = "SXYZCurvesArrays";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            x_curves: inst.get_array("xCurves")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            y_curves: inst.get_array("yCurves")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
            z_curves: inst.get_array("zCurves")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<SCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<SCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `SHandsRecoilCurveNoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHandsRecoilCurveNoiseParams {
    /// `xNoise` (Single)
    #[serde(default)]
    pub x_noise: f32,
    /// `canInvertXCurve` (Boolean)
    #[serde(default)]
    pub can_invert_xcurve: bool,
    /// `yNoise` (Single)
    #[serde(default)]
    pub y_noise: f32,
    /// `canInvertYCurve` (Boolean)
    #[serde(default)]
    pub can_invert_ycurve: bool,
    /// `zNoise` (Single)
    #[serde(default)]
    pub z_noise: f32,
    /// `canInvertZCurve` (Boolean)
    #[serde(default)]
    pub can_invert_zcurve: bool,
}

impl Pooled for SHandsRecoilCurveNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.shands_recoil_curve_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.shands_recoil_curve_noise_params }
}

impl<'a> Extract<'a> for SHandsRecoilCurveNoiseParams {
    const TYPE_NAME: &'static str = "SHandsRecoilCurveNoiseParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x_noise: inst.get_f32("xNoise").unwrap_or_default(),
            can_invert_xcurve: inst.get_bool("canInvertXCurve").unwrap_or_default(),
            y_noise: inst.get_f32("yNoise").unwrap_or_default(),
            can_invert_ycurve: inst.get_bool("canInvertYCurve").unwrap_or_default(),
            z_noise: inst.get_f32("zNoise").unwrap_or_default(),
            can_invert_zcurve: inst.get_bool("canInvertZCurve").unwrap_or_default(),
        }
    }
}

/// DCB type: `SXYZCurvesWithMaxValues`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SXYZCurvesWithMaxValues {
    /// `xMaxValue` (Single)
    #[serde(default)]
    pub x_max_value: f32,
    /// `yMaxValue` (Single)
    #[serde(default)]
    pub y_max_value: f32,
    /// `zMaxValue` (Single)
    #[serde(default)]
    pub z_max_value: f32,
    /// `minLimits` (Class)
    #[serde(default)]
    pub min_limits: Option<Handle<Vec3>>,
    /// `maxLimits` (Class)
    #[serde(default)]
    pub max_limits: Option<Handle<Vec3>>,
    /// `curves` (StrongPointer)
    #[serde(default)]
    pub curves: Option<Handle<SXYZCurvesArrays>>,
    /// `noiseParams` (Class)
    #[serde(default)]
    pub noise_params: Option<Handle<SHandsRecoilCurveNoiseParams>>,
}

impl Pooled for SXYZCurvesWithMaxValues {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sxyzcurves_with_max_values }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sxyzcurves_with_max_values }
}

impl<'a> Extract<'a> for SXYZCurvesWithMaxValues {
    const TYPE_NAME: &'static str = "SXYZCurvesWithMaxValues";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            x_max_value: inst.get_f32("xMaxValue").unwrap_or_default(),
            y_max_value: inst.get_f32("yMaxValue").unwrap_or_default(),
            z_max_value: inst.get_f32("zMaxValue").unwrap_or_default(),
            min_limits: match inst.get("minLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_limits: match inst.get("maxLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            curves: match inst.get("curves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurvesArrays>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurvesArrays>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise_params: match inst.get("noiseParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHandsRecoilCurveNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHandsRecoilCurveNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SDecayCurveMaxValueParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDecayCurveMaxValueParams {
    /// `maxValue` (Single)
    #[serde(default)]
    pub max_value: f32,
    /// `minScalingFactor` (Single)
    #[serde(default)]
    pub min_scaling_factor: f32,
    /// `useDecayScaling` (Boolean)
    #[serde(default)]
    pub use_decay_scaling: bool,
    /// `useWeaponOrientation` (Boolean)
    #[serde(default)]
    pub use_weapon_orientation: bool,
}

impl Pooled for SDecayCurveMaxValueParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sdecay_curve_max_value_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sdecay_curve_max_value_params }
}

impl<'a> Extract<'a> for SDecayCurveMaxValueParams {
    const TYPE_NAME: &'static str = "SDecayCurveMaxValueParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            max_value: inst.get_f32("maxValue").unwrap_or_default(),
            min_scaling_factor: inst.get_f32("minScalingFactor").unwrap_or_default(),
            use_decay_scaling: inst.get_bool("useDecayScaling").unwrap_or_default(),
            use_weapon_orientation: inst.get_bool("useWeaponOrientation").unwrap_or_default(),
        }
    }
}

/// DCB type: `SDecayCurveMaxValues`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDecayCurveMaxValues {
    /// `xMaxValueParams` (Class)
    #[serde(default)]
    pub x_max_value_params: Option<Handle<SDecayCurveMaxValueParams>>,
    /// `yMaxValueParams` (Class)
    #[serde(default)]
    pub y_max_value_params: Option<Handle<SDecayCurveMaxValueParams>>,
    /// `zMaxValueParams` (Class)
    #[serde(default)]
    pub z_max_value_params: Option<Handle<SDecayCurveMaxValueParams>>,
}

impl Pooled for SDecayCurveMaxValues {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sdecay_curve_max_values }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sdecay_curve_max_values }
}

impl<'a> Extract<'a> for SDecayCurveMaxValues {
    const TYPE_NAME: &'static str = "SDecayCurveMaxValues";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            x_max_value_params: match inst.get("xMaxValueParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            y_max_value_params: match inst.get("yMaxValueParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            z_max_value_params: match inst.get("zMaxValueParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayCurveMaxValueParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SDecayTimesAndCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDecayTimesAndCurves {
    /// `decayTimeMultipliers` (Class)
    #[serde(default)]
    pub decay_time_multipliers: Option<Handle<Vec3>>,
    /// `decayCurveMaxValues` (Class)
    #[serde(default)]
    pub decay_curve_max_values: Option<Handle<SDecayCurveMaxValues>>,
    /// `decayCurves` (StrongPointer)
    #[serde(default)]
    pub decay_curves: Option<Handle<SXYZCurves>>,
}

impl Pooled for SDecayTimesAndCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sdecay_times_and_curves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sdecay_times_and_curves }
}

impl<'a> Extract<'a> for SDecayTimesAndCurves {
    const TYPE_NAME: &'static str = "SDecayTimesAndCurves";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            decay_time_multipliers: match inst.get("decayTimeMultipliers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decay_curve_max_values: match inst.get("decayCurveMaxValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayCurveMaxValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayCurveMaxValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decay_curves: match inst.get("decayCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SHandsRecoilTimeModifier`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHandsRecoilTimeModifier {
    /// `recoilModifierTime` (Single)
    #[serde(default)]
    pub recoil_modifier_time: f32,
    /// `positionMaxValueCurves` (Class)
    #[serde(default)]
    pub position_max_value_curves: Option<Handle<SXYZCurves>>,
    /// `rotationMaxValueCurves` (Class)
    #[serde(default)]
    pub rotation_max_value_curves: Option<Handle<SXYZCurves>>,
}

impl Pooled for SHandsRecoilTimeModifier {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.shands_recoil_time_modifier }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.shands_recoil_time_modifier }
}

impl<'a> Extract<'a> for SHandsRecoilTimeModifier {
    const TYPE_NAME: &'static str = "SHandsRecoilTimeModifier";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            recoil_modifier_time: inst.get_f32("recoilModifierTime").unwrap_or_default(),
            position_max_value_curves: match inst.get("positionMaxValueCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_max_value_curves: match inst.get("rotationMaxValueCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralHandsRecoilCurveConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralHandsRecoilCurveConfigDef {
    /// `totalRecoilTime` (Single)
    #[serde(default)]
    pub total_recoil_time: f32,
    /// `positionRecoilTimeModifiers` (Class)
    #[serde(default)]
    pub position_recoil_time_modifiers: Option<Handle<Vec3>>,
    /// `rotationRecoilTimeModifiers` (Class)
    #[serde(default)]
    pub rotation_recoil_time_modifiers: Option<Handle<Vec3>>,
    /// `limitTransitionTime` (Single)
    #[serde(default)]
    pub limit_transition_time: f32,
    /// `positionCurves` (Class)
    #[serde(default)]
    pub position_curves: Option<Handle<SXYZCurvesWithMaxValues>>,
    /// `rotationCurves` (Class)
    #[serde(default)]
    pub rotation_curves: Option<Handle<SXYZCurvesWithMaxValues>>,
    /// `minDecayTime` (Single)
    #[serde(default)]
    pub min_decay_time: f32,
    /// `maxDecayTime` (Single)
    #[serde(default)]
    pub max_decay_time: f32,
    /// `positionDecay` (Class)
    #[serde(default)]
    pub position_decay: Option<Handle<SDecayTimesAndCurves>>,
    /// `rotationDecay` (Class)
    #[serde(default)]
    pub rotation_decay: Option<Handle<SDecayTimesAndCurves>>,
    /// `rotationOffset` (Class)
    #[serde(default)]
    pub rotation_offset: Option<Handle<Vec3>>,
    /// `timeModifier` (StrongPointer)
    #[serde(default)]
    pub time_modifier: Option<Handle<SHandsRecoilTimeModifier>>,
}

impl Pooled for SWeaponProceduralHandsRecoilCurveConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sweapon_procedural_hands_recoil_curve_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sweapon_procedural_hands_recoil_curve_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralHandsRecoilCurveConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralHandsRecoilCurveConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            total_recoil_time: inst.get_f32("totalRecoilTime").unwrap_or_default(),
            position_recoil_time_modifiers: match inst.get("positionRecoilTimeModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_recoil_time_modifiers: match inst.get("rotationRecoilTimeModifiers") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            limit_transition_time: inst.get_f32("limitTransitionTime").unwrap_or_default(),
            position_curves: match inst.get("positionCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurvesWithMaxValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurvesWithMaxValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_curves: match inst.get("rotationCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SXYZCurvesWithMaxValues>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SXYZCurvesWithMaxValues>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            min_decay_time: inst.get_f32("minDecayTime").unwrap_or_default(),
            max_decay_time: inst.get_f32("maxDecayTime").unwrap_or_default(),
            position_decay: match inst.get("positionDecay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayTimesAndCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayTimesAndCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_decay: match inst.get("rotationDecay") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SDecayTimesAndCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SDecayTimesAndCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_offset: match inst.get("rotationOffset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            time_modifier: match inst.get("timeModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHandsRecoilTimeModifier>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHandsRecoilTimeModifier>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralHandsRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralHandsRecoilConfigDef {
    /// `decay` (Single)
    #[serde(default)]
    pub decay: f32,
    /// `endDecay` (Single)
    #[serde(default)]
    pub end_decay: f32,
    /// `fireRecoilTime` (Single)
    #[serde(default)]
    pub fire_recoil_time: f32,
    /// `fireRecoilStrengthFirst` (Single)
    #[serde(default)]
    pub fire_recoil_strength_first: f32,
    /// `fireRecoilStrength` (Single)
    #[serde(default)]
    pub fire_recoil_strength: f32,
    /// `angleRecoilStrength` (Single)
    #[serde(default)]
    pub angle_recoil_strength: f32,
    /// `useRandomRotation` (Boolean)
    #[serde(default)]
    pub use_random_rotation: bool,
    /// `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Ang3>>,
    /// `randomness` (Single)
    #[serde(default)]
    pub randomness: f32,
    /// `randomnessBackPush` (Single)
    #[serde(default)]
    pub randomness_back_push: f32,
    /// `frontalOscillationRotation` (Single)
    #[serde(default)]
    pub frontal_oscillation_rotation: f32,
    /// `frontalOscillationStrength` (Single)
    #[serde(default)]
    pub frontal_oscillation_strength: f32,
    /// `frontalOscillationDecay` (Single)
    #[serde(default)]
    pub frontal_oscillation_decay: f32,
    /// `frontalOscillationRandomness` (Single)
    #[serde(default)]
    pub frontal_oscillation_randomness: f32,
    /// `curveRecoil` (Class)
    #[serde(default)]
    pub curve_recoil: Option<Handle<SWeaponProceduralHandsRecoilCurveConfigDef>>,
}

impl Pooled for SWeaponProceduralHandsRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sweapon_procedural_hands_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sweapon_procedural_hands_recoil_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralHandsRecoilConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralHandsRecoilConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            decay: inst.get_f32("decay").unwrap_or_default(),
            end_decay: inst.get_f32("endDecay").unwrap_or_default(),
            fire_recoil_time: inst.get_f32("fireRecoilTime").unwrap_or_default(),
            fire_recoil_strength_first: inst.get_f32("fireRecoilStrengthFirst").unwrap_or_default(),
            fire_recoil_strength: inst.get_f32("fireRecoilStrength").unwrap_or_default(),
            angle_recoil_strength: inst.get_f32("angleRecoilStrength").unwrap_or_default(),
            use_random_rotation: inst.get_bool("useRandomRotation").unwrap_or_default(),
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            randomness: inst.get_f32("randomness").unwrap_or_default(),
            randomness_back_push: inst.get_f32("randomnessBackPush").unwrap_or_default(),
            frontal_oscillation_rotation: inst.get_f32("frontalOscillationRotation").unwrap_or_default(),
            frontal_oscillation_strength: inst.get_f32("frontalOscillationStrength").unwrap_or_default(),
            frontal_oscillation_decay: inst.get_f32("frontalOscillationDecay").unwrap_or_default(),
            frontal_oscillation_randomness: inst.get_f32("frontalOscillationRandomness").unwrap_or_default(),
            curve_recoil: match inst.get("curveRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHandsRecoilCurveConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHandsRecoilCurveConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SYawPitchRollCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SYawPitchRollCurves {
    /// `yawCurve` (Class)
    #[serde(default)]
    pub yaw_curve: Option<Handle<BezierCurve>>,
    /// `pitchCurve` (Class)
    #[serde(default)]
    pub pitch_curve: Option<Handle<BezierCurve>>,
    /// `rollCurve` (Class)
    #[serde(default)]
    pub roll_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SYawPitchRollCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.syaw_pitch_roll_curves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.syaw_pitch_roll_curves }
}

impl<'a> Extract<'a> for SYawPitchRollCurves {
    const TYPE_NAME: &'static str = "SYawPitchRollCurves";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            yaw_curve: match inst.get("yawCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pitch_curve: match inst.get("pitchCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            roll_curve: match inst.get("rollCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAimRecoilNoiseCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAimRecoilNoiseCurves {
    /// `yawNoiseMaxValue` (Single)
    #[serde(default)]
    pub yaw_noise_max_value: f32,
    /// `pitchNoiseMaxValue` (Single)
    #[serde(default)]
    pub pitch_noise_max_value: f32,
    /// `rollNoiseMaxValue` (Single)
    #[serde(default)]
    pub roll_noise_max_value: f32,
    /// `yawPitchRollNoiseCurves` (StrongPointer)
    #[serde(default)]
    pub yaw_pitch_roll_noise_curves: Option<Handle<SYawPitchRollCurves>>,
}

impl Pooled for SAimRecoilNoiseCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.saim_recoil_noise_curves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.saim_recoil_noise_curves }
}

impl<'a> Extract<'a> for SAimRecoilNoiseCurves {
    const TYPE_NAME: &'static str = "SAimRecoilNoiseCurves";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            yaw_noise_max_value: inst.get_f32("yawNoiseMaxValue").unwrap_or_default(),
            pitch_noise_max_value: inst.get_f32("pitchNoiseMaxValue").unwrap_or_default(),
            roll_noise_max_value: inst.get_f32("rollNoiseMaxValue").unwrap_or_default(),
            yaw_pitch_roll_noise_curves: match inst.get("yawPitchRollNoiseCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SYawPitchRollCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SYawPitchRollCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralAimRecoilCurveConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralAimRecoilCurveConfigDef {
    /// `yawMaxDegrees` (Single)
    #[serde(default)]
    pub yaw_max_degrees: f32,
    /// `pitchMaxDegrees` (Single)
    #[serde(default)]
    pub pitch_max_degrees: f32,
    /// `rollMaxDegrees` (Single)
    #[serde(default)]
    pub roll_max_degrees: f32,
    /// `maxFireTime` (Single)
    #[serde(default)]
    pub max_fire_time: f32,
    /// `recoilSmoothTime` (Single)
    #[serde(default)]
    pub recoil_smooth_time: f32,
    /// `minLimits` (Class)
    #[serde(default)]
    pub min_limits: Option<Handle<Vec3>>,
    /// `maxLimits` (Class)
    #[serde(default)]
    pub max_limits: Option<Handle<Vec3>>,
    /// `yawPitchRollCurves` (StrongPointer)
    #[serde(default)]
    pub yaw_pitch_roll_curves: Option<Handle<SYawPitchRollCurves>>,
    /// `decayStartTime` (Single)
    #[serde(default)]
    pub decay_start_time: f32,
    /// `minDecayTime` (Single)
    #[serde(default)]
    pub min_decay_time: f32,
    /// `maxDecayTime` (Single)
    #[serde(default)]
    pub max_decay_time: f32,
    /// `yawPitchRollDecayCurves` (StrongPointer)
    #[serde(default)]
    pub yaw_pitch_roll_decay_curves: Option<Handle<SYawPitchRollCurves>>,
    /// `noiseCurves` (Class)
    #[serde(default)]
    pub noise_curves: Option<Handle<SAimRecoilNoiseCurves>>,
}

impl Pooled for SWeaponProceduralAimRecoilCurveConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sweapon_procedural_aim_recoil_curve_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sweapon_procedural_aim_recoil_curve_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralAimRecoilCurveConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralAimRecoilCurveConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            yaw_max_degrees: inst.get_f32("yawMaxDegrees").unwrap_or_default(),
            pitch_max_degrees: inst.get_f32("pitchMaxDegrees").unwrap_or_default(),
            roll_max_degrees: inst.get_f32("rollMaxDegrees").unwrap_or_default(),
            max_fire_time: inst.get_f32("maxFireTime").unwrap_or_default(),
            recoil_smooth_time: inst.get_f32("recoilSmoothTime").unwrap_or_default(),
            min_limits: match inst.get("minLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_limits: match inst.get("maxLimits") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            yaw_pitch_roll_curves: match inst.get("yawPitchRollCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SYawPitchRollCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SYawPitchRollCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            decay_start_time: inst.get_f32("decayStartTime").unwrap_or_default(),
            min_decay_time: inst.get_f32("minDecayTime").unwrap_or_default(),
            max_decay_time: inst.get_f32("maxDecayTime").unwrap_or_default(),
            yaw_pitch_roll_decay_curves: match inst.get("yawPitchRollDecayCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SYawPitchRollCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SYawPitchRollCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise_curves: match inst.get("noiseCurves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAimRecoilNoiseCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAimRecoilNoiseCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralAimRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralAimRecoilConfigDef {
    /// `max` (Class)
    #[serde(default)]
    pub max: Option<Handle<Vec2>>,
    /// `pull_left_percentage` (Single)
    #[serde(default)]
    pub pull_left_percentage: f32,
    /// `shot_kick_first` (Class)
    #[serde(default)]
    pub shot_kick_first: Option<Handle<Vec2>>,
    /// `shot_kick` (Class)
    #[serde(default)]
    pub shot_kick: Option<Handle<Vec2>>,
    /// `random_pitch` (Single)
    #[serde(default)]
    pub random_pitch: f32,
    /// `random_yaw` (Single)
    #[serde(default)]
    pub random_yaw: f32,
    /// `decay` (Single)
    #[serde(default)]
    pub decay: f32,
    /// `end_decay` (Single)
    #[serde(default)]
    pub end_decay: f32,
    /// `recoil_time` (Single)
    #[serde(default)]
    pub recoil_time: f32,
    /// `delay` (Single)
    #[serde(default)]
    pub delay: f32,
    /// `curveAimRecoil` (Class)
    #[serde(default)]
    pub curve_aim_recoil: Option<Handle<SWeaponProceduralAimRecoilCurveConfigDef>>,
}

impl Pooled for SWeaponProceduralAimRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sweapon_procedural_aim_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sweapon_procedural_aim_recoil_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralAimRecoilConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralAimRecoilConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            max: match inst.get("max") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            pull_left_percentage: inst.get_f32("pull_left_percentage").unwrap_or_default(),
            shot_kick_first: match inst.get("shot_kick_first") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            shot_kick: match inst.get("shot_kick") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec2>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec2>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            random_pitch: inst.get_f32("random_pitch").unwrap_or_default(),
            random_yaw: inst.get_f32("random_yaw").unwrap_or_default(),
            decay: inst.get_f32("decay").unwrap_or_default(),
            end_decay: inst.get_f32("end_decay").unwrap_or_default(),
            recoil_time: inst.get_f32("recoil_time").unwrap_or_default(),
            delay: inst.get_f32("delay").unwrap_or_default(),
            curve_aim_recoil: match inst.get("curveAimRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralAimRecoilCurveConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralAimRecoilCurveConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralBodyRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralBodyRecoilConfigDef {
    /// `hipsPushForce` (Single)
    #[serde(default)]
    pub hips_push_force: f32,
    /// `hipsDampStrength` (Single)
    #[serde(default)]
    pub hips_damp_strength: f32,
    /// `hipsDampStrengthEnd` (Single)
    #[serde(default)]
    pub hips_damp_strength_end: f32,
    /// `spinePushForceFirst` (Single)
    #[serde(default)]
    pub spine_push_force_first: f32,
    /// `spinePushForce` (Single)
    #[serde(default)]
    pub spine_push_force: f32,
    /// `spineDampStrength` (Single)
    #[serde(default)]
    pub spine_damp_strength: f32,
    /// `spineDampStrengthEnd` (Single)
    #[serde(default)]
    pub spine_damp_strength_end: f32,
}

impl Pooled for SWeaponProceduralBodyRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sweapon_procedural_body_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sweapon_procedural_body_recoil_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralBodyRecoilConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralBodyRecoilConfigDef";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            hips_push_force: inst.get_f32("hipsPushForce").unwrap_or_default(),
            hips_damp_strength: inst.get_f32("hipsDampStrength").unwrap_or_default(),
            hips_damp_strength_end: inst.get_f32("hipsDampStrengthEnd").unwrap_or_default(),
            spine_push_force_first: inst.get_f32("spinePushForceFirst").unwrap_or_default(),
            spine_push_force: inst.get_f32("spinePushForce").unwrap_or_default(),
            spine_damp_strength: inst.get_f32("spineDampStrength").unwrap_or_default(),
            spine_damp_strength_end: inst.get_f32("spineDampStrengthEnd").unwrap_or_default(),
        }
    }
}

/// DCB type: `SHeadRecoilNoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHeadRecoilNoiseParams {
    /// `xNoise` (Single)
    #[serde(default)]
    pub x_noise: f32,
    /// `canXInvert` (Boolean)
    #[serde(default)]
    pub can_xinvert: bool,
    /// `yNoise` (Single)
    #[serde(default)]
    pub y_noise: f32,
    /// `canYInvert` (Boolean)
    #[serde(default)]
    pub can_yinvert: bool,
    /// `zNoise` (Single)
    #[serde(default)]
    pub z_noise: f32,
    /// `canZInvert` (Boolean)
    #[serde(default)]
    pub can_zinvert: bool,
}

impl Pooled for SHeadRecoilNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.shead_recoil_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.shead_recoil_noise_params }
}

impl<'a> Extract<'a> for SHeadRecoilNoiseParams {
    const TYPE_NAME: &'static str = "SHeadRecoilNoiseParams";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            x_noise: inst.get_f32("xNoise").unwrap_or_default(),
            can_xinvert: inst.get_bool("canXInvert").unwrap_or_default(),
            y_noise: inst.get_f32("yNoise").unwrap_or_default(),
            can_yinvert: inst.get_bool("canYInvert").unwrap_or_default(),
            z_noise: inst.get_f32("zNoise").unwrap_or_default(),
            can_zinvert: inst.get_bool("canZInvert").unwrap_or_default(),
        }
    }
}

/// DCB type: `SVecWithNoiseParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVecWithNoiseParams {
    /// `offset` (Class)
    #[serde(default)]
    pub offset: Option<Handle<Vec3>>,
    /// `noise` (Class)
    #[serde(default)]
    pub noise: Option<Handle<SHeadRecoilNoiseParams>>,
}

impl Pooled for SVecWithNoiseParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.svec_with_noise_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.svec_with_noise_params }
}

impl<'a> Extract<'a> for SVecWithNoiseParams {
    const TYPE_NAME: &'static str = "SVecWithNoiseParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            offset: match inst.get("offset") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise: match inst.get("noise") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHeadRecoilNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHeadRecoilNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SAmplitudeFreqencyDecayCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAmplitudeFreqencyDecayCurves {
    /// `frequencyDecayCurve` (Class)
    #[serde(default)]
    pub frequency_decay_curve: Option<Handle<BezierCurve>>,
    /// `amplitudeDecayCurve` (Class)
    #[serde(default)]
    pub amplitude_decay_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SAmplitudeFreqencyDecayCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.samplitude_freqency_decay_curves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.samplitude_freqency_decay_curves }
}

impl<'a> Extract<'a> for SAmplitudeFreqencyDecayCurves {
    const TYPE_NAME: &'static str = "SAmplitudeFreqencyDecayCurves";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            frequency_decay_curve: match inst.get("frequencyDecayCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            amplitude_decay_curve: match inst.get("amplitudeDecayCurve") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<BezierCurve>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<BezierCurve>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SWeaponProceduralHeadRecoilCurveConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralHeadRecoilCurveConfigDef {
    /// `position` (Class)
    #[serde(default)]
    pub position: Option<Handle<SVecWithNoiseParams>>,
    /// `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<SVecWithNoiseParams>>,
    /// `curves` (StrongPointer)
    #[serde(default)]
    pub curves: Option<Handle<SAmplitudeFreqencyDecayCurves>>,
    /// `headRecoilTime` (Single)
    #[serde(default)]
    pub head_recoil_time: f32,
    /// `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
    /// `smoothingSpeed` (Single)
    #[serde(default)]
    pub smoothing_speed: f32,
}

impl Pooled for SWeaponProceduralHeadRecoilCurveConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sweapon_procedural_head_recoil_curve_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sweapon_procedural_head_recoil_curve_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralHeadRecoilCurveConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralHeadRecoilCurveConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            position: match inst.get("position") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVecWithNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVecWithNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SVecWithNoiseParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SVecWithNoiseParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            curves: match inst.get("curves") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SAmplitudeFreqencyDecayCurves>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SAmplitudeFreqencyDecayCurves>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            head_recoil_time: inst.get_f32("headRecoilTime").unwrap_or_default(),
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            smoothing_speed: inst.get_f32("smoothingSpeed").unwrap_or_default(),
        }
    }
}

/// DCB type: `SWeaponProceduralHeadRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SWeaponProceduralHeadRecoilConfigDef {
    /// `frequency` (Single)
    #[serde(default)]
    pub frequency: f32,
    /// `smoothFactor` (Single)
    #[serde(default)]
    pub smooth_factor: f32,
    /// `frequencyNoiseFactor` (Single)
    #[serde(default)]
    pub frequency_noise_factor: f32,
    /// `maxDistance` (Single)
    #[serde(default)]
    pub max_distance: f32,
    /// `phase` (Single)
    #[serde(default)]
    pub phase: f32,
    /// `translation` (Class)
    #[serde(default)]
    pub translation: Option<Handle<Vec3>>,
    /// `translationNoise` (Single)
    #[serde(default)]
    pub translation_noise: f32,
    /// `rotation` (Class)
    #[serde(default)]
    pub rotation: Option<Handle<Ang3>>,
    /// `rotationNoise` (Single)
    #[serde(default)]
    pub rotation_noise: f32,
    /// `usePerlinNoise` (Boolean)
    #[serde(default)]
    pub use_perlin_noise: bool,
    /// `referenceSpeed` (Single)
    #[serde(default)]
    pub reference_speed: f32,
    /// `minSpeed` (Single)
    #[serde(default)]
    pub min_speed: f32,
    /// `minScale` (Single)
    #[serde(default)]
    pub min_scale: f32,
    /// `maxSpeed` (Single)
    #[serde(default)]
    pub max_speed: f32,
    /// `maxScale` (Single)
    #[serde(default)]
    pub max_scale: f32,
    /// `curveRecoil` (Class)
    #[serde(default)]
    pub curve_recoil: Option<Handle<SWeaponProceduralHeadRecoilCurveConfigDef>>,
}

impl Pooled for SWeaponProceduralHeadRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.sweapon_procedural_head_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.sweapon_procedural_head_recoil_config_def }
}

impl<'a> Extract<'a> for SWeaponProceduralHeadRecoilConfigDef {
    const TYPE_NAME: &'static str = "SWeaponProceduralHeadRecoilConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            frequency: inst.get_f32("frequency").unwrap_or_default(),
            smooth_factor: inst.get_f32("smoothFactor").unwrap_or_default(),
            frequency_noise_factor: inst.get_f32("frequencyNoiseFactor").unwrap_or_default(),
            max_distance: inst.get_f32("maxDistance").unwrap_or_default(),
            phase: inst.get_f32("phase").unwrap_or_default(),
            translation: match inst.get("translation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            translation_noise: inst.get_f32("translationNoise").unwrap_or_default(),
            rotation: match inst.get("rotation") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_noise: inst.get_f32("rotationNoise").unwrap_or_default(),
            use_perlin_noise: inst.get_bool("usePerlinNoise").unwrap_or_default(),
            reference_speed: inst.get_f32("referenceSpeed").unwrap_or_default(),
            min_speed: inst.get_f32("minSpeed").unwrap_or_default(),
            min_scale: inst.get_f32("minScale").unwrap_or_default(),
            max_speed: inst.get_f32("maxSpeed").unwrap_or_default(),
            max_scale: inst.get_f32("maxScale").unwrap_or_default(),
            curve_recoil: match inst.get("curveRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilCurveConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilCurveConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `WeaponProceduralRecoilConfigDef`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponProceduralRecoilConfigDef {
    /// `weaponProceduralHandsRecoil` (Class)
    #[serde(default)]
    pub weapon_procedural_hands_recoil: Option<Handle<SWeaponProceduralHandsRecoilConfigDef>>,
    /// `weaponProceduralAimRecoil` (Class)
    #[serde(default)]
    pub weapon_procedural_aim_recoil: Option<Handle<SWeaponProceduralAimRecoilConfigDef>>,
    /// `weaponProceduralBodyRecoil` (Class)
    #[serde(default)]
    pub weapon_procedural_body_recoil: Option<Handle<SWeaponProceduralBodyRecoilConfigDef>>,
    /// `weaponProceduralHeadRecoil` (Class)
    #[serde(default)]
    pub weapon_procedural_head_recoil: Option<Handle<SWeaponProceduralHeadRecoilConfigDef>>,
}

impl Pooled for WeaponProceduralRecoilConfigDef {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.weaponproceduralrecoil.weapon_procedural_recoil_config_def }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.weaponproceduralrecoil.weapon_procedural_recoil_config_def }
}

impl<'a> Extract<'a> for WeaponProceduralRecoilConfigDef {
    const TYPE_NAME: &'static str = "WeaponProceduralRecoilConfigDef";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            weapon_procedural_hands_recoil: match inst.get("weaponProceduralHandsRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHandsRecoilConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHandsRecoilConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_procedural_aim_recoil: match inst.get("weaponProceduralAimRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralAimRecoilConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralAimRecoilConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_procedural_body_recoil: match inst.get("weaponProceduralBodyRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralBodyRecoilConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralBodyRecoilConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            weapon_procedural_head_recoil: match inst.get("weaponProceduralHeadRecoil") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilConfigDef>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SWeaponProceduralHeadRecoilConfigDef>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

