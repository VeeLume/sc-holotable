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

/// Pool storage for the `entities-rastar` feature.
#[derive(Default)]
pub struct EntitiesRastarPools {
    pub base_building_params: Vec<Option<BaseBuildingParams>>,
    pub object_container_modifier_params: Vec<Option<ObjectContainerModifierParams>>,
    pub rastar_location_params: Vec<Option<RastarLocationParams>>,
    pub rastar_uiparams: Vec<Option<RastarUIParams>>,
}
