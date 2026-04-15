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

/// Pool storage for the `entities-rastar` feature.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntitiesRastarPools {
    #[serde(default)]
    pub base_building_params: Vec<Option<BaseBuildingParams>>,
    #[serde(default)]
    pub object_container_modifier_params: Vec<Option<ObjectContainerModifierParams>>,
    #[serde(default)]
    pub rastar_location_params: Vec<Option<RastarLocationParams>>,
    #[serde(default)]
    pub rastar_uiparams: Vec<Option<RastarUIParams>>,
}
