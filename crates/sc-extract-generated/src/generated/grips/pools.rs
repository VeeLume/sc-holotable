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

/// Pool storage for the `grips` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GripsPools {
    #[serde(default)]
    pub sgrip: Vec<Option<SGrip>>,
    #[serde(default)]
    pub sgrip_shape_params: Vec<Option<SGripShapeParams>>,
    #[serde(default)]
    pub grip: Vec<Option<Grip>>,
}
