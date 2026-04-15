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

/// Pool storage for the `entities-objectcontainers` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesObjectcontainersPools {
    #[serde(default)]
    pub smovable_object_container_params: Vec<Option<SMovableObjectContainerParams>>,
    #[serde(default)]
    pub socinstance_slot: Vec<Option<SOCInstanceSlot>>,
    #[serde(default)]
    pub socinstance_component_params: Vec<Option<SOCInstanceComponentParams>>,
    #[serde(default)]
    pub sentity_component_procedural_ocmodifier_params: Vec<Option<SEntityComponentProceduralOCModifierParams>>,
    #[serde(default)]
    pub sorbit_component_params: Vec<Option<SOrbitComponentParams>>,
    #[serde(default)]
    pub sentity_component_physics_grid_params: Vec<Option<SEntityComponentPhysicsGridParams>>,
    #[serde(default)]
    pub sentity_base_physics_grid_params: Vec<Option<SEntityBasePhysicsGridParams>>,
}
