// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `ui-transformationinterpolatorrecords` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiTransformationinterpolatorrecordsPools {
    #[serde(default)]
    pub transformation_interpolator_params: Vec<Option<TransformationInterpolatorParams>>,
    #[serde(default)]
    pub movie_clip_transformation_interpolator_params: Vec<Option<MovieClipTransformationInterpolatorParams>>,
    #[serde(default)]
    pub movie_clip_transformation_interpolator: Vec<Option<MovieClipTransformationInterpolator>>,
    #[serde(default)]
    pub transformation_interpolator: Vec<Option<TransformationInterpolator>>,
}
