// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, dead_code, unused_imports)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use crate::Handle;
use super::super::*;

/// Record index for the `instancedinterior` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstancedinteriorIndex {
    #[serde(default)]
    pub instanced_interior_location_params: HashMap<CigGuid, Handle<InstancedInteriorLocationParams>>,
    #[serde(default)]
    pub instanced_interior_location_map: HashMap<CigGuid, Handle<InstancedInteriorLocationMap>>,
}

impl InstancedinteriorIndex {
    pub fn len(&self) -> usize {
        self.instanced_interior_location_params.len()
            + self.instanced_interior_location_map.len()
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
