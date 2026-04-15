// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]

use super::super::*;

/// Pool storage for the `entities-geometryinstancer` feature.
#[derive(Default)]
pub struct EntitiesGeometryinstancerPools {
    pub geometry_instancer_serialized: Vec<Option<GeometryInstancer_Serialized>>,
    pub geometry_instancer_component_params: Vec<Option<GeometryInstancerComponentParams>>,
}
