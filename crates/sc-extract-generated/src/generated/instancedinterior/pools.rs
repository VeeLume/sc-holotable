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

/// Pool storage for the `instancedinterior` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstancedinteriorPools {
    #[serde(default)]
    pub instanced_interior_location_params: Vec<Option<InstancedInteriorLocationParams>>,
    #[serde(default)]
    pub instanced_interior_location_map: Vec<Option<InstancedInteriorLocationMap>>,
}
