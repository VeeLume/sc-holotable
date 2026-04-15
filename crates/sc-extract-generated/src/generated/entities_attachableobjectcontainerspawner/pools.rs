// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use serde::{Deserialize, Serialize};
use super::super::*;

/// Pool storage for the `entities-attachableobjectcontainerspawner` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesAttachableobjectcontainerspawnerPools {
    #[serde(default)]
    pub sattachable_object_container_data: Vec<Option<SAttachableObjectContainerData>>,
    #[serde(default)]
    pub sattachable_object_container_spawner_params: Vec<Option<SAttachableObjectContainerSpawnerParams>>,
}
