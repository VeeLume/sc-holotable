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

use crate::{Builder, Extract, Handle, LocaleKey, Pooled};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};

use super::super::*;

/// DCB type: `TransformationInterpolator`
pub struct TransformationInterpolator {
    /// `interpolationTime` (Single)
    pub interpolation_time: f32,
    /// `transformationInterpolatorParams` (Class)
    pub transformation_interpolator_params: Option<Handle<TransformationInterpolatorParams>>,
}

impl Pooled for TransformationInterpolator {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> {
        &pools
            .ui_transformationinterpolatorrecords
            .transformation_interpolator
    }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> {
        &mut pools
            .ui_transformationinterpolatorrecords
            .transformation_interpolator
    }
}

impl<'a> Extract<'a> for TransformationInterpolator {
    const TYPE_NAME: &'static str = "TransformationInterpolator";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            interpolation_time: inst.get_f32("interpolationTime").unwrap_or_default(),
            transformation_interpolator_params: match inst.get("transformationInterpolatorParams") {
                Some(Value::Class { struct_index, data }) => {
                    Some(b.alloc_nested::<TransformationInterpolatorParams>(
                        Instance::from_inline_data(b.db, struct_index, data),
                        false,
                    ))
                }
                _ => None,
            },
        }
    }
}
