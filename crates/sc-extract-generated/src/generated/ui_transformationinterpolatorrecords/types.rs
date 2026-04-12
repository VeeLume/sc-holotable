// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `ui-transformationinterpolatorrecords`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `TransformationInterpolatorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationInterpolatorParams {
    /// `startOffsetValues` (Class)
    #[serde(default)]
    pub start_offset_values: Option<Handle<Vec3>>,
    /// `endOffsetValues` (Class)
    #[serde(default)]
    pub end_offset_values: Option<Handle<Vec3>>,
    /// `offsetInterpolationModes` (EnumChoice)
    #[serde(default)]
    pub offset_interpolation_modes: String,
    /// `startRotationValues` (Class)
    #[serde(default)]
    pub start_rotation_values: Option<Handle<Ang3>>,
    /// `endRotationValues` (Class)
    #[serde(default)]
    pub end_rotation_values: Option<Handle<Ang3>>,
    /// `rotationInterpolationModes` (EnumChoice)
    #[serde(default)]
    pub rotation_interpolation_modes: String,
    /// `startScaleValue` (Single)
    #[serde(default)]
    pub start_scale_value: f32,
    /// `endScaleValue` (Single)
    #[serde(default)]
    pub end_scale_value: f32,
    /// `scaleInterpolationMode` (EnumChoice)
    #[serde(default)]
    pub scale_interpolation_mode: String,
}

impl Pooled for TransformationInterpolatorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_transformationinterpolatorrecords.transformation_interpolator_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_transformationinterpolatorrecords.transformation_interpolator_params }
}

impl<'a> Extract<'a> for TransformationInterpolatorParams {
    const TYPE_NAME: &'static str = "TransformationInterpolatorParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            start_offset_values: match inst.get("startOffsetValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            end_offset_values: match inst.get("endOffsetValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Vec3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Vec3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            offset_interpolation_modes: inst.get_str("offsetInterpolationModes").map(String::from).unwrap_or_default(),
            start_rotation_values: match inst.get("startRotationValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            end_rotation_values: match inst.get("endRotationValues") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<Ang3>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<Ang3>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            rotation_interpolation_modes: inst.get_str("rotationInterpolationModes").map(String::from).unwrap_or_default(),
            start_scale_value: inst.get_f32("startScaleValue").unwrap_or_default(),
            end_scale_value: inst.get_f32("endScaleValue").unwrap_or_default(),
            scale_interpolation_mode: inst.get_str("scaleInterpolationMode").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `MovieClipTransformationInterpolatorParams`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieClipTransformationInterpolatorParams {
    /// `movieClipName` (String)
    #[serde(default)]
    pub movie_clip_name: String,
    /// `transformationInterpolatorParams` (Class)
    #[serde(default)]
    pub transformation_interpolator_params: Option<Handle<TransformationInterpolatorParams>>,
}

impl Pooled for MovieClipTransformationInterpolatorParams {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_transformationinterpolatorrecords.movie_clip_transformation_interpolator_params }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_transformationinterpolatorrecords.movie_clip_transformation_interpolator_params }
}

impl<'a> Extract<'a> for MovieClipTransformationInterpolatorParams {
    const TYPE_NAME: &'static str = "MovieClipTransformationInterpolatorParams";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            movie_clip_name: inst.get_str("movieClipName").map(String::from).unwrap_or_default(),
            transformation_interpolator_params: match inst.get("transformationInterpolatorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TransformationInterpolatorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TransformationInterpolatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

/// DCB type: `MovieClipTransformationInterpolator`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieClipTransformationInterpolator {
    /// `interpolationTime` (Single)
    #[serde(default)]
    pub interpolation_time: f32,
    /// `movieClipTransformationInterpolatorParams` (Class (array))
    #[serde(default)]
    pub movie_clip_transformation_interpolator_params: Vec<Handle<MovieClipTransformationInterpolatorParams>>,
}

impl Pooled for MovieClipTransformationInterpolator {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_transformationinterpolatorrecords.movie_clip_transformation_interpolator }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_transformationinterpolatorrecords.movie_clip_transformation_interpolator }
}

impl<'a> Extract<'a> for MovieClipTransformationInterpolator {
    const TYPE_NAME: &'static str = "MovieClipTransformationInterpolator";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interpolation_time: inst.get_f32("interpolationTime").unwrap_or_default(),
            movie_clip_transformation_interpolator_params: inst.get_array("movieClipTransformationInterpolatorParams")
                .map(|arr| arr.filter_map(|v| match v {
                        Value::Class { struct_index, data } => Some(b.alloc_nested::<MovieClipTransformationInterpolatorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                        Value::ClassRef(r)
                        | Value::StrongPointer(Some(r))
                        | Value::WeakPointer(Some(r)) => Some(b.alloc_nested::<MovieClipTransformationInterpolatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                        _ => None,
                    }).collect())
                .unwrap_or_default(),
        }
    }
}

/// DCB type: `TransformationInterpolator`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationInterpolator {
    /// `interpolationTime` (Single)
    #[serde(default)]
    pub interpolation_time: f32,
    /// `transformationInterpolatorParams` (Class)
    #[serde(default)]
    pub transformation_interpolator_params: Option<Handle<TransformationInterpolatorParams>>,
}

impl Pooled for TransformationInterpolator {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.ui_transformationinterpolatorrecords.transformation_interpolator }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.ui_transformationinterpolatorrecords.transformation_interpolator }
}

impl<'a> Extract<'a> for TransformationInterpolator {
    const TYPE_NAME: &'static str = "TransformationInterpolator";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interpolation_time: inst.get_f32("interpolationTime").unwrap_or_default(),
            transformation_interpolator_params: match inst.get("transformationInterpolatorParams") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<TransformationInterpolatorParams>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<TransformationInterpolatorParams>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
        }
    }
}

