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

/// DCB type: `SXYZCurvesWithMaxValuesModifer`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SXYZCurvesWithMaxValuesModifer {
    /// DCB field: `xMaxValueModifier` (Single)
    #[serde(default)]
    pub x_max_value_modifier: f32,
    /// DCB field: `yMaxValueModifier` (Single)
    #[serde(default)]
    pub y_max_value_modifier: f32,
    /// DCB field: `zMaxValueModifier` (Single)
    #[serde(default)]
    pub z_max_value_modifier: f32,
    /// DCB field: `minLimitsModifier` (Class)
    #[serde(default)]
    pub min_limits_modifier: Option<Handle<Vec3>>,
    /// DCB field: `maxLimitsModifier` (Class)
    #[serde(default)]
    pub max_limits_modifier: Option<Handle<Vec3>>,
    /// DCB field: `noiseModifier` (Class)
    #[serde(default)]
    pub noise_modifier: Option<Handle<SHandsRecoilCurveNoiseModifer>>,
}

impl Pooled for SXYZCurvesWithMaxValuesModifer {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sx.sxyzcurves_with_max_values_modifer }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sx.sxyzcurves_with_max_values_modifer }
}

impl<'a> Extract<'a> for SXYZCurvesWithMaxValuesModifer {
    const TYPE_NAME: &'static str = "SXYZCurvesWithMaxValuesModifer";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            x_max_value_modifier: inst.get_f32("xMaxValueModifier").unwrap_or_default(),
            y_max_value_modifier: inst.get_f32("yMaxValueModifier").unwrap_or_default(),
            z_max_value_modifier: inst.get_f32("zMaxValueModifier").unwrap_or_default(),
            min_limits_modifier: match inst.get("minLimitsModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            max_limits_modifier: match inst.get("maxLimitsModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            noise_modifier: match inst.get("noiseModifier") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SHandsRecoilCurveNoiseModifer>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SHandsRecoilCurveNoiseModifer>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `SXYZCurves`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SXYZCurves {
    /// DCB field: `xCurve` (Class)
    #[serde(default)]
    pub x_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `yCurve` (Class)
    #[serde(default)]
    pub y_curve: Option<Handle<BezierCurve>>,
    /// DCB field: `zCurve` (Class)
    #[serde(default)]
    pub z_curve: Option<Handle<BezierCurve>>,
}

impl Pooled for SXYZCurves {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sx.sxyzcurves }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sx.sxyzcurves }
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

/// DCB type: `SXYZCurvesArrays`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SXYZCurvesArrays {
    /// DCB field: `xCurves` (Class (array))
    #[serde(default)]
    pub x_curves: Vec<Handle<SCurve>>,
    /// DCB field: `yCurves` (Class (array))
    #[serde(default)]
    pub y_curves: Vec<Handle<SCurve>>,
    /// DCB field: `zCurves` (Class (array))
    #[serde(default)]
    pub z_curves: Vec<Handle<SCurve>>,
}

impl Pooled for SXYZCurvesArrays {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sx.sxyzcurves_arrays }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sx.sxyzcurves_arrays }
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

/// DCB type: `SXYZCurvesWithMaxValues`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SXYZCurvesWithMaxValues {
    /// DCB field: `xMaxValue` (Single)
    #[serde(default)]
    pub x_max_value: f32,
    /// DCB field: `yMaxValue` (Single)
    #[serde(default)]
    pub y_max_value: f32,
    /// DCB field: `zMaxValue` (Single)
    #[serde(default)]
    pub z_max_value: f32,
    /// DCB field: `minLimits` (Class)
    #[serde(default)]
    pub min_limits: Option<Handle<Vec3>>,
    /// DCB field: `maxLimits` (Class)
    #[serde(default)]
    pub max_limits: Option<Handle<Vec3>>,
    /// DCB field: `curves` (StrongPointer)
    #[serde(default)]
    pub curves: Option<Handle<SXYZCurvesArrays>>,
    /// DCB field: `noiseParams` (Class)
    #[serde(default)]
    pub noise_params: Option<Handle<SHandsRecoilCurveNoiseParams>>,
}

impl Pooled for SXYZCurvesWithMaxValues {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_sx.sxyzcurves_with_max_values }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_sx.sxyzcurves_with_max_values }
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

